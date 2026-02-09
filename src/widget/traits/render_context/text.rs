//! Text drawing methods for RenderContext

use crate::render::Cell;
use crate::style::Color;
use crate::utils::unicode::{char_width, display_width};

impl RenderContext<'_> {
    /// Helper: Draw text with custom cell styling, handling wide characters correctly.
    pub(super) fn draw_text_with_style<F>(&mut self, x: u16, y: u16, text: &str, mut make_cell: F)
    where
        F: FnMut(char) -> Cell,
    {
        let mut offset = 0u16;
        for ch in text.chars() {
            let width = char_width(ch) as u16;
            if width == 0 {
                continue;
            }
            self.buffer.set(x.saturating_add(offset), y, make_cell(ch));
            for i in 1..width {
                self.buffer
                    .set(x.saturating_add(offset + i), y, Cell::continuation());
            }
            offset = offset.saturating_add(width);
        }
    }

    /// Helper: Draw text clipped to max_width, handling wide characters correctly.
    pub(super) fn draw_text_clipped_with_style<F>(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        max_width: u16,
        mut make_cell: F,
    ) where
        F: FnMut(char) -> Cell,
    {
        let mut offset = 0u16;
        for ch in text.chars() {
            let width = char_width(ch) as u16;
            if width == 0 {
                continue;
            }
            if offset.saturating_add(width) > max_width {
                break;
            }
            self.buffer.set(x.saturating_add(offset), y, make_cell(ch));
            for i in 1..width {
                self.buffer
                    .set(x.saturating_add(offset + i), y, Cell::continuation());
            }
            offset = offset.saturating_add(width);
        }
    }

    /// Draw a single character at position
    #[inline]
    pub fn draw_char(&mut self, x: u16, y: u16, ch: char, fg: Color) {
        let cell = Cell::new(ch).fg(fg);
        self.buffer.set(x, y, cell);
    }

    /// Draw a character with background color
    #[inline]
    pub fn draw_char_bg(&mut self, x: u16, y: u16, ch: char, fg: Color, bg: Color) {
        let cell = Cell::new(ch).fg(fg).bg(bg);
        self.buffer.set(x, y, cell);
    }

    /// Draw a bold character
    #[inline]
    pub fn draw_char_bold(&mut self, x: u16, y: u16, ch: char, fg: Color) {
        let cell = Cell::new(ch).fg(fg).bold();
        self.buffer.set(x, y, cell);
    }

    /// Draw text at position
    pub fn draw_text(&mut self, x: u16, y: u16, text: &str, fg: Color) {
        self.draw_text_with_style(x, y, text, |ch| Cell::new(ch).fg(fg));
    }

    /// Draw text with background color
    pub fn draw_text_bg(&mut self, x: u16, y: u16, text: &str, fg: Color, bg: Color) {
        self.draw_text_with_style(x, y, text, |ch| Cell::new(ch).fg(fg).bg(bg));
    }

    /// Draw bold text
    pub fn draw_text_bold(&mut self, x: u16, y: u16, text: &str, fg: Color) {
        self.draw_text_with_style(x, y, text, |ch| Cell::new(ch).fg(fg).bold());
    }

    /// Draw bold text with background color
    pub fn draw_text_bg_bold(&mut self, x: u16, y: u16, text: &str, fg: Color, bg: Color) {
        self.draw_text_with_style(x, y, text, |ch| Cell::new(ch).fg(fg).bg(bg).bold());
    }

    /// Draw text clipped to max_width (stops drawing at boundary)
    pub fn draw_text_clipped(&mut self, x: u16, y: u16, text: &str, fg: Color, max_width: u16) {
        self.draw_text_clipped_with_style(x, y, text, max_width, |ch| Cell::new(ch).fg(fg));
    }

    /// Draw bold text clipped to max_width
    pub fn draw_text_clipped_bold(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        fg: Color,
        max_width: u16,
    ) {
        self.draw_text_clipped_with_style(x, y, text, max_width, |ch| Cell::new(ch).fg(fg).bold());
    }

    /// Draw dimmed text
    pub fn draw_text_dim(&mut self, x: u16, y: u16, text: &str, fg: Color) {
        self.draw_text_with_style(x, y, text, |ch| Cell::new(ch).fg(fg).dim());
    }

    /// Draw italic text
    pub fn draw_text_italic(&mut self, x: u16, y: u16, text: &str, fg: Color) {
        self.draw_text_with_style(x, y, text, |ch| Cell::new(ch).fg(fg).italic());
    }

    /// Draw underlined text
    pub fn draw_text_underline(&mut self, x: u16, y: u16, text: &str, fg: Color) {
        self.draw_text_with_style(x, y, text, |ch| Cell::new(ch).fg(fg).underline());
    }

    /// Draw text centered within a given width
    pub fn draw_text_centered(&mut self, x: u16, y: u16, width: u16, text: &str, fg: Color) {
        let text_width = display_width(text) as u16;
        let start_x = if text_width >= width {
            x
        } else {
            x + (width - text_width) / 2
        };
        self.draw_text_clipped(start_x, y, text, fg, width);
    }

    /// Draw text right-aligned within a given width
    pub fn draw_text_right(&mut self, x: u16, y: u16, width: u16, text: &str, fg: Color) {
        let text_width = display_width(text) as u16;
        let start_x = if text_width >= width {
            x
        } else {
            x + width - text_width
        };
        self.draw_text_clipped(start_x, y, text, fg, width);
    }
}

