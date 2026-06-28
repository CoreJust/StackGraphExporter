use super::bitset::BitSet;
use crate::core::CFLRuleIndex;

use core::arch::x86_64::*;

pub trait ReachabilityState: Send + Clone {
    // Monotonoues means that as propagation continues,
    // the state can only accumulate more reachables.
    // This allows to use SCC to optimize the algorithm.
    const MONOTONOUS: bool;

    fn empty(size: u32) -> Self;
    fn from_opening(size: u32, opening: CFLRuleIndex) -> Self;
    fn from_closing(size: u32, closing: CFLRuleIndex) -> Self;
    fn from_scc(size: u32, openings: &[CFLRuleIndex], closings: &[CFLRuleIndex]) -> Self;

    // Invader is the state is is propagated upon us
    fn merge_with(&mut self, invader: &Self) -> bool;
    // Current for openings and the opposite is for closings (or vice versa)
    fn unreachable_if_opposite(&self, opposite: &Self) -> bool;
}

#[derive(Clone)]
pub struct TrivialReachabilityState {
    state: bool,
}

impl ReachabilityState for TrivialReachabilityState {
    const MONOTONOUS: bool = true;

    fn empty(_: u32) -> Self {
        Self { state: false }
    }

    fn from_opening(_: u32, _: CFLRuleIndex) -> Self {
        Self { state: true }
    }

    fn from_closing(_: u32, _: CFLRuleIndex) -> Self {
        Self { state: false }
    }

    fn from_scc(_: u32, openings: &[CFLRuleIndex], _: &[CFLRuleIndex]) -> Self {
        Self {
            state: !openings.is_empty(),
        }
    }

    fn merge_with(&mut self, invader: &Self) -> bool {
        if self.state >= invader.state {
            false
        } else {
            self.state |= invader.state;
            true
        }
    }

    fn unreachable_if_opposite(&self, opposite: &Self) -> bool {
        !self.state || !opposite.state
    }
}

#[derive(Clone)]
pub struct SingleReachabilityState<T: BitSet> {
    state: T,
}

impl<T: BitSet> ReachabilityState for SingleReachabilityState<T> {
    const MONOTONOUS: bool = true;

    fn empty(size: u32) -> Self {
        Self {
            state: T::empty(size),
        }
    }

    fn from_opening(size: u32, opening: CFLRuleIndex) -> Self {
        Self {
            state: T::with_bits_set(size, &[opening]),
        }
    }

    fn from_closing(size: u32, _: CFLRuleIndex) -> Self {
        Self {
            state: T::empty(size),
        }
    }

    fn from_scc(size: u32, openings: &[CFLRuleIndex], _: &[CFLRuleIndex]) -> Self {
        Self {
            state: T::with_bits_set(size, openings),
        }
    }

    fn merge_with(&mut self, invader: &Self) -> bool {
        self.state.unite_with(&invader.state)
    }

    fn unreachable_if_opposite(&self, opposite: &Self) -> bool {
        self.state.is_disjoint(&opposite.state)
    }
}

// The following is currently experimental
#[derive(Clone)]
pub struct KReachabilityState<T: BitSet, const K: usize> {
    deep_stack: T,
    fronts: Vec<T>,              // len = K - 1
    is_reached_by_opening: bool, // Marks a closing node as reachable
    opening: Option<CFLRuleIndex>,
    closing: Option<CFLRuleIndex>,
}

impl<T: BitSet, const K: usize> KReachabilityState<T, K> {
    #[inline]
    fn front_depth() -> usize {
        K.saturating_sub(1)
    }

    #[inline]
    fn empty_fronts(size: u32) -> Vec<T> {
        (0..Self::front_depth()).map(|_| T::empty(size)).collect()
    }
}

impl<T: BitSet, const K: usize> ReachabilityState for KReachabilityState<T, K> {
    const MONOTONOUS: bool = false;

    fn empty(size: u32) -> Self {
        Self {
            deep_stack: T::empty(size),
            fronts: Self::empty_fronts(size),
            is_reached_by_opening: false,
            opening: None,
            closing: None,
        }
    }

    fn from_opening(size: u32, opening: CFLRuleIndex) -> Self {
        let mut fronts = Self::empty_fronts(size);
        fronts[0].insert(opening);
        Self {
            deep_stack: T::empty(size),
            fronts,
            is_reached_by_opening: false,
            opening: Some(opening),
            closing: None,
        }
    }

    fn from_closing(size: u32, closing: CFLRuleIndex) -> Self {
        Self {
            deep_stack: T::empty(size),
            fronts: Self::empty_fronts(size),
            is_reached_by_opening: false,
            opening: None,
            closing: Some(closing),
        }
    }

