use super::bitset::BitSet;
use crate::core::CFLRuleIndex;

pub trait ReachabilityState: Send {
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

#[derive(Clone)]
pub struct DoubleReachabilityState<T: BitSet> {
    state: T,
    front: T, // Openings that can reach here immediately, solely by epsilon nodes
    opening: Option<CFLRuleIndex>,
    closing: Option<CFLRuleIndex>,
}

impl<T: BitSet> ReachabilityState for DoubleReachabilityState<T> {
    const MONOTONOUS: bool = false;

    fn empty(size: u32) -> Self {
        Self {
            state: T::empty(size),
            front: T::empty(size),
            opening: None,
            closing: None,
        }
    }

    fn from_opening(size: u32, opening: CFLRuleIndex) -> Self {
        Self {
            state: T::with_bits_set(size, &[opening]),
            front: T::with_bits_set(size, &[opening]),
            opening: Some(opening),
            closing: None,
        }
    }

    fn from_closing(size: u32, closing: CFLRuleIndex) -> Self {
        Self {
            state: T::empty(size),
            front: T::empty(size),
            opening: None,
            closing: Some(closing),
        }
    }

    fn from_scc(_: u32, _: &[CFLRuleIndex], _: &[CFLRuleIndex]) -> Self {
        panic!("Cannot create DoubleReachabilityState from SCC!")
    }

    fn merge_with(&mut self, invader: &Self) -> bool {
        if let Some(_) = self.opening {
            // No front can reach an opening node - it overwrites it with itself
            self.state.unite_with(&invader.state)
        } else if let Some(closing) = self.closing {
            // If front has no openning corresponding to this closing,
            // then it cannot pass since a path will not be formed
            if invader.front.contains(closing) == false {
                false
            } else {
                let changed_state = self.state.unite_with(&invader.state);
                // We have no information about the front after crossing the
                // closing node, it can be any opening from the state
                let changed_front = self.front.unite_with(&invader.state);
                changed_state || changed_front
            }
        } else {
            let changed_state = self.state.unite_with(&invader.state);
            let changed_front = self.front.unite_with(&invader.front);
            changed_state || changed_front
        }
    }

    fn unreachable_if_opposite(&self, opposite: &Self) -> bool {
        self.state.is_disjoint(&opposite.state)
    }
}

// The following is currently experimental
pub struct KReachabilityState<T: BitSet, const K: usize> {
    size: u32,
    state: T,
    fronts: Vec<T>, // len = K - 1
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

    #[inline]
    fn singleton(size: u32, bit: CFLRuleIndex) -> T {
        T::with_bits_set(size, &[bit])
    }
}

impl<T: BitSet, const K: usize> ReachabilityState for KReachabilityState<T, K> {
    const MONOTONOUS: bool = false;

    fn empty(size: u32) -> Self {
        Self {
            size,
            state: T::empty(size),
            fronts: Self::empty_fronts(size),
            opening: None,
            closing: None,
        }
    }

    fn from_opening(size: u32, opening: CFLRuleIndex) -> Self {
        let mut fronts = Self::empty_fronts(size);
        if !fronts.is_empty() {
            fronts[0] = Self::singleton(size, opening);
        }

        Self {
            size,
            state: Self::singleton(size, opening),
            fronts,
            opening: Some(opening),
            closing: None,
        }
    }

    fn from_closing(size: u32, closing: CFLRuleIndex) -> Self {
        Self {
            size,
            state: T::empty(size),
            fronts: Self::empty_fronts(size),
            opening: None,
            closing: Some(closing),
        }
    }

    fn from_scc(_: u32, _: &[CFLRuleIndex], _: &[CFLRuleIndex]) -> Self {
        panic!("KReachabilityState with K >= 2 does not use SCC condensation");
    }

    fn merge_with(&mut self, invader: &Self) -> bool {
        let depth = Self::front_depth();

        if depth == 0 {
            // K = 1: exact Single semantics
            return self.state.unite_with(&invader.state);
        }

        if depth == 1 {
            // K = 2: exact Double semantics
            let mut changed = false;

            match (self.opening, self.closing) {
                (Some(opening), None) => {
                    changed |= self.state.unite_with(&invader.state);
                    let opening_bit = Self::singleton(self.size, opening);
                    changed |= self.fronts[0].unite_with(&opening_bit);
                }
                (None, Some(closing)) => {
                    if invader.fronts[0].contains(closing) {
                        changed |= self.state.unite_with(&invader.state);
                        changed |= self.fronts[0].unite_with(&invader.state);
                    }
                }
                (None, None) => {
                    changed |= self.state.unite_with(&invader.state);
                    changed |= self.fronts[0].unite_with(&invader.fronts[0]);
                }
                (Some(_), Some(_)) => unreachable!("A node cannot be both opening and closing"),
            }

            return changed;
        }

        // K >= 3: bounded frontier-stack semantics
        let mut changed = false;

        match (self.opening, self.closing) {
            (Some(opening), None) => {
                // Opening:
                // - current frontier becomes this opening
                // - previous frontier shifts one level deeper
                // - deeper history shifts down
                changed |= self.state.unite_with(&invader.state);

                let opening_bit = Self::singleton(self.size, opening);
                changed |= self.fronts[0].unite_with(&opening_bit);

                for i in 1..depth {
                    changed |= self.fronts[i].unite_with(&invader.fronts[i - 1]);
                }
            }

            (None, Some(closing)) => {
                // Closing:
                // - only allowed if current frontier contains the matching symbol
                // - then restore the previous frontier history
                // - the deepest restored level falls back to `state`
                if invader.fronts[0].contains(closing) {
                    changed |= self.state.unite_with(&invader.state);

                    for i in 0..(depth - 1) {
                        changed |= self.fronts[i].unite_with(&invader.fronts[i + 1]);
                    }
                    changed |= self.fronts[depth - 1].unite_with(&invader.state);
                }
            }
            (None, None) => {
                // Ordinary / epsilon node: propagate everything unchanged
                changed |= self.state.unite_with(&invader.state);
                for i in 0..depth {
                    changed |= self.fronts[i].unite_with(&invader.fronts[i]);
                }
            }

            (Some(_), Some(_)) => unreachable!("A node cannot be both opening and closing"),
        }

        changed
    }

    fn unreachable_if_opposite(&self, opposite: &Self) -> bool {
        self.state.is_disjoint(&opposite.state)
    }
}
