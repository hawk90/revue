//! Content access methods for TextArea
//!
//! The text, cursors and selections live in an [`EditState`](crate::utils::EditState), whose columns
//! are grapheme clusters; this API counts columns in chars, as it always
//! has, and turns them into the model's positions and back.

use super::cursor::CursorPos;
use crate::utils::{EditCursor, EditPos};
use crate::widget::syntax::{Language, SyntaxHighlighter};

impl TextArea {
    /// Get the current highlighting language
    pub fn get_syntax_language(&self) -> Language {
        self.highlighter
            .as_ref()
            .map(|h| h.get_language())
            .unwrap_or(Language::None)
    }

    /// Set the syntax highlighting language
    pub fn set_language(&mut self, language: Language) {
        if language == Language::None {
            self.highlighter = None;
        } else {
            self.highlighter = Some(SyntaxHighlighter::new(language));
        }
    }

    /// Get the current text content
    pub fn get_content(&self) -> String {
        self.edit.text()
    }

    /// Set the text content. A final line break is kept (as an empty last
    /// line), so [`get_content`](Self::get_content) gives the text back;
    /// `\r\n` line breaks read as `\n`, other control characters but tab
    /// are dropped, and lines past [`max_lines`](Self::max_lines) are cut.
    /// The cursor goes to the start and the undo history is cleared.
    pub fn set_content(&mut self, text: &str) {
        self.edit.set_text(text);
        self.selecting = false;
        self.scroll.set(0);
        self.scroll_row.set(0);
        self.scroll_x.set(0);
    }

    /// Get the number of lines
    pub fn line_count(&self) -> usize {
        self.edit.line_count()
    }

    /// Get the cursor position (the primary cursor), in chars
    pub fn cursor_position(&self) -> (usize, usize) {
        self.primary_pos().into()
    }

    /// Get all cursor positions, in text order
    pub fn cursor_positions(&self) -> Vec<(usize, usize)> {
        self.edit
            .cursors()
            .iter()
            .map(|c| self.char_pos(c.head()).into())
            .collect()
    }

    /// Get the number of cursors
    pub fn cursor_count(&self) -> usize {
        self.edit.cursors().len()
    }

    /// Set the cursor position, in chars (one cursor; a column inside a
    /// character the user sees as one goes to its end)
    pub fn set_cursor(&mut self, line: usize, col: usize) {
        let pos = self.edit_pos(CursorPos::new(line, col));
        self.edit.set_cursor(pos);
        self.selecting = false;
    }

    /// The text of line `line` (empty past the last)
    pub(super) fn line_text(&self, line: usize) -> &str {
        self.edit.line(line).unwrap_or("")
    }

    /// The primary cursor's position, in chars
    pub(super) fn primary_pos(&self) -> CursorPos {
        self.char_pos(self.edit.cursor())
    }

    /// The char index where the character the user sees as one (the
    /// grapheme cluster) holding char `idx` of `line` starts: `idx` itself
    /// on a boundary or past the end
    pub(super) fn cluster_start(&self, line: usize, idx: usize) -> usize {
        let text = self.line_text(line);
        let pos = self.edit.pos_at_char(line, idx);
        let at = self.edit.char_index(pos);
        if at == idx || idx >= text.chars().count() {
            return idx;
        }
        // Rounded up to the cluster's end: step back one cluster
        self.edit
            .char_index(EditPos::new(line, pos.col.saturating_sub(1)))
    }

    /// A model position in chars
    pub(super) fn char_pos(&self, pos: EditPos) -> CursorPos {
        CursorPos::new(pos.line, self.edit.char_index(pos))
    }

    /// A position in chars as the model's
    pub(super) fn edit_pos(&self, pos: CursorPos) -> EditPos {
        self.edit.pos_at_char(pos.line, pos.col)
    }

    /// Get the selected text (of the primary cursor)
    pub fn get_selection(&self) -> Option<String> {
        let cursor = self.edit.primary();
        cursor.has_selection().then(|| {
            let (from, to) = cursor.range();
            self.edit.text_in(from, to)
        })
    }

    /// Delete the selected text, at every cursor
    pub fn delete_selection(&mut self) {
        self.selecting = false;
        self.edit.delete_selection();
    }

    /// Whether the primary cursor selects any text
    pub fn has_selection(&self) -> bool {
        self.edit.primary().has_selection()
    }

    /// Start a selection at the cursor: the move methods
    /// ([`move_left`](Self::move_left) and the others) extend it from here
    /// until the selection is cleared, the text is edited or the cursor is
    /// set. Shift with a moving key selects without this.
    pub fn start_selection(&mut self) {
        self.selecting = true;
    }

    /// Clear the selection of every cursor, keeping the cursors where they
    /// are
    pub fn clear_selection(&mut self) {
        self.selecting = false;
        let cursors = self
            .edit
            .cursors()
            .iter()
            .map(|c| EditCursor::at(c.head()))
            .collect::<Vec<_>>();
        self.set_cursors_keeping_primary(cursors);
    }

    /// Replace the cursors, the primary one staying primary
    pub(super) fn set_cursors_keeping_primary(&mut self, mut cursors: Vec<EditCursor>) {
        let primary = self
            .edit
            .cursors()
            .iter()
            .position(|c| c == self.edit.primary())
            .unwrap_or(0);
        // The last cursor given is the primary one
        let last = cursors.len() - 1;
        cursors.swap(primary.min(last), last);
        self.edit.set_cursors(cursors);
    }
}

use super::TextArea;
