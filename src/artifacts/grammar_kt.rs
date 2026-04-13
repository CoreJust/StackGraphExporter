use crate::{
    artifacts::grammar_kt_pieces::*,
    core::{CFLGraph, CFLSymbol},
    error::Result,
};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub trait ToKTGrammar {
    fn to_kotlin_lines(&self, class_name: &str) -> Vec<String>;

    fn write_to_kotlin_file(&self, out_path: &PathBuf, class_name: &str) -> Result<()> {
        let mut file = File::create(out_path)?;
        let kt = self.to_kotlin_lines(class_name);
        for line in kt {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }
}

impl ToKTGrammar for CFLGraph {
    fn to_kotlin_lines(&self, class_name: &str) -> Vec<String> {
        KotlinGrammarGenerator::new(self, class_name).generate()
    }
}

struct KotlinGrammarGenerator<'a> {
    graph: &'a CFLGraph,
    class_name: &'a str,
}

impl<'a> KotlinGrammarGenerator<'a> {
    fn new(graph: &'a CFLGraph, class_name: &'a str) -> Self {
        Self { graph, class_name }
    }

    fn generate(&self) -> Vec<String> {
        let non_terminal_indices = self.collect_non_terminal_indices();
        let name_map = self.build_name_map(&non_terminal_indices);
        let start_index = self.find_start_index(&non_terminal_indices);
        let productions = self.collect_productions();
        let mut kt_lines = vec![KT_GRAMMAR_HEADER.to_string()];
        self.append_class_declaration(&mut kt_lines);
        Self::append_non_terminal_declarations(&mut kt_lines, &name_map, start_index);
        self.append_helper_functions(&mut kt_lines, &name_map);
        self.append_init_block(&mut kt_lines, &productions, &name_map);

        kt_lines.push("}".to_string());
        kt_lines
    }

    fn collect_non_terminal_indices(&self) -> BTreeSet<usize> {
        let mut indices = BTreeSet::new();
        for rule in &self.graph.rules {
            indices.insert(rule.from_non_terminal);
            for symbol in &rule.to {
                if let CFLSymbol::NonTerminal(nt) = symbol {
                    indices.insert(*nt);
                }
            }
        }
        if indices.is_empty() && !self.graph.symbols.is_empty() {
            indices.insert(0);
        }
        indices
    }

    fn build_name_map(&self, indices: &BTreeSet<usize>) -> HashMap<usize, String> {
        let mut name_map = HashMap::new();
        let mut used_names = HashSet::new();

        for &idx in indices {
            let raw_name = &self.graph.symbols[idx];
            let mut candidate = Self::sanitize_ident(raw_name, idx);
            let mut suffix = 1;
            while used_names.contains(&candidate) {
                candidate = format!("{}_{}", Self::sanitize_ident(raw_name, idx), suffix);
                suffix += 1;
            }
            used_names.insert(candidate.clone());
            name_map.insert(idx, candidate);
        }
        name_map
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

    fn collect_productions(&self) -> BTreeMap<usize, Vec<Vec<CFLSymbol>>> {
        let mut productions = BTreeMap::new();
        for rule in &self.graph.rules {
            productions
                .entry(rule.from_non_terminal)
                .or_insert_with(Vec::new)
                .push(rule.to.clone());
        }
        productions
    }

    fn append_class_declaration(&self, kt_lines: &mut Vec<String>) {
        kt_lines.push(format!("class {} : Grammar() {{\n", self.class_name));
    }

    fn append_non_terminal_declarations(
        kt_lines: &mut Vec<String>,
        name_map: &HashMap<usize, String>,
        start_index: usize,
    ) {
        let mut indices: Vec<_> = name_map.keys().copied().collect();
        indices.sort();
        let mut had_placeholder = false;
        for idx in indices {
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
        }
        if !had_placeholder {
            crate::error!("No start non-terminal index in grammar");
        }
        kt_lines.push("".to_string());
    }

    fn append_helper_functions(
        &self,
        kt_lines: &mut Vec<String>,
        name_map: &HashMap<usize, String>,
    ) {
        kt_lines.push(KT_GRAMMAR_PARSE_PRODUCTION_DATA.to_string());

        let mut indices: Vec<_> = name_map.keys().copied().collect();
        indices.sort();
        kt_lines.push(kt_grammar_get_nt(
            indices
                .into_iter()
                .map(|idx| &name_map[&idx])
                .collect::<Vec<_>>(),
        ));
    }

    fn append_init_block(
        &self,
        kt_lines: &mut Vec<String>,
        productions: &BTreeMap<usize, Vec<Vec<CFLSymbol>>>,
        name_map: &HashMap<usize, String>,
    ) {
        kt_lines.push("\tinit {".to_string());

        kt_lines.push("\t\tval productionData = \"\"\"".to_string());
        self.append_production_data(kt_lines, productions, name_map);
        kt_lines.push("\t\t\"\"\".trimIndent()\n".to_string());

        let nt_names: Vec<String> = name_map.values().cloned().collect();
        kt_lines.push(kt_grammar_productions_map_build(nt_names));
        kt_lines.push("\t}".to_string());
    }

    fn append_production_data(
        &self,
        kt_lines: &mut Vec<String>,
        productions: &BTreeMap<usize, Vec<Vec<CFLSymbol>>>,
        name_map: &HashMap<usize, String>,
    ) {
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
