use crate::cfl_query::progress_event::ProgressEvent;
use crate::error::{Error, Result};
use regex::Regex;
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};
use std::time::Instant;

fn prepare_symbol_query_grammar(grammar_path: &Path, symbol: &str) -> Result<()> {
    let content = read_to_string(grammar_path)?;

    // Find the start nonterminal (the one marked with .asStart())
    let re_start_decl =
        Regex::new(r"(val\s+([A-Za-z_][A-Za-z0-9_]*)\s+by\s+Nt\([^)]*\))\.asStart\(\)").unwrap();

    let (content_without_asstart, start_name) = if let Some(cap) = re_start_decl.captures(&content)
    {
        let var_name = cap.get(2).unwrap().as_str();
        let content_replaced = re_start_decl.replace(&content, "$1").to_string();
        (content_replaced, var_name.to_string())
    } else {
        crate::warn!("Fallback: pick the first nonterminal declaration");
        let re_first_nt = Regex::new(r"val\s+([A-Za-z_][A-Za-z0-9_]*)\s+by\s+Nt\(\)").unwrap();
        if let Some(cap) = re_first_nt.captures(&content) {
            let var_name = cap.get(1).unwrap().as_str().to_string();
            (content.clone(), var_name)
        } else {
            return Err(Error::Query(
                "Could not find any nonterminal declaration `val NAME by Nt()` in grammar kotlin file.".into()
            ));
        }
    };

    // Insert the new query start rule before the `init {` block
    let insert_before = "init {";
    if let Some(pos) = content_without_asstart.find(insert_before) {
        let (head, tail) = content_without_asstart.split_at(pos);
        let q_decl = format!(
            "    val Q by Nt(Term(\"push_{q}\") * {start} * Term(\"pop_{q}\")).asStart()\n\n",
            q = symbol.replace('"', "\\\""),
            start = start_name
        );
        let new_content = format!("{}{}{}", head, q_decl, tail);
        let query_grammar_path = grammar_path.with_file_name(".cfl_query.kt");
        write(&query_grammar_path, new_content)?;
        Ok(())
    } else {
        Err(Error::Query(
            "Could not find `init {{` block in the grammar kotlin file (needed to insert query start).".into()
        ))
    }
}

fn modify_dot_file(dot_path: &Path, indices: &[u32], output_path: &Path) -> Result<()> {
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

    write(output_path, new_dot)?;
    Ok(())
}

pub fn ucfs_query<F>(
    grammar_path: &Path,
    dot_path: &Path,
    output_dir: &Path,
    symbol: &str,
    indices: &[u32],
    mut progress: F,
) -> Result<PathBuf>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let start = Instant::now();

    progress(ProgressEvent::PreparingQueryGrammar {
        elapsed: start.elapsed(),
    })?;
    prepare_symbol_query_grammar(grammar_path, symbol)?;

    progress(ProgressEvent::ModifyingDot {
        elapsed: start.elapsed(),
    })?;
    let ucfs_dot_path = output_dir.join("cfl_ucfs.dot");
    modify_dot_file(dot_path, indices, &ucfs_dot_path)?;

    progress(ProgressEvent::UcfsDone {
        elapsed: start.elapsed(),
    })?;

    Ok(ucfs_dot_path)
}
