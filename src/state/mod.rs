//! State management for Revue
//!
//! This module contains state management systems:
//! - **reactive**: Reactive signals and computed values
//! - **patterns**: Common state management patterns
//! - **tasks**: Async task management
//! - **plugin**: Plugin system
//! - **worker**: Background worker threads

pub mod patterns;
pub mod plugin;
pub mod reactive;
pub mod tasks;
pub mod worker;
