use crate::core::{SGNodeId, SGNodeIndex, SGSymbol, SGSymbolIndex};
use std::collections::HashMap;

pub struct FileIndexer {
    name_to_index: HashMap<String, usize>,
    names: Vec<String>,
}

impl FileIndexer {
    pub fn new() -> Self {
        Self {
            name_to_index: HashMap::new(),
            names: Vec::new(),
        }
    }

    pub fn index_of(&mut self, name: &str) -> usize {
        if let Some(&idx) = self.name_to_index.get(name) {
            return idx;
        }
        let idx = self.names.len();
        self.names.push(name.to_string());
        self.name_to_index.insert(name.to_string(), idx);
        idx
    }

    pub fn get_index(&self, name: &str) -> Option<usize> {
        self.name_to_index.get(name).copied()
    }

    pub fn into_files(self) -> Vec<String> {
        self.names
    }
}

pub struct SymbolIndexer {
    symbol_to_index: HashMap<SGSymbol, usize>,
    symbols: Vec<SGSymbol>,
}

impl SymbolIndexer {
    pub fn new() -> Self {
        Self {
            symbol_to_index: HashMap::new(),
            symbols: Vec::new(),
        }
    }

    pub fn index_of(&mut self, symbol: SGSymbol) -> SGSymbolIndex {
        if let Some(&idx) = self.symbol_to_index.get(&symbol) {
            return idx;
        }
        let idx = self.symbols.len();
        self.symbols.push(symbol.clone());
        self.symbol_to_index.insert(symbol, idx);
        idx
    }

    pub fn into_symbols(self) -> Vec<SGSymbol> {
        self.symbols
    }
}

pub struct NodeIdIndexer {
    id_to_index: HashMap<SGNodeId, SGNodeIndex>,
    ids: Vec<SGNodeId>,
}

impl NodeIdIndexer {
    pub fn new() -> Self {
        Self {
            id_to_index: HashMap::new(),
            ids: Vec::new(),
        }
    }

    pub fn index_of(&mut self, id: SGNodeId) -> SGNodeIndex {
        if let Some(&idx) = self.id_to_index.get(&id) {
            return idx;
        }
        let idx = self.ids.len() as SGNodeIndex;
        self.ids.push(id.clone());
        self.id_to_index.insert(id, idx);
        idx
    }

    pub fn get_index(&self, id: &SGNodeId) -> Option<SGNodeIndex> {
        self.id_to_index.get(id).copied()
    }

    pub fn into_ids(self) -> Vec<SGNodeId> {
        self.ids
    }
}
