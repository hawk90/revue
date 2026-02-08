//! Code editor types and configuration

/// Maximum undo history size
pub const MAX_UNDO_HISTORY: usize = 100;

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Constants
    // =========================================================================

    #[test]
    fn test_max_undo_history_value() {
        assert_eq!(MAX_UNDO_HISTORY, 100);
    }

    // =========================================================================
    // BracketPair tests
    // =========================================================================

    #[test]
    fn test_bracket_pair_new() {
        let pair = BracketPair {
            open: (0, 0),
            close: (0, 5),
        };
        assert_eq!(pair.open, (0, 0));
        assert_eq!(pair.close, (0, 5));
    }

    #[test]
    fn test_bracket_pair_clone() {
        let pair1 = BracketPair {
            open: (1, 2),
            close: (3, 4),
        };
        let pair2 = pair1;
        assert_eq!(pair1.open, pair2.open);
        assert_eq!(pair1.close, pair2.close);
    }

    #[test]
    fn test_bracket_pair_copy() {
        let pair1 = BracketPair {
            open: (0, 1),
            close: (2, 3),
        };
        let pair2 = pair1;
        assert_eq!(pair1, pair2);
    }

    #[test]
    fn test_bracket_pair_equality() {
        let pair1 = BracketPair {
            open: (0, 0),
            close: (0, 5),
        };
        let pair2 = BracketPair {
            open: (0, 0),
            close: (0, 5),
        };
        let pair3 = BracketPair {
            open: (1, 0),
            close: (0, 5),
        };
        assert_eq!(pair1, pair2);
        assert_ne!(pair1, pair3);
    }

    #[test]
    fn test_bracket_pair_debug() {
        let pair = BracketPair {
            open: (0, 0),
            close: (0, 5),
        };
        let debug_str = format!("{:?}", pair);
        assert!(debug_str.contains("BracketPair"));
    }

    // =========================================================================
    // BracketMatch tests
    // =========================================================================

    #[test]
    fn test_bracket_match_new() {
        let match_result = BracketMatch {
            position: (0, 5),
            char: ')',
        };
        assert_eq!(match_result.position, (0, 5));
        assert_eq!(match_result.char, ')');
    }

    #[test]
    fn test_bracket_match_clone() {
        let match1 = BracketMatch {
            position: (1, 2),
            char: ']',
        };
        let match2 = match1.clone();
        assert_eq!(match1.position, match2.position);
        assert_eq!(match1.char, match2.char);
    }

    #[test]
    fn test_bracket_match_copy() {
        let match1 = BracketMatch {
            position: (0, 1),
            char: '}',
        };
        let match2 = match1;
        assert_eq!(match1.position, match2.position);
        assert_eq!(match1.char, match2.char);
    }

    #[test]
    fn test_bracket_match_debug() {
        let match_result = BracketMatch {
            position: (0, 5),
            char: ')',
        };
        let debug_str = format!("{:?}", match_result);
        assert!(debug_str.contains("BracketMatch"));
    }

    // =========================================================================
    // IndentStyle enum tests
    // =========================================================================

    #[test]
    fn test_indent_style_default() {
        assert_eq!(IndentStyle::default(), IndentStyle::Spaces);
    }

    #[test]
    fn test_indent_style_clone() {
        let style = IndentStyle::Tabs;
        assert_eq!(style, style.clone());
    }

    #[test]
    fn test_indent_style_copy() {
        let style1 = IndentStyle::Spaces;
        let style2 = style1;
        assert_eq!(style1, IndentStyle::Spaces);
        assert_eq!(style2, IndentStyle::Spaces);
    }

    #[test]
    fn test_indent_style_equality() {
        assert_eq!(IndentStyle::Spaces, IndentStyle::Spaces);
        assert_eq!(IndentStyle::Tabs, IndentStyle::Tabs);
        assert_ne!(IndentStyle::Spaces, IndentStyle::Tabs);
    }

    #[test]
    fn test_indent_style_debug() {
        let debug_str = format!("{:?}", IndentStyle::Tabs);
        assert!(debug_str.contains("Tabs"));
    }

    // =========================================================================
    // EditorConfig tests
    // =========================================================================

    #[test]
    fn test_editor_config_default() {
        let config = EditorConfig::default();
        assert_eq!(config.indent_style, IndentStyle::Spaces);
        assert_eq!(config.indent_size, 4);
        assert!(config.auto_indent);
        assert!(config.bracket_matching);
        assert!(config.highlight_current_line);
        assert!(!config.show_minimap);
        assert_eq!(config.minimap_width, 10);
        assert!(!config.show_whitespace);
        assert!(!config.word_wrap);
    }

    #[test]
    fn test_editor_config_clone() {
        let config1 = EditorConfig::default();
        let config2 = config1.clone();
        assert_eq!(config1.indent_style, config2.indent_style);
        assert_eq!(config1.indent_size, config2.indent_size);
    }

    #[test]
    fn test_editor_config_debug() {
        let config = EditorConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("EditorConfig"));
    }

    // =========================================================================
    // EditOp enum tests
    // =========================================================================

    #[test]
    fn test_edit_op_insert() {
        let op = EditOp::Insert {
            line: 0,
            col: 5,
            text: "hello".to_string(),
        };
        assert_eq!(matches!(op, EditOp::Insert { .. }), true);
    }

    #[test]
    fn test_edit_op_delete() {
        let op = EditOp::Delete {
            line: 1,
            col: 3,
            text: "x".to_string(),
        };
        assert_eq!(matches!(op, EditOp::Delete { .. }), true);
    }

    #[test]
    fn test_edit_op_split_line() {
        let op = EditOp::SplitLine { line: 0, col: 5 };
        assert_eq!(matches!(op, EditOp::SplitLine { .. }), true);
    }

    #[test]
    fn test_edit_op_merge_line() {
        let op = EditOp::MergeLine { line: 1, col: 0 };
        assert_eq!(matches!(op, EditOp::MergeLine { .. }), true);
    }

    #[test]
    fn test_edit_op_clone() {
        let op1 = EditOp::Insert {
            line: 0,
            col: 0,
            text: "test".to_string(),
        };
        let op2 = op1.clone();
        assert!(matches!(op2, EditOp::Insert { .. }));
    }

    #[test]
    fn test_edit_op_debug() {
        let op = EditOp::Insert {
            line: 0,
            col: 0,
            text: "test".to_string(),
        };
        let debug_str = format!("{:?}", op);
        assert!(debug_str.contains("Insert"));
    }
}

