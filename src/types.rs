#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct SGSymbol {
    pub(crate) name: String,
    pub(crate) real: bool, // Reference of definition
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct SGNodeId {
    pub(crate) file: Option<String>,
    pub(crate) local_id: SGNodeIndex,
}

pub type SGNodeIndex = usize;
pub type SGSymbolIndex = usize;

pub enum SGNode {
    Scope(bool), // is exported
    Root,
    Push(SGSymbolIndex),
    Pop(SGSymbolIndex),
    JumpTo,
    PushScoped(SGSymbolIndex, SGNodeIndex),
    PushScopedUnresolved(SGSymbolIndex, SGNodeId),
    PopScoped(SGSymbolIndex),
    DropScopes,
}

pub struct SGEdge {
    pub(crate) from: SGNodeIndex,
    pub(crate) to: SGNodeIndex,
    // precedence: Option<i64>?
}

pub struct SGGraph {
    pub(crate) nodes: Vec<SGNode>,
    pub(crate) edges: Vec<SGEdge>,
    pub(crate) ids: Vec<SGNodeId>,
    pub(crate) symbols: Vec<SGSymbol>,
}

pub enum CFLSymbol {
    Terminal(String),
    NonTerminal(String),
}

pub struct CFLRule {
    pub(crate) from_non_terminal: String,
    pub(crate) to: Vec<CFLSymbol>,
}

pub struct CFLEdge {
    pub(crate) symbol: String,
    pub(crate) from: u64,
    pub(crate) to: u64,
}

pub struct CFLGraph {
    pub(crate) rules: Vec<CFLRule>,
    pub(crate) edges: Vec<CFLEdge>,
    pub(crate) nodes_count: u64,
}
