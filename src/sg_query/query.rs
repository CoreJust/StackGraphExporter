use super::progress_event::ProgressEvent;
use crate::cfl_builder::get_symbol_of;
use crate::core::{SGFileIndex, SGNode, SGNodeId, SGNodeIndex, SGSymbolIndex};
use crate::error::{Error, Result};
use crate::io::ElapsedAndCount;
use crate::sg_builder::StackGraphContext;
use stack_graphs::partial::PartialPaths;
use stack_graphs::stitching::{
    Appendable, Database, DatabaseCandidates, ForwardPartialPathStitcher, StitcherConfig,
};
use stack_graphs::NoCancellation;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

const PROGRESS_ONCE_IN: usize = 128;

#[derive(Debug, Clone)]
pub struct ResolvedDefinition {
    pub file: String,
    pub line: usize,
    pub col: usize,
    pub local_id: u32,
    pub sg_node_index: SGNodeIndex,
}

#[derive(Debug, Clone)]
pub struct ResolutionResult {
    pub name: String,
    pub symbol_index: SGSymbolIndex,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub node_index: SGNodeIndex,
    pub resolved_in: Duration,
    pub defs: Vec<ResolvedDefinition>,
}

impl StackGraphContext {
    pub fn find_reference_nodes<F>(
        &self,
        by_symbol: Option<&str>,
        mut progress: F,
    ) -> Result<Vec<SGNodeIndex>>
    where
        F: FnMut(ProgressEvent) -> Result<()>,
    {
        let start = Instant::now();
        let mut result = Vec::new();
        let mut found_defs = 0;
        let total = self.sggraph.nodes.len();
        for (idx, node) in self.sggraph.nodes.iter().enumerate() {
            if idx % PROGRESS_ONCE_IN == 0 {
                if let Some(name) = by_symbol {
                    progress(ProgressEvent::LookingForSymbolReferences {
                        elapsed_and_count: ElapsedAndCount {
                            current: idx,
                            total,
                            elapsed: start.elapsed(),
                        },
                        symbol: name,
                    })?;
                } else {
                    progress(ProgressEvent::LookingForReferences(ElapsedAndCount {
                        current: idx,
                        total,
                        elapsed: start.elapsed(),
                    }))?;
                }
            }
            let symbol_idx = match node {
                SGNode::Push(s) | SGNode::PushScoped(s, _) => Some(*s),
                SGNode::Pop(s) | SGNode::PopScoped(s) => {
                    let sym = &self.sggraph.symbols[*s];
                    if (by_symbol.is_none() || sym.name == by_symbol.unwrap()) && sym.real {
                        found_defs += 1;
                    }
                    None
                }
                _ => None,
            };
            if let Some(sym_idx) = symbol_idx {
                let sym = &self.sggraph.symbols[sym_idx];
                if (by_symbol.is_none() || sym.name == by_symbol.unwrap()) && sym.real {
                    result.push(idx as SGNodeIndex);
                }
            }
        }
        if let Some(name) = by_symbol {
            progress(ProgressEvent::FoundSymbolReferences {
                elapsed: start.elapsed(),
                symbol: name,
                found_refs: result.len(),
                found_defs,
            })?;
        } else {
            progress(ProgressEvent::FoundReferences {
                elapsed: start.elapsed(),
                found_refs: result.len(),
                found_defs,
            })?;
        }
        Ok(result)
    }

    pub fn find_all_partial_starts<F>(&mut self, mut progress: F) -> Result<HashSet<SGNodeIndex>>
    where
        F: FnMut(ProgressEvent) -> Result<()>,
    {
        let db = self.database(&mut progress)?;
        let start = Instant::now();
        let total = db.iter_partial_paths().count();
        let mut partials_starts = Vec::with_capacity(total);
        for (i, handle) in db.iter_partial_paths().enumerate() {
            if i % PROGRESS_ONCE_IN == 0 {
                progress(ProgressEvent::FindingPartialStarts(ElapsedAndCount {
                    current: i,
                    total,
                    elapsed: start.elapsed(),
                }))?;
            }
            partials_starts.push(db[handle].start_node());
        }

        let total = self.sggraph.ids.len();
        let mut sg_id_to_index = HashMap::with_capacity(total);
        for (i, id) in self.sggraph.ids.iter().enumerate() {
            if i % PROGRESS_ONCE_IN == 0 {
                progress(ProgressEvent::BuildingNodeIdToPositionIndex(
                    ElapsedAndCount {
                        current: i,
                        total,
                        elapsed: start.elapsed(),
                    },
                ))?;
            }
            sg_id_to_index.insert(id, i as u32);
        }

        let total = self.node_handle_map.len();
        let mut handle_to_sg_index = HashMap::with_capacity(total);
        for (i, (sg_id, handle)) in self.node_handle_map.iter().enumerate() {
            if i % PROGRESS_ONCE_IN == 0 {
                progress(ProgressEvent::BuildingNodeHandleToPositionIndex(
                    ElapsedAndCount {
                        current: i,
                        total,
                        elapsed: start.elapsed(),
                    },
                ))?;
            }
            handle_to_sg_index.insert(handle, sg_id_to_index[&sg_id]);
        }
        let total = partials_starts.len();
        let mut result = HashSet::new();
        for (i, start_node) in partials_starts.into_iter().enumerate() {
            if i % PROGRESS_ONCE_IN == 0 {
                progress(ProgressEvent::CollectingNodesAtPartialStarts(
                    ElapsedAndCount {
                        current: i,
                        total,
                        elapsed: start.elapsed(),
                    },
                ))?;
            }
            let node_idx_opt = handle_to_sg_index.get(&start_node);
            if let Some(node_idx) = node_idx_opt {
                result.insert(*node_idx);
            }
        }
        progress(ProgressEvent::NodesAtPartialStartsIndexed {
            elapsed: start.elapsed(),
        })?;
        Ok(result)
    }

