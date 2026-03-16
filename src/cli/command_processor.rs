use std::path::PathBuf;

use crate::{
    cli::engine::{ArtifactType, Engine},
    core::SGNodeIndex,
    error::{Error, Result},
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
    QuerySymbol {
        symbol: String,
    },
    QueryNode {
        node: SGNodeIndex,
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
            Command::QuerySymbol { symbol } => self.cmd_query_symbol(&symbol),
            Command::QueryNode { node } => self.cmd_query_node(node),
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
            "verify" => self.engine.verify = true,
            "all_symbols" => self.engine.all_symbols = true,
            "simplify" => self.engine.simplify_cfl = true,
            _ => {
                crate::error!("Unknown feature '{}'", feature);
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
            "verify" => self.engine.verify = false,
            "all_symbols" => self.engine.all_symbols = false,
            "simplify" => self.engine.simplify_cfl = false,
            _ => {
                crate::error!("Unknown feature '{}'", feature);
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
            self.engine.generate_artifact(art)?;
            crate::success!("Generated {:?}", art);
        } else {
            self.engine.generate_artifacts()?;
            crate::success!("Generated all enabled artifacts");
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
        for &node_idx in indices {
            let defs = self.engine.resolve_reference(node_idx)?;
            crate::info!("Node {} resolves to {} definitions:", node_idx, defs.len());
            for def in defs {
                println!("  - {}:{} local_id {}", def.file, def.line, def.local_id);
            }
        }

        if self.engine.kotgll_enabled {
            self.engine.kotgll_query(symbol)?;
        }
        if self.engine.ucfs_enabled {
            let indices_u32: Vec<u32> = indices.into_iter().map(|i| *i as u32).collect();
            let dot_path = self.engine.generate_ucfs_query(symbol, &indices_u32)?;
            crate::info!("UCFS query DOT generated at {}", dot_path.display());
        }

        Ok(())
    }

    fn cmd_query_node(&mut self, node: SGNodeIndex) -> Result<()> {
        let defs = self.engine.resolve_reference(node)?;
        crate::info!("Node {} resolves to {} definitions:", node, defs.len());
        for def in defs {
            println!("  - {}:{} local_id {}", def.file, def.line, def.local_id);
        }

        if self.engine.kotgll_enabled {
            crate::warn!("KotGLL query not supported for node-based queries (symbol needed)");
        }
        if self.engine.ucfs_enabled {
            let indices_u32 = vec![node as u32];
            let dot_path = self
                .engine
                .generate_ucfs_query("node_query", &indices_u32)?;
            crate::info!("UCFS query DOT generated at {}", dot_path.display());
        }
        Ok(())
    }

    fn cmd_state(&self) -> Result<()> {
        crate::info!("Current configuration:");
        crate::info!("  Kotlin GLL enabled: {}", self.engine.kotgll_enabled);
        crate::info!("  UCFS enabled: {}", self.engine.ucfs_enabled);
        crate::info!("  Verify: {}", self.engine.verify);
        crate::info!("  All symbols: {}", self.engine.all_symbols);
        crate::info!("  Simplify CFL: {}", self.engine.simplify_cfl);
        crate::info!("  Output directory: {}", self.engine.output_dir.display());
        crate::info!("  Artifact overrides: {:?}", self.engine.output_overrides);
        Ok(())
    }

    fn cmd_help(&self) -> Result<()> {
        crate::info!("Available commands:");
        crate::info!("  open <path>");
        crate::info!("  enable <feature>");
        crate::info!("  disable <feature>");
        crate::info!("  output [artifact] <path>");
        crate::info!("  create <artifact>");
        crate::info!("  query <symbol>");
        crate::info!("  state");
        crate::info!("  help");
        crate::info!("  exit");
        Ok(())
    }

    fn cmd_exit(&self) -> Result<()> {
        std::process::exit(0);
    }
}
