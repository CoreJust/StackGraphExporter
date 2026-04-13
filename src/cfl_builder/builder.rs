use super::simplify_graph;
use crate::cfl_builder::progress_event::{ProgressEvent, ProgressMonitor};
use crate::core::{
    CFLEdge, CFLGraph, CFLNodeIndex, CFLNodeMetadata, CFLRule, CFLSymbol, SGEdge, SGGraph, SGNode,
    SGNodeIndex, SGSymbol, SGSymbolIndex,
};
use crate::error::Result;
use std::collections::HashMap;
use std::time::Duration;

pub fn get_symbol_of(node: &SGNode) -> Option<SGSymbolIndex> {
    match node {
        SGNode::Pop(s)
        | SGNode::Push(s)
        | SGNode::PopScoped(s)
        | SGNode::PushScoped(s, _)
        | SGNode::PushScopedUnresolved(s, _) => Some(*s),
        _ => None,
    }
}

fn is_push_node(node: &SGNode) -> bool {
    matches!(
        node,
        SGNode::Push(_) | SGNode::PushScoped(_, _) | SGNode::PushScopedUnresolved(_, _)
    )
}

// For each symbol node in Stack Graph we produce 2 nodes in CFL graph:
// in-node and out-node, where all the incoming edges of the original
// node enter the in-node, all the outcoming edges go out of the out-node,
// and there is an edge from in-node to out-node with the symbol of the original
// node as its label.
// Non-symbol nodes have the same index as before, for symbol nodes the in-node
// has the same index and the out-node needs a new index which we assign here.
fn generate_out_indices<F>(
    src_nodes: &[SGNode],
    progress: &mut ProgressMonitor<F>,
) -> Result<HashMap<SGNodeIndex, CFLNodeIndex>>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut out_indices = HashMap::new();
    progress.stage_total = src_nodes.len();
    for (i, src_node) in src_nodes.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::BuildingOutIds(v))?;
        if get_symbol_of(src_node).is_some() {
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
    symbols: &[SGSymbol],
    src_nodes: &[SGNode],
    out_indices: &HashMap<SGNodeIndex, CFLNodeIndex>,
    progress: &mut ProgressMonitor<F>,
) -> Result<HashMap<CFLNodeIndex, CFLNodeMetadata>>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut metadatas = HashMap::with_capacity(out_indices.len());
    progress.stage_total = out_indices.len();
    for (i, (from, to)) in out_indices.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::BuildingNodeMetadata(v))?;
        let node = &src_nodes[*from as usize];
        let symbol_index = get_symbol_of(node).unwrap();
        let symbol = &symbols[symbol_index];
        let mapped_index = if is_push_node(node) { *from } else { *to };
        let metadata = CFLNodeMetadata {
            name: symbol.name.clone(),
            is_real: symbol.real,
            file: symbol.file,
            line: symbol.line,
            sg_node_index: mapped_index as SGNodeIndex,
        };
        metadatas.insert(mapped_index as CFLNodeIndex, metadata);
    }
    Ok(metadatas)
}

// Generates the edges for the already existing ones.
fn generate_for_current_edges<F>(
    src_edges: &[SGEdge],
    src_nodes: &[SGNode],
    out_indices: &HashMap<SGNodeIndex, CFLNodeIndex>,
    progress: &mut ProgressMonitor<F>,
) -> Result<Vec<CFLEdge>>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut edges = Vec::with_capacity(src_edges.len() + out_indices.len());
    progress.stage_total = src_edges.len();
    for (i, src_edge) in src_edges.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::BuildingForCurrentEdges(v))?;
        let from_node = &src_nodes[src_edge.from as usize];
        if get_symbol_of(from_node).is_some() {
            edges.push(CFLEdge {
                from: *out_indices.get(&src_edge.from).unwrap(),
                to: src_edge.to,
                symbol: None,
            });
        } else {
            edges.push(CFLEdge {
                from: src_edge.from,
                to: src_edge.to,
                symbol: None,
            });
        }
    }
    Ok(edges)
}

