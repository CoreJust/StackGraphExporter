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

use anyhow::Result;
use conversion::{build_sggraph, ResolvedDefinition, StackGraphContext};
use converter::convert_to_cfl;
use core::{CFLGraph, CFLNodeIndex, SGNodeIndex};
use csv::ToCSV;
use dot::ToDOT;
use grammar_cfg::ToCFGGrammar;
use grammar_kt::ToKTGrammar;
use io::ProgressEvent as IoProgressEvent;
use loading::{load_stack_graph, Language};
use std::collections::HashMap;
use std::path::Path;

struct GraphBundle {
    stack_context: StackGraphContext,
    cfl_graph: CFLGraph,
    pop_sg_to_cfl_out: HashMap<SGNodeIndex, CFLNodeIndex>,
}

fn sgexport(project_dir: &str, language: &str) -> Result<GraphBundle> {
    let stack_graph = load_stack_graph(
        Path::new(project_dir),
        Language::from_str(language)?,
        |progress| progress.print_to_stdout(),
    )?;

    let context = build_sggraph(stack_graph, |event| event.print_to_stdout())?;

    let (cfl_graph, pop_sg_to_cfl_out) = convert_to_cfl(&context.sggraph, true)?;

    // Write output files
    let sg_output_dot = format!("{}.stackgraph.dot", project_dir);
    let cfl_output_dot = format!("{}.cfl.dot", project_dir);
    let cfl_output_csv = format!("{}.cfl.csv", project_dir);
    let cfl_output_grammar_cfg = format!("{}.cfl_grammar.cfg", project_dir);
    let cfl_output_grammar_kt = format!("{}.cfl_grammar.kt", project_dir);

    context.sggraph.write_to_dot_file(&sg_output_dot)?;
    cfl_graph.write_to_dot_file(&cfl_output_dot)?;
    cfl_graph.write_to_csv_file(&cfl_output_csv, false)?;
    cfl_graph.write_to_grammar_file(&cfl_output_grammar_cfg)?;
    cfl_graph.write_to_kotlin_file(&cfl_output_grammar_kt, "UCFSGrammar")?;

    println!(
        "Wrote stack graph DOT to {}; CFL DOT to {}, CSV to {}, grammar CFG to {}, Kotlin to {}",
        sg_output_dot,
        cfl_output_dot,
        cfl_output_csv,
        cfl_output_grammar_cfg,
        cfl_output_grammar_kt
    );

    Ok(GraphBundle {
        stack_context: context,
        cfl_graph,
        pop_sg_to_cfl_out,
    })
}

fn print_resolved_definitions(
    ref_idx: SGNodeIndex,
    defs: &[ResolvedDefinition],
    symbols: &[crate::core::SGSymbol],
) {
    let ref_symbol = symbols.iter().find(|s| {
        // Find symbol associated with the reference node (simplified)
        // In practice, you'd get it from the node itself, but we don't have direct mapping here.
        // For display, we'll just show the definitions.
        false
    });
    for def in defs {
        println!(
            "Found definition at {}:{}, local_id {}",
            def.file.as_deref().unwrap_or("<unknown>"),
            def.line.unwrap_or(0) + 1,
            def.local_id
        );
    }
}

fn handle_interactive_mode(project_dir: &str, language: &str) -> Result<()> {
    let mut bundle = sgexport(project_dir, language)?;

    println!("Interactive mode – type 'q <symbol>' to query stack graph, or 'exit' to quit.");
    loop {
        let input = prompt_line(">>> ")?;
        if input.starts_with("q ") {
            let symbol = input[2..].trim();
            if symbol.is_empty() {
                continue;
            }

            // Find reference nodes for the symbol
            let refs = bundle.stack_context.find_reference_nodes_by_symbol(symbol);
            println!(
                "Found {} reference nodes for symbol '{}':",
                refs.len(),
                symbol
            );
            for (i, &ref_idx) in refs.iter().enumerate() {
                let node_id = &bundle.stack_context.sggraph.ids[ref_idx as usize];
                let file_name = node_id
                    .file
                    .and_then(|f| bundle.stack_context.sggraph.files.get(f))
                    .map(String::as_str);
                let line = bundle
                    .stack_context
                    .sggraph
                    .symbols
                    .iter()
                    .find(|s| {
                        s.name == symbol
                            && s.file == node_id.file
                            && s.line == Some(node_id.local_id as usize)
                    })
                    .and_then(|s| s.line);
                println!(
                    "  [{}] node {} at {}:{}",
                    i,
                    ref_idx,
                    file_name.unwrap_or("<unknown>"),
                    line.map(|l| l + 1).unwrap_or(0)
                );
            }

            // For each reference, resolve definitions
            for &ref_idx in &refs {
                let defs = bundle
                    .stack_context
                    .resolve_reference(ref_idx, |event| event.print_to_stdout())?;
                println!(
                    "Reference node {} resolves to {} definitions:",
                    ref_idx,
                    defs.len()
                );
                for def in defs {
                    println!(
                        "  - {}:{} local_id {}",
                        def.file.as_deref().unwrap_or("<unknown>"),
                        def.line.unwrap_or(0) + 1,
                        def.local_id
                    );
                }
            }
        } else if input.eq_ignore_ascii_case("exit")
            || input.eq_ignore_ascii_case("quit")
            || input.eq_ignore_ascii_case("done")
        {
            break;
        }
    }
    Ok(())
}

