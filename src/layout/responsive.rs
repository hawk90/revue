#![allow(clippy::should_implement_trait, clippy::module_inception)]
//! Responsive layout breakpoints for terminal applications
//!
//! Provides a CSS-like breakpoint system for adapting layouts to different
//! terminal sizes.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::layout::{Breakpoints, Breakpoint, ResponsiveValue};
//!
//! // Create custom breakpoints
//! let bp = Breakpoints::new()
//!     .add(Breakpoint::new("sm", 40))
//!     .add(Breakpoint::new("md", 80))
//!     .add(Breakpoint::new("lg", 120));
//!
//! // Get current breakpoint for terminal width
//! let current = bp.current(100);
//! assert_eq!(current.name, "md");
//!
//! // Responsive values
//! let columns = ResponsiveValue::new(1)
//!     .at("sm", 2)
//!     .at("md", 3)
//!     .at("lg", 4);
//!
//! let cols = columns.resolve(&bp, 100);
//! assert_eq!(cols, 3);
//! ```

use std::collections::HashMap;

/// A responsive breakpoint
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Breakpoint {
    /// Breakpoint name (e.g., "sm", "md", "lg")
    pub name: &'static str,
    /// Minimum width for this breakpoint
    pub min_width: u16,
}

impl Breakpoint {
    /// Create a new breakpoint
    pub fn new(name: &'static str, min_width: u16) -> Self {
        Self { name, min_width }
    }
}

/// Common terminal breakpoints
impl Breakpoint {
    /// Extra small (< 40 columns) - minimal terminals
    pub const XS: Breakpoint = Breakpoint {
        name: "xs",
        min_width: 0,
    };
    /// Small (40-79 columns) - compact terminals
    pub const SM: Breakpoint = Breakpoint {
        name: "sm",
        min_width: 40,
    };
    /// Medium (80-119 columns) - standard terminals
    pub const MD: Breakpoint = Breakpoint {
        name: "md",
        min_width: 80,
    };
    /// Large (120-159 columns) - wide terminals
    pub const LG: Breakpoint = Breakpoint {
        name: "lg",
        min_width: 120,
    };
    /// Extra large (160+ columns) - ultra-wide terminals
    pub const XL: Breakpoint = Breakpoint {
        name: "xl",
        min_width: 160,
    };
}

/// Breakpoint collection
#[derive(Debug, Clone)]
pub struct Breakpoints {
    /// Sorted list of breakpoints (by min_width ascending)
    points: Vec<Breakpoint>,
}

impl Breakpoints {
    /// Create an empty breakpoint set
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Create standard terminal breakpoints
    pub fn terminal() -> Self {
        Self {
            points: vec![
                Breakpoint::XS,
                Breakpoint::SM,
                Breakpoint::MD,
                Breakpoint::LG,
                Breakpoint::XL,
            ],
        }
    }

    /// Create minimal breakpoints (small, medium, large)
    pub fn simple() -> Self {
        Self {
            points: vec![Breakpoint::SM, Breakpoint::MD, Breakpoint::LG],
        }
    }

    /// Add a breakpoint
    pub fn add(mut self, bp: Breakpoint) -> Self {
        self.points.push(bp);
        self.points.sort_by_key(|b| b.min_width);
        self
    }

    /// Get current breakpoint for width
    pub fn current(&self, width: u16) -> &Breakpoint {
        self.points
            .iter()
            .rev()
            .find(|bp| width >= bp.min_width)
            .unwrap_or_else(|| self.points.first().unwrap_or(&Breakpoint::XS))
    }

    /// Get breakpoint by name
    pub fn get(&self, name: &str) -> Option<&Breakpoint> {
        self.points.iter().find(|bp| bp.name == name)
    }

    /// Check if width matches a breakpoint name
    pub fn matches(&self, width: u16, name: &str) -> bool {
        self.current(width).name == name
    }

    /// Check if width is at least the given breakpoint
    pub fn at_least(&self, width: u16, name: &str) -> bool {
        if let Some(target) = self.get(name) {
            width >= target.min_width
        } else {
            false
        }
    }

    /// Check if width is less than the given breakpoint
    pub fn below(&self, width: u16, name: &str) -> bool {
        if let Some(target) = self.get(name) {
            width < target.min_width
        } else {
            true
        }
    }

