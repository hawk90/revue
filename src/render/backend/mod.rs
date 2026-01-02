//! Terminal backend abstraction
//!
//! This module provides a trait-based abstraction for terminal backends,
//! allowing the framework to support multiple terminal libraries.
//!
//! # Available Backends
//!
//! - `CrosstermBackend` - Cross-platform backend using crossterm
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::render::backend::{Backend, CrosstermBackend};
//!
//! let backend = CrosstermBackend::new(std::io::stdout())?;
//! let mut terminal = Terminal::with_backend(backend)?;
//! ```

mod traits;
mod crossterm;

pub use traits::{Backend, BackendCapabilities};
pub use self::crossterm::CrosstermBackend;
