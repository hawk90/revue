//! Markdown widget types

#![allow(dead_code)]

use crate::render::Modifier;
use crate::style::Color;

/// Admonition/Callout type for GitHub-style callouts
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmonitionType {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

impl AdmonitionType {
    /// Parse admonition type from text like "[!NOTE]"
    pub fn from_marker(text: &str) -> Option<Self> {
        let text = text.trim();
        if !text.starts_with("[!") || !text.contains(']') {
            return None;
        }
        let end = text.find(']')?;
        let type_str = &text[2..end];
        match type_str.to_uppercase().as_str() {
            "NOTE" => Some(AdmonitionType::Note),
            "TIP" => Some(AdmonitionType::Tip),
            "IMPORTANT" => Some(AdmonitionType::Important),
            "WARNING" => Some(AdmonitionType::Warning),
            "CAUTION" => Some(AdmonitionType::Caution),
            _ => None,
        }
    }

    /// Get icon for this admonition type
    pub fn icon(&self) -> &'static str {
        match self {
            AdmonitionType::Note => "ℹ️ ",
            AdmonitionType::Tip => "💡",
            AdmonitionType::Important => "❗",
            AdmonitionType::Warning => "⚠️ ",
            AdmonitionType::Caution => "🔴",
        }
    }

    /// Get color for this admonition type
    pub fn color(&self) -> Color {
        match self {
            AdmonitionType::Note => Color::rgb(88, 166, 255), // Blue
            AdmonitionType::Tip => Color::rgb(63, 185, 80),   // Green
            AdmonitionType::Important => Color::rgb(163, 113, 247), // Purple
            AdmonitionType::Warning => Color::rgb(210, 153, 34), // Yellow/Orange
            AdmonitionType::Caution => Color::rgb(248, 81, 73), // Red
        }
    }

    /// Get label for this admonition type
    pub fn label(&self) -> &'static str {
        match self {
            AdmonitionType::Note => "Note",
            AdmonitionType::Tip => "Tip",
            AdmonitionType::Important => "Important",
            AdmonitionType::Warning => "Warning",
            AdmonitionType::Caution => "Caution",
        }
    }
}

/// Footnote definition
#[derive(Clone, Debug)]
pub struct FootnoteDefinition {
    pub label: String,
    pub content: String,
}

/// Styled text segment
#[derive(Clone)]
pub struct StyledText {
    pub text: String,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub modifier: Modifier,
}

impl StyledText {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            fg: None,
            bg: None,
            modifier: Modifier::empty(),
        }
    }

    pub fn with_fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    pub fn with_modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self
    }
}

/// A line of styled text
#[derive(Clone)]
pub struct Line {
    pub segments: Vec<StyledText>,
}

impl Line {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn push(&mut self, segment: StyledText) {
        self.segments.push(segment);
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty() || self.segments.iter().all(|s| s.text.is_empty())
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

/// Table of contents entry
#[derive(Clone, Debug)]
pub struct TocEntry {
    /// Heading level (1-6)
    pub level: u8,
    /// Heading text
    pub text: String,
}
