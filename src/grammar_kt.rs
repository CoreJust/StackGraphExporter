use anyhow::Result;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::Write;

pub trait ToKTGrammar {
    fn to_kotlin_lines(&self, class_name: &str) -> Vec<String>;

    fn write_to_kotlin_file(&self, out_path: &str, class_name: &str) -> Result<()> {
        let mut file = File::create(out_path)?;
        for line in self.to_kotlin_lines(class_name).into_iter() {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }
}

impl ToKTGrammar for crate::types::CFLGraph {
    fn to_kotlin_lines(&self, class_name: &str) -> Vec<String> {
        fn sanitize_ident(s: &str, fallback_idx: usize) -> String {
            let mut out = String::with_capacity(s.len());
            for (i, ch) in s.chars().enumerate() {
                if (i == 0 && (ch.is_ascii_alphabetic() || ch == '_'))
                    || (i > 0 && (ch.is_ascii_alphanumeric() || ch == '_'))
                {
                    out.push(ch);
                } else {
                    out.push('_');
                }
            }
            if out.is_empty() {
                out = format!("NT_{}", fallback_idx);
            }
            if out.chars().next().unwrap().is_ascii_digit() {
                out = format!("_{}", out);
            }
            out
        }

        let mut nt_indices: BTreeMap<usize, ()> = BTreeMap::new();
        for r in &self.rules {
            nt_indices.insert(r.from_non_terminal, ());
            for sym in &r.to {
                if let crate::types::CFLSymbol::NonTerminal(i) = sym {
                    nt_indices.insert(*i, ());
                }
            }
        }
        if nt_indices.is_empty() && !self.symbols.is_empty() {
            nt_indices.insert(0usize, ());
        }

        let mut name_map: HashMap<usize, String> = HashMap::new();
        let mut used_names: HashSet<String> = HashSet::new();
        for &idx in nt_indices.keys() {
            let raw = &self.symbols[idx];
            let mut candidate = sanitize_ident(raw, idx);
            let mut suffix = 1usize;
            while used_names.contains(&candidate) {
                candidate = format!("{}_{}", candidate, suffix);
                suffix += 1;
            }
            used_names.insert(candidate.clone());
            name_map.insert(idx, candidate);
        }

        let mut found_start: Option<usize> = None;
        for &k in nt_indices.keys() {
            if self.symbols[k] == "S" {
                found_start = Some(k);
                break;
            }
        }
        let start_idx = found_start.unwrap_or_else(|| *nt_indices.keys().next().unwrap());

        let mut prods: BTreeMap<usize, Vec<Vec<crate::types::CFLSymbol>>> = BTreeMap::new();
        for r in &self.rules {
            prods
                .entry(r.from_non_terminal)
                .or_default()
                .push(r.to.clone());
        }

        let mut lines: Vec<String> = Vec::new();
        lines.push("package sg_bench\n".to_string());
        lines.push("import org.ucfs.grammar.combinator.Grammar".to_string());
        lines.push("import org.ucfs.grammar.combinator.regexp.*".to_string());
        lines.push("import org.ucfs.rsm.symbol.Term".to_string());
        lines.push("".to_string());

        lines.push(format!("class {} : Grammar() {{", class_name));
        lines.push("".to_string());

        for (idx, _) in nt_indices.iter() {
            let var = &name_map[idx];
            if *idx == start_idx {
                lines.push(format!("    val {} by Nt().asStart()", var));
            } else {
                lines.push(format!("    val {} by Nt()", var));
            }
        }
        lines.push("".to_string());

        lines.push("    init {".to_string());
        for (lhs_idx, alt_list) in prods.iter() {
            let lhs_var = &name_map[lhs_idx];
            let mut alt_exprs: HashSet<String> = HashSet::new();
            for rhs in alt_list.iter() {
                if rhs.is_empty() {
                    alt_exprs.insert("Epsilon".to_string());
                    continue;
                }
                let mut parts: Vec<String> = Vec::new();
                for sym in rhs.iter() {
                    match sym {
                        crate::types::CFLSymbol::Terminal(i) => {
                            let raw = &self.symbols[*i];
                            let esc = raw.replace('\\', "\\\\").replace('"', "\\\"");
                            parts.push(format!("Term(\"{}\")", esc));
                        }
                        crate::types::CFLSymbol::NonTerminal(i) => {
                            let name = name_map.get(i).expect("nonterminal mapping missing");
                            parts.push(name.clone());
                        }
                    }
                }
                alt_exprs.insert(parts.join(" * "));
            }

            let alt_exprs = alt_exprs.into_iter().collect::<Vec<_>>();
            let rhs_text = alt_exprs.join(", ");
            lines.push(format!(
                "      val {}_production = listOf({})",
                lhs_var, rhs_text
            ));
            lines.push(format!(
                "        {} /= Term(\"\") * S or S * Term(\"\") or {}_production.reduce {{ acc, prod -> acc or prod }}",
                lhs_var, lhs_var
            ));
        }
        lines.push("    }".to_string());
        lines.push("}".to_string());
        lines
    }
}
