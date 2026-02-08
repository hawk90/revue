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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_style_default() {
        let style = SpacingStyle::default();
        assert_eq!(style.padding, Spacing::default());
        assert_eq!(style.margin, Spacing::default());
        assert_eq!(style.top, None);
        assert_eq!(style.right, None);
        assert_eq!(style.bottom, None);
        assert_eq!(style.left, None);
    }

    #[test]
    fn test_spacing_style_clone() {
        let mut style = SpacingStyle::default();
        style.top = Some(10);
        let cloned = style.clone();
        assert_eq!(cloned.top, Some(10));
    }

    #[test]
    fn test_spacing_style_partial_eq() {
        let style1 = SpacingStyle::default();
        let style2 = SpacingStyle::default();
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_spacing_style_not_equal() {
        let mut style1 = SpacingStyle::default();
        style1.top = Some(10);
        let style2 = SpacingStyle::default();
        assert_ne!(style1, style2);
    }

    #[test]
    fn test_spacing_style_copy_trait() {
        let style1 = SpacingStyle {
            top: Some(10),
            ..Default::default()
        };
        let style2 = style1;
        assert_eq!(style2.top, Some(10));
    }

    #[test]
    fn test_spacing_style_debug() {
        let style = SpacingStyle::default();
        let debug_str = format!("{:?}", style);
        assert!(debug_str.contains("SpacingStyle"));
    }
}
