//! Core types for terminal widget

use crate::render::Modifier;
use crate::style::Color;

/// Terminal cell with character and styling
#[derive(Clone, Debug)]
pub struct TermCell {
    /// Character
    pub ch: char,
    /// Foreground color
    pub fg: Color,
    /// Background color
    pub bg: Color,
    /// Text modifiers
    pub modifiers: Modifier,
}

impl Default for TermCell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::WHITE,
            bg: Color::BLACK,
            modifiers: Modifier::empty(),
        }
    }
}

impl TermCell {
    /// Create a new terminal cell
    pub fn new(ch: char) -> Self {
        Self {
            ch,
            ..Default::default()
        }
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Set modifiers
    pub fn modifiers(mut self, modifiers: Modifier) -> Self {
        self.modifiers = modifiers;
        self
    }
}

/// Terminal line containing cells
#[derive(Clone, Debug)]
pub struct TermLine {
    /// Cells in this line
    pub cells: Vec<TermCell>,
    /// Whether line is wrapped from previous
    pub wrapped: bool,
}

impl TermLine {
    /// Create empty line
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            wrapped: false,
        }
    }

    /// Create line with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cells: Vec::with_capacity(capacity),
            wrapped: false,
        }
    }
}

impl Default for TermLine {
    fn default() -> Self {
        Self::new()
    }
}

/// Cursor display style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CursorStyle {
    /// Block cursor █
    #[default]
    Block,
    /// Underline cursor _
    Underline,
    /// Vertical bar cursor |
    Bar,
}

/// Actions from terminal input
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TerminalAction {
    /// User submitted command
    Submit(String),
    /// User cancelled
    Cancel,
    /// Tab completion requested
    TabComplete(String),
}
