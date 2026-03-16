use std::fmt;
use std::time::Duration;

use crate::io::ProgressEvent as IOProgressEvent;

pub enum ProgressEvent {
    BuildingDatabase { elapsed: Duration },
    DatabaseBuilt { elapsed: Duration },
    StitchingPaths { elapsed: Duration },
    PathsStitched { elapsed: Duration },
}

impl fmt::Display for ProgressEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgressEvent::BuildingDatabase { elapsed } => {
                write!(
                    f,
                    "[{} ms] Building database of partial paths",
                    elapsed.as_millis()
                )
            }
            ProgressEvent::DatabaseBuilt { elapsed } => {
                write!(
                    f,
                    "[{} ms] Database built successfully",
                    elapsed.as_millis()
                )
            }
            ProgressEvent::StitchingPaths { elapsed } => {
                write!(f, "[{} ms] Stitching paths", elapsed.as_millis())
            }
            ProgressEvent::PathsStitched { elapsed } => {
                write!(
                    f,
                    "[{} ms] Paths stitched successfully",
                    elapsed.as_millis()
                )
            }
        }
    }
}

impl IOProgressEvent for ProgressEvent {
    fn is_final_state(&self) -> bool {
        match self {
            ProgressEvent::DatabaseBuilt { .. } => true,
            ProgressEvent::PathsStitched { .. } => true,
            _ => false,
        }
    }
}
