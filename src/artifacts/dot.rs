use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

use crate::artifacts::progress_event::ProgressEvent;
use crate::core::{CFLGraph, SGGraph, SGNode, SGNodeId, SGSymbol};
use crate::error::Result;
use crate::io::ElapsedAndCount;

const WRITE_ONCE_IN_N: usize = 64;

pub trait ToDOT {
    const ARTIFACT_NAME: &'static str;

    fn to_dot_lines<F>(self: &Self, clean_dot: bool, progress: &mut F) -> Result<Vec<String>>
    where
        F: FnMut(ProgressEvent) -> Result<()>;

    fn write_to_dot_file<F>(
        self: &Self,
        out_path: &PathBuf,
        clean_dot: bool,
        mut progress: F,
    ) -> Result<()>
    where
        F: FnMut(ProgressEvent) -> Result<()>,
    {
        let start = Instant::now();
        let mut out_file = File::create(&out_path)?;
        let dot = self.to_dot_lines(clean_dot, &mut progress)?;
        let total_lines = dot.len();

        for (i, line) in dot.into_iter().enumerate() {
            writeln!(out_file, "{line}")?;
            if i % WRITE_ONCE_IN_N == 0 {
                progress(ProgressEvent::WritingLines {
                    elapsed_and_count: ElapsedAndCount {
                        current: i,
                        total: total_lines,
                        elapsed: start.elapsed(),
                    },
                    artifact_name: Self::ARTIFACT_NAME,
                })?;
            }
        }
        progress(ProgressEvent::ArtifactStored {
            elapsed_and_count: ElapsedAndCount {
                current: total_lines,
                total: total_lines,
                elapsed: start.elapsed(),
            },
            artifact_name: Self::ARTIFACT_NAME,
        })
    }
}

fn esc_dot_label(s: &String) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => {} // ignore
            _ => out.push(ch),
        }
    }
    out
}

fn id_to_str(id: &SGNodeId, files: &Vec<String>) -> String {
    format!(
        "{}:{}",
        id.file
            .as_ref()
            .and_then(|x| Some(files[*x].as_str()))
            .unwrap_or_else(|| "<global>"),
        id.local_id
    )
}

fn symbol_to_str(symbol: &SGSymbol) -> String {
    if !symbol.real {
        format!("virtual {}", symbol.name)
    } else {
        symbol.name.clone()
    }
}

fn make_node_name(
    ids: &Vec<SGNodeId>,
    id: &SGNodeId,
    symbols: &Vec<SGSymbol>,
    files: &Vec<String>,
    node: &SGNode,
) -> String {
    match node {
        SGNode::Scope(is_exported) => {
            if *is_exported {
                format!("scope {} exported", id_to_str(id, files))
            } else {
                format!("scope {}", id_to_str(id, files))
            }
        }
        SGNode::Root => "root".to_string(),
        SGNode::Push(symbol) => format!("push {}", symbol_to_str(&symbols[*symbol])),
        SGNode::Pop(symbol) => format!("pop {}", symbol_to_str(&symbols[*symbol])),
        SGNode::JumpTo => "jump_to".to_string(),
        SGNode::PushScoped(symbol, scope) => format!(
            "push_scoped {} at {}",
            symbol_to_str(&symbols[*symbol]),
            id_to_str(&ids[*scope as usize], files)
        ),
        SGNode::PushScopedUnresolved(symbol, scope_raw) => format!(
            "push_scoped {} at {}",
            symbol_to_str(&symbols[*symbol]),
            id_to_str(&scope_raw, files)
        ),
        SGNode::PopScoped(symbol) => format!("pop_scoped {}", symbol_to_str(&symbols[*symbol])),
        SGNode::DropScopes => "drop_scopes".to_string(),
    }
}

impl ToDOT for SGGraph {
    const ARTIFACT_NAME: &'static str = "Stack Graph DOT";

    fn to_dot_lines<F>(self: &Self, clean_dot: bool, progress: &mut F) -> Result<Vec<String>>
    where
        F: FnMut(ProgressEvent) -> Result<()>,
    {
        let start = Instant::now();
        let mut dot_lines: Vec<String> = Vec::new();
        dot_lines.push("digraph stackgraph {".to_string());
        if !clean_dot {
            dot_lines.push("  rankdir=LR;".to_string());
            dot_lines.push("  node [shape=box, fontsize=10];".to_string());
        }
        for (i, node) in self.nodes.iter().enumerate() {
            let id = &self.ids[i];
            let node_name = make_node_name(&self.ids, &id, &self.symbols, &self.files, &node);
            let node_name = esc_dot_label(&node_name);
            dot_lines.push(format!("  {} [label=\"{}\"];", i, node_name));
            if i % WRITE_ONCE_IN_N == 0 {
                progress(ProgressEvent::GeneratingArtifact {
                    elapsed: start.elapsed(),
                    progress: Some((i, self.nodes.len())),
                    message: "Generating Stack Graph DOT nodes".into(),
                })?;
            }
        }

        for (i, edge) in self.edges.iter().enumerate() {
            dot_lines.push(format!("  {} -> {};", edge.from, edge.to));
            if i % WRITE_ONCE_IN_N == 0 {
                progress(ProgressEvent::GeneratingArtifact {
                    elapsed: start.elapsed(),
                    progress: Some((i, self.edges.len())),
                    message: "Generating Stack Graph DOT edges".into(),
                })?;
            }
        }

        dot_lines.push("}".to_string());
        Ok(dot_lines)
    }
}

impl ToDOT for CFLGraph {
    const ARTIFACT_NAME: &'static str = "CFL Graph DOT";

    fn to_dot_lines<F>(self: &Self, clean_dot: bool, progress: &mut F) -> Result<Vec<String>>
    where
        F: FnMut(ProgressEvent) -> Result<()>,
    {
        let start = Instant::now();
        let mut dot_lines: Vec<String> = Vec::new();
        dot_lines.push("digraph stackgraph {".to_string());
        if !clean_dot {
            dot_lines.push("  rankdir=LR;".to_string());
            dot_lines.push("  node [shape=box, fontsize=10];".to_string());
        }

        for (i, edge) in self.edges.iter().enumerate() {
            let label = edge
                .symbol
                .and_then(|s| Some(format!("[label = \"{}\"]", Self::get_symbol_name(s))))
                .unwrap_or("[label = \"\"]".to_string());
            dot_lines.push(format!("  {} -> {} {};", edge.from, edge.to, label));
            if i % WRITE_ONCE_IN_N == 0 {
                progress(ProgressEvent::GeneratingArtifact {
                    elapsed: start.elapsed(),
                    progress: Some((i, self.edges.len())),
                    message: "Generating CFL DOT edges".into(),
                })?;
            }
        }

        dot_lines.push("}".to_string());
        Ok(dot_lines)
    }
}
