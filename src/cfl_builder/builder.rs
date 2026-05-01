use crate::cfl_builder::cfl_simplifier::simplify_graph;
use crate::cfl_builder::progress_event::{ProgressEvent, ProgressMonitor};
use crate::cfl_builder::transient_builder::convert_to_transient;
use crate::cfl_builder::transient_graph::{TGraph, TNode};
use crate::cfl_builder::SimplificationOptions;
use crate::core::{CFLEdge, CFLGraph, CFLNodeIndex, CFLNodeMetadata, SGGraph, SGNode, SGNodeIndex};
use crate::error::Result;
use std::collections::HashMap;
use std::time::Duration;

// For each symbol node in Stack Graph we produce 2 nodes in CFL graph:
// in-node and out-node, where all the incoming edges of the original
// node enter the in-node, all the outcoming edges go out of the out-node,
// and there is an edge from in-node to out-node with the symbol of the original
// node as its label.
// Non-symbol nodes have the same index as before, for symbol nodes the in-node
// has the same index and the out-node needs a new index which we assign here.
fn generate_out_indices<F>(
    src_nodes: &[TNode],
    progress: &mut ProgressMonitor<F>,
) -> Result<HashMap<SGNodeIndex, CFLNodeIndex>>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut out_indices = HashMap::new();
    progress.stage_total = src_nodes.len();
    for (i, src_node) in src_nodes.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::BuildingOutIds(v))?;
        if src_node.symbol.is_some() {
            out_indices.insert(i as u32, out_indices.len() as u32 + src_nodes.len() as u32);
        }
    }
    Ok(out_indices)
}

// Generates a mapping from CFL graph nodes to the metadata
// (like source location).
// For push nodes the in-nodes are mapped, for pop nodes
// the out-nodes are mapped.
fn generate_node_metadata<F>(
    src_nodes: &[TNode],
    out_indices: &HashMap<SGNodeIndex, CFLNodeIndex>,
    progress: &mut ProgressMonitor<F>,
) -> Result<HashMap<CFLNodeIndex, CFLNodeMetadata>>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut metadatas = HashMap::with_capacity(out_indices.len());
    progress.stage_total = src_nodes.len();
    for (i, src_node) in src_nodes.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::BuildingNodeMetadata(v))?;
        if let Some(metadata) = &src_node.metadata {
            let symbol_index = src_node.symbol.unwrap();
            let mapped_index = if TGraph::is_push(symbol_index) {
                i as CFLNodeIndex
            } else {
                out_indices[&(i as u32)] as CFLNodeIndex
            };
            metadatas.insert(mapped_index as CFLNodeIndex, metadata.clone());
        }
    }
    Ok(metadatas)
}

// Generates the edges for the already existing ones.
fn generate_for_current_edges<F>(
    src_nodes: &[TNode],
    out_indices: &HashMap<SGNodeIndex, CFLNodeIndex>,
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
            if src_node.symbol.is_some() {
                edges.push(CFLEdge {
                    from: *out_indices.get(&(i as u32)).unwrap(),
                    to: *out_edge,
                    symbol: None,
                });
            } else {
                edges.push(CFLEdge {
                    from: i as CFLNodeIndex,
                    to: *out_edge,
                    symbol: None,
                });
            }
        }
    }
    Ok(edges)
}

// Generates new edges between in-nodes and out-nodes
// that didn;t exist in the original graph.
fn generate_symbol_edges<F>(
    edges: &mut Vec<CFLEdge>,
    src_nodes: &[TNode],
    out_indices: &HashMap<SGNodeIndex, CFLNodeIndex>,
    progress: &mut ProgressMonitor<F>,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    progress.stage_total = out_indices.len();
    for (i, (in_idx, out_idx)) in out_indices.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::BuildingSymbolEdges(v))?;
        if let Some(cfl_symbol_idx) = src_nodes[*in_idx as usize].symbol {
            edges.push(CFLEdge {
                symbol: Some(cfl_symbol_idx),
                from: *in_idx,
                to: *out_idx,
            });
        }
    }
    Ok(())
}

fn generate_edges<F>(
    tgraph: &TGraph,
    progress: &mut ProgressMonitor<F>,
) -> Result<(Vec<CFLEdge>, HashMap<SGNodeIndex, CFLNodeIndex>)>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let out_indices = generate_out_indices(&tgraph.nodes, progress)?;
    let mut edges =
        generate_for_current_edges(&tgraph.nodes, &out_indices, tgraph.edges_count, progress)?;
    generate_symbol_edges(&mut edges, &tgraph.nodes, &out_indices, progress)?;
    Ok((edges, out_indices))
}

pub fn convert_to_cfl<F>(
    sggraph: &SGGraph,
    simplification_options: &SimplificationOptions,
    progress: F,
) -> Result<(CFLGraph, HashMap<SGNodeIndex, CFLNodeIndex>, Duration)>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut progress_monitor = ProgressMonitor::new(progress);
    let transient_graph =
        convert_to_transient(sggraph, simplification_options, &mut progress_monitor)?;
    let (mut edges, out_indices) = generate_edges(&transient_graph, &mut progress_monitor)?;
    let mut metadata =
        generate_node_metadata(&transient_graph.nodes, &out_indices, &mut progress_monitor)?;

    progress_monitor.stage_total = out_indices.len();
    let mut pop_sg_to_cfl_out = HashMap::new();
    for (i, (&sg_idx, &cfl_out)) in (&out_indices).iter().enumerate() {
        progress_monitor.emit_nth(i, |v| ProgressEvent::BuildingPopNodesMapping(v))?;
        let node = &sggraph.nodes[sg_idx as usize];
        if matches!(node, SGNode::Pop(_) | SGNode::PopScoped(_)) {
            pop_sg_to_cfl_out.insert(sg_idx, cfl_out);
        }
    }

    if simplification_options.simplify && simplification_options.simplify_cfl {
        simplify_graph(
            &mut edges,
            &mut metadata,
            sggraph.nodes.len() + out_indices.len(),
            &mut progress_monitor,
        )?;
    }

    let cfl_graph = CFLGraph {
        edges,
        metadata,
        files: transient_graph.files,
        potentially_virtual_rules: transient_graph.potentially_virtual_rules,
        sg_to_cfl_rule_index: transient_graph.sg_to_cfl_rule_index,
        sg_unique_symbols_count: transient_graph.sg_unique_symbols_count,
    };

    progress_monitor.emit(|e| ProgressEvent::Done(e))?;
    Ok((
        cfl_graph,
        pop_sg_to_cfl_out,
        progress_monitor.start.elapsed(),
    ))
}
