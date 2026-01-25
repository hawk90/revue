//! Positioned widget for absolute positioning
//!
//! Allows placing widgets at specific coordinates within their parent area.

use crate::layout::Rect;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Position anchor point
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Anchor {
    /// Top-left corner (default)
    #[default]
    TopLeft,
    /// Top-center
    TopCenter,
    /// Top-right corner
    TopRight,
    /// Middle-left
    MiddleLeft,
    /// Center of the widget
    Center,
    /// Middle-right
    MiddleRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-center
    BottomCenter,
    /// Bottom-right corner
    BottomRight,
}

/// A widget that positions its child at specific coordinates
///
/// The position can be specified as:
/// - Absolute pixels from top-left
/// - Percentage of parent area
/// - Relative to different anchor points
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// // Position at absolute coordinates
/// let pos = Positioned::new(Text::new("Hello"))
///     .x(10)
///     .y(5);
///
/// // Position at center
/// let centered = Positioned::new(Text::new("Centered"))
///     .anchor(Anchor::Center)
///     .percent_x(50.0)
///     .percent_y(50.0);
/// ```
pub struct Positioned {
    child: Box<dyn View>,
    x: Option<i16>,
    y: Option<i16>,
    percent_x: Option<f32>,
    percent_y: Option<f32>,
    width: Option<u16>,
    height: Option<u16>,
    anchor: Anchor,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Positioned {
    /// Create a new positioned widget
    pub fn new<V: View + 'static>(child: V) -> Self {
        Self {
            child: Box::new(child),
            x: None,
            y: None,
            percent_x: None,
            percent_y: None,
            width: None,
            height: None,
            anchor: Anchor::default(),
            props: WidgetProps::new(),
        }
    }

    /// Set absolute X position
    pub fn x(mut self, x: i16) -> Self {
        self.x = Some(x);
        self.percent_x = None;
        self
    }

    /// Set absolute Y position
    pub fn y(mut self, y: i16) -> Self {
        self.y = Some(y);
        self.percent_y = None;
        self
    }

    /// Set both X and Y position
    pub fn at(self, x: i16, y: i16) -> Self {
        self.x(x).y(y)
    }

    /// Set X position as percentage of parent width
    pub fn percent_x(mut self, percent: f32) -> Self {
        self.percent_x = Some(percent);
        self.x = None;
        self
    }

    /// Set Y position as percentage of parent height
    pub fn percent_y(mut self, percent: f32) -> Self {
        self.percent_y = Some(percent);
        self.y = None;
        self
    }

    /// Set both positions as percentages
    pub fn percent(self, x: f32, y: f32) -> Self {
        self.percent_x(x).percent_y(y)
    }

    /// Set fixed width for the child
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Set fixed height for the child
    pub fn height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }

    /// Set both width and height
    pub fn size(self, width: u16, height: u16) -> Self {
        self.width(width).height(height)
    }

    /// Set the anchor point for positioning
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Calculate final position based on settings and parent area
    fn calculate_position(&self, parent: &Rect, child_width: u16, child_height: u16) -> (u16, u16) {
        // Calculate base position
        let base_x = if let Some(x) = self.x {
            if x >= 0 {
                parent.x.saturating_add(x as u16)
            } else {
                parent.x.saturating_sub((-x) as u16)
            }
        } else if let Some(percent) = self.percent_x {
            let offset = (parent.width as f32 * percent / 100.0) as u16;
            parent.x.saturating_add(offset)
        } else {
            parent.x
        };

        let base_y = if let Some(y) = self.y {
            if y >= 0 {
                parent.y.saturating_add(y as u16)
            } else {
                parent.y.saturating_sub((-y) as u16)
            }
        } else if let Some(percent) = self.percent_y {
            let offset = (parent.height as f32 * percent / 100.0) as u16;
            parent.y.saturating_add(offset)
        } else {
            parent.y
        };

        // Adjust for anchor point
        let (x, y) = match self.anchor {
            Anchor::TopLeft => (base_x, base_y),
            Anchor::TopCenter => (base_x.saturating_sub(child_width / 2), base_y),
            Anchor::TopRight => (base_x.saturating_sub(child_width), base_y),
            Anchor::MiddleLeft => (base_x, base_y.saturating_sub(child_height / 2)),
            Anchor::Center => (
                base_x.saturating_sub(child_width / 2),
                base_y.saturating_sub(child_height / 2),
            ),
            Anchor::MiddleRight => (
                base_x.saturating_sub(child_width),
                base_y.saturating_sub(child_height / 2),
            ),
            Anchor::BottomLeft => (base_x, base_y.saturating_sub(child_height)),
            Anchor::BottomCenter => (
                base_x.saturating_sub(child_width / 2),
                base_y.saturating_sub(child_height),
            ),
            Anchor::BottomRight => (
                base_x.saturating_sub(child_width),
                base_y.saturating_sub(child_height),
            ),
        };

        (x, y)
    }
}

