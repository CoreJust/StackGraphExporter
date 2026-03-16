use crate::core::CFLGraph;
use anyhow::Result;

pub trait ToCFGGrammar {
    fn to_grammar_lines(self: &Self) -> Vec<String>;

    fn write_to_grammar_file(self: &Self, out_path: &String) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut out_file = File::create(&out_path)?;
        for line in self.to_grammar_lines().into_iter() {
            writeln!(out_file, "{}", line)?;
        }
        Ok(())
    }
}

impl ToCFGGrammar for CFLGraph {
    fn to_grammar_lines(self: &Self) -> Vec<String> {
        self.rules
            .iter()
            .map(|r| {
                let mut right_part =
                    r.to.iter()
                        .map(|s| {
                            match s {
                                crate::core::CFLSymbol::Terminal(i) => {
                                    format!("Terminal(\"{}\")", &self.symbols[*i])
                                }
                                crate::core::CFLSymbol::NonTerminal(i) => {
                                    format!("Nonterminal(\"{}\")", &self.symbols[*i])
                                }
                            }
                            .clone()
                        })
                        .collect::<Vec<_>>()
                        .join(" ");
                if right_part.is_empty() {
                    right_part = "Terminal(\"eps\")".to_owned();
                }
                format!(
                    "Nonterminal(\"{}\") -> {}",
                    &self.symbols[r.from_non_terminal], right_part
                )
            })
            .collect()
    }
}
