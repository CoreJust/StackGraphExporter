use std::fmt;
use std::path::PathBuf;
use std::time::Duration;

use crate::io::ProgressEvent as IOProgressEvent;

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
            ProgressEvent::FilesFound { count, elapsed } => {
                write!(f, "[{} ms] Found {} files", elapsed.as_millis(), count)
            }
            ProgressEvent::FileStarted {
                path,
                processed,
                total,
                elapsed,
            } => {
                write!(
                    f,
                    "[{} ms] [{}/{}] Loading {}",
                    elapsed.as_millis(),
                    processed,
                    total,
                    path.display(),
                )
            }
            ProgressEvent::MergeProgress {
                current,
                total,
                elapsed,
            } => {
                let percent = (*current as f64 / *total as f64) * 100.0;
                write!(
                    f,
                    "[{} ms] Merging {}/{} graphs ({:.1}%)",
                    elapsed.as_millis(),
                    current,
                    total,
                    percent,
                )
            }
            ProgressEvent::Done { elapsed } => {
                write!(
                    f,
                    "[{} ms] Stack graph built successfully",
                    elapsed.as_millis(),
                )
            }
        }
    }
}

impl IOProgressEvent for ProgressEvent {
    fn is_final_state(&self) -> bool {
        match self {
            ProgressEvent::Done { .. } => true,
            _ => false,
        }
    }
}
