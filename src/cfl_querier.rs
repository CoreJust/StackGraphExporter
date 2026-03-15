use crate::core::{CFLNodeIndex, CFLPath};
use anyhow::Result;
use regex::Regex;
use std::path::Path;

pub fn cflquery(artifacts_dir: &String, query: &str, sppf_on: bool) -> Result<Vec<CFLPath>> {
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

pub fn ucfs_cflquery(artifacts_dir: &str, query: &str) -> Result<String> {
    use std::fs;

    let grammar_kt_path = format!("{}.cfl_grammar.kt", &artifacts_dir);
    let grammar_path = Path::new(grammar_kt_path.as_str());
    let query_grammar_path = format!("{}.cfl_query.kt", artifacts_dir);

    let content = fs::read_to_string(grammar_path)
        .map_err(|e| anyhow::anyhow!("Failed to read grammar kotlin file: {}", e))?;

    // Attempt to find existing start nonterminal declared with `.asStart()`:
    // pattern: val <name> by Nt( ... ).asStart()
    // We'll remove `.asStart()` from that declaration and record the name to use in wrapper.
    let re_start_decl =
        Regex::new(r"(val\s+([A-Za-z_][A-Za-z0-9_]*)\s+by\s+Nt\([^)]*\))\.asStart\(\)").unwrap();

    let (content_without_asstart, start_name) = if let Some(cap) = re_start_decl.captures(&content)
    {
        // cap[1] is the "val X by Nt(...)" portion, cap[2] is var name
        let var_name = cap.get(2).unwrap().as_str();
        let content_replaced = re_start_decl.replace(&content, "$1").to_string();
        (content_replaced, var_name.to_string())
    } else {
        // if not found, try to locate the first `val <name> by Nt()` declaration and pick that as start
        let re_first_nt = Regex::new(r"val\s+([A-Za-z_][A-Za-z0-9_]*)\s+by\s+Nt\(\)").unwrap();
        if let Some(cap) = re_first_nt.captures(&content) {
            let var_name = cap.get(1).unwrap().as_str().to_string();
            (content.clone(), var_name)
        } else {
            return Err(anyhow::anyhow!(
                "Could not find any nonterminal declaration `val NAME by Nt()` in grammar kotlin file."
            ));
        }
    };

    // Insert the new `val Q by Nt(Term("push_{q}") * <start> * Term("pop_{q}")).asStart()` before the `init {` block.
    // We'll search for first occurrence of `init {` and insert our declaration immediately before it.
    let insert_before = "init {";
    if let Some(pos) = content_without_asstart.find(insert_before) {
        // split
        let (head, tail) = content_without_asstart.split_at(pos);
        // prepare the Q declaration with indentation similar to other declarations
        // but be conservative and use 4 spaces indentation
        let q_decl = format!(
            "    val Q by Nt(Term(\"push_{q}\") * {start} * Term(\"pop_{q}\")).asStart()\n\n",
            q = query.replace('"', "\\\""),
            start = start_name
        );
        let new_content = format!("{}{}{}", head, q_decl, tail);
        fs::write(&query_grammar_path, new_content)
            .map_err(|e| anyhow::anyhow!("Failed to write query kotlin file: {}", e))?;
        Ok(query_grammar_path)
    } else {
        Err(anyhow::anyhow!(
            "Could not find `init {{` block in the grammar kotlin file (needed to insert query start)."
        ))
    }
}
