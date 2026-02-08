//! Shape drawing methods for RenderContext

use crate::render::Cell;
use crate::style::Color;
use crate::utils::unicode::{char_width, display_width};

impl RenderContext<'_> {
    /// Draw a horizontal line
    pub fn draw_hline(&mut self, x: u16, y: u16, len: u16, ch: char, fg: Color) {
        for i in 0..len {
            self.draw_char(x + i, y, ch, fg);
        }
    }

    /// Draw a vertical line
    pub fn draw_vline(&mut self, x: u16, y: u16, len: u16, ch: char, fg: Color) {
        for i in 0..len {
            self.draw_char(x, y + i, ch, fg);
        }
    }

    /// Draw a box with rounded corners
    pub fn draw_box_rounded(&mut self, x: u16, y: u16, w: u16, h: u16, fg: Color) {
        if w < 2 || h < 2 {
            return;
        }
        self.draw_char(x, y, '╭', fg);
        self.draw_char(x + w - 1, y, '╮', fg);
        self.draw_char(x, y + h - 1, '╰', fg);
        self.draw_char(x + w - 1, y + h - 1, '╯', fg);
        self.draw_hline(x + 1, y, w - 2, '─', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a box without top border (for custom multi-color headers)
    pub fn draw_box_no_top(&mut self, x: u16, y: u16, w: u16, h: u16, fg: Color) {
        if w < 2 || h < 2 {
            return;
        }
        self.draw_char(x, y + h - 1, '╰', fg);
        self.draw_char(x + w - 1, y + h - 1, '╯', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a complete header line with corners for use with draw_box_no_top
    pub fn draw_header_line(
        &mut self,
        x: u16,
        y: u16,
        width: u16,
        parts: &[(&str, Color)],
        border_color: Color,
    ) {
        if width < 4 {
            return;
        }
        self.draw_text(x, y, "╭─", border_color);
        let mut pos = x + 2;
        for (text, color) in parts {
            self.draw_text(pos, y, text, *color);
            pos += display_width(text) as u16;
        }
        let end = x + width - 1;
        while pos < end {
            self.draw_char(pos, y, '─', border_color);
            pos += 1;
        }
        self.draw_char(end, y, '╮', border_color);
    }

    /// Draw a box with single border
    pub fn draw_box_single(&mut self, x: u16, y: u16, w: u16, h: u16, fg: Color) {
        if w < 2 || h < 2 {
            return;
        }
        self.draw_char(x, y, '┌', fg);
        self.draw_char(x + w - 1, y, '┐', fg);
        self.draw_char(x, y + h - 1, '└', fg);
        self.draw_char(x + w - 1, y + h - 1, '┘', fg);
        self.draw_hline(x + 1, y, w - 2, '─', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a box with double border
    pub fn draw_box_double(&mut self, x: u16, y: u16, w: u16, h: u16, fg: Color) {
        if w < 2 || h < 2 {
            return;
        }
        self.draw_char(x, y, '╔', fg);
        self.draw_char(x + w - 1, y, '╗', fg);
        self.draw_char(x, y + h - 1, '╚', fg);
        self.draw_char(x + w - 1, y + h - 1, '╝', fg);
        self.draw_hline(x + 1, y, w - 2, '═', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '═', fg);
        self.draw_vline(x, y + 1, h - 2, '║', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '║', fg);
    }

    /// Fill a rectangular area with a character
    pub fn fill(&mut self, x: u16, y: u16, w: u16, h: u16, ch: char, fg: Color) {
        for dy in 0..h {
            for dx in 0..w {
                self.draw_char(x + dx, y + dy, ch, fg);
            }
        }
    }

    /// Fill with background color
    pub fn fill_bg(&mut self, x: u16, y: u16, w: u16, h: u16, bg: Color) {
        for dy in 0..h {
            for dx in 0..w {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                self.buffer.set(x + dx, y + dy, cell);
            }
        }
    }

    /// Clear area (fill with spaces)
    pub fn clear(&mut self, x: u16, y: u16, w: u16, h: u16) {
        for dy in 0..h {
            for dx in 0..w {
                self.buffer.set(x + dx, y + dy, Cell::empty());
            }
        }
    }

    // =========================================================================
    // Box with title utilities
    // =========================================================================

    /// Draw a rounded box with a title on the top border
    pub fn draw_box_titled(&mut self, x: u16, y: u16, w: u16, h: u16, title: &str, fg: Color) {
        if w < 2 || h < 2 {
            return;
        }
        self.draw_char(x, y, '╭', fg);
        self.draw_char(x + w - 1, y, '╮', fg);
        self.draw_char(x, y + h - 1, '╰', fg);
        self.draw_char(x + w - 1, y + h - 1, '╯', fg);
        self.draw_top_border_with_title(x, y, w, title, '─', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a single-line box with a title
    pub fn draw_box_titled_single(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        title: &str,
        fg: Color,
    ) {
        if w < 2 || h < 2 {
            return;
        }
        self.draw_char(x, y, '┌', fg);
        self.draw_char(x + w - 1, y, '┐', fg);
        self.draw_char(x, y + h - 1, '└', fg);
        self.draw_char(x + w - 1, y + h - 1, '┘', fg);
        self.draw_top_border_with_title(x, y, w, title, '─', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a double-line box with a title
    pub fn draw_box_titled_double(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        title: &str,
        fg: Color,
    ) {
        if w < 2 || h < 2 {
            return;
        }
        self.draw_char(x, y, '╔', fg);
        self.draw_char(x + w - 1, y, '╗', fg);
        self.draw_char(x, y + h - 1, '╚', fg);
        self.draw_char(x + w - 1, y + h - 1, '╝', fg);
        self.draw_top_border_with_title(x, y, w, title, '═', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '═', fg);
        self.draw_vline(x, y + 1, h - 2, '║', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '║', fg);
    }

    /// Helper: Draw top border with embedded title using O(n) iterator
    fn draw_top_border_with_title(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        title: &str,
        border_char: char,
        fg: Color,
    ) {
        let title_start = 2u16;
        let border_end = w.saturating_sub(1);
        let mut title_chars = title.chars().peekable();
        let mut pos = 1u16;

        while pos < border_end {
            if pos >= title_start {
                if let Some(ch) = title_chars.next() {
                    let ch_width = char_width(ch) as u16;
                    if ch_width == 0 {
                        continue;
                    }
                    if pos + ch_width > border_end {
                        break;
                    }
                    self.draw_char(x + pos, y, ch, fg);
                    for i in 1..ch_width {
                        self.buffer.set(x + pos + i, y, Cell::continuation());
                    }
                    pos += ch_width;
                    continue;
                }
            }
            self.draw_char(x + pos, y, border_char, fg);
            pos += 1;
        }
    }
}

use crate::widget::traits::render_context::RenderContext;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::{RenderContext, View};

    // Test widget to create a render context
    #[allow(dead_code)]
    struct TestWidget;
    impl View for TestWidget {
        fn render(&self, _ctx: &mut RenderContext) {}
    }

    #[test]
    fn test_draw_hline() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not panic
        ctx.draw_hline(10, 5, 20, '-', Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_draw_vline() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ctx.draw_vline(10, 5, 10, '|', Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_draw_box_rounded() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ctx.draw_box_rounded(5, 5, 20, 10, Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_draw_box_rounded_too_small() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Width or height < 2 should return early
        ctx.draw_box_rounded(5, 5, 1, 10, Color::rgb(255, 255, 255));
        ctx.draw_box_rounded(5, 5, 10, 1, Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_draw_box_no_top() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ctx.draw_box_no_top(5, 5, 20, 10, Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_draw_header_line() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let parts = &[("Title", Color::rgb(255, 0, 0))];
        ctx.draw_header_line(5, 5, 30, parts, Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_draw_header_line_multiple_parts() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let parts = &[
            ("A", Color::rgb(255, 0, 0)),
            ("B", Color::rgb(0, 255, 0)),
            ("C", Color::rgb(0, 0, 255)),
        ];
        ctx.draw_header_line(5, 5, 40, parts, Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_draw_header_line_too_small() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let parts = &[("Title", Color::rgb(255, 0, 0))];
        // Width < 4 should return early
        ctx.draw_header_line(5, 5, 3, parts, Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_draw_box_single() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ctx.draw_box_single(5, 5, 20, 10, Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_draw_box_double() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ctx.draw_box_double(5, 5, 20, 10, Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_fill() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ctx.fill(10, 10, 5, 3, '*', Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_fill_bg() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ctx.fill_bg(10, 10, 5, 3, Color::rgb(100, 100, 100));
    }

    #[test]
    fn test_clear() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ctx.clear(10, 10, 5, 3);
    }

    #[test]
    fn test_draw_box_titled() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ctx.draw_box_titled(5, 5, 30, 10, "Title", Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_draw_box_titled_empty_title() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ctx.draw_box_titled(5, 5, 30, 10, "", Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_draw_box_titled_unicode() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ctx.draw_box_titled(5, 5, 30, 10, "标题", Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_draw_box_titled_single() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ctx.draw_box_titled_single(5, 5, 30, 10, "Title", Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_draw_box_titled_double() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ctx.draw_box_titled_double(5, 5, 30, 10, "Title", Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_fill_zero_area() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Zero width/height should not panic
        ctx.fill(10, 10, 0, 0, '*', Color::rgb(255, 255, 255));
        ctx.fill_bg(10, 10, 0, 0, Color::rgb(100, 100, 100));
        ctx.clear(10, 10, 0, 0);
    }

    #[test]
    fn test_draw_lines_zero_length() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        ctx.draw_hline(10, 5, 0, '-', Color::rgb(255, 255, 255));
        ctx.draw_vline(10, 5, 0, '|', Color::rgb(255, 255, 255));
    }
}
