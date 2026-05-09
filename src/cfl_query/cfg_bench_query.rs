use crate::cfl_query::progress_event::ProgressEvent;
use crate::core::CFLRuleIndex;
use crate::error::Result;
use std::fs::{read_to_string, write};
use std::path::Path;
use std::time::Instant;

fn prepare_symbol_query_grammar(grammar_path: &Path, rule: CFLRuleIndex) -> Result<()> {
    write(
        grammar_path,
        format!(
            "S
S eps
S S S
S#psh_i psh_i S
S S#psh_i pp_i
Q S#psh_{rule} pp_{rule}

Count:
Q"
        ),
    )?;
    Ok(())
}

fn modify_g_file(g_path: &Path, indices: &[u32]) -> Result<()> {
    let mut result = read_to_string(g_path)?;
    if let Some(start_nodes_at) = result.find("->") {
        result.truncate(start_nodes_at);
    }

    for idx in indices {
        result.push_str(&format!("-> {}\n", idx));
    }

    write(g_path, result)?;
    Ok(())
}

pub fn cfg_bench_query<F>(
    grammar_path: &Path,
    g_path: &Path,
    rule: CFLRuleIndex,
    indices: &[u32],
    mut progress: F,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let start = Instant::now();

    progress(ProgressEvent::PreparingQueryGrammar {
        elapsed: start.elapsed(),
    })?;
    prepare_symbol_query_grammar(grammar_path, rule)?;
    progress(ProgressEvent::ModifyingG {
        elapsed: start.elapsed(),
    })?;
    modify_g_file(g_path, indices)?;
    progress(ProgressEvent::CfgBenchDone {
        elapsed: start.elapsed(),
    })?;

    Ok(())
}
