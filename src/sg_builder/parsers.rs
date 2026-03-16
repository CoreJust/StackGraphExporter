use super::indexers::{FileIndexer, NodeIdIndexer, SymbolIndexer};
use crate::core::{SGEdge, SGNode, SGNodeId, SGNodeIndex, SGSymbol};
use crate::error::{Error, Result};
use stack_graphs::serde::{Edge, Node, NodeID};

pub fn parse_node(
    node: &Node,
    node_id_indexer: &mut NodeIdIndexer,
    symbol_indexer: &mut SymbolIndexer,
    file_indexer: &mut FileIndexer,
) -> Result<(SGNodeIndex, SGNode)> {
    match node {
        Node::Scope {
            id, is_exported, ..
        } => {
            let file = id.file.as_deref().map(|f| file_indexer.index_of(f));
            let local_id = id.local_id;
            let node_id = SGNodeId { file, local_id };
            let node_index = node_id_indexer.index_of(node_id);
            let sg_node = SGNode::Scope(*is_exported);
            Ok((node_index, sg_node))
        }
        Node::Root { id, .. } => {
            let file = id.file.as_deref().map(|f| file_indexer.index_of(f));
            let local_id = id.local_id;
            let node_id = SGNodeId { file, local_id };
            let node_index = node_id_indexer.index_of(node_id);
            let sg_node = SGNode::Root;
            Ok((node_index, sg_node))
        }
        Node::PushSymbol {
            id,
            symbol,
            is_reference,
            source_info,
            ..
        } => {
            let file = id.file.as_deref().map(|f| file_indexer.index_of(f));
            let local_id = id.local_id;
            let node_id = SGNodeId { file, local_id };
            let node_index = node_id_indexer.index_of(node_id.clone());
            let line = source_info.as_ref().map(|si| si.span.start.line as usize);
            let sym = SGSymbol {
                name: symbol.clone(),
                real: *is_reference,
                file: node_id.file,
                line,
            };
            let symbol_index = symbol_indexer.index_of(sym);
            let sg_node = SGNode::Push(symbol_index);
            Ok((node_index, sg_node))
        }
        Node::PopSymbol {
            id,
            symbol,
            is_definition,
            source_info,
            ..
        } => {
            let file = id.file.as_deref().map(|f| file_indexer.index_of(f));
            let local_id = id.local_id;
            let node_id = SGNodeId { file, local_id };
            let node_index = node_id_indexer.index_of(node_id.clone());
            let line = source_info.as_ref().map(|si| si.span.start.line as usize);
            let sym = SGSymbol {
                name: symbol.clone(),
                real: *is_definition,
                file: node_id.file,
                line,
            };
            let symbol_index = symbol_indexer.index_of(sym);
            let sg_node = SGNode::Pop(symbol_index);
            Ok((node_index, sg_node))
        }
        Node::PushScopedSymbol {
            id,
            symbol,
            scope,
            is_reference,
            source_info,
            ..
        } => {
            let file = id.file.as_deref().map(|f| file_indexer.index_of(f));
            let local_id = id.local_id;
            let node_id = SGNodeId { file, local_id };
            let node_index = node_id_indexer.index_of(node_id.clone());
            let line = source_info.as_ref().map(|si| si.span.start.line as usize);
            let sym = SGSymbol {
                name: symbol.clone(),
                real: *is_reference,
                file: node_id.file,
                line,
            };
            let symbol_index = symbol_indexer.index_of(sym);

            let scope_file = scope.file.as_deref().map(|f| file_indexer.index_of(f));
            let scope_id = SGNodeId {
                file: scope_file,
                local_id: scope.local_id,
            };
            let sg_node = SGNode::PushScopedUnresolved(symbol_index, scope_id);
            Ok((node_index, sg_node))
        }
        Node::PopScopedSymbol {
            id,
            symbol,
            is_definition,
            source_info,
            ..
        } => {
            let file = id.file.as_deref().map(|f| file_indexer.index_of(f));
            let local_id = id.local_id;
            let node_id = SGNodeId { file, local_id };
            let node_index = node_id_indexer.index_of(node_id.clone());
            let line = source_info.as_ref().map(|si| si.span.start.line as usize);
            let sym = SGSymbol {
                name: symbol.clone(),
                real: *is_definition,
                file: node_id.file,
                line,
            };
            let symbol_index = symbol_indexer.index_of(sym);
            let sg_node = SGNode::PopScoped(symbol_index);
            Ok((node_index, sg_node))
        }
        Node::JumpToScope { id, .. } => {
            let file = id.file.as_deref().map(|f| file_indexer.index_of(f));
            let local_id = id.local_id;
            let node_id = SGNodeId { file, local_id };
            let node_index = node_id_indexer.index_of(node_id);
            let sg_node = SGNode::JumpTo;
            Ok((node_index, sg_node))
        }
        Node::DropScopes { id, .. } => {
            let file = id.file.as_deref().map(|f| file_indexer.index_of(f));
            let local_id = id.local_id;
            let node_id = SGNodeId { file, local_id };
            let node_index = node_id_indexer.index_of(node_id);
            let sg_node = SGNode::DropScopes;
            Ok((node_index, sg_node))
        }
    }
}

pub fn parse_edge(
    edge: &Edge,
    node_id_indexer: &NodeIdIndexer,
    file_indexer: &FileIndexer,
) -> Result<SGEdge> {
    let from = parse_node_ref(&edge.source, node_id_indexer, file_indexer)?;
    let to = parse_node_ref(&edge.sink, node_id_indexer, file_indexer)?;
    Ok(SGEdge { from, to })
}

fn parse_node_ref(
    node_ref: &NodeID,
    node_id_indexer: &NodeIdIndexer,
    file_indexer: &FileIndexer,
) -> Result<SGNodeIndex> {
    let file = node_ref
        .file
        .as_deref()
        .and_then(|f| file_indexer.get_index(f));
    let node_id = SGNodeId {
        file,
        local_id: node_ref.local_id,
    };
    node_id_indexer
        .get_index(&node_id)
        .ok_or_else(|| Error::CflConversion(format!("Node not found: {:?}", node_id)))
}
