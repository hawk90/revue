//! Text formatting options for rich text editor

/// Text formatting options
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextFormat {
    /// Bold text
    pub bold: bool,
    /// Italic text
    pub italic: bool,
    /// Underline text
    pub underline: bool,
    /// Strikethrough text
    pub strikethrough: bool,
    /// Code/monospace text
    pub code: bool,
}

impl TextFormat {
    /// Create default format
    pub fn new() -> Self {
        Self::default()
    }

    /// Toggle bold
    pub fn toggle_bold(mut self) -> Self {
        self.bold = !self.bold;
        self
    }

    /// Toggle italic
    pub fn toggle_italic(mut self) -> Self {
        self.italic = !self.italic;
        self
    }

    /// Toggle underline
    pub fn toggle_underline(mut self) -> Self {
        self.underline = !self.underline;
        self
    }

    /// Toggle strikethrough
    pub fn toggle_strikethrough(mut self) -> Self {
        self.strikethrough = !self.strikethrough;
        self
    }

    /// Toggle code
    pub fn toggle_code(mut self) -> Self {
        self.code = !self.code;
        self
    }
}
