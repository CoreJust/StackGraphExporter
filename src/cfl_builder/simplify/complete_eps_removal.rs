use crate::{
    cfl_builder::{
        progress_event::{ProgressEvent, ProgressMonitor},
        simplify::simplification_stats::SimplificationStats,
        transient_graph::{TGraph, TNode, TNodeIndex},
    },
    error::Result,
};
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

pub fn remove_all_eps_nodes<F>(
    tgraph: &mut TGraph,
    progress_monitor: &mut ProgressMonitor<F>,
    simplification_stats: &mut SimplificationStats,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    simplification_stats.eps_removed += tgraph.nodes.iter().filter(|n| n.is_eps()).count();
    simplification_stats.eps_removal_iterations += 1;

    let eps_data = build_eps_scc_data(tgraph, progress_monitor)?;
    let closure = compute_eps_closure(&eps_data, progress_monitor)?;
    rebuild_in_place(tgraph, &eps_data, &closure, progress_monitor)?;

    progress_monitor.stage_total = 1;
    progress_monitor.emit_simplification_nth("Replacing graph with epsilon-free projection", 0)?;

    Ok(())
}

struct EpsSccData {
    is_eps: Vec<bool>,
    comp_of: Vec<usize>,
    comp_succs: Vec<Vec<usize>>,
    comp_terminals: Vec<Vec<TNodeIndex>>,
    topo: Vec<usize>,
}

fn build_eps_scc_data<F>(
    tgraph: &TGraph,
    progress_monitor: &mut ProgressMonitor<F>,
) -> Result<EpsSccData>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let n = tgraph.nodes.len();
    let is_eps: Vec<bool> = tgraph.nodes.iter().map(|node| node.is_eps()).collect();

    fn strongconnect(
        v: usize,
        nodes: &[TNode],
        is_eps: &[bool],
        index: &mut [usize],
        lowlink: &mut [usize],
        on_stack: &mut [bool],
        stack: &mut Vec<usize>,
        next_index: &mut usize,
        comp_of: &mut [usize],
        comps: &mut Vec<Vec<TNodeIndex>>,
    ) {
        index[v] = *next_index;
        lowlink[v] = *next_index;
        *next_index += 1;
        stack.push(v);
        on_stack[v] = true;

        for &w in &nodes[v].outcoming {
            let w = w as usize;
            if !is_eps[w] {
                continue;
            }
            if index[w] == usize::MAX {
                strongconnect(
                    w, nodes, is_eps, index, lowlink, on_stack, stack, next_index, comp_of, comps,
                );
                lowlink[v] = lowlink[v].min(lowlink[w]);
            } else if on_stack[w] {
                lowlink[v] = lowlink[v].min(index[w]);
            }
        }

        if lowlink[v] == index[v] {
            let comp_idx = comps.len();
            let mut comp = Vec::new();
            loop {
                let w = stack.pop().expect("Tarjan stack underflow");
                on_stack[w] = false;
                comp_of[w] = comp_idx;
                comp.push(w as TNodeIndex);
                if w == v {
                    break;
                }
            }
            comps.push(comp);
        }
    }

    let mut index = vec![usize::MAX; n];
    let mut lowlink = vec![0usize; n];
    let mut on_stack = vec![false; n];
    let mut stack = Vec::new();
    let mut next_index = 0usize;
    let mut comp_of = vec![usize::MAX; n];
    let mut comps: Vec<Vec<TNodeIndex>> = Vec::new();

    progress_monitor.stage_total = n;
    for i in 0..n {
        progress_monitor.emit_simplification_nth("Building epsilon SCCs", i)?;
        if is_eps[i] && index[i] == usize::MAX {
            strongconnect(
                i,
                &tgraph.nodes,
                &is_eps,
                &mut index,
                &mut lowlink,
                &mut on_stack,
                &mut stack,
                &mut next_index,
                &mut comp_of,
                &mut comps,
            );
        }
    }

    let comp_count = comps.len();

    let mut comp_succ_sets: Vec<HashSet<usize>> = vec![HashSet::new(); comp_count];
    let mut comp_terminals: Vec<Vec<TNodeIndex>> = vec![Vec::new(); comp_count];

    progress_monitor.stage_total = comp_count;
    for (comp_idx, members) in comps.iter().enumerate() {
        progress_monitor
            .emit_simplification_nth("Building epsilon condensation graph", comp_idx)?;

        for &node_idx in members {
            let node = &tgraph.nodes[node_idx as usize];
            for &to in &node.outcoming {
                let to_usize = to as usize;
                if is_eps[to_usize] {
                    let to_comp = comp_of[to_usize];
                    if to_comp != comp_idx {
                        comp_succ_sets[comp_idx].insert(to_comp);
                    }
                } else {
                    comp_terminals[comp_idx].push(to);
                }
            }
        }
    }

    let comp_succs: Vec<Vec<usize>> = comp_succ_sets
        .into_iter()
        .map(|s| {
            let mut v: Vec<usize> = s.into_iter().collect();
            v.sort_unstable();
            v
        })
        .collect();

    for term in &mut comp_terminals {
        term.sort_unstable();
        term.dedup();
    }

    let mut indeg = vec![0usize; comp_count];
    for succs in &comp_succs {
        for &succ in succs {
            indeg[succ] += 1;
        }
    }
    let mut q = VecDeque::with_capacity(comp_count);
    for i in 0..comp_count {
        if indeg[i] == 0 {
            q.push_back(i);
        }
    }
    let mut topo = Vec::with_capacity(comp_count);
    while let Some(c) = q.pop_front() {
        topo.push(c);
        for &succ in &comp_succs[c] {
            indeg[succ] -= 1;
            if indeg[succ] == 0 {
                q.push_back(succ);
            }
        }
    }
    assert_eq!(
        topo.len(),
        comp_count,
        "Epsilon condensation graph must be a DAG"
    );

    Ok(EpsSccData {
        is_eps,
        comp_of,
        comp_succs,
        comp_terminals,
        topo,
    })
}

