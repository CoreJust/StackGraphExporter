use std::fmt;
use std::path::PathBuf;
use std::time::Duration;

use crate::io::{ProgressEvent as IOProgressEvent, ProgressState};

#[derive(Debug, Clone)]
pub enum ProgressEvent {
    FilesFound {
        count: usize,
        elapsed: Duration,
    },
    FileStarted {
        path: PathBuf,
        processed: usize,
        total: usize,
        elapsed: Duration,
    },
    MergeProgress {
        current: usize,
        total: usize,
        elapsed: Duration,
    },
    Done {
        elapsed: Duration,
    },
}

impl fmt::Display for ProgressEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgressEvent::FilesFound { count, .. } => {
                write!(f, "Found {} files", count)
            }
            ProgressEvent::FileStarted { path, .. } => {
                write!(f, "Loading file {}", path.display())
            }
            ProgressEvent::MergeProgress { .. } => {
                write!(f, "Merging graphs")
            }
            ProgressEvent::Done { .. } => {
                write!(f, "Stack graph built successfully")
            }
        }
    }
}

impl IOProgressEvent for ProgressEvent {
    fn state(&self) -> ProgressState {
        match self {
            ProgressEvent::FilesFound { elapsed, .. } => ProgressState {
                is_final: false,
                progress: 1.0,
                elapsed: *elapsed,
                objects_handled: None,
            },
            ProgressEvent::FileStarted {
                processed,
                total,
                elapsed,
                ..
            } => ProgressState {
                is_final: false,
                progress: *processed as f32 / *total as f32,
                elapsed: *elapsed,
                objects_handled: Some((*processed, *total)),
            },
            ProgressEvent::MergeProgress {
                current,
                total,
                elapsed,
            } => ProgressState {
                is_final: false,
                progress: *current as f32 / *total as f32,
                elapsed: *elapsed,
                objects_handled: Some((*current, *total)),
            },
            ProgressEvent::Done { elapsed } => ProgressState {
                is_final: true,
                progress: 1.0,
                elapsed: *elapsed,
                objects_handled: None,
            },
        }
    }
}
