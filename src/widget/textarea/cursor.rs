//! Cursor types for TextArea
//!
//! Provides cursor positioning and multi-cursor support.

use super::selection::Selection;

/// Maximum number of cursors allowed
pub const MAX_CURSORS: usize = 100;

/// A cursor position in the text (line, column)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CursorPos {
    /// Line index (0-based)
    pub line: usize,
    /// Column index (0-based)
    pub col: usize,
}

impl CursorPos {
    /// Create a new cursor position
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

impl From<(usize, usize)> for CursorPos {
    fn from((line, col): (usize, usize)) -> Self {
        Self { line, col }
    }
}

impl From<CursorPos> for (usize, usize) {
    fn from(pos: CursorPos) -> Self {
        (pos.line, pos.col)
    }
}

/// A cursor with optional selection anchor
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor {
    /// Current position
    pub pos: CursorPos,
    /// Selection anchor (if selecting)
    pub anchor: Option<CursorPos>,
}

impl Cursor {
    /// Create a new cursor at position
    pub fn new(pos: CursorPos) -> Self {
        Self { pos, anchor: None }
    }

    /// Create a cursor with selection
    pub fn with_selection(pos: CursorPos, anchor: CursorPos) -> Self {
        Self {
            pos,
            anchor: Some(anchor),
        }
    }

    /// Get the selection as a Selection struct if selecting
    pub fn selection(&self) -> Option<Selection> {
        self.anchor.map(|anchor| {
            let start = if self.pos < anchor {
                (self.pos.line, self.pos.col)
            } else {
                (anchor.line, anchor.col)
            };
            let end = if self.pos < anchor {
                (anchor.line, anchor.col)
            } else {
                (self.pos.line, self.pos.col)
            };
            Selection::new(start, end)
        })
    }

    /// Check if this cursor is selecting
    pub fn is_selecting(&self) -> bool {
        self.anchor.is_some()
    }

    /// Start selection at current position
    pub fn start_selection(&mut self) {
        self.anchor = Some(self.pos);
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.anchor = None;
    }
}

/// Collection of cursors (always has at least one - the primary cursor)
#[derive(Clone, Debug)]
pub struct CursorSet {
    /// All cursors, primary is at index 0
    cursors: Vec<Cursor>,
}

impl CursorSet {
    /// Create a new cursor set with a single cursor at position
    pub fn new(pos: CursorPos) -> Self {
        Self {
            cursors: vec![Cursor::new(pos)],
        }
    }

    /// Get the primary cursor (immutable)
    pub fn primary(&self) -> &Cursor {
        &self.cursors[0]
    }

    /// Get the primary cursor (mutable)
    pub fn primary_mut(&mut self) -> &mut Cursor {
        &mut self.cursors[0]
    }

    /// Get all cursors
    #[allow(dead_code)]
    pub fn all(&self) -> &[Cursor] {
        &self.cursors
    }

    /// Get all cursors (mutable)
    #[allow(dead_code)]
    pub fn all_mut(&mut self) -> &mut [Cursor] {
        &mut self.cursors
    }

    /// Get the number of cursors
    pub fn len(&self) -> usize {
        self.cursors.len()
    }

    /// Check if empty (always false - always has at least one cursor)
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Check if there's only one cursor
    #[allow(dead_code)]
    pub fn is_single(&self) -> bool {
        self.cursors.len() == 1
    }

    /// Add a cursor at position
    pub fn add(&mut self, cursor: Cursor) {
        if self.cursors.len() < MAX_CURSORS {
            self.cursors.push(cursor);
            self.normalize();
        }
    }

    /// Add a cursor at position (convenience method)
    pub fn add_at(&mut self, pos: CursorPos) {
        self.add(Cursor::new(pos));
    }

    /// Clear all secondary cursors, keeping only the primary
    pub fn clear_secondary(&mut self) {
        self.cursors.truncate(1);
    }

    /// Set the primary cursor position
    pub fn set_primary(&mut self, pos: CursorPos) {
        self.cursors[0].pos = pos;
    }

    /// Iterate over all cursors
    pub fn iter(&self) -> impl Iterator<Item = &Cursor> {
        self.cursors.iter()
    }

    /// Iterate over all cursors (mutable)
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cursor> {
        self.cursors.iter_mut()
    }

    /// Sort cursors by position and merge overlapping ones
    fn normalize(&mut self) {
        // Sort by position (line, then column)
        self.cursors.sort_by(|a, b| a.pos.cmp(&b.pos));

        // Remove duplicates (cursors at same position)
        self.cursors.dedup_by(|a, b| a.pos == b.pos);

        // Ensure we always have at least one cursor
        if self.cursors.is_empty() {
            self.cursors.push(Cursor::new(CursorPos::new(0, 0)));
        }
    }

    /// Get positions of all cursors sorted in reverse order (for editing)
    #[allow(dead_code)]
    pub fn positions_reversed(&self) -> Vec<CursorPos> {
        let mut positions: Vec<CursorPos> = self.cursors.iter().map(|c| c.pos).collect();
        positions.sort_by(|a, b| b.cmp(a)); // Reverse order
        positions
    }
}

impl Default for CursorSet {
    fn default() -> Self {
        Self::new(CursorPos::new(0, 0))
    }
}