    /// Get all breakpoint names in order
    pub fn names(&self) -> Vec<&'static str> {
        self.points.iter().map(|bp| bp.name).collect()
    }

    /// Iterate over breakpoints
    pub fn iter(&self) -> impl Iterator<Item = &Breakpoint> {
        self.points.iter()
    }
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self::terminal()
    }
}

/// A value that varies based on breakpoint
#[derive(Debug, Clone)]
pub struct ResponsiveValue<T: Clone> {
    /// Default value (for smallest breakpoint)
    default: T,
    /// Values for each breakpoint
    values: HashMap<&'static str, T>,
}

impl<T: Clone> ResponsiveValue<T> {
    /// Create a new responsive value with a default
    pub fn new(default: T) -> Self {
        Self {
            default,
            values: HashMap::new(),
        }
    }

    /// Set value for a breakpoint
    pub fn at(mut self, breakpoint: &'static str, value: T) -> Self {
        self.values.insert(breakpoint, value);
        self
    }

    /// Resolve value for current width
    pub fn resolve(&self, breakpoints: &Breakpoints, width: u16) -> T {
        // Find the current breakpoint
        let current = breakpoints.current(width);

        // Walk from current breakpoint down to find a defined value
        for bp in breakpoints.points.iter().rev() {
            if bp.min_width <= current.min_width {
                if let Some(value) = self.values.get(bp.name) {
                    return value.clone();
                }
            }
        }

        self.default.clone()
    }

    /// Get the default value
    pub fn default_value(&self) -> &T {
        &self.default
    }
}

/// Responsive layout configuration
#[derive(Debug, Clone)]
pub struct ResponsiveLayout {
    /// Breakpoints to use
    breakpoints: Breakpoints,
    /// Current terminal width
    width: u16,
    /// Current terminal height
    height: u16,
}

impl ResponsiveLayout {
    /// Create a new responsive layout
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            breakpoints: Breakpoints::terminal(),
            width,
            height,
        }
    }

    /// Set custom breakpoints
    pub fn with_breakpoints(mut self, breakpoints: Breakpoints) -> Self {
        self.breakpoints = breakpoints;
        self
    }

    /// Update dimensions
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Get current breakpoint
    pub fn current(&self) -> &Breakpoint {
        self.breakpoints.current(self.width)
    }

    /// Get current breakpoint name
    pub fn breakpoint_name(&self) -> &'static str {
        self.current().name
    }

    /// Check if at least the given breakpoint
    pub fn at_least(&self, name: &str) -> bool {
        self.breakpoints.at_least(self.width, name)
    }

    /// Check if below the given breakpoint
    pub fn below(&self, name: &str) -> bool {
        self.breakpoints.below(self.width, name)
    }

    /// Resolve a responsive value
    pub fn resolve<T: Clone>(&self, value: &ResponsiveValue<T>) -> T {
        value.resolve(&self.breakpoints, self.width)
    }

    /// Get width
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Get height
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Check if in portrait orientation (height > width)
    pub fn is_portrait(&self) -> bool {
        self.height > self.width
    }

    /// Check if in landscape orientation (width >= height)
    pub fn is_landscape(&self) -> bool {
        self.width >= self.height
    }
}

impl Default for ResponsiveLayout {
    fn default() -> Self {
        Self::new(80, 24)
    }
}

/// Media query-like condition
#[derive(Debug, Clone)]
pub enum MediaQuery {
    /// Minimum width
    MinWidth(u16),
    /// Maximum width
    MaxWidth(u16),
    /// Width range (inclusive)
    WidthRange(u16, u16),
    /// Minimum height
    MinHeight(u16),
    /// Maximum height
    MaxHeight(u16),
    /// Breakpoint name
    Breakpoint(&'static str),
    /// Portrait orientation
    Portrait,
    /// Landscape orientation
    Landscape,
    /// Combine queries with AND
    And(Box<MediaQuery>, Box<MediaQuery>),
    /// Combine queries with OR
    Or(Box<MediaQuery>, Box<MediaQuery>),
    /// Negate a query
    Not(Box<MediaQuery>),
}

impl MediaQuery {
    /// Check if query matches
    pub fn matches(&self, layout: &ResponsiveLayout) -> bool {
        match self {
            MediaQuery::MinWidth(w) => layout.width >= *w,
            MediaQuery::MaxWidth(w) => layout.width <= *w,
            MediaQuery::WidthRange(min, max) => layout.width >= *min && layout.width <= *max,
            MediaQuery::MinHeight(h) => layout.height >= *h,
            MediaQuery::MaxHeight(h) => layout.height <= *h,
            MediaQuery::Breakpoint(name) => layout.breakpoint_name() == *name,
            MediaQuery::Portrait => layout.is_portrait(),
            MediaQuery::Landscape => layout.is_landscape(),
            MediaQuery::And(a, b) => a.matches(layout) && b.matches(layout),
            MediaQuery::Or(a, b) => a.matches(layout) || b.matches(layout),
            MediaQuery::Not(q) => !q.matches(layout),
        }
    }

