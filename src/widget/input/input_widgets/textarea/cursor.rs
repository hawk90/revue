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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_pos_new() {
        let pos = CursorPos::new(5, 10);
        assert_eq!(pos.line, 5);
        assert_eq!(pos.col, 10);
    }

    #[test]
    fn test_cursor_pos_from_tuple() {
        let pos: CursorPos = (3, 7).into();
        assert_eq!(pos.line, 3);
        assert_eq!(pos.col, 7);
    }

    #[test]
    fn test_cursor_pos_into_tuple() {
        let pos = CursorPos::new(2, 4);
        let tuple: (usize, usize) = pos.into();
        assert_eq!(tuple, (2, 4));
    }

    #[test]
    fn test_cursor_new() {
        let cursor = Cursor::new(CursorPos::new(1, 2));
        assert_eq!(cursor.pos, CursorPos::new(1, 2));
        assert!(cursor.anchor.is_none());
        assert!(!cursor.is_selecting());
    }

    #[test]
    fn test_cursor_with_selection() {
        let cursor = Cursor::with_selection(CursorPos::new(1, 5), CursorPos::new(1, 0));
        assert_eq!(cursor.pos, CursorPos::new(1, 5));
        assert_eq!(cursor.anchor, Some(CursorPos::new(1, 0)));
        assert!(cursor.is_selecting());
    }

    #[test]
    fn test_cursor_selection() {
        let cursor = Cursor::with_selection(CursorPos::new(1, 5), CursorPos::new(1, 0));
        let sel = cursor.selection().unwrap();
        assert_eq!(sel.start, (1, 0));
        assert_eq!(sel.end, (1, 5));
    }

    #[test]
    fn test_cursor_start_clear_selection() {
        let mut cursor = Cursor::new(CursorPos::new(1, 5));
        assert!(!cursor.is_selecting());

        cursor.start_selection();
        assert!(cursor.is_selecting());
        assert_eq!(cursor.anchor, Some(CursorPos::new(1, 5)));

        cursor.clear_selection();
        assert!(!cursor.is_selecting());
    }

    #[test]
    fn test_cursor_set_new() {
        let set = CursorSet::new(CursorPos::new(2, 3));
        assert_eq!(set.len(), 1);
        assert_eq!(set.primary().pos, CursorPos::new(2, 3));
    }

    #[test]
    fn test_cursor_set_default() {
        let set = CursorSet::default();
        assert_eq!(set.len(), 1);
        assert_eq!(set.primary().pos, CursorPos::new(0, 0));
    }

    #[test]
    fn test_cursor_set_add() {
        let mut set = CursorSet::new(CursorPos::new(0, 0));
        set.add_at(CursorPos::new(1, 0));
        set.add_at(CursorPos::new(2, 0));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_cursor_set_add_duplicate() {
        let mut set = CursorSet::new(CursorPos::new(0, 0));
        set.add_at(CursorPos::new(0, 0)); // duplicate
        assert_eq!(set.len(), 1); // normalized away
    }

    #[test]
    fn test_cursor_set_clear_secondary() {
        let mut set = CursorSet::new(CursorPos::new(0, 0));
        set.add_at(CursorPos::new(1, 0));
        set.add_at(CursorPos::new(2, 0));
        assert_eq!(set.len(), 3);

        set.clear_secondary();
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_cursor_set_set_primary() {
        let mut set = CursorSet::new(CursorPos::new(0, 0));
        set.set_primary(CursorPos::new(5, 10));
        assert_eq!(set.primary().pos, CursorPos::new(5, 10));
    }

    #[test]
    fn test_cursor_set_is_empty() {
        let set = CursorSet::default();
        assert!(!set.is_empty()); // always has at least one
    }

    #[test]
    fn test_cursor_set_is_single() {
        let mut set = CursorSet::default();
        assert!(set.is_single());

        set.add_at(CursorPos::new(1, 0));
        assert!(!set.is_single());
    }

    #[test]
    fn test_cursor_set_positions_reversed() {
        let mut set = CursorSet::new(CursorPos::new(0, 0));
        set.add_at(CursorPos::new(2, 5));
        set.add_at(CursorPos::new(1, 3));

        let positions = set.positions_reversed();
        assert_eq!(positions[0], CursorPos::new(2, 5));
        assert_eq!(positions[1], CursorPos::new(1, 3));
        assert_eq!(positions[2], CursorPos::new(0, 0));
    }

    #[test]
    fn test_cursor_set_max_cursors() {
        let mut set = CursorSet::default();
        for i in 0..MAX_CURSORS + 10 {
            set.add_at(CursorPos::new(i, 0));
        }
        assert_eq!(set.len(), MAX_CURSORS);
    }

    // =========================================================================
    // CursorPos trait tests
    // =========================================================================

    #[test]
    fn test_cursor_pos_clone() {
        let pos1 = CursorPos::new(5, 10);
        let pos2 = pos1.clone();
        assert_eq!(pos1.line, pos2.line);
        assert_eq!(pos1.col, pos2.col);
    }

    #[test]
    fn test_cursor_pos_copy() {
        let pos1 = CursorPos::new(3, 7);
        let pos2 = pos1;
        assert_eq!(pos1.line, 3);
        assert_eq!(pos1.col, 7);
        assert_eq!(pos2.line, 3);
        assert_eq!(pos2.col, 7);
    }

    #[test]
    fn test_cursor_pos_partial_eq() {
        let pos1 = CursorPos::new(5, 10);
        let pos2 = CursorPos::new(5, 10);
        let pos3 = CursorPos::new(5, 11);
        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }

    #[test]
    fn test_cursor_pos_equality() {
        assert_eq!(CursorPos::new(0, 0), CursorPos::new(0, 0));
        assert_ne!(CursorPos::new(0, 0), CursorPos::new(0, 1));
        assert_ne!(CursorPos::new(0, 0), CursorPos::new(1, 0));
    }

    #[test]
    fn test_cursor_pos_partial_ord() {
        let pos1 = CursorPos::new(1, 5);
        let pos2 = CursorPos::new(1, 10);
        let pos3 = CursorPos::new(2, 0);
        assert!(pos1 < pos2);
        assert!(pos2 < pos3);
        assert!(pos1 <= pos2);
        assert!(pos2 > pos1);
        assert!(pos3 >= pos2);
    }

    #[test]
    fn test_cursor_pos_ord() {
        let pos1 = CursorPos::new(1, 5);
        let pos2 = CursorPos::new(1, 10);
        let pos3 = CursorPos::new(2, 0);
        assert!(pos1.cmp(&pos2).is_lt());
        assert!(pos2.cmp(&pos3).is_lt());
        assert!(pos3.cmp(&pos1).is_gt());
    }

    #[test]
    fn test_cursor_pos_debug() {
        let pos = CursorPos::new(5, 10);
        let debug_str = format!("{:?}", pos);
        assert!(debug_str.contains("CursorPos"));
    }

    #[test]
    fn test_cursor_pos_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let pos1 = CursorPos::new(5, 10);
        let pos2 = CursorPos::new(5, 10);
        let pos3 = CursorPos::new(5, 11);

        let mut hasher = DefaultHasher::new();
        pos1.hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        pos2.hash(&mut hasher);
        let hash2 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        pos3.hash(&mut hasher);
        let hash3 = hasher.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    // =========================================================================
    // Cursor trait tests
    // =========================================================================

    #[test]
    fn test_cursor_clone() {
        let cursor1 = Cursor::with_selection(CursorPos::new(1, 5), CursorPos::new(1, 0));
        let cursor2 = cursor1.clone();
        assert_eq!(cursor1.pos, cursor2.pos);
        assert_eq!(cursor1.anchor, cursor2.anchor);
    }

    #[test]
    fn test_cursor_copy() {
        let cursor1 = Cursor::new(CursorPos::new(1, 2));
        let cursor2 = cursor1;
        assert_eq!(cursor1.pos, CursorPos::new(1, 2));
        assert_eq!(cursor2.pos, CursorPos::new(1, 2));
    }

    #[test]
    fn test_cursor_partial_eq() {
        let pos = CursorPos::new(1, 2);
        let cursor1 = Cursor::new(pos);
        let cursor2 = Cursor::new(pos);
        assert_eq!(cursor1, cursor2);
    }

    #[test]
    fn test_cursor_eq_ne() {
        let cursor1 = Cursor::new(CursorPos::new(1, 2));
        let cursor2 = Cursor::new(CursorPos::new(1, 2));
        let cursor3 = Cursor::new(CursorPos::new(1, 3));
        assert_eq!(cursor1, cursor2);
        assert_ne!(cursor1, cursor3);
    }

    #[test]
    fn test_cursor_debug() {
        let cursor = Cursor::new(CursorPos::new(1, 2));
        let debug_str = format!("{:?}", cursor);
        assert!(debug_str.contains("Cursor"));
    }

    #[test]
    fn test_cursor_selection_none() {
        let cursor = Cursor::new(CursorPos::new(1, 2));
        assert!(cursor.selection().is_none());
    }

    #[test]
    fn test_cursor_selection_reversed() {
        let cursor = Cursor::with_selection(CursorPos::new(1, 0), CursorPos::new(1, 5));
        let sel = cursor.selection().unwrap();
        // Should normalize
        assert_eq!(sel.start, (1, 0));
        assert_eq!(sel.end, (1, 5));
    }

    #[test]
    fn test_cursor_selection_multiline() {
        let cursor = Cursor::with_selection(CursorPos::new(3, 10), CursorPos::new(1, 5));
        let sel = cursor.selection().unwrap();
        assert_eq!(sel.start, (1, 5));
        assert_eq!(sel.end, (3, 10));
    }

    // =========================================================================
    // CursorSet trait tests
    // =========================================================================

    #[test]
    fn test_cursor_set_clone() {
        let mut set1 = CursorSet::new(CursorPos::new(0, 0));
        set1.add_at(CursorPos::new(1, 0));
        set1.add_at(CursorPos::new(2, 0));
        let set2 = set1.clone();
        assert_eq!(set1.len(), set2.len());
        assert_eq!(set1.primary().pos, set2.primary().pos);
    }

    #[test]
    fn test_cursor_set_debug() {
        let set = CursorSet::new(CursorPos::new(1, 2));
        let debug_str = format!("{:?}", set);
        assert!(debug_str.contains("CursorSet"));
    }

    #[test]
    fn test_cursor_set_iter() {
        let mut set = CursorSet::new(CursorPos::new(0, 0));
        set.add_at(CursorPos::new(1, 0));
        set.add_at(CursorPos::new(2, 0));

        let count = set.iter().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_cursor_set_all() {
        let mut set = CursorSet::new(CursorPos::new(0, 0));
        set.add_at(CursorPos::new(1, 0));
        set.add_at(CursorPos::new(2, 0));

        let all = set.all();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_cursor_set_all_mut() {
        let mut set = CursorSet::new(CursorPos::new(0, 0));
        set.add_at(CursorPos::new(1, 0));

        let all_mut = set.all_mut();
        assert_eq!(all_mut.len(), 2);
    }

    #[test]
    fn test_cursor_set_iter_mut() {
        let mut set = CursorSet::new(CursorPos::new(0, 0));
        set.add_at(CursorPos::new(1, 0));

        let count = set.iter_mut().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_cursor_set_add_cursor_object() {
        let mut set = CursorSet::new(CursorPos::new(0, 0));
        let cursor = Cursor::with_selection(CursorPos::new(1, 5), CursorPos::new(1, 0));
        set.add(cursor);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_cursor_set_add_beyond_max() {
        let mut set = CursorSet::default();
        for i in 0..=MAX_CURSORS {
            set.add_at(CursorPos::new(i, 0));
        }
        // Should be capped at MAX_CURSORS
        assert_eq!(set.len(), MAX_CURSORS);
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_cursor_pos_zero_position() {
        let pos = CursorPos::new(0, 0);
        assert_eq!(pos.line, 0);
        assert_eq!(pos.col, 0);
    }

    #[test]
    fn test_cursor_pos_large_values() {
        let pos = CursorPos::new(999999, 999999);
        assert_eq!(pos.line, 999999);
        assert_eq!(pos.col, 999999);
    }

    #[test]
    fn test_cursor_with_same_anchor_and_pos() {
        let cursor = Cursor::with_selection(CursorPos::new(1, 5), CursorPos::new(1, 5));
        assert!(cursor.is_selecting());
        // Selection with same start and end is valid but empty
        let sel = cursor.selection().unwrap();
        assert_eq!(sel.start, (1, 5));
        assert_eq!(sel.end, (1, 5));
    }

    #[test]
    fn test_cursor_set_primary_mut() {
        let mut set = CursorSet::new(CursorPos::new(0, 0));
        set.primary_mut().pos = CursorPos::new(5, 10);
        assert_eq!(set.primary().pos, CursorPos::new(5, 10));
    }

    #[test]
    fn test_cursor_set_clear_secondary_with_one() {
        let mut set = CursorSet::new(CursorPos::new(0, 0));
        set.clear_secondary();
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_cursor_set_clear_secondary_with_many() {
        let mut set = CursorSet::new(CursorPos::new(0, 0));
        set.add_at(CursorPos::new(1, 0));
        set.add_at(CursorPos::new(2, 0));
        set.add_at(CursorPos::new(3, 0));
        set.clear_secondary();
        assert_eq!(set.len(), 1);
        assert_eq!(set.primary().pos, CursorPos::new(0, 0));
    }
}
