use crate::error::{Error, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum ReachabilityTestMode {
    None,
    Trivial,
    Single,
    Double,
    Custom(u8),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimplificationOptions {
    pub simplify: bool,
    pub transient_simplification_iterations: Option<usize>,
    pub eps_removal_tolerance: isize,

    // The following might affect performance signigicantly!
    pub remove_unreachable: ReachabilityTestMode,
}

const DEFAULT_EPS_REMOVAL_TOLERANCE: isize = 0;

impl SimplificationOptions {
    pub fn no_simpify() -> Self {
        Self {
            simplify: false,
            transient_simplification_iterations: None,
            eps_removal_tolerance: DEFAULT_EPS_REMOVAL_TOLERANCE,
            remove_unreachable: ReachabilityTestMode::None,
        }
    }

    pub fn simpify() -> Self {
        Self {
            simplify: true,
            transient_simplification_iterations: Some(2),
            eps_removal_tolerance: DEFAULT_EPS_REMOVAL_TOLERANCE,
            remove_unreachable: ReachabilityTestMode::Custom(3),
        }
    }

    pub fn make(
        simplify: bool,
        max_transient_simplification_iterations: Option<usize>,
        eps_removal_tolerance: Option<isize>,
        remove_unreachable_trivial: bool,
        remove_unreachable: bool,
        remove_unreachable_with_front: bool,
        remove_unreachable_deep: Option<u8>,
    ) -> Result<Self> {
        if remove_unreachable_deep
            .and_then(|d| Some(d > 8))
            .unwrap_or(false)
        {
            return Err(Error::InvalidArgument(format!(
                "remove-unreachable-deep <depth>: only depth up to 8 is allowed; received {}",
                remove_unreachable_deep.unwrap()
            )));
        }
        Ok(Self {
            simplify,
            transient_simplification_iterations: max_transient_simplification_iterations,
            eps_removal_tolerance: eps_removal_tolerance.unwrap_or(DEFAULT_EPS_REMOVAL_TOLERANCE),
            remove_unreachable: if remove_unreachable_with_front {
                ReachabilityTestMode::Double
            } else if remove_unreachable {
                ReachabilityTestMode::Single
            } else if remove_unreachable_trivial {
                ReachabilityTestMode::Trivial
            } else if let Some(depth) = remove_unreachable_deep {
                ReachabilityTestMode::Custom(depth)
            } else {
                ReachabilityTestMode::None
            },
        })
    }
}
