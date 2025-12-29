mod cfl_simplifier;
mod converter;
mod csv;
mod dot;
mod from_serde;
mod grammar_cfg;
mod loader;
mod types;

use converter::convert_to_cfl;
use dot::ToDOT;
use from_serde::FromSerde;
use loader::load_graph;
use types::SGGraph;

use anyhow::{Context, Error, Result};

use crate::{csv::ToCSV, grammar_cfg::ToCFGGrammar};

fn sgexport(project_dir: String, language: String) -> Result<()> {
    let (output_path, sg_output_dot, cfl_output_dot, cfl_output_csv, cfl_output_grammar_cfg) = (
        format!("{}.stackgraph.json", &project_dir),
        format!("{}.stackgraph.dot", &project_dir),
        format!("{}.cfl.dot", &project_dir),
        format!("{}.cfl.csv", &project_dir),
        format!("{}.cfl_grammar.cfg", &project_dir),
    );

    let stack_graph = load_graph(&project_dir, &language)?.to_serializable();
    let out_file = std::fs::File::create(&output_path)
        .with_context(|| format!("cannot create output file {}", output_path))?;
    serde_json::to_writer_pretty(out_file, &stack_graph)
        .with_context(|| format!("failed to write JSON to {}", output_path))?;

    let sggraph = SGGraph::from_serde(serde_json::to_value(stack_graph)?)?;
    sggraph.write_to_dot_file(&sg_output_dot)?;

    let cfl = convert_to_cfl(&sggraph, true)?;
    cfl.write_to_dot_file(&cfl_output_dot)?;
    cfl.write_to_csv_file(&cfl_output_csv, false)?;
    cfl.write_to_grammar_file(&cfl_output_grammar_cfg)?;

    println!(
      "Wrote stack graph JSON to {} and DOT to {}; CFL DOT to {} and CSV to {}, it's grammar CFG to {}",
      output_path, sg_output_dot, cfl_output_dot, cfl_output_csv, cfl_output_grammar_cfg
    );

    Ok(())
}

fn cflquery(artifacts_dir: String, query: String) -> Result<()> {
    use std::fs;
    use std::process::Command;

    let graph_path = format!("{}.cfl.csv", &artifacts_dir);
    let grammar_path = format!("{}.cfl_grammar.cfg", &artifacts_dir);
    let query_grammar_path = format!("{}.cfl_query.cfg", &artifacts_dir);
    let output_path = format!("{}.output.txt", &artifacts_dir);
    let grammar = match fs::read_to_string(grammar_path) {
        Ok(content) => content,
        Err(e) => return Err(Error::msg(e.to_string())),
    };
    fs::write(
        &query_grammar_path,
        format!("StartNonterminal(\"{}\")\n{}", &query, &grammar),
    )
    .map_err(|e| Error::msg(e.to_string()))?;
    let output = Command::new("java")
        .arg("-jar")
        .arg("C:/kotgll-1.0.8.jar")
        .arg("--input")
        .arg("graph")
        .arg("--grammar")
        .arg("cfg")
        .arg("--inputPath")
        .arg(graph_path)
        .arg("--grammarPath")
        .arg(query_grammar_path)
        .arg("--outputPath")
        .arg(output_path)
        .output()
        .expect("Failed to run kotgll");
    println!("Status: {}", output.status);
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    Ok(())
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let mode = args
        .next()
        .expect("Usage: stackgraph_exporter q/query <path-to-generated-artifacts> <query> | <path-to-project-dir> [language: \"py\"|\"java\"]");
    if mode == "q" || mode == "query" {
        let project_dir = args
            .next()
            .expect("Usage: stackgraph_exporter q/query <path-to-generated-artifacts> <query>");
        let query = args
            .next()
            .expect("Usage: stackgraph_exporter q/query <path-to-generated-artifacts> <query>");
        cflquery(project_dir, query)?;
    } else {
        let project_dir = mode;
        let language = args.next().unwrap_or_else(|| String::from("py"));
        sgexport(project_dir, language)?;
    }

    Ok(())
}