    pub fn sg_index_from_def(&self, file: &str, local_id: u32) -> Option<SGNodeIndex> {
        let file_idx = self
            .sggraph
            .files
            .iter()
            .position(|name| name == file)
            .map(|i| i as SGFileIndex);
        let node_id = SGNodeId {
            file: file_idx,
            local_id: local_id,
        };
        self.sggraph
            .ids
            .iter()
            .position(|id| *id == node_id)
            .map(|i| i as SGNodeIndex)
    }

    pub fn resolve_reference<F>(
        &mut self,
        node_index: SGNodeIndex,
        mut progress: F,
    ) -> Result<ResolutionResult>
    where
        F: FnMut(ProgressEvent) -> Result<()>,
    {
        let _ = self.database(&mut progress)?;
        let start = Instant::now();
        let node_id = &self.sggraph.ids[node_index as usize];
        let start_node_handle = self.node_handle_map.get(node_id).copied().ok_or_else(|| {
            Error::PathExtraction(format!(
                "No stack graph node handle for node index {}",
                node_index
            ))
        })?;

        if !self.stack_graph[start_node_handle].is_reference() {
            panic!("Passed a non-reference node handle to resolve_reference");
        }

        progress(ProgressEvent::StitchingPaths {
            elapsed: start.elapsed(),
        })?;

        let resolution_start = Instant::now();
        let (db, partials) = self.database.as_mut().unwrap();
        let mut db_candidates = DatabaseCandidates::new(&self.stack_graph, partials, db);
        let stitcher_config = StitcherConfig::default().with_detect_similar_paths(true);

        let mut end_nodes = std::collections::HashSet::new();
        ForwardPartialPathStitcher::find_all_complete_partial_paths(
            &mut db_candidates,
            vec![start_node_handle],
            stitcher_config,
            &NoCancellation,
            |g, _ps, p| {
                let node = &g[p.end_node()];
                if !node.is_definition() {
                    panic!("end_node was not a definition!");
                }
                end_nodes.insert(p.end_node());
            },
        )
        .map_err(|e| Error::PathExtraction(format!("Failed to find complete paths: {}", e)))?;

        let resolved_in = resolution_start.elapsed();
        let defs = end_nodes
            .into_iter()
            .map(|node_handle| {
                let node_id = self.stack_graph[node_handle].id();
                let file_handle_opt = node_id.file();
                let local_id = node_id.local_id();
                let file = file_handle_opt.map(|fh| {
                    let file_struct = &self.stack_graph[fh];
                    file_struct.name()
                });
                let (line, col) = self
                    .stack_graph
                    .source_info(node_handle)
                    .and_then(|si| {
                        Some((
                            si.span.start.line as usize,
                            si.span.start.column.utf8_offset as usize,
                        ))
                    })
                    .expect("An end node must have source info");
                let file = file.expect("An end node must have a file").to_string();
                let sg_node_index = self
                    .sg_index_from_def(file.as_str(), local_id)
                    .expect("Found definition has no corresponding node in SGGraph");
                ResolvedDefinition {
                    file,
                    line,
                    col,
                    local_id,
                    sg_node_index,
                }
            })
            .collect();

        progress(ProgressEvent::PathsStitched {
            elapsed: start.elapsed(),
        })?;
        let ref_symbol_index = get_symbol_of(&self.sggraph.nodes[node_index as usize])
            .expect("Resolved reference has no corresponding symbol in SGGraph");
        let symbol = &self.sggraph.symbols[ref_symbol_index];
        Ok(ResolutionResult {
            name: symbol.name.to_string(),
            symbol_index: ref_symbol_index,
            file: self.sggraph.files[symbol.file.unwrap()].clone(),
            line: symbol.line.unwrap(),
            column: symbol.column.unwrap(),
            node_index,
            resolved_in,
            defs,
        })
    }

    fn database<F>(&mut self, mut progress: F) -> Result<&Database>
    where
        F: FnMut(ProgressEvent) -> Result<()>,
    {
        if self.database.is_none() {
            let start = Instant::now();
            let mut db = Database::new();
            let mut partials = PartialPaths::new();
            let stitcher_config = StitcherConfig::default()
                .with_detect_similar_paths(true)
                .with_collect_stats(false);

            let total = self.sggraph.files.len(); // Same as in stack_graph
            for (i, file_handle) in self.stack_graph.iter_files().enumerate() {
                progress(ProgressEvent::BuildingDatabase(ElapsedAndCount {
                    current: i,
                    total,
                    elapsed: start.elapsed(),
                }))?;
                ForwardPartialPathStitcher::find_minimal_partial_path_set_in_file(
                    &self.stack_graph,
                    &mut partials,
                    file_handle,
                    stitcher_config,
                    &NoCancellation,
                    |_g, ps, p| {
                        db.add_partial_path(&self.stack_graph, ps, p.clone());
                    },
                )
                .map_err(|e| {
                    Error::PathExtraction(format!("Failed to build database for file: {}", e))
                })?;
            }

            progress(ProgressEvent::DatabaseBuilt {
                elapsed: start.elapsed(),
            })?;

            self.database = Some((db, partials));
            self.database_built_in = Some(start.elapsed());
        }
        Ok(&self.database.as_ref().unwrap().0)
    }
}
