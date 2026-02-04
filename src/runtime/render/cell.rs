//! Terminal cell representation

use crate::style::Color;
use bitflags::bitflags;

bitflags! {
    /// Modifier flags for cell styling (1 byte instead of 5)
    ///
    /// Uses bitflags for compact storage. Multiple modifiers can be combined.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Modifier: u8 {
        /// Bold text
        const BOLD = 0b00000001;
        /// Italic text
        const ITALIC = 0b00000010;
        /// Underlined text
        const UNDERLINE = 0b00000100;
        /// Dimmed/faint text
        const DIM = 0b00001000;
        /// Strikethrough/crossed out text
        const CROSSED_OUT = 0b00010000;
        /// Reverse video (swap foreground/background)
        const REVERSE = 0b00100000;
    }
}

impl Modifier {
    /// Merge two modifiers (union)
    pub fn merge(&self, other: &Modifier) -> Modifier {
        *self | *other
    }
}

/// A single cell in the terminal buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// The character in this cell
    pub symbol: char,
    /// Foreground color
    pub fg: Option<Color>,
    /// Background color
    pub bg: Option<Color>,
    /// Text modifiers (bold, italic, etc.)
    pub modifier: Modifier,
    /// Hyperlink ID (references Buffer's hyperlink registry)
    pub hyperlink_id: Option<u16>,
    /// Escape sequence ID (references Buffer's sequence registry)
    /// When set, the sequence is written instead of the symbol
    pub sequence_id: Option<u16>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            symbol: ' ',
            fg: None,
            bg: None,
            modifier: Modifier::empty(),
            hyperlink_id: None,
            sequence_id: None,
        }
    }
}

impl Cell {
    /// Create a new cell with the given character
    pub fn new(symbol: char) -> Self {
        Self {
            symbol,
            fg: None,
            bg: None,
            modifier: Modifier::empty(),
            hyperlink_id: None,
            sequence_id: None,
        }
    }

    /// Create an empty cell (space character)
    pub fn empty() -> Self {
        Self::new(' ')
    }

    /// Check if cell is a continuation of a wide character or escape sequence
    pub fn is_continuation(&self) -> bool {
        self.symbol == '\0'
    }

    /// Create a continuation cell (for wide characters or escape sequences)
    pub fn continuation() -> Self {
        Self {
            symbol: '\0',
            fg: None,
            bg: None,
            modifier: Modifier::empty(),
            hyperlink_id: None,
            sequence_id: None,
        }
    }

    /// Set escape sequence ID
    pub fn sequence(mut self, id: u16) -> Self {
        self.sequence_id = Some(id);
        self
    }

    /// Set hyperlink ID
    pub fn hyperlink(mut self, id: u16) -> Self {
        self.hyperlink_id = Some(id);
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set bold modifier
    pub fn bold(mut self) -> Self {
        self.modifier |= Modifier::BOLD;
        self
    }

    /// Set italic modifier
    pub fn italic(mut self) -> Self {
        self.modifier |= Modifier::ITALIC;
        self
    }

    /// Set underline modifier
    pub fn underline(mut self) -> Self {
        self.modifier |= Modifier::UNDERLINE;
        self
    }

    /// Set dim modifier
    pub fn dim(mut self) -> Self {
        self.modifier |= Modifier::DIM;
        self
    }

    /// Set reverse modifier (swap foreground/background)
    pub fn reverse(mut self) -> Self {
        self.modifier |= Modifier::REVERSE;
        self
    }

    /// Reset the cell to default state
    pub fn reset(&mut self) {
        self.symbol = ' ';
        self.fg = None;
        self.bg = None;
        self.modifier = Modifier::empty();
        self.hyperlink_id = None;
        self.sequence_id = None;
    }
}

// Tests moved to tests/render_tests.rs
