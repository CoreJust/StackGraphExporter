#[derive(Clone)]
pub struct SGSymbol {
    pub(crate) name: String,
    pub(crate) real: bool, // Reference of definition
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct SGNodeId {
    pub(crate) file: Option<String>,
    pub(crate) local_id: u64,
}

pub enum SGNode {
    Scope(bool), // is exported
    Root,
    Push(SGSymbol),
    Pop(SGSymbol),
    JumpTo,
    PushScoped(SGSymbol, u64),
    PushScopedUnresolved(SGSymbol, SGNodeId),
    PopScoped(SGSymbol),
    DropScopes,
}

pub struct SGEdge {
    pub(crate) from: u64,
    pub(crate) to: u64,
    // precedence: Option<i64>?
}

pub struct SGGraph {
    pub(crate) nodes: Vec<SGNode>,
    pub(crate) edges: Vec<SGEdge>,
    pub(crate) ids: Vec<SGNodeId>,
}

pub struct CFLGraph {}
