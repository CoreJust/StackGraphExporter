use crate::{
    artifacts::{grammar_kt_pieces::*, progress_event::ProgressEvent},
    core::{CFLGraph, CFLSymbol},
    error::Result,
    io::ElapsedAndCount,
};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    time::Instant,
};

const WRITE_ONCE_IN_N: usize = 64;

pub trait ToKTGrammar {
    fn to_kotlin_lines<F>(&self, class_name: &str, progress: &mut F) -> Result<Vec<String>>
    where
        F: FnMut(ProgressEvent) -> Result<()>;

    fn write_to_kotlin_file<F>(
        &self,
        out_path: &PathBuf,
        class_name: &str,
        mut progress: F,
    ) -> Result<()>
    where
        F: FnMut(ProgressEvent) -> Result<()>,
    {
        let mut file = File::create(out_path)?;
        let start = Instant::now();
        let kt = self.to_kotlin_lines(class_name, &mut progress)?;
        let total_lines = kt.len();
        for (i, line) in kt.into_iter().enumerate() {
            writeln!(file, "{line}")?;
            if i % WRITE_ONCE_IN_N == 0 {
                progress(ProgressEvent::WritingLines {
                    elapsed_and_count: ElapsedAndCount {
                        current: i,
                        total: total_lines,
                        elapsed: start.elapsed(),
                    },
                    artifact_name: "Kotlin Grammar",
                })?;
            }
        }
        progress(ProgressEvent::ArtifactStored {
            elapsed_and_count: ElapsedAndCount {
                current: total_lines,
                total: total_lines,
                elapsed: start.elapsed(),
            },
            artifact_name: "Kotlin Grammar",
        })
    }
}

impl ToKTGrammar for CFLGraph {
    fn to_kotlin_lines<F>(&self, class_name: &str, progress: &mut F) -> Result<Vec<String>>
    where
        F: FnMut(ProgressEvent) -> Result<()>,
    {
        KotlinGrammarGenerator::new(self, class_name, progress).generate()
    }
}

struct KotlinGrammarGenerator<'a, F>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    graph: &'a CFLGraph,
    class_name: &'a str,
    progress: &'a mut F,
    start: Instant,
}

