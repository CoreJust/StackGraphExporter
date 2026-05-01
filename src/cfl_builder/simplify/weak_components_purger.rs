use std::collections::HashSet;

use crate::{
    cfl_builder::{
        progress_event::{ProgressEvent, ProgressMonitor},
        simplify::simplification_stats::SimplificationStats,
        transient_graph::{TGraph, TNodeIndex},
    },
    error::Result,
};

// Note: standalone nodes are ignored since such nodes are to be removed anyways
fn purge_weak_components_if<F, CallBack>(
    tgraph: &mut TGraph,
    simplification_stats: &mut SimplificationStats,
    progress_monitor: &mut ProgressMonitor<F>,
    mut predicate: CallBack,
) -> Result<()>
where
    CallBack: FnMut(&mut TGraph, &[TNodeIndex]) -> bool,
    F: FnMut(ProgressEvent) -> Result<()>,
{
    progress_monitor.stage_total = tgraph.nodes.len();

    let n = tgraph.nodes.len();
    let mut visited = vec![false; n];
    let mut removed = 0;

    for start_idx in 0..n {
        progress_monitor.emit_simplification_nth("Purging weak components", start_idx)?;
        if visited[start_idx] {
            continue;
        }

        if tgraph.nodes[start_idx].to_be_removed() {
            visited[start_idx] = true;
            continue;
        }

        let mut stack = vec![start_idx as TNodeIndex];
        let mut component = Vec::new();

        visited[start_idx] = true;

        while let Some(current) = stack.pop() {
            component.push(current);

            let neighbours: Vec<TNodeIndex> = {
                let node = &tgraph.nodes[current as usize];
                node.incoming
                    .iter()
                    .chain(node.outcoming.iter())
                    .copied()
                    .collect()
            };

            for nb in neighbours {
                let nb_usize = nb as usize;
                if !visited[nb_usize] {
                    visited[nb_usize] = true;
                    stack.push(nb);
                }
            }
        }

        if predicate(tgraph, &component) {
            for &node_idx in &component {
                let node = &mut tgraph.nodes[node_idx as usize];
                removed += node.incoming.len();
                node.incoming.clear();
                node.outcoming.clear();
            }
            simplification_stats.weak_components_purged += 1;
            simplification_stats.weak_components_nodes_purged += component.len();
        }
    }
    tgraph.edges_count -= removed;
    Ok(())
}

// There are no paths if the component has not a single pair pshX ppX,
// where both Xs are real.
pub fn remove_weak_components_without_paths<F>(
    tgraph: &mut TGraph,
    progress_monitor: &mut ProgressMonitor<F>,
    simplification_stats: &mut SimplificationStats,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    purge_weak_components_if(
        tgraph,
        simplification_stats,
        progress_monitor,
        |tgraph, nodes| {
            let mut real_pushes = HashSet::new();
            let mut real_pops = HashSet::new();
            !nodes.iter().map(|&i| &tgraph.nodes[i as usize]).any(|n| {
                // Check if the node is real and has pair
                if !n.is_real() {
                    false
                } else {
                    let rule = n.rule().expect("Cannot be eps");
                    if n.is_push() {
                        real_pushes.insert(rule);
                        real_pops.contains(&rule)
                    } else {
                        real_pops.insert(rule);
                        real_pushes.contains(&rule)
                    }
                }
            })
        },
    )
}
