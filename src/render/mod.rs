//! Terminal rendering
//!
//! Double buffering and efficient diff-based updates.
//!
//! # Architecture
//!
//! The rendering system is composed of:
//!
//! - **Backend**: Low-level terminal I/O abstraction (`backend` module)
//! - **Buffer**: Double-buffered screen state
//! - **Cell**: Individual terminal cell with character, colors, and modifiers
//! - **Terminal**: High-level renderer that uses diff-based updates

pub mod backend;
mod buffer;
mod cell;
mod diff;
mod terminal;
mod batch;

pub use backend::{Backend, BackendCapabilities, CrosstermBackend};
pub use buffer::Buffer;
pub use cell::{Cell, Modifier};
pub use diff::{diff, Change};
pub use terminal::{Terminal, stdout_terminal};
pub use batch::{RenderBatch, RenderOp, BatchStats};
