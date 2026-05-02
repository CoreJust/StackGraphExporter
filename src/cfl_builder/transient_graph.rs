// Transient graph is a temporary representation between SGGraph and CFLGraph
// It is similar to CFLGraph, but instead of labeled edges it still uses
// labeled nodes. The node indices here are preserved from SGGraph.

use std::collections::HashSet;

use crate::core::{CFLNodeMetadata, CFLRuleIndex, CFLSymbolIndex};

pub type TNodeIndex = u32;

#[derive(Debug, Clone)]
pub struct TNode {
    pub symbol: Option<CFLSymbolIndex>,
    pub metadata: Option<CFLNodeMetadata>,

    // No self-loops are allowed
    pub incoming: Vec<TNodeIndex>,
    pub outcoming: Vec<TNodeIndex>,
}

#[derive(Debug)]
pub struct TGraph {
    pub nodes: Vec<TNode>,
    pub files: Vec<String>,
    pub potentially_virtual_rules: HashSet<CFLRuleIndex>, // Those might contain virtual symbols and must not be queried
    pub sg_to_cfl_rule_index: Vec<CFLRuleIndex>,
    pub cfl_push_pop_rules_count: u32,
    pub edges_count: usize,
}

impl TNode {
    #[allow(dead_code)]
    pub fn to_be_removed(&self) -> bool {
        self.incoming.is_empty() && self.outcoming.is_empty()
    }

    pub fn is_push(&self) -> bool {
        match self.symbol {
            Some(index) => TGraph::is_push(index),
            None => false,
        }
    }

    pub fn is_pop(&self) -> bool {
        match self.symbol {
            Some(index) => !TGraph::is_push(index),
            None => false,
        }
    }

    pub fn is_eps(&self) -> bool {
        self.symbol.is_none()
    }

    #[allow(dead_code)]
    pub fn is_real(&self) -> bool {
        match &self.metadata {
            Some(m) => m.is_real,
            None => false,
        }
    }

    pub fn rule(&self) -> Option<CFLRuleIndex> {
        match self.symbol {
            Some(index) => Some(index / 2),
            None => None,
        }
    }
}

impl TGraph {
    // Does not remove node physically - indices are not invalidated
    // Reconects edges of surrounding nodes
    pub fn remove_node(&mut self, index: TNodeIndex) {
        let incoming = std::mem::take(&mut self.nodes[index as usize].incoming);
        let outcoming = std::mem::take(&mut self.nodes[index as usize].outcoming);

        let insize = incoming.len();
        let outsize = outcoming.len();
        let mut added_edges = 0usize;

        for &in_node_index in &incoming {
            let in_node_out = &mut self.nodes[in_node_index as usize].outcoming;
            let pos = in_node_out
                .iter()
                .position(|&o| o == index)
                .expect("Incorrect graph: in-node's outcoming does not contain removed node");
            in_node_out.swap_remove(pos);

            for &out_node_index in &outcoming {
                if out_node_index != in_node_index {
                    in_node_out.push(out_node_index);
                    added_edges += 1;
                }
            }
            in_node_out.sort_unstable();
            in_node_out.dedup();
        }

        for &out_node_index in &outcoming {
            let out_node_in = &mut self.nodes[out_node_index as usize].incoming;
            let pos = out_node_in
                .iter()
                .position(|&i| i == index)
                .expect("Incorrect graph: out-node's incoming does not contain removed node");
            out_node_in.swap_remove(pos);

            for &in_node_index in &incoming {
                if in_node_index != out_node_index {
                    out_node_in.push(in_node_index);
                }
            }
            out_node_in.sort_unstable();
            out_node_in.dedup();
        }
        self.edges_count = self.edges_count - (insize + outsize) + added_edges;
    }

    // Does not remove node physically - indices are not invalidated
    // Does not preserve node's edges and forcefully removes them all
    pub fn tear_out_node(&mut self, index: TNodeIndex) {
        let incoming = std::mem::take(&mut self.nodes[index as usize].incoming);
        let outcoming = std::mem::take(&mut self.nodes[index as usize].outcoming);

        incoming.iter().for_each(|&i| {
            Self::remove_half_edge(&mut self.nodes[i as usize].outcoming, index);
        });
        outcoming.iter().for_each(|&i| {
            Self::remove_half_edge(&mut self.nodes[i as usize].incoming, index);
        });
        self.edges_count -= incoming.len() + outcoming.len();
    }

    pub fn remove_edge(&mut self, from: TNodeIndex, to: TNodeIndex) {
        Self::remove_half_edge(&mut self.nodes[from as usize].outcoming, to);
        Self::remove_half_edge(&mut self.nodes[to as usize].incoming, from);
        self.edges_count -= 1;
    }

    pub fn remove_half_edge(edges: &mut Vec<TNodeIndex>, to: TNodeIndex) {
        let edge_index = edges
            .iter()
            .position(|&i| i == to)
            .expect("Cannot remove edge which does not exist");
        edges.swap_remove(edge_index);
    }

    #[allow(dead_code)]
    pub fn get_symbol_name(index: CFLSymbolIndex) -> String {
        let rule_index = index / 2;
        if Self::is_push(index) {
            format!("psh{rule_index}")
        } else {
            format!("pp{rule_index}")
        }
    }

    pub fn is_push(index: CFLSymbolIndex) -> bool {
        index % 2 == 0
    }
}
