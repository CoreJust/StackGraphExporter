use crate::cfl_builder::progress_event::{ProgressEvent, ProgressMonitor};
use crate::cfl_builder::simplify::simplify_transient_graph;
use crate::cfl_builder::transient_graph::{TGraph, TNode};
use crate::cfl_builder::SimplificationOptions;
use crate::core::{
    CFLNodeMetadata, CFLSymbolIndex, SGEdge, SGGraph, SGNode, SGNodeIndex, SGSymbol, SGSymbolIndex,
};
use crate::error::Result;
use std::collections::{HashMap, HashSet};

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

fn generate_symbols<F>(
    symbols: &[SGSymbol],
    progress: &mut ProgressMonitor<F>,
) -> Result<(Vec<CFLSymbolIndex>, usize)>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut result = Vec::new();
    let mut symbols_mapping = HashMap::new();

    progress.stage_total = symbols.len();
    for (i, symbol) in symbols.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::BuildingSymbolRules(v))?;
        if let Some(cfl_idx) = symbols_mapping.get(&symbol.name) {
            result.push(*cfl_idx);
        } else {
            let cfl_idx = symbols_mapping.len();
            symbols_mapping.insert(&symbol.name, cfl_idx);
            result.push(cfl_idx);
        }
    }
    Ok((result, symbols_mapping.len()))
}

fn generate_empty_nodes<F>(
    src_nodes: &[SGNode],
    src_symbols: &[SGSymbol],
    symbol_mapping: &Vec<CFLSymbolIndex>,
    progress: &mut ProgressMonitor<F>,
) -> Result<Vec<TNode>>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut nodes = Vec::with_capacity(src_nodes.len());
    progress.stage_total = src_nodes.len();
    for (i, src_node) in src_nodes.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::BuildingTransientNodes(v))?;
        if let Some(symbol_idx) = get_symbol_of(src_node) {
            let cfl_rule_idx = symbol_mapping[symbol_idx];
            let symbol = &src_symbols[symbol_idx];
            let metadata = CFLNodeMetadata {
                name: symbol.name.clone(),
                is_real: symbol.real,
                file: symbol.file,
                line: symbol.line,
                sg_node_index: i as SGNodeIndex,
            };
            nodes.push(TNode {
                symbol: Some(2 * cfl_rule_idx + if is_push_node(src_node) { 0 } else { 1 }),
                metadata: Some(metadata),
                incoming: vec![],
                outcoming: vec![],
            });
        } else {
            nodes.push(TNode {
                symbol: None,
                metadata: None,
                incoming: vec![],
                outcoming: vec![],
            });
        }
    }
    Ok(nodes)
}

fn populate_node_edges<F>(
    src_edges: &[SGEdge],
    nodes: &mut Vec<TNode>,
    progress: &mut ProgressMonitor<F>,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    progress.stage_total = src_edges.len();
    for (i, src_edge) in src_edges.iter().enumerate() {
        progress.emit_nth(i, |v| ProgressEvent::PopulatingTransientNodes(v))?;
        nodes[src_edge.from as usize].outcoming.push(src_edge.to);
        nodes[src_edge.to as usize].incoming.push(src_edge.from);
    }
    Ok(())
}

fn generate_nodes<F>(
    sggraph: &SGGraph,
    symbol_mapping: &Vec<CFLSymbolIndex>,
    progress: &mut ProgressMonitor<F>,
) -> Result<Vec<TNode>>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let mut nodes =
        generate_empty_nodes(&sggraph.nodes, &sggraph.symbols, symbol_mapping, progress)?;
    populate_node_edges(&sggraph.edges, &mut nodes, progress)?;
    Ok(nodes)
}

pub fn convert_to_transient<F>(
    sggraph: &SGGraph,
    simplification_options: &SimplificationOptions,
    progress_monitor: &mut ProgressMonitor<F>,
) -> Result<TGraph>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let (sg_to_cfl_rule_index, sg_unique_symbols_count) =
        generate_symbols(&sggraph.symbols, progress_monitor)?;
    let nodes = generate_nodes(sggraph, &sg_to_cfl_rule_index, progress_monitor)?;

    let potentially_virtual_rules = sggraph
        .symbols
        .iter()
        .enumerate()
        .filter(|s| !s.1.real)
        .map(|s| sg_to_cfl_rule_index[s.0])
        .collect::<HashSet<_>>();

    let mut transient_graph = TGraph {
        nodes,
        files: sggraph.files.clone(),
        potentially_virtual_rules,
        sg_to_cfl_rule_index,
        sg_unique_symbols_count,
        edges_count: sggraph.edges.len(),
    };

    if simplification_options.simplify
        && simplification_options.transient_simplification_iterations != Some(0)
    {
        simplify_transient_graph(
            &mut transient_graph,
            simplification_options,
            progress_monitor,
        )?;
    }

    progress_monitor.emit(|e| ProgressEvent::TransientGraphBuilt(e))?;
    Ok(transient_graph)
}
