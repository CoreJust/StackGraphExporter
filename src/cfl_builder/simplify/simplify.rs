use crate::{
    cfl_builder::{
        progress_event::{ProgressEvent, ProgressMonitor},
        simplification_options::ReachabilityTestMode,
        simplify::{
            complete_eps_removal::remove_all_eps_nodes,
            invalid_end_nodes_removal::remove_invalid_end_nodes,
            invalid_pairs_removal::remove_invalid_pairs,
            reachability_test::{
                remove_unreachable, BitSetFixed, KReachabilityState, PackedDoubleReachabilityState,
                SingleReachabilityState, TrivialReachabilityState,
            },
            simplification_stats::SimplificationStats,
            transient_graph_reindexer::{reindex_graph, reindex_graph_rules},
            trivial_eps_removal::remove_trivial_eps_nodes,
            weak_components_purger::remove_weak_components_without_paths,
        },
        transient_graph::TGraph,
        SimplificationOptions,
    },
    error::{Error, Result},
    io::measure::Measurer,
};

pub fn simplify_by_removing_unreachable<F>(
    tgraph: &mut TGraph,
    reachability_test_mode: ReachabilityTestMode,
    stats: &mut SimplificationStats,
    progress_monitor: &mut ProgressMonitor<F>,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    match reachability_test_mode {
        ReachabilityTestMode::Trivial => {
            remove_unreachable::<TrivialReachabilityState, _>(tgraph, progress_monitor, stats)
        }
        ReachabilityTestMode::Single => {
            remove_unreachable::<SingleReachabilityState<BitSetFixed>, _>(
                tgraph,
                progress_monitor,
                stats,
            )
        }
        ReachabilityTestMode::Double => {
            remove_unreachable::<PackedDoubleReachabilityState, _>(tgraph, progress_monitor, stats)
        }
        ReachabilityTestMode::Custom(depth) => match depth {
            1 => remove_unreachable::<SingleReachabilityState<BitSetFixed>, _>(
                tgraph,
                progress_monitor,
                stats,
            ),
            2 => remove_unreachable::<PackedDoubleReachabilityState, _>(
                tgraph,
                progress_monitor,
                stats,
            ),
            3 => remove_unreachable::<KReachabilityState<BitSetFixed, 3>, _>(
                tgraph,
                progress_monitor,
                stats,
            ),
            4 => remove_unreachable::<KReachabilityState<BitSetFixed, 4>, _>(
                tgraph,
                progress_monitor,
                stats,
            ),
            5 => remove_unreachable::<KReachabilityState<BitSetFixed, 5>, _>(
                tgraph,
                progress_monitor,
                stats,
            ),
            6 => remove_unreachable::<KReachabilityState<BitSetFixed, 6>, _>(
                tgraph,
                progress_monitor,
                stats,
            ),
            7 => remove_unreachable::<KReachabilityState<BitSetFixed, 7>, _>(
                tgraph,
                progress_monitor,
                stats,
            ),
            8 => remove_unreachable::<KReachabilityState<BitSetFixed, 8>, _>(
                tgraph,
                progress_monitor,
                stats,
            ),
            _ => {
                return Err(Error::InvalidArgument(format!(
                    "ReachabilityTestMode::Custom is allowed only till depth 8, but got {depth}"
                )))
            }
        },
        ReachabilityTestMode::None => unreachable!(),
    }
}

pub fn simplify_transient_graph<F>(
    tgraph: &mut TGraph,
    simplification_options: &SimplificationOptions,
    progress_monitor: &mut ProgressMonitor<F>,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut stats = SimplificationStats::new();
    let mut m = Measurer::new();
    progress_monitor.simplification_iteration = 0;
    if simplification_options.eps_removal_tolerance == isize::MAX {
        let old_edges_count = tgraph.edges_count;
        m.measure("remove_all_eps_nodes", || {
            remove_all_eps_nodes(tgraph, progress_monitor, &mut stats)
        })?;
        stats.total_edges_removed += old_edges_count - tgraph.edges_count;
        m.measure("reindex_graph", || {
            reindex_graph(tgraph, progress_monitor, &mut stats)
        })?;
    }
    loop {
        loop {
            let old_stats = stats.clone();
            let old_edges_count = tgraph.edges_count;
            if simplification_options.eps_removal_tolerance != isize::MAX {
                m.measure("remove_trivial_eps_nodes", || {
                    remove_trivial_eps_nodes(
                        tgraph,
                        simplification_options.eps_removal_tolerance,
                        progress_monitor,
                        &mut stats,
                    )
                })?;
            }
            m.measure("remove_invalid_pairs", || {
                remove_invalid_pairs(tgraph, progress_monitor, &mut stats)
            })?;
            m.measure("remove_invalid_end_nodes", || {
                remove_invalid_end_nodes(tgraph, progress_monitor, &mut stats)
            })?;
            m.measure("remove_weak_components_without_paths", || {
                remove_weak_components_without_paths(tgraph, progress_monitor, &mut stats)
            })?;
            m.measure("reindex_graph", || {
                reindex_graph(tgraph, progress_monitor, &mut stats)
            })?;

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
            m.measure("reindex_graph_rules", || {
                reindex_graph_rules(tgraph, progress_monitor, &mut stats)
            })?;
            m.measure("simplify_by_removing_unreachable", || {
                simplify_by_removing_unreachable(
                    tgraph,
                    simplification_options.remove_unreachable.clone(),
                    &mut stats,
                    progress_monitor,
                )
            })?;
            if old_unreachable_removed != stats.unreachable_nodes_removed {
                if simplification_options
                    .transient_simplification_iterations
                    .and_then(|limit| Some(stats.iterations >= limit))
                    .unwrap_or(false)
                {
                    m.measure("reindex_graph", || {
                        reindex_graph(tgraph, progress_monitor, &mut stats)
                    })?;
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
    m.measure("reindex_graph_rules", || {
        reindex_graph_rules(tgraph, progress_monitor, &mut stats)
    })?;
    crate::info!("Transient simplification stats: {stats}");
    m.dump();
    Ok(())
}
