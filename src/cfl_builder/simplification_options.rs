#[derive(Clone, Debug, PartialEq)]
pub struct SimplificationOptions {
    pub simplify: bool,
    pub simplify_cfl: bool,
    pub transient_simplification_iterations: Option<usize>,
    pub eps_removal_tolerance: isize,

    // The following might affect performance signigicantly!
    pub remove_unreachable: bool,
    pub remove_unreachable_with_front: bool,
}

const DEFAULT_EPS_REMOVAL_TOLERANCE: isize = 5;

impl SimplificationOptions {
    pub fn no_simpify() -> Self {
        Self {
            simplify: false,
            simplify_cfl: false,
            transient_simplification_iterations: None,
            eps_removal_tolerance: DEFAULT_EPS_REMOVAL_TOLERANCE,
            remove_unreachable: false,
            remove_unreachable_with_front: false,
        }
    }

    pub fn simpify() -> Self {
        Self {
            simplify: true,
            simplify_cfl: true,
            transient_simplification_iterations: None,
            eps_removal_tolerance: DEFAULT_EPS_REMOVAL_TOLERANCE,
            remove_unreachable: true,
            remove_unreachable_with_front: true,
        }
    }

    pub fn make(
        simplify: bool,
        no_simplify_cfl: bool,
        no_simplify_transient: bool,
        max_transient_simplification_iterations: Option<usize>,
        eps_removal_tolerance: Option<isize>,
        remove_unreachable: bool,
        remove_unreachable_with_front: bool,
    ) -> Self {
        Self {
            simplify,
            simplify_cfl: !no_simplify_cfl,
            transient_simplification_iterations: if no_simplify_transient {
                Some(0)
            } else {
                max_transient_simplification_iterations
            },
            eps_removal_tolerance: eps_removal_tolerance.unwrap_or(DEFAULT_EPS_REMOVAL_TOLERANCE),
            remove_unreachable: remove_unreachable || remove_unreachable_with_front,
            remove_unreachable_with_front,
        }
    }
}