    fn from_scc(_: u32, _: &[CFLRuleIndex], _: &[CFLRuleIndex]) -> Self {
        panic!("KReachabilityState with K >= 2 does not use SCC condensation");
    }

    fn merge_with(&mut self, invader: &Self) -> bool {
        let depth = Self::front_depth();

        let mut changed = false;
        match (self.opening, self.closing) {
            (Some(_), None) => {
                changed |= self.deep_stack.unite_with(&invader.deep_stack);
                changed |= self.deep_stack.unite_with(&invader.fronts.last().unwrap());
                for i in 1..depth {
                    changed |= self.fronts[i].unite_with(&invader.fronts[i - 1]);
                }
            }
            (None, Some(closing)) => {
                if invader.fronts[0].contains(closing) {
                    self.is_reached_by_opening = true;
                    changed |= self.deep_stack.unite_with(&invader.deep_stack);
                    changed |= self.fronts[depth - 1].unite_with(&invader.deep_stack);
                    for i in 0..(depth - 1) {
                        changed |= self.fronts[i].unite_with(&invader.fronts[i + 1]);
                    }
                }
            }
            (None, None) => {
                changed |= self.deep_stack.unite_with(&invader.deep_stack);
                for i in 0..depth {
                    changed |= self.fronts[i].unite_with(&invader.fronts[i]);
                }
            }
            (Some(_), Some(_)) => unreachable!("A node cannot be both opening and closing"),
        }

        changed
    }

    fn unreachable_if_opposite(&self, opposite: &Self) -> bool {
        self.deep_stack.is_disjoint(&opposite.deep_stack)
            && self
                .fronts
                .iter()
                .zip(opposite.fronts.iter())
                .all(|(front, opposite_front)| front.is_disjoint(opposite_front))
            && !self.is_reached_by_opening
            && !opposite.is_reached_by_opening
    }
}

// Store deep_stack and front in one Vec<usize>: first half = deep_stack, second half = front.
#[derive(Clone)]
pub struct PackedDoubleReachabilityState {
    bits: Vec<usize>, // length = 2 * words_per_set
    is_reached_by_opening: bool,
    opening: Option<CFLRuleIndex>,
    closing: Option<CFLRuleIndex>,
    words_per_set: usize, // number of usize words per bitset
}

impl PackedDoubleReachabilityState {
    #[inline]
    fn deep_mut(&mut self) -> &mut [usize] {
        &mut self.bits[..self.words_per_set]
    }
    #[inline]
    fn front_mut(&mut self) -> &mut [usize] {
        &mut self.bits[self.words_per_set..]
    }

