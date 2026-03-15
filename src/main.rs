mod cfl_querier;
mod cfl_simplifier;
mod conversion;
mod converter;
mod core;
mod csv;
mod dot;
mod error;
mod grammar_cfg;
mod grammar_kt;
mod io;
mod loading;

use std::{collections::HashSet, path::Path};

use converter::convert_to_cfl;
use dot::ToDOT;

use anyhow::Result;

use crate::{
    cfl_querier::{cflquery, ucfs_cflquery},
    conversion::{build_sggraph, ProgressEvent as ConversionProgressEvent},
    core::{CFLGraph, CFLPath},
    csv::ToCSV,
    grammar_cfg::ToCFGGrammar,
    grammar_kt::ToKTGrammar,
    loading::{load_stack_graph, Language, ProgressEvent as LoadingProgressEvent},
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
    let (
        output_path,
        sg_output_dot,
        cfl_output_dot,
        cfl_output_csv,
        cfl_output_grammar_cfg,
        cfl_output_grammar_kt,
    ) = (
        format!("{}.stackgraph.json", &project_dir),
        format!("{}.stackgraph.dot", &project_dir),
        format!("{}.cfl.dot", &project_dir),
        format!("{}.cfl.csv", &project_dir),
        format!("{}.cfl_grammar.cfg", &project_dir),
        format!("{}.cfl_grammar.kt", &project_dir),
    );

    let stack_graph = load_stack_graph(
        Path::new(project_dir),
        Language::from_str(language.as_str())?,
        |progress| {
            io::on_same_console_line(|| {
                if matches!(progress, LoadingProgressEvent::Done { .. }) {
                    println!("{}", progress)
                } else {
                    print!("{}", progress)
                }
            })
        },
    )?;
    //let out_file = std::fs::File::create(&output_path)
    //    .with_context(|| format!("cannot create output file {}", output_path))?;
    //serde_json::to_writer_pretty(out_file, &stack_graph)
    //    .with_context(|| format!("failed to write JSON to {}", output_path))?;

    let sggraph = build_sggraph(&stack_graph, |event| {
        io::on_same_console_line(|| {
            if matches!(event, ConversionProgressEvent::Done { .. }) {
                println!("{}", event)
            } else {
                print!("{}", event)
            }
        })
    })?;
    sggraph.write_to_dot_file(&sg_output_dot)?;

    let cfl = convert_to_cfl(sggraph, true)?;
    cfl.write_to_dot_file(&cfl_output_dot)?;
    cfl.write_to_csv_file(&cfl_output_csv, false)?;
    cfl.write_to_grammar_file(&cfl_output_grammar_cfg)?;
    cfl.write_to_kotlin_file(&cfl_output_grammar_kt, "UCFSGrammar")?;

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
    ucfs_cflquery(artifacts_dir, query)?;
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

fn prompt_line(prompt: &str) -> Result<String> {
    use std::io::{self, Write};
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}

fn collect_symbol_matches(cflgraph: &CFLGraph, name: &str) -> Vec<u32> {
    let mut set = std::collections::BTreeSet::new();
    for p in &cflgraph.paths {
        if let Some(meta) = cflgraph.metadata.get(&p.from) {
            if meta.name == name {
                set.insert(p.from);
            }
        }
    }

    let mut vec_idxs: Vec<u32> = set.into_iter().collect();
    vec_idxs.sort_by_key(|&idx| {
        let meta = &cflgraph.metadata[&idx];
        let file_idx = meta.file.unwrap_or(usize::MAX);
        let line = meta.line.unwrap_or(usize::MAX);
        (file_idx, line, idx)
    });

    vec_idxs
}

fn choose_indices_interactive(matches: &[u32], cflgraph: &CFLGraph) -> Result<Vec<u32>> {
    if matches.len() == 1 {
        println!("Found single occurrence at node index {}.", matches[0]);
        return Ok(vec![matches[0]]);
    }
    println!("Found {} occurrences:", matches.len());
    for (i, idx) in matches.iter().enumerate() {
        let meta = &cflgraph.metadata[idx];
        let file_str = meta
            .file
            .and_then(|f| Some(cflgraph.files[f].as_str()))
            .unwrap_or("");
        let line = meta.line.map(|l| l + 1).unwrap_or(0);
        println!(
            "  [{}] node {} — {}{} at {}:{}",
            i,
            idx,
            if meta.is_real { "" } else { "virtual " },
            &meta.name,
            file_str,
            line
        );
    }
    println!(
        "Enter the bracketed number to choose that occurrence, or `a` for all, or empty to cancel."
    );

    let sel = prompt_line("select> ")?;
    if sel.eq_ignore_ascii_case("a") {
        return Ok(matches.to_vec());
    }
    if sel.is_empty() {
        anyhow::bail!("selection cancelled");
    }
    let n = sel
        .parse::<usize>()
        .map_err(|_| anyhow::anyhow!("invalid selection"))?;
    if n >= matches.len() {
        anyhow::bail!("selection out of range");
    }
    Ok(vec![matches[n]])
}

fn write_ucfs_dot_for_indices(
    orig_dot_path: &str,
    out_dot_path: &str,
    indices: &[u32],
) -> Result<()> {
    let orig_dot = std::fs::read_to_string(orig_dot_path)
        .map_err(|e| anyhow::anyhow!("Failed to read DOT {}: {}", orig_dot_path, e))?;

    let mut start_fragment = String::new();
    //start_fragment.push_str("    start [shape=point];\n");
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
        let mut new_dot = orig_dot.clone();
        new_dot.push_str("\n");
        new_dot.push_str(&start_fragment);
        new_dot
    };

    std::fs::write(out_dot_path, new_dot)
        .map_err(|e| anyhow::anyhow!("Failed to write UCFS DOT {}: {}", out_dot_path, e))?;
    Ok(())
}

