// Based on https://github.com/FormalLanguageConstrainedPathQuerying/CFPQ_PyAlgo/blob/murav/optimize-matrix/docs/cli.md#grammar-format
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use crate::artifacts::cfl_display_symbol::CFLDisplaySymbol;
use crate::core::CFLGraph;
use crate::error::Result;

pub enum CNFCFGSymbol {
    Term(CFLDisplaySymbol),
    NonTerm(String),
}

pub enum CNFCFGRuleRightPart {
    Empty,
    #[allow(dead_code)]
    One(CNFCFGSymbol),
    Two(CNFCFGSymbol, CNFCFGSymbol),
}

pub trait ToCNFCFGGrammar {
    fn to_cnf_cfg_lines(self: &Self) -> (String, Vec<(String, CNFCFGRuleRightPart)>);

    fn write_to_cnf_cfg_file(self: &Self, out_path: &PathBuf) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut out_file = File::create(&out_path)?;
        let (start, rules) = self.to_cnf_cfg_lines();
        for (from, to) in rules.into_iter() {
            writeln!(out_file, "{from} {to}")?;
        }
        writeln!(out_file, "\nCount:\n{start}")?;
        Ok(())
    }
}

impl ToCNFCFGGrammar for CFLGraph {
    fn to_cnf_cfg_lines(self: &Self) -> (String, Vec<(String, CNFCFGRuleRightPart)>) {
        const PURIFY: bool = true;
        let mut rules = Vec::with_capacity((self.cfl_push_pop_rules_count * 2 + 4) as usize);
        rules.push(("S".into(), CNFCFGRuleRightPart::Empty));
        rules.push((
            "S".into(),
            CNFCFGRuleRightPart::Two(
                CNFCFGSymbol::NonTerm("S".into()),
                CNFCFGSymbol::Term(CFLDisplaySymbol::Epsilon),
            ),
        ));
        rules.push((
            "S".into(),
            CNFCFGRuleRightPart::Two(
                CNFCFGSymbol::NonTerm("S".into()),
                CNFCFGSymbol::NonTerm("Q".into()),
            ),
        ));
        rules.push((
            "S".into(),
            CNFCFGRuleRightPart::Two(
                CNFCFGSymbol::NonTerm("S".into()),
                CNFCFGSymbol::NonTerm("V".into()),
            ), // V - virtuals
        ));
        (0..self.cfl_push_pop_rules_count).for_each(|r| {
            rules.push((
                format!("S#psh{r}"),
                CNFCFGRuleRightPart::Two(
                    CNFCFGSymbol::Term(CFLDisplaySymbol::Push(r)),
                    CNFCFGSymbol::NonTerm("S".into()),
                ),
            ));
            rules.push((
                if PURIFY && self.potentially_virtual_rules.contains(&r) {
                    "V".into()
                } else {
                    "Q".into()
                },
                CNFCFGRuleRightPart::Two(
                    CNFCFGSymbol::NonTerm(format!("S#psh{r}")),
                    CNFCFGSymbol::Term(CFLDisplaySymbol::Pop(r)),
                ),
            ));
        });
        ("Q".into(), rules)
    }
}

impl Display for CNFCFGRuleRightPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, ""),
            Self::One(sym) => write!(f, "{sym}"),
            Self::Two(a, b) => write!(f, "{a} {b}"),
        }
    }
}

impl Display for CNFCFGSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Term(t) => write!(f, "{t}"),
            Self::NonTerm(nt) => write!(f, "{nt}"),
        }
    }
}
