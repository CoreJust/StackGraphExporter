use std::fmt;
use std::time::Duration;

use crate::io::{Elapsed, ElapsedAndCount, ProgressEvent as IOProgressEvent, ProgressState};

pub enum ProgressEvent {
    GeneratingArtifact {
        elapsed: Duration,
        progress: Option<(usize, usize)>,
        message: String,
    },
    WritingLines {
        elapsed_and_count: ElapsedAndCount,
        artifact_name: &'static str,
    },
    ArtifactStored {
        elapsed_and_count: ElapsedAndCount,
        artifact_name: &'static str,
    },
}

impl fmt::Display for ProgressEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgressEvent::GeneratingArtifact { message, .. } => {
                write!(f, "{message}")
            }
            ProgressEvent::WritingLines { artifact_name, .. } => {
                write!(f, "Writing lines of {artifact_name}")
            }
            ProgressEvent::ArtifactStored { artifact_name, .. } => {
                write!(f, "Stored artifact {artifact_name}")
            }
        }
    }
}

impl IOProgressEvent for ProgressEvent {
    fn state(&self) -> crate::io::ProgressState {
        match self {
            ProgressEvent::GeneratingArtifact {
                elapsed, progress, ..
            } => {
                if let Some(progress) = progress {
                    ProgressState::from_elapsed_and_count(
                        &ElapsedAndCount {
                            current: progress.0,
                            total: progress.1,
                            elapsed: *elapsed,
                        },
                        false,
                    )
                } else {
                    ProgressState::from_elapsed(&Elapsed { elapsed: *elapsed }, false)
                }
            }
            ProgressEvent::WritingLines {
                elapsed_and_count, ..
            } => ProgressState::from_elapsed_and_count(elapsed_and_count, false),
            ProgressEvent::ArtifactStored {
                elapsed_and_count, ..
            } => ProgressState::from_elapsed_and_count(elapsed_and_count, true),
        }
    }
}
