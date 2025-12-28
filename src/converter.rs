use crate::types::{CFLEdge, CFLGraph, SGGraph};
use anyhow::Result;
use std::collections::HashSet;

pub fn convert_to_cfl(sggraph: &SGGraph) -> Result<CFLGraph> {
    let edges = Vec::<CFLEdge>::new();
    return Ok(CFLGraph {
        rules: vec![],
        edges,
        nodes_count: sggraph.nodes.len() as u64,
    });
}
