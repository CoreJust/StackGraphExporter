use std::fmt;
use std::time::Duration;

use crate::io::ProgressEvent as IOProgressEvent;

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
            ProgressEvent::SerializingStackGraph { elapsed } => {
                write!(f, "[{} ms] Serializing stack graph", elapsed.as_millis())
            }
            ProgressEvent::ParsingNodes {
                current,
                total,
                elapsed,
            } => {
                write!(
                    f,
                    "[{} ms] Parsing nodes... {}/{}",
                    elapsed.as_millis(),
                    current,
                    total,
                )
            }
            ProgressEvent::ParsingEdges {
                current,
                total,
                elapsed,
            } => {
                write!(
                    f,
                    "[{} ms] Parsing edges... {}/{}",
                    elapsed.as_millis(),
                    current,
                    total,
                )
            }
            ProgressEvent::ResolvingScopes { elapsed } => {
                write!(f, "[{} ms] Resolving scopes", elapsed.as_millis())
            }
            ProgressEvent::GraphBuilt { elapsed } => {
                write!(f, "[{} ms] SGGraph built successfully", elapsed.as_millis())
            }
        }
    }
}

impl IOProgressEvent for ProgressEvent {
    fn is_final_state(&self) -> bool {
        match self {
            ProgressEvent::GraphBuilt { .. } => true,
            _ => false,
        }
    }
}
