use crate::cfl_query::progress_event::ProgressEvent;
use crate::core::CFLRuleIndex;
use crate::error::{Error, Result};
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};
use std::time::Instant;

const PLACEHOLDER: &'static str = "<placeholder nt=\"";

fn parse_start_symbol_from_placeholder(line: &String) -> Option<&str> {
    if let Some(idx) = line.find(PLACEHOLDER) {
        let nt_start_idx = idx + PLACEHOLDER.len();
        let after_placeholder = &line[nt_start_idx..];
        let nt_end_idx = after_placeholder
            .find("\"")
            .unwrap_or(after_placeholder.len());
        Some(&after_placeholder[..nt_end_idx])
    } else {
        None
    }
}

fn prepare_symbol_query_grammar(
    grammar_path: &Path,
    rule: CFLRuleIndex,
    query_grammar_path: &PathBuf,
) -> Result<()> {
    let content = read_to_string(grammar_path)?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    let mut start_name = None;
    let mut start_line_index = None;
    for (i, line) in lines.iter().enumerate() {
        if let Some(start_symbol) = parse_start_symbol_from_placeholder(line) {
            start_name = Some(start_symbol.to_string());
            start_line_index = Some(i);
            break;
        }
    }

    let start_name = start_name.ok_or_else(|| {
        Error::Query("Could not find any non‑terminal declaration in the grammar file.".into())
    })?;
    let start_line_index = start_line_index.unwrap();

    // Remove .asStart() from its line
    let line = &lines[start_line_index];
    let without_suffix = line.replace(".asStart()", "");
    lines[start_line_index] = without_suffix;

    let q_line = format!(
        "    val Q by Nt(Term(\"psh{rule}\") * {start_name} * Term(\"pp{rule}\")).asStart()",
    );

    // Insert the Q declaration
    lines.insert(start_line_index + 1, q_line);

    let new_content = lines.join("\n");
    write(query_grammar_path, new_content)?;

    Ok(())
}

fn modify_dot_file(dot_path: &Path, indices: &[u32], query_dot_path: &Path) -> Result<()> {
    let orig_dot = read_to_string(dot_path)?;

    let mut start_fragment = String::new();
    for idx in indices {
        start_fragment.push_str(&format!("    start -> {};\n", idx));
    }

    let new_dot = {
        let pos = orig_dot
            .find("{\n")
            .expect("Invalid DOT: there is no '{\n'");
        let insert_at = pos + 2;
        let mut new_dot = String::with_capacity(orig_dot.len() + start_fragment.len());
        new_dot.push_str(&orig_dot[..insert_at]);
        new_dot.push_str(&start_fragment);
        new_dot.push_str(&orig_dot[insert_at..]);
        new_dot
    };

    write(query_dot_path, new_dot)?;
    Ok(())
}

pub fn ucfs_query<F>(
    grammar_path: &Path,
    dot_path: &Path,
    output_dir: &Path,
    rule: CFLRuleIndex,
    indices: &[u32],
    mut progress: F,
) -> Result<(PathBuf, PathBuf)>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let start = Instant::now();

    progress(ProgressEvent::PreparingQueryGrammar {
        elapsed: start.elapsed(),
    })?;
    let query_grammar_path = grammar_path
        .with_file_name("UCFSGrammar")
        .with_extension("kt");
    prepare_symbol_query_grammar(grammar_path, rule, &query_grammar_path)?;

    progress(ProgressEvent::ModifyingDot {
        elapsed: start.elapsed(),
    })?;
    let query_dot_path = output_dir
        .with_file_name("query")
        .with_extension("cfl_ucfs.dot");
    modify_dot_file(dot_path, indices, &query_dot_path)?;

    progress(ProgressEvent::UcfsDone {
        elapsed: start.elapsed(),
    })?;

    Ok((query_dot_path, query_grammar_path))
}
