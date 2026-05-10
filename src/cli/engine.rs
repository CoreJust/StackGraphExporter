use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use super::artifact_type::ArtifactType;
use crate::cfl_query::cfg_bench_query;
use crate::core::CFLPath;
use crate::{
    artifacts::*,
    cfl_builder::{convert_to_cfl, SimplificationOptions},
    cfl_query::{kotgll_query, ucfs_query},
    core::{CFLGraph, CFLNodeIndex, CFLRuleIndex, SGFileIndex, SGNodeIndex, SGSymbolIndex, Stats},
    error::{Error, Result},
    io::{ElapsedAndCount, ProgressRenderer},
    loading::{load_stack_graph, Language},
    sg_builder::{build_sggraph, StackGraphContext},
    sg_query::{ProgressEvent, QueryAllResult, QueryOneResult},
    unsupported_features_cleaner::clean_unsupported_features,
};

pub struct Engine {
    stack_graph: Option<stack_graphs::graph::StackGraph>,
    language: Language,
    pub remove_unsupported: bool,
    pub kotgll_enabled: bool,
    pub ucfs_enabled: bool,
    pub cfg_bench_enabled: bool,
    pub all_symbols: bool,
    pub inverse: bool,
    pub simplification_options: SimplificationOptions,
    pub used_simplification_options: SimplificationOptions,
    pub sppf: bool,
    pub verbose: bool,
    pub verify: bool,
    pub gen_cfg: bool,
    pub gen_csv: bool,
    pub gen_dot: bool,
    pub gen_dot_ucfs: bool,
    pub gen_kt: bool,
    pub gen_json: bool,
    pub gen_g: bool,
    pub gen_g_cfg: bool,
    pub gen_cnf: bool,
    pub gen_cnf_cfg: bool,
    pub output_dir: PathBuf,
    pub output_overrides: HashMap<ArtifactType, PathBuf>,
    pub kotgll_path: Option<PathBuf>,
    pub generated_artifacts: HashMap<ArtifactType, PathBuf>,
    pub stats: Stats,
    generated_for_query: HashSet<ArtifactType>,
    context: Option<StackGraphContext>,
    cfl_graph: Option<CFLGraph>,
    nodes_with_partials: HashSet<SGNodeIndex>,
}