fn handle_ucfs_mode(project_dir: &str, language: &str) -> Result<()> {
    let bundle = sgexport(project_dir, language)?;
    let cfl_dot_path = format!("{}.cfl.dot", project_dir);
    let ucfs_dot_path = format!("{}.cfl_ucfs.dot", project_dir);
    let grammar_kt_path = format!("{}.cfl_grammar.kt", project_dir);

    println!("UCFS mode – type a symbol name to select, or 'exit' to quit.");
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

        let refs = bundle.stack_context.find_reference_nodes_by_symbol(&symbol);
        if refs.is_empty() {
            println!("No reference nodes found for symbol '{}'.", symbol);
            continue;
        }

        // Display matches with file/line info
        println!("Found {} occurrences:", refs.len());
        for (i, &ref_idx) in refs.iter().enumerate() {
            let node_id = &bundle.stack_context.sggraph.ids[ref_idx as usize];
            let file_name = node_id
                .file
                .and_then(|f| bundle.stack_context.sggraph.files.get(f))
                .map(String::as_str);
            // Find line from symbol metadata (simplified: assume first matching symbol)
            let line = bundle
                .stack_context
                .sggraph
                .symbols
                .iter()
                .find(|s| s.name == symbol && s.file == node_id.file)
                .and_then(|s| s.line);
            println!(
                "  [{}] node {} at {}:{}",
                i,
                ref_idx,
                file_name.unwrap_or("<unknown>"),
                line.map(|l| l + 1).unwrap_or(0)
            );
        }

        println!("Enter the bracketed number to choose that occurrence, or `a` for all, or empty to cancel.");
        let sel = prompt_line("select> ")?;
        if sel.is_empty() {
            continue;
        }
        let chosen_indices_sg = if sel.eq_ignore_ascii_case("a") {
            refs
        } else {
            let n = sel
                .parse::<usize>()
                .map_err(|_| anyhow::anyhow!("Invalid number"))?;
            if n >= refs.len() {
                println!("Selection out of range.");
                continue;
            }
            vec![refs[n]]
        };

        // Convert SG node indices to CFL node indices for the DOT start arrows
        let mut chosen_cfl_indices = Vec::new();
        for sg_idx in chosen_indices_sg {
            // For a reference (push) node, its CFL index is the same as SG index
            chosen_cfl_indices.push(sg_idx as CFLNodeIndex);
        }

        write_ucfs_dot_for_indices(&cfl_dot_path, &ucfs_dot_path, &chosen_cfl_indices)?;
        println!("Wrote UCFS query DOT: {}", ucfs_dot_path);
        println!(
            "Use this DOT together with the Kotlin grammar at:\n  {}",
            grammar_kt_path
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

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let mode = args.next().expect(
        "Usage: stackgraph_exporter [q/query <path> <query> | i/interactive <dir> [lang] | ucfs <dir> [lang] | <dir> [lang]]",
    );

    if mode == "q" || mode == "query" {
        let project_dir = args.next().expect("Missing artifacts directory");
        let query = args.next().expect("Missing query");
        // Run CFL query using external tool – still works without paths
        let results = cfl_querier::cflquery(&project_dir, &query, true)?;
        println!("CFL query results:");
        for path in results {
            println!("{} -> {}", path.from, path.to);
        }
        Ok(())
    } else if mode == "i" || mode == "interactive" {
        let project_dir = args.next().expect("Missing project directory");
        let language = args.next().unwrap_or_else(|| "py".to_string());
        handle_interactive_mode(&project_dir, &language)
    } else if mode == "ucfs" {
        let project_dir = args.next().expect("Missing project directory");
        let language = args.next().unwrap_or_else(|| "py".to_string());
        handle_ucfs_mode(&project_dir, &language)
    } else {
        let project_dir = mode;
        let language = args.next().unwrap_or_else(|| "py".to_string());
        sgexport(&project_dir, &language)?;
        Ok(())
    }
}
