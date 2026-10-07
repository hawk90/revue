//! Viewport-sized responsive layout and media queries

use super::{Breakpoint, Breakpoints, ResponsiveValue};

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
    fn test_responsive_layout_new() {
        let layout = ResponsiveLayout::new(80, 24);
        assert_eq!(layout.width(), 80);
        assert_eq!(layout.height(), 24);
        assert!(layout.is_landscape());
        assert!(!layout.is_portrait());
    }

    #[test]
    fn test_responsive_layout_default() {
        let layout = ResponsiveLayout::default();
        assert_eq!(layout.width(), 80);
        assert_eq!(layout.height(), 24);
    }

    #[test]
    fn test_responsive_layout_portrait() {
        let layout = ResponsiveLayout::new(40, 80);
        assert!(layout.is_portrait());
        assert!(!layout.is_landscape());
    }

    #[test]
    fn test_responsive_layout_resize() {
        let mut layout = ResponsiveLayout::new(80, 24);
        layout.resize(100, 30);
        assert_eq!(layout.width(), 100);
        assert_eq!(layout.height(), 30);
    }

    #[test]
    fn test_responsive_layout_breakpoint_name() {
        let layout = ResponsiveLayout::new(80, 24);
        assert_eq!(layout.breakpoint_name(), "md");
    }

    #[test]
    fn test_responsive_layout_at_least() {
        let layout = ResponsiveLayout::new(80, 24);
        assert!(layout.at_least("sm"));
        assert!(layout.at_least("md"));
        assert!(!layout.at_least("lg"));
    }

    #[test]
    fn test_responsive_layout_below() {
        let layout = ResponsiveLayout::new(80, 24);
        assert!(layout.below("lg"));
        assert!(!layout.below("md"));
        assert!(!layout.below("sm"));
    }

    #[test]
    fn test_responsive_layout_resolve() {
        let layout = ResponsiveLayout::new(80, 24);
        let rv = ResponsiveValue::new(1).at("md", 3);
        assert_eq!(layout.resolve(&rv), 3);
    }

    #[test]
    fn test_media_query_min_width() {
        let layout = ResponsiveLayout::new(80, 24);
        assert!(MediaQuery::MinWidth(50).matches(&layout));
        assert!(MediaQuery::MinWidth(80).matches(&layout));
        assert!(!MediaQuery::MinWidth(100).matches(&layout));
    }

    #[test]
    fn test_media_query_max_width() {
        let layout = ResponsiveLayout::new(80, 24);
        assert!(MediaQuery::MaxWidth(100).matches(&layout));
        assert!(MediaQuery::MaxWidth(80).matches(&layout));
        assert!(!MediaQuery::MaxWidth(50).matches(&layout));
    }

    #[test]
    fn test_media_query_width_range() {
        let layout = ResponsiveLayout::new(80, 24);
        assert!(MediaQuery::WidthRange(60, 100).matches(&layout));
        assert!(MediaQuery::WidthRange(80, 80).matches(&layout));
        assert!(!MediaQuery::WidthRange(90, 100).matches(&layout));
        assert!(!MediaQuery::WidthRange(60, 70).matches(&layout));
    }

    #[test]
    fn test_media_query_min_height() {
        let layout = ResponsiveLayout::new(80, 24);
        assert!(MediaQuery::MinHeight(20).matches(&layout));
        assert!(!MediaQuery::MinHeight(30).matches(&layout));
    }

    #[test]
    fn test_media_query_max_height() {
        let layout = ResponsiveLayout::new(80, 24);
        assert!(MediaQuery::MaxHeight(30).matches(&layout));
        assert!(!MediaQuery::MaxHeight(20).matches(&layout));
    }

    #[test]
    fn test_media_query_breakpoint() {
        let layout = ResponsiveLayout::new(80, 24);
        assert!(MediaQuery::Breakpoint("md").matches(&layout));
        assert!(!MediaQuery::Breakpoint("sm").matches(&layout));
        assert!(!MediaQuery::Breakpoint("lg").matches(&layout));
    }

    #[test]
    fn test_media_query_portrait() {
        let layout = ResponsiveLayout::new(40, 80);
        assert!(MediaQuery::Portrait.matches(&layout));
        assert!(!MediaQuery::Landscape.matches(&layout));
    }

    #[test]
    fn test_media_query_landscape() {
        let layout = ResponsiveLayout::new(80, 40);
        assert!(MediaQuery::Landscape.matches(&layout));
        assert!(!MediaQuery::Portrait.matches(&layout));
    }

    #[test]
    fn test_media_query_and() {
        let layout = ResponsiveLayout::new(80, 24);
        let query = MediaQuery::MinWidth(60).and(MediaQuery::MaxWidth(100));
        assert!(query.matches(&layout));

        let layout2 = ResponsiveLayout::new(120, 24);
        assert!(!query.matches(&layout2));
    }

    #[test]
    fn test_media_query_or() {
        let layout = ResponsiveLayout::new(80, 24);
        let query = MediaQuery::MinWidth(100).or(MediaQuery::MaxWidth(90));
        assert!(query.matches(&layout));
    }

    #[test]
    fn test_media_query_not() {
        let layout = ResponsiveLayout::new(80, 24);
        let query = MediaQuery::MinWidth(100).not();
        assert!(query.matches(&layout));

        let query2 = MediaQuery::MinWidth(50).not();
        assert!(!query2.matches(&layout));
    }

    #[test]
    fn test_responsive_layout_function() {
        let layout = responsive_layout(100, 30);
        assert_eq!(layout.width(), 100);
        assert_eq!(layout.height(), 30);
    }

    #[test]
    fn test_min_width_function() {
        let query = min_width(50);
        assert!(matches!(query, MediaQuery::MinWidth(50)));
    }

    #[test]
    fn test_max_width_function() {
        let query = max_width(50);
        assert!(matches!(query, MediaQuery::MaxWidth(50)));
    }

    #[test]
    fn test_responsive_columns() {
        let layout = ResponsiveLayout::new(30, 24);
        assert_eq!(responsive::columns(&layout), 1);

        let layout = ResponsiveLayout::new(50, 24);
        assert_eq!(responsive::columns(&layout), 2);

        let layout = ResponsiveLayout::new(100, 24);
        assert_eq!(responsive::columns(&layout), 3);

        let layout = ResponsiveLayout::new(150, 24);
        assert_eq!(responsive::columns(&layout), 4);
    }

    #[test]
    fn test_responsive_gap() {
        let layout = ResponsiveLayout::new(30, 24);
        assert_eq!(responsive::gap(&layout), 0);

        let layout = ResponsiveLayout::new(50, 24);
        assert_eq!(responsive::gap(&layout), 1);

        let layout = ResponsiveLayout::new(100, 24);
        assert_eq!(responsive::gap(&layout), 2);
    }

    #[test]
    fn test_responsive_padding() {
        let layout = ResponsiveLayout::new(30, 24);
        assert_eq!(responsive::padding(&layout), 0);

        let layout = ResponsiveLayout::new(50, 24);
        assert_eq!(responsive::padding(&layout), 1);

        let layout = ResponsiveLayout::new(100, 24);
        assert_eq!(responsive::padding(&layout), 2);

        let layout = ResponsiveLayout::new(150, 24);
        assert_eq!(responsive::padding(&layout), 3);
    }

    #[test]
    fn test_responsive_show_sidebar() {
        let layout = ResponsiveLayout::new(30, 24);
        assert!(!responsive::show_sidebar(&layout));

        let layout = ResponsiveLayout::new(80, 24);
        assert!(responsive::show_sidebar(&layout));
    }

    #[test]
    fn test_responsive_compact_mode() {
        let layout = ResponsiveLayout::new(30, 24);
        assert!(responsive::compact_mode(&layout));

        let layout = ResponsiveLayout::new(80, 24);
        assert!(!responsive::compact_mode(&layout));
    }

    #[test]
    fn test_responsive_max_content_width() {
        let layout = ResponsiveLayout::new(180, 24);
        assert_eq!(responsive::max_content_width(&layout), 120);

        let layout = ResponsiveLayout::new(130, 24);
        assert_eq!(responsive::max_content_width(&layout), 100);

        let layout = ResponsiveLayout::new(100, 24);
        assert_eq!(responsive::max_content_width(&layout), 100);
    }
}
