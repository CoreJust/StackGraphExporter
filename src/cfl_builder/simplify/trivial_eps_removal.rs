use crate::{
    cfl_builder::{
        progress_event::{ProgressEvent, ProgressMonitor},
        simplify::simplification_stats::SimplificationStats,
        transient_graph::{TGraph, TNodeIndex},
    },
    error::Result,
};

fn edges_increase_from_removing(incoming: usize, outcoming: usize) -> isize {
    (incoming as isize * outcoming as isize) - (incoming as isize + outcoming as isize)
}

// Removes epsilon nodes where there is either:
// 1. No more than 1 incoming edges
// 2. No more than 1 outcoming edges
// 3. Strictly 2 incoming and outcoming edges
pub fn remove_trivial_eps_nodes<F>(
    tgraph: &mut TGraph,
    tolerable_increase: isize,
    progress_monitor: &mut ProgressMonitor<F>,
    simplification_stats: &mut SimplificationStats,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut needs_more_iterations = true;
    while needs_more_iterations {
        needs_more_iterations = false;
        progress_monitor.stage_total = tgraph.nodes.len();
        for i in 0..tgraph.nodes.len() {
            progress_monitor.emit_simplification_nth("Removing trivial epsilon nodes", i)?;
            let to_be_removed = {
                let node = &tgraph.nodes[i];
                node.is_eps()
                    && edges_increase_from_removing(node.incoming.len(), node.outcoming.len())
                        <= tolerable_increase
                    && !node.to_be_removed() // If it's already to be removed - there's no use in doing it again
            };
            if to_be_removed {
                tgraph.remove_node(i as TNodeIndex);
                simplification_stats.trivial_eps_removed += 1;
                needs_more_iterations = true;
            }
        }
        simplification_stats.trivial_eps_removal_iterations += 1;
    }
    Ok(())
}
