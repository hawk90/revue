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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_new() {
        let cell = Cell::new('A');
        assert_eq!(cell.symbol, 'A');
        assert!(cell.fg.is_none());
        assert!(cell.bg.is_none());
        assert!(cell.modifier.is_empty());
    }

    #[test]
    fn test_cell_builder() {
        let cell = Cell::new('X')
            .fg(Color::RED)
            .bg(Color::BLACK)
            .bold()
            .underline();

        assert_eq!(cell.symbol, 'X');
        assert_eq!(cell.fg, Some(Color::RED));
        assert_eq!(cell.bg, Some(Color::BLACK));
        assert!(cell.modifier.contains(Modifier::BOLD));
        assert!(cell.modifier.contains(Modifier::UNDERLINE));
        assert!(!cell.modifier.contains(Modifier::ITALIC));
    }

    #[test]
    fn test_cell_empty() {
        let cell = Cell::empty();
        assert_eq!(cell.symbol, ' ');
    }

    #[test]
    fn test_cell_continuation() {
        let cell = Cell::continuation();
        assert!(cell.is_continuation());
        assert_eq!(cell.symbol, '\0');
    }

    #[test]
    fn test_cell_reset() {
        let mut cell = Cell::new('A').fg(Color::RED).bold();
        cell.reset();
        assert_eq!(cell.symbol, ' ');
        assert!(cell.fg.is_none());
        assert!(cell.modifier.is_empty());
    }

    #[test]
    fn test_modifier_merge() {
        let m1 = Modifier::BOLD;
        let m2 = Modifier::ITALIC;
        let merged = m1.merge(&m2);

        assert!(merged.contains(Modifier::BOLD));
        assert!(merged.contains(Modifier::ITALIC));
        assert!(!merged.contains(Modifier::UNDERLINE));
    }

    #[test]
    fn test_cell_equality() {
        let c1 = Cell::new('A').fg(Color::RED);
        let c2 = Cell::new('A').fg(Color::RED);
        let c3 = Cell::new('A').fg(Color::BLUE);

        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }

    #[test]
    fn test_cell_is_copy() {
        let c1 = Cell::new('A');
        let c2 = c1; // Copy, not move
        assert_eq!(c1.symbol, c2.symbol); // Both still valid
    }

    #[test]
    fn test_modifier_size() {
        // Modifier should be 1 byte with bitflags
        assert_eq!(std::mem::size_of::<Modifier>(), 1);
    }

    #[test]
    fn test_cell_sequence() {
        let cell = Cell::new('X').sequence(42);
        assert_eq!(cell.sequence_id, Some(42));
    }

    #[test]
    fn test_cell_reset_clears_sequence() {
        let mut cell = Cell::new('A').sequence(1);
        assert!(cell.sequence_id.is_some());
        cell.reset();
        assert!(cell.sequence_id.is_none());
    }
}
