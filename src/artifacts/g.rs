use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use crate::artifacts::cfl_display_symbol::CFLDisplaySymbol;
use crate::core::{CFLGraph, CFLNodeIndex};
use crate::error::Result;

pub enum GOrder {
    FromLabelTo,
    FromToLabel,
}

pub trait ToG {
    fn to_g_lines(
        self: &Self,
        index_rules: bool,
    ) -> Vec<(CFLNodeIndex, CFLDisplaySymbol, CFLNodeIndex)>;

    fn write_to_g_file(
        self: &Self,
        out_path: &PathBuf,
        order: GOrder,
        index_rules: bool,
        inverse_graph: bool,
    ) -> Result<()> {
        if index_rules {
            assert!(matches!(order, GOrder::FromToLabel));
        }

        let mut out_file = BufWriter::new(File::create(&out_path)?);
        let mut g_lines = self.to_g_lines(index_rules);
        g_lines.sort();

        for (from, label, to) in g_lines {
            let (from, to) = if inverse_graph {
                (to, from)
            } else {
                (from, to)
            };
            match order {
                GOrder::FromLabelTo => writeln!(out_file, "{from} {label} {to}"),
                GOrder::FromToLabel => {
                    if index_rules && label.rule().is_some() {
                        writeln!(
                            out_file,
                            "{from} {to} {} {}",
                            label.short_name(),
                            label.rule().unwrap(),
                        )
                    } else {
                        writeln!(out_file, "{from} {to} {label}")
                    }
                }
            }?;
        }
        Ok(())
    }
}

impl<'a> ToG for CFLGraph {
    fn to_g_lines(
        self: &Self,
        index_rules: bool,
    ) -> Vec<(CFLNodeIndex, CFLDisplaySymbol, CFLNodeIndex)> {
        self.edges
            .iter()
            .map(|e| {
                (
                    e.from,
                    match e.symbol {
                        None => CFLDisplaySymbol::Epsilon,
                        Some(index) => {
                            let rule = index / 2;
                            if index_rules && self.potentially_virtual_rules.contains(&rule) {
                                if index % 2 == 0 {
                                    CFLDisplaySymbol::VPush(rule)
                                } else {
                                    CFLDisplaySymbol::VPop(rule)
                                }
                            } else {
                                if index % 2 == 0 {
                                    CFLDisplaySymbol::Push(rule)
                                } else {
                                    CFLDisplaySymbol::Pop(rule)
                                }
                            }
                        }
                    },
                    e.to,
                )
            })
            .collect()
    }
}
