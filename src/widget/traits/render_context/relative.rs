//! Relative coordinate methods for RenderContext
//!
//! These methods use coordinates relative to the widget's area,
//! where (0, 0) is the top-left corner of the area.

use crate::render::Cell;
use crate::style::Color;
use crate::utils::unicode::char_width;

impl super::RenderContext<'_> {
    /// Area width (convenience accessor)
    pub fn width(&self) -> u16 {
        self.area.width
    }

    /// Area height (convenience accessor)
    pub fn height(&self) -> u16 {
        self.area.height
    }

    /// Set a cell at relative position (0,0 = area top-left)
    ///
    /// Respects the clipping region set by `overflow: hidden`. A wide glyph
    /// takes the cell to its right as well (see [`Buffer::set`]); where
    /// that cell is outside the area or the clip, a space is drawn instead
    /// of half the glyph.
    ///
    /// [`Buffer::set`]: crate::render::Buffer::set
    pub fn set(&mut self, x: u16, y: u16, mut cell: Cell) {
        if x < self.area.width && y < self.area.height {
            let abs_x = self.area.x.saturating_add(x);
            let abs_y = self.area.y.saturating_add(y);
            if !self.is_clipped(abs_x, abs_y) {
                let wide = !cell.is_continuation() && char_width(cell.symbol) == 2;
                if wide
                    && (x.saturating_add(1) >= self.area.width
                        || self.is_clipped(abs_x.saturating_add(1), abs_y))
                {
                    cell.symbol = ' ';
                    cell.sequence_id = None;
                }
                self.buffer.set(abs_x, abs_y, cell);
            }
        }
    }

    /// Put the caret of the text this widget takes typing for at relative
    /// `(x, y)`, the cell of the glyph it is on (or the blank past the end
    /// of the text). Draw the text without a caret of your own first.
    ///
    /// In a terminal the caret is shown with the terminal's own cursor, so
    /// an input method opens its preedit and candidate window there (#1457).
    /// Where there is no cursor (a snapshot, a buffer) the caret's cells are
    /// drawn reversed. A caret on a wide glyph covers both of its columns;
    /// on the glyph's right half it is moved to its left one.
    ///
    /// One caret per frame is shown: the last one put, as only one widget
    /// takes typing. Call it only while the widget has focus. A caret
    /// outside the area or the clip shows none, and neither does one that
    /// something drawn later covers, such as a popup.
    ///
    /// ```
    /// use revue::prelude::*;
    ///
    /// struct Field;
    ///
    /// impl View for Field {
    ///     fn render(&self, ctx: &mut RenderContext) {
    ///         ctx.draw_text(0, 0, "abc", Color::WHITE);
    ///         ctx.set_caret(3, 0); // past the end of "abc"
    ///     }
    /// }
    /// ```
    pub fn set_caret(&mut self, x: u16, y: u16) {
        let x = match self.get(x, y) {
            Some(cell) if cell.is_continuation() && x > 0 => x - 1,
            _ => x,
        };
        let Some(&cell) = self.get(x, y) else {
            self.mark_caret(x, y, &[]);
            return;
        };
        let mut under = vec![cell];
        if !cell.is_continuation() && char_width(cell.symbol) == 2 {
            if let Some(&next) = x.checked_add(1).and_then(|nx| self.get(nx, y)) {
                under.push(next);
            }
        }
        if self.caret_inside(x, y, under.len() as u16) {
            let abs_y = self.area.y.saturating_add(y);
            for dx in 0..under.len() as u16 {
                let abs_x = self.area.x.saturating_add(x).saturating_add(dx);
                if let Some(drawn) = self.buffer.get_mut(abs_x, abs_y) {
                    drawn.modifier.toggle(crate::render::Modifier::REVERSE);
                }
            }
        }
        self.mark_caret(x, y, &under);
    }

    /// Mark the caret just drawn at relative `(x, y)`, for the terminal to
    /// show with its own cursor: `under` is how its cells, from `x` on, look
    /// without it (see [`caret_cells`]). A caret not wholly inside the area
    /// and the clip is out of view, and the frame shows none.
    ///
    /// For a widget that draws its caret in colors of its own; one that
    /// does not calls [`set_caret`](Self::set_caret). The last caret marked
    /// in a frame is the one shown, as only one field can take typing; any
    /// other stays drawn as the cells it was given.
    pub(crate) fn mark_caret(&mut self, x: u16, y: u16, under: &[Cell]) {
        let abs_x = self.area.x.saturating_add(x);
        let abs_y = self.area.y.saturating_add(y);
        let inside = self.caret_inside(x, y, under.len() as u16);
        self.buffer
            .set_caret(abs_x, abs_y, if inside { under } else { &[] });
    }

    /// Whether `n` cells from relative `(x, y)` are all inside the area and
    /// the clip
    fn caret_inside(&self, x: u16, y: u16, n: u16) -> bool {
        let abs_x = self.area.x.saturating_add(x);
        let abs_y = self.area.y.saturating_add(y);
        n > 0
            && y < self.area.height
            && x.checked_add(n).is_some_and(|end| end <= self.area.width)
            && (0..n).all(|dx| !self.is_clipped(abs_x.saturating_add(dx), abs_y))
    }

    /// Draw `text` at relative `(x, y)` in terminal columns, stopping before
    /// relative column `max_x` (and the area's edge), with each glyph's cell
    /// built by `make_cell`. Respects clipping like [`Self::set`]. A wide
    /// glyph takes two columns, a zero-width char none; see
    /// [`overlay::lay_out_str`](super::overlay::lay_out_str). Returns the
    /// columns used.
    pub(crate) fn put_str_with<F>(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        max_x: u16,
        make_cell: F,
    ) -> u16
    where
        F: FnMut(char) -> Cell,
    {
        let max_x = max_x.min(self.area.width);
        super::overlay::lay_out_str(x, text, max_x, make_cell, |cx, cell| self.set(cx, y, cell))
    }

    /// Set foreground color at relative position
    pub fn set_fg(&mut self, x: u16, y: u16, fg: Color) {
        if x < self.area.width && y < self.area.height {
            let abs_x = self.area.x.saturating_add(x);
            let abs_y = self.area.y.saturating_add(y);
            if !self.is_clipped(abs_x, abs_y) {
                self.buffer.set_fg(abs_x, abs_y, fg);
            }
        }
    }

    /// Set background color at relative position
    pub fn set_bg(&mut self, x: u16, y: u16, bg: Color) {
        if x < self.area.width && y < self.area.height {
            let abs_x = self.area.x.saturating_add(x);
            let abs_y = self.area.y.saturating_add(y);
            if !self.is_clipped(abs_x, abs_y) {
                self.buffer.set_bg(abs_x, abs_y, bg);
            }
        }
    }

    /// Get a cell at relative position
    pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        if x < self.area.width && y < self.area.height {
            self.buffer
                .get(self.area.x.saturating_add(x), self.area.y.saturating_add(y))
        } else {
            None
        }
    }

    /// Get a mutable cell at relative position
    ///
    /// Respects the clipping region set by `overflow: hidden` like
    /// [`Self::set`]: a cell outside the clip is `None`, so it cannot be
    /// modified through this context.
    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        if x < self.area.width && y < self.area.height {
            let abs_x = self.area.x.saturating_add(x);
            let abs_y = self.area.y.saturating_add(y);
            if self.is_clipped(abs_x, abs_y) {
                return None;
            }
            self.buffer.get_mut(abs_x, abs_y)
        } else {
            None
        }
    }

    /// Put a string at relative position, handling wide characters.
    /// Returns the number of columns written.
    ///
    /// Respects the clipping region set by `overflow: hidden`.
    pub fn put_str(&mut self, x: u16, y: u16, s: &str) -> u16 {
        if y >= self.area.height {
            return 0;
        }
        let abs_x = self.area.x.saturating_add(x);
        let abs_y = self.area.y.saturating_add(y);
        let max_x = self.area.x.saturating_add(self.area.width);

        let mut offset = 0u16;
        for ch in s.chars() {
            let w = char_width(ch) as u16;
            if w == 0 {
                continue;
            }
            let cx = abs_x.saturating_add(offset);
            if cx.saturating_add(w) > max_x {
                break;
            }
            if !self.is_clipped(cx, abs_y) {
                self.buffer.set(cx, abs_y, Cell::new(ch));
                for i in 1..w {
                    if !self.is_clipped(cx + i, abs_y) {
                        self.buffer.set(cx + i, abs_y, Cell::continuation());
                    }
                }
            }
            offset = offset.saturating_add(w);
        }
        offset
    }
}

/// The cells a glyph takes: `cell`, and a continuation with its colors,
/// modifiers and link when it is wide - for
/// [`mark_caret`](super::RenderContext::mark_caret)
pub(crate) fn caret_cells(cell: Cell) -> Vec<Cell> {
    let mut cells = vec![cell];
    if !cell.is_continuation() && char_width(cell.symbol) == 2 {
        let mut cont = Cell::continuation();
        cont.fg = cell.fg;
        cont.bg = cell.bg;
        cont.modifier = cell.modifier;
        cont.hyperlink_id = cell.hyperlink_id;
        cells.push(cont);
    }
    cells
}
