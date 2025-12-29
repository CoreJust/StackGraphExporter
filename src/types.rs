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

pub type CFLSymbolIndex = usize;
pub type CFLNodeIndex = usize;

pub enum CFLSymbol {
    Terminal(CFLSymbolIndex),
    NonTerminal(CFLSymbolIndex),
}

pub struct CFLRule {
    pub(crate) from_non_terminal: CFLSymbolIndex,
    pub(crate) to: Vec<CFLSymbol>,
}

#[derive(Clone)]
pub struct CFLEdge {
    pub(crate) symbol: Option<CFLSymbolIndex>,
    pub(crate) from: CFLNodeIndex,
    pub(crate) to: CFLNodeIndex,
}

pub struct CFLGraph {
    pub(crate) rules: Vec<CFLRule>,
    pub(crate) edges: Vec<CFLEdge>,
    pub(crate) symbols: Vec<String>,
    pub(crate) nodes_count: usize,
}
