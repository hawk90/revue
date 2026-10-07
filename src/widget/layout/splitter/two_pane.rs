//! Two-pane horizontal and vertical splits

use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::DARK_GRAY;
use crate::widget::traits::{RenderContext, View};

/// Two-pane horizontal split
#[derive(Clone)]
pub struct HSplit {
    /// Left pane ratio
    pub ratio: f32,
    /// Minimum left width
    pub min_left: u16,
    /// Minimum right width
    pub min_right: u16,
    /// Splitter visible
    pub show_splitter: bool,
    /// Splitter color
    pub color: Color,
}

impl HSplit {
    /// Create new horizontal split
    pub fn new(ratio: f32) -> Self {
        Self {
            ratio: ratio.clamp(0.1, 0.9),
            min_left: 5,
            min_right: 5,
            show_splitter: true,
            color: DARK_GRAY,
        }
    }

    /// Set minimum widths
    pub fn min_widths(mut self, left: u16, right: u16) -> Self {
        self.min_left = left;
        self.min_right = right;
        self
    }

    /// Hide splitter
    pub fn hide_splitter(mut self) -> Self {
        self.show_splitter = false;
        self
    }

    /// Get left and right areas
    pub fn areas(&self, area: Rect) -> (Rect, Rect) {
        let splitter_width = if self.show_splitter { 1 } else { 0 };
        let available = area.width.saturating_sub(splitter_width);

        let left_width = (available as f32 * self.ratio).clamp(0.0, available as f32);
        let mut left_width = left_width as u16;
        left_width = left_width.max(self.min_left);
        left_width = left_width.min(available.saturating_sub(self.min_right));

        let right_width = available.saturating_sub(left_width);

        let left = Rect::new(area.x, area.y, left_width, area.height);
        let right = Rect::new(
            area.x + left_width + splitter_width,
            area.y,
            right_width,
            area.height,
        );

        (left, right)
    }
}

impl View for HSplit {
    fn render(&self, ctx: &mut RenderContext) {
        if !self.show_splitter {
            return;
        }

        let (left, _) = self.areas(ctx.area);
        let x = left.x + left.width - ctx.area.x;

        for y in 0..ctx.area.height {
            let mut cell = Cell::new('│');
            cell.fg = Some(self.color);
            ctx.set(x, y, cell);
        }
    }
}

/// Two-pane vertical split
#[derive(Clone)]
pub struct VSplit {
    /// Top pane ratio
    pub ratio: f32,
    /// Minimum top height
    pub min_top: u16,
    /// Minimum bottom height
    pub min_bottom: u16,
    /// Splitter visible
    pub show_splitter: bool,
    /// Splitter color
    pub color: Color,
}

impl VSplit {
    /// Create new vertical split
    pub fn new(ratio: f32) -> Self {
        Self {
            ratio: ratio.clamp(0.1, 0.9),
            min_top: 3,
            min_bottom: 3,
            show_splitter: true,
            color: DARK_GRAY,
        }
    }

    /// Get top and bottom areas
    pub fn areas(&self, area: Rect) -> (Rect, Rect) {
        let splitter_height = if self.show_splitter { 1 } else { 0 };
        let available = area.height.saturating_sub(splitter_height);

        let top_height = (available as f32 * self.ratio).clamp(0.0, available as f32);
        let mut top_height = top_height as u16;
        top_height = top_height.max(self.min_top);
        top_height = top_height.min(available.saturating_sub(self.min_bottom));

        let bottom_height = available.saturating_sub(top_height);

        let top = Rect::new(area.x, area.y, area.width, top_height);
        let bottom = Rect::new(
            area.x,
            area.y + top_height + splitter_height,
            area.width,
            bottom_height,
        );

        (top, bottom)
    }
}

impl View for VSplit {
    fn render(&self, ctx: &mut RenderContext) {
        if !self.show_splitter {
            return;
        }

        let (top, _) = self.areas(ctx.area);
        let y = top.y + top.height - ctx.area.y;

        for x in 0..ctx.area.width {
            let mut cell = Cell::new('─');
            cell.fg = Some(self.color);
            ctx.set(x, y, cell);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{hsplit, vsplit};
    use super::*;

    #[test]
    fn test_hsplit_new() {
        let h = hsplit(0.3);
        assert_eq!(h.ratio, 0.3);
    }

    #[test]
    fn test_vsplit_new() {
        let v = vsplit(0.6);
        assert_eq!(v.ratio, 0.6);
    }

    #[test]
    fn test_hsplit_areas() {
        let h = HSplit::new(0.5);
        let area = Rect::new(0, 0, 81, 24);
        let (left, right) = h.areas(area);
        assert!(left.width > 0);
        assert!(right.width > 0);
        assert_eq!(left.height, 24);
        assert_eq!(right.height, 24);
    }

    #[test]
    fn test_vsplit_areas() {
        let v = VSplit::new(0.5);
        let area = Rect::new(0, 0, 80, 25);
        let (top, bottom) = v.areas(area);
        assert!(top.height > 0);
        assert!(bottom.height > 0);
        assert_eq!(top.width, 80);
        assert_eq!(bottom.width, 80);
    }
}
