use super::bitset::BitSet;
use crate::core::CFLRuleIndex;

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

#[derive(Clone)]
pub struct DoubleReachabilityState<T: BitSet> {
    deep_stack: T,               // Emulates stack depths of 2 and more
    front: T, // Openings that can reach here immediately, solely by epsilon nodes
    is_reached_by_opening: bool, // Marks a closing node as reachable
    opening: Option<CFLRuleIndex>,
    closing: Option<CFLRuleIndex>,
}

impl<T: BitSet> ReachabilityState for DoubleReachabilityState<T> {
    const MONOTONOUS: bool = false;

    fn empty(size: u32) -> Self {
        Self {
            deep_stack: T::empty(size),
            front: T::empty(size),
            is_reached_by_opening: false,
            opening: None,
            closing: None,
        }
    }

    fn from_opening(size: u32, opening: CFLRuleIndex) -> Self {
        Self {
            deep_stack: T::empty(size),
            front: T::with_bits_set(size, &[opening]),
            is_reached_by_opening: false,
            opening: Some(opening),
            closing: None,
        }
    }

    fn from_closing(size: u32, closing: CFLRuleIndex) -> Self {
        Self {
            deep_stack: T::empty(size),
            front: T::empty(size),
            is_reached_by_opening: false,
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
            let changed_deep = self.deep_stack.unite_with(&invader.deep_stack);
            // Old front advances to the deep_stack due to new push
            let changed_from_front = self.deep_stack.unite_with(&invader.front);
            changed_from_front || changed_deep
        } else if let Some(closing) = self.closing {
            // If front has no openning corresponding to this closing,
            // then it cannot pass since a path will not be formed
            if invader.front.contains(closing) == false {
                false
            } else {
                // The front will not propagate further, but we have to make sure this
                // node won't be pruned.
                self.is_reached_by_opening = true;
                let changed_deep = self.deep_stack.unite_with(&invader.deep_stack);
                // We have no information about the front after crossing the
                // closing node, it can be any opening from the deeper stack
                let changed_front = self.front.unite_with(&invader.deep_stack);
                changed_deep || changed_front
            }
        } else {
            let changed_deep = self.deep_stack.unite_with(&invader.deep_stack);
            let changed_front = self.front.unite_with(&invader.front);
            changed_deep || changed_front
        }
    }

    fn unreachable_if_opposite(&self, opposite: &Self) -> bool {
        self.deep_stack.is_disjoint(&opposite.deep_stack)
            && self.front.is_disjoint(&opposite.front)
            && !self.is_reached_by_opening
            && !opposite.is_reached_by_opening
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
