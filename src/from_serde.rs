use anyhow::{Error, Result};
use serde_json::{Map, Value};
use stack_graphs::graph::StackGraph;
use std::collections::{HashMap, HashSet};

use crate::{
    sg_paths_extractor::extract_complete_paths_from,
    types::{
        SGEdge, SGFileIndex, SGGraph, SGNode, SGNodeId, SGNodeIndex, SGPath, SGSymbol,
        SGSymbolIndex,
    },
};

pub fn get_field<'a>(v: &'a Map<String, Value>, f: &str) -> Result<&'a Value> {
    return Ok(v
        .get(f)
        .ok_or(Error::msg(format!("Expected field {}", f)))?);
}

pub fn get_object<'a>(v: &'a Map<String, Value>, f: &str) -> Result<&'a Map<String, Value>> {
    return Ok(get_field(v, f)?
        .as_object()
        .ok_or(Error::msg(format!("Expected field {} to be object", f)))?);
}

pub fn get_array<'a>(v: &'a Map<String, Value>, f: &str) -> Result<&'a Vec<Value>> {
    return Ok(get_field(v, f)?
        .as_array()
        .ok_or(Error::msg(format!("Expected field {} to be array", f)))?);
}

pub fn get_str<'a>(v: &'a Map<String, Value>, f: &str) -> Result<&'a str> {
    return Ok(get_field(v, f)?
        .as_str()
        .ok_or(Error::msg(format!("Expected field {} to be string", f)))?);
}

pub fn get_u64(v: &Map<String, Value>, f: &str) -> Result<u64> {
    return Ok(get_field(v, f)?
        .as_u64()
        .ok_or(Error::msg(format!("Expected field {} to be u64", f)))?);
}

fn get_bool(v: &Map<String, Value>, f: &str) -> Result<bool> {
    return Ok(get_field(v, f)?
        .as_bool()
        .ok_or(Error::msg(format!("Expected field {} to be bool", f)))?);
}

fn get_node_id_unresolved(
    node: &Map<String, Value>,
    f: &str,
    file_indexer: &mut SGFileIndexer,
) -> Result<SGNodeId> {
    let id = get_object(node, f)?;
    let local_id = get_u64(id, "local_id")? as SGNodeIndex;
    let file = get_str(id, "file").ok().and_then(|x| Some(x));
    return Ok(SGNodeId {
        file: file_indexer.index_of(file),
        local_id,
    });
}

#[derive(Default)]
struct SGNodeIdIndexer {
    found_ids: HashMap<SGNodeId, SGNodeIndex>,
    ids: Vec<SGNodeId>,
}

impl SGNodeIdIndexer {
    fn index_of(self: &mut Self, id: SGNodeId) -> SGNodeIndex {
        if let Some(id64) = self.found_ids.get(&id) {
            return *id64;
        }
        let id64 = self.ids.len() as SGNodeIndex;
        self.ids.push(id.clone());
        self.found_ids.insert(id, id64);
        return id64;
    }

    fn get_node_id(
        self: &mut Self,
        node: &Map<String, Value>,
        f: &str,
        file_indexer: &mut SGFileIndexer,
    ) -> Result<SGNodeIndex> {
        return Ok(self.index_of(get_node_id_unresolved(node, f, file_indexer)?));
    }
}

fn get_source_line(node: &Map<String, Value>) -> Option<usize> {
    get_object(node, "source_info")
        .ok()
        .and_then(|si| get_object(si, "span").ok())
        .and_then(|si| get_object(si, "start").ok())
        .and_then(|s| get_u64(s, "line").ok())
        .and_then(|l| Some(l as usize))
}

fn get_symbol(
    node: &Map<String, Value>,
    is_real_field: &str,
    node_id: &SGNodeId,
) -> Result<SGSymbol> {
    Ok(SGSymbol {
        name: get_str(node, "symbol")?.to_owned(),
        real: get_bool(node, is_real_field)?,
        file: node_id.file,
        line: get_source_line(node),
    })
}

#[derive(Default)]
struct SGSymbolIndexer {
    found_symbols: HashMap<SGSymbol, SGSymbolIndex>,
    symbols: Vec<SGSymbol>,
}

impl SGSymbolIndexer {
    fn index_of(self: &mut Self, symbol: SGSymbol) -> SGSymbolIndex {
        if let Some(id64) = self.found_symbols.get(&symbol) {
            return *id64;
        }
        let id64 = self.symbols.len() as SGSymbolIndex;
        self.symbols.push(symbol.clone());
        self.found_symbols.insert(symbol, id64);
        return id64;
    }

    fn get_symbol(
        self: &mut Self,
        node: &Map<String, Value>,
        is_real_field: &str,
        node_id: &SGNodeId,
    ) -> Result<SGSymbolIndex> {
        Ok(self.index_of(get_symbol(node, is_real_field, node_id)?))
    }
}

