use crate::cfl_query::progress_event::ProgressEvent;
use crate::error::{Error, Result};
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};
use std::time::Instant;

fn line_as_start_symbol(line: &String) -> Option<&str> {
    let trimmed = line.trim_start();
    if trimmed.starts_with("val ") && trimmed.contains(" by Nt(") && trimmed.ends_with(".asStart()")
    {
        let after_val = &trimmed[4..]; // skip "val "
        let name_end = after_val
            .find(char::is_whitespace)
            .unwrap_or(after_val.len());
        Some(&after_val[..name_end])
    } else {
        None
    }
}

fn prepare_symbol_query_grammar(
    grammar_path: &Path,
    symbol: &str,
    query_grammar_path: &PathBuf,
) -> Result<()> {
    let content = read_to_string(grammar_path)?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    let mut start_name = None;
    let mut start_line_index = None;
    for (i, line) in lines.iter().enumerate() {
        if let Some(as_start_name) = line_as_start_symbol(line) {
            start_name = Some(as_start_name.to_string());
            start_line_index = Some(i);
            break;
        }
    }

    // If no .asStart() found, fall back to the first non‑terminal declaration
    if start_name.is_none() {
        for line in &lines {
            if let Some(as_start_name) = line_as_start_symbol(line) {
                start_name = Some(as_start_name.to_string());
                start_line_index = None;
                break;
            }
        }
    }

    let start_name = start_name.ok_or_else(|| {
        Error::Query("Could not find any non‑terminal declaration in the grammar file.".into())
    })?;

    // If the original start had .asStart(), remove that suffix from its line
    if let Some(idx) = start_line_index {
        let line = &lines[idx];
        let without_suffix = line.replace(".asStart()", "");
        lines[idx] = without_suffix;
    }

    let init_pos = lines
        .iter()
        .position(|line| line.trim_start().starts_with("init {"))
        .ok_or_else(|| Error::Query("Could not find `init {` block in the grammar file.".into()))?;

    let escaped_symbol = symbol.replace('\\', "\\\\").replace('"', "\\\"");
    let q_line = format!(
        "    val Q by Nt(Term(\"push_{escaped_symbol}\") * {start_name} * Term(\"pop_{escaped_symbol}\")).asStart()",
    );

    // Insert the Q declaration right before `init {`
    lines.insert(init_pos, q_line);
    lines.insert(init_pos + 1, String::new());

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

    let new_dot = if let Some(pos) = orig_dot.find('{') {
        let after_brace = pos + 1;
        if let Some(nl_pos) = orig_dot[after_brace..].find('\n') {
            let insert_at = after_brace + nl_pos + 1;
            let mut new_dot = String::with_capacity(orig_dot.len() + start_fragment.len() + 32);
            new_dot.push_str(&orig_dot[..insert_at]);
            new_dot.push_str(&start_fragment);
            new_dot.push_str(&orig_dot[insert_at..]);
            new_dot
        } else {
            let mut new_dot = String::with_capacity(orig_dot.len() + start_fragment.len() + 32);
            new_dot.push_str(&orig_dot[..after_brace]);
            new_dot.push_str("\n");
            new_dot.push_str(&start_fragment);
            new_dot.push_str(&orig_dot[after_brace..]);
            new_dot
        }
    } else {
        let mut new_dot = orig_dot;
        new_dot.push_str("\n");
        new_dot.push_str(&start_fragment);
        new_dot
    };

    write(query_dot_path, new_dot)?;
    Ok(())
}

pub fn ucfs_query<F>(
    grammar_path: &Path,
    dot_path: &Path,
    output_dir: &Path,
    symbol: &str,
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
    prepare_symbol_query_grammar(grammar_path, symbol, &query_grammar_path)?;

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
