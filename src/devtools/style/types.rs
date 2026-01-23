//! Type definitions for style inspector

/// Computed CSS property
#[derive(Debug, Clone)]
pub struct ComputedProperty {
    /// Property name
    pub name: String,
    /// Property value
    pub value: String,
    /// Source (inline, class, inherited)
    pub source: PropertySource,
    /// Is overridden by higher specificity
    pub overridden: bool,
}

/// Source of a CSS property
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PropertySource {
    /// Inline style
    Inline,
    /// From a CSS class
    Class,
    /// From widget ID selector
    Id,
    /// Inherited from parent
    Inherited,
    /// Default/computed value
    #[default]
    Computed,
    /// From theme
    Theme,
}

/// Style category for grouping properties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StyleCategory {
    /// Layout properties (width, height, margin, padding)
    Layout,
    /// Typography (font, text)
    Typography,
    /// Colors and backgrounds
    Colors,
    /// Borders
    Border,
    /// Effects (shadows, opacity)
    Effects,
    /// Other
    Other,
}
