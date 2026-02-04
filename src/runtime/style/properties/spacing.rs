//! Spacing-related style property structures

use super::types::Spacing;

/// Spacing-related style properties
///
/// Contains padding, margin, and position offset properties.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SpacingStyle {
    /// Inner padding
    pub padding: Spacing,
    /// Outer margin
    pub margin: Spacing,
    /// Top offset (for absolute/fixed/relative)
    pub top: Option<i16>,
    /// Right offset
    pub right: Option<i16>,
    /// Bottom offset
    pub bottom: Option<i16>,
    /// Left offset
    pub left: Option<i16>,
}