impl View for Positioned {
    crate::impl_view_meta!("Positioned");

    fn render(&self, ctx: &mut RenderContext) {
        let parent = ctx.area;
        if parent.width == 0 || parent.height == 0 {
            return;
        }

        // Determine child size
        let child_width = self.width.unwrap_or(parent.width);
        let child_height = self.height.unwrap_or(parent.height);

        // Calculate position
        let (x, y) = self.calculate_position(&parent, child_width, child_height);

        // Create bounded child area
        let child_area = Rect::new(
            x.max(parent.x).min(parent.x + parent.width),
            y.max(parent.y).min(parent.y + parent.height),
            child_width.min(parent.x + parent.width - x.min(parent.x + parent.width)),
            child_height.min(parent.y + parent.height - y.min(parent.y + parent.height)),
        );

        // Render child in calculated area
        let mut child_ctx = RenderContext::new(ctx.buffer, child_area);
        self.child.render(&mut child_ctx);
    }
}

impl_styled_view!(Positioned);
impl_props_builders!(Positioned);

/// Create a positioned widget
pub fn positioned<V: View + 'static>(child: V) -> Positioned {
    Positioned::new(child)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::widget::Text;

    #[test]
    fn test_positioned_new() {
        let p = Positioned::new(Text::new("Test"));
        assert_eq!(p.x, None);
        assert_eq!(p.y, None);
        assert_eq!(p.anchor, Anchor::TopLeft);
    }

    #[test]
    fn test_positioned_absolute() {
        let p = Positioned::new(Text::new("Test")).x(10).y(5);

        assert_eq!(p.x, Some(10));
        assert_eq!(p.y, Some(5));
    }

    #[test]
    fn test_positioned_at() {
        let p = Positioned::new(Text::new("Test")).at(15, 20);

        assert_eq!(p.x, Some(15));
        assert_eq!(p.y, Some(20));
    }

    #[test]
    fn test_positioned_percent() {
        let p = Positioned::new(Text::new("Test"))
            .percent_x(50.0)
            .percent_y(25.0);

        assert_eq!(p.percent_x, Some(50.0));
        assert_eq!(p.percent_y, Some(25.0));
        assert_eq!(p.x, None);
        assert_eq!(p.y, None);
    }

    #[test]
    fn test_positioned_size() {
        let p = Positioned::new(Text::new("Test")).width(20).height(10);

        assert_eq!(p.width, Some(20));
        assert_eq!(p.height, Some(10));
    }

    #[test]
    fn test_positioned_anchor() {
        let p = Positioned::new(Text::new("Test")).anchor(Anchor::Center);

        assert_eq!(p.anchor, Anchor::Center);
    }

    #[test]
    fn test_positioned_render() {
        let p = Positioned::new(Text::new("Hello")).at(5, 2).size(10, 1);

        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        p.render(&mut ctx);
        // Text should be rendered at position (5, 2)
    }

    #[test]
    fn test_positioned_center() {
        let p = Positioned::new(Text::new("Centered"))
            .anchor(Anchor::Center)
            .percent(50.0, 50.0)
            .size(10, 1);

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        p.render(&mut ctx);
        // Text should be centered in the area
    }

    #[test]
    fn test_positioned_helper() {
        let p = positioned(Text::new("Test"));
        assert_eq!(p.x, None);
    }

    #[test]
    fn test_calculate_position_top_left() {
        let p = Positioned::new(Text::new("Test"))
            .at(10, 5)
            .anchor(Anchor::TopLeft);

        let parent = Rect::new(0, 0, 100, 50);
        let (x, y) = p.calculate_position(&parent, 20, 3);

        assert_eq!(x, 10);
        assert_eq!(y, 5);
    }

    #[test]
    fn test_calculate_position_center() {
        let p = Positioned::new(Text::new("Test"))
            .percent(50.0, 50.0)
            .anchor(Anchor::Center);

        let parent = Rect::new(0, 0, 100, 50);
        let (x, y) = p.calculate_position(&parent, 20, 4);

        // 50% of 100 = 50, minus half of 20 = 40
        assert_eq!(x, 40);
        // 50% of 50 = 25, minus half of 4 = 23
        assert_eq!(y, 23);
    }
}
