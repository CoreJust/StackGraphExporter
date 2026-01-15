use crate::types::{CFLNodeIndex, CFLPath};

pub fn cflquery(
    artifacts_dir: &String,
    query: &str,
    sppf_on: bool,
) -> anyhow::Result<Vec<CFLPath>> {
    use std::fs;
    use std::process::Command;

    let graph_path = format!("{}.cfl.csv", &artifacts_dir);
    let grammar_path = format!("{}.cfl_grammar.cfg", &artifacts_dir);
    let query_grammar_path = format!("{}.cfl_query.cfg", &artifacts_dir);
    let output_path = format!("{}.output.txt", &artifacts_dir);
    let grammar = match fs::read_to_string(grammar_path) {
        Ok(content) => content,
        Err(e) => return Err(anyhow::Error::msg(e.to_string())),
    };
    fs::write(
        &query_grammar_path,
        format!(
          "StartNonterminal(\"Q\")\nNonterminal(\"Q\") -> Terminal(\"push_{0}\") Nonterminal(\"S\") Terminal(\"pop_{0}\")\n{1}",
          query,
          &grammar),
    )
    .map_err(|e| anyhow::Error::msg(e.to_string()))?;
    let output = Command::new("java")
        .arg("-jar")
        .arg("C:/kotgll-1.0.8.jar")
        .arg("--input")
        .arg("graph")
        .arg("--grammar")
        .arg("cfg")
        .arg("--sppf")
        .arg(if sppf_on { "on" } else { "off" })
        .arg("--inputPath")
        .arg(graph_path)
        .arg("--grammarPath")
        .arg(query_grammar_path)
        .arg("--outputPath")
        .arg(&output_path)
        .output()
        .expect("Failed to run kotgll");
    if !output.status.success() {
        return Err(anyhow::Error::msg(format!("Status: {}", output.status)));
    }
    if !output.stdout.is_empty() {
        return Err(anyhow::Error::msg(format!(
            "Stdout: {}",
            String::from_utf8_lossy(&output.stdout)
        )));
    }
    if !output.stderr.is_empty() {
        return Err(anyhow::Error::msg(format!(
            "Stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(fs::read_to_string(output_path)?
        .split("\n")
        .filter(|r| r.contains(' '))
        .map(|r| {
            let mut indices = r
                .split_whitespace()
                .map(|x| x.parse::<CFLNodeIndex>().unwrap());
            CFLPath {
                from: indices.next().unwrap(),
                to: indices.next().unwrap(),
            }
        })
        .collect())
}
