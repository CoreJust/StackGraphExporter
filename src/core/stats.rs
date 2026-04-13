use serde::Serialize;

use crate::core::{CFLNodeIndex, SGNodeIndex};

#[derive(Serialize, Default)]
pub struct StackGraphStats {
    pub built_in: u64,
    pub vertices: usize,
    pub edges: usize,
    pub symbols: usize,
}

#[derive(Serialize, Default)]
pub struct CFLGraphStats {
    pub path: String,
    pub file_size: u64,
    pub built_in: u64,
    pub vertices: usize,
    pub edges: usize,
}

#[derive(Serialize, Default)]
pub struct CFLGrammarStats {
    pub path: String,
    pub file_size: u64,
    pub rules: usize,
}

#[derive(Serialize)]
pub struct SymbolStats {
    pub name: String,
    pub sg_index: SGNodeIndex,
    pub cfl_index: CFLNodeIndex,
    pub file: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Serialize)]
pub struct DefinitionStats {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Serialize)]
pub struct QueryStats {
    pub symbol: SymbolStats,
    pub resolved_to: Vec<DefinitionStats>,
    pub resolution_time: [u64; 7],
}

#[derive(Serialize, Default)]
pub struct Stats {
    pub project_path: String,
    pub partial_database_built_in: u64,
    pub stack_gtaph: StackGraphStats,
    pub cfl_graph: CFLGraphStats,
    pub cfl_graph_simplified: CFLGraphStats,
    pub cfl_grammar: CFLGrammarStats,
    pub queries: Vec<QueryStats>,
}
