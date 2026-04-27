use std::fmt::Display;
use std::path::PathBuf;

use crate::artifacts::cfl_display_symbol::CFLDisplaySymbol;
use crate::core::{CFLGraph, CFLNodeIndex};
use crate::error::Result;

pub trait ToG {
    type Node: Display + Ord;
    type Edge: Display + Ord;

    fn to_g_lines(self: &Self) -> Vec<(Self::Node, Self::Edge, Self::Node)>;

    fn write_to_g_file(self: &Self, out_path: &PathBuf) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut out_file = File::create(&out_path)?;
        let mut g_lines = self.to_g_lines();
        g_lines.sort();

        for (from, label, to) in g_lines {
            writeln!(out_file, "{from} {label} {to}")?;
        }
        Ok(())
    }
}

impl<'a> ToG for CFLGraph {
    type Node = CFLNodeIndex;
    type Edge = CFLDisplaySymbol;

    fn to_g_lines(self: &Self) -> Vec<(Self::Node, Self::Edge, Self::Node)> {
        self.edges
            .iter()
            .map(|e| {
                (
                    e.from,
                    match e.symbol {
                        None => CFLDisplaySymbol::Epsilon,
                        Some(index) => {
                            if index % 2 == 0 {
                                CFLDisplaySymbol::Push(index / 2)
                            } else {
                                CFLDisplaySymbol::Pop(index / 2)
                            }
                        }
                    },
                    e.to,
                )
            })
            .collect()
    }
}
