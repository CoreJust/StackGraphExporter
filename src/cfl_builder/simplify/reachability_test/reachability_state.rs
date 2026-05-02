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
