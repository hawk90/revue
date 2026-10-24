//! Cursor navigation methods for TextArea

impl TextArea {
    /// Move every cursor; the selections grow while there is one or after
    /// [`start_selection`](Self::start_selection), and the cursors just
    /// move otherwise
    fn move_by(&mut self, motion: EditMotion) {
        let extend = self.extending();
        self.edit.move_cursors(motion, extend);
    }

    /// Whether a move extends the selection rather than drop it
    fn extending(&self) -> bool {
        self.selecting || self.edit.has_selection()
    }

    /// Move one character left
    pub fn move_left(&mut self) {
        self.move_by(EditMotion::Left);
    }

    /// Move one character right
    pub fn move_right(&mut self) {
        self.move_by(EditMotion::Right);
    }

    /// Move one line up, keeping the screen column
    pub fn move_up(&mut self) {
        self.move_by(EditMotion::Up);
    }

    /// Move one line down, keeping the screen column
    pub fn move_down(&mut self) {
        self.move_by(EditMotion::Down);
    }

    /// Move to the start of the line
    pub fn move_home(&mut self) {
        self.move_by(EditMotion::LineStart);
    }

    /// Move to the end of the line
    pub fn move_end(&mut self) {
        self.move_by(EditMotion::LineEnd);
    }

    /// Move to the start of the text
    pub fn move_document_start(&mut self) {
        self.move_by(EditMotion::Start);
    }

    /// Move to the end of the text
    pub fn move_document_end(&mut self) {
        self.move_by(EditMotion::End);
    }

    /// Move a word left: over whitespace, then one run of word characters
    /// or of other characters (`foo.bar` stops at `bar`, `.`, `foo`)
    pub fn move_word_left(&mut self) {
        self.move_by(EditMotion::WordLeft);
    }

    /// Move a word right, as [`move_word_left`](Self::move_word_left) does
    /// left
    pub fn move_word_right(&mut self) {
        self.move_by(EditMotion::WordRight);
    }

    /// Page up: scroll the view up `page_size` rows and move the cursor up
    /// with it, keeping its screen row and column. When the view is already
    /// at the top, the cursor moves to the first line instead.
    ///
    /// With wrapping on, rows are screen rows, and the view can start
    /// partway down a wrapped line, so a line taller than the page pages
    /// through it too.
    pub fn page_up(&mut self, page_size: usize) {
        let extend = self.extending();
        self.page(page_size, false, extend);
    }

    /// Page down: scroll the view down `page_size` rows and move the cursor
    /// down with it, keeping its screen row and column. The view stops with
    /// the last row of the text on its bottom row; once there, the cursor
    /// moves to the last line instead.
    ///
    /// With wrapping on, rows are screen rows, as for [`page_up`](Self::page_up).
    pub fn page_down(&mut self, page_size: usize) {
        let extend = self.extending();
        self.page(page_size, true, extend);
    }

    /// Page the view and the primary cursor (the other cursors go); with
    /// `extend` the primary cursor's selection grows to where it lands
    pub(super) fn page(&mut self, page_size: usize, down: bool, extend: bool) {
        let page = page_size.max(1);
        let width = self.last_text_width.get();
        let pos = self.primary_pos();
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
            let line = if down { self.edit.line_count() - 1 } else { 0 };
            let col = self.same_column_on(pos, line);
            self.page_to(CursorPos::new(line, col), extend);
            return;
        };

        self.scroll.set(new_top.0);
        self.scroll_row.set(new_top.1);
        let (line, row) = rows.forward(new_top, row);
        let col = self.col_at(line, row, width, x);
        self.page_to(CursorPos::new(line, col), extend);
    }

    /// Put the primary cursor at `to` (in chars), alone, its selection
    /// grown to there with `extend`
    fn page_to(&mut self, to: CursorPos, extend: bool) {
        let head = self.edit_pos(to);
        let anchor = if extend {
            self.edit.primary().anchor()
        } else {
            head
        };
        self.edit.select(anchor, head);
        self.selecting = self.selecting && extend;
    }

    /// The char index `x` columns into visual row `row` of `line`. Only a
    /// line's last row holds the position at the row's end: elsewhere that
    /// position is drawn at the start of the next row.
    fn col_at(&self, line: usize, row: usize, width: u16, x: usize) -> usize {
        let segments = self.line_rows(line, width);
        let k = row.min(segments.len() - 1);
        let (start, end) = segments[k];
        let text = self.line_text(line);
        let seg: String = text.chars().skip(start).take(end - start).collect();
        let col = start + char_at_col(&seg, x);
        // On the start of a character the user sees as one, not inside it
        if k + 1 < segments.len() && col >= end {
            self.cluster_start(line, end.saturating_sub(1)).max(start)
        } else {
            self.cluster_start(line, col)
        }
    }

    /// The char index on `line` under the screen column of `pos`, so moving
    /// between lines keeps the cursor in place across wide glyphs (a char
    /// index alone drifts one column per wide glyph before it).
    pub(super) fn same_column_on(&self, pos: CursorPos, line: usize) -> usize {
        let col = char_at_col(
            self.line_text(line),
            col_of(self.line_text(pos.line), pos.col),
        );
        self.cluster_start(line, col)
    }

    /// Select all text
    pub fn select_all(&mut self) {
        self.edit.select_all();
    }
}

use super::cursor::CursorPos;
use super::layout::RowCounts;
use super::TextArea;
use crate::utils::EditMotion;
use crate::widget::traits::render_context::edit_line::{char_at_col, col_of};
