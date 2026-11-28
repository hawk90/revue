//! Shape drawing methods for RenderContext

use crate::render::Cell;
use crate::style::Color;
use crate::utils::unicode::char_width;
use crate::utils::unicode::display_width_u16;

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
            pos = pos.saturating_add(display_width_u16(text));
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

    /// Fill with background color (relative coordinates)
    pub fn fill_bg(&mut self, x: u16, y: u16, w: u16, h: u16, bg: Color) {
        for dy in 0..h {
            for dx in 0..w {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                self.set(x + dx, y + dy, cell);
            }
        }
    }

    /// Give every cell of the rectangle (relative coordinates) that has no
    /// background yet the color `bg`, keeping its glyph.
    ///
    /// For a band of background (a row, a content area) under glyphs drawn
    /// without one: draw them first, then call this. [`Buffer::set`] replaces
    /// the whole cell, so a glyph written after [`fill_bg`](Self::fill_bg)
    /// without a background punches a hole in the band; a cell that has its
    /// own background (a tag, a highlight) keeps it. The band version of what
    /// [`fill_box_background`](Self::fill_box_background) does for the box.
    ///
    /// [`Buffer::set`]: crate::render::Buffer::set
    pub(crate) fn fill_bg_under(&mut self, x: u16, y: u16, w: u16, h: u16, bg: Color) {
        for dy in 0..h {
            for dx in 0..w {
                let (rx, ry) = (x.saturating_add(dx), y.saturating_add(dy));
                if rx >= self.area.width || ry >= self.area.height {
                    continue;
                }
                let (abs_x, abs_y) = (self.area.x + rx, self.area.y + ry);
                if self.is_clipped(abs_x, abs_y) {
                    continue;
                }
                if let Some(cell) = self.buffer.get_mut(abs_x, abs_y) {
                    if cell.bg.is_none() {
                        cell.bg = Some(bg);
                    }
                }
            }
        }
    }

    /// Fill a row with spaces, setting optional fg and bg colors
    pub fn fill_row(&mut self, y: u16, width: u16, fg: Option<Color>, bg: Option<Color>) {
        for x in 0..width {
            let mut cell = Cell::new(' ');
            cell.fg = fg;
            cell.bg = bg;
            self.set(x, y, cell);
        }
    }

    /// Clear area (fill with spaces, relative coordinates)
    pub fn clear(&mut self, x: u16, y: u16, w: u16, h: u16) {
        for dy in 0..h {
            for dx in 0..w {
                self.set(x + dx, y + dy, Cell::empty());
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

    /// Helper: Draw top border with embedded title using O(n) iterator (relative coordinates)
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
                        // The rest of the title does not fit: drop it and
                        // finish the border instead of leaving a gap
                        title_chars.by_ref().for_each(drop);
                        continue;
                    }
                    self.draw_char(x + pos, y, ch, fg);
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