    /// Combine with AND
    pub fn and(self, other: MediaQuery) -> MediaQuery {
        MediaQuery::And(Box::new(self), Box::new(other))
    }

    /// Combine with OR
    pub fn or(self, other: MediaQuery) -> MediaQuery {
        MediaQuery::Or(Box::new(self), Box::new(other))
    }

    /// Negate
    pub fn not(self) -> MediaQuery {
        MediaQuery::Not(Box::new(self))
    }
}

/// Helper functions for common responsive patterns
pub mod responsive {
    use super::*;

    /// Create responsive columns based on width
    pub fn columns(layout: &ResponsiveLayout) -> u16 {
        if layout.below("sm") {
            1
        } else if layout.below("md") {
            2
        } else if layout.below("lg") {
            3
        } else {
            4
        }
    }

    /// Create responsive gap size
    pub fn gap(layout: &ResponsiveLayout) -> u16 {
        if layout.below("sm") {
            0
        } else if layout.below("md") {
            1
        } else {
            2
        }
    }

    /// Create responsive padding
    pub fn padding(layout: &ResponsiveLayout) -> u16 {
        if layout.below("sm") {
            0
        } else if layout.below("md") {
            1
        } else if layout.below("lg") {
            2
        } else {
            3
        }
    }

    /// Check if sidebar should be visible
    pub fn show_sidebar(layout: &ResponsiveLayout) -> bool {
        layout.at_least("md")
    }

    /// Check if should use compact mode
    pub fn compact_mode(layout: &ResponsiveLayout) -> bool {
        layout.below("sm")
    }

