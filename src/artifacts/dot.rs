use std::path::PathBuf;

use crate::core::{CFLGraph, SGGraph, SGNode, SGNodeId, SGSymbol};
use crate::error::Result;

pub trait ToDOT {
    fn to_dot_lines(self: &Self, clean_dot: bool) -> Vec<String>;

    fn write_to_dot_file(self: &Self, out_path: &PathBuf, clean_dot: bool) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut out_file = File::create(&out_path)?;

        for line in self.to_dot_lines(clean_dot).into_iter() {
            writeln!(out_file, "{}", line)?;
        }
        Ok(())
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
    fn to_dot_lines(self: &Self, clean_dot: bool) -> Vec<String> {
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
        }

        for edge in &self.edges {
            dot_lines.push(format!("  {} -> {};", edge.from, edge.to));
        }

        dot_lines.push("}".to_string());
        dot_lines
    }
}

impl ToDOT for CFLGraph {
    fn to_dot_lines(self: &Self, clean_dot: bool) -> Vec<String> {
        let mut dot_lines: Vec<String> = Vec::new();
        dot_lines.push("digraph stackgraph {".to_string());
        if !clean_dot {
            dot_lines.push("  rankdir=LR;".to_string());
            dot_lines.push("  node [shape=box, fontsize=10];".to_string());
        }

        for edge in &self.edges {
            let label = edge
                .symbol
                .and_then(|s| {
                    Some(format!(
                        " [label = \"{}\"]",
                        esc_dot_label(&self.symbols[s])
                    ))
                })
                .unwrap_or(" [label = \"\"]".to_string());
            dot_lines.push(format!("  {} -> {}{};", edge.from, edge.to, label,));
        }

        dot_lines.push("}".to_string());
        /*for rule in &self.rules {
            dot_lines.push(format!(
                "// {} := {}",
                &self.symbols[rule.from_non_terminal],
                rule.to
                    .iter()
                    .map(|s| self.symbols[match s {
                        crate::types::CFLSymbol::Terminal(i) => *i,
                        crate::types::CFLSymbol::NonTerminal(i) => *i,
                    }]
                    .clone())
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }*/
        dot_lines
    }
}
