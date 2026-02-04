//! Visual-related style property structures

use super::types::{BorderStyle, Color};

/// Visual style properties
///
/// Contains colors, border, opacity, and visibility properties.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VisualStyle {
    /// Border style
    pub border_style: BorderStyle,
    /// Border color
    pub border_color: Color,
    /// Text/foreground color (INHERITED)
    pub color: Color,
    /// Background color
    pub background: Color,
    /// Opacity (0.0 to 1.0, INHERITED)
    pub opacity: f32,
    /// Visibility flag (INHERITED)
    pub visible: bool,
    /// Z-index for stacking order
    pub z_index: i16,
}

impl Default for VisualStyle {
    fn default() -> Self {
        Self {
            border_style: BorderStyle::default(),
            border_color: Color::default(),
            color: Color::default(),
            background: Color::default(),
            opacity: 1.0,
            visible: true,
            z_index: 0,
        }
    }
}
