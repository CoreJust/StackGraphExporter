use super::indexers::{FileIndexer, NodeIdIndexer, SymbolIndexer};
use crate::core::{SGEdge, SGNode, SGNodeId, SGNodeIndex, SGSymbol, SGSymbolIndex};
use crate::error::{Error, Result};
use stack_graphs::serde::{Edge, Node, NodeID, SourceInfo};

pub struct Indexers<'a> {
    pub node_id_indexer: &'a mut NodeIdIndexer,
    pub file_indexer: &'a mut FileIndexer,
    pub symbol_indexer: &'a mut SymbolIndexer,
}

fn make_symbol(
    file: Option<usize>,
    symbol: &String,
    is_real: bool,
    source_info: &Option<SourceInfo>,
) -> SGSymbol {
    let (line, column) = source_info
        .as_ref()
        .map(|si| &si.span.start)
        .map(|s| (Some(s.line as usize), Some(s.column.utf8_offset as usize)))
        .unwrap_or((None, None));
    SGSymbol {
        name: symbol.clone(),
        real: is_real,
        file,
        line,
        column,
    }
}

fn make_symbol_node(
    indexers: Indexers,
    id: &NodeID,
    symbol: &String,
    is_real: bool,
    source_info: &Option<SourceInfo>,
) -> (SGNodeIndex, SGSymbolIndex) {
    let file = id
        .file
        .as_deref()
        .map(|f| indexers.file_indexer.index_of(f));
    let node_id = SGNodeId {
        file: file.clone(),
        local_id: id.local_id,
    };
    let node_index = indexers.node_id_indexer.index_of(node_id);
    let sym = make_symbol(file, symbol, is_real, source_info);
    let symbol_index = indexers.symbol_indexer.index_of(sym);
    (node_index, symbol_index)
}

fn make_scoped_symbol_node(
    indexers: Indexers,
    id: &NodeID,
    scope: &NodeID,
    symbol: &String,
    is_real: bool,
    source_info: &Option<SourceInfo>,
) -> (SGNodeIndex, SGNodeId, SGSymbolIndex) {
    let file = id
        .file
        .as_deref()
        .map(|f| indexers.file_indexer.index_of(f));
    let node_id = SGNodeId {
        file: file.clone(),
        local_id: id.local_id,
    };
    let node_index = indexers.node_id_indexer.index_of(node_id);
    let sym = make_symbol(file, symbol, is_real, source_info);
    let symbol_index = indexers.symbol_indexer.index_of(sym);

    let scope_file = scope
        .file
        .as_deref()
        .map(|f| indexers.file_indexer.index_of(f));
    let scope_id = SGNodeId {
        file: scope_file,
        local_id: scope.local_id,
    };
    (node_index, scope_id, symbol_index)
}

pub fn parse_node(node: &Node, indexers: Indexers) -> Result<(SGNodeIndex, SGNode)> {
    match node {
        Node::Scope {
            id, is_exported, ..
        } => {
            let file = id
                .file
                .as_deref()
                .map(|f| indexers.file_indexer.index_of(f));
            let local_id = id.local_id;
            let node_id = SGNodeId { file, local_id };
            let node_index = indexers.node_id_indexer.index_of(node_id);
            let sg_node = SGNode::Scope(*is_exported);
            Ok((node_index, sg_node))
        }
        Node::Root { id, .. } => {
            let file = id
                .file
                .as_deref()
                .map(|f| indexers.file_indexer.index_of(f));
            let local_id = id.local_id;
            let node_id = SGNodeId { file, local_id };
            let node_index = indexers.node_id_indexer.index_of(node_id);
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
            let (node_index, symbol_index) =
                make_symbol_node(indexers, id, symbol, *is_reference, source_info);
            Ok((node_index, SGNode::Push(symbol_index)))
        }
        Node::PopSymbol {
            id,
            symbol,
            is_definition,
            source_info,
            ..
        } => {
            let (node_index, symbol_index) =
                make_symbol_node(indexers, id, symbol, *is_definition, source_info);
            Ok((node_index, SGNode::Pop(symbol_index)))
        }
        Node::PushScopedSymbol {
            id,
            symbol,
            scope,
            is_reference,
            source_info,
            ..
        } => {
            let (node_index, scope_id, symbol_index) =
                make_scoped_symbol_node(indexers, id, scope, symbol, *is_reference, source_info);
            Ok((
                node_index,
                SGNode::PushScopedUnresolved(symbol_index, scope_id),
            ))
        }
        Node::PopScopedSymbol {
            id,
            symbol,
            is_definition,
            source_info,
            ..
        } => {
            let (node_index, symbol_index) =
                make_symbol_node(indexers, id, symbol, *is_definition, source_info);
            Ok((node_index, SGNode::PopScoped(symbol_index)))
        }
        Node::JumpToScope { id, .. } => {
            let file = id
                .file
                .as_deref()
                .map(|f| indexers.file_indexer.index_of(f));
            let local_id = id.local_id;
            let node_id = SGNodeId { file, local_id };
            let node_index = indexers.node_id_indexer.index_of(node_id);
            let sg_node = SGNode::JumpTo;
            Ok((node_index, sg_node))
        }
        Node::DropScopes { id, .. } => {
            let file = id
                .file
                .as_deref()
                .map(|f| indexers.file_indexer.index_of(f));
            let local_id = id.local_id;
            let node_id = SGNodeId { file, local_id };
            let node_index = indexers.node_id_indexer.index_of(node_id);
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
