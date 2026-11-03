//! The text model: writing characters into the lines, wrapping, newlines,
//! trimming the scrollback, clearing and resizing

use super::ansi::{Edit, Read};
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
        let cell = match self.parser.parse(ch) {
            None => return,
            Some(Read::Cell(cell)) => cell,
            Some(Read::Edit(edit)) => return self.edit(edit),
            Some(Read::Control(ch)) => return self.control(ch),
        };
        self.put_cell(cell);
    }

    /// Act on a newline, carriage return, tab or backspace
    fn control(&mut self, ch: char) {
        match ch {
            '\n' => self.newline(),
            '\r' => {
                self.cursor_col = 0;
                self.wrap_pending = false;
            }
            // Back one column, stopping at the first (no reverse wrap); from
            // a pending wrap that is the column before the last
            '\x08' => {
                self.cursor_col = self.cursor_col.saturating_sub(1);
                self.wrap_pending = false;
            }
            '\t' => {
                // Tab to the next 8-column boundary in spaces of the colors
                // in effect, stopping at the last column as in xterm: a tab
                // never wraps
                let next_tab = ((self.cursor_col / 8) + 1) * 8;
                let last = (self.width as usize).saturating_sub(1);
                let spaces = next_tab.min(last).saturating_sub(self.cursor_col);
                for _ in 0..spaces {
                    self.put_cell(self.parser.cell(' '));
                }
            }
            _ => {}
        }
    }

    /// Act on a line-oriented control sequence. The cursor stays on the
    /// screen: the last `height` lines, never back into the scrollback, and
    /// within the columns. Erased cells take the background in effect, as
    /// in xterm.
    fn edit(&mut self, edit: Edit) {
        let last_col = (self.width as usize).saturating_sub(1);
        // Each of them ends a pending wrap, erasing too (xterm's
        // `ResetWrap`): the next char goes on the cursor's column
        self.wrap_pending = false;
        match edit {
            Edit::EraseToEnd => self.erase(self.cursor_col, self.width as usize),
            Edit::EraseToCursor => self.erase(0, self.cursor_col + 1),
            Edit::EraseLine => self.erase(0, self.width as usize),
            Edit::Up(n) => {
                let top = self.lines.len().saturating_sub(self.height as usize);
                self.cursor_row = self.cursor_row.saturating_sub(n).max(top);
            }
            Edit::Down(n) => {
                let bottom = self.lines.len().saturating_sub(1);
                self.cursor_row = self.cursor_row.saturating_add(n).min(bottom);
            }
            Edit::Right(n) => self.cursor_col = self.cursor_col.saturating_add(n).min(last_col),
            Edit::Left(n) => self.cursor_col = self.cursor_col.saturating_sub(n),
            Edit::Column(n) => self.cursor_col = (n - 1).min(last_col),
        }
    }

    /// Blank the columns `from..to` of the cursor's line in the background
    /// in effect. Past the end of the line in the default background there
    /// is nothing to blank: the line just ends there.
    fn erase(&mut self, from: usize, to: usize) {
        let to = to.min(self.width as usize);
        let Some(line) = self.lines.get_mut(self.cursor_row) else {
            return;
        };
        let mut blank = super::ansi::blank();
        blank.bg = self.parser.cell(' ').bg;
        let end = if blank.bg.is_default() {
            to.min(line.cells.len())
        } else {
            to
        };
        if line.cells.len() < end {
            line.cells.resize(end, super::ansi::blank());
        }
        for cell in line.cells.iter_mut().take(end).skip(from) {
            *cell = blank.clone();
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
        if self.wrap_pending {
            self.wrap_pending = false;
            self.wrap_line();
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

    /// Put one column at the cursor and move it on; on the last column it
    /// stays there with the wrap pending
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

        // Advance the cursor, or leave it on the last column
        if self.cursor_col + 1 >= self.width as usize {
            self.wrap_pending = true;
        } else {
            self.cursor_col += 1;
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
        self.wrap_pending = false;
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
        self.wrap_pending = false;
        self.sync_scroll();
        self.scroll.scroll_to_end();
    }

    /// Clear current line
    pub fn clear_line(&mut self) {
        if self.cursor_row < self.lines.len() {
            self.lines[self.cursor_row].cells.clear();
        }
        self.cursor_col = 0;
        self.wrap_pending = false;
    }

    /// Resize terminal
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.wrap_pending = false;
        // Add lines if needed
        while self.lines.len() < height as usize {
            self.lines.push(TermLine::with_capacity(width as usize));
        }
        self.sync_scroll();
    }
}
