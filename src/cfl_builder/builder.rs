use crate::cfl_builder::progress_event::{ProgressEvent, ProgressMonitor};
use crate::cfl_builder::transient_builder::convert_to_transient;
use crate::cfl_builder::transient_graph::{TGraph, TNode, TNodeIndex};
use crate::cfl_builder::SimplificationOptions;
use crate::core::{CFLEdge, CFLGraph, CFLNodeIndex, CFLNodeMetadata, SGGraph};
use crate::error::Result;
use std::collections::HashMap;
use std::time::Duration;

// For some symbol nodes in Stack Graph we produce 2 nodes in CFL graph:
// in-node and out-node, where all the incoming edges of the original
// node enter the in-node, all the outcoming edges go out of the out-node,
// and there is an edge from in-node to out-node with the symbol of the original
// node as its label.
// It is required in 2 case:
// 1. The symbol node has no outcoming edges - then we have nowhere
//    to put the symbol unless we create a new edge
// 2. The symbol node has multiple outcoming edges - then if we
//    label each one of them it will create multiple paths at once.
fn generate_out_indices<F>(
    src_nodes: &[TNode],
    rules_count: usize,
    progress: &mut ProgressMonitor<F>,
) -> Result<HashMap<TNodeIndex, CFLNodeIndex>>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut out_indices = HashMap::new();
    progress.stage_total = src_nodes.len();
    let mut out_node_idx = src_nodes.len() as CFLNodeIndex; // For nodes with multiple outcoming edges
    let mut sink_node_buckets = Vec::new(); // For nodes without outcoming edges
    let mut sink_node_counts = vec![0; rules_count];
    for (i, src_node) in src_nodes.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::BuildingOutIds(v))?;
        // TODO: check for cases when there are several definition nodes with a single edge and all of them end in the same node.
        // Then we cannot attach the symbol to all the edges lest we want to lose some pairs in query results.
        if src_node.is_real() && src_node.outcoming.len() != 1 {
            out_indices.insert(
                i as TNodeIndex,
                if src_node.outcoming.is_empty() {
                    let rule = src_node.rule().unwrap();
                    let index_within_same_rule = sink_node_counts[rule as usize];
                    sink_node_counts[rule as usize] += 1;
                    if index_within_same_rule >= sink_node_buckets.len() {
                        sink_node_buckets.push(out_node_idx);
                        out_node_idx += 1;
                    }
                    sink_node_buckets[index_within_same_rule]
                } else {
                    let new_idx = out_node_idx;
                    out_node_idx += 1;
                    new_idx
                },
            );
        }
    }
    Ok(out_indices)
}

// Generates a mapping from CFL graph nodes to the metadata
// (like source location). It is created only for nodes
// from which labeled edges go out.
fn generate_node_metadata<F>(
    src_nodes: &[TNode],
    progress: &mut ProgressMonitor<F>,
) -> Result<HashMap<CFLNodeIndex, CFLNodeMetadata>>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut metadatas = HashMap::new();
    progress.stage_total = src_nodes.len();
    for (i, src_node) in src_nodes.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::BuildingNodeMetadata(v))?;
        if let Some(metadata) = &src_node.metadata {
            metadatas.insert(i as CFLNodeIndex, metadata.clone());
        }
    }
    Ok(metadatas)
}

// Generates the edges for the already existing ones.
// Add labels where appropriate.
fn generate_for_current_edges<F>(
    src_nodes: &[TNode],
    out_indices: &HashMap<TNodeIndex, CFLNodeIndex>,
    edges_count: usize,
    progress: &mut ProgressMonitor<F>,
) -> Result<Vec<CFLEdge>>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut edges = Vec::with_capacity(edges_count + out_indices.len());
    progress.stage_total = src_nodes.len();
    for (i, src_node) in src_nodes.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::BuildingForCurrentEdges(v))?;
        for out_edge in &src_node.outcoming {
            edges.push(
                if let Some(&out_index) = out_indices.get(&(i as TNodeIndex)) {
                    CFLEdge {
                        from: out_index,
                        to: *out_edge,
                        symbol: None,
                    }
                } else {
                    CFLEdge {
                        from: i as CFLNodeIndex,
                        to: *out_edge,
                        symbol: src_node.symbol,
                    }
                },
            );
        }
    }
    Ok(edges)
}

// Generates new edges between in-nodes and out-nodes
// that didn;t exist in the original graph.
fn generate_symbol_edges<F>(
    edges: &mut Vec<CFLEdge>,
    src_nodes: &[TNode],
    out_indices: &HashMap<TNodeIndex, CFLNodeIndex>,
    progress: &mut ProgressMonitor<F>,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    progress.stage_total = out_indices.len();
    for (i, (in_idx, out_idx)) in out_indices.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::BuildingSymbolEdges(v))?;
        edges.push(CFLEdge {
            symbol: src_nodes[*in_idx as usize].symbol,
            from: *in_idx,
            to: *out_idx,
        });
    }
    Ok(())
}

fn generate_edges<F>(tgraph: &TGraph, progress: &mut ProgressMonitor<F>) -> Result<Vec<CFLEdge>>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let out_indices = generate_out_indices(
        &tgraph.nodes,
        tgraph.cfl_push_pop_rules_count as usize,
        progress,
    )?;
    let mut edges =
        generate_for_current_edges(&tgraph.nodes, &out_indices, tgraph.edges_count, progress)?;
    generate_symbol_edges(&mut edges, &tgraph.nodes, &out_indices, progress)?;
    Ok(edges)
}

pub fn convert_to_cfl<F>(
    sggraph: &SGGraph,
    simplification_options: &SimplificationOptions,
    progress: F,
) -> Result<(CFLGraph, Duration)>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut progress_monitor = ProgressMonitor::new(progress);
    let transient_graph =
        convert_to_transient(sggraph, simplification_options, &mut progress_monitor)?;
    let edges = generate_edges(&transient_graph, &mut progress_monitor)?;
    let metadata = generate_node_metadata(&transient_graph.nodes, &mut progress_monitor)?;

    let cfl_graph = CFLGraph {
        edges,
        metadata,
        files: transient_graph.files,
        potentially_virtual_rules: transient_graph.potentially_virtual_rules,
        sg_to_cfl_rule_index: transient_graph.sg_to_cfl_rule_index,
        cfl_push_pop_rules_count: transient_graph.cfl_push_pop_rules_count,
    };

    progress_monitor.emit(|e| ProgressEvent::Done(e))?;
    Ok((cfl_graph, progress_monitor.start.elapsed()))
}
