use crate::{
    cfl_builder::{
        progress_event::{ProgressEvent, ProgressMonitor},
        simplify::simplification_stats::SimplificationStats,
        transient_graph::{TGraph, TNodeIndex},
    },
    error::Result,
};

// Removes edges like pushX -> popY where X != Y.
pub fn remove_invalid_pairs<F>(
    tgraph: &mut TGraph,
    progress_monitor: &mut ProgressMonitor<F>,
    simplification_stats: &mut SimplificationStats,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    progress_monitor.stage_total = tgraph.nodes.len();
    let mut to_remove = Vec::new();
    for (i, node) in tgraph.nodes.iter().enumerate() {
        progress_monitor.emit_simplification_nth("Removing invalid push->pop pairs", i)?;
        if !node.is_push() {
            continue;
        }
        let rule = node.rule().unwrap();
        for &other in &node.outcoming {
            let other_node = &tgraph.nodes[other as usize];
            if other_node.is_pop() && other_node.rule().unwrap() != rule {
                to_remove.push((i as TNodeIndex, other));
                simplification_stats.invalid_pairs_removed += 1;
            }
        }
    }
    for (from, to) in to_remove {
        tgraph.remove_edge(from, to);
    }
    Ok(())
}
