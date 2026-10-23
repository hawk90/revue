//! Character indices for the public methods, over the edit state's
//! grapheme columns

use unicode_segmentation::UnicodeSegmentation;

use super::Input;
#[cfg(test)]
use crate::utils::EditPos;

impl Input {
    /// The text (an Input has one line)
    pub(super) fn line(&self) -> &str {
        self.state.line(0).unwrap_or("")
    }

    /// The character index where column `col` starts
    pub(super) fn char_of(&self, col: usize) -> usize {
        self.line()
            .graphemes(true)
            .take(col)
            .map(|g| g.chars().count())
            .sum()
    }

    /// The column of character index `index`; inside a cluster it rounds
    /// up to the cluster's end
    #[cfg(test)]
    pub(super) fn col_of_char(&self, index: usize) -> usize {
        let line = self.line();
        let byte = line
            .char_indices()
            .nth(index)
            .map_or(line.len(), |(i, _)| i);
        line.grapheme_indices(true)
            .take_while(|&(i, _)| i < byte)
            .count()
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
        let col = self.col_of_char(index);
        self.state.set_cursor(EditPos::new(0, col));
    }

    /// A selection from `anchor` to `head`, character indices (test setup)
    #[cfg(test)]
    pub(super) fn select_chars(&mut self, anchor: usize, head: usize) {
        let (a, h) = (self.col_of_char(anchor), self.col_of_char(head));
        self.state.select(EditPos::new(0, a), EditPos::new(0, h));
    }
}
