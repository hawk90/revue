//! Screen reader backend implementations
//!
//! Platform-specific backends for screen reader communication.

mod core;
mod detection;
mod global;
mod platform;
mod types;

pub use types::{BackendType, ScreenReader, ScreenReaderConfig};

// Re-export core backend
pub use core::ScreenReaderBackend;

// Re-export platform-specific types
pub use platform::LoggingBackend;

// Re-export global functions
pub use global::{announce_to_screen_reader, get_backend, init_backend, set_backend};

#[cfg(test)]
mod tests {
    include!("tests.rs");
}
