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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizing_style_default() {
        let style = SizingStyle::default();
        assert_eq!(style.width, Size::default());
        assert_eq!(style.height, Size::default());
        assert_eq!(style.min_width, Size::default());
        assert_eq!(style.max_width, Size::default());
        assert_eq!(style.min_height, Size::default());
        assert_eq!(style.max_height, Size::default());
    }

    #[test]
    fn test_sizing_style_clone() {
        let mut style = SizingStyle::default();
        style.width = Size::Fixed(100);
        let cloned = style.clone();
        assert_eq!(cloned.width, Size::Fixed(100));
    }

    #[test]
    fn test_sizing_style_partial_eq() {
        let style1 = SizingStyle::default();
        let style2 = SizingStyle::default();
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_sizing_style_not_equal() {
        let mut style1 = SizingStyle::default();
        style1.width = Size::Fixed(100);
        let style2 = SizingStyle::default();
        assert_ne!(style1, style2);
    }

    #[test]
    fn test_sizing_style_copy_trait() {
        let style1 = SizingStyle {
            width: Size::Fixed(100),
            ..Default::default()
        };
        let style2 = style1;
        assert_eq!(style2.width, Size::Fixed(100));
    }

    #[test]
    fn test_sizing_style_debug() {
        let style = SizingStyle::default();
        let debug_str = format!("{:?}", style);
        assert!(debug_str.contains("SizingStyle"));
    }
}
