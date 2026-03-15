use std::time::Instant;

use stack_graphs::graph::StackGraph;

use super::ProgressEvent;
use crate::error::Result;

pub fn merge_graphs_with_progress<F>(
    local_graphs: Vec<StackGraph>,
    start_time: Instant,
    progress: &mut F,
) -> Result<StackGraph>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    let total = local_graphs.len();
    let mut final_graph = StackGraph::new();

    for (i, local_graph) in local_graphs.into_iter().enumerate() {
        final_graph
            .add_from_graph(&local_graph)
            .expect("File name conflict during merge - this should not happen with unique paths");

        progress(ProgressEvent::MergeProgress {
            current: i + 1,
            total,
            elapsed: start_time.elapsed(),
        })?;
    }

    Ok(final_graph)
}
