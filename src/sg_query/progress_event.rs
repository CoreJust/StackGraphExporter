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
    LookingForSymbolReferences {
        elapsed_and_count: ElapsedAndCount,
        symbol: &'a str,
    },
    LookingForReferences(ElapsedAndCount),
    FoundSymbolReferences {
        elapsed: Duration,
        symbol: &'a str,
        found_refs: usize,
        found_defs: usize,
    },
    FoundReferences {
        elapsed: Duration,
        found_refs: usize,
        found_defs: usize,
    },
    StitchingPaths {
        elapsed: Duration,
    },
    PathsStitched {
        elapsed: Duration,
    },
    ResolvingSymbols {
        elapsed_and_processed: ElapsedAndCount,
        found_resolvable_refs: usize,
        needed_at_most: u32,
    },
    RetryingQueries(ElapsedAndCount),
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
            ProgressEvent::LookingForSymbolReferences { symbol, .. } => {
                write!(f, "Looking for references for symbol '{symbol}'")
            }
            ProgressEvent::LookingForReferences { .. } => {
                write!(f, "Looking for references")
            }
            ProgressEvent::FoundSymbolReferences {
                symbol,
                found_defs,
                found_refs,
                ..
            } => {
                write!(f, "Found {found_refs} references and {found_defs} definitions for symbol '{symbol}'")
            }
            ProgressEvent::FoundReferences {
                found_defs,
                found_refs,
                ..
            } => {
                write!(
                    f,
                    "Found {found_refs} references and {found_defs} definitions",
                )
            }
            ProgressEvent::StitchingPaths { .. } => {
                write!(f, "Stitching paths")
            }
            ProgressEvent::PathsStitched { .. } => {
                write!(f, "Paths stitched successfully")
            }
            ProgressEvent::ResolvingSymbols {
                found_resolvable_refs,
                needed_at_most,
                ..
            } => {
                write!(
                    f,
                    "Resolving symbols ({found_resolvable_refs} resolvable references found / {needed_at_most} needed at most)"
                )
            }
            ProgressEvent::RetryingQueries(_) => {
                write!(f, "Retrying queries for more precise duration")
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
            ProgressEvent::LookingForSymbolReferences {
                elapsed_and_count, ..
            } => ProgressState::from_elapsed_and_count(elapsed_and_count),
            ProgressEvent::LookingForReferences(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count)
            }
            ProgressEvent::FoundSymbolReferences { elapsed, .. } => {
                ProgressState::from_elapsed(&Elapsed { elapsed: *elapsed }, true)
            }
            ProgressEvent::FoundReferences { elapsed, .. } => {
                ProgressState::from_elapsed(&Elapsed { elapsed: *elapsed }, true)
            }
            ProgressEvent::StitchingPaths { elapsed } => {
                ProgressState::from_elapsed(&Elapsed { elapsed: *elapsed }, false)
            }
            ProgressEvent::PathsStitched { elapsed } => {
                ProgressState::from_elapsed(&Elapsed { elapsed: *elapsed }, true)
            }
            ProgressEvent::ResolvingSymbols {
                elapsed_and_processed,
                ..
            } => ProgressState::from_elapsed_and_count(elapsed_and_processed),
            ProgressEvent::RetryingQueries(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count)
            }
        }
    }
}
