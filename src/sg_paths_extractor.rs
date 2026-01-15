use stack_graphs::{
    graph::StackGraph,
    partial::PartialPaths,
    stitching::{Database, DatabaseCandidates, ForwardPartialPathStitcher, StitcherConfig},
    NoCancellation,
};

use crate::from_serde::{get_object, get_str, get_u64};

fn make_populated_database_from<'a>(
    graph: &'a StackGraph,
    db: &'a mut Database,
    partials: &'a mut PartialPaths,
) -> anyhow::Result<DatabaseCandidates<'a>> {
    for file in graph.iter_files() {
        let stitcher_config = StitcherConfig::default()
            .with_detect_similar_paths(true)
            .with_collect_stats(false);
        ForwardPartialPathStitcher::find_minimal_partial_path_set_in_file(
            &graph,
            partials,
            file,
            stitcher_config,
            &NoCancellation,
            |_g, ps, p| {
                db.add_partial_path(&graph, ps, p.clone());
            },
        )?;
    }
    Ok(DatabaseCandidates::new(&graph, partials, db))
}

pub struct UnresolvedNode<'a> {
    pub(crate) file: Option<&'a str>,
    pub(crate) id: usize,
}

impl<'a> UnresolvedNode<'a> {
    fn from_object<'b>(
        value: &'b serde_json::Map<String, serde_json::Value>,
    ) -> anyhow::Result<UnresolvedNode<'b>> {
        Ok(UnresolvedNode {
            file: get_str(value, "file").ok(),
            id: get_u64(value, "local_id")? as usize,
        })
    }
}

pub fn extract_complete_paths_from<F: FnMut(UnresolvedNode, UnresolvedNode)>(
    graph: &StackGraph,
    mut func: F,
) -> anyhow::Result<()> {
    let mut db = Database::new();
    let mut partials = PartialPaths::new();
    let mut db = make_populated_database_from(graph, &mut db, &mut partials)?;
    let starting_nodes = graph
        .iter_nodes()
        .filter(|n| graph[*n].is_reference())
        .collect::<Vec<_>>();
    let stitcher_config = StitcherConfig::default().with_detect_similar_paths(true);
    let mut complete_paths_db = Database::new();
    ForwardPartialPathStitcher::find_all_complete_partial_paths(
        &mut db,
        starting_nodes,
        stitcher_config,
        &NoCancellation,
        |g, ps, p| {
            complete_paths_db.add_partial_path(g, ps, p.clone());
        },
    )?;

    let paths = complete_paths_db.to_serializable(&graph, &mut partials);
    let paths = serde_json::to_value(paths)?;
    let paths = paths
        .as_array()
        .ok_or(anyhow::Error::msg(format!("Expected paths to be array")))?;
    for path in paths {
        let path = path
            .as_object()
            .ok_or(anyhow::Error::msg(format!("Expected a path to be object")))?;
        let start_node = get_object(path, "start_node")?;
        let end_node = get_object(path, "end_node")?;
        func(
            UnresolvedNode::from_object(start_node)?,
            UnresolvedNode::from_object(end_node)?,
        );
    }
    Ok(())
}
