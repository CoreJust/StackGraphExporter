use fixedbitset::FixedBitSet;

use crate::core::CFLRuleIndex;

pub trait ReachabilityState {
    fn empty(size: usize) -> Self;
    fn from_opening(size: usize, opening: CFLRuleIndex) -> Self;
    fn from_closing(size: usize, closing: CFLRuleIndex) -> Self;

    // Invader is the state is is propagated upon us
    fn merge_with(&mut self, invader: &Self) -> bool;
    // Current for openings and the opposite is for closings (or vice versa)
    fn unreachable_if_opposite(&self, opposite: &Self) -> bool;
}

pub struct TrivialReachabilityState {
    state: FixedBitSet,
}

impl ReachabilityState for TrivialReachabilityState {
    fn empty(size: usize) -> Self {
        Self {
            state: FixedBitSet::with_capacity(size),
        }
    }

    fn from_opening(size: usize, opening: CFLRuleIndex) -> Self {
        let mut state = FixedBitSet::with_capacity(size);
        state.insert(opening);
        Self { state }
    }

    fn from_closing(size: usize, _: CFLRuleIndex) -> Self {
        Self {
            state: FixedBitSet::with_capacity(size),
        }
    }

    fn merge_with(&mut self, invader: &Self) -> bool {
        if self.state.is_superset(&invader.state) {
            false
        } else {
            self.state.union_with(&invader.state);
            true
        }
    }

    fn unreachable_if_opposite(&self, opposite: &Self) -> bool {
        self.state.is_disjoint(&opposite.state)
    }
}

pub struct DoubleReachabilityState {
    state: FixedBitSet,
    front: FixedBitSet, // Openings that can reach here immediately, solely by epsilon nodes
    opening: Option<CFLRuleIndex>,
    closing: Option<CFLRuleIndex>,
}

impl ReachabilityState for DoubleReachabilityState {
    fn empty(size: usize) -> Self {
        Self {
            state: FixedBitSet::with_capacity(size),
            front: FixedBitSet::with_capacity(size),
            opening: None,
            closing: None,
        }
    }

    fn from_opening(size: usize, opening: CFLRuleIndex) -> Self {
        let mut state = FixedBitSet::with_capacity(size);
        state.insert(opening);
        let front = state.clone();
        Self {
            state,
            front,
            opening: Some(opening),
            closing: None,
        }
    }

    fn from_closing(size: usize, closing: CFLRuleIndex) -> Self {
        Self {
            state: FixedBitSet::with_capacity(size),
            front: FixedBitSet::with_capacity(size),
            opening: None,
            closing: Some(closing),
        }
    }

    fn merge_with(&mut self, invader: &Self) -> bool {
        if let Some(_) = self.opening {
            // No front can reach an opening node - it overwrites it with itself
            if self.state.is_superset(&invader.state) {
                false
            } else {
                self.state.union_with(&invader.state);
                true
            }
        } else if let Some(closing) = self.closing {
            // If front has no openning corresponding to this closing,
            // then it cannot pass since a path will not be formed
            if invader.front[closing] == false {
                false
            } else {
                if self.state.is_superset(&invader.state) && self.front.is_superset(&invader.state)
                {
                    false
                } else {
                    self.state.union_with(&invader.state);
                    // We have no information about the front after crossing the
                    // closing node, it can be any opening from the state
                    self.front.union_with(&invader.state);
                    true
                }
            }
        } else {
            if self.state.is_superset(&invader.state) && self.front.is_superset(&invader.front) {
                false
            } else {
                self.state.union_with(&invader.state);
                self.front.union_with(&invader.front);
                true
            }
        }
    }

    fn unreachable_if_opposite(&self, opposite: &Self) -> bool {
        self.state.is_disjoint(&opposite.state)
    }
}
