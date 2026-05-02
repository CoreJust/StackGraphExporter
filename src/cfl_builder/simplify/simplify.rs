use crate::{
    cfl_builder::{
        progress_event::{ProgressEvent, ProgressMonitor},
        simplification_options::ReachabilityTestMode,
        simplify::{
            invalid_end_nodes_removal::remove_invalid_end_nodes,
            invalid_pairs_removal::remove_or_merge_invalid_pairs,
            reachability_test::{
                remove_unreachable, BitSetFixed, DoubleReachabilityState, KReachabilityState,
                SingleReachabilityState, TrivialReachabilityState,
            },
            simplification_stats::SimplificationStats,
            transient_graph_reindexer::reindex_graph,
            trivial_eps_removal::remove_trivial_eps_nodes,
            weak_components_purger::remove_weak_components_without_paths,
        },
        transient_graph::TGraph,
        SimplificationOptions,
    },
    error::{Error, Result},
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
            remove_or_merge_invalid_pairs(tgraph, progress_monitor, &mut stats)?;
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
        if simplification_options.remove_unreachable != ReachabilityTestMode::None {
            let old_unreachable_removed = stats.unreachable_nodes_removed;
            match simplification_options.remove_unreachable {
                ReachabilityTestMode::Trivial => remove_unreachable::<TrivialReachabilityState, _>(
                    tgraph,
                    progress_monitor,
                    &mut stats,
                ),
                ReachabilityTestMode::Single => remove_unreachable::<
                    SingleReachabilityState<BitSetFixed>,
                    _,
                >(
                    tgraph, progress_monitor, &mut stats
                ),
                ReachabilityTestMode::Double => remove_unreachable::<
                    DoubleReachabilityState<BitSetFixed>,
                    _,
                >(
                    tgraph, progress_monitor, &mut stats
                ),
                ReachabilityTestMode::Custom(depth) => match depth {
                    1 => remove_unreachable::<SingleReachabilityState<BitSetFixed>, _>(
                        tgraph,
                        progress_monitor,
                        &mut stats,
                    ),
                    2 => remove_unreachable::<DoubleReachabilityState<BitSetFixed>, _>(
                        tgraph,
                        progress_monitor,
                        &mut stats,
                    ),
                    3 => remove_unreachable::<KReachabilityState<BitSetFixed, 3>, _>(
                        tgraph,
                        progress_monitor,
                        &mut stats,
                    ),
                    4 => remove_unreachable::<KReachabilityState<BitSetFixed, 4>, _>(
                        tgraph,
                        progress_monitor,
                        &mut stats,
                    ),
                    5 => remove_unreachable::<KReachabilityState<BitSetFixed, 5>, _>(
                        tgraph,
                        progress_monitor,
                        &mut stats,
                    ),
                    6 => remove_unreachable::<KReachabilityState<BitSetFixed, 6>, _>(
                        tgraph,
                        progress_monitor,
                        &mut stats,
                    ),
                    7 => remove_unreachable::<KReachabilityState<BitSetFixed, 7>, _>(
                        tgraph,
                        progress_monitor,
                        &mut stats,
                    ),
                    8 => remove_unreachable::<KReachabilityState<BitSetFixed, 8>, _>(
                        tgraph,
                        progress_monitor,
                        &mut stats,
                    ),
                    _ => return Err(Error::InvalidArgument(format!("ReachabilityTestMode::Custom is allowed only till depth 8, but got {depth}"))),
                },
                ReachabilityTestMode::None => unreachable!(),
            }?;
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
