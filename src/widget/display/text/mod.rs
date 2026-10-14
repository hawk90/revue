//! Text widget
//!
//! A simple text widget that internally uses RichText for rendering.
//! This ensures consistent text rendering across all widgets.

mod justify;
mod style;
mod view;

use crate::style::Color;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Text alignment
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum Alignment {
    /// Left-aligned text (default)
    #[default]
    Left,
    /// Center-aligned text
    Center,
    /// Right-aligned text
    Right,
    /// Justified text (both edges aligned)
    Justify,
}

/// A text display widget
#[derive(Clone, Debug)]
pub struct Text {
    content: String,
    fg: Option<Color>,
    bg: Option<Color>,
    bold: bool,
    italic: bool,
    underline: bool,
    dim: bool,
    reverse: bool,
    /// The alignment the builder named, if it named one.
    ///
    /// `None` is not the same as `Some(Alignment::Left)`. While this was a
    /// plain `Alignment` whose default is `Left`, an explicit `.align(Left)`
    /// was indistinguishable from saying nothing - so it fell through to CSS
    /// and an inherited `text-align: center` won, inverting the precedence
    /// the builder is supposed to have.
    align: Option<Alignment>,
    /// OSC 8 hyperlink target for every cell this text paints
    hyperlink: Option<String>,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Text {
    /// Create a new text widget
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            fg: None,
            bg: None,
            bold: false,
            italic: false,
            underline: false,
            dim: false,
            reverse: false,
            align: None,
            hyperlink: None,
            props: WidgetProps::new(),
        }
    }

    /// Link every cell this text paints to `url` (OSC 8)
    ///
    /// The URL rides on the cells, not in the content: the terminal writer
    /// opens and closes the hyperlink around the run when it prints them.
    pub(crate) fn hyperlink(mut self, url: impl Into<String>) -> Self {
        self.hyperlink = Some(url.into());
        self
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Preset builders
    // ─────────────────────────────────────────────────────────────────────────

    /// Create a heading (bold white text)
    pub fn heading(content: impl Into<String>) -> Self {
        Self::new(content).bold().fg(Color::WHITE)
    }

    /// Create muted/secondary text (dimmed gray)
    pub fn muted(content: impl Into<String>) -> Self {
        Self::new(content).fg(PLACEHOLDER_FG)
    }

    /// Create error text (red)
    pub fn error(content: impl Into<String>) -> Self {
        Self::new(content).fg(Color::RED)
    }

    /// Create success text (green)
    pub fn success(content: impl Into<String>) -> Self {
        Self::new(content).fg(Color::GREEN)
    }

    /// Create warning text (yellow)
    pub fn warning(content: impl Into<String>) -> Self {
        Self::new(content).fg(Color::YELLOW)
    }

    /// Create info text (cyan)
    pub fn info(content: impl Into<String>) -> Self {
        Self::new(content).fg(Color::CYAN)
    }

    /// Create a label (bold)
    pub fn label(content: impl Into<String>) -> Self {
        Self::new(content).bold()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Builder methods
    // ─────────────────────────────────────────────────────────────────────────

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

    /// Make text bold
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Make text italic
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Underline text
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Dim text (reduced intensity/bright)
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Reverse video (swap foreground/background colors)
    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    /// Set text alignment
    pub fn align(mut self, align: Alignment) -> Self {
        self.align = Some(align);
        self
    }

    /// Get the text content
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new("")
    }
}

impl_styled_view!(Text);
impl_props_builders!(Text);

// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    // KEEP HERE - These tests access private fields and must stay inline

    #[test]
    fn test_text_private_initialization() {
        // Test private field initialization that can't be tested via public API
        use super::*;

        let text = Text::new("Test");
        // Test that private fields are properly initialized
        assert_eq!(text.content, "Test");
        assert!(text.fg.is_none());
        assert!(text.bg.is_none());
        assert!(!text.bold);
        assert!(!text.italic);
        assert!(!text.underline);
        assert!(!text.dim);
        assert!(!text.reverse);
    }

    #[test]
    fn test_text_private_builder_patterns() {
        // Test builder pattern implementation on private fields
        use super::*;

        let text = Text::new("Test")
            .fg(Color::RED)
            .bg(Color::BLUE)
            .bold()
            .italic();

        assert_eq!(text.content, "Test");
        assert_eq!(text.fg, Some(Color::RED));
        assert_eq!(text.bg, Some(Color::BLUE));
        assert!(text.bold);
        assert!(text.italic);
    }

    #[test]
    fn test_text_private_alignment() {
        // Test private alignment field that can't be tested via public API
        use super::*;

        let text = Text::new("Test").align(Alignment::Center);
        assert_eq!(text.align, Some(Alignment::Center));
    }

    #[test]
    fn test_text_private_reverse() {
        // Test reverse private field implementation
        use super::*;

        let text = Text::new("Test").reverse();
        assert!(text.reverse);
    }
}
