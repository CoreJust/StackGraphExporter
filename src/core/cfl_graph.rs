use std::collections::{HashMap, HashSet};

use crate::core::SGNodeIndex;

pub type CFLSymbolIndex = u32;
pub type CFLRuleIndex = u32;
pub type CFLNodeIndex = u32;
pub type CFLFileIndex = u32;

#[derive(Debug, Clone)]
pub struct CFLEdge {
    pub symbol: Option<CFLSymbolIndex>,
    pub from: CFLNodeIndex,
    pub to: CFLNodeIndex,
}

#[derive(Debug, Clone)]
pub struct CFLNodeMetadata {
    pub name: String,
    #[allow(unused)]
    pub file: Option<CFLFileIndex>,
    #[allow(unused)]
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
    #[allow(unused)]
    pub files: Vec<String>,
    pub potentially_virtual_rules: HashSet<CFLRuleIndex>, // Those might contain virtual symbols and must not be queried
    pub sg_to_cfl_rule_index: Vec<CFLRuleIndex>,
    pub cfl_push_pop_rules_count: u32,
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
