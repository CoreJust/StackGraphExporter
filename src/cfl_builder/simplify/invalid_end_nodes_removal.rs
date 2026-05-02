use crate::{
    cfl_builder::{
        progress_event::{ProgressEvent, ProgressMonitor},
        simplify::simplification_stats::SimplificationStats,
        transient_graph::{TGraph, TNodeIndex},
    },
    error::Result,
};

// Idea:
// There may be nodes that have no incoming or outcoming
// edges. If it's virtual or push without outcoming
// or pop without incoming - no valid paths can exist
// with this node.
pub fn remove_invalid_end_nodes<F>(
    tgraph: &mut TGraph,
    progress_monitor: &mut ProgressMonitor<F>,
    simplification_stats: &mut SimplificationStats,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    progress_monitor.stage_total = tgraph.nodes.len();
    for i in 0..tgraph.nodes.len() {
        progress_monitor.emit_simplification_nth("Removing invalid end nodes", i)?;
        let to_be_removed = {
            let node = &tgraph.nodes[i];
            (node.incoming.is_empty() && (!node.is_real() || !node.is_push()))
                || (node.outcoming.is_empty() && (!node.is_real() || !node.is_pop()))
        };
        if to_be_removed {
            tgraph.remove_node(i as TNodeIndex);
            simplification_stats.invalid_end_nodes_removed += 1;
        }
    }
    Ok(())
}
