//! Types for the CSV Viewer widget

/// CSV delimiter options
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Delimiter {
    /// Auto-detect delimiter
    #[default]
    Auto,
    /// Comma (,)
    Comma,
    /// Tab (\t)
    Tab,
    /// Semicolon (;)
    Semicolon,
    /// Pipe (|)
    Pipe,
    /// Custom delimiter
    Custom(char),
}

impl Delimiter {
    /// Get the actual character for parsing
    pub fn char(&self) -> Option<char> {
        match self {
            Delimiter::Auto => None,
            Delimiter::Comma => Some(','),
            Delimiter::Tab => Some('\t'),
            Delimiter::Semicolon => Some(';'),
            Delimiter::Pipe => Some('|'),
            Delimiter::Custom(c) => Some(*c),
        }
    }
}

/// Sort direction for columns
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SortOrder {
    /// No sorting applied
    #[default]
    None,
    /// Ascending order (A-Z, 0-9)
    Ascending,
    /// Descending order (Z-A, 9-0)
    Descending,
}
