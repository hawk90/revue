//! Character indices for the public methods, over the edit state's
//! grapheme columns

use super::Input;
use crate::utils::EditPos;

impl Input {
    /// The text (an Input has one line)
    pub(super) fn line(&self) -> &str {
        self.state.line(0).unwrap_or("")
    }

    /// The character index where column `col` starts
    pub(super) fn char_of(&self, col: usize) -> usize {
        self.state.char_index(EditPos::new(0, col))
    }

    /// The text between two character indices
    pub(super) fn substring(&self, start: usize, end: usize) -> &str {
        let line = self.line();
        let byte = |index: usize| {
            line.char_indices()
                .nth(index)
                .map_or(line.len(), |(i, _)| i)
        };
        &line[byte(start)..byte(end)]
    }

    /// The cursor at character index `index` (test setup)
    #[cfg(test)]
    pub(super) fn set_cursor_chars(&mut self, index: usize) {
        let pos = self.state.pos_at_char(0, index);
        self.state.set_cursor(pos);
    }

    /// A selection from `anchor` to `head`, character indices (test setup)
    #[cfg(test)]
    pub(super) fn select_chars(&mut self, anchor: usize, head: usize) {
        let (a, h) = (
            self.state.pos_at_char(0, anchor),
            self.state.pos_at_char(0, head),
        );
        self.state.select(a, h);
    }
}