    fn unite_deep_with_two(dst: &mut [usize], a: &[usize], b: &[usize]) -> bool {
        #[cfg(target_arch = "x86_64")]
        if std::is_x86_feature_detected!("avx2") {
            return unsafe { Self::unite_deep_two_avx2(dst, a, b) };
        }
        Self::unite_scalar(dst, |i| a[i] | b[i])
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn unite_deep_two_avx2(dst: &mut [usize], a: &[usize], b: &[usize]) -> bool {
        let mut changed = false;
        let mut i = 0;
        let len = dst.len();
        while i + 4 <= len {
            let d = _mm256_loadu_si256(dst.as_ptr().add(i) as *const __m256i);
            let av = _mm256_loadu_si256(a.as_ptr().add(i) as *const __m256i);
            let bv = _mm256_loadu_si256(b.as_ptr().add(i) as *const __m256i);
            let combined = _mm256_or_si256(av, bv);
            let newv = _mm256_or_si256(d, combined);
            _mm256_storeu_si256(dst.as_mut_ptr().add(i) as *mut __m256i, newv);
            if _mm256_testz_si256(_mm256_xor_si256(d, newv), _mm256_xor_si256(d, newv)) == 0 {
                changed = true;
            }
            i += 4;
        }
        while i < len {
            let old = dst[i];
            let new = old | a[i] | b[i];
            if new != old {
                dst[i] = new;
                changed = true;
            }
            i += 1;
        }
        changed
    }

    fn unite_scalar<F: Fn(usize) -> usize>(dst: &mut [usize], f: F) -> bool {
        let mut changed = false;
        for i in 0..dst.len() {
            let old = dst[i];
            let new = old | f(i);
            if new != old {
                dst[i] = new;
                changed = true;
            }
        }
        changed
    }

    fn unite_deep_with_one(dst: &mut [usize], src: &[usize]) -> bool {
        #[cfg(target_arch = "x86_64")]
        if std::is_x86_feature_detected!("avx2") {
            return unsafe { Self::unite_avx2(dst, src) };
        }
        Self::unite_scalar(dst, |i| src[i])
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn unite_avx2(dst: &mut [usize], src: &[usize]) -> bool {
        let mut changed = false;
        let mut i = 0;
        let len = dst.len();
        while i + 4 <= len {
            let d = _mm256_loadu_si256(dst.as_ptr().add(i) as *const __m256i);
            let s = _mm256_loadu_si256(src.as_ptr().add(i) as *const __m256i);
            let newv = _mm256_or_si256(d, s);
            _mm256_storeu_si256(dst.as_mut_ptr().add(i) as *mut __m256i, newv);
            if _mm256_testz_si256(_mm256_xor_si256(d, newv), _mm256_xor_si256(d, newv)) == 0 {
                changed = true;
            }
            i += 4;
        }
        while i < len {
            let old = dst[i];
            let new = old | src[i];
            if new != old {
                dst[i] = new;
                changed = true;
            }
            i += 1;
        }
        changed
    }
}

impl ReachabilityState for PackedDoubleReachabilityState {
    const MONOTONOUS: bool = false;

    fn empty(size: u32) -> Self {
        let words = (size as usize + usize::BITS as usize - 1) / usize::BITS as usize;
        Self {
            bits: vec![0; words * 2],
            is_reached_by_opening: false,
            opening: None,
            closing: None,
            words_per_set: words,
        }
    }

    fn from_opening(size: u32, opening: CFLRuleIndex) -> Self {
        let words = (size as usize + usize::BITS as usize - 1) / usize::BITS as usize;
        let mut bits = vec![0; words * 2];
        let word_idx = (opening as usize) / (usize::BITS as usize);
        let bit = (opening as usize) % (usize::BITS as usize);
        bits[words + word_idx] = 1 << bit;
        Self {
            bits,
            is_reached_by_opening: false,
            opening: Some(opening),
            closing: None,
            words_per_set: words,
        }
    }

    fn from_closing(size: u32, closing: CFLRuleIndex) -> Self {
        let words = (size as usize + usize::BITS as usize - 1) / usize::BITS as usize;
        Self {
            bits: vec![0; words * 2],
            is_reached_by_opening: false,
            opening: None,
            closing: Some(closing),
            words_per_set: words,
        }
    }

    fn from_scc(_: u32, _: &[CFLRuleIndex], _: &[CFLRuleIndex]) -> Self {
        panic!("PackedDoubleReachabilityState does not use SCC condensation");
    }

    fn merge_with(&mut self, invader: &Self) -> bool {
        debug_assert_eq!(self.words_per_set, invader.words_per_set);
        let words = self.words_per_set;

        if self.opening.is_some() {
            Self::unite_deep_with_two(
                self.deep_mut(),
                &invader.bits[..words],
                &invader.bits[words..],
            )
        } else if let Some(closing) = self.closing {
            let front_invader = &invader.bits[words..];
            let closing_word = (closing as usize) / (usize::BITS as usize);
            let closing_bit = 1 << ((closing as usize) % (usize::BITS as usize));
            if (front_invader[closing_word] & closing_bit) == 0 {
                false
            } else {
                if !self.is_reached_by_opening {
                    self.is_reached_by_opening = true;
                    let mut changed =
                        Self::unite_deep_with_one(self.deep_mut(), &invader.bits[..words]);
                    changed |= Self::unite_deep_with_one(self.front_mut(), &invader.bits[..words]);
                    changed
                } else {
                    let mut changed =
                        Self::unite_deep_with_one(self.deep_mut(), &invader.bits[..words]);
                    changed |= Self::unite_deep_with_one(self.front_mut(), &invader.bits[..words]);
                    changed
                }
            }
        } else {
            let changed;
            let total = words * 2;
            #[cfg(target_arch = "x86_64")]
            if std::is_x86_feature_detected!("avx2") {
                unsafe {
                    changed = Self::unite_avx2(&mut self.bits[..total], &invader.bits[..total]);
                }
            } else {
                changed = Self::unite_scalar(&mut self.bits[..total], |i| invader.bits[i]);
            }
            changed
        }
    }

    fn unreachable_if_opposite(&self, opposite: &Self) -> bool {
        let words = self.words_per_set;
        let deep_self = &self.bits[..words];
        let front_self = &self.bits[words..];
        let deep_opp = &opposite.bits[..words];
        let front_opp = &opposite.bits[words..];
        (deep_self
            .iter()
            .zip(deep_opp.iter())
            .all(|(a, b)| a & b == 0))
            && (front_self
                .iter()
                .zip(front_opp.iter())
                .all(|(a, b)| a & b == 0))
            && !self.is_reached_by_opening
            && !opposite.is_reached_by_opening
    }
}
