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
    use crate::style::Color;

    #[test]
    fn test_computed_style_new() {
        let computed = ComputedStyle::new();
        // Default style has default color
        let _ = computed.style.color(); // Should not panic
    }

    #[test]
    fn test_computed_style_default() {
        let computed = ComputedStyle::default();
        // Should be equivalent to new()
        let _ = computed.style.color();
    }

    #[test]
    fn test_computed_style_compute_no_parent() {
        let style = Style::default();
        let computed = ComputedStyle::compute(style, None);
        // Without parent, just returns the style
        let _ = computed.style.color();
    }

    #[test]
    fn test_computed_style_compute_with_parent() {
        // Parent has a color
        let mut parent_style = Style::default();
        parent_style.visual.color = Color::RED;
        let parent = ComputedStyle {
            style: parent_style,
        };

        // Child style without color set
        let child_style = Style::default();
        let computed = ComputedStyle::compute(child_style, Some(&parent));

        // Child should inherit color from parent
        assert_eq!(computed.style.color(), Color::RED);
    }

    #[test]
    fn test_computed_style_compute_child_overrides_parent() {
        // Parent has a color
        let mut parent_style = Style::default();
        parent_style.visual.color = Color::RED;
        let parent = ComputedStyle {
            style: parent_style,
        };

        // Child style with its own color
        let mut child_style = Style::default();
        child_style.visual.color = Color::BLUE;
        let computed = ComputedStyle::compute(child_style, Some(&parent));

        // Child's color should override parent's
        assert_eq!(computed.style.color(), Color::BLUE);
    }

    #[test]
    fn test_computed_style_clone() {
        let computed = ComputedStyle::new();
        let cloned = computed.clone();
        // Clone should work
        let _ = cloned.style.color();
    }

    #[test]
    fn test_computed_style_debug() {
        let computed = ComputedStyle::new();
        let debug = format!("{:?}", computed);
        assert!(debug.contains("ComputedStyle"));
    }

    #[test]
    fn test_computed_style_inheritance_chain() {
        // Test multiple levels of inheritance
        let mut grandparent_style = Style::default();
        grandparent_style.visual.color = Color::RED;
        let grandparent = ComputedStyle {
            style: grandparent_style,
        };

        // Parent inherits from grandparent
        let parent_style = Style::default();
        let parent = ComputedStyle::compute(parent_style, Some(&grandparent));

        // Child inherits from parent
        let child_style = Style::default();
        let child = ComputedStyle::compute(child_style, Some(&parent));

        // Color should propagate through the chain
        assert_eq!(child.style.color(), Color::RED);
    }
}
