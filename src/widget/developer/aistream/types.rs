//! Typing effect, cursor and stream status settings for the AI stream widget

/// Typing effect style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TypingStyle {
    /// No effect, show all text immediately
    None,
    /// Character by character
    #[default]
    Character,
    /// Word by word
    Word,
    /// Line by line
    Line,
    /// Chunk by chunk (for streaming)
    Chunk,
}

/// Cursor style for typing effect
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StreamCursor {
    /// Block cursor █
    #[default]
    Block,
    /// Underline cursor _
    Underline,
    /// Bar cursor |
    Bar,
    /// No cursor
    None,
}

/// Stream status
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StreamStatus {
    /// Not started
    #[default]
    Idle,
    /// Currently streaming
    Streaming,
    /// Paused
    Paused,
    /// Completed
    Complete,
    /// Error occurred
    Error,
}
