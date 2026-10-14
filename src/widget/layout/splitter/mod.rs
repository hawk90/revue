//! Splitter widget for resizable split panels
//!
//! Allows dividing an area into resizable panes with draggable dividers.

mod input;
mod layout;
mod render;
mod two_pane;
mod types;

pub use two_pane::{HSplit, VSplit};
pub use types::{Pane, SplitOrientation, SplitterStyle};

use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Splitter widget
#[derive(Clone)]
pub struct Splitter {
    /// Panes
    panes: Vec<Pane>,
    /// Orientation
    orientation: SplitOrientation,
    /// Style
    style: SplitterStyle,
    /// Splitter color
    /// The color the builder named, if it named one - see #656.
    color: Option<Color>,
    /// Active splitter color
    active_color: Color,
    /// Currently active divider (for resizing)
    active_divider: Option<usize>,
    /// Focused pane index
    focused_pane: usize,
    /// Splitter width
    splitter_width: u16,
    /// Widget props for CSS integration
    props: WidgetProps,
}

impl Splitter {
    /// Create a new splitter
    pub fn new() -> Self {
        Self {
            panes: Vec::new(),
            orientation: SplitOrientation::Horizontal,
            style: SplitterStyle::Line,
            color: None,
            active_color: Color::CYAN,
            active_divider: None,
            focused_pane: 0,
            splitter_width: 1,
            props: WidgetProps::new(),
        }
    }

    /// Set orientation
    pub fn orientation(mut self, orientation: SplitOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set horizontal orientation
    pub fn horizontal(mut self) -> Self {
        self.orientation = SplitOrientation::Horizontal;
        self
    }

    /// Set vertical orientation
    pub fn vertical(mut self) -> Self {
        self.orientation = SplitOrientation::Vertical;
        self
    }

    /// Add a pane
    pub fn pane(mut self, pane: Pane) -> Self {
        self.panes.push(pane);
        self
    }

    /// Add panes
    pub fn panes(mut self, panes: Vec<Pane>) -> Self {
        self.panes.extend(panes);
        self
    }

    /// Set style
    pub fn style(mut self, style: SplitterStyle) -> Self {
        self.style = style;
        self
    }

    /// Set splitter color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set active color
    pub fn active_color(mut self, color: Color) -> Self {
        self.active_color = color;
        self
    }

    /// Get focused pane id
    pub fn focused(&self) -> Option<&str> {
        self.panes.get(self.focused_pane).map(|p| p.id.as_str())
    }
}

impl Default for Splitter {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(Splitter);
impl_props_builders!(Splitter);

// Helper functions

/// Create a new splitter container
pub fn splitter() -> Splitter {
    Splitter::new()
}

/// Create a new pane with an identifier
pub fn pane(id: impl Into<String>) -> Pane {
    Pane::new(id)
}

/// Create a horizontal split with the given ratio
pub fn hsplit(ratio: f32) -> HSplit {
    HSplit::new(ratio)
}

/// Create a vertical split with the given ratio
pub fn vsplit(ratio: f32) -> VSplit {
    VSplit::new(ratio)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;

    #[test]
    fn test_splitter_new() {
        let s = Splitter::new();
        assert_eq!(s.orientation, SplitOrientation::Horizontal);
        assert!(s.panes.is_empty());
    }

    #[test]
    fn test_splitter_orientation() {
        let s = Splitter::new().vertical();
        assert_eq!(s.orientation, SplitOrientation::Vertical);

        let s = Splitter::new().horizontal();
        assert_eq!(s.orientation, SplitOrientation::Horizontal);
    }

    #[test]
    fn test_splitter_pane_areas_horizontal() {
        let s = Splitter::new()
            .pane(Pane::new("left").ratio(0.5))
            .pane(Pane::new("right").ratio(0.5));
        let area = Rect::new(0, 0, 81, 24); // 81 = 40 + 1(splitter) + 40
        let areas = s.pane_areas(area);
        assert_eq!(areas.len(), 2);
        assert_eq!(areas[0].0, "left");
        assert_eq!(areas[1].0, "right");
        // Both should have roughly equal width
        assert!(areas[0].1.width > 0);
        assert!(areas[1].1.width > 0);
    }

    fn inside(inner: &Rect, outer: &Rect) -> bool {
        let end = |p: u16, len: u16| u32::from(p) + u32::from(len);
        inner.x >= outer.x
            && inner.y >= outer.y
            && end(inner.x, inner.width) <= end(outer.x, outer.width)
            && end(inner.y, inner.height) <= end(outer.y, outer.height)
    }

    // Default panes have `min_size` 5, so a few of them overrun a narrow
    // area; against the end of the coordinate space that used to overflow.
    #[test]
    fn test_splitter_pane_areas_stay_inside_an_area_at_the_coordinate_edge() {
        for orientation in [SplitOrientation::Horizontal, SplitOrientation::Vertical] {
            let mut s =
                Splitter::new().panes((0..20).map(|i| Pane::new(format!("p{i}"))).collect());
            s.orientation = orientation;
            for area in [
                Rect::new(u16::MAX - 5, 1, 5, 2),
                Rect::new(1, u16::MAX - 5, 2, 5),
                Rect::new(0, 0, 7, 3),
            ] {
                let areas = s.pane_areas(area);
                assert_eq!(areas.len(), 20);
                for (id, pane) in &areas {
                    assert!(inside(pane, &area), "{id} {pane:?} leaves {area:?}");
                }
            }
        }
    }

    #[test]
    fn test_splitter_pane_areas_empty() {
        let s = Splitter::new();
        let area = Rect::new(0, 0, 80, 24);
        let areas = s.pane_areas(area);
        assert!(areas.is_empty());
    }

    #[test]
    fn test_splitter_focus_navigation() {
        let mut s = Splitter::new()
            .pane(Pane::new("a"))
            .pane(Pane::new("b"))
            .pane(Pane::new("c"));
        assert_eq!(s.focused(), Some("a"));

        s.focus_next();
        assert_eq!(s.focused(), Some("b"));

        s.focus_next();
        assert_eq!(s.focused(), Some("c"));

        s.focus_next(); // wraps
        assert_eq!(s.focused(), Some("a"));

        s.focus_prev(); // wraps back
        assert_eq!(s.focused(), Some("c"));
    }
}
