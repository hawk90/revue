//! Cursor navigation methods for TextArea

impl TextArea {
    /// Move cursor left
    pub fn move_left(&mut self) {
        let pos = self.cursors.primary().pos;
        if pos.col > 0 {
            self.set_primary_cursor(pos.line, pos.col - 1);
        } else if pos.line > 0 {
            let new_line = pos.line - 1;
            let new_col = self.line_len(new_line);
            self.set_primary_cursor(new_line, new_col);
        }
        self.update_selection();
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        let pos = self.cursors.primary().pos;
        let line_len = self.line_len(pos.line);
        if pos.col < line_len {
            self.set_primary_cursor(pos.line, pos.col + 1);
        } else if pos.line + 1 < self.lines.len() {
            self.set_primary_cursor(pos.line + 1, 0);
        }
        self.update_selection();
    }

    /// Move cursor up
    pub fn move_up(&mut self) {
        let pos = self.cursors.primary().pos;
        if pos.line > 0 {
            let new_line = pos.line - 1;
            let new_col = self.same_column_on(pos, new_line);
            self.set_primary_cursor(new_line, new_col);
        }
        self.update_selection();
    }

    /// Move cursor down
    pub fn move_down(&mut self) {
        let pos = self.cursors.primary().pos;
        if pos.line + 1 < self.lines.len() {
            let new_line = pos.line + 1;
            let new_col = self.same_column_on(pos, new_line);
            self.set_primary_cursor(new_line, new_col);
        }
        self.update_selection();
    }

    /// Move to start of line
    pub fn move_home(&mut self) {
        let pos = self.cursors.primary().pos;
        self.set_primary_cursor(pos.line, 0);
        self.update_selection();
    }

    /// Move to end of line
    pub fn move_end(&mut self) {
        let pos = self.cursors.primary().pos;
        let line_len = self.line_len(pos.line);
        self.set_primary_cursor(pos.line, line_len);
        self.update_selection();
    }

    /// Move to start of document
    pub fn move_document_start(&mut self) {
        self.set_primary_cursor(0, 0);
        self.update_selection();
    }

    /// Move to end of document
    pub fn move_document_end(&mut self) {
        let last_line = self.lines.len().saturating_sub(1);
        let last_col = self.line_len(last_line);
        self.set_primary_cursor(last_line, last_col);
        self.update_selection();
    }

    /// Move cursor by word to the left
    pub fn move_word_left(&mut self) {
        let pos = self.cursors.primary().pos;
        if pos.col == 0 {
            if pos.line > 0 {
                let new_line = pos.line - 1;
                let new_col = self.line_len(new_line);
                self.set_primary_cursor(new_line, new_col);
            }
            return;
        }

        let Some(line) = self.lines.get(pos.line) else {
            return;
        };
        let chars: Vec<char> = line.chars().collect();
        let mut col = pos.col.min(chars.len());

        // Skip spaces
        while col > 0 && chars[col - 1].is_whitespace() {
            col -= 1;
        }
        // Skip word
        while col > 0 && !chars[col - 1].is_whitespace() {
            col -= 1;
        }

        self.set_primary_cursor(pos.line, col);
        self.update_selection();
    }

    /// Move cursor by word to the right
    pub fn move_word_right(&mut self) {
        let pos = self.cursors.primary().pos;
        let Some(line) = self.lines.get(pos.line) else {
            return;
        };
        let chars: Vec<char> = line.chars().collect();
        let mut col = pos.col;

        if col >= chars.len() {
            if pos.line + 1 < self.lines.len() {
                self.set_primary_cursor(pos.line + 1, 0);
            }
            return;
        }

        // Skip current word
        while col < chars.len() && !chars[col].is_whitespace() {
            col += 1;
        }
        // Skip spaces
        while col < chars.len() && chars[col].is_whitespace() {
            col += 1;
        }

        self.set_primary_cursor(pos.line, col);
        self.update_selection();
    }

