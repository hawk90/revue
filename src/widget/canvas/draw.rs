//! Character-based drawing context

use crate::layout::Rect;
use crate::render::{Buffer, Cell, Modifier};
use crate::style::Color;

/// A context for drawing on a canvas (character-based)
pub struct DrawContext<'a> {
    pub(super) buffer: &'a mut Buffer,
    pub(super) area: Rect,
}

impl<'a> DrawContext<'a> {
    /// Create a new draw context
    pub fn new(buffer: &'a mut Buffer, area: Rect) -> Self {
        Self { buffer, area }
    }

    /// Get canvas width
    pub fn width(&self) -> u16 {
        self.area.width
    }

    /// Get canvas height
    pub fn height(&self) -> u16 {
        self.area.height
    }

    /// Get canvas area
    pub fn area(&self) -> Rect {
        self.area
    }

    /// Set a character at position
    pub fn set(&mut self, x: u16, y: u16, ch: char) {
        let abs_x = self.area.x + x;
        let abs_y = self.area.y + y;
        if x < self.area.width && y < self.area.height {
            self.buffer.set(abs_x, abs_y, Cell::new(ch));
        }
    }

    /// Set a character with style at position
    pub fn set_styled(&mut self, x: u16, y: u16, ch: char, fg: Option<Color>, bg: Option<Color>) {
        let abs_x = self.area.x + x;
        let abs_y = self.area.y + y;
        if x < self.area.width && y < self.area.height {
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            self.buffer.set(abs_x, abs_y, cell);
        }
    }

    /// Set a cell at position
    pub fn set_cell(&mut self, x: u16, y: u16, cell: Cell) {
        let abs_x = self.area.x + x;
        let abs_y = self.area.y + y;
        if x < self.area.width && y < self.area.height {
            self.buffer.set(abs_x, abs_y, cell);
        }
    }

    /// Draw a horizontal line
    pub fn hline(&mut self, x: u16, y: u16, length: u16, ch: char, fg: Option<Color>) {
        for i in 0..length {
            if x + i < self.area.width {
                self.set_styled(x + i, y, ch, fg, None);
            }
        }
    }

    /// Draw a vertical line
    pub fn vline(&mut self, x: u16, y: u16, length: u16, ch: char, fg: Option<Color>) {
        for i in 0..length {
            if y + i < self.area.height {
                self.set_styled(x, y + i, ch, fg, None);
            }
        }
    }

    /// Draw a rectangle outline
    pub fn rect(&mut self, x: u16, y: u16, width: u16, height: u16, fg: Option<Color>) {
        if width == 0 || height == 0 {
            return;
        }

        // Top and bottom
        self.hline(x, y, width, '─', fg);
        self.hline(x, y + height - 1, width, '─', fg);

        // Left and right
        self.vline(x, y, height, '│', fg);
        self.vline(x + width - 1, y, height, '│', fg);

        // Corners
        self.set_styled(x, y, '┌', fg, None);
        self.set_styled(x + width - 1, y, '┐', fg, None);
        self.set_styled(x, y + height - 1, '└', fg, None);
        self.set_styled(x + width - 1, y + height - 1, '┘', fg, None);
    }

    /// Fill a rectangle
    pub fn fill_rect(&mut self, rect: Rect, ch: char, fg: Option<Color>, bg: Option<Color>) {
        for dy in 0..rect.height {
            for dx in 0..rect.width {
                if rect.x + dx < self.area.width && rect.y + dy < self.area.height {
                    self.set_styled(rect.x + dx, rect.y + dy, ch, fg, bg);
                }
            }
        }
    }

    /// Draw a filled bar (for Gantt charts, progress bars, etc.)
    pub fn bar(&mut self, x: u16, y: u16, width: u16, fg: Color, bg: Option<Color>) {
        for i in 0..width {
            if x + i < self.area.width {
                let mut cell = Cell::new('█');
                cell.fg = Some(fg);
                cell.bg = bg;
                self.set_cell(x + i, y, cell);
            }
        }
    }

