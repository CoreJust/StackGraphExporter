use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use crate::artifacts::cfl_display_symbol::CFLDisplaySymbol;
use crate::artifacts::virtual_rules_indices_splitter::split_virtual_rules_indices;
use crate::core::{CFLGraph, CFLNodeIndex, CFLRuleIndex, CFLSymbolIndex};
use crate::error::Result;

pub enum GOrder {
    FromLabelTo,
    FromToLabel,
}

pub trait ToG {
    fn to_g_lines(
        self: &Self,
        for_query_generation: bool,
        index_rules: bool,
        inverse_graph: bool,
    ) -> Vec<(CFLNodeIndex, CFLDisplaySymbol, CFLNodeIndex)>;

    fn write_to_g_file(
        self: &Self,
        out_path: &PathBuf,
        for_query_generation: bool,
        order: GOrder,
        index_rules: bool,
        inverse_graph: bool,
    ) -> Result<()> {
        if index_rules {
            assert!(matches!(order, GOrder::FromToLabel));
        }

        let mut out_file = BufWriter::new(File::create(&out_path)?);
        let mut g_lines = self.to_g_lines(for_query_generation, index_rules, inverse_graph);
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

fn cfl_symbol_to_display(
    index: CFLSymbolIndex,
    virtual_indices: &[Option<CFLRuleIndex>],
    real_indices: &[Option<CFLRuleIndex>],
    index_rules: bool,
    inverse_graph: bool,
) -> CFLDisplaySymbol {
    let rule = index / 2;
    let expected_push_modulo = if inverse_graph { 1 } else { 0 };
    if index_rules && virtual_indices[rule as usize].is_some() {
        let rule = virtual_indices[rule as usize].unwrap();
        if index % 2 == expected_push_modulo {
            CFLDisplaySymbol::VPush(rule)
        } else {
            CFLDisplaySymbol::VPop(rule)
        }
    } else {
        let rule = if index_rules {
            real_indices[rule as usize].unwrap()
        } else {
            rule
        };
        if index % 2 == expected_push_modulo {
            CFLDisplaySymbol::Push(rule)
        } else {
            CFLDisplaySymbol::Pop(rule)
        }
    }
}

impl<'a> ToG for CFLGraph {
    fn to_g_lines(
        self: &Self,
        for_query_generation: bool,
        index_rules: bool,
        inverse_graph: bool,
    ) -> Vec<(CFLNodeIndex, CFLDisplaySymbol, CFLNodeIndex)> {
        let (virtual_indices, real_indices) =
            split_virtual_rules_indices(self, for_query_generation);
        self.edges
            .iter()
            .map(|e| {
                (
                    e.from,
                    match e.symbol {
                        None => CFLDisplaySymbol::Epsilon,
                        Some(index) => cfl_symbol_to_display(
                            index,
                            &virtual_indices,
                            &real_indices,
                            index_rules,
                            inverse_graph,
                        ),
                    },
                    e.to,
                )
            })
            .collect()
    }
}
