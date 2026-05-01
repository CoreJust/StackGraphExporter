use std::fmt::{Display, Formatter};

#[derive(Default, Clone)]
pub struct SimplificationStats {
    pub iterations: usize,
    pub total_nodes_removed: usize,
    pub total_edges_removed: usize,
    pub invalid_pairs_removed: usize,
    pub invalid_end_nodes_removed: usize,
    pub trivial_eps_removed: usize,
    pub trivial_eps_removal_iterations: usize,
    pub weak_components_purged: usize,
    pub weak_components_nodes_purged: usize,
    pub unreachable_nodes_removed: usize,
}

impl SimplificationStats {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl PartialEq for SimplificationStats {
    fn eq(&self, other: &Self) -> bool {
        self.total_nodes_removed == other.total_nodes_removed
            && self.total_edges_removed == other.total_edges_removed
    }
}

impl Display for SimplificationStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
\tIterations: {}
\tTotal removed: {} nodes and {} edges
\tInvalid pairs removed: {}
\tInvalid end nodes removed: {}
\tTrivial eps removed: {} (over {} iterations in total)
\tWeak components purged: {} ({} nodes)
\tUnreachable nodes removed: {}",
            self.iterations,
            self.total_nodes_removed,
            self.total_edges_removed,
            self.invalid_pairs_removed,
            self.invalid_end_nodes_removed,
            self.trivial_eps_removed,
            self.trivial_eps_removal_iterations,
            self.weak_components_purged,
            self.weak_components_nodes_purged,
            self.unreachable_nodes_removed,
        )
    }
}
