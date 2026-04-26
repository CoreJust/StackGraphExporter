use std::collections::HashMap;

use crate::core::SGNodeIndex;

pub type CFLSymbolIndex = usize;
pub type CFLRuleIndex = usize;
pub type CFLNodeIndex = u32;
pub type CFLFileIndex = usize;

#[derive(Debug, Clone)]
pub struct CFLEdge {
    pub symbol: Option<CFLSymbolIndex>,
    pub from: CFLNodeIndex,
    pub to: CFLNodeIndex,
}

#[derive(Debug, Clone)]
pub struct CFLNodeMetadata {
    pub name: String,
    pub file: Option<CFLFileIndex>,
    pub line: Option<usize>,
    pub is_real: bool,
    pub sg_node_index: SGNodeIndex, // Original index from SGGraph
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CFLPath {
    pub from: CFLNodeIndex,
    pub to: CFLNodeIndex,
}

#[derive(Debug)]
pub struct CFLGraph {
    pub edges: Vec<CFLEdge>,
    pub metadata: HashMap<CFLNodeIndex, CFLNodeMetadata>,
    pub files: Vec<String>,
    pub sg_to_cfl_rule_index: Vec<CFLRuleIndex>,
    pub sg_unique_symbols_count: usize,
}

impl CFLGraph {
    pub fn get_symbol_name(index: CFLSymbolIndex) -> String {
        let rule_index = index / 2;
        if index % 2 == 0 {
            format!("psh{rule_index}")
        } else {
            format!("pp{rule_index}")
        }
    }
}
