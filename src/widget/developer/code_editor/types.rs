//! Code editor types and configuration

/// Maximum undo history size
pub const MAX_UNDO_HISTORY: usize = 100;

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
