//! Span styles and styled text spans

use crate::style::Color;

/// Text style for spans
#[derive(Clone, Debug, Default)]
pub struct Style {
    /// Foreground color
    pub fg: Option<Color>,
    /// Background color
    pub bg: Option<Color>,
    /// Bold text
    pub bold: bool,
    /// Italic text
    pub italic: bool,
    /// Underlined text
    pub underline: bool,
    /// Dim text
    pub dim: bool,
    /// Strikethrough text
    pub strikethrough: bool,
    /// Reverse video (swap fg/bg)
    pub reverse: bool,
}

impl Style {
    /// Create a new empty style
    pub fn new() -> Self {
        Self::default()
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

    /// Set bold
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set italic
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Set underline
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Set dim
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Set strikethrough
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    /// Set reverse video (swap foreground/background)
    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Preset styles
    // ─────────────────────────────────────────────────────────────────────────

    /// Red foreground
    pub fn red() -> Self {
        Self::new().fg(Color::RED)
    }

    /// Green foreground
    pub fn green() -> Self {
        Self::new().fg(Color::GREEN)
    }

    /// Blue foreground
    pub fn blue() -> Self {
        Self::new().fg(Color::BLUE)
    }

    /// Yellow foreground
    pub fn yellow() -> Self {
        Self::new().fg(Color::YELLOW)
    }

    /// Cyan foreground
    pub fn cyan() -> Self {
        Self::new().fg(Color::CYAN)
    }

    /// Magenta foreground
    pub fn magenta() -> Self {
        Self::new().fg(Color::MAGENTA)
    }

    /// White foreground
    pub fn white() -> Self {
        Self::new().fg(Color::WHITE)
    }
}

/// A styled text span
#[derive(Clone, Debug)]
pub struct Span {
    /// Text content
    pub text: String,
    /// Style
    pub style: Style,
    /// Optional hyperlink URL
    pub link: Option<String>,
}

impl Span {
    /// Create a new span with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: Style::default(),
            link: None,
        }
    }

    /// Create a styled span
    pub fn styled(text: impl Into<String>, style: Style) -> Self {
        Self {
            text: text.into(),
            style,
            link: None,
        }
    }

    /// Create a hyperlink span
    pub fn link(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: Style::new().fg(Color::CYAN).underline(),
            link: Some(url.into()),
        }
    }

    /// Set style
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set hyperlink
    pub fn href(mut self, url: impl Into<String>) -> Self {
        self.link = Some(url.into());
        self
    }

    /// Get text width
    pub fn width(&self) -> usize {
        unicode_width::UnicodeWidthStr::width(self.text.as_str())
    }
}
