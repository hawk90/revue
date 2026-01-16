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
//! - **Image protocols**: Support for Kitty, iTerm2, and Sixel graphics

pub mod backend;
mod batch;
mod buffer;
mod cell;
mod diff;
pub mod image_protocol;
mod terminal;

pub use backend::{Backend, BackendCapabilities, CrosstermBackend};
pub use batch::{BatchStats, RenderBatch, RenderOp};
pub use buffer::Buffer;
pub use cell::{Cell, Modifier};
pub use diff::{diff, Change};
pub use image_protocol::{
    GraphicsCapabilities, ImageEncoder, ImageProtocol, Iterm2Image, KittyImage, PixelFormat,
    SixelEncoder,
};
pub use terminal::{stdout_terminal, Terminal};
