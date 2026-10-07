//! Titles and info labels drawn on a border's edges

use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::RenderContext;

/// Position for border title
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TitlePosition {
    /// Left/Top aligned
    #[default]
    Start,
    /// Center aligned
    Center,
    /// Right/Bottom aligned
    End,
}

/// Edge of the border for title placement
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum BorderEdge {
    /// Top border (horizontal)
    #[default]
    Top,
    /// Bottom border (horizontal)
    Bottom,
    /// Left border (vertical)
    Left,
    /// Right border (vertical)
    Right,
}

/// A title or info section to draw on a border
#[derive(Clone, Debug)]
pub struct BorderTitle {
    /// Text content (supports any chars including nerd font icons)
    pub text: String,
    /// Which edge to draw on
    pub edge: BorderEdge,
    /// Position along the edge
    pub position: TitlePosition,
    /// Foreground color
    pub fg: Option<Color>,
    /// Background color (uses border bg if None)
    pub bg: Option<Color>,
    /// Padding before text
    pub pad_start: u16,
    /// Padding after text
    pub pad_end: u16,
    /// Offset from calculated position (can be negative via wrapping)
    pub offset: i16,
}

impl BorderTitle {
    /// Create a new border title
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            edge: BorderEdge::Top,
            position: TitlePosition::Start,
            fg: None,
            bg: None,
            pad_start: 1,
            pad_end: 1,
            offset: 0,
        }
    }

    /// Set the edge (top, bottom, left, right)
    pub fn edge(mut self, edge: BorderEdge) -> Self {
        self.edge = edge;
        self
    }

    /// Set position along edge (start, center, end)
    pub fn position(mut self, pos: TitlePosition) -> Self {
        self.position = pos;
        self
    }

    /// Convenience: top edge
    pub fn top(mut self) -> Self {
        self.edge = BorderEdge::Top;
        self
    }

    /// Convenience: bottom edge
    pub fn bottom(mut self) -> Self {
        self.edge = BorderEdge::Bottom;
        self
    }

    /// Convenience: left edge
    pub fn left(mut self) -> Self {
        self.edge = BorderEdge::Left;
        self
    }

    /// Convenience: right edge
    pub fn right(mut self) -> Self {
        self.edge = BorderEdge::Right;
        self
    }

    /// Convenience: start position
    pub fn start(mut self) -> Self {
        self.position = TitlePosition::Start;
        self
    }

    /// Convenience: center position
    pub fn center(mut self) -> Self {
        self.position = TitlePosition::Center;
        self
    }

    /// Convenience: end position
    pub fn end(mut self) -> Self {
        self.position = TitlePosition::End;
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set padding (both sides)
    pub fn padding(mut self, pad: u16) -> Self {
        self.pad_start = pad;
        self.pad_end = pad;
        self
    }

    /// Set start padding only
    pub fn pad_start(mut self, pad: u16) -> Self {
        self.pad_start = pad;
        self
    }

    /// Set end padding only
    pub fn pad_end(mut self, pad: u16) -> Self {
        self.pad_end = pad;
        self
    }

    /// Set offset from calculated position
    pub fn offset(mut self, offset: i16) -> Self {
        self.offset = offset;
        self
    }

    /// Get the display width of the title (text + padding)
    pub fn width(&self) -> u16 {
        crate::utils::unicode::display_width(&self.text) as u16 + self.pad_start + self.pad_end
    }
}

