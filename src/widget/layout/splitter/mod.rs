//! Splitter widget for resizable split panels
//!
//! Allows dividing an area into resizable panes with draggable dividers.

mod render;
mod two_pane;
mod types;

pub use two_pane::{HSplit, VSplit};
pub use types::{Pane, SplitOrientation, SplitterStyle};

use crate::event::Key;
use crate::layout::Rect;
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

    /// Get pane areas
    pub fn pane_areas(&self, area: Rect) -> Vec<(String, Rect)> {
        let mut areas = Vec::new();
        let visible_panes: Vec<_> = self.panes.iter().filter(|p| !p.collapsed).collect();

        if visible_panes.is_empty() {
            return areas;
        }

        let dividers = u16::try_from(visible_panes.len().saturating_sub(1)).unwrap_or(u16::MAX);
        let total_splitter_width = dividers.saturating_mul(self.splitter_width);
        let extent = match self.orientation {
            SplitOrientation::Horizontal => area.width,
            SplitOrientation::Vertical => area.height,
        };
        let available = extent.saturating_sub(total_splitter_width);

        // Normalize ratios
        let total_ratio: f32 = visible_panes.iter().map(|p| p.ratio).sum();
        let mut offset = 0u16;

        for (i, pane) in visible_panes.iter().enumerate() {
            let ratio = if total_ratio > 0.0 {
                pane.ratio / total_ratio
            } else {
                1.0 / visible_panes.len() as f32
            };
            let size = (available as f32 * ratio).clamp(0.0, available as f32);
            let mut size = size as u16;

            // Apply constraints
            size = size.max(pane.min_size);
            if pane.max_size > 0 {
                size = size.min(pane.max_size);
            }

            // Last pane takes the remaining space. `offset` already counts
            // the dividers placed so far, so measure it against the whole
            // extent, not against `available` (which has them taken out).
            // Panes whose minimum sizes overrun the extent are cut at its end
            // (and later ones get no room), so no pane leaves the area and
            // no coordinate runs past `u16::MAX`.
            let remaining = extent.saturating_sub(offset);
            if i == visible_panes.len() - 1 {
                size = remaining;
            }
            size = size.min(remaining);
            let start = offset.min(extent);

            let pane_area = match self.orientation {
                SplitOrientation::Horizontal => {
                    Rect::new(area.x.saturating_add(start), area.y, size, area.height)
                }
                SplitOrientation::Vertical => {
                    Rect::new(area.x, area.y.saturating_add(start), area.width, size)
                }
            };

            areas.push((pane.id.clone(), pane_area));
            offset = offset
                .saturating_add(size)
                .saturating_add(self.splitter_width);
        }

        areas
    }

    /// Get focused pane id
    pub fn focused(&self) -> Option<&str> {
        self.panes.get(self.focused_pane).map(|p| p.id.as_str())
    }

    /// Focus next pane
    pub fn focus_next(&mut self) {
        let visible: Vec<_> = self
            .panes
            .iter()
            .enumerate()
            .filter(|(_, p)| !p.collapsed)
            .map(|(i, _)| i)
            .collect();

        if let Some(pos) = visible.iter().position(|&i| i == self.focused_pane) {
            let next = (pos + 1) % visible.len();
            self.focused_pane = visible[next];
        }
    }

    /// Focus previous pane
    pub fn focus_prev(&mut self) {
        let visible: Vec<_> = self
            .panes
            .iter()
            .enumerate()
            .filter(|(_, p)| !p.collapsed)
            .map(|(i, _)| i)
            .collect();

        if let Some(pos) = visible.iter().position(|&i| i == self.focused_pane) {
            let prev = if pos == 0 { visible.len() - 1 } else { pos - 1 };
            self.focused_pane = visible[prev];
        }
    }

    /// Start resizing divider
    pub fn start_resize(&mut self, divider: usize) {
        if divider + 1 < self.panes.len() {
            self.active_divider = Some(divider);
        }
    }

    /// Stop resizing
    pub fn stop_resize(&mut self) {
        self.active_divider = None;
    }

    /// Resize by delta
    pub fn resize(&mut self, delta: i16) {
        if let Some(divider) = self.active_divider {
            if divider + 1 < self.panes.len() {
                let current_ratio = self.panes[divider].ratio;
                let next_ratio = self.panes[divider + 1].ratio;

                let change = delta as f32 * 0.01;
                let new_current = (current_ratio + change).clamp(0.1, 0.9);
                let new_next = (next_ratio - change).clamp(0.1, 0.9);

                self.panes[divider].ratio = new_current;
                self.panes[divider + 1].ratio = new_next;
            }
        }
    }

    /// Toggle collapse for pane
    pub fn toggle_pane(&mut self, index: usize) {
        if let Some(pane) = self.panes.get_mut(index) {
            pane.toggle_collapse();
        }
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Tab => {
                self.focus_next();
                true
            }
            Key::Left | Key::Char('h') if self.active_divider.is_some() => {
                self.resize(-5);
                true
            }
            Key::Right | Key::Char('l') if self.active_divider.is_some() => {
                self.resize(5);
                true
            }
            Key::Up | Key::Char('k') if self.active_divider.is_some() => {
                self.resize(-5);
                true
            }
            Key::Down | Key::Char('j') if self.active_divider.is_some() => {
                self.resize(5);
                true
            }
            Key::Enter if self.active_divider.is_some() => {
                self.stop_resize();
                true
            }
            Key::Escape if self.active_divider.is_some() => {
                self.stop_resize();
                true
            }
            _ => false,
        }
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
