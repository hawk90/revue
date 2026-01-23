//! UTF-8 helper methods for character-based operations

use super::types::Input;

impl Input {
    // ─────────────────────────────────────────────────────────────────────────
    // UTF-8 helper methods
    // ─────────────────────────────────────────────────────────────────────────

    /// Get byte index from character index
    pub(super) fn char_to_byte_index(&self, char_idx: usize) -> usize {
        self.value
            .char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap_or(self.value.len())
    }

    /// Get character count
    pub(super) fn char_count(&self) -> usize {
        self.value.chars().count()
    }

    /// Insert string at character position (returns new cursor position)
    pub(super) fn insert_at_char(&mut self, char_idx: usize, s: &str) -> usize {
        let byte_idx = self.char_to_byte_index(char_idx);
        self.value.insert_str(byte_idx, s);
        char_idx + s.chars().count()
    }

    /// Remove character at character position
    pub(super) fn remove_char_at(&mut self, char_idx: usize) {
        let byte_idx = self.char_to_byte_index(char_idx);
        if let Some((_, ch)) = self.value.char_indices().nth(char_idx) {
            self.value.drain(byte_idx..byte_idx + ch.len_utf8());
        }
    }

    /// Remove range of characters (start..end in char indices)
    pub(super) fn remove_char_range(&mut self, start: usize, end: usize) {
        let start_byte = self.char_to_byte_index(start);
        let end_byte = self.char_to_byte_index(end);
        self.value.drain(start_byte..end_byte);
    }

    /// Get substring by character range
    pub(super) fn substring(&self, start: usize, end: usize) -> &str {
        let start_byte = self.char_to_byte_index(start);
        let end_byte = self.char_to_byte_index(end);
        &self.value[start_byte..end_byte]
    }
}
