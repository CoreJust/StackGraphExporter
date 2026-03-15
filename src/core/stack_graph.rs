use std::collections::HashSet;

pub type SGNodeIndex = u32;
pub type SGSymbolIndex = usize;
pub type SGFileIndex = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SGSymbol {
    pub name: String,
    pub real: bool, // Reference of definition
    pub file: Option<SGFileIndex>,
    pub line: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SGNodeId {
    pub file: Option<SGFileIndex>,
    pub local_id: SGNodeIndex,
}

#[derive(Debug, Clone)]
pub enum SGNode {
    Scope(bool), // is_exported
    Root,
    Push(SGSymbolIndex),
    Pop(SGSymbolIndex),
    JumpTo,
    PushScoped(SGSymbolIndex, SGNodeIndex),
    PushScopedUnresolved(SGSymbolIndex, SGNodeId),
    PopScoped(SGSymbolIndex),
    DropScopes,
}

#[derive(Debug, Clone)]
pub struct SGEdge {
    pub from: SGNodeIndex,
    pub to: SGNodeIndex,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SGPath {
    pub from: SGNodeIndex,
    pub to: SGNodeIndex,
}

#[derive(Debug)]
pub struct SGGraph {
    pub nodes: Vec<SGNode>,
    pub edges: Vec<SGEdge>,
    pub ids: Vec<SGNodeId>,
    pub symbols: Vec<SGSymbol>,
    pub paths: HashSet<SGPath>,
    pub files: Vec<String>,
}