fn handle_ucfs_mode(project_dir: &str, language: &str) -> Result<()> {
    let cflgraph = sgexport(&project_dir.to_string(), language.to_string())?;
    let cfl_dot_path = format!("{}.cfl.dot", project_dir);
    let ucfs_dot_path = format!("{}.cfl_ucfs.dot", project_dir);
    let grammar_kt_path = format!("{}.cfl_grammar.kt", project_dir);

    println!("UCFS artifacts generated:");
    println!("  - original CFL DOT: {}", cfl_dot_path);
    println!("  - Kotlin grammar (.kt): {}", grammar_kt_path);
    println!("Type a symbol name to select (or 'exit' to quit).");

    loop {
        let symbol = prompt_line("symbol> ")?;
        if symbol.is_empty() {
            continue;
        }
        if symbol.eq_ignore_ascii_case("exit")
            || symbol.eq_ignore_ascii_case("quit")
            || symbol.eq_ignore_ascii_case("done")
        {
            break;
        }
        let matches = collect_symbol_matches(&cflgraph, &symbol);
        if matches.is_empty() {
            println!("No occurrences found for `{}`.", symbol);
            continue;
        }
        let chosen = match choose_indices_interactive(&matches, &cflgraph) {
            Ok(c) => c,
            Err(e) => {
                println!("Selection aborted: {}", e);
                continue;
            }
        };
        write_ucfs_dot_for_indices(&cfl_dot_path, &ucfs_dot_path, &chosen)?;
        println!("Wrote UCFS query DOT: {}", ucfs_dot_path);
        println!(
            "Use this DOT together with the Kotlin grammar at:\n  {}",
            grammar_kt_path
        );
    }
    Ok(())
}

fn handle_interactive_mode(project_dir: &str, language: &str) -> Result<()> {
    let cflgraph = sgexport(&project_dir.to_string(), language.to_string())?;
    loop {
        let input = prompt_line(">>> ")?;
        if input.starts_with("q ") {
            let query = input[2..].trim();
            if !query.is_empty() {
                run_cflquery(&project_dir.to_string(), query, true, Some(&cflgraph))?;
            }
        } else if input.eq_ignore_ascii_case("end")
            || input.eq_ignore_ascii_case("exit")
            || input.eq_ignore_ascii_case("done")
        {
            break;
        }
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
        handle_interactive_mode(&project_dir, &language)?;
    } else if mode == "ucfs" {
        let project_dir = args.next().expect(
            "Usage: stackgraph_exporter ucfs <path-to-project-dir> [language: \"py\"|\"java\"]",
        );
        let language = args.next().unwrap_or_else(|| String::from("py"));
        handle_ucfs_mode(&project_dir, &language)?;
    } else {
        let project_dir = mode;
        let language = args.next().unwrap_or_else(|| String::from("py"));
        sgexport(&project_dir, language)?;
    }

    Ok(())
}
