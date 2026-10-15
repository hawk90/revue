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

    /// Write a single character
    pub(super) fn write_char(&mut self, ch: char) {
        if ch == '\n' {
            self.newline();
            return;
        }
        if ch == '\r' {
            self.cursor_col = 0;
            return;
        }
        if ch == '\t' {
            // Tab to the next 8-column boundary, or to the right edge. Count
            // the spaces up front: the cursor wraps to column 0 when a space
            // fills the last column, and a space the ANSI parser swallows does
            // not move it at all - a loop on the cursor never ends in either
            // case.
            let next_tab = ((self.cursor_col / 8) + 1) * 8;
            let spaces = next_tab
                .min(self.width as usize)
                .saturating_sub(self.cursor_col);
            for _ in 0..spaces {
                self.write_char(' ');
            }
            return;
        }

        if let Some(cell) = self.parser.parse(ch) {
            self.put_cell(cell);
        }
    }

    /// Put a cell at current cursor position
    fn put_cell(&mut self, cell: TermCell) {
        // Ensure we have enough lines
        while self.cursor_row >= self.lines.len() {
            self.lines
                .push(TermLine::with_capacity(self.width as usize));
        }

        // Get current line
        let line = &mut self.lines[self.cursor_row];

        // Ensure line has enough cells
        while line.cells.len() <= self.cursor_col {
            line.cells.push(TermCell::default());
        }

        // Set cell
        line.cells[self.cursor_col] = cell;

        // Advance cursor
        self.cursor_col += 1;

        // Handle line wrapping
        if self.cursor_col >= self.width as usize {
            self.cursor_col = 0;
            self.cursor_row += 1;
            if self.cursor_row >= self.lines.len() {
                let mut new_line = TermLine::with_capacity(self.width as usize);
                new_line.wrapped = true;
                self.lines.push(new_line);
            } else {
                self.lines[self.cursor_row].wrapped = true;
            }
            self.trim_scrollback();
        }
    }

    /// Move to new line
    fn newline(&mut self) {
        self.cursor_col = 0;
        self.cursor_row += 1;

        while self.cursor_row >= self.lines.len() {
            self.lines
                .push(TermLine::with_capacity(self.width as usize));
        }

        self.trim_scrollback();
    }

    /// Trim scrollback to max
    fn trim_scrollback(&mut self) {
        let total_lines = self.height as usize + self.max_scrollback;
        while self.lines.len() > total_lines {
            self.lines.remove(0);
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
        self.scroll_offset = 0;
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
    }
}