// Generates new edges between in-nodes and out-nodes
// that didn;t exist in the original graph.
fn generate_symbol_edges<F>(
    edges: &mut Vec<CFLEdge>,
    nodes: &[SGNode],
    out_indices: &HashMap<SGNodeIndex, CFLNodeIndex>,
    progress: &mut ProgressMonitor<F>,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    progress.stage_total = out_indices.len();
    for (i, (in_idx, out_idx)) in out_indices.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::BuildingSymbolEdges(v))?;
        if let Some(symbol_idx) = get_symbol_of(&nodes[*in_idx as usize]) {
            edges.push(CFLEdge {
                symbol: Some(
                    2 * symbol_idx
                        + if is_push_node(&nodes[*in_idx as usize]) {
                            0
                        } else {
                            1
                        },
                ),
                from: *in_idx,
                to: *out_idx,
            });
        }
    }
    Ok(())
}

fn generate_edges<F>(
    sggraph: &SGGraph,
    progress: &mut ProgressMonitor<F>,
) -> Result<(Vec<CFLEdge>, HashMap<SGNodeIndex, CFLNodeIndex>)>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let out_indices = generate_out_indices(&sggraph.nodes, progress)?;
    let mut edges =
        generate_for_current_edges(&sggraph.edges, &sggraph.nodes, &out_indices, progress)?;
    generate_symbol_edges(&mut edges, &sggraph.nodes, &out_indices, progress)?;
    Ok((edges, out_indices))
}

fn generate_symbols_rules<F>(
    symbols: &[SGSymbol],
    progress: &mut ProgressMonitor<F>,
) -> Result<(Vec<String>, Vec<CFLRule>)>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut rules = Vec::with_capacity(2 + symbols.len());
    let mut cfl_symbols = Vec::with_capacity(1 + 2 * symbols.len());
    let s_non_terminal = 2 * symbols.len();

    progress.stage_total = symbols.len();
    for (i, symbol) in symbols.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::BuildingSymbolRules(v))?;
        let id = cfl_symbols.len();
        cfl_symbols.push(format!("push_{}", symbol.name));
        cfl_symbols.push(format!("pop_{}", symbol.name));
        // S := push_X S pop_X
        rules.push(CFLRule {
            from_non_terminal: s_non_terminal,
            to: vec![
                CFLSymbol::Terminal(id),
                CFLSymbol::NonTerminal(s_non_terminal),
                CFLSymbol::Terminal(id + 1),
            ],
        });
    }

    cfl_symbols.push("S".to_string());
    // S := epsilon
    rules.push(CFLRule {
        from_non_terminal: s_non_terminal,
        to: vec![],
    });
    // S := S S
    rules.push(CFLRule {
        from_non_terminal: s_non_terminal,
        to: vec![
            CFLSymbol::NonTerminal(s_non_terminal),
            CFLSymbol::NonTerminal(s_non_terminal),
        ],
    });

    Ok((cfl_symbols, rules))
}

pub fn convert_to_cfl<F>(
    sggraph: &SGGraph,
    simplify: bool,
    progress: F,
) -> Result<(CFLGraph, HashMap<SGNodeIndex, CFLNodeIndex>, Duration)>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut progress_monitor = ProgressMonitor::new(progress);
    let (symbols, rules) = generate_symbols_rules(&sggraph.symbols, &mut progress_monitor)?;
    let (mut edges, out_indices) = generate_edges(sggraph, &mut progress_monitor)?;
    let mut metadata = generate_node_metadata(
        &sggraph.symbols,
        &sggraph.nodes,
        &out_indices,
        &mut progress_monitor,
    )?;

    progress_monitor.stage_total = out_indices.len();
    let mut pop_sg_to_cfl_out = HashMap::new();
    for (i, (&sg_idx, &cfl_out)) in (&out_indices).iter().enumerate() {
        progress_monitor.emit_nth(i, |v| ProgressEvent::BuildingPopNodesMapping(v))?;
        let node = &sggraph.nodes[sg_idx as usize];
        if matches!(node, SGNode::Pop(_) | SGNode::PopScoped(_)) {
            pop_sg_to_cfl_out.insert(sg_idx, cfl_out);
        }
    }

    if simplify {
        simplify_graph(
            &mut edges,
            &mut metadata,
            sggraph.nodes.len() + out_indices.len(),
            &mut progress_monitor,
        )?;
    }

    let cfl_graph = CFLGraph {
        rules,
        edges,
        symbols,
        metadata,
        files: sggraph.files.clone(),
    };

    progress_monitor.emit(|e| ProgressEvent::Done(e))?;
    Ok((
        cfl_graph,
        pop_sg_to_cfl_out,
        progress_monitor.start.elapsed(),
    ))
}
