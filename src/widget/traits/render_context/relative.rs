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
    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if x < self.area.width && y < self.area.height {
            self.buffer.set(
                self.area.x.saturating_add(x),
                self.area.y.saturating_add(y),
                cell,
            );
        }
    }

    /// Set foreground color at relative position
    pub fn set_fg(&mut self, x: u16, y: u16, fg: Color) {
        if x < self.area.width && y < self.area.height {
            self.buffer.set_fg(
                self.area.x.saturating_add(x),
                self.area.y.saturating_add(y),
                fg,
            );
        }
    }

    /// Set background color at relative position
    pub fn set_bg(&mut self, x: u16, y: u16, bg: Color) {
        if x < self.area.width && y < self.area.height {
            self.buffer.set_bg(
                self.area.x.saturating_add(x),
                self.area.y.saturating_add(y),
                bg,
            );
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

    /// Put a string at relative position, handling wide characters.
    /// Returns the number of columns written.
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
            self.buffer.set(cx, abs_y, Cell::new(ch));
            for i in 1..w {
                self.buffer.set(cx + i, abs_y, Cell::continuation());
            }
            offset = offset.saturating_add(w);
        }
        offset
    }
}
