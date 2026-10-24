//! Code editor cursor and navigation. Columns in this API are chars; the
//! cursor is never placed inside a character the user sees as one (a
//! column inside one goes to its end).

use crate::utils::{EditMotion, EditPos};
use crate::widget::traits::render_context::edit_line::{char_at_col, col_of};

impl super::CodeEditor {
    // =========================================================================
    // Cursor and Navigation
    // =========================================================================

    /// Get cursor position, in chars
    pub fn cursor_position(&self) -> (usize, usize) {
        self.char_pos(self.edit.cursor())
    }

    /// Set cursor position, in chars; in selection mode the selection
    /// grows to there, otherwise it is dropped
    pub fn set_cursor(&mut self, line: usize, col: usize) {
        let extend = self.selecting;
        self.move_to((line, col), extend);
    }

    /// Get line count
    pub fn line_count(&self) -> usize {
        self.edit.line_count()
    }

    /// A model position in chars
    pub(super) fn char_pos(&self, pos: EditPos) -> (usize, usize) {
        (pos.line, self.edit.char_index(pos))
    }

    /// The text of line `line` (empty past the last)
    pub(super) fn line_text(&self, line: usize) -> &str {
        self.edit.line(line).unwrap_or("")
    }

    /// Move the cursor; the selection grows while there is one or in
    /// selection mode, and is dropped otherwise
    fn move_by(&mut self, motion: EditMotion) {
        let extend = self.extending();
        self.edit.move_cursors(motion, extend);
        self.ensure_cursor_visible();
    }

    /// Whether a move extends the selection rather than drop it
    fn extending(&self) -> bool {
        self.selecting || self.edit.has_selection()
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        self.move_by(EditMotion::Left);
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        self.move_by(EditMotion::Right);
    }

    /// Move cursor up, keeping the screen column
    pub fn move_up(&mut self) {
        self.move_by(EditMotion::Up);
    }

    /// Move cursor down, keeping the screen column
    pub fn move_down(&mut self) {
        self.move_by(EditMotion::Down);
    }

    /// Move to the first non-blank character of the line, or to the line's
    /// start when already there (smart Home)
    pub fn move_home(&mut self) {
        let extend = self.extending();
        self.smart_home(extend);
    }

    /// Smart Home, extending the selection with `extend`
    pub(super) fn smart_home(&mut self, extend: bool) {
        let (line, col) = self.cursor_position();
        let first = self
            .line_text(line)
            .chars()
            .position(|c| !c.is_whitespace())
            .unwrap_or(0);
        // A blank followed by a combining mark is one character: compare
        // with where `first` rounds to
        let first_at = self.edit.char_index(self.edit.pos_at_char(line, first));
        let target = if col == first_at { 0 } else { first };
        self.move_to((line, target), extend);
    }

    /// Move to end of line
    pub fn move_end(&mut self) {
        self.move_by(EditMotion::LineEnd);
    }

    /// Move to start of document
    pub fn move_document_start(&mut self) {
        self.move_by(EditMotion::Start);
    }

    /// Move to end of document
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

    /// Page up: scroll the view up `page_size` lines and move the cursor up
    /// the same number, keeping its screen row and column. When the view is
    /// already at the top, the cursor moves to the first line instead.
    pub fn page_up(&mut self, page_size: usize) {
        let extend = self.extending();
        self.page(page_size, false, extend);
    }

    /// Page down: scroll the view down `page_size` lines and move the cursor
    /// down the same number, keeping its screen row and column. The view
    /// stops with the last line on its bottom row; once there, the cursor
    /// moves to the last line instead.
    pub fn page_down(&mut self, page_size: usize) {
        let extend = self.extending();
        self.page(page_size, true, extend);
    }

    /// Page the view and the cursor; with `extend` the selection grows to
    /// where the cursor lands
    pub(super) fn page(&mut self, page_size: usize, down: bool, extend: bool) {
        let page = page_size.max(1);
        let top = self.scroll_for_cursor(page);
        let line = self.cursor_position().0;
        let last = self.edit.line_count() - 1;
        let target = if down {
            let max_top = self.edit.line_count().saturating_sub(page);
            let new_top = (top + page).min(max_top);
            if new_top <= top {
                self.scroll.set(top);
                last
            } else {
                self.scroll.set(new_top);
                (line + (new_top - top)).min(last)
            }
        } else if top == 0 {
            self.scroll.set(top);
            0
        } else {
            let new_top = top.saturating_sub(page);
            self.scroll.set(new_top);
            line - (top - new_top)
        };
        let col = self.same_column_on(target);
        self.move_to((target, col), extend);
    }

    /// Move the cursor to `(line, col)` in chars, the selection growing to
    /// there with `extend`
    fn move_to(&mut self, (line, col): (usize, usize), extend: bool) {
        let head = self.edit.pos_at_char(line, col);
        let anchor = if extend {
            self.edit.primary().anchor()
        } else {
            head
        };
        self.edit.select(anchor, head);
        self.ensure_cursor_visible();
    }

    /// The first visible line that keeps the cursor's line in a view
    /// `visible_lines` high, moving the current scroll as little as possible.
    pub(super) fn scroll_for_cursor(&self, visible_lines: usize) -> usize {
        let cursor_line = self.edit.cursor().line;
        self.scroll
            .get()
            .min(cursor_line)
            .max((cursor_line + 1).saturating_sub(visible_lines))
    }

    /// The char index on `line` under the cursor's screen column (a char
    /// index alone drifts one column per wide glyph before it), on the
    /// start of the character the user sees as one there
    fn same_column_on(&self, line: usize) -> usize {
        let (from, col) = self.cursor_position();
        let x = col_of(self.line_text(from), col);
        let idx = char_at_col(self.line_text(line), x);
        let pos = self.edit.pos_at_char(line, idx);
        if self.edit.char_index(pos) == idx {
            idx
        } else {
            // Inside a cluster: its start
            self.edit
                .char_index(EditPos::new(line, pos.col.saturating_sub(1)))
        }
    }

    /// Ensure cursor is visible. Only scrolling up can be settled here;
    /// render scrolls down, and sideways, once it knows the view size.
    pub(super) fn ensure_cursor_visible(&mut self) {
        let line = self.edit.cursor().line;
        if line < self.scroll.get() {
            self.scroll.set(line);
        }
    }
}
