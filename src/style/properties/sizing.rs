//! Sizing-related style property structures

use super::types::Size;

/// Size constraint style properties
///
/// Contains width, height, and min/max constraints.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SizingStyle {
    /// Width constraint
    pub width: Size,
    /// Height constraint
    pub height: Size,
    /// Minimum width
    pub min_width: Size,
    /// Maximum width
    pub max_width: Size,
    /// Minimum height
    pub min_height: Size,
    /// Maximum height
    pub max_height: Size,
}
