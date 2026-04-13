use super::indexers::{FileIndexer, NodeIdIndexer, SymbolIndexer};
use super::parsers::{parse_edge, parse_node};
use super::progress_event::ProgressEvent;
use super::resolver::resolve_push_scoped;
use crate::core::{SGGraph, SGNodeId};
use crate::error::Result;
use crate::sg_builder::parsers::Indexers;
use stack_graphs::arena::Handle;
use stack_graphs::graph::{Node as StackGraphNode, StackGraph};
use stack_graphs::partial::PartialPaths;
use stack_graphs::stitching::Database;
use std::collections::HashMap;
use std::time::{Duration, Instant};

const PROGRESS_ONCE_IN_N_NODES: usize = 128;
const PROGRESS_ONCE_IN_N_EDGES: usize = 128;

pub struct StackGraphContext {
    pub stack_graph: StackGraph,
    pub sggraph: SGGraph,
    pub node_handle_map: HashMap<SGNodeId, Handle<StackGraphNode>>,
    pub database: Option<(Database, PartialPaths)>,
    pub database_built_in: Option<Duration>,
}

pub fn build_sggraph<F>(graph: StackGraph, mut progress: F) -> Result<StackGraphContext>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let start = Instant::now();

    progress(ProgressEvent::SerializingStackGraph {
        elapsed: start.elapsed(),
    })?;
    let serializable = graph.to_serializable();

    let mut file_indexer = FileIndexer::new();
    let mut file_handle_to_index = HashMap::new();
    for file_handle in graph.iter_files() {
        let file = &graph[file_handle];
        let file_name = file.name();
        let file_index = file_indexer.index_of(file_name);
        file_handle_to_index.insert(file_handle, file_index);
    }

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
            Indexers {
                node_id_indexer: &mut node_id_indexer,
                symbol_indexer: &mut symbol_indexer,
                file_indexer: &mut file_indexer,
            },
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
        files: file_indexer.into_files(),
    };

    let mut node_handle_map = HashMap::new();
    for node_handle in graph.iter_nodes() {
        let node = &graph[node_handle];
        let (file_handle_opt, local_id) = (node.id().file(), node.id().local_id());
        let file_index = file_handle_opt.and_then(|fh| file_handle_to_index.get(&fh).copied());
        let node_id = SGNodeId {
            file: file_index,
            local_id,
        };
        node_handle_map.insert(node_id, node_handle);
    }

    progress(ProgressEvent::GraphBuilt {
        elapsed: start.elapsed(),
    })?;

    Ok(StackGraphContext {
        stack_graph: graph,
        sggraph,
        node_handle_map,
        database: None,
        database_built_in: None,
    })
}