impl Engine {
    pub fn new(args: crate::cli::args::OpenArgs) -> Result<Self> {
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
        if let Some(p) = args.output_g {
            overrides.insert(ArtifactType::G, p);
        }
        if let Some(p) = args.output_g_cfg {
            overrides.insert(ArtifactType::GCfg, p);
        }
        if let Some(p) = args.output_cnf {
            overrides.insert(ArtifactType::Cnf, p);
        }
        if let Some(p) = args.output_cnf_cfg {
            overrides.insert(ArtifactType::CnfCfg, p);
        }

        Ok(Self {
            stack_graph: None,
            remove_unsupported: args.remove_unsupported,
            language,
            kotgll_enabled: args.kotgll,
            ucfs_enabled: args.ucfs,
            cfg_bench_enabled: args.cfg_bench,
            all_symbols: args.all_symbols,
            inverse: args.inverse,
            simplification_options: SimplificationOptions::make(
                args.simplify,
                args.max_simplification_iterations,
                args.eps_removal_tolerance,
                args.remove_unreachable_trivial,
                args.remove_unreachable,
                args.remove_unreachable_with_front,
                args.remove_unreachable_deep,
            )?,
            sppf: args.sppf,
            verbose: args.verbose,
            verify: args.verify,
            gen_cfg: args.cfg,
            gen_csv: args.csv,
            gen_dot: args.stack_graph_dot,
            gen_dot_ucfs: args.dot_ucfs,
            gen_kt: args.kt,
            gen_json: args.stack_graph_json,
            gen_g: args.g,
            gen_g_cfg: args.g_cfg,
            gen_cnf: args.cnf,
            gen_cnf_cfg: args.cnf_cfg,
            output_dir,
            output_overrides: overrides,
            kotgll_path: args.kotgll_path,
            generated_artifacts: HashMap::new(),
            stats: Stats {
                ..Default::default()
            },
            generated_for_query: HashSet::new(),
            context: None,
            cfl_graph: None,
            used_simplification_options: SimplificationOptions::no_simpify(),
            nodes_with_partials: HashSet::new(),
        })
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
        self.stats.stack_graph.built_in = built_in.as_millis() as u64;
        self.stats.project_path = path.display().to_string();
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
            self.stats.stack_graph.vertices = ctx.sggraph.nodes.len();
            self.stats.stack_graph.edges = ctx.sggraph.edges.len();
            self.stats.stack_graph.symbols = ctx.sggraph.symbols.len();
            self.context = Some(ctx);
        }
        Ok(self.context.as_mut().unwrap())
    }

    fn ensure_cfl_graph<'a>(&'a mut self) -> Result<&'a CFLGraph> {
        if matches!(self.language, Language::Python) {
            return Err(Error::CflConversion(
                "CFL conversion is only available for language with static types; Python is not one - cannot generate CFLGraph"
                    .into()));
        }
        let simplify = self.simplification_options.clone();
        if self.cfl_graph.is_some() && self.used_simplification_options == simplify {
            if self.verbose {
                crate::debug!("ensure_cfl_graph: CFL graph already exists, returning it (simplified? {simplify:?})");
            }
            Ok(self.cfl_graph.as_ref().unwrap())
        } else {
            self.cfl_graph = {
                let ctx = self.ensure_context()?;
                let mut renderer = ProgressRenderer::new();
                let (graph, built_in) =
                    convert_to_cfl(&ctx.sggraph, &simplify, |e| renderer.render(&e))?;
                let vertices_count = graph
                    .edges
                    .iter()
                    .map(|e| e.from.max(e.to))
                    .max()
                    .unwrap_or(0);
                crate::info!(
                    "Generated CFL graph size: {vertices_count} vertices, {} edges; {} rules",
                    graph.edges.len(),
                    graph.cfl_push_pop_rules_count * 2 + 1,
                );
                let cfl_stats = if simplify.simplify {
                    &mut self.stats.cfl_graph_simplified
                } else {
                    &mut self.stats.cfl_graph
                };
                cfl_stats.built_in = built_in.as_millis() as u64;
                cfl_stats.vertices = vertices_count as usize;
                cfl_stats.edges = graph.edges.len();
                cfl_stats.grammar.rules = graph.cfl_push_pop_rules_count * 2 + 1;
                Some(graph)
            };
            self.used_simplification_options = simplify;
            Ok(self.cfl_graph.as_ref().unwrap())
        }
    }

    pub fn rule_index_of_symbol(&self, index: SGSymbolIndex) -> CFLRuleIndex {
        let result = self.cfl_graph.as_ref().unwrap().sg_to_cfl_rule_index[index as usize];
        assert_ne!(
            result,
            CFLRuleIndex::MAX,
            "The rule for sg symbol was eliminated during simplification",
        );
        result
    }

    pub fn grab_rule_index_of_symbol_mapping(&mut self) -> Vec<CFLRuleIndex> {
        std::mem::take(&mut self.cfl_graph.as_mut().unwrap().sg_to_cfl_rule_index)
    }

    pub fn query_all_symbols(&mut self) -> Result<QueryAllResult> {
        let ctx = self.ensure_context()?;
        let mut renderer = ProgressRenderer::new();
        ctx.resolve_all_references(|e| renderer.render(&e))
    }

    pub fn query_all_symbols_by_one(&mut self, needed_at_most: u32) -> Result<Vec<QueryOneResult>> {
        let ctx = self.ensure_context()?;
        let mut renderer = ProgressRenderer::new();
        let refs = ctx.find_reference_nodes(None, |e| renderer.render(&e))?;
        let mut refs = refs
            .into_iter()
            .filter(|r| self.nodes_with_partials.contains(&r))
            .collect::<Vec<_>>();
        let mut rng = StdRng::seed_from_u64(42);
        refs.shuffle(&mut rng);
        let mut result = Vec::new();
        let ctx = self.ensure_context()?;
        let start = Instant::now();
        let total_refs = refs.len();
        for (i, r) in refs.into_iter().enumerate() {
            let resolution_result = ctx.resolve_reference(r, false, |_| Ok(()))?;
            if !resolution_result.defs.is_empty() {
                let second_resolution_result = ctx.resolve_reference(r, false, |_| Ok(()))?;
                result.push(
                    // Ensure more stable results
                    if second_resolution_result.resolved_in < resolution_result.resolved_in {
                        second_resolution_result
                    } else {
                        resolution_result
                    },
                );
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
        resolution: &QueryOneResult,
        result: &mut [Duration],
    ) -> Result<()> {
        let ctx = self.ensure_context()?;
        for r in result {
            let resolution_result =
                ctx.resolve_reference(resolution.node_index, false, |_| Ok(()))?;
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
        if !self.simplification_options.simplify {
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
        let file = node_id
            .file
            .and_then(|f| ctx.sggraph.files.get(f as usize).cloned());
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

    pub fn resolve_reference(&mut self, node_idx: SGNodeIndex) -> Result<QueryOneResult> {
        let ctx = self.ensure_context()?;
        let mut renderer = ProgressRenderer::new();
        let result = ctx.resolve_reference(node_idx, true, |e| renderer.render(&e));
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

    fn verify_query_results(&mut self, results: HashSet<CFLPath>) -> Result<()> {
        let sg_start_indices = {
            let cfl = self.cfl_graph.as_ref().unwrap();
            results
                .iter()
                .map(|path| cfl.metadata[&path.from].sg_node_index)
                .collect::<Vec<_>>()
        };
        let mut renderer = ProgressRenderer::new();
        let expected_sg_end_indices = {
            let ctx = self.ensure_context()?;
            sg_start_indices
                .iter()
                .map(|i| {
                    ctx.resolve_reference(*i, false, |e| renderer.render(&e))
                        .expect("Failed to resolve reference")
                        .defs
                        .iter()
                        .map(|d| d.sg_node_index)
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<HashSet<_>>()
        };
        let expected_cfl_end_indices = {
            let cfl = self.cfl_graph.as_ref().unwrap();
            let end_in_indices = expected_sg_end_indices
                .iter()
                .map(|i| cfl.metadata[&i].sg_node_index)
                .collect::<HashSet<_>>();
            cfl.edges
                .iter()
                .filter_map(|e| {
                    if end_in_indices.contains(&e.from) {
                        Some(e.to)
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>()
        };
        let actual_cfl_end_indices = results.into_iter().map(|p| p.to).collect::<HashSet<_>>();
        if actual_cfl_end_indices == expected_cfl_end_indices {
            crate::success!("Verification successful");
        } else {
            crate::error!("Verification failed!\nKotGLL end indices are: {actual_cfl_end_indices:?}\nStack graphs end indices are: {expected_cfl_end_indices:?}");
        }
        Ok(())
    }

    pub fn kotgll_query(&mut self, symbol: &str) -> Result<()> {
        if !self.kotgll_enabled {
            return Err(Error::Internal("KotGLL backend not enabled".into()));
        }
        self.generate_artifact_or_get_cached(ArtifactType::Cfg, true)?;
        self.generate_artifact_or_get_cached(ArtifactType::Csv, true)?;

        let sg_symbol_index = self
            .ensure_context()?
            .sggraph
            .symbols
            .iter()
            .position(|sym| sym.name == symbol)
            .expect("No such symbol") as SGSymbolIndex;
        let rule_index = self.rule_index_of_symbol(sg_symbol_index);
        let mut renderer = ProgressRenderer::new();
        let results = kotgll_query(
            self.kotgll_path
                .as_ref()
                .expect("No KotGLL path was provided; add --kotgll-path with path to JAR"),
            &self.generated_artifacts[&ArtifactType::Cfg],
            &self.generated_artifacts[&ArtifactType::Csv],
            &self.output_dir,
            rule_index,
            self.sppf,
            |e| renderer.render(&e),
        )?
        .into_iter()
        .collect::<HashSet<_>>();
        self.verify_query_results(results)
    }

    pub fn generate_ucfs_query(
        &mut self,
        symbol: &str,
        indices: &[u32],
    ) -> Result<(PathBuf, PathBuf)> {
        if !self.ucfs_enabled {
            return Err(Error::Internal("UCFS backend not enabled".into()));
        }
        let grammar_path = self.generate_artifact_or_get_cached(ArtifactType::Kt, true)?;
        let dot_path = self.generate_artifact_or_get_cached(ArtifactType::DotUcfs, true)?;
        let output_dir = self.output_dir.clone();
        let sg_symbol_index = self
            .ensure_context()?
            .sggraph
            .symbols
            .iter()
            .position(|sym| sym.name == symbol)
            .expect("No such symbol") as SGSymbolIndex;
        let rule_index = self.rule_index_of_symbol(sg_symbol_index);
        let mut renderer = ProgressRenderer::new();
        ucfs_query(
            &grammar_path,
            &dot_path,
            &output_dir,
            rule_index,
            indices,
            |e| renderer.render(&e),
        )
    }

    pub fn generate_cfg_bench_query(
        &mut self,
        symbol: &str,
        indices: &[u32],
    ) -> Result<(PathBuf, PathBuf)> {
        if !self.cfg_bench_enabled {
            return Err(Error::Internal("CFG_bench backend not enabled".into()));
        }
        let cnf_path = self.generate_artifact_or_get_cached(ArtifactType::CnfCfg, true)?;
        let g_path = self.generate_artifact_or_get_cached(ArtifactType::GCfg, true)?;
        let sg_symbol_index = self
            .ensure_context()?
            .sggraph
            .symbols
            .iter()
            .position(|sym| sym.name == symbol)
            .expect("No such symbol") as SGSymbolIndex;
        let rule_index = self.rule_index_of_symbol(sg_symbol_index);
        let mut renderer = ProgressRenderer::new();
        cfg_bench_query(&cnf_path, &g_path, rule_index, indices, |e| {
            renderer.render(&e)
        })?;
        Ok((g_path, cnf_path))
    }

    fn generate_artifact_or_get_cached(
        &mut self,
        artifact: ArtifactType,
        for_query_generation: bool,
    ) -> Result<PathBuf> {
        if self.generated_artifacts.contains_key(&artifact)
            && self.generated_for_query.contains(&artifact) == for_query_generation
        {
            Ok(self.generated_artifacts[&artifact].clone())
        } else {
            self.generate_artifact(artifact, for_query_generation)
        }
    }

    pub fn generate_artifact(
        &mut self,
        artifact: ArtifactType,
        for_query_generation: bool,
    ) -> Result<PathBuf> {
        let path = self.output_path(artifact);
        self.generated_artifacts.insert(artifact, path.clone());
        if for_query_generation {
            self.generated_for_query.insert(artifact);
        } else {
            self.generated_for_query.remove(&artifact);
        }
        let inverse = self.inverse;
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
                let mut renderer = ProgressRenderer::new();
                ctx.sggraph
                    .write_to_dot_file(&path, false, false, false, |e| renderer.render(&e))?;
            }
            ArtifactType::DotUcfs => {
                let cfl = self.ensure_cfl_graph()?;
                let mut renderer = ProgressRenderer::new();
                cfl.write_to_dot_file(&path, true, for_query_generation, inverse, |e| {
                    renderer.render(&e)
                })?;
                let cfl_stats = if self.used_simplification_options.simplify {
                    &mut self.stats.cfl_graph_simplified
                } else {
                    &mut self.stats.cfl_graph
                };
                cfl_stats.path = path.display().to_string();
                cfl_stats.file_size = std::fs::metadata(&cfl_stats.path)?.len();
            }
            ArtifactType::Kt => {
                let cfl = self.ensure_cfl_graph()?;
                let mut renderer = ProgressRenderer::new();
                cfl.write_to_kotlin_file(
                    &path,
                    "UCFSGrammar",
                    for_query_generation,
                    inverse,
                    |e| renderer.render(&e),
                )?;
                let cfl_stats = if self.simplification_options.simplify {
                    &mut self.stats.cfl_graph_simplified
                } else {
                    &mut self.stats.cfl_graph
                };
                cfl_stats.grammar.path = path.display().to_string();
                cfl_stats.grammar.file_size = std::fs::metadata(&cfl_stats.grammar.path)?.len();
            }
            ArtifactType::Json => {
                let serializable = self.stack_graph().to_serializable();
                let json = serde_json::to_string_pretty(&serializable)?;
                std::fs::write(&path, json)?;
            }
            ArtifactType::G => {
                let cfl = self.ensure_cfl_graph()?;
                cfl.write_to_g_file(
                    &path,
                    for_query_generation,
                    GOrder::FromLabelTo,
                    false,
                    inverse,
                )?;
            }
            ArtifactType::GCfg => {
                let cfl = self.ensure_cfl_graph()?;
                cfl.write_to_g_file(
                    &path,
                    for_query_generation,
                    GOrder::FromToLabel,
                    true,
                    inverse,
                )?;
            }
            ArtifactType::Cnf => {
                let cfl = self.ensure_cfl_graph()?;
                cfl.write_to_cnf_file(&path, inverse)?;
            }
            ArtifactType::CnfCfg => {
                write_to_cnf_cfg_file(&path)?; // Graph has its symbols swapped instead
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
            (self.gen_g, ArtifactType::G),
            (self.gen_g_cfg, ArtifactType::GCfg),
            (self.gen_cnf, ArtifactType::Cnf),
            (self.gen_cnf_cfg, ArtifactType::CnfCfg),
        ];
        for (enabled, artifact) in artifacts {
            if enabled {
                let path = self.generate_artifact(artifact, false)?;
                crate::success!("Generated {artifact:?} at {}", path.display());
            }
        }
        Ok(())
    }

    pub fn output_path(&self, artifact: ArtifactType) -> PathBuf {
        if let Some(overridden) = self.output_overrides.get(&artifact) {
            overridden.clone()
        } else {
            self.output_dir.join(artifact.default_file_name())
        }
    }

    pub fn test(&mut self) -> Result<()> {
        let syms = self.ensure_context()?.sggraph.symbols.clone();
        for (i, s) in syms.iter().enumerate() {
            let cfl_rule = self.ensure_cfl_graph()?.sg_to_cfl_rule_index[i];
            crate::debug!("cfl {cfl_rule} -> {s:?}");
        }
        Ok(())
    }
}