impl<'a, F> KotlinGrammarGenerator<'a, F>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    fn new(graph: &'a CFLGraph, class_name: &'a str, progress: &'a mut F) -> Self {
        Self {
            graph,
            class_name,
            progress,
            start: Instant::now(),
        }
    }

    fn generate(&mut self) -> Result<Vec<String>> {
        let non_terminal_indices = self.collect_non_terminal_indices()?;
        let name_map = self.build_name_map(&non_terminal_indices)?;
        let start_index = self.find_start_index(&non_terminal_indices);
        let productions = self.collect_productions()?;
        let mut kt_lines = vec![KT_GRAMMAR_HEADER.to_string()];
        self.append_class_declaration(&mut kt_lines);
        self.append_non_terminal_declarations(&mut kt_lines, &name_map, start_index)?;
        self.append_helper_functions(&mut kt_lines, &name_map)?;
        self.append_init_block(&mut kt_lines, &productions, &name_map)?;

        kt_lines.push("}".to_string());
        Ok(kt_lines)
    }

    fn collect_non_terminal_indices(&mut self) -> Result<BTreeSet<usize>> {
        let mut indices = BTreeSet::new();
        for (i, rule) in self.graph.rules.iter().enumerate() {
            indices.insert(rule.from_non_terminal);
            for symbol in &rule.to {
                if let CFLSymbol::NonTerminal(nt) = symbol {
                    indices.insert(*nt);
                }
            }
            (self.progress)(ProgressEvent::GeneratingArtifact {
                elapsed: self.start.elapsed(),
                progress: Some((i, self.graph.rules.len())),
                message: "Collecting non-terminal indices".into(),
            })?;
        }
        if indices.is_empty() && !self.graph.symbols.is_empty() {
            indices.insert(0);
        }
        Ok(indices)
    }

    fn build_name_map(&mut self, indices: &BTreeSet<usize>) -> Result<HashMap<usize, String>> {
        let mut name_map = HashMap::new();
        let mut used_names = HashSet::new();

        let total_indices = indices.len();
        for (i, &idx) in indices.into_iter().enumerate() {
            let raw_name = &self.graph.symbols[idx];
            let mut candidate = Self::sanitize_ident(raw_name, idx);
            let mut suffix = 1;
            while used_names.contains(&candidate) {
                candidate = format!("{}_{}", Self::sanitize_ident(raw_name, idx), suffix);
                suffix += 1;
            }
            used_names.insert(candidate.clone());
            name_map.insert(idx, candidate);
            (self.progress)(ProgressEvent::GeneratingArtifact {
                elapsed: self.start.elapsed(),
                progress: Some((i, total_indices)),
                message: "Building name map".into(),
            })?;
        }
        Ok(name_map)
    }

    fn sanitize_ident(s: &str, fallback_idx: usize) -> String {
        let mut out = String::with_capacity(s.len());
        for (i, ch) in s.chars().enumerate() {
            let valid = if i == 0 {
                ch.is_ascii_alphabetic() || ch == '_'
            } else {
                ch.is_ascii_alphanumeric() || ch == '_'
            };
            out.push(if valid { ch } else { '_' });
        }
        if out.is_empty() {
            out = format!("NT_{}", fallback_idx);
        }
        if out.chars().next().map_or(false, |c| c.is_ascii_digit()) {
            out.insert(0, '_');
        }
        out
    }

    fn find_start_index(&self, indices: &BTreeSet<usize>) -> usize {
        indices
            .iter()
            .find(|&&idx| self.graph.symbols[idx] == "S")
            .copied()
            .unwrap_or_else(|| *indices.first().unwrap())
    }

    fn collect_productions(&mut self) -> Result<BTreeMap<usize, Vec<Vec<CFLSymbol>>>> {
        let mut productions = BTreeMap::new();
        for (i, rule) in self.graph.rules.iter().enumerate() {
            productions
                .entry(rule.from_non_terminal)
                .or_insert_with(Vec::new)
                .push(rule.to.clone());
            (self.progress)(ProgressEvent::GeneratingArtifact {
                elapsed: self.start.elapsed(),
                progress: Some((i, self.graph.rules.len())),
                message: "Collecting productions".into(),
            })?;
        }
        Ok(productions)
    }

    fn append_class_declaration(&self, kt_lines: &mut Vec<String>) {
        kt_lines.push(format!("class {} : Grammar() {{\n", self.class_name));
    }

    fn append_non_terminal_declarations(
        &mut self,
        kt_lines: &mut Vec<String>,
        name_map: &HashMap<usize, String>,
        start_index: usize,
    ) -> Result<()> {
        let mut indices: Vec<_> = name_map.keys().copied().collect();
        indices.sort();
        let mut had_placeholder = false;
        let total_indices = indices.len();
        for (i, idx) in indices.into_iter().enumerate() {
            let var_name = &name_map[&idx];
            if idx == start_index {
                kt_lines.push(format!(
                    "\tval {} by Nt().asStart() // <placeholder nt=\"{0}\"/>",
                    var_name
                ));
                had_placeholder = true;
            } else {
                kt_lines.push(format!("\tval {} by Nt()", var_name));
            }
            (self.progress)(ProgressEvent::GeneratingArtifact {
                elapsed: self.start.elapsed(),
                progress: Some((i, total_indices)),
                message: "Appending non-terminal declarations".into(),
            })?;
        }
        if !had_placeholder {
            crate::error!("No start non-terminal index in grammar");
        }
        kt_lines.push("".to_string());
        Ok(())
    }

    fn append_helper_functions(
        &mut self,
        kt_lines: &mut Vec<String>,
        name_map: &HashMap<usize, String>,
    ) -> Result<()> {
        kt_lines.push(KT_GRAMMAR_PARSE_PRODUCTION_DATA.to_string());

        let mut indices: Vec<_> = name_map.keys().copied().collect();
        indices.sort();
        (self.progress)(ProgressEvent::GeneratingArtifact {
            elapsed: self.start.elapsed(),
            progress: None,
            message: "Appending helper functions".into(),
        })?;
        kt_lines.push(kt_grammar_get_nt(
            indices
                .into_iter()
                .map(|idx| &name_map[&idx])
                .collect::<Vec<_>>(),
        ));
        Ok(())
    }

    fn append_init_block(
        &mut self,
        kt_lines: &mut Vec<String>,
        productions: &BTreeMap<usize, Vec<Vec<CFLSymbol>>>,
        name_map: &HashMap<usize, String>,
    ) -> Result<()> {
        kt_lines.push("\tinit {".to_string());
        (self.progress)(ProgressEvent::GeneratingArtifact {
            elapsed: self.start.elapsed(),
            progress: None,
            message: "Appending init block".into(),
        })?;

        kt_lines.push("\t\tval productionData = \"\"\"".to_string());
        self.append_production_data(kt_lines, productions, name_map)?;
        kt_lines.push("\t\t\"\"\".trimIndent()\n".to_string());

        let nt_names: Vec<String> = name_map.values().cloned().collect();
        kt_lines.push(kt_grammar_productions_map_build(nt_names));
        kt_lines.push("\t}".to_string());
        Ok(())
    }

    fn append_production_data(
        &mut self,
        kt_lines: &mut Vec<String>,
        productions: &BTreeMap<usize, Vec<Vec<CFLSymbol>>>,
        name_map: &HashMap<usize, String>,
    ) -> Result<()> {
        (self.progress)(ProgressEvent::GeneratingArtifact {
            elapsed: self.start.elapsed(),
            progress: None,
            message: "Appending production data".into(),
        })?;
        for (&lhs_idx, alternatives) in productions {
            let lhs_name = &name_map[&lhs_idx];
            kt_lines.push(lhs_name.clone());

            let mut unique_alts = std::collections::HashSet::new();
            for rhs in alternatives {
                if rhs.is_empty() {
                    unique_alts.insert("Epsilon".to_string());
                    continue;
                }
                let mut tokens = Vec::with_capacity(rhs.len());
                for sym in rhs {
                    tokens.push(self.format_symbol(sym, name_map));
                }
                unique_alts.insert(tokens.join(" "));
            }

            let mut sorted_alts: Vec<_> = unique_alts.into_iter().collect();
            sorted_alts.sort();

            for alt in sorted_alts {
                kt_lines.push(format!("\t{}", alt));
            }
            kt_lines.push(String::new());
        }
        Ok(())
    }

    fn format_symbol(&self, symbol: &CFLSymbol, name_map: &HashMap<usize, String>) -> String {
        match symbol {
            CFLSymbol::Terminal(idx) => {
                let raw = &self.graph.symbols[*idx];
                let escaped = raw.replace('\\', "\\\\").replace('"', "\\\"");
                format!("\"{}\"", escaped)
            }
            CFLSymbol::NonTerminal(idx) => name_map[idx].clone(),
        }
    }
}
