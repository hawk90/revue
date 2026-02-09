//! Focus indicator methods for RenderContext

use super::super::event::FocusStyle;
use crate::render::Cell;
use crate::style::Color;

impl RenderContext<'_> {
    /// Draw a focus ring around an area
    pub fn draw_focus_ring(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        color: Color,
        style: FocusStyle,
    ) {
        if w < 2 || h < 2 {
            return;
        }

        let (h_char, v_char, tl, tr, bl, br) = match style {
            FocusStyle::Solid => ('─', '│', '┌', '┐', '└', '┘'),
            FocusStyle::Rounded => ('─', '│', '╭', '╮', '╰', '╯'),
            FocusStyle::Double => ('═', '║', '╔', '╗', '╚', '╝'),
            FocusStyle::Dotted => ('╌', '╎', '┌', '┐', '└', '┘'),
            FocusStyle::Bold => ('━', '┃', '┏', '┓', '┗', '┛'),
            FocusStyle::Ascii => ('-', '|', '+', '+', '+', '+'),
        };

        self.draw_char(x, y, tl, color);
        self.draw_char(x + w - 1, y, tr, color);
        self.draw_char(x, y + h - 1, bl, color);
        self.draw_char(x + w - 1, y + h - 1, br, color);
        self.draw_hline(x + 1, y, w - 2, h_char, color);
        self.draw_hline(x + 1, y + h - 1, w - 2, h_char, color);
        self.draw_vline(x, y + 1, h - 2, v_char, color);
        self.draw_vline(x + w - 1, y + 1, h - 2, v_char, color);
    }

    /// Draw a focus ring with automatic style based on context
    pub fn draw_focus_ring_auto(&mut self, x: u16, y: u16, w: u16, h: u16, color: Color) {
        self.draw_focus_ring(x, y, w, h, color, FocusStyle::Rounded);
    }

    /// Draw a focus underline (for inline elements)
    pub fn draw_focus_underline(&mut self, x: u16, y: u16, w: u16, color: Color) {
        for i in 0..w {
            let cell = Cell::new('▔').fg(color);
            self.buffer.set(x + i, y, cell);
        }
    }

    /// Draw a focus indicator at a specific position
    pub fn draw_focus_marker(&mut self, x: u16, y: u16, color: Color) {
        self.draw_char(x, y, '▶', color);
    }

    /// Draw a focus indicator on the left side of an item
    pub fn draw_focus_marker_left(&mut self, y: u16, color: Color) {
        if self.area.x > 0 {
            self.draw_char(self.area.x - 1, y, '▶', color);
        } else {
            self.draw_char(self.area.x, y, '▶', color);
        }
    }

    /// Invert colors in a region (for high contrast focus indication)
    pub fn invert_colors(&mut self, x: u16, y: u16, w: u16, h: u16) {
        for dy in 0..h {
            for dx in 0..w {
                if let Some(cell) = self.buffer.get_mut(x + dx, y + dy) {
                    let old_fg = cell.fg;
                    let old_bg = cell.bg;
                    cell.fg = old_bg;
                    cell.bg = old_fg;
                }
            }
        }
    }

    /// Add reverse video effect to indicate focus
    pub fn draw_focus_reverse(&mut self, x: u16, y: u16, w: u16, h: u16) {
        self.invert_colors(x, y, w, h);
    }
}

