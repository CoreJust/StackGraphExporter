use std::collections::HashSet;

use crate::core::{CFLGraph, CFLNodeIndex};

pub fn collect_start_nodes(graph: &CFLGraph, inverse_graph: bool) -> HashSet<CFLNodeIndex> {
    let mut start_nodes = HashSet::new();
    let potential_virtuals = graph
        .metadata
        .iter()
        .filter(|m| !m.1.is_real)
        .map(|m| &m.1.name)
        .collect::<HashSet<_>>();
    let expected_start_nodes_modulo = if inverse_graph { 1 } else { 0 };
    for edge in graph.edges.iter() {
        if edge.symbol.is_some()
            && edge.symbol.unwrap() % 2 == expected_start_nodes_modulo
            && graph
                .metadata
                .get(&edge.from)
                .and_then(|m| Some(m.is_real && !potential_virtuals.contains(&m.name)))
                .unwrap_or(false)
        {
            if inverse_graph {
                start_nodes.insert(edge.to);
            } else {
                start_nodes.insert(edge.from);
            }
        }
    }
    if !start_nodes.is_empty() {
        crate::debug!("Collected {} start nodes", start_nodes.len());
    }
    start_nodes
}
