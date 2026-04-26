use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use crate::artifacts::cfl_display_symbol::CFLDisplaySymbol;
use crate::core::CFLGraph;
use crate::error::Result;

pub struct NonTerminal(String);

pub enum CNFRuleRightPart {
    Terminal(CFLDisplaySymbol),
    NonTerminals(NonTerminal, NonTerminal),
}

pub trait ToCNFGrammar {
    fn to_cnf_lines(self: &Self) -> (NonTerminal, Vec<(NonTerminal, CNFRuleRightPart)>);

    fn write_to_cnf_file(self: &Self, out_path: &PathBuf) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut out_file = File::create(&out_path)?;
        let (start, rules) = self.to_cnf_lines();
        writeln!(out_file, "{}\n", start.0)?;
        for (from, to) in rules.into_iter() {
            writeln!(out_file, "{} -> {to}", from.0)?;
        }
        Ok(())
    }
}

impl ToCNFGrammar for CFLGraph {
    fn to_cnf_lines(self: &Self) -> (NonTerminal, Vec<(NonTerminal, CNFRuleRightPart)>) {
        let mut rules = Vec::with_capacity(self.sg_unique_symbols_count * 6 + 4);
        (0..self.sg_unique_symbols_count).for_each(|r| {
            rules.push((
                NonTerminal(format!("NT#psh{r}")),
                CNFRuleRightPart::Terminal(CFLDisplaySymbol::Push(r)),
            ));
            rules.push((
                NonTerminal(format!("NT#pp{r}")),
                CNFRuleRightPart::Terminal(CFLDisplaySymbol::Pop(r)),
            ));
            rules.push((
                NonTerminal(format!("S#psh{r}")),
                CNFRuleRightPart::NonTerminals(
                    NonTerminal(format!("NT#psh{r}")),
                    NonTerminal("SEps".into()),
                ),
            ));
            rules.push((
                NonTerminal(format!("S#psh{r}")),
                CNFRuleRightPart::NonTerminals(
                    NonTerminal(format!("NT#psh{r}")),
                    NonTerminal("S".into()),
                ),
            ));
            rules.push((
                NonTerminal("S".into()),
                CNFRuleRightPart::NonTerminals(
                    NonTerminal(format!("S#psh{r}")),
                    NonTerminal(format!("NT#pp{r}")),
                ),
            ));
            rules.push((
                NonTerminal("S".into()),
                CNFRuleRightPart::NonTerminals(
                    NonTerminal(format!("NT#psh{r}")),
                    NonTerminal(format!("NT#pp{r}")),
                ),
            ));
        });
        rules.push((
            NonTerminal("Eps".into()),
            CNFRuleRightPart::Terminal(CFLDisplaySymbol::Epsilon),
        ));
        rules.push((
            NonTerminal("SEps".into()),
            CNFRuleRightPart::NonTerminals(NonTerminal("SEps".into()), NonTerminal("SEps".into())),
        ));
        rules.push((
            NonTerminal("SEps".into()),
            CNFRuleRightPart::NonTerminals(NonTerminal("S".into()), NonTerminal("Eps".into())),
        ));
        rules.push((
            NonTerminal("SEps".into()),
            CNFRuleRightPart::NonTerminals(NonTerminal("Eps".into()), NonTerminal("S".into())),
        ));
        (NonTerminal("S".into()), rules)
    }
}

impl Display for CNFRuleRightPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Terminal(sym) => write!(f, "{sym}"),
            Self::NonTerminals(a, b) => write!(f, "{} {}", a.0, b.0),
        }
    }
}
