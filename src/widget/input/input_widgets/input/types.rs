//! Undo history types for the Input widget

/// Maximum undo history size
pub const MAX_UNDO_HISTORY: usize = 100;

/// An edit operation for undo/redo (single-line input)
#[derive(Clone, Debug)]
pub enum EditOperation {
    /// Insert text at position
    Insert { pos: usize, text: String },
    /// Delete text at position
    Delete { pos: usize, text: String },
    /// Replace entire value (for paste over selection, etc.)
    Replace {
        old_value: String,
        old_cursor: usize,
        new_value: String,
        new_cursor: usize,
    },
}