/// Bracket pair for matching
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BracketPair {
    /// Opening bracket position (line, col)
    pub open: (usize, usize),
    /// Closing bracket position (line, col)
    pub close: (usize, usize),
}

/// A bracket match result
#[derive(Clone, Copy, Debug)]
pub struct BracketMatch {
    /// Position of the matching bracket
    pub position: (usize, usize),
    /// The matching character
    pub char: char,
}

/// Indent style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IndentStyle {
    /// Use spaces for indentation
    #[default]
    Spaces,
    /// Use tabs for indentation
    Tabs,
}

/// Code editor configuration
#[derive(Clone, Debug)]
pub struct EditorConfig {
    /// Indent style (spaces or tabs)
    pub indent_style: IndentStyle,
    /// Indent size (number of spaces or tab width)
    pub indent_size: usize,
    /// Enable auto-indent on newline
    pub auto_indent: bool,
    /// Enable bracket matching
    pub bracket_matching: bool,
    /// Enable current line highlight
    pub highlight_current_line: bool,
    /// Enable minimap
    pub show_minimap: bool,
    /// Minimap width
    pub minimap_width: u16,
    /// Show whitespace characters
    pub show_whitespace: bool,
    /// Enable word wrap
    pub word_wrap: bool,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            indent_style: IndentStyle::Spaces,
            indent_size: 4,
            auto_indent: true,
            bracket_matching: true,
            highlight_current_line: true,
            show_minimap: false,
            minimap_width: 10,
            show_whitespace: false,
            word_wrap: false,
        }
    }
}

/// Edit operation for undo/redo
#[derive(Clone, Debug)]
pub enum EditOp {
    /// Insert text at position
    Insert {
        /// Line number
        line: usize,
        /// Column number
        col: usize,
        /// Text to insert
        text: String,
    },
    /// Delete text at position
    Delete {
        /// Line number
        line: usize,
        /// Column number
        col: usize,
        /// Text that was deleted
        text: String,
    },
    /// Split line at position (Enter key)
    SplitLine {
        /// Line number
        line: usize,
        /// Column number
        col: usize,
    },
    /// Merge with next line (Backspace at line start)
    MergeLine {
        /// Line number
        line: usize,
        /// Column number
        col: usize,
    },
}
