use super::indexers::NodeIdIndexer;
use crate::core::SGNode;

pub fn resolve_push_scoped(nodes: &mut [SGNode], node_id_indexer: &NodeIdIndexer) {
    for node in nodes.iter_mut() {
        if let SGNode::PushScopedUnresolved(symbol, scope_id) = node {
            if let Some(scope_index) = node_id_indexer.get_index(scope_id) {
                *node = SGNode::PushScoped(*symbol, scope_index);
            }
        }
    }
}
