use std::fmt;
use std::time::Duration;

use crate::io::{ProgressEvent as IOProgressEvent, ProgressState};

pub enum ProgressEvent {
    SerializingStackGraph {
        elapsed: Duration,
    },
    ParsingNodes {
        current: usize,
        total: usize,
        elapsed: Duration,
    },
    ParsingEdges {
        current: usize,
        total: usize,
        elapsed: Duration,
    },
    ResolvingScopes {
        elapsed: Duration,
    },
    GraphBuilt {
        elapsed: Duration,
    },
}

impl fmt::Display for ProgressEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgressEvent::SerializingStackGraph { .. } => {
                write!(f, "Serializing stack graph")
            }
            ProgressEvent::ParsingNodes { .. } => {
                write!(f, "Parsing nodes")
            }
            ProgressEvent::ParsingEdges { .. } => {
                write!(f, "Parsing edges")
            }
            ProgressEvent::ResolvingScopes { .. } => {
                write!(f, "Resolving scopes")
            }
            ProgressEvent::GraphBuilt { .. } => {
                write!(f, "SGGraph built successfully")
            }
        }
    }
}

impl IOProgressEvent for ProgressEvent {
    fn state(&self) -> crate::io::ProgressState {
        match self {
            ProgressEvent::SerializingStackGraph { elapsed } => ProgressState {
                is_final: false,
                elapsed: *elapsed,
                progress: 0.0,
                objects_handled: None,
            },
            ProgressEvent::ParsingNodes {
                current,
                total,
                elapsed,
            } => ProgressState {
                is_final: false,
                elapsed: *elapsed,
                progress: *current as f32 / *total as f32,
                objects_handled: Some((*current, *total)),
            },
            ProgressEvent::ParsingEdges {
                current,
                total,
                elapsed,
            } => ProgressState {
                is_final: false,
                elapsed: *elapsed,
                progress: *current as f32 / *total as f32,
                objects_handled: Some((*current, *total)),
            },
            ProgressEvent::ResolvingScopes { elapsed } => ProgressState {
                is_final: false,
                elapsed: *elapsed,
                progress: 0.0,
                objects_handled: None,
            },
            ProgressEvent::GraphBuilt { elapsed } => ProgressState {
                is_final: true,
                elapsed: *elapsed,
                progress: 1.0,
                objects_handled: None,
            },
        }
    }
}
