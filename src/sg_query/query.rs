use super::progress_event::ProgressEvent;
use crate::core::{SGNode, SGNodeIndex};
use crate::error::{Error, Result};
use crate::sg_builder::StackGraphContext;
use stack_graphs::partial::PartialPaths;
use stack_graphs::stitching::{
    Appendable, Database, DatabaseCandidates, ForwardPartialPathStitcher, StitcherConfig,
};
use stack_graphs::NoCancellation;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct ResolvedDefinition {
    pub file: String,
    pub line: usize,
    pub col: usize,
    pub local_id: u32,
}

impl StackGraphContext {
    pub fn find_reference_nodes_by_symbol(&self, name: &str) -> Vec<SGNodeIndex> {
        let mut result = Vec::new();
        let mut defs = 0;
        for (idx, node) in self.sggraph.nodes.iter().enumerate() {
            let symbol_idx = match node {
                SGNode::Push(s) | SGNode::PushScoped(s, _) => Some(*s),
                SGNode::Pop(s) | SGNode::PopScoped(s) => {
                    let sym = &self.sggraph.symbols[*s];
                    if sym.name == name && sym.real {
                        defs += 1;
                    }
                    None
                }
                _ => None,
            };
            if let Some(sym_idx) = symbol_idx {
                let sym = &self.sggraph.symbols[sym_idx];
                if sym.name == name && sym.real {
                    result.push(idx as SGNodeIndex);
                }
            }
        }
        crate::info!(
            "Found {} refs and {defs} defs for symbol {name}",
            result.len()
        );
        result
    }

    pub fn resolve_reference<F>(
        &mut self,
        node_index: SGNodeIndex,
        mut progress: F,
    ) -> Result<Vec<ResolvedDefinition>>
    where
        F: FnMut(ProgressEvent) -> Result<()>,
    {
        if self.database.is_none() {
            let start = Instant::now();
            progress(ProgressEvent::BuildingDatabase {
                elapsed: start.elapsed(),
            })?;
            self.build_database()?;
            progress(ProgressEvent::DatabaseBuilt {
                elapsed: start.elapsed(),
            })?;
        }

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

        let results = end_nodes
            .into_iter()
            .map(|node_handle| {
                let node_id = self.stack_graph[node_handle].id();
                let file_handle_opt = node_id.file();
                let local_id = node_id.local_id();
                let file = file_handle_opt.map(|fh| {
                    let file_struct = &self.stack_graph[fh];
                    file_struct.name().to_string()
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
                ResolvedDefinition {
                    file: file.expect("An end node must have a file"),
                    line,
                    col,
                    local_id,
                }
            })
            .collect();

        progress(ProgressEvent::PathsStitched {
            elapsed: start.elapsed(),
        })?;
        Ok(results)
    }

    fn build_database(&mut self) -> Result<()> {
        let mut db = Database::new();
        let mut partials = PartialPaths::new();
        let stitcher_config = StitcherConfig::default()
            .with_detect_similar_paths(true)
            .with_collect_stats(false);

        for file_handle in self.stack_graph.iter_files() {
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

        self.database = Some((db, partials));
        Ok(())
    }
}