/// Draw a border title on the given area
///
/// This draws the title text with padding, replacing the border characters.
/// Use this after drawing the border.
///
/// # Example
/// ```ignore
/// render_border(ctx, area, Color::BLUE);
/// draw_border_title(ctx, area, &BorderTitle::new(" Title ").fg(Color::BLUE));
/// draw_border_title(ctx, area, &BorderTitle::new("Info").end().fg(Color::GREEN));
/// ```
pub fn draw_border_title(ctx: &mut RenderContext, area: Rect, title: &BorderTitle) {
    if area.width < 3 || area.height < 2 {
        return;
    }

    let text_width = crate::utils::unicode::display_width(&title.text) as u16;
    let total_width = text_width + title.pad_start + title.pad_end;

    match title.edge {
        BorderEdge::Top | BorderEdge::Bottom => {
            let available = area.width.saturating_sub(2); // Exclude corners
            if total_width > available {
                return;
            }

            let y = if title.edge == BorderEdge::Top {
                area.y
            } else {
                area.y + area.height - 1
            };

            // Calculate x position
            let base_x = match title.position {
                TitlePosition::Start => area.x + 1,
                TitlePosition::Center => area.x + 1 + (available.saturating_sub(total_width)) / 2,
                TitlePosition::End => area.x + area.width - 1 - total_width,
            };
            let x = base_x
                .saturating_add_signed(title.offset)
                .max(area.x.saturating_add(1));

            // Draw padding (spaces to clear border chars)
            for dx in 0..title.pad_start {
                let mut cell = Cell::new(' ');
                cell.fg = title.fg;
                cell.bg = title.bg;
                ctx.buffer.set(x.saturating_add(dx), y, cell);
            }

            // Draw text by column: a wide glyph takes two cells, a
            // zero-width one (e.g. VS16) none, matching `text_width`.
            let text_x = x.saturating_add(title.pad_start);
            ctx.buffer
                .put_str_styled(text_x, y, &title.text, title.fg, title.bg);

            // Draw end padding
            let end_x = text_x.saturating_add(text_width);
            for dx in 0..title.pad_end {
                let mut cell = Cell::new(' ');
                cell.fg = title.fg;
                cell.bg = title.bg;
                ctx.buffer.set(end_x.saturating_add(dx), y, cell);
            }
        }
        BorderEdge::Left | BorderEdge::Right => {
            let available = area.height.saturating_sub(2); // Exclude corners
            let text_len = title.text.chars().count() as u16;
            let total_height = text_len + title.pad_start + title.pad_end;

            if total_height > available {
                return;
            }

            let x = if title.edge == BorderEdge::Left {
                area.x
            } else {
                area.x + area.width - 1
            };

            // Calculate y position
            let base_y = match title.position {
                TitlePosition::Start => area.y + 1,
                TitlePosition::Center => area.y + 1 + (available.saturating_sub(total_height)) / 2,
                TitlePosition::End => area.y + area.height - 1 - total_height,
            };
            let y = base_y
                .saturating_add_signed(title.offset)
                .max(area.y.saturating_add(1));

            // Draw padding (spaces)
            for dy in 0..title.pad_start {
                let mut cell = Cell::new(' ');
                cell.fg = title.fg;
                cell.bg = title.bg;
                ctx.buffer.set(x, y.saturating_add(dy), cell);
            }

            // Draw text (vertically)
            let text_y = y.saturating_add(title.pad_start);
            for (i, ch) in title.text.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = title.fg;
                cell.bg = title.bg;
                ctx.buffer.set(x, text_y.saturating_add(i as u16), cell);
            }

            // Draw end padding
            let end_y = text_y.saturating_add(text_len);
            for dy in 0..title.pad_end {
                let mut cell = Cell::new(' ');
                cell.fg = title.fg;
                cell.bg = title.bg;
                ctx.buffer.set(x, end_y.saturating_add(dy), cell);
            }
        }
    }
}

/// Draw multiple border titles
pub fn draw_border_titles(ctx: &mut RenderContext, area: Rect, titles: &[BorderTitle]) {
    for title in titles {
        draw_border_title(ctx, area, title);
    }
}

/// Convenience function: draw a simple title on top-left
pub fn draw_title(ctx: &mut RenderContext, area: Rect, text: &str, color: Color) {
    draw_border_title(ctx, area, &BorderTitle::new(text).fg(color));
}

/// Convenience function: draw info on top-right
pub fn draw_title_right(ctx: &mut RenderContext, area: Rect, text: &str, color: Color) {
    draw_border_title(ctx, area, &BorderTitle::new(text).end().fg(color));
}

