//! Core types and enums for lazy loading patterns

/// Loading state for lazy data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadState {
    /// Not yet loaded
    Idle,
    /// Currently loading
    Loading,
    /// Successfully loaded
    Loaded,
    /// Loading failed
    Failed,
}
