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
    /// Minimum width constraint (0 = no constraint)
    min_width: u16,
    /// Minimum height constraint (0 = no constraint)
    min_height: u16,
    /// Maximum width constraint (0 = no constraint)
    max_width: u16,
    /// Maximum height constraint (0 = no constraint)
    max_height: u16,
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
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0,
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

    /// Set minimum width constraint
    pub fn min_width(mut self, width: u16) -> Self {
        self.min_width = width;
        self
    }

    /// Set minimum height constraint
    pub fn min_height(mut self, height: u16) -> Self {
        self.min_height = height;
        self
    }

    /// Set maximum width constraint (0 = no limit)
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = width;
        self
    }

    /// Set maximum height constraint (0 = no limit)
    pub fn max_height(mut self, height: u16) -> Self {
        self.max_height = height;
        self
    }

    /// Set both min width and height
    pub fn min_size(self, width: u16, height: u16) -> Self {
        self.min_width(width).min_height(height)
    }

    /// Set both max width and height (0 = no limit)
    pub fn max_size(self, width: u16, height: u16) -> Self {
        self.max_width(width).max_height(height)
    }

    /// Set all size constraints at once
    pub fn constrain(self, min_w: u16, min_h: u16, max_w: u16, max_h: u16) -> Self {
        self.min_width(min_w)
            .min_height(min_h)
            .max_width(max_w)
            .max_height(max_h)
    }

    /// Apply size constraints to the available area
    fn apply_constraints(&self, area: Rect) -> Rect {
        let eff_max_w = if self.max_width > 0 {
            self.max_width.max(self.min_width)
        } else {
            u16::MAX
        };
        let eff_max_h = if self.max_height > 0 {
            self.max_height.max(self.min_height)
        } else {
            u16::MAX
        };
        let width = area.width.clamp(self.min_width, eff_max_w);
        let height = area.height.clamp(self.min_height, eff_max_h);

        Rect::new(area.x, area.y, width, height)
    }

    /// Calculate final position in relative coordinates based on settings and parent dimensions
    fn calculate_position_relative(
        &self,
        parent_width: u16,
        parent_height: u16,
        child_width: u16,
        child_height: u16,
    ) -> (u16, u16) {
        // Calculate base position (relative to parent top-left)
        let base_x = if let Some(x) = self.x {
            if x >= 0 {
                x as u16
            } else {
                0u16 // Can't go negative in relative coords
            }
        } else if let Some(percent) = self.percent_x {
            (parent_width as f32 * percent / 100.0)
                .max(0.0)
                .min(parent_width as f32) as u16
        } else {
            0
        };

        let base_y = if let Some(y) = self.y {
            if y >= 0 {
                y as u16
            } else {
                0u16
            }
        } else if let Some(percent) = self.percent_y {
            (parent_height as f32 * percent / 100.0)
                .max(0.0)
                .min(parent_height as f32) as u16
        } else {
            0
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
        let parent = self.apply_constraints(ctx.area);
        if parent.width == 0 || parent.height == 0 {
            return;
        }

        // Determine child size
        let child_width = self.width.unwrap_or(parent.width);
        let child_height = self.height.unwrap_or(parent.height);

        // Calculate position in relative coordinates
        let (rel_x, rel_y) = self.calculate_position_relative(
            parent.width,
            parent.height,
            child_width,
            child_height,
        );

        // Create bounded child area (clamp to parent bounds)
        let clamped_x = rel_x.min(parent.width);
        let clamped_y = rel_y.min(parent.height);
        let bounded_w = child_width.min(parent.width.saturating_sub(clamped_x));
        let bounded_h = child_height.min(parent.height.saturating_sub(clamped_y));

        let child_area = ctx.sub_area(clamped_x, clamped_y, bounded_w, bounded_h);

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
