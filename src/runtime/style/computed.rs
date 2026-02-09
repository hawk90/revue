//! Computed style values

use super::Style;

/// Fully computed style values for rendering
///
/// Represents style values after CSS cascading and inheritance
/// have been applied. Ready for use in layout and rendering.
#[derive(Debug, Clone, Default)]
pub struct ComputedStyle {
    /// The resolved style after cascading
    pub style: Style,
}

impl ComputedStyle {
    /// Create a new empty computed style
    pub fn new() -> Self {
        Self::default()
    }

    /// Compute style from raw style with inheritance
    ///
    /// CSS Inherited Properties (from parent):
    /// - `color` - text color
    /// - `opacity` - visual opacity
    /// - `visible` - visibility
    ///
    /// Non-inherited properties reset to their defaults.
    pub fn compute(style: Style, parent: Option<&ComputedStyle>) -> Self {
        let computed_style = match parent {
            Some(parent_computed) => style.with_inheritance(&parent_computed.style),
            None => style,
        };
        Self {
            style: computed_style,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computed_style_new() {
        let computed = ComputedStyle::new();
        // Verify it creates a style
        assert_eq!(computed.style.layout.gap, 0);
        assert_eq!(computed.style.spacing.padding.top, 0);
    }

    #[test]
    fn test_computed_style_compute_no_parent() {
        let style = Style::default();
        let computed = ComputedStyle::compute(style.clone(), None);

        // Should create a computed style with the same values
        assert_eq!(computed.style.layout.gap, style.layout.gap);
    }

    #[test]
    fn test_computed_style_compute_with_parent() {
        let parent_style = Style::default();
        let parent = ComputedStyle {
            style: parent_style,
        };

        let child_style = Style::default();
        let computed = ComputedStyle::compute(child_style.clone(), Some(&parent));

        // Should create a computed style
        assert_eq!(computed.style.layout.gap, parent.style.layout.gap);
    }

    #[test]
    fn test_computed_style_default() {
        let computed = ComputedStyle::default();
        // Default should have default values
        assert_eq!(computed.style.layout.gap, 0);
    }

    #[test]
    fn test_computed_style_preserves_values() {
        let mut style = Style::default();
        style.layout.gap = 10;
        style.spacing.padding.top = 5;

        let computed = ComputedStyle::compute(style, None);

        // Should preserve the values
        assert_eq!(computed.style.layout.gap, 10);
        assert_eq!(computed.style.spacing.padding.top, 5);
    }

    #[test]
    fn test_computed_style_clones_style() {
        let style = Style::default();
        let computed = ComputedStyle::compute(style.clone(), None);

        // Should not affect original style
        assert_eq!(computed.style.layout.gap, style.layout.gap);
    }
}

// Tests moved to tests/style_tests.rs
