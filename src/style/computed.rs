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
