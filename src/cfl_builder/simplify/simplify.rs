use crate::{
    cfl_builder::{
        progress_event::{ProgressEvent, ProgressMonitor},
        simplify::{
            invalid_end_nodes_removal::remove_invalid_end_nodes,
            invalid_pairs_removal::remove_invalid_pairs,
            reachability_state::{DoubleReachabilityState, TrivialReachabilityState},
            simplification_stats::SimplificationStats,
            transient_graph_reindexer::reindex_graph,
            trivial_eps_removal::remove_trivial_eps_nodes,
            unreachable_removal::remove_unreachable,
            weak_components_purger::remove_weak_components_without_paths,
        },
        transient_graph::TGraph,
        SimplificationOptions,
    },
    error::Result,
};

pub fn simplify_transient_graph<F>(
    tgraph: &mut TGraph,
    simplification_options: &SimplificationOptions,
    progress_monitor: &mut ProgressMonitor<F>,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut stats = SimplificationStats::new();
    progress_monitor.simplification_iteration = 0;
    loop {
        loop {
            let old_stats = stats.clone();
            let old_edges_count = tgraph.edges_count;
            remove_invalid_pairs(tgraph, progress_monitor, &mut stats)?;
            remove_trivial_eps_nodes(
                tgraph,
                simplification_options.eps_removal_tolerance,
                progress_monitor,
                &mut stats,
            )?;
            remove_invalid_end_nodes(tgraph, progress_monitor, &mut stats)?;
            remove_weak_components_without_paths(tgraph, progress_monitor, &mut stats)?;
            reindex_graph(tgraph, progress_monitor, &mut stats)?;

            stats.iterations += 1;
            progress_monitor.simplification_iteration += 1;
            stats.total_edges_removed += old_edges_count - tgraph.edges_count;
            if old_stats == stats
                || simplification_options
                    .transient_simplification_iterations
                    .and_then(|limit| Some(stats.iterations >= limit))
                    .unwrap_or(false)
            {
                break;
            }
        }

        // Unreachable removal is relatively heavier than the other heuristics, thus
        // we do it only after the other heuristics are done.
        if simplification_options.remove_unreachable {
            let old_unreachable_removed = stats.unreachable_nodes_removed;
            if simplification_options.remove_unreachable_with_front {
                remove_unreachable::<DoubleReachabilityState, _>(
                    tgraph,
                    progress_monitor,
                    &mut stats,
                )?;
            } else {
                remove_unreachable::<TrivialReachabilityState, _>(
                    tgraph,
                    progress_monitor,
                    &mut stats,
                )?;
            }
            if old_unreachable_removed != stats.unreachable_nodes_removed {
                if simplification_options
                    .transient_simplification_iterations
                    .and_then(|limit| Some(stats.iterations >= limit))
                    .unwrap_or(false)
                {
                    reindex_graph(tgraph, progress_monitor, &mut stats)?;
                    break;
                }
                continue; // Only continue if there was some effect from unreachable removal
            } else {
                break;
            }
        } else {
            break;
        }
    }
    crate::info!("Transient simplification stats: {stats}");
    Ok(())
}
