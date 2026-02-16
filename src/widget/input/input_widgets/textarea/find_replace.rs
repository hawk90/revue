//! Find and replace functionality for TextArea

use super::cursor::CursorPos;

/// Find/Replace mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FindReplaceMode {
    /// Find mode - search for text
    #[default]
    Find,
    /// Replace mode - find and replace text
    Replace,
}

/// Options for find/replace operations
#[derive(Clone, Debug, Default)]
pub struct FindOptions {
    /// Case-sensitive search
    pub case_sensitive: bool,
    /// Match whole words only
    pub whole_word: bool,
    /// Use regex pattern
    pub use_regex: bool,
}

/// A match found in the text
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FindMatch {
    /// Start position
    pub start: CursorPos,
    /// End position
    pub end: CursorPos,
}

impl FindMatch {
    /// Create a new find match
    pub fn new(start: CursorPos, end: CursorPos) -> Self {
        Self { start, end }
    }
}

/// Find/Replace state
#[derive(Clone, Debug, Default)]
pub struct FindReplaceState {
    /// Search query
    pub query: String,
    /// Replacement text
    pub replace_with: String,
    /// Search options
    pub options: FindOptions,
    /// All matches in document
    pub matches: Vec<FindMatch>,
    /// Currently focused match index
    pub current_match: Option<usize>,
    /// UI mode (Find or Replace)
    pub mode: FindReplaceMode,
    /// Input focus: true = query input, false = replace input
    pub query_focused: bool,
}

impl FindReplaceState {
    /// Create a new find/replace state
    pub fn new(mode: FindReplaceMode) -> Self {
        Self {
            mode,
            query_focused: true,
            ..Default::default()
        }
    }

    /// Get match count
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Get current match (1-indexed for display)
    pub fn current_match_display(&self) -> usize {
        self.current_match.map(|i| i + 1).unwrap_or(0)
    }
}

// # KEEP HERE - Private tests that cannot be extracted
