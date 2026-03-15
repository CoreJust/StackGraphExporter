use serde_json::{Map, Value};
use stack_graphs::graph::StackGraph;
use stack_graphs::partial::PartialPaths;
use stack_graphs::stitching::{
    Database, DatabaseCandidates, ForwardPartialPathStitcher, StitcherConfig,
};
use stack_graphs::NoCancellation;

use crate::error::{Error, Result};

/// A node identifier as extracted from the stack graph stitching results,
/// containing a file path (if any) and a local node ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawNode {
    pub file: Option<String>,
    pub id: usize,
}

impl RawNode {
    fn from_object(obj: &Map<String, Value>) -> Result<Self> {
        let file = obj.get("file").and_then(|v| v.as_str()).map(String::from);
        let id = obj
            .get("local_id")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| Error::PathExtraction("missing or invalid 'local_id'".to_string()))?
            as usize;
        Ok(RawNode { file, id })
    }
}

/// Extract all complete reference-definition paths from the stack graph.
/// Calls the provided closure for each found path, passing the start and end nodes.
pub fn extract_paths<F>(graph: &StackGraph, mut callback: F) -> Result<()>
where
    F: FnMut(RawNode, RawNode),
{
    let mut db = Database::new();
    let mut partials = PartialPaths::new();

    let mut db_candidates = populate_database(graph, &mut db, &mut partials)?;

    let starting_nodes: Vec<_> = graph
        .iter_nodes()
        .filter(|n| graph[*n].is_reference())
        .collect();

    let mut complete_paths_db = Database::new();
    let stitcher_config = StitcherConfig::default().with_detect_similar_paths(true);
    ForwardPartialPathStitcher::find_all_complete_partial_paths(
        &mut db_candidates,
        starting_nodes,
        stitcher_config,
        &NoCancellation,
        |g, ps, p| {
            complete_paths_db.add_partial_path(g, ps, p.clone());
        },
    )
    .map_err(|e| Error::PathExtraction(format!("Failed to find complete paths: {}", e)))?;

    let paths_serializable = complete_paths_db.to_serializable(graph, &mut partials);
    let paths_json = serde_json::to_value(paths_serializable)
        .map_err(|e| Error::PathExtraction(format!("Failed to serialize paths: {}", e)))?;

    let paths_array = paths_json
        .as_array()
        .ok_or_else(|| Error::PathExtraction("Expected paths to be an array".to_string()))?;

    for path_value in paths_array {
        let path_obj = path_value
            .as_object()
            .ok_or_else(|| Error::PathExtraction("Expected path to be an object".to_string()))?;

        let start_obj = path_obj
            .get("start_node")
            .and_then(|v| v.as_object())
            .ok_or_else(|| Error::PathExtraction("Missing or invalid start_node".to_string()))?;
        let end_obj = path_obj
            .get("end_node")
            .and_then(|v| v.as_object())
            .ok_or_else(|| Error::PathExtraction("Missing or invalid end_node".to_string()))?;

        let start = RawNode::from_object(start_obj)?;
        let end = RawNode::from_object(end_obj)?;
        callback(start, end);
    }

    Ok(())
}

fn populate_database<'a>(
    graph: &'a StackGraph,
    db: &'a mut Database,
    partials: &'a mut PartialPaths,
) -> Result<DatabaseCandidates<'a>> {
    let stitcher_config = StitcherConfig::default()
        .with_detect_similar_paths(true)
        .with_collect_stats(false);

    for file in graph.iter_files() {
        ForwardPartialPathStitcher::find_minimal_partial_path_set_in_file(
            graph,
            partials,
            file,
            stitcher_config,
            &NoCancellation,
            |_g, ps, p| {
                db.add_partial_path(graph, ps, p.clone());
            },
        )
        .map_err(|e| {
            Error::PathExtraction(format!("Failed to populate database for file: {}", e))
        })?;
    }

    Ok(DatabaseCandidates::new(graph, partials, db))
}
