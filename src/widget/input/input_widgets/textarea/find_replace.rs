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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // FindReplaceMode Tests
    // =========================================================================

    #[test]
    fn test_find_replace_mode_default() {
        assert_eq!(FindReplaceMode::default(), FindReplaceMode::Find);
    }

    #[test]
    fn test_find_replace_mode_equality() {
        assert_eq!(FindReplaceMode::Find, FindReplaceMode::Find);
        assert_eq!(FindReplaceMode::Replace, FindReplaceMode::Replace);
        assert_ne!(FindReplaceMode::Find, FindReplaceMode::Replace);
    }

    // =========================================================================
    // FindOptions Tests
    // =========================================================================

    #[test]
    fn test_find_options_default() {
        let opts = FindOptions::default();
        assert!(!opts.case_sensitive);
        assert!(!opts.whole_word);
        assert!(!opts.use_regex);
    }

    #[test]
    fn test_find_options_custom() {
        let opts = FindOptions {
            case_sensitive: true,
            whole_word: true,
            use_regex: false,
        };
        assert!(opts.case_sensitive);
        assert!(opts.whole_word);
        assert!(!opts.use_regex);
    }

    // =========================================================================
    // FindMatch Tests
    // =========================================================================

    #[test]
    fn test_find_match_new() {
        let start = CursorPos { line: 0, col: 5 };
        let end = CursorPos { line: 0, col: 10 };
        let match_result = FindMatch::new(start, end);

        assert_eq!(match_result.start.line, 0);
        assert_eq!(match_result.start.col, 5);
        assert_eq!(match_result.end.line, 0);
        assert_eq!(match_result.end.col, 10);
    }

    #[test]
    fn test_find_match_equality() {
        let m1 = FindMatch::new(CursorPos { line: 0, col: 0 }, CursorPos { line: 0, col: 5 });
        let m2 = FindMatch::new(CursorPos { line: 0, col: 0 }, CursorPos { line: 0, col: 5 });
        let m3 = FindMatch::new(CursorPos { line: 1, col: 0 }, CursorPos { line: 1, col: 5 });

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn test_find_match_clone() {
        let m = FindMatch::new(CursorPos { line: 2, col: 3 }, CursorPos { line: 2, col: 8 });
        let cloned = m.clone();
        assert_eq!(m, cloned);
    }

    // =========================================================================
    // FindReplaceState Tests
    // =========================================================================

    #[test]
    fn test_find_replace_state_default() {
        let state = FindReplaceState::default();
        assert!(state.query.is_empty());
        assert!(state.replace_with.is_empty());
        assert!(state.matches.is_empty());
        assert!(state.current_match.is_none());
        assert_eq!(state.mode, FindReplaceMode::Find);
    }

    #[test]
    fn test_find_replace_state_new_find() {
        let state = FindReplaceState::new(FindReplaceMode::Find);
        assert_eq!(state.mode, FindReplaceMode::Find);
        assert!(state.query_focused);
    }

    #[test]
    fn test_find_replace_state_new_replace() {
        let state = FindReplaceState::new(FindReplaceMode::Replace);
        assert_eq!(state.mode, FindReplaceMode::Replace);
        assert!(state.query_focused);
    }

    #[test]
    fn test_find_replace_state_match_count_empty() {
        let state = FindReplaceState::default();
        assert_eq!(state.match_count(), 0);
    }

    #[test]
    fn test_find_replace_state_match_count() {
        let mut state = FindReplaceState::default();
        state.matches = vec![
            FindMatch::new(CursorPos { line: 0, col: 0 }, CursorPos { line: 0, col: 5 }),
            FindMatch::new(CursorPos { line: 1, col: 0 }, CursorPos { line: 1, col: 5 }),
            FindMatch::new(CursorPos { line: 2, col: 0 }, CursorPos { line: 2, col: 5 }),
        ];
        assert_eq!(state.match_count(), 3);
    }

    #[test]
    fn test_find_replace_state_current_match_display_none() {
        let state = FindReplaceState::default();
        assert_eq!(state.current_match_display(), 0);
    }

    #[test]
    fn test_find_replace_state_current_match_display() {
        let mut state = FindReplaceState::default();
        state.matches = vec![
            FindMatch::new(CursorPos { line: 0, col: 0 }, CursorPos { line: 0, col: 5 }),
            FindMatch::new(CursorPos { line: 1, col: 0 }, CursorPos { line: 1, col: 5 }),
        ];
        state.current_match = Some(0);
        assert_eq!(state.current_match_display(), 1);

        state.current_match = Some(1);
        assert_eq!(state.current_match_display(), 2);
    }

    #[test]
    fn test_find_replace_state_query_focused() {
        let state = FindReplaceState::new(FindReplaceMode::Find);
        assert!(state.query_focused);
    }

    #[test]
    fn test_find_replace_state_with_data() {
        let mut state = FindReplaceState::new(FindReplaceMode::Replace);
        state.query = "search".to_string();
        state.replace_with = "replace".to_string();
        state.options = FindOptions {
            case_sensitive: true,
            whole_word: false,
            use_regex: false,
        };

        assert_eq!(state.query, "search");
        assert_eq!(state.replace_with, "replace");
        assert!(state.options.case_sensitive);
    }
}
