//! Code editor editing operations. Each goes through the
//! [`EditState`](crate::utils::EditState) as one undo step (typing and
//! deletes in quick succession join one), auto-indent and closed brackets
//! included (#892), and none changes a read-only editor (#1080).

use super::types::IndentStyle;

impl super::CodeEditor {
    // =========================================================================
    // Editing
    // =========================================================================

    /// Type a character, replacing the selection. `'\n'` starts a new line
    /// indented as the cursor's (one level more after an opening bracket or
    /// `:` when auto-indent is on), `'\t'` types one indent, and with bracket
    /// matching on an opening bracket or a quote types its closing one after
    /// the cursor.
    pub fn insert_char(&mut self, ch: char) {
        let changed = match ch {
            '\n' => {
                let indent = self.new_line_indent();
                self.edit.insert(&format!("\n{indent}"))
            }
            '\t' => self.edit.insert(&self.indent_unit()),
            ch => match closing(ch).filter(|_| self.config.bracket_matching) {
                Some(close) => self
                    .edit
                    .insert_pair(ch.encode_utf8(&mut [0; 4]), close.encode_utf8(&mut [0; 4])),
                None => self.edit.insert(ch.encode_utf8(&mut [0; 4])),
            },
        };
        self.edited(changed);
    }

    /// Insert a string, replacing the selection, as one undo step: as it
    /// is, without auto-indent or closed brackets
    pub fn insert_str(&mut self, s: &str) {
        let changed = self.edit.paste(s);
        self.edited(changed);
    }

    /// The indent a new line after the cursor (the start of the
    /// selection) starts with: none without auto-indent; else the line's
    /// indent up to the cursor, one level more when the text before the
    /// cursor ends with `{`, `[`, `(` or `:`
    fn new_line_indent(&self) -> String {
        if !self.config.auto_indent {
            return String::new();
        }
        // Where the new line goes in: the start of the selection
        let (from, _) = self.edit.primary().range();
        let (line, col) = self.char_pos(from);
        let text = self.edit.line(line).unwrap_or("");
        let before: String = text.chars().take(col).collect();
        // The blanks before the cursor: Enter inside the indent keeps the
        // rest of it on the new line, so only that much is added
        let base: String = before.chars().take_while(|c| c.is_whitespace()).collect();
        let opens = before.trim_end().ends_with(['{', '[', '(', ':']);
        if opens {
            base + &self.indent_unit()
        } else {
            base
        }
    }

    /// Backspace: the selection, or the last code point of the character
    /// before the cursor (a whole emoji), or the line break before the line
    pub fn delete_char_before(&mut self) {
        let changed = self.edit.backspace();
        self.edited(changed);
    }

    /// Delete: the selection, or the character after the cursor, or the
    /// line break after the line
    pub fn delete_char_at(&mut self) {
        let changed = self.edit.delete();
        self.edited(changed);
    }

    /// Delete the lines the cursor or its selection touches
    pub fn delete_line(&mut self) {
        let changed = self.edit.delete_lines();
        self.edited(changed);
    }

    /// Copy the lines the cursor or its selection touches below them; the
    /// cursor moves to the copy
    pub fn duplicate_line(&mut self) {
        let changed = self.edit.duplicate_lines();
        self.edited(changed);
    }

    /// One level of indentation: `indent_size` spaces (one at least) or a
    /// tab
    fn indent_unit(&self) -> String {
        match self.config.indent_style {
            IndentStyle::Spaces => " ".repeat(self.config.indent_size.max(1)),
            IndentStyle::Tabs => "\t".to_string(),
        }
    }

    // =========================================================================
    // Undo and redo
    // =========================================================================

    /// Undo the last edit, putting back the cursor and selection it started
    /// with
    pub fn undo(&mut self) {
        self.selecting = false;
        self.edit.undo();
        self.text_changed();
    }

    /// Redo the last undone edit, with the cursor it ended with
    pub fn redo(&mut self) {
        self.selecting = false;
        self.edit.redo();
        self.text_changed();
    }

    /// Whether there is an edit to undo
    pub fn can_undo(&self) -> bool {
        self.edit.can_undo()
    }

    /// Whether there is an edit to redo
    pub fn can_redo(&self) -> bool {
        self.edit.can_redo()
    }

    /// After an edit: when it changed the text, selection mode ends
    pub(super) fn edited(&mut self, changed: bool) {
        if changed {
            self.selecting = false;
        }
        self.text_changed();
    }

    /// After the text changed: the find matches are found again in it, so
    /// none points at text that moved or went (#975), and the view goes up
    /// to the cursor
    pub(super) fn text_changed(&mut self) {
        if !self.find_query.is_empty() {
            self.refresh_find_matches();
        }
        self.ensure_cursor_visible();
    }
}

/// The closing bracket or quote typed after `ch`
fn closing(ch: char) -> Option<char> {
    match ch {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '"' => Some('"'),
        '\'' => Some('\''),
        _ => None,
    }
}
