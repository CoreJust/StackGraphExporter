use crate::cfl_query::progress_event::ProgressEvent;
use crate::core::{CFLNodeIndex, CFLPath};
use crate::error::{Error, Result};
use std::fs::{read_to_string, write};
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

fn prepare_query_grammar(grammar_path: &PathBuf, symbol: &str) -> Result<PathBuf> {
    let query_grammar_path = grammar_path.with_file_name(".cfl_query.cfg");
    let grammar = read_to_string(grammar_path)?;
    let content = format!(
        "StartNonterminal(\"Q\")\nNonterminal(\"Q\") -> Terminal(\"push_{0}\") Nonterminal(\"S\") Terminal(\"pop_{0}\")\n{1}",
        symbol, grammar,
    );
    write(&query_grammar_path, content)?;
    Ok(query_grammar_path)
}

fn run_kotgll(
    kotgll_path: &PathBuf,
    graph_path: &PathBuf,
    grammar_path: &PathBuf,
    output_path: &PathBuf,
    sppf: bool,
) -> Result<()> {
    let sppf_arg = if sppf { "on" } else { "off" };
    let status = Command::new("java")
        .arg("-jar")
        .arg(kotgll_path)
        .arg("--input")
        .arg("graph")
        .arg("--grammar")
        .arg("cfg")
        .arg("--sppf")
        .arg(sppf_arg)
        .arg("--inputPath")
        .arg(graph_path)
        .arg("--grammarPath")
        .arg(grammar_path)
        .arg("--outputPath")
        .arg(output_path)
        .status()
        .map_err(|e| Error::Query(format!("Failed to execute kotgll: {}", e)))?;

    if !status.success() {
        return Err(Error::Query(format!(
            "kotgll exited with status: {}",
            status
        )));
    }
    Ok(())
}

fn parse_output(output_path: &PathBuf) -> Result<Vec<CFLPath>> {
    let content = read_to_string(output_path)?;
    let mut paths = Vec::new();
    for line in content.lines() {
        if line.contains(' ') {
            let mut parts = line.split_whitespace();
            if let (Some(from_str), Some(to_str)) = (parts.next(), parts.next()) {
                let from = from_str
                    .parse::<CFLNodeIndex>()
                    .map_err(|_| Error::Query(format!("Invalid from node: {}", from_str)))?;
                let to = to_str
                    .parse::<CFLNodeIndex>()
                    .map_err(|_| Error::Query(format!("Invalid to node: {}", to_str)))?;
                paths.push(CFLPath { from, to });
            }
        }
    }
    Ok(paths)
}

pub fn kotgll_query<F>(
    kotgll_path: &PathBuf,
    grammar_path: &PathBuf,
    graph_path: &PathBuf,
    output_dir: &PathBuf,
    symbol: &str,
    sppf: bool,
    mut progress: F,
) -> Result<Vec<CFLPath>>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let start = Instant::now();
    progress(ProgressEvent::PreparingQueryGrammar {
        elapsed: start.elapsed(),
    })?;
    let query_grammar_path = prepare_query_grammar(grammar_path, symbol)?;
    let output_path = output_dir.join(".kotgll_result.txt");

    progress(ProgressEvent::RunningKotgll {
        elapsed: start.elapsed(),
    })?;
    run_kotgll(
        kotgll_path,
        graph_path,
        &query_grammar_path,
        &output_path,
        sppf,
    )?;

    progress(ProgressEvent::ParsingOutput {
        elapsed: start.elapsed(),
    })?;
    let paths = parse_output(&output_path)?;

    progress(ProgressEvent::KotGllDone {
        elapsed: start.elapsed(),
    })?;
    Ok(paths)
}
