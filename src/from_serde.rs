use anyhow::{Error, Result};
use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::types::{SGEdge, SGGraph, SGNode, SGNodeId, SGNodeIndex, SGSymbol, SGSymbolIndex};

fn get_field<'a>(v: &'a Map<String, Value>, f: &str) -> Result<&'a Value> {
    return Ok(v
        .get(f)
        .ok_or(Error::msg(format!("Expected field {}", f)))?);
}

fn get_object<'a>(v: &'a Map<String, Value>, f: &str) -> Result<&'a Map<String, Value>> {
    return Ok(get_field(v, f)?
        .as_object()
        .ok_or(Error::msg(format!("Expected field {} to be object", f)))?);
}

fn get_array<'a>(v: &'a Map<String, Value>, f: &str) -> Result<&'a Vec<Value>> {
    return Ok(get_field(v, f)?
        .as_array()
        .ok_or(Error::msg(format!("Expected field {} to be array", f)))?);
}

fn get_str<'a>(v: &'a Map<String, Value>, f: &str) -> Result<&'a str> {
    return Ok(get_field(v, f)?
        .as_str()
        .ok_or(Error::msg(format!("Expected field {} to be string", f)))?);
}

fn get_u64(v: &Map<String, Value>, f: &str) -> Result<u64> {
    return Ok(get_field(v, f)?
        .as_u64()
        .ok_or(Error::msg(format!("Expected field {} to be u64", f)))?);
}

fn get_bool(v: &Map<String, Value>, f: &str) -> Result<bool> {
    return Ok(get_field(v, f)?
        .as_bool()
        .ok_or(Error::msg(format!("Expected field {} to be bool", f)))?);
}

pub trait FromSerde {
    type To;

    fn from_serde(value: Value) -> Result<Self::To>;
}

fn get_node_id_unresolved(node: &Map<String, Value>, f: &str) -> Result<SGNodeId> {
    let id = get_object(node, f)?;
    let local_id = get_u64(id, "local_id")? as SGNodeIndex;
    let file = get_str(id, "file").ok().and_then(|x| Some(x.to_owned()));
    return Ok(SGNodeId { file, local_id });
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

    fn get_node_id(self: &mut Self, node: &Map<String, Value>, f: &str) -> Result<SGNodeIndex> {
        return Ok(self.index_of(get_node_id_unresolved(node, f)?));
    }
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
}

fn node_from_serde(
    indexer: &mut SGNodeIdIndexer,
    symbol_indexer: &mut SGSymbolIndexer,
    node: &Map<String, Value>,
) -> Result<(SGNodeIndex, SGNode)> {
    let kind = get_str(node, "type")?;
    let id = indexer.get_node_id(node, "id")?;
    return Ok((
        id,
        match kind {
            "scope" => SGNode::Scope(get_bool(node, "is_exported")?),
            "root" => SGNode::Root,
            "push_symbol" => SGNode::Push(symbol_indexer.index_of(SGSymbol {
                name: get_str(node, "symbol")?.to_owned(),
                real: get_bool(node, "is_reference")?,
            })),
            "pop_symbol" => SGNode::Pop(symbol_indexer.index_of(SGSymbol {
                name: get_str(node, "symbol")?.to_owned(),
                real: get_bool(node, "is_definition")?,
            })),
            "push_scoped_symbol" => SGNode::PushScopedUnresolved(
                symbol_indexer.index_of(SGSymbol {
                    name: get_str(node, "symbol")?.to_owned(),
                    real: get_bool(node, "is_reference")?,
                }),
                get_node_id_unresolved(node, "scope")?,
            ),
            "pop_scoped_symbol" => SGNode::PopScoped(symbol_indexer.index_of(SGSymbol {
                name: get_str(node, "symbol")?.to_owned(),
                real: get_bool(node, "is_definition")?,
            })),
            "jump_to_scope" => SGNode::JumpTo,
            "drop_scopes" => SGNode::DropScopes,
            _ => return Err(Error::msg(format!("Unrecognized node kind: {}", kind))),
        },
    ));
}

fn edge_from_serde(indexer: &mut SGNodeIdIndexer, node: &Map<String, Value>) -> Result<SGEdge> {
    return Ok(SGEdge {
        from: indexer.get_node_id(&node, "source")?,
        to: indexer.get_node_id(&node, "sink")?,
    });
}

fn resolve_push_scoped(indexer: &mut SGNodeIdIndexer, nodes: &mut Vec<SGNode>) {
    for node in nodes.iter_mut() {
        if let SGNode::PushScopedUnresolved(symbol, scope) = node {
            *node = SGNode::PushScoped(symbol.clone(), indexer.index_of(scope.clone()));
        }
    }
}

impl FromSerde for SGGraph {
    type To = SGGraph;

    fn from_serde(value: Value) -> Result<Self::To> {
        let src_obj = value.as_object().ok_or(Error::msg(
            "Invalid serde convertion of StackGraph to JSON: root value must be object",
        ))?;
        let mut indexer = SGNodeIdIndexer {
            ..Default::default()
        };
        let mut symbol_indexer = SGSymbolIndexer {
            ..Default::default()
        };

        let mut nodes = Vec::<SGNode>::new();
        let src_nodes = get_array(&src_obj, "nodes")?;
        for src_node in src_nodes {
            let (_, node) = node_from_serde(
                &mut indexer,
                &mut symbol_indexer,
                src_node
                    .as_object()
                    .ok_or(Error::msg("Expected node to be object"))?,
            )?;
            nodes.push(node);
        }

        resolve_push_scoped(&mut indexer, &mut nodes);

        let mut edges = Vec::<SGEdge>::new();
        let src_edges = get_array(&src_obj, "edges")?;
        for src_edge in src_edges {
            edges.push(edge_from_serde(
                &mut indexer,
                src_edge
                    .as_object()
                    .ok_or(Error::msg("Expected edge to be object"))?,
            )?);
        }
        return Ok(SGGraph {
            nodes,
            edges,
            ids: indexer.ids,
            symbols: symbol_indexer.symbols,
        });
    }
}
