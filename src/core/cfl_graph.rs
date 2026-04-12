use std::collections::HashMap;

use crate::core::SGNodeIndex;

pub type CFLSymbolIndex = usize;
pub type CFLNodeIndex = u32;
pub type CFLFileIndex = usize;

#[derive(Debug, Clone)]
pub enum CFLSymbol {
    Terminal(CFLSymbolIndex),
    NonTerminal(CFLSymbolIndex),
}

#[derive(Debug, Clone)]
pub struct CFLRule {
    pub from_non_terminal: CFLSymbolIndex,
    pub to: Vec<CFLSymbol>,
}

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
    pub rules: Vec<CFLRule>,
    pub edges: Vec<CFLEdge>,
    pub symbols: Vec<String>,
    pub metadata: HashMap<CFLNodeIndex, CFLNodeMetadata>,
    pub files: Vec<String>,
}
