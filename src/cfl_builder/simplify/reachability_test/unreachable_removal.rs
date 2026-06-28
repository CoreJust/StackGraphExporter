use fixedbitset::FixedBitSet;

use super::{reachability_state::ReachabilityState, scc::condense_with_scc};
use crate::{
    cfl_builder::{
        progress_event::{ProgressEvent, ProgressMonitor},
        simplify::{
            simplification_stats::SimplificationStats,
            transient_graph_walker::{BackwardTGraphWalker, ForwardTGraphWalker, TGraphWalker},
        },
        transient_graph::{TGraph, TNodeIndex},
    },
    error::Result,
};

struct NodeQueue {
    stack: Vec<TNodeIndex>,
    in_stack: FixedBitSet,
}

impl NodeQueue {
    fn new(size: usize) -> Self {
        Self {
            stack: Vec::with_capacity(size / 10),
            in_stack: FixedBitSet::with_capacity(size),
        }
    }

    fn push(&mut self, node: TNodeIndex) {
        if !self.in_stack.contains(node as usize) {
            self.stack.push(node);
            self.in_stack.insert(node as usize);
        }
    }

    fn pop(&mut self) -> Option<TNodeIndex> {
        let node = self.stack.pop()?;
        self.in_stack.remove(node as usize);
        Some(node)
    }
}

fn mark_initial_nodes_reachability<State, Walker>(
    tgraph: &TGraph,
) -> Result<(Vec<State>, NodeQueue)>
where
    State: ReachabilityState,
    Walker: TGraphWalker,
{
    let mut states = Vec::with_capacity(tgraph.nodes.len());
    let mut queue = NodeQueue::new(tgraph.nodes.len());
    for (i, n) in tgraph.nodes.iter().enumerate() {
        if let Some(rule) = Walker::as_opening(n) {
            states.push(State::from_opening(tgraph.cfl_push_pop_rules_count, rule));
            queue.push(i as TNodeIndex);
        } else if let Some(rule) = Walker::as_closing(n) {
            states.push(State::from_closing(tgraph.cfl_push_pop_rules_count, rule));
        } else {
            states.push(State::empty(tgraph.cfl_push_pop_rules_count));
        }
    }
    Ok((states, queue))
}

fn mark_initial_nodes_reachability_scc<State, Walker>(
    tgraph: &TGraph,
    condensed: &TGraph,
    mapping_back: &[Vec<TNodeIndex>],
) -> Result<(Vec<State>, NodeQueue)>
where
    State: ReachabilityState,
    Walker: TGraphWalker,
{
    let scc_count = condensed.nodes.len();
    let mut states = Vec::with_capacity(scc_count);
    let mut queue = NodeQueue::new(scc_count);

    let mut openings = Vec::new();
    let mut closings = Vec::new();
    for (scc_idx, members) in mapping_back.iter().enumerate() {
        openings.clear();
        closings.clear();
        for &orig_node in members {
            let node = &tgraph.nodes[orig_node as usize];
            if let Some(rule) = Walker::as_opening(node) {
                openings.push(rule);
            } else if let Some(rule) = Walker::as_closing(node) {
                closings.push(rule);
            }
        }

        if !openings.is_empty() {
            queue.push(scc_idx as TNodeIndex);
        }
        states.push(State::from_scc(
            tgraph.cfl_push_pop_rules_count,
            &openings,
            &closings,
        ));
    }

    Ok((states, queue))
}

// Marks nodes with bitsets of what opening symbols can reach them.
fn mark_nodes_reachability<State, Walker>(
    tgraph: &TGraph,
    mut state: Vec<State>,
    mut queue: NodeQueue,
) -> Result<Vec<State>>
where
    State: ReachabilityState,
    Walker: TGraphWalker,
{
    while let Some(current) = queue.pop() {
        let node = &tgraph.nodes[current as usize];
        let next = Walker::next_vertices(node);
        let (left, right) = state.split_at_mut(current as usize);
        let (local_state, right) = right.split_first_mut().unwrap();
        for &target in next {
            if target == current {
                continue;
            }
            let target_state = if target < current {
                &mut left[target as usize]
            } else {
                &mut right[(target - current - 1) as usize]
            };
            if target_state.merge_with(&local_state) {
                queue.push(target);
            }
        }
    }
    Ok(state)
}

