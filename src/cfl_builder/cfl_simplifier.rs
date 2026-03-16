use std::collections::HashMap;

use crate::core::{CFLEdge, CFLNodeIndex, CFLNodeMetadata, CFLSymbolIndex};

#[derive(Default)]
struct EdgeInfo {
    symbol: Option<CFLSymbolIndex>,
    other: CFLNodeIndex,
}

#[derive(Default)]
struct NodeInfo {
    incoming: Vec<EdgeInfo>,
    outcoming: Vec<EdgeInfo>,
}

fn collect_nodes_info(edges: &[CFLEdge], nodes_count: usize) -> Vec<NodeInfo> {
    let mut nodes_info = (0..nodes_count)
        .map(|_| NodeInfo::default())
        .collect::<Vec<_>>();
    for edge in edges {
        nodes_info[edge.from as usize].outcoming.push(EdgeInfo {
            symbol: edge.symbol,
            other: edge.to,
        });
        nodes_info[edge.to as usize].incoming.push(EdgeInfo {
            symbol: edge.symbol,
            other: edge.from,
        });
    }
    nodes_info
}

fn traverse_collapsed_group(
    nodes_info: &[NodeInfo],
    group_start: CFLNodeIndex,
) -> Vec<CFLNodeIndex> {
    let mut collapsed_group = vec![group_start];
    let mut node_index = group_start;
    loop {
        let node = &nodes_info[node_index as usize];
        if node.outcoming.len() != 1 || node.outcoming[0].symbol.is_some() {
            break;
        }
        node_index = node.outcoming[0].other;
        collapsed_group.push(node_index);
    }
    collapsed_group
}

fn find_collapsed_nodes(nodes_info: &[NodeInfo]) -> HashMap<CFLNodeIndex, CFLNodeIndex> {
    let mut collapsed_nodes = HashMap::new();
    for node in nodes_info.iter() {
        if node.incoming.len() != 1
            && node.outcoming.len() == 1
            && node.outcoming[0].symbol.is_none()
        {
            let other_node = &nodes_info[node.outcoming[0].other as usize];
            if other_node.incoming.len() == 1 {
                let mut collapsed_group =
                    traverse_collapsed_group(nodes_info, other_node.incoming[0].other);
                let group_end = collapsed_group.pop().unwrap();
                for collapsed_node in collapsed_group {
                    collapsed_nodes.insert(collapsed_node, group_end);
                }
            }
        }
    }
    collapsed_nodes
}

fn remove_edges(edges: &mut Vec<CFLEdge>, removed: &HashMap<CFLNodeIndex, CFLNodeIndex>) {
    for edge in edges.iter_mut() {
        edge.from = *removed.get(&edge.from).unwrap_or(&edge.from);
        edge.to = *removed.get(&edge.to).unwrap_or(&edge.to);
    }
    edges.retain(|e| e.from != e.to);
}

fn reindex_nodes(
    edges: &mut Vec<CFLEdge>,
    metadata: &mut HashMap<CFLNodeIndex, CFLNodeMetadata>,
    nodes_count: usize,
) {
    let mut node_to_node_index = HashMap::with_capacity(nodes_count);
    let mut counter = 0;
    let mut new_node_index = |index| {
        *node_to_node_index.entry(index).or_insert_with(|| {
            let new = counter;
            counter += 1;
            new
        })
    };

    for edge in edges {
        edge.from = new_node_index(edge.from);
        edge.to = new_node_index(edge.to);
    }

    let mut new_metadata = HashMap::with_capacity(metadata.len());
    for (old_idx, meta) in metadata.drain() {
        let new_idx = new_node_index(old_idx);
        new_metadata.insert(new_idx, meta);
    }
    *metadata = new_metadata;
}

pub fn simplify_graph(
    edges: &mut Vec<CFLEdge>,
    metadata: &mut HashMap<CFLNodeIndex, CFLNodeMetadata>,
    nodes_count: usize,
) {
    let nodes_info = collect_nodes_info(edges, nodes_count);
    let collapsed_nodes = find_collapsed_nodes(&nodes_info);
    let new_nodes_count = nodes_count - collapsed_nodes.len();
    remove_edges(edges, &collapsed_nodes);
    reindex_nodes(edges, metadata, new_nodes_count);
}