/// Convenience function: draw centered title
pub fn draw_title_center(ctx: &mut RenderContext, area: Rect, text: &str, color: Color) {
    draw_border_title(ctx, area, &BorderTitle::new(text).center().fg(color));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::utils::border::render_border;

    #[test]
    fn test_border_title_builder() {
        let title = BorderTitle::new("Test")
            .top()
            .start()
            .fg(Color::BLUE)
            .padding(2);

        assert_eq!(title.text, "Test");
        assert_eq!(title.edge, BorderEdge::Top);
        assert_eq!(title.position, TitlePosition::Start);
        assert_eq!(title.fg, Some(Color::BLUE));
        assert_eq!(title.pad_start, 2);
        assert_eq!(title.pad_end, 2);
    }

    #[test]
    fn test_border_title_width() {
        let title = BorderTitle::new("Hello").padding(1);
        assert_eq!(title.width(), 7); // 5 + 1 + 1
    }

    #[test]
    fn test_draw_border_title_start() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        render_border(&mut ctx, area, Color::WHITE);
        draw_border_title(&mut ctx, area, &BorderTitle::new("Title").padding(1));

        // Should have space, T, i, t, l, e, space at positions 1-7
        assert_eq!(buffer.get(1, 0).unwrap().symbol, ' ');
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'T');
        assert_eq!(buffer.get(3, 0).unwrap().symbol, 'i');
        assert_eq!(buffer.get(6, 0).unwrap().symbol, 'e');
        assert_eq!(buffer.get(7, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_draw_border_title_end() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        render_border(&mut ctx, area, Color::WHITE);
        draw_border_title(&mut ctx, area, &BorderTitle::new("End").end().padding(1));

        // "End" with padding = 5 chars total, position = 20 - 1 - 5 = 14
        // 14: pad_start, 15: E, 16: n, 17: d, 18: pad_end
        assert_eq!(buffer.get(14, 0).unwrap().symbol, ' ');
        assert_eq!(buffer.get(15, 0).unwrap().symbol, 'E');
        assert_eq!(buffer.get(16, 0).unwrap().symbol, 'n');
        assert_eq!(buffer.get(17, 0).unwrap().symbol, 'd');
    }

    #[test]
    fn test_draw_border_title_wide_chars_advance_by_columns() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        render_border(&mut ctx, area, Color::WHITE);
        draw_border_title(&mut ctx, area, &BorderTitle::new("🔄Sync").padding(1));

        // 1: pad, 2-3: 🔄, 4-7: Sync, 8: pad, 9: border again
        assert_eq!(buffer.get(1, 0).unwrap().symbol, ' ');
        assert_eq!(buffer.get(2, 0).unwrap().symbol, '🔄');
        assert!(buffer.get(3, 0).unwrap().is_continuation());
        assert_eq!(buffer.get(4, 0).unwrap().symbol, 'S');
        assert_eq!(buffer.get(7, 0).unwrap().symbol, 'c');
        assert_eq!(buffer.get(8, 0).unwrap().symbol, ' ');
        assert_eq!(buffer.get(9, 0).unwrap().symbol, '─');
    }

    #[test]
    fn test_draw_border_title_vs16_takes_no_cell() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        render_border(&mut ctx, area, Color::WHITE);
        draw_border_title(&mut ctx, area, &BorderTitle::new("⚙\u{FE0F}Set").padding(1));

        // 1: pad, 2-3: ⚙ (selector dropped), 4-6: Set, 7: pad, 8: border
        assert_eq!(buffer.get(2, 0).unwrap().symbol, '⚙');
        assert!(buffer.get(3, 0).unwrap().is_continuation());
        assert_eq!(buffer.get(4, 0).unwrap().symbol, 'S');
        assert_eq!(buffer.get(6, 0).unwrap().symbol, 't');
        assert_eq!(buffer.get(7, 0).unwrap().symbol, ' ');
        assert_eq!(buffer.get(8, 0).unwrap().symbol, '─');
    }

    #[test]
    fn test_draw_border_title_center() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        render_border(&mut ctx, area, Color::WHITE);
        draw_border_title(&mut ctx, area, &BorderTitle::new("Hi").center().padding(1));

        // "Hi" with padding = 4 chars, available = 18, should be centered
        // Center position = 1 + (18 - 4) / 2 = 1 + 7 = 8
        assert_eq!(buffer.get(9, 0).unwrap().symbol, 'H');
        assert_eq!(buffer.get(10, 0).unwrap().symbol, 'i');
    }

    #[test]
    fn test_draw_border_title_bottom() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        render_border(&mut ctx, area, Color::WHITE);
        draw_border_title(
            &mut ctx,
            area,
            &BorderTitle::new("Bottom").bottom().padding(1),
        );

        // Should be on bottom border (y = 4)
        assert_eq!(buffer.get(2, 4).unwrap().symbol, 'B');
        assert_eq!(buffer.get(7, 4).unwrap().symbol, 'm');
    }

    #[test]
    fn test_draw_multiple_titles() {
        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(0, 0, 30, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        render_border(&mut ctx, area, Color::WHITE);
        draw_border_titles(
            &mut ctx,
            area,
            &[
                BorderTitle::new("Left").start(),
                BorderTitle::new("Right").end(),
            ],
        );

        // "Left" at start: pos 1 pad, 2 L, 3 e, 4 f, 5 t, 6 pad
        // "Right" at end: 30 - 1 - 7 = 22 pad, 23 R, 24 i, 25 g, 26 h, 27 t, 28 pad
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'L');
        assert_eq!(buffer.get(23, 0).unwrap().symbol, 'R');
    }

    #[test]
    fn test_draw_title_convenience() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        render_border(&mut ctx, area, Color::WHITE);
        draw_title(&mut ctx, area, "Test", Color::BLUE);

        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'T');
        assert_eq!(buffer.get(2, 0).unwrap().fg, Some(Color::BLUE));
    }

    fn draw_on(buffer: &mut Buffer, area: Rect, title: BorderTitle) {
        let mut ctx = RenderContext::new(buffer, area);
        draw_border_title(&mut ctx, area, &title);
    }

    #[test]
    fn test_draw_title_with_large_positive_offset() {
        // 11 + i16::MAX overflowed the old i16 sum
        let mut buffer = Buffer::new(30, 30);
        draw_on(
            &mut buffer,
            Rect::new(10, 0, 20, 3),
            BorderTitle::new("T").offset(i16::MAX),
        );
        draw_on(
            &mut buffer,
            Rect::new(0, 10, 3, 20),
            BorderTitle::new("T")
                .edge(BorderEdge::Left)
                .offset(i16::MAX),
        );
        // Pushed off the buffer, so nothing is drawn
        for y in 0..30 {
            for x in 0..30 {
                assert_ne!(buffer.get(x, y).unwrap().symbol, 'T');
            }
        }

        // An offset that still lands inside the border is honored
        let mut buffer = Buffer::new(30, 3);
        draw_on(
            &mut buffer,
            Rect::new(10, 0, 20, 3),
            BorderTitle::new("T").offset(5),
        );
        assert_eq!(buffer.get(17, 0).unwrap().symbol, 'T');
    }

    #[test]
    fn test_draw_title_with_large_negative_offset() {
        // Clamped to just after the corner
        let mut buffer = Buffer::new(30, 30);
        draw_on(
            &mut buffer,
            Rect::new(10, 0, 20, 3),
            BorderTitle::new("T").offset(i16::MIN),
        );
        assert_eq!(buffer.get(12, 0).unwrap().symbol, 'T');
        draw_on(
            &mut buffer,
            Rect::new(0, 10, 3, 20),
            BorderTitle::new("T")
                .edge(BorderEdge::Left)
                .offset(i16::MIN),
        );
        assert_eq!(buffer.get(0, 12).unwrap().symbol, 'T');
    }

    #[test]
    fn test_draw_title_on_area_past_i16_max() {
        // Off any buffer, but placing the title must not overflow
        let mut buffer = Buffer::new(10, 10);
        for (x, y) in [
            (32_767, 0),
            (32_760, 0),
            (65_500, 0),
            (0, 32_767),
            (0, 65_500),
        ] {
            for edge in [
                BorderEdge::Top,
                BorderEdge::Bottom,
                BorderEdge::Left,
                BorderEdge::Right,
            ] {
                for offset in [0, 10, i16::MAX, i16::MIN] {
                    let title = BorderTitle::new("T").edge(edge).end().offset(offset);
                    draw_on(&mut buffer, Rect::new(x, y, 30, 30), title);
                }
            }
        }
    }
}
