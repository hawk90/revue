//! Time-Travel Debugging for Revue applications
//!
//! Records state snapshots and allows stepping through history.
//!
//! # Features
//!
//! - Automatic state snapshot recording
//! - Step forward/backward through history
//! - Jump to specific snapshot
//! - Action/event log with timestamps
//! - State diff visualization
//! - Export/import session history
//! - Configurable snapshot limits
//! - Pause/resume recording

mod debugger;
mod types;

pub use debugger::TimeTravelDebugger;
pub use types::{
    Action, SnapshotValue, StateDiff, StateSnapshot, TimeTravelConfig, TimeTravelView,
};

#[cfg(test)]
mod tests;
