use std::fmt;
use std::time::Duration;

use crate::io::{ProgressEvent as IOProgressEvent, ProgressState};

pub enum ProgressEvent {
    PreparingQueryGrammar { elapsed: Duration },
    RunningKotgll { elapsed: Duration },
    ModifyingDot { elapsed: Duration },
    ParsingOutput { elapsed: Duration },
    KotGllDone { elapsed: Duration },
    UcfsDone { elapsed: Duration },
}

impl fmt::Display for ProgressEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgressEvent::PreparingQueryGrammar { .. } => {
                write!(f, "Preparing query grammar")
            }
            ProgressEvent::RunningKotgll { .. } => {
                write!(f, "Running KotGLL")
            }
            ProgressEvent::ModifyingDot { .. } => {
                write!(f, "Modifying DOT file")
            }
            ProgressEvent::ParsingOutput { .. } => {
                write!(f, "Parsing KotGLL output")
            }
            ProgressEvent::KotGllDone { .. } => {
                write!(f, "KotGLL query done")
            }
            ProgressEvent::UcfsDone { .. } => {
                write!(f, "UCFS query grammar generated")
            }
        }
    }
}

impl IOProgressEvent for ProgressEvent {
    fn state(&self) -> crate::io::ProgressState {
        match self {
            ProgressEvent::PreparingQueryGrammar { elapsed } => ProgressState {
                is_final: false,
                elapsed: *elapsed,
                progress: 0.0,
                objects_handled: None,
            },
            ProgressEvent::RunningKotgll { elapsed } => ProgressState {
                is_final: false,
                elapsed: *elapsed,
                progress: 0.25,
                objects_handled: None,
            },
            ProgressEvent::ModifyingDot { elapsed } => ProgressState {
                is_final: false,
                elapsed: *elapsed,
                progress: 0.5,
                objects_handled: None,
            },
            ProgressEvent::ParsingOutput { elapsed } => ProgressState {
                is_final: false,
                elapsed: *elapsed,
                progress: 0.75,
                objects_handled: None,
            },
            ProgressEvent::KotGllDone { elapsed } => ProgressState {
                is_final: true,
                elapsed: *elapsed,
                progress: 1.0,
                objects_handled: None,
            },
            ProgressEvent::UcfsDone { elapsed } => ProgressState {
                is_final: true,
                elapsed: *elapsed,
                progress: 1.0,
                objects_handled: None,
            },
        }
    }
}
