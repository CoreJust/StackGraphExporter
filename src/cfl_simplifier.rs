use std::collections::{HashMap, HashSet};

use crate::core::{CFLEdge, CFLNodeIndex, CFLNodeMetadata, CFLPath, CFLSymbolIndex};

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

fn collect_nodes_info(edges: &Vec<CFLEdge>, nodes_count: usize) -> Vec<NodeInfo> {
    let mut nodes_info = (0..nodes_count)
        .map(|_| NodeInfo {
            ..Default::default()
        })
        .collect::<Vec<_>>();
    for edge in edges {
        nodes_info[edge.from as usize].outcoming.push(EdgeInfo {
            symbol: edge.symbol.clone(),
            other: edge.to,
        });
        nodes_info[edge.to as usize].incoming.push(EdgeInfo {
            symbol: edge.symbol.clone(),
            other: edge.from,
        });
    }
    nodes_info
}

fn traverse_collapsed_group(
    nodes_info: &Vec<NodeInfo>,
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

fn find_collapsed_nodes(nodes_info: &Vec<NodeInfo>) -> HashMap<CFLNodeIndex, CFLNodeIndex> {
    let mut collapsed_nodes = HashMap::<CFLNodeIndex, CFLNodeIndex>::new();
    for node in nodes_info.iter().filter(|n| {
        n.incoming.len() != 1 && n.outcoming.len() == 1 && n.outcoming[0].symbol.is_none()
    }) {
        let other_node = &nodes_info[node.outcoming[0].other as usize];
        if other_node.incoming.len() == 1 {
            let mut collapsed_group =
                traverse_collapsed_group(nodes_info, other_node.incoming[0].other);
            let group_end = collapsed_group.pop().unwrap();
            for collapsed_node in &collapsed_group {
                collapsed_nodes.insert(*collapsed_node, group_end);
            }
        }
    }
    collapsed_nodes
}

fn remove_edges(edges: &mut Vec<CFLEdge>, removed: &HashMap<CFLNodeIndex, CFLNodeIndex>) {
    edges.iter_mut().for_each(|e| {
        e.from = *removed.get(&e.from).unwrap_or(&e.from);
        e.to = *removed.get(&e.to).unwrap_or(&e.to);
    });
    *edges = edges
        .into_iter()
        .filter(|e| e.from != e.to)
        .map(|e| e.clone())
        .collect::<Vec<_>>();
}

fn reindex_nodes(
    edges: &mut Vec<CFLEdge>,
    metadata: &mut HashMap<CFLNodeIndex, CFLNodeMetadata>,
    paths: &mut HashSet<CFLPath>,
    nodes_count: usize,
) {
    let mut node_to_node_index = HashMap::<CFLNodeIndex, CFLNodeIndex>::with_capacity(nodes_count);
    let mut counter = 0;
    let mut new_node_index = |index| {
        node_to_node_index
            .get(&index)
            .and_then(|x| Some(*x))
            .or_else(|| {
                counter += 1;
                node_to_node_index.insert(index, counter - 1);
                Some(counter - 1)
            })
            .unwrap()
    };
    for edge in edges {
        edge.from = new_node_index(edge.from);
        edge.to = new_node_index(edge.to);
    }
    let mut new_metadata = HashMap::<CFLNodeIndex, CFLNodeMetadata>::with_capacity(metadata.len());
    for m in metadata.into_iter() {
        new_metadata.insert(new_node_index(*m.0), m.1.clone());
    }
    *metadata = new_metadata;
    let mut new_paths = HashSet::new();
    for path in paths.iter() {
        new_paths.insert(CFLPath {
            from: new_node_index(path.from),
            to: new_node_index(path.to),
        });
    }
    *paths = new_paths;
}

pub fn simplify_graph(
    edges: &mut Vec<CFLEdge>,
    metadata: &mut HashMap<CFLNodeIndex, CFLNodeMetadata>,
    paths: &mut HashSet<CFLPath>,
    mut nodes_count: usize,
) {
    let nodes_info = collect_nodes_info(&edges, nodes_count);
    let collapsed_nodes = find_collapsed_nodes(&nodes_info);
    nodes_count -= collapsed_nodes.len();
    remove_edges(edges, &collapsed_nodes);
    reindex_nodes(edges, metadata, paths, nodes_count);
}
