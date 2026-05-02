use std::collections::VecDeque;

use fixedbitset::FixedBitSet;

use super::{reachability_state::ReachabilityState, scc::condense_with_scc};
use crate::{
    cfl_builder::{
        progress_event::{ProgressEvent, ProgressMonitor},
        simplify::{
            reachability_test::nodes_marking_progress::{BranchProgress, ProgressMsg},
            simplification_stats::SimplificationStats,
            transient_graph_walker::{BackwardTGraphWalker, ForwardTGraphWalker, TGraphWalker},
        },
        transient_graph::{TGraph, TNodeIndex},
    },
    error::Result,
};

struct NodeQueue {
    queue: VecDeque<TNodeIndex>,
    queued: FixedBitSet,
}

impl NodeQueue {
    fn new(size: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            queued: FixedBitSet::with_capacity(size),
        }
    }

    fn push(&mut self, node: TNodeIndex) {
        if !self.queued.contains(node as usize) {
            self.queue.push_front(node);
            self.queued.insert(node as usize);
        }
    }

    fn pop(&mut self) -> Option<TNodeIndex> {
        let result = self.queue.pop_back()?;
        self.queued.remove(result as usize);
        Some(result)
    }
}

fn mark_initial_nodes_reachability<State, Walker>(
    tgraph: &TGraph,
    progress: &mut BranchProgress,
) -> Result<(Vec<State>, NodeQueue)>
where
    State: ReachabilityState,
    Walker: TGraphWalker,
{
    let mut states = Vec::with_capacity(tgraph.nodes.len());
    let mut queue = NodeQueue::new(tgraph.nodes.len());
    progress.set_total(tgraph.nodes.len());
    for (i, n) in tgraph.nodes.iter().enumerate() {
        progress.tick("Initializing nodes reachability", i);
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
    progress: &mut BranchProgress,
) -> Result<(Vec<State>, NodeQueue)>
where
    State: ReachabilityState,
    Walker: TGraphWalker,
{
    let scc_count = condensed.nodes.len();
    let mut states = Vec::with_capacity(scc_count);
    let mut queue = NodeQueue::new(scc_count);

    progress.set_total(tgraph.nodes.len());
    let mut processed_originals = 0usize;

    let mut openings = Vec::new();
    let mut closings = Vec::new();
    for (scc_idx, members) in mapping_back.iter().enumerate() {
        openings.clear();
        closings.clear();
        for &orig_node in members {
            processed_originals += 1;
            progress.tick(
                "Initializing nodes reachability with SCC",
                processed_originals,
            );

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
    progress: &mut BranchProgress,
) -> Result<Vec<State>>
where
    State: ReachabilityState,
    Walker: TGraphWalker,
{
    let mut visited_nodes = 0usize;
    while let Some(current) = queue.pop() {
        progress.tick("Marking nodes reachability", visited_nodes);
        progress.set_total(visited_nodes + queue.queue.len());
        visited_nodes += 1;

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
    progress: &mut BranchProgress,
) -> Result<Vec<State>>
where
    State: ReachabilityState,
    Walker: TGraphWalker,
{
    if let Some(condensed) = condensed.as_ref() {
        let (state, queue) = mark_initial_nodes_reachability_scc::<State, Walker>(
            tgraph,
            condensed,
            mapping_back,
            progress,
        )?;
        mark_nodes_reachability::<State, Walker>(condensed, state, queue, progress)
    } else {
        let (state, queue) = mark_initial_nodes_reachability::<State, Walker>(tgraph, progress)?;
        mark_nodes_reachability::<State, Walker>(tgraph, state, queue, progress)
    }
}

fn compute_push_pop_reachability_parallel<State, F>(
    tgraph: &TGraph,
    condensed: &Option<TGraph>,
    mapping_back: &[Vec<TNodeIndex>],
    progress_monitor: &mut ProgressMonitor<F>,
) -> Result<(Vec<State>, Vec<State>)>
where
    State: ReachabilityState + Send,
    F: FnMut(ProgressEvent) -> Result<()>,
{
    use std::sync::mpsc;

    let (progress_tx, progress_rx) = mpsc::channel::<ProgressMsg>();
    let (result_tx, result_rx) = mpsc::channel::<(usize, Result<Vec<State>>)>();

    rayon::scope(|s| {
        // PUSH
        {
            let tx = progress_tx.clone();
            let rtx = result_tx.clone();
            s.spawn(move |_| {
                let mut p = BranchProgress::new(tx, 0);
                let res = mark_nodes_reachability_maybe_scc::<State, ForwardTGraphWalker>(
                    tgraph,
                    condensed,
                    mapping_back,
                    &mut p,
                );
                p.finish("Marking nodes reachability", 0);
                let _ = rtx.send((0, res));
            });
        }

        // POP
        {
            let tx = progress_tx.clone();
            let rtx = result_tx.clone();
            s.spawn(move |_| {
                let mut p = BranchProgress::new(tx, 1);
                let res = mark_nodes_reachability_maybe_scc::<State, BackwardTGraphWalker>(
                    tgraph,
                    condensed,
                    mapping_back,
                    &mut p,
                );
                p.finish("Marking nodes reachability", 0);
                let _ = rtx.send((1, res));
            });
        }
    });

    drop(progress_tx);
    drop(result_tx);

    // MAIN THREAD: collect results
    let mut totals = [0usize; 2];
    let mut done = [0usize; 2];

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

        match progress_rx.recv() {
            Ok(msg) => match msg {
                ProgressMsg::SetTotal { branch, total } => {
                    totals[branch] = total;
                }
                ProgressMsg::Tick {
                    branch,
                    done: d,
                    label,
                } => {
                    done[branch] = d;

                    progress_monitor.stage_total = totals[0] + totals[1];
                    progress_monitor.emit_simplification_nth(label, done[0] + done[1])?;
                }
                ProgressMsg::Finished { .. } => {}
            },
            Err(_) => break,
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
    let (push_reachable, pop_reachable) = compute_push_pop_reachability_parallel::<State, _>(
        tgraph,
        &condensed_tgraph,
        &mapping_back,
        progress_monitor,
    )?;
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
