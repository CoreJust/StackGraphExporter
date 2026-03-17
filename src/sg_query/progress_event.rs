use std::fmt;
use std::time::Duration;

use crate::io::{Elapsed, ElapsedAndCount, ProgressEvent as IOProgressEvent, ProgressState};

pub enum ProgressEvent<'a> {
    BuildingDatabase(ElapsedAndCount),
    DatabaseBuilt {
        elapsed: Duration,
    },
    FindingPartialStarts(ElapsedAndCount),
    BuildingNodeIdToPositionIndex(ElapsedAndCount),
    BuildingNodeHandleToPositionIndex(ElapsedAndCount),
    CollectingNodesAtPartialStarts(ElapsedAndCount),
    NodesAtPartialStartsIndexed {
        elapsed: Duration,
    },
    LookingForReferences {
        elapsed_and_count: ElapsedAndCount,
        symbol: &'a str,
    },
    FoundReferences {
        elapsed: Duration,
        symbol: &'a str,
        found_refs: usize,
        found_defs: usize,
    },
    StitchingPaths {
        elapsed: Duration,
    },
    PathsStitched {
        elapsed: Duration,
    },
}

impl<'a> fmt::Display for ProgressEvent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgressEvent::BuildingDatabase { .. } => {
                write!(f, "Building database of partial paths")
            }
            ProgressEvent::DatabaseBuilt { .. } => {
                write!(f, "Database built successfully")
            }
            ProgressEvent::FindingPartialStarts(_) => {
                write!(f, "Looking for partial path starts")
            }
            ProgressEvent::BuildingNodeIdToPositionIndex(_) => {
                write!(f, "Building node id -> node index")
            }
            ProgressEvent::BuildingNodeHandleToPositionIndex(_) => {
                write!(f, "Building node handle -> node index")
            }
            ProgressEvent::CollectingNodesAtPartialStarts(_) => {
                write!(f, "Collecting nodes at partial path start")
            }
            ProgressEvent::NodesAtPartialStartsIndexed { .. } => {
                write!(f, "Indexed nodes at partial path start")
            }
            ProgressEvent::LookingForReferences { symbol, .. } => {
                write!(f, "Looking for references for symbol '{symbol}'")
            }
            ProgressEvent::FoundReferences {
                symbol,
                found_defs,
                found_refs,
                ..
            } => {
                write!(f, "Found for {found_refs} references and {found_defs} definitions for symbol '{symbol}'")
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

impl<'a> IOProgressEvent for ProgressEvent<'a> {
    fn state(&self) -> ProgressState {
        match self {
            ProgressEvent::BuildingDatabase(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count)
            }
            ProgressEvent::DatabaseBuilt { elapsed } => {
                ProgressState::from_elapsed(&Elapsed { elapsed: *elapsed }, true)
            }
            ProgressEvent::FindingPartialStarts(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count)
            }
            ProgressEvent::BuildingNodeIdToPositionIndex(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count)
            }
            ProgressEvent::BuildingNodeHandleToPositionIndex(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count)
            }
            ProgressEvent::CollectingNodesAtPartialStarts(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count)
            }
            ProgressEvent::NodesAtPartialStartsIndexed { elapsed } => {
                ProgressState::from_elapsed(&Elapsed { elapsed: *elapsed }, true)
            }
            ProgressEvent::LookingForReferences {
                elapsed_and_count, ..
            } => ProgressState::from_elapsed_and_count(elapsed_and_count),
            ProgressEvent::FoundReferences { elapsed, .. } => {
                ProgressState::from_elapsed(&Elapsed { elapsed: *elapsed }, true)
            }
            ProgressEvent::StitchingPaths { elapsed } => {
                ProgressState::from_elapsed(&Elapsed { elapsed: *elapsed }, false)
            }
            ProgressEvent::PathsStitched { elapsed } => {
                ProgressState::from_elapsed(&Elapsed { elapsed: *elapsed }, true)
            }
        }
    }
}
