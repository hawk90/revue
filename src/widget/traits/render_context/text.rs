//! Text drawing methods for RenderContext

use crate::render::Cell;
use crate::style::Color;
use crate::utils::unicode::{char_width, display_width};

impl RenderContext<'_> {
    /// Helper: Draw text with custom cell styling, handling wide characters correctly.
    ///
    /// Coordinates are relative to the area (0,0 = top-left of area).
    pub(super) fn draw_text_with_style<F>(&mut self, x: u16, y: u16, text: &str, mut make_cell: F)
    where
        F: FnMut(char) -> Cell,
    {
        if y >= self.area.height {
            return;
        }
        let abs_y = self.area.y.saturating_add(y);
        let mut offset = 0u16;
        for ch in text.chars() {
            let width = char_width(ch) as u16;
            if width == 0 {
                continue;
            }
            let Some(cx) = self.text_cell_x(x, offset, width) else {
                break;
            };
            self.put_text_char(cx, abs_y, width, make_cell(ch));
            offset = offset.saturating_add(width);
        }
    }

    /// Helper: Draw text clipped to max_width, handling wide characters correctly.
    ///
    /// Coordinates are relative to the area (0,0 = top-left of area).
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
        if y >= self.area.height {
            return;
        }
        let abs_y = self.area.y.saturating_add(y);
        let mut offset = 0u16;
        for ch in text.chars() {
            let width = char_width(ch) as u16;
            if width == 0 {
                continue;
            }
            if offset.saturating_add(width) > max_width {
                break;
            }
            let Some(cx) = self.text_cell_x(x, offset, width) else {
                break;
            };
            self.put_text_char(cx, abs_y, width, make_cell(ch));
            offset = offset.saturating_add(width);
        }
    }

    /// Absolute x of a `width`-cell character drawn `offset` cells after
    /// relative `x`, or `None` when any of its cells would fall past the
    /// area's right edge or past the end of the `u16` coordinate space.
    ///
    /// Computed in `u32` so nothing saturates into a false "fits".
    fn text_cell_x(&self, x: u16, offset: u16, width: u16) -> Option<u16> {
        let start = u32::from(self.area.x) + u32::from(x) + u32::from(offset);
        let area_end = u32::from(self.area.x) + u32::from(self.area.width);
        let end = area_end.min(u32::from(u16::MAX) + 1);
        if start + u32::from(width) > end {
            return None;
        }
        u16::try_from(start).ok()
    }

    /// Put a character at `cx` and continuation cells after it; the caller
    /// has checked (via [`text_cell_x`](Self::text_cell_x)) that all
    /// `width` cells are addressable.
    ///
    /// Respects the clipping region set by `overflow: hidden`: a glyph with
    /// any of its cells outside the clip is not drawn at all, so a wide glyph
    /// on the clip's edge leaves no orphaned half.
    fn put_text_char(&mut self, cx: u16, abs_y: u16, width: u16, cell: Cell) {
        if (0..width).any(|i| self.is_clipped(cx.saturating_add(i), abs_y)) {
            return;
        }
        self.buffer.set(cx, abs_y, cell);
        for i in 1..width {
            self.buffer
                .set(cx.saturating_add(i), abs_y, Cell::continuation());
        }
    }

    /// Draw a single character at relative position (0,0 = area top-left).
    ///
    /// Like every drawing method here, respects the clipping region set by
    /// `overflow: hidden`.
    #[inline]
    pub fn draw_char(&mut self, x: u16, y: u16, ch: char, fg: Color) {
        let (abs_x, abs_y) = (self.area.x.saturating_add(x), self.area.y.saturating_add(y));
        if x < self.area.width && y < self.area.height && !self.is_clipped(abs_x, abs_y) {
            let cell = Cell::new(ch).fg(fg);
            self.buffer.set(abs_x, abs_y, cell);
        }
    }

    /// Draw a character with background color at relative position
    #[inline]
    pub fn draw_char_bg(&mut self, x: u16, y: u16, ch: char, fg: Color, bg: Color) {
        let (abs_x, abs_y) = (self.area.x.saturating_add(x), self.area.y.saturating_add(y));
        if x < self.area.width && y < self.area.height && !self.is_clipped(abs_x, abs_y) {
            let cell = Cell::new(ch).fg(fg).bg(bg);
            self.buffer.set(abs_x, abs_y, cell);
        }
    }

    /// Draw a bold character at relative position
    #[inline]
    pub fn draw_char_bold(&mut self, x: u16, y: u16, ch: char, fg: Color) {
        let (abs_x, abs_y) = (self.area.x.saturating_add(x), self.area.y.saturating_add(y));
        if x < self.area.width && y < self.area.height && !self.is_clipped(abs_x, abs_y) {
            let cell = Cell::new(ch).fg(fg).bold();
            self.buffer.set(abs_x, abs_y, cell);
        }
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

    /// Draw text with background color clipped to max_width
    pub fn draw_text_clipped_bg(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        fg: Color,
        bg: Color,
        max_width: u16,
    ) {
        self.draw_text_clipped_with_style(x, y, text, max_width, |ch| Cell::new(ch).fg(fg).bg(bg));
    }

    /// Draw bold text with background color clipped to max_width
    pub fn draw_text_clipped_bg_bold(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        fg: Color,
        bg: Color,
        max_width: u16,
    ) {
        self.draw_text_clipped_with_style(x, y, text, max_width, |ch| {
            Cell::new(ch).fg(fg).bg(bg).bold()
        });
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
            x.saturating_add((width - text_width) / 2)
        };
        self.draw_text_clipped(start_x, y, text, fg, width);
    }

    /// Draw text right-aligned within a given width
    pub fn draw_text_right(&mut self, x: u16, y: u16, width: u16, text: &str, fg: Color) {
        let text_width = display_width(text) as u16;
        let start_x = if text_width >= width {
            x
        } else {
            x.saturating_add(width - text_width)
        };
        self.draw_text_clipped(start_x, y, text, fg, width);
    }
}

use crate::widget::traits::render_context::RenderContext;