    /// Draw a partial bar (for fractional values)
    pub fn partial_bar(&mut self, x: u16, y: u16, width: f32, fg: Color) {
        let full_blocks = width.floor() as u16;
        let partial = width.fract();

        // Full blocks
        self.bar(x, y, full_blocks, fg, None);

        // Partial block
        if partial > 0.0 && x + full_blocks < self.area.width {
            let partial_char = match (partial * 8.0).round() as u8 {
                1 => '▏',
                2 => '▎',
                3 => '▍',
                4 => '▌',
                5 => '▋',
                6 => '▊',
                7 => '▉',
                _ => ' ',
            };
            self.set_styled(x + full_blocks, y, partial_char, Some(fg), None);
        }
    }

    /// Draw text at position
    pub fn text(&mut self, x: u16, y: u16, s: &str, fg: Option<Color>) {
        for (i, ch) in s.chars().enumerate() {
            let pos_x = x + i as u16;
            if pos_x < self.area.width {
                self.set_styled(pos_x, y, ch, fg, None);
            }
        }
    }

    /// Draw bold text at position
    pub fn text_bold(&mut self, x: u16, y: u16, s: &str, fg: Option<Color>) {
        for (i, ch) in s.chars().enumerate() {
            let pos_x = x + i as u16;
            if pos_x < self.area.width {
                let abs_x = self.area.x + pos_x;
                let abs_y = self.area.y + y;
                let mut cell = Cell::new(ch);
                cell.fg = fg;
                cell.modifier = Modifier::BOLD;
                self.buffer.set(abs_x, abs_y, cell);
            }
        }
    }

    /// Clear the canvas
    pub fn clear(&mut self) {
        self.fill_rect(
            Rect::new(0, 0, self.area.width, self.area.height),
            ' ',
            None,
            None,
        );
    }

    /// Draw a point/dot
    pub fn point(&mut self, x: u16, y: u16, fg: Color) {
        self.set_styled(x, y, '●', Some(fg), None);
    }

    /// Draw a line between two points (Bresenham's algorithm)
    pub fn line(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, ch: char, fg: Option<Color>) {
        let dx = (x1 as i32 - x0 as i32).abs();
        let dy = -(y1 as i32 - y0 as i32).abs();
        let sx: i32 = if x0 < x1 { 1 } else { -1 };
        let sy: i32 = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x0 as i32;
        let mut y = y0 as i32;

        loop {
            if x >= 0 && y >= 0 {
                self.set_styled(x as u16, y as u16, ch, fg, None);
            }

            if x == x1 as i32 && y == y1 as i32 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;

    // =========================================================================
    // DrawContext::new tests
    // =========================================================================

    #[test]
    fn test_draw_context_new() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let ctx = DrawContext::new(&mut buffer, area);
        assert_eq!(ctx.width(), 10);
        assert_eq!(ctx.height(), 10);
    }

    // =========================================================================
    // width and height tests
    // =========================================================================

    #[test]
    fn test_draw_context_width() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let ctx = DrawContext::new(&mut buffer, area);
        assert_eq!(ctx.width(), 20);
    }

    #[test]
    fn test_draw_context_height() {
        let mut buffer = Buffer::new(10, 20);
        let area = Rect::new(0, 0, 10, 20);
        let ctx = DrawContext::new(&mut buffer, area);
        assert_eq!(ctx.height(), 20);
    }

