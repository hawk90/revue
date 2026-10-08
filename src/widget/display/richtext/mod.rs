//! Rich text widget with styled spans and hyperlinks
//!
//! Provides rich text rendering with inline styling, similar to Textual's Rich library.
//!
//! # Examples
//!
//! ```ignore
//! use revue::widget::{RichText, Span, Style};
//!
//! // Builder API
//! let text = RichText::new()
//!     .push("Hello ", Style::new().bold())
//!     .push("World", Style::new().fg(Color::GREEN))
//!     .push_link("Click here", "https://example.com");
//!
//! // Markup API
//! let text = RichText::markup("[bold]Hello[/] [green]World[/]");
//! ```

mod parse;
mod render;
mod types;

pub use types::{Span, Style};

use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Rich text widget with multiple styled spans
#[derive(Clone)]
pub struct RichText {
    /// Spans
    spans: Vec<Span>,
    /// Default style for unstyled text
    default_style: Style,
    /// Widget props for CSS integration
    props: WidgetProps,
}

impl RichText {
    /// Create a new empty rich text
    pub fn new() -> Self {
        Self {
            spans: Vec::new(),
            default_style: Style::default(),
            props: WidgetProps::new(),
        }
    }

    /// Create from a plain string
    pub fn plain(text: impl Into<String>) -> Self {
        Self::new().push(text, Style::default())
    }

    /// Push a styled span
    pub fn push(mut self, text: impl Into<String>, style: Style) -> Self {
        self.spans.push(Span::styled(text, style));
        self
    }

    /// Push a plain text span
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.spans.push(Span::new(text));
        self
    }

    /// Push a hyperlink span
    pub fn push_link(mut self, text: impl Into<String>, url: impl Into<String>) -> Self {
        self.spans.push(Span::link(text, url));
        self
    }

    /// Push a span
    pub fn span(mut self, span: Span) -> Self {
        self.spans.push(span);
        self
    }

    /// Set the style for what a span leaves unset: its colors apply to spans
    /// without their own, its attributes (bold, italic, ...) to every span
    pub fn default_style(mut self, style: Style) -> Self {
        self.default_style = style;
        self
    }

    /// Append styled text (mutable version)
    pub fn append(&mut self, text: impl Into<String>, style: Style) {
        self.spans.push(Span::styled(text, style));
    }

    /// Append a hyperlink (mutable version)
    pub fn append_link(&mut self, text: impl Into<String>, url: impl Into<String>) {
        self.spans.push(Span::link(text, url));
    }

    /// Get the display width: the width of the widest line
    ///
    /// A `\n` inside any span starts a new line, so this is the widest run of
    /// text between newlines, not the sum of every span.
    pub fn width(&self) -> usize {
        let mut widest = 0;
        let mut current = 0;
        for span in &self.spans {
            let mut lines = span.text.split('\n');
            if let Some(first) = lines.next() {
                current += unicode_width::UnicodeWidthStr::width(first);
            }
            for line in lines {
                widest = widest.max(current);
                current = unicode_width::UnicodeWidthStr::width(line);
            }
        }
        widest.max(current)
    }

    /// Get span count
    pub fn len(&self) -> usize {
        self.spans.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.spans.is_empty()
    }

    /// Clear all spans
    pub fn clear(&mut self) {
        self.spans.clear();
    }
}

impl Default for RichText {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(RichText);
impl_props_builders!(RichText);

// ─────────────────────────────────────────────────────────────────────────────
// Helper functions
// ─────────────────────────────────────────────────────────────────────────────

/// Create a new rich text
pub fn rich_text() -> RichText {
    RichText::new()
}

/// Create rich text from markup
pub fn markup(text: &str) -> RichText {
    RichText::markup(text)
}

/// Create a styled span
pub fn span(text: impl Into<String>) -> Span {
    Span::new(text)
}

/// Create a style
pub fn style() -> Style {
    Style::new()
}
