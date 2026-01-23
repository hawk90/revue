//! Gesture recognition module
//!
//! Provides gesture recognition for mouse interactions in terminal applications.

/// Gesture data types and callbacks
pub mod data;

/// Gesture recognizer implementation
pub mod recognizer;

/// Gesture recognition tests
pub mod tests;

/// Gesture type definitions
pub mod types;

pub use data::*;
pub use recognizer::*;
pub use types::*;