use crate::widget::traits::render_context::RenderContext;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    // =========================================================================
    // draw_focus_ring tests
    // =========================================================================

    #[test]
    fn test_draw_focus_ring_min_size() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        // Should not panic with minimum size
        ctx.draw_focus_ring(0, 0, 2, 2, Color::CYAN, FocusStyle::Solid);
    }

    #[test]
    fn test_draw_focus_ring_too_small() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        // Width < 2, should return early
        ctx.draw_focus_ring(0, 0, 1, 5, Color::CYAN, FocusStyle::Solid);
    }

    #[test]
    fn test_draw_focus_ring_height_too_small() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        // Height < 2, should return early
        ctx.draw_focus_ring(0, 0, 5, 1, Color::CYAN, FocusStyle::Solid);
    }

    #[test]
    fn test_draw_focus_ring_solid() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        // Just verify it doesn't panic
        ctx.draw_focus_ring(0, 0, 5, 5, Color::CYAN, FocusStyle::Solid);
    }

    #[test]
    fn test_draw_focus_ring_rounded() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_ring(0, 0, 5, 5, Color::CYAN, FocusStyle::Rounded);
    }

    #[test]
    fn test_draw_focus_ring_double() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_ring(0, 0, 5, 5, Color::CYAN, FocusStyle::Double);
    }

    #[test]
    fn test_draw_focus_ring_dotted() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_ring(0, 0, 5, 5, Color::CYAN, FocusStyle::Dotted);
    }

    #[test]
    fn test_draw_focus_ring_bold() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_ring(0, 0, 5, 5, Color::CYAN, FocusStyle::Bold);
    }

    #[test]
    fn test_draw_focus_ring_ascii() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_ring(0, 0, 5, 5, Color::CYAN, FocusStyle::Ascii);
    }

    #[test]
    fn test_draw_focus_ring_offset() {
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_ring(5, 5, 8, 6, Color::CYAN, FocusStyle::Solid);
    }

    #[test]
    fn test_draw_focus_ring_large() {
        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(0, 0, 50, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_ring(0, 0, 40, 30, Color::CYAN, FocusStyle::Rounded);
    }

    // =========================================================================
    // draw_focus_ring_auto tests
    // =========================================================================

    #[test]
    fn test_draw_focus_ring_auto() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_ring_auto(0, 0, 5, 5, Color::CYAN);
    }

    // =========================================================================
    // draw_focus_underline tests
    // =========================================================================

    #[test]
    fn test_draw_focus_underline_zero_width() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_underline(0, 5, 0, Color::CYAN);
    }

    #[test]
    fn test_draw_focus_underline_single() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_underline(0, 5, 1, Color::CYAN);
    }

    #[test]
    fn test_draw_focus_underline_multiple() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_underline(0, 5, 5, Color::CYAN);
    }

    #[test]
    fn test_draw_focus_underline_offset() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_underline(10, 5, 5, Color::CYAN);
    }

    // =========================================================================
    // draw_focus_marker tests
    // =========================================================================

    #[test]
    fn test_draw_focus_marker() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_marker(0, 5, Color::CYAN);
    }

    #[test]
    fn test_draw_focus_marker_offset() {
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_marker(10, 10, Color::CYAN);
    }

    // =========================================================================
    // draw_focus_marker_left tests
    // =========================================================================

    #[test]
    fn test_draw_focus_marker_left_at_zero() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        // area.x is 0, so should draw at area.x
        ctx.draw_focus_marker_left(5, Color::CYAN);
    }

    #[test]
    fn test_draw_focus_marker_left_with_offset() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(5, 0, 15, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        // area.x is 5, so should draw at area.x - 1 = 4
        ctx.draw_focus_marker_left(5, Color::CYAN);
    }

    // =========================================================================
    // invert_colors tests
    // =========================================================================

    #[test]
    fn test_invert_colors_empty() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.invert_colors(0, 0, 0, 0);
    }

    #[test]
    fn test_invert_colors_single() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.invert_colors(0, 0, 1, 1);
    }

    #[test]
    fn test_invert_colors_region() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.invert_colors(2, 2, 5, 4);
    }

    #[test]
    fn test_invert_colors_full() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.invert_colors(0, 0, 10, 10);
    }

    // =========================================================================
    // draw_focus_reverse tests
    // =========================================================================

    #[test]
    fn test_draw_focus_reverse() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_reverse(0, 0, 5, 5);
    }

    #[test]
    fn test_draw_focus_reverse_offset() {
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_focus_reverse(5, 5, 8, 6);
    }
}
