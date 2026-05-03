use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use crate::artifacts::cfl_display_symbol::CFLDisplaySymbol;
use crate::core::CFLGraph;
use crate::error::Result;

pub struct NonTerminal(String);

pub enum CNFRuleRightPart {
    Empty,
    Terminal(CFLDisplaySymbol),
    NonTerminals(NonTerminal, NonTerminal),
}

pub trait ToCNFGrammar {
    fn to_cnf_lines(
        self: &Self,
        inverse_grammar: bool,
    ) -> (NonTerminal, Vec<(NonTerminal, CNFRuleRightPart)>);

    fn write_to_cnf_file(self: &Self, out_path: &PathBuf, inverse_grammar: bool) -> Result<()> {
        let mut out_file = BufWriter::new(File::create(&out_path)?);
        let (start, rules) = self.to_cnf_lines(inverse_grammar);
        writeln!(out_file, "{}\n", start.0)?;
        for (from, to) in rules.into_iter() {
            writeln!(out_file, "{} -> {to}", from.0)?;
        }
        Ok(())
    }
}

impl ToCNFGrammar for CFLGraph {
    fn to_cnf_lines(
        self: &Self,
        inverse_grammar: bool,
    ) -> (NonTerminal, Vec<(NonTerminal, CNFRuleRightPart)>) {
        let mut rules = Vec::with_capacity((self.cfl_push_pop_rules_count * 4 + 5) as usize);
        rules.push((
            NonTerminal("Eps".into()),
            CNFRuleRightPart::Terminal(CFLDisplaySymbol::Epsilon),
        ));
        rules.push((NonTerminal("S".into()), CNFRuleRightPart::Empty));
        rules.push((
            NonTerminal("S".into()),
            CNFRuleRightPart::NonTerminals(NonTerminal("Eps".into()), NonTerminal("S".into())),
        ));
        rules.push((
            NonTerminal("S".into()),
            CNFRuleRightPart::NonTerminals(NonTerminal("Q".into()), NonTerminal("S".into())),
        ));
        rules.push((
            NonTerminal("S".into()),
            CNFRuleRightPart::NonTerminals(NonTerminal("V".into()), NonTerminal("S".into())), // V - virtuals
        ));
        (0..self.cfl_push_pop_rules_count).for_each(|r| {
            if inverse_grammar {
                rules.push((
                    NonTerminal(format!("NT#pp{r}")),
                    CNFRuleRightPart::Terminal(CFLDisplaySymbol::Pop(r)),
                ));
                rules.push((
                    NonTerminal(format!("NT#psh{r}")),
                    CNFRuleRightPart::Terminal(CFLDisplaySymbol::Push(r)),
                ));
                rules.push((
                    NonTerminal(format!("S#pp{r}")),
                    CNFRuleRightPart::NonTerminals(
                        NonTerminal(format!("NT#pp{r}")),
                        NonTerminal("S".into()),
                    ),
                ));
                rules.push((
                    NonTerminal(if self.potentially_virtual_rules.contains(&r) {
                        "V".into()
                    } else {
                        "Q".into()
                    }),
                    CNFRuleRightPart::NonTerminals(
                        NonTerminal(format!("S#pp{r}")),
                        NonTerminal(format!("NT#psh{r}")),
                    ),
                ));
            } else {
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
                        NonTerminal("S".into()),
                    ),
                ));
                rules.push((
                    NonTerminal(if self.potentially_virtual_rules.contains(&r) {
                        "V".into()
                    } else {
                        "Q".into()
                    }),
                    CNFRuleRightPart::NonTerminals(
                        NonTerminal(format!("S#psh{r}")),
                        NonTerminal(format!("NT#pp{r}")),
                    ),
                ));
            }
        });
        (NonTerminal("Q".into()), rules)
    }
}

impl Display for CNFRuleRightPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, ""),
            Self::Terminal(sym) => write!(f, "{sym}"),
            Self::NonTerminals(a, b) => write!(f, "{} {}", a.0, b.0),
        }
    }
}
