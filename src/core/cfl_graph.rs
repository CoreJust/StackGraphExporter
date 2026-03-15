use std::collections::{HashMap, HashSet};

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
    pub paths: HashSet<CFLPath>,
    pub metadata: HashMap<CFLNodeIndex, CFLNodeMetadata>,
    pub files: Vec<String>,
}
