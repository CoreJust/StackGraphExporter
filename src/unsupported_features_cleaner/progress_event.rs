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
    Done {
        removed_imports: usize,
        removed_static_scopes: usize,
        fixed_c_style_arrays: usize,
        removed_comments: usize,
        elapsed: Duration,
    },
}

impl fmt::Display for ProgressEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgressEvent::FilesFound { count, .. } => {
                write!(f, "Found {} Java files for cleaning", count)
            }
            ProgressEvent::FileStarted { path, .. } => {
                write!(f, "Cleaning {}", path.display())
            }
            ProgressEvent::Done {
                removed_imports,
                removed_static_scopes,
                fixed_c_style_arrays,
                removed_comments,
                ..
            } => {
                write!(f, "Unsupported features cleaned: {removed_imports} unsupported imports, {removed_static_scopes} static scopes, {removed_comments} comments, {fixed_c_style_arrays} arrays")
            }
        }
    }
}

impl IOProgressEvent for ProgressEvent {
    fn state(&self) -> ProgressState {
        match self {
            ProgressEvent::FilesFound { elapsed, .. } => ProgressState {
                is_final: false,
                progress: 0.0,
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
            ProgressEvent::Done { elapsed, .. } => ProgressState {
                is_final: true,
                progress: 1.0,
                elapsed: *elapsed,
                objects_handled: None,
            },
        }
    }
}
