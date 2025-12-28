mod converter;
mod dot;
mod from_serde;
mod loader;
mod types;

use converter::convert_to_cfl;
use dot::ToDOT;
use from_serde::FromSerde;
use loader::load_graph;
use types::SGGraph;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let project_dir = args
        .next()
        .expect("Usage: stackgraph_exporter <path-to-project-dir> [language: \"py\"|\"java\"]");
    let language = args.next().unwrap_or_else(|| String::from("py"));
    let (output_path, sg_output_dot, cfl_output_dot) = (
        format!("{}.stackgraph.json", &project_dir),
        format!("{}.stackgraph.dot", &project_dir),
        format!("{}.cfl.dot", &project_dir),
    );

    let stack_graph = load_graph(&project_dir, &language)?.to_serializable();
    let out_file = std::fs::File::create(&output_path)
        .with_context(|| format!("cannot create output file {}", output_path))?;
    serde_json::to_writer_pretty(out_file, &stack_graph)
        .with_context(|| format!("failed to write JSON to {}", output_path))?;

    let sggraph = SGGraph::from_serde(serde_json::to_value(stack_graph)?)?;
    sggraph.write_to_dot_file(&sg_output_dot)?;

    let cfl = convert_to_cfl(&sggraph)?;
    cfl.write_to_dot_file(&cfl_output_dot)?;

    println!(
        "Wrote stack graph JSON to {} and DOT to {}; CFL DOT to {}",
        output_path, sg_output_dot, cfl_output_dot
    );

    Ok(())
}