#[derive(Default)]
struct SGFileIndexer {
    found_files: HashMap<String, SGFileIndex>,
    files: Vec<String>,
}

impl SGFileIndexer {
    fn index_of(self: &mut Self, file: Option<&str>) -> Option<SGFileIndex> {
        if let Some(file) = file {
            if let Some(id64) = self.found_files.get(file) {
                return Some(*id64);
            }
            let id64 = self.files.len() as SGSymbolIndex;
            self.files.push(file.to_owned());
            self.found_files.insert(file.to_owned(), id64);
            return Some(id64);
        }
        return None;
    }
}

fn node_from_serde(
    indexer: &mut SGNodeIdIndexer,
    symbol_indexer: &mut SGSymbolIndexer,
    file_indexer: &mut SGFileIndexer,
    node: &Map<String, Value>,
) -> Result<(SGNodeIndex, SGNode)> {
    let kind = get_str(node, "type")?;
    let id = indexer.get_node_id(node, "id", file_indexer)?;
    return Ok((
        id,
        match kind {
            "scope" => SGNode::Scope(get_bool(node, "is_exported")?),
            "root" => SGNode::Root,
            "push_symbol" => {
                SGNode::Push(symbol_indexer.get_symbol(node, "is_reference", &indexer.ids[id])?)
            }
            "pop_symbol" => {
                SGNode::Pop(symbol_indexer.get_symbol(node, "is_definition", &indexer.ids[id])?)
            }
            "push_scoped_symbol" => SGNode::PushScopedUnresolved(
                symbol_indexer.get_symbol(node, "is_reference", &indexer.ids[id])?,
                get_node_id_unresolved(node, "scope", file_indexer)?,
            ),
            "pop_scoped_symbol" => SGNode::PopScoped(symbol_indexer.get_symbol(
                node,
                "is_definition",
                &indexer.ids[id],
            )?),
            "jump_to_scope" => SGNode::JumpTo,
            "drop_scopes" => SGNode::DropScopes,
            _ => return Err(Error::msg(format!("Unrecognized node kind: {}", kind))),
        },
    ));
}

fn edge_from_serde(
    indexer: &mut SGNodeIdIndexer,
    file_indexer: &mut SGFileIndexer,
    node: &Map<String, Value>,
) -> Result<SGEdge> {
    return Ok(SGEdge {
        from: indexer.get_node_id(&node, "source", file_indexer)?,
        to: indexer.get_node_id(&node, "sink", file_indexer)?,
    });
}

fn resolve_push_scoped(indexer: &mut SGNodeIdIndexer, nodes: &mut Vec<SGNode>) {
    for node in nodes.iter_mut() {
        if let SGNode::PushScopedUnresolved(symbol, scope) = node {
            *node = SGNode::PushScoped(symbol.clone(), indexer.index_of(scope.clone()));
        }
    }
}

impl SGGraph {
    pub fn from_serde(graph: &StackGraph) -> Result<Self> {
        let value = serde_json::to_value(graph.to_serializable())?;
        let src_obj = value.as_object().ok_or(Error::msg(
            "Invalid serde convertion of StackGraph to JSON: root value must be object",
        ))?;
        let mut indexer = SGNodeIdIndexer {
            ..Default::default()
        };
        let mut symbol_indexer = SGSymbolIndexer {
            ..Default::default()
        };
        let mut file_indexer = SGFileIndexer {
            ..Default::default()
        };

        let src_nodes = get_array(&src_obj, "nodes")?;
        let mut nodes = Vec::<SGNode>::with_capacity(src_nodes.len());
        for src_node in src_nodes {
            let (_, node) = node_from_serde(
                &mut indexer,
                &mut symbol_indexer,
                &mut file_indexer,
                src_node
                    .as_object()
                    .ok_or(Error::msg("Expected node to be object"))?,
            )?;
            nodes.push(node);
        }

        resolve_push_scoped(&mut indexer, &mut nodes);

        let src_edges = get_array(&src_obj, "edges")?;
        let mut edges = Vec::<SGEdge>::with_capacity(src_edges.len());
        for src_edge in src_edges {
            edges.push(edge_from_serde(
                &mut indexer,
                &mut file_indexer,
                src_edge
                    .as_object()
                    .ok_or(Error::msg("Expected edge to be object"))?,
            )?);
        }

        let mut paths = HashSet::new();
        extract_complete_paths_from(graph, |from, to| {
            paths.insert(SGPath {
                from: indexer.index_of(SGNodeId {
                    file: file_indexer.index_of(from.file),
                    local_id: from.id,
                }),
                to: indexer.index_of(SGNodeId {
                    file: file_indexer.index_of(to.file),
                    local_id: to.id,
                }),
            });
        })?;
        return Ok(SGGraph {
            nodes,
            edges,
            ids: indexer.ids,
            symbols: symbol_indexer.symbols,
            paths,
            files: file_indexer.files,
        });
    }
}
