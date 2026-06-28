use fixedbitset::FixedBitSet;

use crate::{
    cfl_builder::{
        progress_event::{ProgressEvent, ProgressMonitor},
        simplify::simplification_stats::SimplificationStats,
        transient_graph::{TGraph, TNodeIndex},
    },
    error::Result,
};

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

    let mut stack = Vec::with_capacity(1024);
    let mut component = Vec::with_capacity(1024);

    for start_idx in 0..n {
        progress_monitor.emit_simplification_nth("Purging weak components", start_idx)?;
        if visited[start_idx] {
            continue;
        }
        if tgraph.nodes[start_idx].to_be_removed() {
            visited[start_idx] = true;
            continue;
        }

        visited[start_idx] = true;
        stack.clear();
        component.clear();
        stack.push(start_idx as TNodeIndex);

        while let Some(current) = stack.pop() {
            component.push(current);
            let node = &tgraph.nodes[current as usize];
            for &nb in node.incoming.iter().chain(node.outcoming.iter()) {
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

pub fn remove_weak_components_without_paths<F>(
    tgraph: &mut TGraph,
    progress_monitor: &mut ProgressMonitor<F>,
    simplification_stats: &mut SimplificationStats,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let n = tgraph.nodes.len();

    let mut node_info: Vec<Option<(bool, usize)>> = vec![None; n];
    let mut max_rule = 0;
    for (i, node) in tgraph.nodes.iter().enumerate() {
        if node.is_real() {
            let rule = node.rule().expect("Cannot be eps") as usize;
            node_info[i] = Some((node.is_push(), rule));
            if rule > max_rule {
                max_rule = rule;
            }
        }
    }

    // Bitsets reused across components
    let mut push_bits = FixedBitSet::with_capacity(max_rule + 1);
    let mut pop_bits = FixedBitSet::with_capacity(max_rule + 1);

    purge_weak_components_if(
        tgraph,
        simplification_stats,
        progress_monitor,
        |_tgraph, nodes| {
            push_bits.clear();
            pop_bits.clear();
            for &idx in nodes {
                if let Some((is_push, rule)) = node_info[idx as usize] {
                    if is_push {
                        if pop_bits.contains(rule) {
                            return false;
                        }
                        push_bits.insert(rule);
                    } else {
                        if push_bits.contains(rule) {
                            return false;
                        }
                        pop_bits.insert(rule);
                    }
                }
            }
            true
        },
    )
}
