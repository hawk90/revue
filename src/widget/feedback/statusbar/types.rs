//! Bar position, section alignment, sections and key hints

use crate::style::Color;

/// Status bar position
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatusBarPosition {
    /// At the top of the area
    Top,
    /// At the bottom of the area
    #[default]
    Bottom,
}

/// Section alignment
#[deprecated(
    since = "3.5.0",
    note = "never read: a section is placed by the builder that adds it - `StatusBar::left`, `center` or `right`"
)]
#[allow(deprecated)] // the derives name the enum
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SectionAlign {
    /// Left-aligned section (default)
    #[default]
    Left,
    /// Center-aligned section
    Center,
    /// Right-aligned section
    Right,
}

/// A section in the status bar
#[derive(Clone)]
pub struct StatusSection {
    /// Section content
    pub content: String,
    /// Foreground color
    pub fg: Option<Color>,
    /// Background color
    pub bg: Option<Color>,
    /// Bold text
    pub bold: bool,
    /// Minimum width
    pub min_width: u16,
    /// Priority (higher = more important, kept when space is limited)
    pub priority: u8,
}

impl StatusSection {
    /// Create a new section
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            fg: None,
            bg: None,
            bold: false,
            min_width: 0,
            priority: 0,
        }
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

    /// Set minimum width
    pub fn min_width(mut self, width: u16) -> Self {
        self.min_width = width;
        self
    }

    /// Set priority
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Get display width in terminal columns (a wide character counts as two)
    pub fn width(&self) -> u16 {
        crate::utils::display_width(&self.content).max(self.min_width as usize) as u16
    }
}

/// Key hint for display in status bar
#[derive(Clone)]
pub struct KeyHint {
    /// Key combination
    pub key: String,
    /// Description
    pub description: String,
}

impl KeyHint {
    /// Create a new key hint
    pub fn new(key: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            description: description.into(),
        }
    }
}
