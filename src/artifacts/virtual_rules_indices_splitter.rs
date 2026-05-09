use crate::core::{CFLGraph, CFLRuleIndex};

pub fn split_virtual_rules_indices(
    graph: &CFLGraph,
    for_query_generation: bool,
) -> (Vec<Option<CFLRuleIndex>>, Vec<Option<CFLRuleIndex>>) {
    let mut i = 0;
    let virtual_indices = (0..graph.cfl_push_pop_rules_count)
        .map(|r| {
            if graph.potentially_virtual_rules.contains(&r) && !for_query_generation {
                let new_index = i;
                i += 1;
                Some(new_index)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut i = 0;
    let real_indices = (0..graph.cfl_push_pop_rules_count)
        .map(|r| {
            if !graph.potentially_virtual_rules.contains(&r) || for_query_generation {
                let new_index = i;
                i += 1;
                Some(new_index)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    (virtual_indices, real_indices)
}
