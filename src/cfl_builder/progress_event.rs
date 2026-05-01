use std::fmt;
use std::time::{Duration, Instant};

use crate::error::Result;
use crate::io::{Elapsed, ElapsedAndCount, ProgressEvent as IOProgressEvent, ProgressState};

const PROGRESS_ONCE_IN: usize = 128;

pub enum ProgressEvent {
    BuildingSymbolRules(ElapsedAndCount),
    BuildingTransientNodes(ElapsedAndCount),
    PopulatingTransientNodes(ElapsedAndCount),
    SimplifyingTransientGraph {
        substage: &'static str,
        iteration: usize,
        elapsed_and_count: ElapsedAndCount,
    },
    TransientGraphBuilt(Elapsed),
    BuildingOutIds(ElapsedAndCount),
    BuildingForCurrentEdges(ElapsedAndCount),
    BuildingSymbolEdges(ElapsedAndCount),
    BuildingNodeMetadata(ElapsedAndCount),
    BuildingPopNodesMapping(ElapsedAndCount),
    SimplifyingGraph {
        substage: &'static str,
        elapsed: Duration,
    },
    Done(Elapsed),
}

pub struct ProgressMonitor<F>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    cb: F,
    pub start: Instant,
    pub simplification_iteration: usize, // Only for SimplifyingTransientGraph
    pub stage_total: usize,
}

impl<F> ProgressMonitor<F>
where
    F: FnMut(ProgressEvent) -> Result<()>,
{
    pub fn new(cb: F) -> Self {
        Self {
            cb,
            start: Instant::now(),
            simplification_iteration: 0,
            stage_total: 0,
        }
    }

    pub fn emit_simplification_nth(&mut self, substage: &'static str, i: usize) -> Result<()> {
        if i % PROGRESS_ONCE_IN == 0 {
            (self.cb)(ProgressEvent::SimplifyingTransientGraph {
                substage,
                iteration: self.simplification_iteration,
                elapsed_and_count: ElapsedAndCount {
                    current: i,
                    total: self.stage_total,
                    elapsed: self.start.elapsed(),
                },
            })
        } else {
            Ok(())
        }
    }

    pub fn emit_nth<CB>(&mut self, i: usize, mut make_progress_event: CB) -> Result<()>
    where
        CB: FnMut(ElapsedAndCount) -> ProgressEvent,
    {
        if i % PROGRESS_ONCE_IN == 0 {
            (self.cb)(make_progress_event(ElapsedAndCount {
                current: i,
                total: self.stage_total,
                elapsed: self.start.elapsed(),
            }))
        } else {
            Ok(())
        }
    }

    pub fn emit<CB>(&mut self, mut make_progress_event: CB) -> Result<()>
    where
        CB: FnMut(Elapsed) -> ProgressEvent,
    {
        (self.cb)(make_progress_event(Elapsed {
            elapsed: self.start.elapsed(),
        }))
    }
}

impl fmt::Display for ProgressEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgressEvent::BuildingSymbolRules { .. } => {
                write!(f, "Building symbol rules")
            }
            ProgressEvent::BuildingTransientNodes { .. } => {
                write!(f, "Building transient nodes")
            }
            ProgressEvent::PopulatingTransientNodes { .. } => {
                write!(f, "Populating transient nodes")
            }
            ProgressEvent::SimplifyingTransientGraph {
                substage,
                iteration,
                ..
            } => {
                write!(
                    f,
                    "Simplifying transient graph (iteration {iteration}): {substage}"
                )
            }
            ProgressEvent::TransientGraphBuilt { .. } => {
                write!(f, "Transient graph built successfully")
            }
            ProgressEvent::BuildingOutIds { .. } => {
                write!(f, "Building out node IDs")
            }
            ProgressEvent::BuildingForCurrentEdges { .. } => {
                write!(f, "Building CFL edges for existing SG edges")
            }
            ProgressEvent::BuildingSymbolEdges { .. } => {
                write!(f, "Building CFL edges for symbols")
            }
            ProgressEvent::BuildingNodeMetadata { .. } => {
                write!(f, "Building metadata for nodes")
            }
            ProgressEvent::BuildingPopNodesMapping { .. } => {
                write!(f, "Building mapping from SG pop nodes")
            }
            ProgressEvent::SimplifyingGraph { substage, .. } => {
                write!(f, "Simplifying CFL graph: {substage}")
            }
            ProgressEvent::Done { .. } => {
                write!(f, "CFL graph built successfully")
            }
        }
    }
}

impl IOProgressEvent for ProgressEvent {
    fn state(&self) -> crate::io::ProgressState {
        match self {
            ProgressEvent::BuildingSymbolRules(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count, false)
            }
            ProgressEvent::BuildingTransientNodes(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count, false)
            }
            ProgressEvent::PopulatingTransientNodes(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count, false)
            }
            ProgressEvent::SimplifyingTransientGraph {
                elapsed_and_count, ..
            } => ProgressState::from_elapsed_and_count(elapsed_and_count, false),
            ProgressEvent::TransientGraphBuilt(elapsed) => {
                ProgressState::from_elapsed(elapsed, true)
            }
            ProgressEvent::BuildingOutIds(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count, false)
            }
            ProgressEvent::BuildingForCurrentEdges(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count, false)
            }
            ProgressEvent::BuildingSymbolEdges(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count, false)
            }
            ProgressEvent::BuildingNodeMetadata(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count, false)
            }
            ProgressEvent::BuildingPopNodesMapping(elapsed_and_count) => {
                ProgressState::from_elapsed_and_count(elapsed_and_count, false)
            }
            ProgressEvent::SimplifyingGraph { elapsed, .. } => ProgressState {
                is_final: false,
                elapsed: *elapsed,
                progress: 0.0,
                objects_handled: None,
            },
            ProgressEvent::Done(elapsed) => ProgressState::from_elapsed(elapsed, true),
        }
    }
}
