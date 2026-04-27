use std::{
    ffi::OsString,
    path::PathBuf,
    time::{Duration, Instant},
};

use rand::prelude::*;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use crate::{
    cli::engine::{ArtifactType, Engine},
    core::{CFLNodeIndex, DefinitionStats, QueryStats, SGNodeIndex, SymbolStats},
    error::{Error, Result},
    io::{ElapsedAndCount, ProgressRenderer},
    sg_query::{ProgressEvent, ResolutionResult},
};

#[derive(Debug, Clone)]
pub enum Command {
    Open {
        path: PathBuf,
    },
    Enable {
        feature: String,
    },
    Disable {
        feature: String,
    },
    Output {
        artifact: Option<ArtifactType>,
        path: PathBuf,
    },
    Create {
        artifact: Option<ArtifactType>,
    },
    Clean {
        artifact: Option<ArtifactType>,
    },
    QuerySymbol {
        symbol: String,
    },
    QueryNode {
        node: SGNodeIndex,
    },
    PickQueries {
        count: u32,
    },
    State,
    Help,
    Exit,
}

pub struct CommandProcessor {
    pub engine: Engine,
}

impl CommandProcessor {
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }

    pub fn process(&mut self, cmd: Command) -> Result<()> {
        match cmd {
            Command::Open { path } => self.cmd_open(path),
            Command::Enable { feature } => self.cmd_enable(&feature),
            Command::Disable { feature } => self.cmd_disable(&feature),
            Command::Output { artifact, path } => self.cmd_output(artifact, path),
            Command::Create { artifact } => self.cmd_create(artifact),
            Command::Clean { artifact } => self.cmd_clean(artifact),
            Command::QuerySymbol { symbol } => self.cmd_query_symbol(&symbol),
            Command::QueryNode { node } => self.cmd_query_node(node),
            Command::PickQueries { count } => self.cmd_pick_queries(count),
            Command::State => self.cmd_state(),
            Command::Help => self.cmd_help(),
            Command::Exit => self.cmd_exit(),
        }
    }

    fn cmd_open(&mut self, path: PathBuf) -> Result<()> {
        self.engine.load(&path)?;
        crate::success!("Loaded project at {}", path.display());
        Ok(())
    }

    fn cmd_enable(&mut self, feature: &str) -> Result<()> {
        match feature {
            "kotgll" => {
                self.engine.kotgll_enabled = true;
                self.engine.gen_cfg = true;
                self.engine.gen_csv = true;
            }
            "ucfs" => {
                self.engine.ucfs_enabled = true;
                self.engine.gen_dot_ucfs = true;
                self.engine.gen_kt = true;
            }
            "g" => self.engine.gen_g = true,
            "cnf" => self.engine.gen_cnf = true,
            "verify" => self.engine.verify = true,
            "all_symbols" | "all-symbols" => self.engine.all_symbols = true,
            "simplify" | "simplify-cfl" | "simplify_cfl" => self.engine.simplify_cfl = true,
            _ => {
                crate::error!("Unknown feature '{feature}'; Supported features: kotgll, ucfs, verify, all-symbols, simplify-cfl");
                return Ok(());
            }
        }
        crate::info!("Enabled {}", feature);
        Ok(())
    }

    fn cmd_disable(&mut self, feature: &str) -> Result<()> {
        match feature {
            "kotgll" => {
                self.engine.kotgll_enabled = false;
                self.engine.gen_cfg = false;
                self.engine.gen_csv = false;
            }
            "ucfs" => {
                self.engine.ucfs_enabled = false;
                self.engine.gen_dot_ucfs = false;
                self.engine.gen_kt = false;
            }
            "g" => self.engine.gen_g = false,
            "cnf" => self.engine.gen_cnf = false,
            "verify" => self.engine.verify = false,
            "all_symbols" | "all-symbols" => self.engine.all_symbols = false,
            "simplify" | "simplify-cfl" | "simplify_cfl" => self.engine.simplify_cfl = false,
            _ => {
                crate::error!("Unknown feature '{}'; Supported features: kotgll, ucfs, verify, all-symbols, simplify-cfl", feature);
                return Ok(());
            }
        }
        crate::info!("Disabled {}", feature);
        Ok(())
    }

    fn cmd_output(&mut self, artifact: Option<ArtifactType>, path: PathBuf) -> Result<()> {
        if let Some(art) = artifact {
            self.engine.output_overrides.insert(art, path);
            crate::success!("Output for {:?} set", art);
        } else {
            self.engine.output_dir = path;
            crate::success!("Default output directory set");
        }
        Ok(())
    }

    fn cmd_create(&mut self, artifact: Option<ArtifactType>) -> Result<()> {
        if let Some(art) = artifact {
            let path = self.engine.generate_artifact(art, false)?;
            crate::success!("Generated {art:?} at {}", path.display());
        } else {
            self.engine.generate_artifacts()?;
            crate::success!("Generated all enabled artifacts");
        }
        Ok(())
    }

    fn cmd_clean(&mut self, artifact: Option<ArtifactType>) -> Result<()> {
        if let Some(art) = artifact {
            self.engine.generated_artifacts.remove(&art);
            crate::info!("Marked artifact {art:?} as not generated");
        } else {
            self.engine.generated_artifacts.clear();
            crate::info!("Marked all artifact as not generated");
        }
        Ok(())
    }

    fn choose_indices_for_symbol(&mut self, symbol: &str) -> Result<Option<Vec<SGNodeIndex>>> {
        let refs = self.engine.find_reference_nodes_by_symbol(symbol)?;
        if refs.is_empty() {
            crate::warn!("No references found for symbol '{}'", symbol);
            return Ok(None);
        }

        crate::info!("Found {} references:", refs.len());
        for (i, &node_idx) in refs.iter().enumerate() {
            let (file, line, col) = self.engine.get_node_location(node_idx)?;
            println!(
                "  [{}] node {} at {}:{}:{}",
                i,
                node_idx,
                file.as_deref().unwrap_or("<unknown>"),
                line.unwrap_or(0),
                col.unwrap_or(0)
            );
        }

        println!("Enter index to resolve (or 'a' for all, empty to cancel):");
        let choice = crate::io::read_line("> ")?;
        if choice.is_empty() {
            return Ok(None);
        }
        if choice == "a" {
            Ok(Some(refs))
        } else {
            match choice.parse::<usize>() {
                Ok(idx) if idx < refs.len() => Ok(Some(vec![refs[idx]])),
                _ => Err(Error::InvalidArgument("Invalid selection.".into())),
            }
        }
    }

    fn cmd_query_symbol(&mut self, symbol: &str) -> Result<()> {
        let indices = self.choose_indices_for_symbol(symbol)?;
        if indices.is_none() {
            return Ok(());
        }
        let indices = indices.as_ref().unwrap();
        let mut unresolved = Vec::new();
        for (i, &node_idx) in indices.into_iter().enumerate() {
            let defs = self.engine.resolve_reference(node_idx)?.defs;
            if defs.is_empty() {
                unresolved.push((i, node_idx));
                continue;
            }
            crate::info!(
                "[{}] Node {} resolves to {} definitions:",
                i,
                node_idx,
                defs.len()
            );
            for def in defs {
                println!(
                    "  - {}:{}:{} local_id {}",
                    def.file, def.line, def.col, def.local_id
                );
            }
        }

        if !unresolved.is_empty() {
            crate::info!(
                "Other {} nodes resolve to 0 definitions: ({})",
                unresolved.len(),
                unresolved
                    .into_iter()
                    .map(|(i, node_idx)| format!("[{i}] {node_idx}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        if self.engine.kotgll_enabled {
            self.engine.kotgll_query(symbol)?;
        }
        if self.engine.ucfs_enabled {
            let cfl_indices = self.engine.map_reference_nodes_to_cfl(indices)?;
            let (dot_path, grammar_path) = self.engine.generate_ucfs_query(symbol, &cfl_indices)?;
            crate::info!("UCFS query DOT generated at {}", dot_path.display());
            crate::info!("UCFS query grammar generated at {}", grammar_path.display());
        }

        Ok(())
    }

    fn cmd_query_node(&mut self, node: SGNodeIndex) -> Result<()> {
        let defs = self.engine.resolve_reference(node)?.defs;
        crate::info!("Node {} resolves to {} definitions:", node, defs.len());
        for def in defs {
            println!(
                "  - {}:{}:{} local_id {}",
                def.file, def.line, def.col, def.local_id,
            );
        }

        if self.engine.kotgll_enabled {
            crate::warn!("KotGLL query not supported for node-based queries (symbol needed)");
        }
        if self.engine.ucfs_enabled {
            let indices_u32 = vec![node as u32];
            let (dot_path, grammar_path) = self
                .engine
                .generate_ucfs_query("node_query", &indices_u32)?;
            crate::info!("UCFS query DOT generated at {}", dot_path.display());
            crate::info!("UCFS query grammar generated at {}", grammar_path.display());
        }
        Ok(())
    }

    fn pick_symbols(&mut self, count: u32) -> Result<Vec<ResolutionResult>> {
        let needed_at_most = count * 128; // Heuristic to get a sufficient number of queries to choose from yet not to take too much time
        let resolved_symbols = self.engine.query_all_symbols(needed_at_most)?;
        let total_symbol = resolved_symbols.len();
        Ok(if (count as usize) < total_symbol {
            let mut rng = StdRng::seed_from_u64(count as u64);
            resolved_symbols
                .choose_multiple_weighted(&mut rng, count as usize, |item| {
                    let resolved_in = item.resolved_in.as_millis() as f64;
                    resolved_in * resolved_in
                })
                .map_err(|e| Error::Internal(format!("Weighted sampling failed: {e}")))?
                .into_iter()
                .cloned()
                .collect::<Vec<_>>()
        } else {
            resolved_symbols
        })
    }

    fn with_file_name_appended(path: &PathBuf, suffix: &str) -> PathBuf {
        let stem = path.file_stem().unwrap();
        let ext = path.extension();

        let mut new_file_name = OsString::from(stem);
        new_file_name.push(suffix);
        if let Some(ext) = ext {
            new_file_name.push(".");
            new_file_name.push(ext);
        }

        path.with_file_name(new_file_name)
    }

    fn cmd_pick_queries(&mut self, count: u32) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        self.engine.generate_artifact(ArtifactType::Kt, true)?;
        self.engine.simplify_cfl = false;
        self.engine.generate_artifact(ArtifactType::DotUcfs, true)?;
        self.engine.simplify_cfl = true;
        let nonsimplified_cfl_path = self.engine.output_path(ArtifactType::DotUcfs);
        self.engine.output_overrides.insert(
            ArtifactType::DotUcfs,
            Self::with_file_name_appended(&nonsimplified_cfl_path, "_simplified"),
        );
        self.engine.generate_artifact(ArtifactType::DotUcfs, true)?;

        let resolved_symbols = self.pick_symbols(count)?;
        self.engine.stats.queries = Vec::with_capacity(resolved_symbols.len());

        let mut renderer = ProgressRenderer::new();
        let start = Instant::now();
        let total_symbols = resolved_symbols.len();
        for (i, rs) in resolved_symbols.into_iter().enumerate() {
            let mut durations = [Duration::ZERO; 7];
            self.engine.retry_query_for_durations(&rs, &mut durations)?;
            let cfl_index = self.engine.map_reference_nodes_to_cfl(&[rs.node_index])?;
            self.engine.stats.queries.push(QueryStats {
                symbol: SymbolStats {
                    name: rs.name,
                    own_index: self.engine.rule_index_of_symbol(rs.symbol_index),
                    cfl_index: rs.node_index as CFLNodeIndex, // For non-simplified it is the same
                    cfl_index_simplified: cfl_index[0],
                    file: rs.file,
                    line: rs.line,
                    column: rs.column,
                },
                resolved_to: rs
                    .defs
                    .into_iter()
                    .map(|d| DefinitionStats {
                        file: d.file,
                        line: d.line,
                        column: d.col,
                    })
                    .collect(),
                resolution_time: durations.map(|d| d.as_millis() as u64),
            });
            renderer.render(&ProgressEvent::RetryingQueries(ElapsedAndCount {
                current: i,
                total: total_symbols,
                elapsed: start.elapsed(),
            }))?;
        }

        let mut sgeq = File::create(self.engine.output_dir.join("queries.sgeq"))?;
        writeln!(
            sgeq,
            "{}",
            serde_json::to_string(&self.engine.stats).unwrap()
        )?;
        renderer.render(&ProgressEvent::PickedQueries(ElapsedAndCount {
            current: total_symbols,
            total: total_symbols,
            elapsed: start.elapsed(),
        }))?;
        Ok(())
    }

    fn cmd_state(&self) -> Result<()> {
        crate::info!("Current configuration:");
        crate::info!("  KotGLL enabled: {}", self.engine.kotgll_enabled);
        crate::info!("  UCFS enabled: {}", self.engine.ucfs_enabled);
        crate::info!("  Verify: {}", self.engine.verify);
        crate::info!("  All symbols: {}", self.engine.all_symbols);
        crate::info!(
            "  Simplify CFL: {} (already simplified? {})",
            self.engine.simplify_cfl,
            self.engine.cfl_graph_simplified,
        );
        crate::info!("  Output directory: {}", self.engine.output_dir.display());
        crate::info!("  Artifact overrides: {:?}", self.engine.output_overrides);
        Ok(())
    }

    fn cmd_help(&self) -> Result<()> {
        crate::info!("Available commands:");
        crate::info!("  open <path>");
        crate::info!("  enable <feature> (alternative: e)");
        crate::info!("  disable <feature> (alternative: d)");
        crate::info!("  output [artifact] <path> (alternative: o)");
        crate::info!("  create <artifact> (alternative: c)");
        crate::info!("  clean <artifact>");
        crate::info!("  query <symbol> (alternative: q, r, run)");
        crate::info!("  state (alternative: s)");
        crate::info!("  help (alternative: h)");
        crate::info!("  exit (alternative: quit, halt)");
        Ok(())
    }

    fn cmd_exit(&self) -> Result<()> {
        std::process::exit(0);
    }
}
