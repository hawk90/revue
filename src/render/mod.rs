//! Terminal rendering system for TUI applications
//!
//! Double buffering and efficient diff-based updates for optimal performance.
//!
//! # Architecture
//!
//! The rendering system is composed of:
//!
//! | Component | Description | Module |
//! |-----------|-------------|--------|
//! | **Backend** | Low-level terminal I/O abstraction | [`backend`] |
//! | **Buffer** | Double-buffered screen state | [`buffer`] |
//! | **Cell** | Individual terminal cell (char, colors, modifiers) | [`cell`] |
//! | **Terminal** | High-level diff-based renderer | [`terminal`] |
//! | **Diff** | Efficient buffer diffing algorithm | [`diff`] |
//! | **Images** | Kitty, iTerm2, and Sixel graphics | [`image_protocol`] |
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use revue::render::{Terminal, Buffer};
//!
//! // Create terminal
//! let mut terminal = Terminal::new()?;
//!
//! // Create buffer matching terminal size
//! let mut buffer = Buffer::new(terminal.size()?);
//!
//! // Draw to buffer
//! buffer.set_string(0, 0, "Hello, World!", style);
//!
//! // Render with diffing
//! terminal.draw(&buffer)?;
//! ```
//!
//! # Double Buffering
//!
//! Revue uses double buffering for efficient rendering:
//!
//! 1. Two buffers of the same size are allocated
//! 2. Each frame renders to the "back" buffer
//! 3. Buffers are compared to find minimal changes
//! 4. Only changed cells are written to the terminal
//! 5. Buffers are swapped for the next frame
//!
//! # Cell Rendering
//!
//! ```rust,ignore
//! use revue::render::{Cell, Modifier};
//!
//! // Create a styled cell
//! let cell = Cell::new('A')
//!     .fg(Color::Blue)
//!     .bg(Color::Black)
//!     .modifier(Modifier::BOLD);
//! ```
//!
//! # Image Support
//!
//! ```rust,ignore
//! use revue::render::image_protocol::KittyImage;
//!
//! // Detect image support
//! if let Some(protocol) = KittyImage::detect() {
//!     // Render image
//!     protocol.draw_image(x, y, &image_data)?;
//! }
//! ```
//!
//! # Performance
//!
//! - Diff-based updates minimize terminal writes
//! - Batch operations group multiple cell updates
//! - ANSI escape sequences are cached
//! - Terminal capabilities are detected once

pub mod backend;
mod batch;
mod buffer;
mod cell;
mod diff;
#[cfg(feature = "image")]
pub mod image_protocol;
mod terminal;

pub use backend::{Backend, BackendCapabilities, CrosstermBackend};
pub use batch::{BatchStats, RenderBatch, RenderOp};
pub use buffer::{Buffer, BufferError};
pub use cell::{Cell, Modifier};
pub use diff::{diff, Change};
#[cfg(feature = "image")]
pub use image_protocol::{
    GraphicsCapabilities, ImageEncoder, ImageProtocol, Iterm2Image, KittyImage, PixelFormat,
    SixelEncoder,
};
pub use terminal::{stdout_terminal, Terminal};
