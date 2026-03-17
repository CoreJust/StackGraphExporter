use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use stack_graphs::stitching::Appendable;

use crate::{
    artifacts::*,
    cfl_builder::convert_to_cfl,
    cfl_query::{kotgll_query, ucfs_query},
    core::{CFLGraph, CFLNodeIndex, SGFileIndex, SGNodeId, SGNodeIndex},
    error::{Error, Result},
    io::ProgressRenderer,
    loading::{load_stack_graph, Language},
    sg_builder::{build_sggraph, StackGraphContext},
    sg_query::ResolvedDefinition,
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
    pub kotgll_enabled: bool,
    pub ucfs_enabled: bool,
    pub query_mode: bool, // TODO: use it or remove it
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
    generated_artifacts: HashMap<ArtifactType, PathBuf>,
    context: Option<StackGraphContext>,
    cfl_graph: Option<CFLGraph>,
    cfl_pop_map: Option<HashMap<SGNodeIndex, CFLNodeIndex>>,
    cfl_graph_simplified: bool,
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
        if let Some(p) = args.output_dot {
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

        Self {
            stack_graph: None,
            language,
            kotgll_enabled: args.kotgll,
            ucfs_enabled: args.ucfs,
            query_mode: args.query,
            verify: args.verify,
            all_symbols: args.all_symbols,
            simplify_cfl: args.simplify_cfl,
            sppf: args.sppf,
            verbose: args.verbose,
            gen_cfg: args.cfg,
            gen_csv: args.csv,
            gen_dot: args.dot,
            gen_dot_ucfs: args.dot_ucfs,
            gen_kt: args.kt,
            gen_json: args.stack_graph_json,
            output_dir,
            output_overrides: overrides,
            kotgll_path: args.kotgll_path,
            generated_artifacts: HashMap::new(),
            context: None,
            cfl_graph: None,
            cfl_pop_map: None,
            cfl_graph_simplified: false,
            nodes_with_partials: HashSet::new(),
        }
    }

    pub fn load(&mut self, path: &Path) -> Result<()> {
        let mut renderer = ProgressRenderer::new();
        let graph = load_stack_graph(path, self.language.clone(), |e| renderer.render(&e))?;
        self.stack_graph = Some(graph);
        Ok(())
    }

    fn stack_graph(&self) -> &stack_graphs::graph::StackGraph {
        if let Some(ctx) = &self.context {
            return &ctx.stack_graph;
        }
        self.stack_graph.as_ref().expect("StackGraph not loaded")
    }

    pub fn ensure_context<'a>(&'a mut self) -> Result<&'a mut StackGraphContext> {
        if self.context.is_none() {
            let graph = self.stack_graph.take().expect("StackGraph not loaded");
            let mut renderer = ProgressRenderer::new();
            self.context = Some(build_sggraph(graph, |e| renderer.render(&e))?);
            let partials_starts = {
                let mut renderer = ProgressRenderer::new();
                let db = self
                    .context
                    .as_mut()
                    .unwrap()
                    .database(|e| renderer.render(&e))?;
                db.iter_partial_paths()
                    .map(|handle| db[handle].start_node())
                    .collect::<Vec<_>>()
            };
            let ctx = self.context.as_ref().unwrap();
            for start_node in partials_starts.into_iter() {
                let node_id_opt = ctx.node_handle_map.iter().find_map(|(id, &h)| {
                    if h == start_node {
                        Some(id)
                    } else {
                        None
                    }
                });
                if let Some(node_id) = node_id_opt {
                    let node_idx =
                        ctx.sggraph.ids.iter().position(|id| id == node_id).unwrap() as SGNodeIndex;
                    self.nodes_with_partials.insert(node_idx);
                }
            }
        }
        Ok(self.context.as_mut().unwrap())
    }

    fn ensure_cfl_graph<'a>(&'a mut self, simplify: bool) -> Result<&'a CFLGraph> {
        if self.cfl_graph.is_some() && simplify == self.simplify_cfl {
            Ok(self.cfl_graph.as_ref().unwrap())
        } else {
            (self.cfl_graph, self.cfl_pop_map) = {
                let ctx = self.ensure_context()?;
                let mut renderer = ProgressRenderer::new();
                let (graph, pop_map) =
                    convert_to_cfl(&ctx.sggraph, simplify, |e| renderer.render(&e))?;
                (Some(graph), Some(pop_map))
            };
            self.cfl_graph_simplified = simplify;
            Ok(self.cfl_graph.as_ref().unwrap())
        }
    }

    pub fn find_reference_nodes_by_symbol(&mut self, symbol: &str) -> Result<Vec<SGNodeIndex>> {
        let ctx = self.ensure_context()?;
        let refs = ctx.find_reference_nodes_by_symbol(symbol);
        if !self.all_symbols {
            Ok(refs
                .into_iter()
                .filter(|r| self.nodes_with_partials.contains(r))
                .collect())
        } else {
            Ok(refs)
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

    pub fn resolve_reference(&mut self, node_idx: SGNodeIndex) -> Result<Vec<ResolvedDefinition>> {
        let ctx = self.ensure_context()?;
        ctx.resolve_reference(node_idx, |_| Ok(()))
    }

    fn sg_index_from_def(&self, def: &ResolvedDefinition) -> Option<SGNodeIndex> {
        let ctx = self.context.as_ref()?;
        let file_idx = ctx
            .sggraph
            .files
            .iter()
            .position(|name| name == &def.file)
            .map(|i| i as SGFileIndex);
        let node_id = SGNodeId {
            file: file_idx,
            local_id: def.local_id,
        };
        ctx.sggraph
            .ids
            .iter()
            .position(|id| id == &node_id)
            .map(|i| i as SGNodeIndex)
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
            let refs = self
                .ensure_context()?
                .find_reference_nodes_by_symbol(symbol);
            let mut stack_defs = HashSet::new();
            for &r in &refs {
                let defs = self.ensure_context()?.resolve_reference(r, |_| Ok(()))?;
                for d in defs {
                    if let Some(sg_idx) = self.sg_index_from_def(&d) {
                        if let Some(cfl_idx) =
                            self.cfl_pop_map.as_ref().and_then(|m| m.get(&sg_idx))
                        {
                            stack_defs.insert(*cfl_idx);
                        }
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

    pub fn generate_ucfs_query(&mut self, symbol: &str, indices: &[u32]) -> Result<PathBuf> {
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

    pub fn generate_artifact(&mut self, artifact: ArtifactType) -> Result<()> {
        let path = self.output_path(artifact);
        self.generated_artifacts.insert(artifact, path.clone());
        match artifact {
            ArtifactType::Cfg => {
                let cfl = self.ensure_cfl_graph(self.simplify_cfl)?;
                cfl.write_to_grammar_file(&path)?;
            }
            ArtifactType::Csv => {
                let cfl = self.ensure_cfl_graph(self.simplify_cfl)?;
                cfl.write_to_csv_file(&path, false)?;
            }
            ArtifactType::Dot => {
                let ctx = self.ensure_context()?;
                ctx.sggraph.write_to_dot_file(&path, false)?;
            }
            ArtifactType::DotUcfs => {
                let cfl = self.ensure_cfl_graph(self.simplify_cfl)?;
                cfl.write_to_dot_file(&path, true)?;
            }
            ArtifactType::Kt => {
                let cfl = self.ensure_cfl_graph(self.simplify_cfl)?;
                cfl.write_to_kotlin_file(&path, "UCFSGrammar")?;
            }
            ArtifactType::Json => {
                let serializable = self.stack_graph().to_serializable();
                let json = serde_json::to_string_pretty(&serializable)?;
                std::fs::write(&path, json)?;
            }
        }
        Ok(())
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
                self.generate_artifact(artifact)?;
            }
        }
        Ok(())
    }

    fn output_path(&self, artifact: ArtifactType) -> PathBuf {
        if let Some(overridden) = self.output_overrides.get(&artifact) {
            overridden.clone()
        } else {
            let filename = match artifact {
                ArtifactType::Cfg => ".cfl_grammar.cfg",
                ArtifactType::Csv => ".cfl.csv",
                ArtifactType::Dot => ".stackgraph.dot",
                ArtifactType::DotUcfs => ".cfl_ucfs.dot",
                ArtifactType::Kt => ".cfl_grammar.kt",
                ArtifactType::Json => ".stackgraph.json",
            };
            self.output_dir.join(filename)
        }
    }
}
