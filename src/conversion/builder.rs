use super::indexers::{FileIndexer, NodeIdIndexer, SymbolIndexer};
use super::parsers::{parse_edge, parse_node};
use super::progress_event::ProgressEvent;
use super::resolver::resolve_push_scoped;
use crate::core::SGGraph;
use crate::error::Result;
use stack_graphs::graph::StackGraph;
use stack_graphs::serde::StackGraph as SerializableStackGraph;
use std::time::Instant;

const PROGRESS_ONCE_IN_N_NODES: usize = 128;
const PROGRESS_ONCE_IN_N_EDGES: usize = 128;

pub fn build_sggraph<F>(graph: &StackGraph, mut progress: F) -> Result<SGGraph>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let start = Instant::now();

    progress(ProgressEvent::SerializingStackGraph {
        elapsed: start.elapsed(),
    })?;
    let serializable: SerializableStackGraph = graph.to_serializable();

    let mut file_indexer = FileIndexer::new();
    let mut symbol_indexer = SymbolIndexer::new();
    let mut node_id_indexer = NodeIdIndexer::new();

    let total_nodes = serializable.nodes.data.len();
    let mut nodes = Vec::with_capacity(total_nodes);

    for (i, node) in serializable.nodes.data.iter().enumerate() {
        if i % PROGRESS_ONCE_IN_N_NODES == 0 {
            progress(ProgressEvent::ParsingNodes {
                current: i + 1,
                total: total_nodes,
                elapsed: start.elapsed(),
            })?;
        }
        let (node_index, sg_node) = parse_node(
            node,
            &mut node_id_indexer,
            &mut symbol_indexer,
            &mut file_indexer,
        )?;
        let idx = node_index as usize;
        if nodes.len() <= idx {
            nodes.resize(idx + 1, crate::core::SGNode::Root);
        }
        nodes[idx] = sg_node;
    }

    progress(ProgressEvent::ResolvingScopes {
        elapsed: start.elapsed(),
    })?;
    resolve_push_scoped(&mut nodes, &node_id_indexer);

    let total_edges = serializable.edges.data.len();
    let mut edges = Vec::with_capacity(total_edges);
    for (i, edge) in serializable.edges.data.iter().enumerate() {
        if i % PROGRESS_ONCE_IN_N_EDGES == 0 {
            progress(ProgressEvent::ParsingEdges {
                current: i + 1,
                total: total_edges,
                elapsed: start.elapsed(),
            })?;
        }
        edges.push(parse_edge(edge, &node_id_indexer, &file_indexer)?);
    }

    let sggraph = SGGraph {
        nodes,
        edges,
        ids: node_id_indexer.into_ids(),
        symbols: symbol_indexer.into_symbols(),
        paths: std::collections::HashSet::new(), // to be removed later
        files: file_indexer.into_files(),
    };

    progress(ProgressEvent::Done {
        elapsed: start.elapsed(),
    })?;
    Ok(sggraph)
}