fn mark_nodes_reachability_maybe_scc<State, Walker>(
    tgraph: &TGraph,
    condensed: &Option<TGraph>,
    mapping_back: &[Vec<TNodeIndex>],
) -> Result<Vec<State>>
where
    State: ReachabilityState,
    Walker: TGraphWalker,
{
    if let Some(condensed) = condensed.as_ref() {
        let (state, queue) =
            mark_initial_nodes_reachability_scc::<State, Walker>(tgraph, condensed, mapping_back)?;
        mark_nodes_reachability::<State, Walker>(condensed, state, queue)
    } else {
        let (state, queue) = mark_initial_nodes_reachability::<State, Walker>(tgraph)?;
        mark_nodes_reachability::<State, Walker>(tgraph, state, queue)
    }
}

fn compute_push_pop_reachability_parallel<State>(
    tgraph: &TGraph,
    condensed: &Option<TGraph>,
    mapping_back: &[Vec<TNodeIndex>],
) -> Result<(Vec<State>, Vec<State>)>
where
    State: ReachabilityState + Send,
{
    use std::sync::mpsc;

    let (result_tx, result_rx) = mpsc::channel::<(usize, Result<Vec<State>>)>();

    rayon::scope(|s| {
        // PUSH
        {
            let rtx = result_tx.clone();
            s.spawn(move |_| {
                let res = mark_nodes_reachability_maybe_scc::<State, ForwardTGraphWalker>(
                    tgraph,
                    condensed,
                    mapping_back,
                );
                let _ = rtx.send((0, res));
            });
        }

        // POP
        {
            let rtx = result_tx.clone();
            s.spawn(move |_| {
                let res = mark_nodes_reachability_maybe_scc::<State, BackwardTGraphWalker>(
                    tgraph,
                    condensed,
                    mapping_back,
                );
                let _ = rtx.send((1, res));
            });
        }
    });

    drop(result_tx);

    // MAIN THREAD: collect results
    let mut results: [Option<Vec<State>>; 2] = [None, None];
    let mut errors: [Option<_>; 2] = [None, None];

    let mut remaining = 2;

    while remaining > 0 {
        if let Ok((branch, res)) = result_rx.try_recv() {
            match res {
                Ok(v) => results[branch] = Some(v),
                Err(e) => errors[branch] = Some(e),
            }
            remaining -= 1;
            continue;
        }
    }

    if let Some(e) = errors[0].take() {
        return Err(e);
    }
    if let Some(e) = errors[1].take() {
        return Err(e);
    }

    Ok((results[0].take().unwrap(), results[1].take().unwrap()))
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
    let (mut condensed_tgraph, mapping_back) = if State::MONOTONOUS {
        let (tgraph, mapping_back) = condense_with_scc(tgraph, progress_monitor)?;
        (Some(tgraph), mapping_back)
    } else {
        (None, Vec::new())
    };
    let (push_reachable, pop_reachable) =
        compute_push_pop_reachability_parallel::<State>(tgraph, &condensed_tgraph, &mapping_back)?;
    let walked_tgraph = condensed_tgraph.as_mut().unwrap_or(tgraph);
    progress_monitor.stage_total = walked_tgraph.nodes.len();
    for i in 0..walked_tgraph.nodes.len() {
        progress_monitor.emit_simplification_nth("Removing unreachable nodes", i)?;
        if push_reachable[i].unreachable_if_opposite(&pop_reachable[i]) {
            if State::MONOTONOUS {
                for &original_node_idx in &mapping_back[i] {
                    tgraph.tear_out_node(original_node_idx as TNodeIndex);
                    simplification_stats.unreachable_nodes_removed += 1;
                }
            } else {
                tgraph.tear_out_node(i as TNodeIndex);
                simplification_stats.unreachable_nodes_removed += 1;
            }
        }
    }
    Ok(())
}
