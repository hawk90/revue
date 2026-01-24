//! UTF-8 aware text buffer for editing widgets
//!
//! Provides a reusable text buffer with:
//! - Character-based (not byte-based) cursor positioning
//! - Selection support (anchor + cursor)
//! - Word navigation
//! - Efficient UTF-8 handling
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::TextBuffer;
//!
//! let mut buffer = TextBuffer::new();
//! buffer.insert_char('H');
//! buffer.insert_char('i');
//! buffer.insert_str(" 🎉");
//!
//! assert_eq!(buffer.text(), "Hi 🎉");
//! assert_eq!(buffer.char_count(), 4);  // Not 7 bytes!
//! ```

/// A UTF-8 aware text buffer for single-line text editing
///
/// All positions are in CHARACTER indices, not byte indices.
/// This ensures correct handling of multi-byte UTF-8 characters
/// (emoji, CJK, etc).
#[derive(Clone, Debug, Default)]
pub struct TextBuffer {
    /// The text content
    content: String,
    /// Cursor position in CHARACTER index
    cursor: usize,
    /// Selection anchor in CHARACTER index (where selection started)
    selection_anchor: Option<usize>,
}

impl TextBuffer {
    /// Create a new empty text buffer
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor: 0,
            selection_anchor: None,
        }
    }

    /// Create a text buffer with initial content
    pub fn with_content(text: impl Into<String>) -> Self {
        let content = text.into();
        let cursor = content.chars().count();
        Self {
            content,
            cursor,
            selection_anchor: None,
        }
    }

    // =========================================================================
    // Basic Accessors
    // =========================================================================

    /// Get the text content
    #[inline]
    pub fn text(&self) -> &str {
        &self.content
    }

    /// Get cursor position (character index)
    #[inline]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Get character count
    #[inline]
    pub fn char_count(&self) -> usize {
        self.content.chars().count()
    }

    /// Check if buffer is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Get byte length
    #[inline]
    pub fn len(&self) -> usize {
        self.content.len()
    }

    // =========================================================================
    // UTF-8 Index Conversion
    // =========================================================================

    /// Convert character index to byte index
    ///
    /// Returns byte position for the character at `char_idx`.
    /// If `char_idx` is beyond the text, returns the byte length.
    #[inline]
    pub fn char_to_byte(&self, char_idx: usize) -> usize {
        self.content
            .char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap_or(self.content.len())
    }

    /// Convert byte index to character index
    ///
    /// Returns the character position that contains `byte_idx`.
    pub fn byte_to_char(&self, byte_idx: usize) -> usize {
        self.content
            .char_indices()
            .take_while(|(i, _)| *i < byte_idx)
            .count()
    }

    /// Get substring by character range
    pub fn substring(&self, start: usize, end: usize) -> &str {
        let start_byte = self.char_to_byte(start);
        let end_byte = self.char_to_byte(end);
        &self.content[start_byte..end_byte]
    }

    /// Get character at position
    pub fn char_at(&self, pos: usize) -> Option<char> {
        self.content.chars().nth(pos)
    }

    // =========================================================================
    // Content Modification
    // =========================================================================

    /// Insert a character at cursor position
    ///
    /// Returns the new cursor position.
    pub fn insert_char(&mut self, ch: char) -> usize {
        let byte_idx = self.char_to_byte(self.cursor);
        self.content.insert(byte_idx, ch);
        self.cursor += 1;
        self.cursor
    }

    /// Insert a string at cursor position
    ///
    /// Returns the new cursor position.
    pub fn insert_str(&mut self, s: &str) -> usize {
        let byte_idx = self.char_to_byte(self.cursor);
        self.content.insert_str(byte_idx, s);
        self.cursor += s.chars().count();
        self.cursor
    }

    /// Insert a character at a specific position
    ///
    /// Returns the new cursor position (after the inserted char).
    pub fn insert_char_at(&mut self, pos: usize, ch: char) -> usize {
        let byte_idx = self.char_to_byte(pos);
        self.content.insert(byte_idx, ch);
        pos + 1
    }

    /// Insert a string at a specific position
    ///
    /// Returns the new cursor position (after the inserted string).
    pub fn insert_str_at(&mut self, pos: usize, s: &str) -> usize {
        let byte_idx = self.char_to_byte(pos);
        self.content.insert_str(byte_idx, s);
        pos + s.chars().count()
    }

    /// Delete character before cursor (backspace)
    ///
    /// Returns the deleted character, if any.
    pub fn delete_char_before(&mut self) -> Option<char> {
        if self.cursor == 0 {
            return None;
        }

        self.cursor -= 1;
        let byte_idx = self.char_to_byte(self.cursor);
        let ch = self.content.chars().nth(self.cursor)?;
        self.content.drain(byte_idx..byte_idx + ch.len_utf8());
        Some(ch)
    }

    /// Delete character at cursor (delete key)
    ///
    /// Returns the deleted character, if any.
    pub fn delete_char_at(&mut self) -> Option<char> {
        if self.cursor >= self.char_count() {
            return None;
        }

        let byte_idx = self.char_to_byte(self.cursor);
        let ch = self.content.chars().nth(self.cursor)?;
        self.content.drain(byte_idx..byte_idx + ch.len_utf8());
        Some(ch)
    }

    /// Delete a range of characters
    ///
    /// Returns the deleted text.
    pub fn delete_range(&mut self, start: usize, end: usize) -> String {
        let start_byte = self.char_to_byte(start);
        let end_byte = self.char_to_byte(end);
        let deleted: String = self.content.drain(start_byte..end_byte).collect();

        // Adjust cursor if it was in or after the deleted range
        if self.cursor > end {
            self.cursor -= end - start;
        } else if self.cursor > start {
            self.cursor = start;
        }

        deleted
    }

    /// Set content (replaces all text)
    pub fn set_content(&mut self, text: impl Into<String>) {
        self.content = text.into();
        self.cursor = self.char_count();
        self.selection_anchor = None;
    }

    /// Clear all content
    pub fn clear(&mut self) {
        self.content.clear();
        self.cursor = 0;
        self.selection_anchor = None;
    }

    // =========================================================================
    // Cursor Movement
    // =========================================================================

    /// Set cursor position (clamped to valid range)
    pub fn set_cursor(&mut self, pos: usize) {
        self.cursor = pos.min(self.char_count());
    }

    /// Move cursor left by one character
    ///
    /// Returns true if cursor moved.
    pub fn move_left(&mut self) -> bool {
        if self.cursor > 0 {
            self.cursor -= 1;
            true
        } else {
            false
        }
    }

    /// Move cursor right by one character
    ///
    /// Returns true if cursor moved.
    pub fn move_right(&mut self) -> bool {
        if self.cursor < self.char_count() {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    /// Move cursor to start
    pub fn move_to_start(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to end
    pub fn move_to_end(&mut self) {
        self.cursor = self.char_count();
    }

    /// Move cursor left by one word
    ///
    /// A word is a sequence of non-whitespace characters.
    pub fn move_word_left(&mut self) {
        if self.cursor == 0 {
            return;
        }

        let byte_pos = self.char_to_byte(self.cursor);
        let before_cursor = &self.content[..byte_pos];

        let mut new_pos = self.cursor;

        // Skip whitespace going backwards
        for ch in before_cursor.chars().rev() {
            if ch.is_whitespace() {
                new_pos -= 1;
            } else {
                break;
            }
        }

        // Skip word characters going backwards
        let byte_pos = self.char_to_byte(new_pos);
        let before_new_pos = &self.content[..byte_pos];
        for ch in before_new_pos.chars().rev() {
            if !ch.is_whitespace() {
                new_pos -= 1;
            } else {
                break;
            }
        }

        self.cursor = new_pos;
    }

    /// Move cursor right by one word
    ///
    /// A word is a sequence of non-whitespace characters.
    pub fn move_word_right(&mut self) {
        let char_len = self.char_count();
        if self.cursor >= char_len {
            return;
        }

        let byte_pos = self.char_to_byte(self.cursor);
        let after_cursor = &self.content[byte_pos..];

        let mut advance = 0;

        // Skip current word characters
        for ch in after_cursor.chars() {
            if !ch.is_whitespace() {
                advance += 1;
            } else {
                break;
            }
        }

        // Skip whitespace
        let new_byte_pos = self.char_to_byte(self.cursor + advance);
        let remaining = &self.content[new_byte_pos..];
        for ch in remaining.chars() {
            if ch.is_whitespace() {
                advance += 1;
            } else {
                break;
            }
        }

        self.cursor = (self.cursor + advance).min(char_len);
    }

    // =========================================================================
    // Selection
    // =========================================================================

    /// Start selection at current cursor position
    pub fn start_selection(&mut self) {
        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor);
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selection_anchor = None;
    }

    /// Check if there's an active selection
    pub fn has_selection(&self) -> bool {
        self.selection_anchor.is_some() && self.selection_anchor != Some(self.cursor)
    }

    /// Get selection range (start, end) if there is a selection
    ///
    /// The returned range is normalized (start < end).
    pub fn selection(&self) -> Option<(usize, usize)> {
        self.selection_anchor.map(|anchor| {
            if anchor < self.cursor {
                (anchor, self.cursor)
            } else {
                (self.cursor, anchor)
            }
        })
    }

    /// Get selected text
    pub fn selected_text(&self) -> Option<&str> {
        self.selection()
            .map(|(start, end)| self.substring(start, end))
    }

    /// Select all text
    pub fn select_all(&mut self) {
        self.selection_anchor = Some(0);
        self.cursor = self.char_count();
    }

    /// Delete selected text
    ///
    /// Returns the deleted text, if any.
    pub fn delete_selection(&mut self) -> Option<String> {
        if let Some((start, end)) = self.selection() {
            let deleted = self.delete_range(start, end);
            self.cursor = start;
            self.selection_anchor = None;
            Some(deleted)
        } else {
            None
        }
    }

    // =========================================================================
    // Word Operations
    // =========================================================================

    /// Delete word before cursor
    ///
    /// Returns the deleted text.
    pub fn delete_word_before(&mut self) -> String {
        if self.cursor == 0 {
            return String::new();
        }

        let end = self.cursor;
        self.move_word_left();
        let start = self.cursor;

        self.delete_range(start, end)
    }

    /// Delete word after cursor
    ///
    /// Returns the deleted text.
    pub fn delete_word_after(&mut self) -> String {
        let start = self.cursor;
        self.move_word_right();
        let end = self.cursor;
        self.cursor = start;

        self.delete_range(start, end)
    }

    // =========================================================================
    // Character Classification (for word boundaries)
    // =========================================================================

    /// Check if position is at a word boundary
    pub fn is_word_boundary(&self, pos: usize) -> bool {
        if pos == 0 || pos >= self.char_count() {
            return true;
        }

        let prev = self.char_at(pos - 1);
        let curr = self.char_at(pos);

        match (prev, curr) {
            (Some(p), Some(c)) => {
                let p_word = !p.is_whitespace();
                let c_word = !c.is_whitespace();
                p_word != c_word
            }
            _ => true,
        }
    }

    /// Find word boundaries around cursor
    ///
    /// Returns (start, end) of the word at cursor position.
    pub fn word_at_cursor(&self) -> (usize, usize) {
        if self.is_empty() {
            return (0, 0);
        }

        let char_len = self.char_count();
        let mut start = self.cursor.min(char_len.saturating_sub(1));
        let mut end = start;

        // Expand start backwards
        while start > 0 {
            if let Some(ch) = self.char_at(start - 1) {
                if ch.is_whitespace() {
                    break;
                }
                start -= 1;
            } else {
                break;
            }
        }

        // Expand end forwards
        while end < char_len {
            if let Some(ch) = self.char_at(end) {
                if ch.is_whitespace() {
                    break;
                }
                end += 1;
            } else {
                break;
            }
        }

        (start, end)
    }

    /// Select the word at cursor position
    pub fn select_word(&mut self) {
        let (start, end) = self.word_at_cursor();
        self.selection_anchor = Some(start);
        self.cursor = end;
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests;