use crate::widget::traits::render_context::RenderContext;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    // =========================================================================
    // draw_char tests
    // =========================================================================

    #[test]
    fn test_draw_char() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_char(0, 0, 'A', Color::WHITE);
    }

    #[test]
    fn test_draw_char_offset() {
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_char(10, 10, 'B', Color::CYAN);
    }

    #[test]
    fn test_draw_char_wide() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_char(0, 0, '你', Color::WHITE);
    }

    // =========================================================================
    // draw_char_bg tests
    // =========================================================================

    #[test]
    fn test_draw_char_bg() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_char_bg(0, 0, 'A', Color::WHITE, Color::BLACK);
    }

    #[test]
    fn test_draw_char_bg_offset() {
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_char_bg(10, 10, 'B', Color::CYAN, Color::BLUE);
    }

    // =========================================================================
    // draw_char_bold tests
    // =========================================================================

    #[test]
    fn test_draw_char_bold() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_char_bold(0, 0, 'A', Color::WHITE);
    }

    #[test]
    fn test_draw_char_bold_offset() {
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_char_bold(10, 10, 'B', Color::CYAN);
    }

    // =========================================================================
    // draw_text tests
    // =========================================================================

    #[test]
    fn test_draw_text_empty() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text(0, 0, "", Color::WHITE);
    }

    #[test]
    fn test_draw_text_single() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text(0, 0, "A", Color::WHITE);
    }

    #[test]
    fn test_draw_text_multiple() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text(0, 0, "Hello", Color::WHITE);
    }

    #[test]
    fn test_draw_text_wide() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text(0, 0, "你好", Color::WHITE);
    }

    #[test]
    fn test_draw_text_mixed() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text(0, 0, "Hello你好", Color::WHITE);
    }

    #[test]
    fn test_draw_text_offset() {
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text(10, 10, "Test", Color::CYAN);
    }

    // =========================================================================
    // draw_text_bg tests
    // =========================================================================

    #[test]
    fn test_draw_text_bg() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_bg(0, 0, "Hello", Color::WHITE, Color::BLACK);
    }

    #[test]
    fn test_draw_text_bg_empty() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_bg(0, 0, "", Color::WHITE, Color::BLACK);
    }

    // =========================================================================
    // draw_text_bold tests
    // =========================================================================

    #[test]
    fn test_draw_text_bold() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_bold(0, 0, "Hello", Color::WHITE);
    }

    #[test]
    fn test_draw_text_bold_empty() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_bold(0, 0, "", Color::WHITE);
    }

    // =========================================================================
    // draw_text_bg_bold tests
    // =========================================================================

    #[test]
    fn test_draw_text_bg_bold() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_bg_bold(0, 0, "Hello", Color::WHITE, Color::BLACK);
    }

    #[test]
    fn test_draw_text_bg_bold_empty() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_bg_bold(0, 0, "", Color::WHITE, Color::BLACK);
    }

    // =========================================================================
    // draw_text_clipped tests
    // =========================================================================

    #[test]
    fn test_draw_text_clipped_empty() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_clipped(0, 0, "", Color::WHITE, 10);
    }

    #[test]
    fn test_draw_text_clipped_fit() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_clipped(0, 0, "Hello", Color::WHITE, 10);
    }

    #[test]
    fn test_draw_text_clipped_truncate() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_clipped(0, 0, "Hello World", Color::WHITE, 5);
    }

    #[test]
    fn test_draw_text_clipped_wide() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_clipped(0, 0, "你好世界", Color::WHITE, 5);
    }

    // =========================================================================
    // draw_text_clipped_bold tests
    // =========================================================================

    #[test]
    fn test_draw_text_clipped_bold() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_clipped_bold(0, 0, "Hello", Color::WHITE, 10);
    }

    #[test]
    fn test_draw_text_clipped_bold_truncate() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_clipped_bold(0, 0, "Hello World", Color::WHITE, 5);
    }

    // =========================================================================
    // draw_text_dim tests
    // =========================================================================

    #[test]
    fn test_draw_text_dim() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_dim(0, 0, "Hello", Color::WHITE);
    }

    #[test]
    fn test_draw_text_dim_empty() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_dim(0, 0, "", Color::WHITE);
    }

    // =========================================================================
    // draw_text_italic tests
    // =========================================================================

    #[test]
    fn test_draw_text_italic() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_italic(0, 0, "Hello", Color::WHITE);
    }

    #[test]
    fn test_draw_text_italic_empty() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_italic(0, 0, "", Color::WHITE);
    }

    // =========================================================================
    // draw_text_underline tests
    // =========================================================================

    #[test]
    fn test_draw_text_underline() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_underline(0, 0, "Hello", Color::WHITE);
    }

    #[test]
    fn test_draw_text_underline_empty() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_underline(0, 0, "", Color::WHITE);
    }

    // =========================================================================
    // draw_text_centered tests
    // =========================================================================

    #[test]
    fn test_draw_text_centered_exact_fit() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_centered(0, 0, 5, "Hello", Color::WHITE);
    }

    #[test]
    fn test_draw_text_centered_short() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_centered(0, 0, 10, "Hi", Color::WHITE);
    }

    #[test]
    fn test_draw_text_centered_long() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_centered(0, 0, 3, "Hello", Color::WHITE);
    }

    #[test]
    fn test_draw_text_centered_empty() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_centered(0, 0, 10, "", Color::WHITE);
    }

    // =========================================================================
    // draw_text_right tests
    // =========================================================================

    #[test]
    fn test_draw_text_right_exact_fit() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_right(0, 0, 5, "Hello", Color::WHITE);
    }

    #[test]
    fn test_draw_text_right_short() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_right(0, 0, 10, "Hi", Color::WHITE);
    }

    #[test]
    fn test_draw_text_right_long() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_right(0, 0, 3, "Hello", Color::WHITE);
    }

    #[test]
    fn test_draw_text_right_empty() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_right(0, 0, 10, "", Color::WHITE);
    }
}
