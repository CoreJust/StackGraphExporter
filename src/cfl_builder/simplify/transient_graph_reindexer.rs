use std::collections::HashMap;

use crate::{
    cfl_builder::{
        progress_event::{ProgressEvent, ProgressMonitor},
        simplify::simplification_stats::SimplificationStats,
        transient_graph::{TGraph, TNodeIndex},
    },
    error::Result,
};

pub fn reindex_graph<F>(
    tgraph: &mut TGraph,
    progress_monitor: &mut ProgressMonitor<F>,
    simplification_stats: &mut SimplificationStats,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    progress_monitor.stage_total = tgraph.nodes.len();
    let mut reindexer = HashMap::new();
    for (i, n) in tgraph.nodes.iter().enumerate() {
        progress_monitor.emit_simplification_nth("Recalculating node indices", i)?;
        if n.to_be_removed() {
            simplification_stats.total_nodes_removed += 1;
            continue;
        }
        reindexer.insert(i as TNodeIndex, reindexer.len() as TNodeIndex);
    }
    let mut i = 0;
    tgraph.nodes.retain_mut(|n| {
        progress_monitor
            .emit_simplification_nth("Reindexing nodes", i)
            .expect("Oops");
        i += 1;
        if n.to_be_removed() {
            return false;
        }
        n.incoming.iter_mut().for_each(|i| *i = reindexer[i]);
        n.outcoming.iter_mut().for_each(|i| *i = reindexer[i]);
        true
    });
    Ok(())
}
