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
    /// With wrapping on, rows are screen rows. The view starts at a line, so
    /// a line that only partly fits above is left for the next page rather
    /// than cut (the view then scrolls a little less than a page), and a line
    /// taller than the page scrolls by that one line.
    pub fn page_up(&mut self, page_size: usize) {
        self.page(page_size, false);
    }

    /// Page down: scroll the view down `page_size` rows and move the cursor
    /// down with it, keeping its screen row and column. The view stops with
    /// the last line on its bottom row; once there, the cursor moves to the
    /// last line instead.
    ///
    /// With wrapping on, rows are screen rows. The view starts at a line, so
    /// a line across the bottom edge becomes the new top line rather than
    /// having its hidden rows skipped (the view then scrolls a little less
    /// than a page), and a line taller than the page scrolls by that one line.
    pub fn page_down(&mut self, page_size: usize) {
        self.page(page_size, true);
    }

    fn page(&mut self, page_size: usize, down: bool) {
        let page = page_size.max(1);
        let width = self.last_text_width.get();
        let pos = self.cursors.primary().pos;
        // Settle the view on the cursor first, as render would.
        let top = self.scroll_to_cursor_row(width, page);
        let new_top = if down {
            self.page_down_top(top, width, page)
        } else {
            self.page_up_top(top, width, page)
        };

        let Some(new_top) = new_top else {
            // The view cannot scroll further: go to the first or last line.
            self.scroll.set(top);
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

        // The cursor's screen row and its column within that row, kept
        // across the scroll.
        let (seg_idx, (seg_start, _)) = self.cursor_segment(width);
        let line_text = self.lines.get(pos.line).map_or("", String::as_str);
        let seg_text: String = line_text.chars().skip(seg_start).collect();
        let x = col_of(&seg_text, pos.col.saturating_sub(seg_start));
        let rows_above: usize = (top..pos.line)
            .map(|l| self.line_rows(l, width).len())
            .sum();
        let row = (rows_above + seg_idx).min(page - 1);

        self.scroll.set(new_top);
        let (line, col) = self.pos_at_row(new_top, row, width, x);
        self.set_primary_cursor(line, col);
        self.update_selection();
    }

    /// The first line of the view a page below one starting at `top`: the
    /// line holding the first row below the view, at least one line on, and
    /// no further than the top that puts the last line on the bottom row.
    /// `None` when the view is already there.
    fn page_down_top(&self, top: usize, width: u16, page: usize) -> Option<usize> {
        let last = self.lines.len().saturating_sub(1);
        let mut max_top = last;
        let mut rows = self.line_rows(last, width).len();
        while max_top > 0 {
            let above = self.line_rows(max_top - 1, width).len();
            if rows + above > page {
                break;
            }
            rows += above;
            max_top -= 1;
        }
        if top >= max_top {
            return None;
        }

        let mut next = top;
        let mut rows = 0;
        while next < last {
            let here = self.line_rows(next, width).len();
            if rows + here > page {
                break;
            }
            rows += here;
            next += 1;
        }
        Some(next.max(top + 1).min(max_top))
    }

    /// The first line of the view a page above one starting at `top`: the
    /// furthest line up whose rows down to `top` fit in a page, at least one
    /// line up. `None` at the top.
    fn page_up_top(&self, top: usize, width: u16, page: usize) -> Option<usize> {
        if top == 0 {
            return None;
        }
        let mut prev = top;
        let mut rows = 0;
        while prev > 0 {
            let above = self.line_rows(prev - 1, width).len();
            if rows + above > page {
                break;
            }
            rows += above;
            prev -= 1;
        }
        Some(prev.min(top - 1))
    }

    /// The position on screen row `row` of a view starting at line `top`,
    /// `x` columns into that row (the last row when `row` is past the text).
    fn pos_at_row(&self, top: usize, row: usize, width: u16, x: usize) -> (usize, usize) {
        let last = self.lines.len().saturating_sub(1);
        let mut remaining = row;
        for line in top..=last {
            let segments = self.line_rows(line, width);
            if remaining < segments.len() || line == last {
                let k = remaining.min(segments.len() - 1);
                let (start, end) = segments[k];
                let text = self.lines.get(line).map_or("", String::as_str);
                let seg: String = text.chars().skip(start).take(end - start).collect();
                let mut col = start + char_at_col(&seg, x);
                // Only a line's last row holds the cursor at the row's end:
                // elsewhere that spot is drawn at the start of the next row.
                if k + 1 < segments.len() && col >= end {
                    col = end.saturating_sub(1).max(start);
                }
                return (line, col);
            }
            remaining -= segments.len();
        }
        (last, 0)
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

use super::TextArea;
use crate::widget::traits::render_context::edit_line::{char_at_col, col_of};
