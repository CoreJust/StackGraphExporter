use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Instant;

use rayon::prelude::*;
use stack_graphs::graph::StackGraph;
use tree_sitter_stack_graphs::{NoCancellation, StackGraphLanguage, Variables};

use super::ProgressEvent;
use crate::error::{Error, Result};

pub enum Message {
    Event(ProgressEvent),
    Result(Result<Vec<StackGraph>>),
}

pub fn spawn_parallel_build(
    file_paths: Vec<PathBuf>,
    sg_language: StackGraphLanguage,
    cancel_flag: NoCancellation,
    total_files: usize,
    start_time: Instant,
) -> (mpsc::Receiver<Message>, thread::JoinHandle<()>) {
    let (tx, rx) = mpsc::channel();
    let counter = Arc::new(AtomicUsize::new(0));

    let builder_tx = tx.clone();
    let builder_counter = Arc::clone(&counter);
    let builder_handle = thread::spawn(move || {
        let result: Result<Vec<StackGraph>> = file_paths
            .par_iter()
            .map(|path| {
                let processed = builder_counter.load(Ordering::Relaxed);
                let _ = builder_tx.send(Message::Event(ProgressEvent::FileStarted {
                    path: path.clone(),
                    processed,
                    total: total_files,
                    elapsed: start_time.elapsed(),
                }));

                let variables = Variables::new();
                let mut local_graph = StackGraph::new();

                let source = fs::read_to_string(path).map_err(|e| Error::Io {
                    path: path.clone(),
                    source: e,
                })?;

                let file_handle = local_graph.get_or_create_file(path.to_string_lossy().as_ref());
                let builder =
                    sg_language.builder_into_stack_graph(&mut local_graph, file_handle, &source);
                builder.build(&variables, &cancel_flag)?;

                builder_counter.fetch_add(1, Ordering::Relaxed);

                Ok(local_graph)
            })
            .collect();

        let _ = builder_tx.send(Message::Result(result));
    });

    (rx, builder_handle)
}

pub fn process_messages<F>(progress: &mut F, rx: mpsc::Receiver<Message>) -> Result<Vec<StackGraph>>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    loop {
        match rx
            .recv()
            .map_err(|_| Error::Internal("channel receive error".into()))?
        {
            Message::Event(event) => {
                progress(event)?;
            }
            Message::Result(result) => {
                return result;
            }
        }
    }
}
