use crate::{
    cfl_simplifier::simplify_graph,
    core::{
        CFLEdge, CFLGraph, CFLNodeIndex, CFLNodeMetadata, CFLRule, CFLSymbol, SGEdge, SGGraph,
        SGNode, SGNodeIndex, SGSymbol, SGSymbolIndex,
    },
};
use anyhow::Result;
use std::collections::HashMap;

fn get_symbol_of(node: &SGNode) -> Option<SGSymbolIndex> {
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

fn generate_out_ids(src_nodes: &[SGNode]) -> HashMap<SGNodeIndex, CFLNodeIndex> {
    let mut out_ids = HashMap::new();
    for (i, src_node) in src_nodes.iter().enumerate() {
        if get_symbol_of(src_node).is_some() {
            out_ids.insert(i as u32, out_ids.len() as u32 + src_nodes.len() as u32);
        }
    }
    out_ids
}

fn generate_node_metadata(
    symbols: &[SGSymbol],
    src_nodes: &[SGNode],
    out_ids: &HashMap<SGNodeIndex, CFLNodeIndex>,
) -> HashMap<CFLNodeIndex, CFLNodeMetadata> {
    let mut metadatas = HashMap::with_capacity(out_ids.len());
    for (from, to) in out_ids {
        let node = &src_nodes[*from as usize];
        let symbol_index = get_symbol_of(node).unwrap();
        let symbol = &symbols[symbol_index];
        let metadata = CFLNodeMetadata {
            name: symbol.name.clone(),
            is_real: symbol.real,
            file: symbol.file,
            line: symbol.line,
        };
        if is_push_node(node) {
            metadatas.insert(*from, metadata);
        } else {
            metadatas.insert(*to, metadata);
        }
    }
    metadatas
}

fn generate_for_current_edges(
    src_edges: &[SGEdge],
    src_nodes: &[SGNode],
    out_ids: &HashMap<SGNodeIndex, CFLNodeIndex>,
) -> Vec<CFLEdge> {
    let mut edges = Vec::with_capacity(src_edges.len() + out_ids.len());
    for src_edge in src_edges {
        let from_node = &src_nodes[src_edge.from as usize];
        if get_symbol_of(from_node).is_some() {
            edges.push(CFLEdge {
                from: *out_ids.get(&src_edge.from).unwrap(),
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
    edges
}

fn generate_symbol_edges(
    edges: &mut Vec<CFLEdge>,
    nodes: &[SGNode],
    out_ids: &HashMap<SGNodeIndex, CFLNodeIndex>,
) {
    for (in_id, out_id) in out_ids {
        if let Some(symbol_id) = get_symbol_of(&nodes[*in_id as usize]) {
            edges.push(CFLEdge {
                symbol: Some(
                    2 * symbol_id
                        + if is_push_node(&nodes[*in_id as usize]) {
                            0
                        } else {
                            1
                        },
                ),
                from: *in_id,
                to: *out_id,
            });
        }
    }
}

fn generate_edges(sggraph: &SGGraph) -> (Vec<CFLEdge>, HashMap<SGNodeIndex, CFLNodeIndex>) {
    let out_ids = generate_out_ids(&sggraph.nodes);
    let mut edges = generate_for_current_edges(&sggraph.edges, &sggraph.nodes, &out_ids);
    generate_symbol_edges(&mut edges, &sggraph.nodes, &out_ids);
    (edges, out_ids)
}

fn generate_symbols_rules(symbols: &[SGSymbol]) -> (Vec<String>, Vec<CFLRule>) {
    let mut rules = Vec::with_capacity(2 + symbols.len());
    let mut cfl_symbols = Vec::with_capacity(1 + 2 * symbols.len());
    let s_non_terminal = 2 * symbols.len();

    for symbol in symbols {
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

    (cfl_symbols, rules)
}

pub fn convert_to_cfl(
    sggraph: &SGGraph,
    simplify: bool,
) -> Result<(CFLGraph, HashMap<SGNodeIndex, CFLNodeIndex>)> {
    let (symbols, rules) = generate_symbols_rules(&sggraph.symbols);
    let (mut edges, out_ids) = generate_edges(sggraph);
    let mut metadata = generate_node_metadata(&sggraph.symbols, &sggraph.nodes, &out_ids);

    let mut pop_sg_to_cfl_out = HashMap::new();
    for (&sg_idx, &cfl_out) in &out_ids {
        let node = &sggraph.nodes[sg_idx as usize];
        if matches!(node, SGNode::Pop(_) | SGNode::PopScoped(_)) {
            pop_sg_to_cfl_out.insert(sg_idx, cfl_out);
        }
    }

    if simplify {
        simplify_graph(
            &mut edges,
            &mut metadata,
            sggraph.nodes.len() + out_ids.len(),
        );
    }

    let cfl_graph = CFLGraph {
        rules,
        edges,
        symbols,
        metadata,
        files: sggraph.files.clone(),
    };

    Ok((cfl_graph, pop_sg_to_cfl_out))
}
