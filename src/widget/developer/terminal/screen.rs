//! The text model: writing characters into the lines, wrapping, newlines,
//! trimming the scrollback, clearing and resizing

use super::types::{TermCell, TermLine};
use super::Terminal;

impl Terminal {
    /// Write text to terminal (with ANSI support)
    pub fn write(&mut self, text: &str) {
        for ch in text.chars() {
            self.write_char(ch);
        }
    }

    /// Write a single character. It goes through the ANSI scanner first, so
    /// a newline, carriage return or tab inside an OSC or DCS string is part
    /// of the string and does nothing; elsewhere (inside a control sequence
    /// too) it acts, as in a terminal.
    pub(super) fn write_char(&mut self, ch: char) {
        let Some(cell) = self.parser.parse(ch) else {
            return;
        };
        match cell.ch {
            '\n' => self.newline(),
            '\r' => self.cursor_col = 0,
            '\t' => {
                // Tab to the next 8-column boundary, or to the right edge,
                // in spaces of the colors in effect. Count the spaces up
                // front: the cursor wraps to column 0 when a space fills the
                // last column.
                let next_tab = ((self.cursor_col / 8) + 1) * 8;
                let spaces = next_tab
                    .min(self.width as usize)
                    .saturating_sub(self.cursor_col);
                for _ in 0..spaces {
                    self.put_cell(self.parser.cell(' '));
                }
            }
            _ => self.put_cell(cell),
        }
    }

    /// Put a cell at current cursor position. As in a real terminal, a
    /// wide character takes two columns (the second holds `'\0'`, its right
    /// half) and goes to the next line when only the last column is left; a
    /// zero-width one (a combining mark) takes none.
    fn put_cell(&mut self, cell: TermCell) {
        let width = crate::utils::unicode::char_width(cell.ch);
        if width == 0 {
            return;
        }
        if width == 2 && self.width >= 2 {
            if self.cursor_col + 1 >= self.width as usize {
                self.wrap_line();
            }
            let half = TermCell {
                ch: '\0',
                ..cell.clone()
            };
            self.put_one(cell);
            self.put_one(half);
        } else {
            self.put_one(cell);
        }
    }

    /// Put one column at the cursor and move it on, wrapping at the edge
    fn put_one(&mut self, cell: TermCell) {
        // Ensure we have enough lines
        while self.cursor_row >= self.lines.len() {
            self.lines
                .push(TermLine::with_capacity(self.width as usize));
        }

        // Get current line
        let line = &mut self.lines[self.cursor_row];

        // Ensure line has enough cells
        while line.cells.len() <= self.cursor_col {
            line.cells.push(super::ansi::blank());
        }

        // Set cell
        line.cells[self.cursor_col] = cell;

        // Advance cursor
        self.cursor_col += 1;

        if self.cursor_col >= self.width as usize {
            self.wrap_line();
        }
    }

    /// Continue on the next line, marked as wrapped from this one
    fn wrap_line(&mut self) {
        self.cursor_col = 0;
        self.cursor_row += 1;
        if self.cursor_row >= self.lines.len() {
            let mut new_line = TermLine::with_capacity(self.width as usize);
            new_line.wrapped = true;
            self.lines.push(new_line);
        } else {
            self.lines[self.cursor_row].wrapped = true;
        }
        self.sync_scroll();
        self.trim_scrollback();
    }

    /// Move to new line
    fn newline(&mut self) {
        self.cursor_col = 0;
        self.cursor_row += 1;

        while self.cursor_row >= self.lines.len() {
            self.lines
                .push(TermLine::with_capacity(self.width as usize));
        }
        self.sync_scroll();

        self.trim_scrollback();
    }

    /// Trim scrollback to max
    fn trim_scrollback(&mut self) {
        let total_lines = self.height as usize + self.max_scrollback;
        while self.lines.len() > total_lines {
            self.lines.remove(0);
            // A scrolled-back view stays on the lines it shows
            self.scroll.rows_removed(0, 1);
            if self.cursor_row > 0 {
                self.cursor_row -= 1;
            }
        }
    }

    /// Write a line with automatic newline
    pub fn writeln(&mut self, text: &str) {
        self.write(text);
        self.write("\n");
    }

    /// Clear the terminal
    pub fn clear(&mut self) {
        self.lines.clear();
        for _ in 0..self.height {
            self.lines
                .push(TermLine::with_capacity(self.width as usize));
        }
        self.cursor_row = 0;
        self.cursor_col = 0;
        self.sync_scroll();
        self.scroll.scroll_to_end();
    }

    /// Clear current line
    pub fn clear_line(&mut self) {
        if self.cursor_row < self.lines.len() {
            self.lines[self.cursor_row].cells.clear();
        }
        self.cursor_col = 0;
    }

    /// Resize terminal
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        // Add lines if needed
        while self.lines.len() < height as usize {
            self.lines.push(TermLine::with_capacity(width as usize));
        }
        self.sync_scroll();
    }
}
