use std::fmt;
use std::time::Duration;

use crate::io::{ProgressEvent as IOProgressEvent, ProgressState};

pub enum ProgressEvent {
    BuildingDatabase { elapsed: Duration },
    DatabaseBuilt { elapsed: Duration },
    StitchingPaths { elapsed: Duration },
    PathsStitched { elapsed: Duration },
}

impl fmt::Display for ProgressEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgressEvent::BuildingDatabase { .. } => {
                write!(f, "Building database of partial paths")
            }
            ProgressEvent::DatabaseBuilt { .. } => {
                write!(f, "Database built successfully")
            }
            ProgressEvent::StitchingPaths { .. } => {
                write!(f, "Stitching paths")
            }
            ProgressEvent::PathsStitched { .. } => {
                write!(f, "Paths stitched successfully")
            }
        }
    }
}

impl IOProgressEvent for ProgressEvent {
    fn state(&self) -> ProgressState {
        match self {
            ProgressEvent::BuildingDatabase { elapsed } => ProgressState {
                is_final: false,
                elapsed: *elapsed,
                progress: 0.0,
                objects_handled: None,
            },
            ProgressEvent::DatabaseBuilt { elapsed } => ProgressState {
                is_final: true,
                elapsed: *elapsed,
                progress: 1.0,
                objects_handled: None,
            },
            ProgressEvent::StitchingPaths { elapsed } => ProgressState {
                is_final: false,
                elapsed: *elapsed,
                progress: 0.0,
                objects_handled: None,
            },
            ProgressEvent::PathsStitched { elapsed } => ProgressState {
                is_final: true,
                elapsed: *elapsed,
                progress: 1.0,
                objects_handled: None,
            },
        }
    }
}
