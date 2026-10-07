//! Vim mode, motion, action and command-result types

use crate::style::Color;

/// Vim mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VimMode {
    /// Normal mode (navigation, commands)
    #[default]
    Normal,
    /// Insert mode (text input)
    Insert,
    /// Visual mode (selection)
    Visual,
    /// Visual Line mode
    VisualLine,
    /// Visual Block mode
    VisualBlock,
    /// Command mode (:commands)
    Command,
    /// Search mode (/search)
    Search,
    /// Replace mode (r, R)
    Replace,
}

impl VimMode {
    /// Get mode name for display
    pub fn name(&self) -> &'static str {
        match self {
            VimMode::Normal => "NORMAL",
            VimMode::Insert => "INSERT",
            VimMode::Visual => "VISUAL",
            VimMode::VisualLine => "V-LINE",
            VimMode::VisualBlock => "V-BLOCK",
            VimMode::Command => "COMMAND",
            VimMode::Search => "SEARCH",
            VimMode::Replace => "REPLACE",
        }
    }

    /// Get mode color
    pub fn color(&self) -> Color {
        match self {
            VimMode::Normal => Color::rgb(100, 150, 255),
            VimMode::Insert => Color::rgb(100, 255, 100),
            VimMode::Visual | VimMode::VisualLine | VimMode::VisualBlock => {
                Color::rgb(255, 150, 100)
            }
            VimMode::Command => Color::rgb(255, 255, 100),
            VimMode::Search => Color::rgb(255, 100, 255),
            VimMode::Replace => Color::rgb(255, 100, 100),
        }
    }
}

/// Vim motion
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VimMotion {
    /// Character left (h)
    Left,
    /// Character right (l)
    Right,
    /// Line up (k)
    Up,
    /// Line down (j)
    Down,
    /// Word forward (w)
    Word,
    /// Word backward (b)
    WordBack,
    /// End of word (e)
    WordEnd,
    /// Start of line (0)
    LineStart,
    /// End of line ($)
    LineEnd,
    /// First non-blank (^)
    FirstNonBlank,
    /// Go to line (G, gg)
    GoToLine(Option<usize>),
    /// Find character (f)
    FindChar(char),
    /// Find character backward (F)
    FindCharBack(char),
    /// Till character (t)
    TillChar(char),
    /// Till character backward (T)
    TillCharBack(char),
    /// Paragraph forward (})
    ParagraphForward,
    /// Paragraph backward ({)
    ParagraphBack,
    /// Match bracket (%)
    MatchBracket,
    /// Search forward (n)
    SearchNext,
    /// Search backward (N)
    SearchPrev,
}

/// Vim action
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VimAction {
    /// Move cursor
    Move(VimMotion),
    /// Delete with motion
    Delete(Option<VimMotion>),
    /// Yank (copy) with motion
    Yank(Option<VimMotion>),
    /// Change with motion
    Change(Option<VimMotion>),
    /// Paste after
    PasteAfter,
    /// Paste before
    PasteBefore,
    /// Undo
    Undo,
    /// Redo
    Redo,
    /// Enter insert mode
    Insert,
    /// Insert at start of line
    InsertStart,
    /// Append after cursor
    Append,
    /// Append at end of line
    AppendEnd,
    /// Open line below
    OpenBelow,
    /// Open line above
    OpenAbove,
    /// Replace character
    ReplaceChar(char),
    /// Enter visual mode
    EnterVisual,
    /// Enter visual line mode
    EnterVisualLine,
    /// Enter visual block mode
    EnterVisualBlock,
    /// Enter command mode
    EnterCommand,
    /// Enter search mode
    EnterSearch,
    /// Escape to normal mode
    Escape,
    /// Repeat last action (.)
    Repeat,
    /// Join lines (J)
    JoinLines,
    /// Indent
    Indent,
    /// Outdent
    Outdent,
    /// Execute command
    ExecuteCommand(String),
    /// Nothing
    None,
}

/// Result of executing a vim command
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VimCommandResult {
    /// Write file
    Write,
    /// Quit
    Quit,
    /// Write and quit
    WriteQuit,
    /// Force quit without saving
    ForceQuit,
    /// Edit file
    Edit(Option<String>),
    /// Set option
    Set(String),
    /// Go to line number
    GoToLine(usize),
    /// Unknown command
    Unknown(String),
}
