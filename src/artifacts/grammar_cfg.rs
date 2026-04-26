use std::path::PathBuf;

use crate::core::CFLGraph;
use crate::error::Result;

pub trait ToCFGGrammar {
    fn to_grammar_lines(self: &Self) -> Vec<String>;

    fn write_to_grammar_file(self: &Self, out_path: &PathBuf) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut out_file = File::create(&out_path)?;
        for line in self.to_grammar_lines().into_iter() {
            writeln!(out_file, "{line}")?;
        }
        Ok(())
    }
}

impl ToCFGGrammar for CFLGraph {
    fn to_grammar_lines(self: &Self) -> Vec<String> {
        let mut result = Vec::with_capacity(self.sg_unique_symbols_count * 2 + 2);
        (0..self.sg_unique_symbols_count).for_each(|r| {
            result.push(format!(
                "Nonterminal(\"S\") -> Terminal(\"psh{0}\") S Terminal(\"pp{0}\")",
                r,
            ));
        });
        result.push("Nonterminal(\"S\") -> Nonterminal(\"S\") Nonterminal(\"S\")".into());
        result.push("Nonterminal(\"S\") -> Terminal(\"eps\")".into());
        result
    }
}
