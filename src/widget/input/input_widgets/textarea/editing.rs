//! Text editing methods for TextArea: each applies at every cursor, through
//! the [`EditState`](crate::utils::EditState), as one undo step (typing and
//! deletes in quick succession join one)

impl TextArea {
    /// Type a character at every cursor, replacing what each selects. `'\n'`
    /// starts a new line (not past [`max_lines`](Self::max_lines)), and `'\t'`
    /// types [`tab_width`](Self::tab_width) spaces.
    pub fn insert_char(&mut self, ch: char) {
        self.selecting = false;
        match ch {
            '\t' => self.edit.insert(&" ".repeat(self.tab_width)),
            ch => self.edit.insert(ch.encode_utf8(&mut [0; 4])),
        };
    }

    /// Insert a string at every cursor, replacing what each selects, as one
    /// undo step. When its line breaks would take the text past
    /// [`max_lines`](Self::max_lines), it goes in without them.
    pub fn insert_str(&mut self, s: &str) {
        self.selecting = false;
        self.edit.paste(s);
    }

    /// Backspace at every cursor: the selection, or the last code point of
    /// the character before the cursor (a whole emoji), or the line break
    /// before the line
    pub fn delete_char_before(&mut self) {
        self.selecting = false;
        self.edit.backspace();
    }

    /// Delete at every cursor: the selection, or the character after the
    /// cursor, or the line break after the line
    pub fn delete_char_at(&mut self) {
        self.selecting = false;
        self.edit.delete();
    }

    /// Delete every line a cursor or its selection touches
    pub fn delete_line(&mut self) {
        self.selecting = false;
        self.edit.delete_lines();
    }

    /// Copy every line a cursor or its selection touches below it; the
    /// cursors move to the copy
    pub fn duplicate_line(&mut self) {
        self.selecting = false;
        self.edit.duplicate_lines();
    }
}

use super::TextArea;
