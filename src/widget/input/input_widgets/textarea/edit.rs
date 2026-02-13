//! Edit operations for TextArea undo/redo

/// An edit operation for undo/redo
#[derive(Clone, Debug)]
pub enum EditOperation {
    /// Insert text at position
    Insert {
        line: usize,
        col: usize,
        text: String,
    },
    /// Delete text at position
    Delete {
        line: usize,
        col: usize,
        text: String,
    },
    /// Insert a new line
    InsertLine { line: usize, content: String },
    /// Delete a line
    DeleteLine { line: usize, content: String },
    /// Merge with previous line
    MergeLines { line: usize, col: usize },
    /// Split line at position
    SplitLine { line: usize, col: usize },
}

// # KEEP HERE - Private tests that cannot be extracted
