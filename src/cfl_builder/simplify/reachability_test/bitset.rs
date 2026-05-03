use std::sync::OnceLock;

use fixedbitset::FixedBitSet;
use roaring::RoaringBitmap;

pub trait BitSet: Send + Clone {
    fn empty(size: u32) -> Self;
    fn with_bits_set(size: u32, bits_list: &[u32]) -> Self;

    fn is_disjoint(&self, other: &Self) -> bool;
    #[allow(dead_code)]
    fn is_superset(&self, other: &Self) -> bool;
    fn unite_with(&mut self, other: &Self) -> bool; // True if anything changed
    fn contains(&self, bit: u32) -> bool;
    #[allow(dead_code)]
    fn insert(&mut self, bit: u32);
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct BitSetFixed {
    data: FixedBitSet,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct BitSetRoaring {
    data: RoaringBitmap,
}

impl BitSetFixed {
    #[inline(always)]
    fn unite_with_scalar(dst: &mut [usize], src: &[usize]) -> bool {
        debug_assert_eq!(dst.len(), src.len());

        let mut changed = false;
        let mut i = 0usize;
        let len = dst.len();

        unsafe {
            let dstp = dst.as_mut_ptr();
            let srcp = src.as_ptr();

            while i + 8 <= len {
                let old0 = *dstp.add(i + 0);
                let new0 = old0 | *srcp.add(i + 0);
                if new0 != old0 {
                    *dstp.add(i + 0) = new0;
                    changed = true;
                }

                let old1 = *dstp.add(i + 1);
                let new1 = old1 | *srcp.add(i + 1);
                if new1 != old1 {
                    *dstp.add(i + 1) = new1;
                    changed = true;
                }

                let old2 = *dstp.add(i + 2);
                let new2 = old2 | *srcp.add(i + 2);
                if new2 != old2 {
                    *dstp.add(i + 2) = new2;
                    changed = true;
                }

                let old3 = *dstp.add(i + 3);
                let new3 = old3 | *srcp.add(i + 3);
                if new3 != old3 {
                    *dstp.add(i + 3) = new3;
                    changed = true;
                }

                let old4 = *dstp.add(i + 4);
                let new4 = old4 | *srcp.add(i + 4);
                if new4 != old4 {
                    *dstp.add(i + 4) = new4;
                    changed = true;
                }

                let old5 = *dstp.add(i + 5);
                let new5 = old5 | *srcp.add(i + 5);
                if new5 != old5 {
                    *dstp.add(i + 5) = new5;
                    changed = true;
                }

                let old6 = *dstp.add(i + 6);
                let new6 = old6 | *srcp.add(i + 6);
                if new6 != old6 {
                    *dstp.add(i + 6) = new6;
                    changed = true;
                }

                let old7 = *dstp.add(i + 7);
                let new7 = old7 | *srcp.add(i + 7);
                if new7 != old7 {
                    *dstp.add(i + 7) = new7;
                    changed = true;
                }

                i += 8;
            }

            while i < len {
                let old = *dstp.add(i);
                let new = old | *srcp.add(i);
                if new != old {
                    *dstp.add(i) = new;
                    changed = true;
                }
                i += 1;
            }
        }

        changed
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn unite_with_avx2(dst: &mut [usize], src: &[usize]) -> bool {
        use core::arch::x86_64::*;

        debug_assert_eq!(dst.len(), src.len());

        let mut changed = false;
        let mut i = 0usize;
        let len = dst.len();

        while i + 4 <= len {
            let a = _mm256_loadu_si256(dst.as_ptr().add(i) as *const __m256i);
            let b = _mm256_loadu_si256(src.as_ptr().add(i) as *const __m256i);
            let c = _mm256_or_si256(a, b);

            _mm256_storeu_si256(dst.as_mut_ptr().add(i) as *mut __m256i, c);

            let diff = _mm256_xor_si256(a, c);
            if _mm256_testz_si256(diff, diff) == 0 {
                changed = true;
            }

            i += 4;
        }

        while i < len {
            let old = *dst.as_ptr().add(i);
            let new = old | *src.as_ptr().add(i);
            if new != old {
                *dst.as_mut_ptr().add(i) = new;
                changed = true;
            }
            i += 1;
        }

        changed
    }
}

type UnionFn = unsafe fn(&mut [usize], &[usize]) -> bool;
static UNION_FN: OnceLock<UnionFn> = OnceLock::new();

impl BitSet for BitSetFixed {
    fn empty(size: u32) -> Self {
        Self {
            data: FixedBitSet::with_capacity(size as usize),
        }
    }

    fn with_bits_set(size: u32, bits_list: &[u32]) -> Self {
        let mut result = FixedBitSet::with_capacity(size as usize);
        for &bit in bits_list {
            result.insert(bit as usize);
        }
        Self { data: result }
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        self.data.is_disjoint(&other.data)
    }

    fn is_superset(&self, other: &Self) -> bool {
        self.data.is_superset(&other.data)
    }

    #[inline]
    fn unite_with(&mut self, other: &Self) -> bool {
        assert!(self.data.len() == other.data.len());
        let f = UNION_FN.get_or_init(|| {
            #[cfg(target_arch = "x86_64")]
            if std::is_x86_feature_detected!("avx2") {
                return Self::unite_with_avx2 as UnionFn;
            }
            Self::unite_with_scalar as UnionFn
        });
        unsafe { f(self.data.as_mut_slice(), other.data.as_slice()) }
    }

    fn contains(&self, bit: u32) -> bool {
        self.data.contains(bit as usize)
    }

    fn insert(&mut self, bit: u32) {
        self.data.insert(bit as usize);
    }
}

impl BitSet for BitSetRoaring {
    fn empty(_: u32) -> Self {
        Self {
            data: RoaringBitmap::new(),
        }
    }

    fn with_bits_set(_: u32, bits_list: &[u32]) -> Self {
        let mut result = RoaringBitmap::new();
        for &bit in bits_list {
            result.insert(bit);
        }
        Self { data: result }
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        self.data.is_disjoint(&other.data)
    }

    fn is_superset(&self, other: &Self) -> bool {
        self.data.is_superset(&other.data)
    }

    fn unite_with(&mut self, other: &Self) -> bool {
        let before = self.data.len();
        self.data |= &other.data;
        self.data.len() != before
    }

    fn contains(&self, bit: u32) -> bool {
        self.data.contains(bit)
    }

    fn insert(&mut self, bit: u32) {
        self.data.insert(bit);
    }
}
