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
        let abs_x = self.area.x.saturating_add(x);
        let abs_y = self.area.y.saturating_add(y);
        let max_x = self.area.x.saturating_add(self.area.width);
        let mut offset = 0u16;
        for ch in text.chars() {
            let width = char_width(ch) as u16;
            if width == 0 {
                continue;
            }
            let cx = abs_x.saturating_add(offset);
            if cx.saturating_add(width) > max_x {
                break;
            }
            self.buffer.set(cx, abs_y, make_cell(ch));
            for i in 1..width {
                self.buffer.set(cx + i, abs_y, Cell::continuation());
            }
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
        let abs_x = self.area.x.saturating_add(x);
        let abs_y = self.area.y.saturating_add(y);
        let max_x = self.area.x.saturating_add(self.area.width);
        let mut offset = 0u16;
        for ch in text.chars() {
            let width = char_width(ch) as u16;
            if width == 0 {
                continue;
            }
            if offset.saturating_add(width) > max_width {
                break;
            }
            let cx = abs_x.saturating_add(offset);
            if cx.saturating_add(width) > max_x {
                break;
            }
            self.buffer.set(cx, abs_y, make_cell(ch));
            for i in 1..width {
                self.buffer.set(cx + i, abs_y, Cell::continuation());
            }
            offset = offset.saturating_add(width);
        }
    }

    /// Draw a single character at relative position (0,0 = area top-left)
    #[inline]
    pub fn draw_char(&mut self, x: u16, y: u16, ch: char, fg: Color) {
        if x < self.area.width && y < self.area.height {
            let cell = Cell::new(ch).fg(fg);
            self.buffer.set(
                self.area.x.saturating_add(x),
                self.area.y.saturating_add(y),
                cell,
            );
        }
    }

    /// Draw a character with background color at relative position
    #[inline]
    pub fn draw_char_bg(&mut self, x: u16, y: u16, ch: char, fg: Color, bg: Color) {
        if x < self.area.width && y < self.area.height {
            let cell = Cell::new(ch).fg(fg).bg(bg);
            self.buffer.set(
                self.area.x.saturating_add(x),
                self.area.y.saturating_add(y),
                cell,
            );
        }
    }

    /// Draw a bold character at relative position
    #[inline]
    pub fn draw_char_bold(&mut self, x: u16, y: u16, ch: char, fg: Color) {
        if x < self.area.width && y < self.area.height {
            let cell = Cell::new(ch).fg(fg).bold();
            self.buffer.set(
                self.area.x.saturating_add(x),
                self.area.y.saturating_add(y),
                cell,
            );
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