fn compute_eps_closure<F>(
    data: &EpsSccData,
    progress_monitor: &mut ProgressMonitor<F>,
) -> Result<Vec<Vec<TNodeIndex>>>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let comp_count = data.comp_terminals.len();
    let mut closure: Vec<Vec<TNodeIndex>> = vec![Vec::new(); comp_count];

    progress_monitor.stage_total = comp_count;
    for (step, &comp_idx) in data.topo.iter().rev().enumerate() {
        progress_monitor.emit_simplification_nth("Computing epsilon closures", step)?;

        let mut reach = data.comp_terminals[comp_idx].clone();
        for &succ in &data.comp_succs[comp_idx] {
            reach.extend_from_slice(&closure[succ]);
        }
        reach.sort_unstable();
        reach.dedup();
        closure[comp_idx] = reach;
    }

    Ok(closure)
}

fn rebuild_in_place<F>(
    tgraph: &mut TGraph,
    data: &EpsSccData,
    closure: &[Vec<TNodeIndex>],
    progress_monitor: &mut ProgressMonitor<F>,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let n = tgraph.nodes.len();

    let old_out: Vec<Vec<TNodeIndex>> = tgraph
        .nodes
        .iter()
        .map(|node| node.outcoming.clone())
        .collect();
    let old_sym: Vec<Option<crate::core::CFLSymbolIndex>> =
        tgraph.nodes.iter().map(|node| node.symbol).collect();

    let total_work = count_rebuild_work(tgraph, data, closure).max(1);
    progress_monitor.stage_total = total_work;

    let non_eps_sources: Vec<usize> = (0..n).filter(|&i| !data.is_eps[i]).collect();

    let mut new_out_non_eps = vec![Vec::new(); non_eps_sources.len()];
    new_out_non_eps
        .par_iter_mut()
        .zip(non_eps_sources.par_iter())
        .for_each(|(target_slot, &src)| {
            *target_slot = compute_rebuilt_targets(src, &old_out, &old_sym, data, closure);
        });

    let mut new_out = vec![Vec::new(); n];
    let mut new_in = vec![Vec::new(); n];
    for (i, &src) in non_eps_sources.iter().enumerate() {
        new_out[src] = std::mem::take(&mut new_out_non_eps[i]);
    }
    for src in 0..n {
        for &dst in &new_out[src] {
            new_in[dst as usize].push(src as TNodeIndex);
        }
    }

    progress_monitor.stage_total = n;
    for i in 0..n {
        progress_monitor.emit_simplification_nth("Applying rebuilt adjacency", i)?;
        tgraph.nodes[i].outcoming = std::mem::take(&mut new_out[i]);
        tgraph.nodes[i].incoming = std::mem::take(&mut new_in[i]);
    }
    tgraph.edges_count = tgraph.nodes.iter().map(|node| node.outcoming.len()).sum();
    progress_monitor.emit_simplification_nth("Rebuilding epsilon-free edges", total_work)?;

    Ok(())
}

fn compute_rebuilt_targets(
    src: usize,
    old_out: &[Vec<TNodeIndex>],
    old_sym: &[Option<crate::core::CFLSymbolIndex>],
    data: &EpsSccData,
    closure: &[Vec<TNodeIndex>],
) -> Vec<TNodeIndex> {
    let src_sym = old_sym[src];
    let src_node_index = src as TNodeIndex;
    let mut targets = Vec::with_capacity(old_out[src].len());
    let mut eps_comps = Vec::new();

    for &to in &old_out[src] {
        if to == src_node_index {
            continue;
        }
        let to_usize = to as usize;
        if data.is_eps[to_usize] {
            eps_comps.push(data.comp_of[to_usize]);
        } else if is_valid_rebuilt_edge(src_sym, old_sym[to_usize]) {
            targets.push(to);
        }
    }

    if !eps_comps.is_empty() {
        eps_comps.sort_unstable();
        eps_comps.dedup();
        for comp_idx in eps_comps {
            for &dst in &closure[comp_idx] {
                if dst != src_node_index && is_valid_rebuilt_edge(src_sym, old_sym[dst as usize]) {
                    targets.push(dst);
                }
            }
        }
        targets.sort_unstable();
        targets.dedup();
    }

    targets
}

fn count_rebuild_work(tgraph: &TGraph, data: &EpsSccData, closure: &[Vec<TNodeIndex>]) -> usize {
    let mut work = 0usize;
    for src in 0..tgraph.nodes.len() {
        if data.is_eps[src] {
            continue;
        }
        for &to in &tgraph.nodes[src].outcoming {
            let to_usize = to as usize;
            if data.is_eps[to_usize] {
                work += closure[data.comp_of[to_usize]].len();
            } else {
                work += 1;
            }
        }
    }
    work
}

fn is_valid_rebuilt_edge(
    src_sym: Option<crate::core::CFLSymbolIndex>,
    dst_sym: Option<crate::core::CFLSymbolIndex>,
) -> bool {
    match (src_sym, dst_sym) {
        (Some(s), Some(d)) if TGraph::is_push(s) && !TGraph::is_push(d) => (s / 2) == (d / 2),
        _ => true,
    }
}