    /// Get recommended max content width
    pub fn max_content_width(layout: &ResponsiveLayout) -> u16 {
        if layout.at_least("xl") {
            120
        } else if layout.at_least("lg") {
            100
        } else {
            layout.width
        }
    }
}

// Convenience constructors

/// Create a responsive value
pub fn responsive<T: Clone>(default: T) -> ResponsiveValue<T> {
    ResponsiveValue::new(default)
}

/// Create standard terminal breakpoints
pub fn breakpoints() -> Breakpoints {
    Breakpoints::terminal()
}

/// Create a responsive layout
pub fn responsive_layout(width: u16, height: u16) -> ResponsiveLayout {
    ResponsiveLayout::new(width, height)
}

/// Create a min-width query
pub fn min_width(w: u16) -> MediaQuery {
    MediaQuery::MinWidth(w)
}

/// Create a max-width query
pub fn max_width(w: u16) -> MediaQuery {
    MediaQuery::MaxWidth(w)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_new() {
        let bp = Breakpoint::new("custom", 100);
        assert_eq!(bp.name, "custom");
        assert_eq!(bp.min_width, 100);
    }

    #[test]
    fn test_breakpoints_current() {
        let bp = Breakpoints::terminal();

        assert_eq!(bp.current(30).name, "xs");
        assert_eq!(bp.current(50).name, "sm");
        assert_eq!(bp.current(80).name, "md");
        assert_eq!(bp.current(120).name, "lg");
        assert_eq!(bp.current(200).name, "xl");
    }

    #[test]
    fn test_breakpoints_at_least() {
        let bp = Breakpoints::terminal();

        assert!(bp.at_least(80, "md"));
        assert!(bp.at_least(100, "md"));
        assert!(!bp.at_least(60, "md"));
    }

    #[test]
    fn test_breakpoints_below() {
        let bp = Breakpoints::terminal();

        assert!(bp.below(60, "md"));
        assert!(!bp.below(80, "md"));
        assert!(!bp.below(100, "md"));
    }

    #[test]
    fn test_responsive_value() {
        let bp = Breakpoints::terminal();
        let value = ResponsiveValue::new(1).at("sm", 2).at("md", 3).at("lg", 4);

        assert_eq!(value.resolve(&bp, 30), 1); // xs
        assert_eq!(value.resolve(&bp, 50), 2); // sm
        assert_eq!(value.resolve(&bp, 80), 3); // md
        assert_eq!(value.resolve(&bp, 120), 4); // lg
    }

    #[test]
    fn test_responsive_value_cascade() {
        let bp = Breakpoints::terminal();
        // Only define value for "md", should cascade up
        let value = ResponsiveValue::new(1).at("md", 5);

        assert_eq!(value.resolve(&bp, 30), 1); // xs - uses default
        assert_eq!(value.resolve(&bp, 80), 5); // md - uses md value
        assert_eq!(value.resolve(&bp, 120), 5); // lg - cascades md value
    }

    #[test]
    fn test_responsive_layout() {
        let layout = ResponsiveLayout::new(100, 30);

        assert_eq!(layout.breakpoint_name(), "md");
        assert!(layout.at_least("sm"));
        assert!(layout.at_least("md"));
        assert!(!layout.at_least("lg"));
        assert!(layout.below("lg"));
    }

    #[test]
    fn test_responsive_layout_orientation() {
        let portrait = ResponsiveLayout::new(80, 100);
        assert!(portrait.is_portrait());
        assert!(!portrait.is_landscape());

        let landscape = ResponsiveLayout::new(120, 40);
        assert!(landscape.is_landscape());
        assert!(!landscape.is_portrait());
    }

    #[test]
    fn test_media_query_basic() {
        let layout = ResponsiveLayout::new(100, 30);

        assert!(MediaQuery::MinWidth(80).matches(&layout));
        assert!(!MediaQuery::MinWidth(120).matches(&layout));
        assert!(MediaQuery::MaxWidth(120).matches(&layout));
        assert!(!MediaQuery::MaxWidth(80).matches(&layout));
    }

    #[test]
    fn test_media_query_range() {
        let layout = ResponsiveLayout::new(100, 30);

        assert!(MediaQuery::WidthRange(80, 120).matches(&layout));
        assert!(!MediaQuery::WidthRange(120, 160).matches(&layout));
    }

    #[test]
    fn test_media_query_combined() {
        let layout = ResponsiveLayout::new(100, 30);

        let query = MediaQuery::MinWidth(80).and(MediaQuery::MaxWidth(120));
        assert!(query.matches(&layout));

        let query = MediaQuery::MinWidth(120).or(MediaQuery::MaxWidth(80));
        assert!(!query.matches(&layout));

        let query = MediaQuery::MinWidth(120).not();
        assert!(query.matches(&layout));
    }

    #[test]
    fn test_media_query_breakpoint() {
        let layout = ResponsiveLayout::new(100, 30);

        assert!(MediaQuery::Breakpoint("md").matches(&layout));
        assert!(!MediaQuery::Breakpoint("lg").matches(&layout));
    }

    #[test]
    fn test_responsive_helpers() {
        let small = ResponsiveLayout::new(30, 24);
        let medium = ResponsiveLayout::new(100, 30);
        let large = ResponsiveLayout::new(150, 40);

        // Columns
        assert_eq!(responsive::columns(&small), 1);
        assert_eq!(responsive::columns(&medium), 3);
        assert_eq!(responsive::columns(&large), 4);

        // Sidebar
        assert!(!responsive::show_sidebar(&small));
        assert!(responsive::show_sidebar(&medium));

        // Compact mode
        assert!(responsive::compact_mode(&small));
        assert!(!responsive::compact_mode(&medium));
    }

    #[test]
    fn test_custom_breakpoints() {
        let bp = Breakpoints::new()
            .add(Breakpoint::new("tiny", 0))
            .add(Breakpoint::new("normal", 60))
            .add(Breakpoint::new("wide", 100));

        assert_eq!(bp.current(50).name, "tiny");
        assert_eq!(bp.current(60).name, "normal");
        assert_eq!(bp.current(100).name, "wide");
    }

    #[test]
    fn test_resize() {
        let mut layout = ResponsiveLayout::new(60, 24);
        assert_eq!(layout.breakpoint_name(), "sm");

        layout.resize(100, 30);
        assert_eq!(layout.breakpoint_name(), "md");
    }
}
