//! Positions in an [`EditState`](super::EditState)

/// A position in the text: a line, and a column in grapheme clusters (what
/// the user sees as one character) from the line's start. A position is
/// never inside a cluster.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EditPos {
    /// Line, from 0
    pub line: usize,
    /// Column in grapheme clusters, from 0
    pub col: usize,
}

impl EditPos {
    /// The position at `line`, `col`
    pub const fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

impl From<(usize, usize)> for EditPos {
    fn from((line, col): (usize, usize)) -> Self {
        Self { line, col }
    }
}
