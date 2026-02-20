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

impl VisualStyle {
    /// Check if opacity is effectively full (1.0)
    pub fn is_fully_opaque(&self) -> bool {
        self.opacity >= 1.0
    }

    /// Check if opacity is effectively invisible (0.0)
    pub fn is_invisible(&self) -> bool {
        self.opacity <= 0.0
    }
}

/// Apply opacity to a cell modifier. Returns true if the cell should be visible.
/// - opacity <= 0.0: invisible
/// - opacity < 0.5: invisible
/// - opacity < 1.0: add DIM modifier
/// - opacity >= 1.0: no change
pub fn apply_opacity(opacity: f32, modifier: &mut crate::render::Modifier) -> bool {
    if opacity <= 0.0 || opacity < 0.5 {
        return false;
    }
    if opacity < 1.0 {
        *modifier |= crate::render::Modifier::DIM;
    }
    true
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visual_style_default() {
        let style = VisualStyle::default();
        assert_eq!(style.border_style, BorderStyle::default());
        assert_eq!(style.border_color, Color::default());
        assert_eq!(style.color, Color::default());
        assert_eq!(style.background, Color::default());
        assert_eq!(style.opacity, 1.0);
        assert_eq!(style.visible, true);
        assert_eq!(style.z_index, 0);
    }

    #[test]
    fn test_visual_style_clone() {
        let mut style = VisualStyle::default();
        style.opacity = 0.5;
        let cloned = style.clone();
        assert_eq!(cloned.opacity, 0.5);
    }

    #[test]
    fn test_visual_style_partial_eq() {
        let style1 = VisualStyle::default();
        let style2 = VisualStyle::default();
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_visual_style_not_equal() {
        let mut style1 = VisualStyle::default();
        style1.opacity = 0.5;
        let style2 = VisualStyle::default();
        assert_ne!(style1, style2);
    }

    #[test]
    fn test_visual_style_copy_trait() {
        let style1 = VisualStyle {
            opacity: 0.5,
            ..Default::default()
        };
        let style2 = style1;
        assert_eq!(style2.opacity, 0.5);
    }

    #[test]
    fn test_visual_style_debug() {
        let style = VisualStyle::default();
        let debug_str = format!("{:?}", style);
        assert!(debug_str.contains("VisualStyle"));
    }

    #[test]
    fn test_visual_style_default_values() {
        let style = VisualStyle::default();
        // Check inherited property defaults
        assert_eq!(style.opacity, 1.0);
        assert_eq!(style.visible, true);
        // Check non-inherited property defaults
        assert_eq!(style.z_index, 0);
    }
}
