use std::path::Path;
use std::time::Instant;

use stack_graphs::graph::StackGraph;
use tree_sitter_stack_graphs::NoCancellation;

use super::Language;
use crate::error::{Error, Result};
use crate::loading::progress_event::ProgressEvent;

use crate::loading::build::spawn_parallel_build;
use crate::loading::discovery::discover_files;
use crate::loading::merge::merge_graphs_with_progress;

pub fn load_stack_graph<F>(
    project_dir: &Path,
    language: Language,
    mut progress: F,
) -> Result<StackGraph>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    if !project_dir.is_dir() {
        return Err(Error::InvalidArgument(format!(
            "Path is not a directory: {}",
            project_dir.display(),
        )));
    }

    let start_time = Instant::now();
    let sg_language = language.build_stack_graph_language()?;
    let extensions = language.file_extensions();

    let file_paths = discover_files(project_dir, &extensions)?;
    let total_files = file_paths.len();

    progress(ProgressEvent::FilesFound {
        count: total_files,
        elapsed: start_time.elapsed(),
    })?;

    if total_files == 0 {
        progress(ProgressEvent::Done {
            elapsed: start_time.elapsed(),
        })?;
        return Ok(StackGraph::new());
    }

    let (rx, builder_handle) = spawn_parallel_build(
        file_paths,
        sg_language,
        NoCancellation,
        total_files,
        start_time,
    );

    let local_graphs = crate::loading::build::process_messages(&mut progress, rx)?;
    builder_handle
        .join()
        .map_err(|_| Error::Internal("Builder thread panicked".into()))?;

    let final_graph = merge_graphs_with_progress(local_graphs, start_time, &mut progress)?;
    progress(ProgressEvent::Done {
        elapsed: start_time.elapsed(),
    })?;

    Ok(final_graph)
}
