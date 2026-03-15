use crate::{
    cfl_simplifier::simplify_graph,
    core::{
        CFLEdge, CFLGraph, CFLNodeIndex, CFLNodeMetadata, CFLPath, CFLRule, CFLSymbol, SGEdge,
        SGGraph, SGNode, SGNodeIndex, SGSymbol, SGSymbolIndex,
    },
};
use anyhow::Result;
use std::collections::HashMap;

fn get_symbol_of(node: &SGNode) -> Option<SGSymbolIndex> {
    return match *node {
        SGNode::Pop(symbol) => Some(symbol),
        SGNode::Push(symbol) => Some(symbol),
        SGNode::PopScoped(symbol) => Some(symbol),
        SGNode::PushScoped(symbol, _) => Some(symbol),
        SGNode::PushScopedUnresolved(symbol, _) => Some(symbol),
        _ => None,
    };
}

fn is_push_node(node: &SGNode) -> bool {
    return match *node {
        SGNode::Push(_) => true,
        SGNode::PushScoped(_, _) => true,
        SGNode::PushScopedUnresolved(_, _) => true,
        _ => false,
    };
}

fn generate_out_ids(src_nodes: &Vec<SGNode>) -> HashMap<SGNodeIndex, CFLNodeIndex> {
    let mut out_ids = HashMap::<SGNodeIndex, CFLNodeIndex>::new();
    for (i, src_node) in src_nodes.iter().enumerate() {
        if get_symbol_of(src_node).is_some() {
            out_ids.insert(i as u32, out_ids.len() as u32 + src_nodes.len() as u32);
        }
    }
    out_ids
}

fn generate_node_metadata(
    symbols: &Vec<SGSymbol>,
    src_nodes: &Vec<SGNode>,
    out_ids: &HashMap<SGNodeIndex, CFLNodeIndex>,
) -> HashMap<CFLNodeIndex, CFLNodeMetadata> {
    let mut metadatas = HashMap::<CFLNodeIndex, CFLNodeMetadata>::with_capacity(out_ids.len());
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
    src_edges: &Vec<SGEdge>,
    src_nodes: &Vec<SGNode>,
    out_ids: &HashMap<SGNodeIndex, CFLNodeIndex>,
) -> Vec<CFLEdge> {
    let mut edges = Vec::<CFLEdge>::with_capacity(src_edges.len() + out_ids.len());
    for src_edge in src_edges {
        let from_node = &src_nodes[src_edge.from as usize];
        if let Some(_) = get_symbol_of(&from_node) {
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
    nodes: &Vec<SGNode>,
    out_ids: &HashMap<SGNodeIndex, CFLNodeIndex>,
) {
    for (in_id, out_id) in out_ids.iter() {
        if let Some(id) = get_symbol_of(&nodes[*in_id as usize]) {
            edges.push(CFLEdge {
                symbol: Some(
                    2 * id
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
    let mut out_ids = generate_out_ids(&sggraph.nodes); // IDs for future out_symbol nodes
    let mut edges = generate_for_current_edges(&sggraph.edges, &sggraph.nodes, &mut out_ids);
    generate_symbol_edges(&mut edges, &sggraph.nodes, &out_ids);
    (edges, out_ids)
}

fn generate_symbols_rules(symbols: &Vec<SGSymbol>) -> (Vec<String>, Vec<CFLRule>) {
    let mut rules = Vec::<CFLRule>::with_capacity(2 + symbols.len());
    let mut cfl_symbols = Vec::<String>::with_capacity(1 + 2 * symbols.len());
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

pub fn convert_to_cfl(sggraph: SGGraph, simplify: bool) -> Result<CFLGraph> {
    let (symbols, rules) = generate_symbols_rules(&sggraph.symbols);
    let (mut edges, out_ids) = generate_edges(&sggraph);
    let mut nodes_metadata = generate_node_metadata(&sggraph.symbols, &sggraph.nodes, &out_ids);
    let mut paths = sggraph
        .paths
        .iter()
        .map(|p| CFLPath {
            from: p.from,
            to: out_ids[&p.to],
        })
        .collect();
    if simplify {
        simplify_graph(
            &mut edges,
            &mut nodes_metadata,
            &mut paths,
            sggraph.nodes.len() + out_ids.len(),
        );
    }
    return Ok(CFLGraph {
        rules,
        edges,
        symbols,
        metadata: nodes_metadata,
        paths,
        files: sggraph.files,
    });
}
