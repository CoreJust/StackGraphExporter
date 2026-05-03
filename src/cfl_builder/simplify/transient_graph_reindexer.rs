use std::collections::HashMap;

use crate::{
    cfl_builder::{
        progress_event::{ProgressEvent, ProgressMonitor},
        simplify::simplification_stats::SimplificationStats,
        transient_graph::TGraph,
    },
    core::CFLRuleIndex,
    error::Result,
};

pub fn reindex_graph<F>(
    tgraph: &mut TGraph,
    progress_monitor: &mut ProgressMonitor<F>,
    simplification_stats: &mut SimplificationStats,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    progress_monitor.stage_total = tgraph.nodes.len();
    let mut reindexer = vec![u32::MAX; tgraph.nodes.len()];
    let mut next_node_index = 0u32;
    for (i, n) in tgraph.nodes.iter().enumerate() {
        progress_monitor.emit_simplification_nth("Recalculating node indices", i)?;
        if n.to_be_removed() {
            simplification_stats.total_nodes_removed += 1;
            continue;
        }
        reindexer[i] = next_node_index;
        next_node_index += 1;
    }
    let mut i = 0;
    tgraph.nodes.retain_mut(|n| {
        progress_monitor
            .emit_simplification_nth("Reindexing nodes", i)
            .expect("Oops");
        i += 1;
        if n.to_be_removed() {
            return false;
        }
        n.incoming
            .iter_mut()
            .for_each(|i| *i = reindexer[*i as usize]);
        n.outcoming
            .iter_mut()
            .for_each(|i| *i = reindexer[*i as usize]);
        true
    });
    Ok(())
}

fn new_rule_index(
    reindexer: &mut HashMap<CFLRuleIndex, CFLRuleIndex>,
    rule: CFLRuleIndex,
) -> CFLRuleIndex {
    if let Some(&new_rule) = reindexer.get(&rule) {
        new_rule
    } else {
        let new_rule = reindexer.len() as CFLRuleIndex;
        reindexer.insert(rule, new_rule);
        new_rule
    }
}

pub fn reindex_graph_rules<F>(
    tgraph: &mut TGraph,
    progress_monitor: &mut ProgressMonitor<F>,
    simplification_stats: &mut SimplificationStats,
) -> Result<()>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    progress_monitor.stage_total = tgraph.nodes.len();
    let mut reindexer = HashMap::new();
    for (i, n) in tgraph.nodes.iter_mut().enumerate() {
        progress_monitor.emit_simplification_nth("Recalculating rule indices for nodes", i)?;
        if let Some(rule) = n.rule() {
            let new_rule = new_rule_index(&mut reindexer, rule);
            n.symbol = Some(2 * new_rule + if n.is_push() { 0 } else { 1 });
        }
    }

    tgraph.potentially_virtual_rules = std::mem::take(&mut tgraph.potentially_virtual_rules)
        .into_iter()
        .filter_map(|r| reindexer.get(&r).and_then(|r| Some(*r)))
        .collect();
    tgraph.sg_to_cfl_rule_index.retain_mut(|r| {
        if let Some(&new_rule) = reindexer.get(&r) {
            *r = new_rule;
            true
        } else {
            false
        }
    });
    let rules_removed = tgraph.cfl_push_pop_rules_count - reindexer.len() as CFLRuleIndex;
    simplification_stats.total_rules_removed += rules_removed as usize;
    tgraph.cfl_push_pop_rules_count -= rules_removed;
    Ok(())
}