    #[test]
    fn test_draw_context_area() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(5, 5, 10, 10);
        let ctx = DrawContext::new(&mut buffer, area);
        assert_eq!(ctx.area(), area);
    }

    // =========================================================================
    // set tests
    // =========================================================================

    #[test]
    fn test_draw_context_set_char() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.set(5, 5, 'X');
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_set_out_of_bounds() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.set(15, 15, 'X');
        // Should not panic, just ignore
    }

    #[test]
    fn test_draw_context_set_at_boundary() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.set(0, 0, 'A');
        ctx.set(9, 9, 'B');
        // Just verify it doesn't panic
    }

    // =========================================================================
    // set_styled tests
    // =========================================================================

    #[test]
    fn test_draw_context_set_styled() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.set_styled(5, 5, 'X', Some(Color::RED), Some(Color::BLUE));
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_set_styled_no_color() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.set_styled(5, 5, 'X', None, None);
        // Just verify it doesn't panic
    }

    // =========================================================================
    // set_cell tests
    // =========================================================================

    #[test]
    fn test_draw_context_set_cell() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        let cell = Cell::new('X');
        ctx.set_cell(5, 5, cell);
        // Just verify it doesn't panic
    }

    // =========================================================================
    // hline tests
    // =========================================================================

    #[test]
    fn test_draw_context_hline() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.hline(2, 5, 10, '─', Some(Color::WHITE));
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_hline_zero_length() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.hline(5, 5, 0, '─', Some(Color::WHITE));
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_hline_truncated() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.hline(5, 5, 20, '─', Some(Color::WHITE));
        // Should be truncated to fit
    }

    // =========================================================================
    // vline tests
    // =========================================================================

    #[test]
    fn test_draw_context_vline() {
        let mut buffer = Buffer::new(10, 20);
        let area = Rect::new(0, 0, 10, 20);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.vline(5, 2, 10, '│', Some(Color::WHITE));
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_vline_zero_length() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.vline(5, 5, 0, '│', Some(Color::WHITE));
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_vline_truncated() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.vline(5, 5, 20, '│', Some(Color::WHITE));
        // Should be truncated to fit
    }

    // =========================================================================
    // rect tests
    // =========================================================================

    #[test]
    fn test_draw_context_rect() {
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.rect(2, 2, 10, 8, Some(Color::WHITE));
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_rect_zero_size() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.rect(5, 5, 0, 0, Some(Color::WHITE));
        // Just verify it doesn't panic (should return early)
    }

    // =========================================================================
    // fill_rect tests
    // =========================================================================

    #[test]
    fn test_draw_context_fill_rect() {
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = DrawContext::new(&mut buffer, area);
        let rect = Rect::new(2, 2, 10, 8);
        ctx.fill_rect(rect, 'X', Some(Color::WHITE), Some(Color::BLACK));
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_fill_rect_zero_size() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        let rect = Rect::new(5, 5, 0, 0);
        ctx.fill_rect(rect, 'X', Some(Color::WHITE), Some(Color::BLACK));
        // Just verify it doesn't panic
    }

    // =========================================================================
    // bar tests
    // =========================================================================

    #[test]
    fn test_draw_context_bar() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.bar(2, 5, 10, Color::WHITE, Some(Color::BLACK));
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_bar_zero_width() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.bar(5, 5, 0, Color::WHITE, None);
        // Just verify it doesn't panic
    }

    // =========================================================================
    // partial_bar tests
    // =========================================================================

    #[test]
    fn test_draw_context_partial_bar_full() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.partial_bar(2, 5, 5.0, Color::WHITE);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_partial_bar_fractional() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.partial_bar(2, 5, 3.5, Color::WHITE);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_partial_bar_zero() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.partial_bar(5, 5, 0.0, Color::WHITE);
        // Just verify it doesn't panic
    }

    // =========================================================================
    // text tests
    // =========================================================================

    #[test]
    fn test_draw_context_text() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.text(2, 5, "hello", Some(Color::WHITE));
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_text_empty() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.text(5, 5, "", Some(Color::WHITE));
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_text_truncated() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.text(5, 5, "very long text", Some(Color::WHITE));
        // Should be truncated to fit
    }

    // =========================================================================
    // text_bold tests
    // =========================================================================

    #[test]
    fn test_draw_context_text_bold() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.text_bold(2, 5, "bold", Some(Color::WHITE));
        // Just verify it doesn't panic
    }

    // =========================================================================
    // clear tests
    // =========================================================================

    #[test]
    fn test_draw_context_clear() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.set(5, 5, 'X');
        ctx.clear();
        // Just verify it doesn't panic
    }

    // =========================================================================
    // point tests
    // =========================================================================

    #[test]
    fn test_draw_context_point() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.point(5, 5, Color::WHITE);
        // Just verify it doesn't panic
    }

    // =========================================================================
    // line tests
    // =========================================================================

    #[test]
    fn test_draw_context_line_horizontal() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.line(2, 5, 15, 5, '─', Some(Color::WHITE));
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_line_vertical() {
        let mut buffer = Buffer::new(10, 20);
        let area = Rect::new(0, 0, 10, 20);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.line(5, 2, 5, 15, '│', Some(Color::WHITE));
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_line_diagonal() {
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.line(2, 2, 15, 15, '*', Some(Color::WHITE));
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_line_same_point() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.line(5, 5, 5, 5, 'X', Some(Color::WHITE));
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_context_line_no_color() {
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = DrawContext::new(&mut buffer, area);
        ctx.line(2, 2, 15, 15, '*', None);
        // Just verify it doesn't panic
    }
}
