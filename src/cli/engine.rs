use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::core::Stats;
use crate::io::ElapsedAndCount;
use crate::sg_query::{ProgressEvent, ResolutionResult};
use crate::unsupported_features_cleaner::clean_unsupported_features;
use crate::{
    artifacts::*,
    cfl_builder::convert_to_cfl,
    cfl_query::{kotgll_query, ucfs_query},
    core::{CFLGraph, CFLNodeIndex, SGFileIndex, SGNodeIndex},
    error::{Error, Result},
    io::ProgressRenderer,
    loading::{load_stack_graph, Language},
    sg_builder::{build_sggraph, StackGraphContext},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactType {
    Cfg,
    Csv,
    Dot,
    DotUcfs,
    Kt,
    Json,
}

pub struct Engine {
    stack_graph: Option<stack_graphs::graph::StackGraph>,
    language: Language,
    pub remove_unsupported: bool,
    pub kotgll_enabled: bool,
    pub ucfs_enabled: bool,
    pub verify: bool,
    pub all_symbols: bool,
    pub simplify_cfl: bool,
    pub sppf: bool,
    pub verbose: bool,
    pub gen_cfg: bool,
    pub gen_csv: bool,
    pub gen_dot: bool,
    pub gen_dot_ucfs: bool,
    pub gen_kt: bool,
    pub gen_json: bool,
    pub output_dir: PathBuf,
    pub output_overrides: HashMap<ArtifactType, PathBuf>,
    pub kotgll_path: Option<PathBuf>,
    pub generated_artifacts: HashMap<ArtifactType, PathBuf>,
    pub stats: Stats,
    context: Option<StackGraphContext>,
    cfl_graph: Option<CFLGraph>,
    cfl_pop_map: Option<HashMap<SGNodeIndex, CFLNodeIndex>>,
    pub cfl_graph_simplified: bool,
    nodes_with_partials: HashSet<SGNodeIndex>,
}

impl Engine {
    pub fn new(args: crate::cli::args::OpenArgs) -> Self {
        let language = if args.python {
            Language::Python
        } else {
            Language::Java
        };
        let output_dir = args.output.unwrap_or_else(|| PathBuf::from("."));
        let mut overrides = HashMap::new();
        if let Some(p) = args.output_cfg {
            overrides.insert(ArtifactType::Cfg, p);
        }
        if let Some(p) = args.output_csv {
            overrides.insert(ArtifactType::Csv, p);
        }
        if let Some(p) = args.output_stack_graph_dot {
            overrides.insert(ArtifactType::Dot, p);
        }
        if let Some(p) = args.output_dot_ucfs {
            overrides.insert(ArtifactType::DotUcfs, p);
        }
        if let Some(p) = args.output_kt {
            overrides.insert(ArtifactType::Kt, p);
        }
        if let Some(p) = args.output_stack_graph_json {
            overrides.insert(ArtifactType::Json, p);
        }

        let project_path = output_dir.display().to_string();
        Self {
            stack_graph: None,
            remove_unsupported: args.remove_unsupported,
            language,
            kotgll_enabled: args.kotgll,
            ucfs_enabled: args.ucfs,
            verify: args.verify,
            all_symbols: args.all_symbols,
            simplify_cfl: args.simplify_cfl,
            sppf: args.sppf,
            verbose: args.verbose,
            gen_cfg: args.cfg,
            gen_csv: args.csv,
            gen_dot: args.stack_graph_dot,
            gen_dot_ucfs: args.dot_ucfs,
            gen_kt: args.kt,
            gen_json: args.stack_graph_json,
            output_dir,
            output_overrides: overrides,
            kotgll_path: args.kotgll_path,
            generated_artifacts: HashMap::new(),
            stats: Stats {
                project_path,
                ..Default::default()
            },
            context: None,
            cfl_graph: None,
            cfl_pop_map: None,
            cfl_graph_simplified: false,
            nodes_with_partials: HashSet::new(),
        }
    }

    fn clean_unsupported_features(&mut self, path: &Path) -> Result<()> {
        let mut renderer = ProgressRenderer::new();
        clean_unsupported_features(path, &self.language, |e| renderer.render(&e))
    }

    pub fn load(&mut self, path: &Path) -> Result<()> {
        if self.remove_unsupported {
            self.clean_unsupported_features(path)?;
        }
        let mut renderer = ProgressRenderer::new();
        let (graph, built_in) = load_stack_graph(path, &self.language, |e| renderer.render(&e))?;
        self.stack_graph = Some(graph);
        self.stats.stack_gtaph.built_in = built_in.as_millis() as u64;
        Ok(())
    }

    fn stack_graph(&self) -> &stack_graphs::graph::StackGraph {
        if let Some(ctx) = &self.context {
            return &ctx.stack_graph;
        }
        self.stack_graph.as_ref().expect("StackGraph not loaded")
    }

    fn ensure_context<'a>(&'a mut self) -> Result<&'a mut StackGraphContext> {
        if self.context.is_none() {
            let graph = self.stack_graph.take().expect("StackGraph not loaded");
            let mut renderer = ProgressRenderer::new();
            let mut ctx = build_sggraph(graph, |e| renderer.render(&e))?;
            self.nodes_with_partials = ctx.find_all_partial_starts(|e| renderer.render(&e))?;
            crate::info!(
                "Generated SG graph size: {} vertices, {} edges; {} symbols",
                ctx.sggraph.nodes.len(),
                ctx.sggraph.edges.len(),
                ctx.sggraph.symbols.len(),
            );
            self.stats.stack_gtaph.vertices = ctx.sggraph.nodes.len();
            self.stats.stack_gtaph.edges = ctx.sggraph.edges.len();
            self.stats.stack_gtaph.symbols = ctx.sggraph.symbols.len();
            self.context = Some(ctx);
        }
        Ok(self.context.as_mut().unwrap())
    }

    fn ensure_cfl_graph<'a>(&'a mut self) -> Result<&'a CFLGraph> {
        let simplify = self.simplify_cfl;
        if self.cfl_graph.is_some() && self.cfl_graph_simplified == simplify {
            if self.verbose {
                crate::debug!("ensure_cfl_graph: CFL graph already exists, returning it (simplified? {simplify})");
            }
            Ok(self.cfl_graph.as_ref().unwrap())
        } else {
            (self.cfl_graph, self.cfl_pop_map) = {
                let ctx = self.ensure_context()?;
                let mut renderer = ProgressRenderer::new();
                let (graph, pop_map, built_in) =
                    convert_to_cfl(&ctx.sggraph, simplify, |e| renderer.render(&e))?;
                let vertices_count = graph
                    .edges
                    .iter()
                    .map(|e| e.from.max(e.to))
                    .max()
                    .unwrap_or(0);
                crate::info!(
                    "Generated CFL graph size: {vertices_count} vertices, {} edges; {} rules",
                    graph.edges.len(),
                    graph.rules.len(),
                );
                let cfl_stats = if simplify {
                    &mut self.stats.cfl_graph_simplified
                } else {
                    &mut self.stats.cfl_graph
                };
                cfl_stats.built_in = built_in.as_millis() as u64;
                cfl_stats.vertices = vertices_count as usize;
                cfl_stats.edges = graph.edges.len();
                self.stats.cfl_grammar.rules = graph.rules.len();
                (Some(graph), Some(pop_map))
            };
            self.cfl_graph_simplified = simplify;
            Ok(self.cfl_graph.as_ref().unwrap())
        }
    }

    pub fn query_all_symbols(&mut self, needed_at_most: u32) -> Result<Vec<ResolutionResult>> {
        let ctx = self.ensure_context()?;
        let mut renderer = ProgressRenderer::new();
        let refs = ctx.find_reference_nodes(None, |e| renderer.render(&e))?;
        let refs = refs
            .into_iter()
            .filter(|r| self.nodes_with_partials.contains(&r))
            .collect::<Vec<_>>();
        let mut result = Vec::new();
        let ctx = self.ensure_context()?;
        let start = Instant::now();
        let total_refs = refs.len();
        for (i, r) in refs.into_iter().enumerate() {
            let resolution_result = ctx.resolve_reference(r, |_| Ok(()))?;
            if !resolution_result.defs.is_empty() {
                result.push(resolution_result);
                renderer.render(&ProgressEvent::ResolvingSymbols {
                    elapsed_and_processed: ElapsedAndCount {
                        current: i,
                        total: total_refs,
                        elapsed: start.elapsed(),
                    },
                    found_resolvable_refs: result.len(),
                    needed_at_most,
                })?;
                if result.len() >= needed_at_most as usize {
                    break;
                }
            }
        }
        self.stats.partial_database_built_in =
            ctx.database_built_in.unwrap_or(Duration::ZERO).as_millis() as u64;
        Ok(result)
    }

    pub fn retry_query_for_durations(
        &mut self,
        resolution: &ResolutionResult,
        result: &mut [Duration],
    ) -> Result<()> {
        let ctx = self.ensure_context()?;
        for r in result {
            let resolution_result = ctx.resolve_reference(resolution.node_index, |_| Ok(()))?;
            *r = resolution_result.resolved_in;
        }
        Ok(())
    }

    pub fn find_reference_nodes_by_symbol(&mut self, symbol: &str) -> Result<Vec<SGNodeIndex>> {
        let ctx = self.ensure_context()?;
        let mut renderer = ProgressRenderer::new();
        let refs: Vec<SGNodeIndex> =
            ctx.find_reference_nodes(Some(symbol), |e| renderer.render(&e))?;
        if !self.all_symbols {
            Ok(refs
                .into_iter()
                .filter(|r| self.nodes_with_partials.contains(r))
                .collect())
        } else {
            Ok(refs)
        }
    }

    pub fn map_reference_nodes_to_cfl(
        &mut self,
        indices: &[SGNodeIndex],
    ) -> Result<Vec<CFLNodeIndex>> {
        if !self.simplify_cfl {
            // If graph wasn't simplified, then in-nodes have the same IDs
            // as in sggraph.
            Ok(indices.iter().map(|i| *i).collect::<Vec<CFLNodeIndex>>())
        } else {
            let verbose = self.verbose;
            let cfl_graph = self.ensure_cfl_graph()?;
            let sg_node_index_to_cfl = cfl_graph
                .metadata
                .iter()
                .map(|(cfl_idx, meta)| (meta.sg_node_index, *cfl_idx))
                .collect::<HashMap<SGNodeIndex, CFLNodeIndex>>();
            Ok(indices
                .iter()
                .map(|i| {
                    if verbose {
                        let idx = sg_node_index_to_cfl
                            .get(i)
                            .and_then(|i| Some(*i))
                            .unwrap_or(0);
                        crate::debug!(
                            "map_reference_nodes_to_cfl: Mapping {i} to {idx}, symbol {}",
                            &cfl_graph
                                .metadata
                                .get(&idx)
                                .and_then(|m| Some(m.name.as_str()))
                                .unwrap_or("none")
                        );
                    }
                    *sg_node_index_to_cfl.get(i).expect(
                        "SGNodeIndex doesn't correspond to any cfl node in the built mapping",
                    )
                })
                .collect::<Vec<CFLNodeIndex>>())
        }
    }

    pub fn get_node_location(
        &mut self,
        node_idx: SGNodeIndex,
    ) -> Result<(Option<String>, Option<usize>, Option<usize>)> {
        let ctx = self.ensure_context()?;
        let node_id = &ctx.sggraph.ids[node_idx as usize];
        let file = node_id.file.and_then(|f| ctx.sggraph.files.get(f).cloned());
        let line_col = ctx
            .node_handle_map
            .get(node_id)
            .and_then(|&handle| ctx.stack_graph.source_info(handle))
            .map(|si| {
                (
                    si.span.start.line as usize,
                    si.span.start.column.utf8_offset as usize,
                )
            });
        Ok((file, line_col.map(|(l, _)| l), line_col.map(|(_, c)| c)))
    }

    pub fn resolve_reference(&mut self, node_idx: SGNodeIndex) -> Result<ResolutionResult> {
        let ctx = self.ensure_context()?;
        let mut renderer = ProgressRenderer::new();
        let result = ctx.resolve_reference(node_idx, |e| renderer.render(&e));
        self.stats.partial_database_built_in =
            ctx.database_built_in.unwrap_or(Duration::ZERO).as_millis() as u64;
        result
    }

    pub fn find_node_at_source(
        &mut self,
        file: &str,
        line: usize,
        col: usize,
    ) -> Result<SGNodeIndex> {
        let ctx = self.ensure_context()?;
        let file_idx = ctx
            .sggraph
            .files
            .iter()
            .position(|f| f == file)
            .ok_or_else(|| Error::Internal(format!("File not found: {}", file)))?;

        for (idx, node_id) in ctx.sggraph.ids.iter().enumerate() {
            if node_id.file != Some(file_idx as SGFileIndex) {
                continue;
            }
            if let Some(handle) = ctx.node_handle_map.get(node_id) {
                if let Some(si) = ctx.stack_graph.source_info(*handle) {
                    let start_line = si.span.start.line as usize;
                    let start_col = si.span.start.column.utf8_offset as usize;
                    let end_line = si.span.end.line as usize;
                    let end_col = si.span.end.column.utf8_offset as usize;
                    if (line > start_line || (line == start_line && col >= start_col))
                        && (line < end_line || (line == end_line && col <= end_col))
                    {
                        return Ok(idx as SGNodeIndex);
                    }
                }
            }
        }
        Err(Error::Internal(format!(
            "No node found at {}:{}:{}",
            file, line, col
        )))
    }

    pub fn kotgll_query(&mut self, symbol: &str) -> Result<()> {
        if !self.kotgll_enabled {
            return Err(Error::Internal("KotGLL backend not enabled".into()));
        }
        if !self.generated_artifacts.contains_key(&ArtifactType::Cfg) {
            self.generate_artifact(ArtifactType::Cfg)?;
        }
        if !self.generated_artifacts.contains_key(&ArtifactType::Csv) {
            self.generate_artifact(ArtifactType::Csv)?;
        }
        let mut renderer = ProgressRenderer::new();
        let kotgll_defs = kotgll_query(
            self.kotgll_path
                .as_ref()
                .expect("No KotGLL path was provided; add --kotgll-path with path to JAR"),
            &self.generated_artifacts[&ArtifactType::Cfg],
            &self.generated_artifacts[&ArtifactType::Csv],
            &self.output_dir,
            symbol,
            self.sppf,
            |e| renderer.render(&e),
        )?;
        if self.verify {
            let mut renderer = ProgressRenderer::new();
            // TODO: pass references already acquired in the command_processor here
            let refs = self
                .ensure_context()?
                .find_reference_nodes(Some(symbol), |e| renderer.render(&e))?;
            let mut stack_defs = HashSet::new();
            for r in refs {
                let mut renderer = ProgressRenderer::new();
                let def_indices = self
                    .ensure_context()?
                    .resolve_reference(r, |e| renderer.render(&e))?
                    .defs
                    .into_iter()
                    .map(|d| d.sg_node_index)
                    .collect::<Vec<_>>();
                for sg_index in def_indices {
                    if let Some(cfl_idx) = self.cfl_pop_map.as_ref().and_then(|m| m.get(&sg_index))
                    {
                        stack_defs.insert(*cfl_idx);
                    }
                }
            }
            let cfl_defs: HashSet<_> = kotgll_defs.iter().map(|p| p.to).collect();
            if stack_defs == cfl_defs {
                crate::info!(
                    "Received same results from KotGLL and StackGraph: {} definitions found",
                    stack_defs.len()
                );
                if self.verbose {
                    for cfl_idx in &stack_defs {
                        crate::info!("  definition {}", self.node_metadata_as_str(*cfl_idx));
                    }
                }
            } else {
                crate::error!("Results for KotGLL and StackGraph differ");
                let missing: Vec<_> = stack_defs.difference(&cfl_defs).collect();
                let extra: Vec<_> = cfl_defs.difference(&stack_defs).collect();
                if !missing.is_empty() {
                    crate::info!("Missing in KotGLL ({}):", missing.len());
                    for cfl_idx in missing {
                        crate::warn!("  {}", self.node_metadata_as_str(*cfl_idx));
                    }
                }
                if !extra.is_empty() {
                    crate::warn!("Extra in KotGLL ({}):", extra.len());
                    for cfl_idx in extra {
                        crate::warn!("  {}", self.node_metadata_as_str(*cfl_idx));
                    }
                }
            }
            self.stats.partial_database_built_in = self
                .ensure_context()?
                .database_built_in
                .unwrap_or(Duration::ZERO)
                .as_millis() as u64;
        }
        Ok(())
    }

    fn node_metadata_as_str(&self, cfl_idx: CFLNodeIndex) -> String {
        if let Some(meta) = self
            .cfl_graph
            .as_ref()
            .and_then(|g| g.metadata.get(&cfl_idx))
        {
            let file_str = meta
                .file
                .and_then(|f| self.cfl_graph.as_ref().and_then(|g| g.files.get(f)))
                .map(String::as_str)
                .unwrap_or("");
            format!(
                " {} at {}:{} ({}node {})",
                meta.name,
                file_str,
                meta.line.unwrap_or(0) + 1,
                if meta.is_real { "" } else { "virtual " },
                cfl_idx,
            )
        } else {
            format!("  node {} (no metadata)", cfl_idx)
        }
    }

    pub fn generate_ucfs_query(
        &mut self,
        symbol: &str,
        indices: &[u32],
    ) -> Result<(PathBuf, PathBuf)> {
        if !self.ucfs_enabled {
            return Err(Error::Internal("UCFS backend not enabled".into()));
        }
        if !self.generated_artifacts.contains_key(&ArtifactType::Kt) {
            self.generate_artifact(ArtifactType::Kt)?;
        }
        if !self
            .generated_artifacts
            .contains_key(&ArtifactType::DotUcfs)
        {
            self.generate_artifact(ArtifactType::DotUcfs)?;
        }
        let grammar_path = self.generated_artifacts[&ArtifactType::Kt].clone();
        let dot_path = self.generated_artifacts[&ArtifactType::DotUcfs].clone();
        let output_dir = self.output_dir.clone();
        let mut renderer = ProgressRenderer::new();
        ucfs_query(
            &grammar_path,
            &dot_path,
            &output_dir,
            symbol,
            indices,
            |e| renderer.render(&e),
        )
    }

    pub fn generate_artifact(&mut self, artifact: ArtifactType) -> Result<PathBuf> {
        let path = self.output_path(artifact);
        self.generated_artifacts.insert(artifact, path.clone());
        match artifact {
            ArtifactType::Cfg => {
                let cfl = self.ensure_cfl_graph()?;
                cfl.write_to_grammar_file(&path)?;
            }
            ArtifactType::Csv => {
                let cfl = self.ensure_cfl_graph()?;
                cfl.write_to_csv_file(&path, false)?;
            }
            ArtifactType::Dot => {
                let ctx = self.ensure_context()?;
                ctx.sggraph.write_to_dot_file(&path, false)?;
            }
            ArtifactType::DotUcfs => {
                let cfl = self.ensure_cfl_graph()?;
                cfl.write_to_dot_file(&path, true)?;
                let cfl_stats = if self.cfl_graph_simplified {
                    &mut self.stats.cfl_graph_simplified
                } else {
                    &mut self.stats.cfl_graph
                };
                cfl_stats.path = path.display().to_string();
                cfl_stats.file_size = std::fs::metadata(&cfl_stats.path)?.len();
            }
            ArtifactType::Kt => {
                let cfl = self.ensure_cfl_graph()?;
                cfl.write_to_kotlin_file(&path, "UCFSGrammar")?;
                self.stats.cfl_grammar.path = path.display().to_string();
                self.stats.cfl_grammar.file_size =
                    std::fs::metadata(&self.stats.cfl_grammar.path)?.len();
            }
            ArtifactType::Json => {
                let serializable = self.stack_graph().to_serializable();
                let json = serde_json::to_string_pretty(&serializable)?;
                std::fs::write(&path, json)?;
            }
        }
        Ok(path)
    }

    pub fn generate_artifacts(&mut self) -> Result<()> {
        let artifacts = [
            (self.gen_cfg, ArtifactType::Cfg),
            (self.gen_csv, ArtifactType::Csv),
            (self.gen_dot, ArtifactType::Dot),
            (self.gen_dot_ucfs, ArtifactType::DotUcfs),
            (self.gen_kt, ArtifactType::Kt),
            (self.gen_json, ArtifactType::Json),
        ];
        for (enabled, artifact) in artifacts {
            if enabled {
                let path = self.generate_artifact(artifact)?;
                crate::success!("Generated {artifact:?} at {}", path.display());
            }
        }
        Ok(())
    }

    pub fn output_path(&self, artifact: ArtifactType) -> PathBuf {
        if let Some(overridden) = self.output_overrides.get(&artifact) {
            overridden.clone()
        } else {
            let filename = match artifact {
                ArtifactType::Cfg => "cfl_grammar.cfg",
                ArtifactType::Csv => "cfl.csv",
                ArtifactType::Dot => "stackgraph.dot",
                ArtifactType::DotUcfs => "cfl_ucfs.dot",
                ArtifactType::Kt => "UCFSGrammar.kt",
                ArtifactType::Json => "stackgraph.json",
            };
            self.output_dir.join(filename)
        }
    }
}
