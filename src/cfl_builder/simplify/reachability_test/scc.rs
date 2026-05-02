use std::collections::HashSet;

use crate::cfl_builder::progress_event::{ProgressEvent, ProgressMonitor};
use crate::cfl_builder::transient_graph::{TGraph, TNode, TNodeIndex};
use crate::error::Result;

// Generates all strongly connected components (SCCs) of `tgraph` using Tarjan’s algorithm.
pub fn generate_scc<F>(
    tgraph: &TGraph,
    progress: &mut ProgressMonitor<F>,
    mut callback: impl FnMut(Vec<TNodeIndex>) -> Result<()>,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let node_count = tgraph.nodes.len();
    if node_count == 0 {
        return Ok(());
    }

    progress.stage_total = node_count;

    let mut index_counter = 0u32;
    let mut indices: Vec<Option<u32>> = vec![None; node_count];
    let mut lowlink = vec![0u32; node_count];
    let mut on_stack = vec![false; node_count];
    let mut node_stack = Vec::new();

    let mut dfs_stack = Vec::new();

    let mut completed_nodes = 0usize;

    for v in 0..node_count {
        if indices[v].is_some() {
            continue;
        }

        dfs_stack.push((v, 0));

        while let Some(&mut (v, ref mut i)) = dfs_stack.last_mut() {
            let edges = &tgraph.nodes[v].outcoming;

            if *i == 0 {
                indices[v] = Some(index_counter);
                lowlink[v] = index_counter;
                index_counter += 1;
                node_stack.push(v);
                on_stack[v] = true;
            }

            let mut recurse = false;
            while *i < edges.len() {
                let w = edges[*i] as usize;
                *i += 1;

                if indices[w].is_none() {
                    dfs_stack.push((w, 0));
                    recurse = true;
                    break;
                } else if on_stack[w] {
                    lowlink[v] = lowlink[v].min(indices[w].unwrap());
                }
            }

            if recurse {
                continue;
            }

            dfs_stack.pop();
            completed_nodes += 1;

            if let Some(&(parent, _)) = dfs_stack.last() {
                lowlink[parent] = lowlink[parent].min(lowlink[v]);
            }

            if lowlink[v] == indices[v].unwrap() {
                let mut scc = Vec::new();
                loop {
                    let w = node_stack.pop().unwrap();
                    on_stack[w] = false;
                    scc.push(w as TNodeIndex);
                    if w == v {
                        break;
                    }
                }
                if !(scc.len() == 1
                    && tgraph.nodes[scc[0] as usize].incoming.is_empty()
                    && tgraph.nodes[scc[0] as usize].outcoming.is_empty())
                // Skip isolated nodes
                {
                    callback(scc)?;
                }
            }
            progress.emit_simplification_nth("Computing SCCs", completed_nodes)?;
        }
    }

    Ok(())
}

// Returns a condensed graph and mapping from condensed nodes to original ones
pub fn condense_with_scc<F>(
    tgraph: &TGraph,
    progress: &mut ProgressMonitor<F>,
) -> Result<(TGraph, Vec<Vec<TNodeIndex>>)>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut result = TGraph {
        nodes: Vec::new(),
        files: Vec::new(),
        potentially_virtual_rules: HashSet::new(),
        sg_to_cfl_rule_index: Vec::new(),
        cfl_push_pop_rules_count: tgraph.cfl_push_pop_rules_count,
        edges_count: 0,
    };
    let mut mapping = vec![0; tgraph.nodes.len()];
    let mut mapping_back = Vec::new();
    generate_scc(tgraph, progress, |nodes| {
        let new_node_idx = mapping_back.len() as TNodeIndex;
        nodes.iter().for_each(|&n| {
            mapping[n as usize] = new_node_idx;
        });
        mapping_back.push(nodes);
        result.nodes.push(TNode {
            symbol: None,
            metadata: None,
            incoming: Vec::new(),
            outcoming: Vec::new(),
        });
        Ok(())
    })?;

    let mut buf = Vec::new();
    for (condensed_node_idx, original_nodes) in mapping_back.iter().enumerate() {
        buf.clear();
        for &orig in original_nodes {
            buf.extend(
                tgraph.nodes[orig as usize]
                    .incoming
                    .iter()
                    .map(|&n| mapping[n as usize])
                    .filter(|&n| n != condensed_node_idx as TNodeIndex),
            );
        }
        buf.sort_unstable();
        buf.dedup();
        result.nodes[condensed_node_idx].incoming = std::mem::take(&mut buf);

        buf.clear();
        for &orig in original_nodes {
            buf.extend(
                tgraph.nodes[orig as usize]
                    .outcoming
                    .iter()
                    .map(|&n| mapping[n as usize])
                    .filter(|&n| n != condensed_node_idx as TNodeIndex),
            );
        }
        buf.sort_unstable();
        buf.dedup();
        result.nodes[condensed_node_idx].outcoming = std::mem::take(&mut buf);
    }
    let max = mapping_back.iter().max_by_key(|m| m.len()).unwrap().len();
    let max_pos = mapping_back.iter().position(|m| m.len() == max).unwrap();
    crate::debug!(
        "Condensed into {} components; largest one is {} nodes with {} edges total",
        result.nodes.len(),
        max,
        result.nodes[max_pos].incoming.len() + result.nodes[max_pos].outcoming.len(),
    );
    Ok((result, mapping_back))
}
