//! A single slide and its content builders

use super::SlideAlign;
use crate::style::Color;

/// A single slide
#[derive(Clone, Debug)]
pub struct Slide {
    /// Slide title
    pub title: String,
    /// Slide content (supports basic markdown)
    pub content: Vec<String>,
    /// Speaker notes (not displayed)
    pub notes: String,
    /// Background color
    pub bg: Option<Color>,
    /// Title color
    pub title_color: Color,
    /// Body text color. `None` lets the stylesheet's `color` decide, falling
    /// back to white; `Some` outranks the stylesheet.
    pub content_color: Option<Color>,
    /// Text alignment
    pub align: SlideAlign,
}

impl Slide {
    /// Create a new slide with title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: Vec::new(),
            notes: String::new(),
            bg: None,
            title_color: Color::CYAN,
            content_color: None,
            align: SlideAlign::Center,
        }
    }

    /// Add content line
    pub fn line(mut self, text: impl Into<String>) -> Self {
        self.content.push(text.into());
        self
    }

    /// Add multiple content lines
    pub fn lines(mut self, lines: &[&str]) -> Self {
        for line in lines {
            self.content.push((*line).to_string());
        }
        self
    }

    /// Add bullet point
    pub fn bullet(mut self, text: impl Into<String>) -> Self {
        self.content.push(format!("  • {}", text.into()));
        self
    }

    /// Add numbered item
    pub fn numbered(mut self, num: usize, text: impl Into<String>) -> Self {
        self.content.push(format!("  {}. {}", num, text.into()));
        self
    }

    /// Add code block
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.content.push(String::new());
        for line in code.into().lines() {
            self.content.push(format!("    {}", line));
        }
        self.content.push(String::new());
        self
    }

    /// Set speaker notes
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = notes.into();
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set title color
    pub fn title_color(mut self, color: Color) -> Self {
        self.title_color = color;
        self
    }

    /// Set content color
    pub fn content_color(mut self, color: Color) -> Self {
        self.content_color = Some(color);
        self
    }

    /// Set alignment
    pub fn align(mut self, align: SlideAlign) -> Self {
        self.align = align;
        self
    }
}
