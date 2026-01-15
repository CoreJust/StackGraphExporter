mod cfl_querier;
mod cfl_simplifier;
mod converter;
mod csv;
mod dot;
mod from_serde;
mod grammar_cfg;
mod loader;
mod sg_paths_extractor;
mod types;

use std::{collections::HashSet, io::Write};

use converter::convert_to_cfl;
use dot::ToDOT;
use loader::load_graph;
use types::SGGraph;

use anyhow::Result;

use crate::{
    cfl_querier::cflquery,
    csv::ToCSV,
    grammar_cfg::ToCFGGrammar,
    types::{CFLGraph, CFLPath},
};

fn print_query_results_with_metadata(results: Vec<CFLPath>, cflgraph: &CFLGraph) {
    let metadata = &cflgraph.metadata;
    results.iter().for_each(|r| {
        let mut indices = [r.from, r.to].into_iter();
        let mut next_symbol_str = || {
            indices
                .next()
                .and_then(|i| {
                    Some(
                        metadata
                            .get(&i)
                            .and_then(|m| {
                                Some(format!(
                                    "{}{} (node id {}) at {}:{}",
                                    if m.is_real { "" } else { "virtual " },
                                    &m.name,
                                    i,
                                    m.file
                                        .and_then(|f| Some(cflgraph.files[f].as_str()))
                                        .unwrap_or(""),
                                    m.line.and_then(|l| Some(l + 1)).unwrap_or(0),
                                ))
                            })
                            .unwrap_or("<No metadata>".to_owned()),
                    )
                })
                .unwrap_or("".to_owned())
        };
        println!(
            "Found ref {} -> def {}",
            next_symbol_str(),
            next_symbol_str()
        );
    });
}

fn sgexport(project_dir: &String, language: String) -> Result<CFLGraph> {
    let (output_path, sg_output_dot, cfl_output_dot, cfl_output_csv, cfl_output_grammar_cfg) = (
        format!("{}.stackgraph.json", &project_dir),
        format!("{}.stackgraph.dot", &project_dir),
        format!("{}.cfl.dot", &project_dir),
        format!("{}.cfl.csv", &project_dir),
        format!("{}.cfl_grammar.cfg", &project_dir),
    );

    let stack_graph = load_graph(&project_dir, &language)?;
    //let out_file = std::fs::File::create(&output_path)
    //    .with_context(|| format!("cannot create output file {}", output_path))?;
    //serde_json::to_writer_pretty(out_file, &stack_graph)
    //    .with_context(|| format!("failed to write JSON to {}", output_path))?;

    let sggraph = SGGraph::from_serde(&stack_graph)?;
    sggraph.write_to_dot_file(&sg_output_dot)?;

    let cfl = convert_to_cfl(sggraph, true)?;
    cfl.write_to_dot_file(&cfl_output_dot)?;
    cfl.write_to_csv_file(&cfl_output_csv, false)?;
    cfl.write_to_grammar_file(&cfl_output_grammar_cfg)?;

    println!(
      "Wrote stack graph /*JSON to {}*/ and DOT to {}; CFL DOT to {} and CSV to {}, it's grammar CFG to {}",
      output_path, sg_output_dot, cfl_output_dot, cfl_output_csv, cfl_output_grammar_cfg
    );

    Ok(cfl)
}

fn run_cflquery(
    artifacts_dir: &String,
    query: &str,
    sppf_on: bool,
    cflgraph: Option<&CFLGraph>,
) -> Result<()> {
    let results = cflquery(artifacts_dir, query, sppf_on)?;
    if let Some(cflgraph) = cflgraph {
        let found_correct = results.iter().all(|x| cflgraph.paths.contains(x));
        let as_hash_set = results.iter().cloned().collect::<HashSet<_>>();
        let found_all = cflgraph
            .paths
            .iter()
            .filter(|x| cflgraph.metadata[&x.from].name == query)
            .all(|x| as_hash_set.contains(x));
        print_query_results_with_metadata(results, cflgraph);
        if found_all && found_correct {
            println!("Query results match the ones precalculated using stack graphs");
        } else {
            println!(
                "Incorrect query results: found all? {}, found correct ones? {}\nCorrect results are:\n{}",
                found_all,
                found_correct,
                cflgraph
                  .paths
                  .iter()
                  .filter(|x| cflgraph.metadata[&x.from].name == query)
                  .map(|x| format!("(ref {} -> def {})", x.from, x.to))
                  .collect::<Vec<_>>()
                  .join("\n")
            );
        }
    } else {
        println!(
            "Query results:\n{}",
            results
                .iter()
                .map(|x| format!("{} {}", x.from, x.to))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let mode = args
        .next()
        .expect("Usage: stackgraph_exporter q/query <path-to-generated-artifacts> <query> | [i/interactive] <path-to-project-dir> [language: \"py\"|\"java\"]");
    if mode == "q" || mode == "query" {
        let project_dir = args
            .next()
            .expect("Usage: stackgraph_exporter q/query <path-to-generated-artifacts> <query>");
        let query = args
            .next()
            .expect("Usage: stackgraph_exporter q/query <path-to-generated-artifacts> <query>");
        run_cflquery(&project_dir, query.as_str(), true, None)?;
    } else if mode == "i" || mode == "interactive" {
        let project_dir = args.next().expect(
            "Usage: stackgraph_exporter i/interactive <path-to-project-dir> [language: \"py\"|\"java\"]",
        );
        let language = args.next().unwrap_or_else(|| String::from("py"));
        let cflgraph = sgexport(&project_dir, language)?;
        loop {
            let mut input = String::new();
            print!(">>> ");
            std::io::stdout().flush()?;
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read user input");
            let input = input.trim();
            if input.starts_with("q") {
                run_cflquery(&project_dir, &input[2..], true, Some(&cflgraph))?;
            } else if input.starts_with("end")
                || input.starts_with("exit")
                || input.starts_with("done")
            {
                break;
            }
        }
    } else {
        let project_dir = mode;
        let language = args.next().unwrap_or_else(|| String::from("py"));
        sgexport(&project_dir, language)?;
    }

    Ok(())
}
