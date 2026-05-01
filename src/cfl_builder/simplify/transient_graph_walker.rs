use crate::{
    cfl_builder::transient_graph::{TNode, TNodeIndex},
    core::CFLRuleIndex,
};

pub trait TGraphWalker {
    fn next_vertices(node: &TNode) -> &[TNodeIndex];
    #[allow(dead_code)]
    fn prev_vertices(node: &TNode) -> &[TNodeIndex];

    fn as_opening(node: &TNode) -> Option<CFLRuleIndex>;
    fn as_closing(node: &TNode) -> Option<CFLRuleIndex>;
}

pub struct ForwardTGraphWalker; // For push nodes
pub struct BackwardTGraphWalker; // For pop nodes

impl TGraphWalker for ForwardTGraphWalker {
    fn next_vertices(node: &TNode) -> &[TNodeIndex] {
        &node.outcoming
    }

    fn prev_vertices(node: &TNode) -> &[TNodeIndex] {
        &node.incoming
    }

    fn as_opening(node: &TNode) -> Option<CFLRuleIndex> {
        if node.is_push() {
            node.rule()
        } else {
            None
        }
    }

    fn as_closing(node: &TNode) -> Option<CFLRuleIndex> {
        if node.is_pop() {
            node.rule()
        } else {
            None
        }
    }
}

impl TGraphWalker for BackwardTGraphWalker {
    fn next_vertices(node: &TNode) -> &[TNodeIndex] {
        &node.incoming
    }

    fn prev_vertices(node: &TNode) -> &[TNodeIndex] {
        &node.outcoming
    }

    fn as_opening(node: &TNode) -> Option<CFLRuleIndex> {
        if node.is_pop() {
            node.rule()
        } else {
            None
        }
    }

    fn as_closing(node: &TNode) -> Option<CFLRuleIndex> {
        if node.is_push() {
            node.rule()
        } else {
            None
        }
    }
}