    /// Page up: scroll the view up `page_size` rows and move the cursor up
    /// with it, keeping its screen row and column. When the view is already
    /// at the top, the cursor moves to the first line instead.
    ///
    /// With wrapping on, rows are screen rows, and the view can start
    /// partway down a wrapped line, so a line taller than the page pages
    /// through it too.
    pub fn page_up(&mut self, page_size: usize) {
        self.page(page_size, false);
    }

    /// Page down: scroll the view down `page_size` rows and move the cursor
    /// down with it, keeping its screen row and column. The view stops with
    /// the last row of the text on its bottom row; once there, the cursor
    /// moves to the last line instead.
    ///
    /// With wrapping on, rows are screen rows, as for [`page_up`](Self::page_up).
    pub fn page_down(&mut self, page_size: usize) {
        self.page(page_size, true);
    }

    fn page(&mut self, page_size: usize, down: bool) {
        let page = page_size.max(1);
        let width = self.last_text_width.get();
        let pos = self.cursors.primary().pos;
        // Settle the view on the cursor first, as render would.
        let top = self.scroll_to_cursor_row(width, page);
        let mut rows = RowCounts::new(self, width);
        let (k, x) = self.place(pos, width);
        // The cursor's screen row, kept across the scroll
        let row = rows.between(top, (pos.line, k), page).min(page - 1);
        let new_top = if down {
            // The furthest the view goes: the last row on the bottom row
            let end = rows.end();
            let max_top = rows.back(end, page - 1);
            (top < max_top).then(|| rows.forward(top, page).min(max_top))
        } else {
            (top > (0, 0)).then(|| rows.back(top, page))
        };

        let Some(new_top) = new_top else {
            // The view cannot scroll further: go to the first or last line.
            self.scroll.set(top.0);
            self.scroll_row.set(top.1);
            let line = if down {
                self.lines.len().saturating_sub(1)
            } else {
                0
            };
            let col = self.same_column_on(pos, line);
            self.set_primary_cursor(line, col);
            self.update_selection();
            return;
        };

        self.scroll.set(new_top.0);
        self.scroll_row.set(new_top.1);
        let (line, row) = rows.forward(new_top, row);
        let col = self.col_at(line, row, width, x);
        self.set_primary_cursor(line, col);
        self.update_selection();
    }

    /// The char index `x` columns into visual row `row` of `line`. Only a
    /// line's last row holds the position at the row's end: elsewhere that
    /// position is drawn at the start of the next row.
    fn col_at(&self, line: usize, row: usize, width: u16, x: usize) -> usize {
        let segments = self.line_rows(line, width);
        let k = row.min(segments.len() - 1);
        let (start, end) = segments[k];
        let text = self.lines.get(line).map_or("", String::as_str);
        let seg: String = text.chars().skip(start).take(end - start).collect();
        let col = start + char_at_col(&seg, x);
        if k + 1 < segments.len() && col >= end {
            end.saturating_sub(1).max(start)
        } else {
            col
        }
    }

    /// The char index on `line` under the screen column of `pos`, so moving
    /// between lines keeps the cursor in place across wide glyphs (a char
    /// index alone drifts one column per wide glyph before it).
    fn same_column_on(&self, pos: super::cursor::CursorPos, line: usize) -> usize {
        let from = self.lines.get(pos.line).map_or("", String::as_str);
        let to = self.lines.get(line).map_or("", String::as_str);
        char_at_col(to, col_of(from, pos.col))
    }

    /// Select all text
    pub fn select_all(&mut self) {
        use super::cursor::CursorPos;
        use super::cursor::CursorSet;

        let last_line = self.lines.len().saturating_sub(1);
        let last_col = self.lines.last().map(|l| l.chars().count()).unwrap_or(0);
        // Create cursor at end with anchor at start
        self.cursors = CursorSet::new(CursorPos::new(last_line, last_col));
        self.cursors.primary_mut().anchor = Some(CursorPos::new(0, 0));
    }
}

use super::layout::RowCounts;
use super::TextArea;
use crate::widget::traits::render_context::edit_line::{char_at_col, col_of};
