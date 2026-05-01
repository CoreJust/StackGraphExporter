use std::collections::HashSet;

use crate::{
    cfl_builder::{
        progress_event::{ProgressEvent, ProgressMonitor},
        simplify::{
            reachability_state::ReachabilityState,
            simplification_stats::SimplificationStats,
            transient_graph_walker::{BackwardTGraphWalker, ForwardTGraphWalker, TGraphWalker},
        },
        transient_graph::{TGraph, TNodeIndex},
    },
    error::Result,
};

fn mark_initial_nodes_reachability<State, Walker, F>(
    tgraph: &mut TGraph,
    progress_monitor: &mut ProgressMonitor<F>,
) -> Result<(Vec<State>, HashSet<TNodeIndex>)>
where
    State: ReachabilityState,
    Walker: TGraphWalker,
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut states = Vec::with_capacity(tgraph.nodes.len());
    let mut queue = HashSet::new();
    let size = tgraph.nodes.len();
    progress_monitor.stage_total = size;
    for (i, n) in tgraph.nodes.iter().enumerate() {
        progress_monitor.emit_simplification_nth("Initializing nodes reachability", i)?;
        if let Some(rule) = Walker::as_opening(n) {
            states.push(State::from_opening(size, rule));
            queue.insert(i as TNodeIndex);
        } else if let Some(rule) = Walker::as_closing(n) {
            states.push(State::from_closing(size, rule));
        } else {
            states.push(State::empty(size));
        }
    }
    Ok((states, queue))
}

// Marks nodes with bitsets of what opening symbols can reach them.
fn mark_nodes_reachability<State, Walker, F>(
    tgraph: &mut TGraph,
    progress_monitor: &mut ProgressMonitor<F>,
) -> Result<Vec<State>>
where
    State: ReachabilityState,
    Walker: TGraphWalker,
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let (mut state, mut queue) =
        mark_initial_nodes_reachability::<State, Walker, _>(tgraph, progress_monitor)?;
    let mut visited_nodes = 0usize;
    while let Some(&current) = queue.iter().next() {
        queue.remove(&current);
        progress_monitor.emit_simplification_nth("Marking nodes reachability", visited_nodes)?;
        progress_monitor.stage_total = visited_nodes + queue.len();
        visited_nodes += 1;

        let node = &tgraph.nodes[current as usize];
        let next = Walker::next_vertices(node);
        for &target in next {
            let (local_state, target_state) = {
                let (left, right) = state.split_at_mut(current as usize);
                let (local, right) = right.split_first_mut().unwrap();
                let target = if target < current {
                    &mut left[target as usize]
                } else {
                    &mut right[(target - current - 1) as usize]
                };
                (local, target)
            };
            if target_state.merge_with(&local_state) {
                queue.insert(target);
            }
        }
    }
    Ok(state)
}

// Removes nodes for which there is no symbol X such that the node
// is reachable from pushX and popX is reachable from that node.
pub fn remove_unreachable<State, F>(
    tgraph: &mut TGraph,
    progress_monitor: &mut ProgressMonitor<F>,
    simplification_stats: &mut SimplificationStats,
) -> Result<()>
where
    State: ReachabilityState,
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let push_reachable =
        mark_nodes_reachability::<State, ForwardTGraphWalker, _>(tgraph, progress_monitor)?;
    let pop_reachable =
        mark_nodes_reachability::<State, BackwardTGraphWalker, _>(tgraph, progress_monitor)?;
    progress_monitor.stage_total = tgraph.nodes.len();
    for i in 0..tgraph.nodes.len() {
        progress_monitor.emit_simplification_nth("Removing unreachable nodes", i)?;
        if push_reachable[i].unreachable_if_opposite(&pop_reachable[i]) {
            tgraph.tear_out_node(i as TNodeIndex);
            simplification_stats.unreachable_nodes_removed += 1;
        }
    }
    Ok(())
}
