//! Markdown widget for rendering markdown content
//!
//! This module provides a comprehensive markdown renderer with syntax highlighting,
//! table of contents generation, and support for CommonMark syntax.
//!
//! ## Features
//!
//! - **Full CommonMark support** via pulldown-cmark
//! - **Syntax highlighting** for fenced code blocks, by the fence language
//! - **Table of contents** generation
//! - **Admonitions** (note, tip, warning, danger)
//! - **Footnotes** support
//! - **Task lists** with checkboxes
//! - **Headings** with FIGLET big text option
//! - **Links** with styling
//! - **Block quotes** with styling
//! - **Code blocks** with line numbers
//! - **Horizontal rules**
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! let markdown = "# Welcome to Revue\n\nThis is **bold** and this is *italic*.\n\n## Features\n\n- CSS styling\n- Reactive state\n- 100+ widgets\n\n> Tip: Check out the docs!";
//!
//! markdown()
//!     .content(markdown)
//!     .width(60);
//! ```
//!
//! # Configuration
//!
//! ```rust,ignore
//! use revue::widget::markdown::MarkdownConfig;
//! use revue::style::Color;
//!
//! let config = MarkdownConfig {
//!     link_fg: Color::CYAN,
//!     code_fg: Color::YELLOW,
//!     heading_fg: Color::WHITE,
//!     quote_fg: PLACEHOLDER_FG,
//!     show_toc: true,
//!     syntax_highlight: true,
//!     code_line_numbers: true,
//!     ..Default::default()
//! };
//! ```

#![allow(missing_docs)]

mod blocks;
mod events;
mod helpers;
pub mod parser;
pub mod types;

use crate::render::Cell;
use crate::style::Color;
use crate::utils::figlet::FigletFont;
use crate::utils::syntax::SyntaxTheme;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

pub use types::{AdmonitionType, FootnoteDefinition, Line, StyledText, TocEntry};

// Re-export helpers
pub use helpers::markdown;

/// Markdown configuration options
#[derive(Clone, Debug)]
pub struct MarkdownConfig {
    pub link_fg: Color,
    pub code_fg: Color,
    pub heading_fg: Color,
    pub quote_fg: Color,
    pub toc_fg: Color,
    pub figlet_font: Option<FigletFont>,
    pub figlet_max_level: u8,
    pub show_toc: bool,
    pub toc_title: String,
    pub syntax_highlight: bool,
    pub syntax_theme: SyntaxTheme,
    pub code_line_numbers: bool,
    pub code_border: bool,
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self {
            link_fg: Color::CYAN,
            code_fg: Color::YELLOW,
            heading_fg: Color::WHITE,
            quote_fg: PLACEHOLDER_FG,
            toc_fg: Color::CYAN,
            figlet_font: None,
            figlet_max_level: 1,
            show_toc: false,
            toc_title: "Table of Contents".to_string(),
            syntax_highlight: true,
            syntax_theme: SyntaxTheme::monokai(),
            code_line_numbers: false,
            code_border: true,
        }
    }
}

/// A markdown widget for rendering markdown content
#[derive(Clone)]
pub struct Markdown {
    pub source: String,
    pub lines: Vec<Line>,
    pub toc: Vec<TocEntry>,
    pub config: MarkdownConfig,
    pub props: WidgetProps,
}

impl Markdown {
    /// Create a new markdown widget
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        let toc = Self::extract_toc(&source);
        let config = MarkdownConfig::default();
        let mut md = Self {
            source,
            lines: Vec::new(),
            toc,
            config,
            props: WidgetProps::new(),
        };
        md.lines = md.parse_with_options();
        md
    }

    /// Extract table of contents from markdown source
    fn extract_toc(source: &str) -> Vec<TocEntry> {
        #[cfg(feature = "markdown")]
        use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_FOOTNOTES);

        let parser = Parser::new_ext(source, options);
        let mut toc = Vec::new();
        let mut in_heading = false;
        let mut heading_level: u8 = 1;
        let mut heading_text = String::new();

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    in_heading = true;
                    heading_level = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    heading_text.clear();
                }
                Event::End(TagEnd::Heading(_)) => {
                    if in_heading && !heading_text.is_empty() {
                        toc.push(TocEntry {
                            level: heading_level,
                            text: heading_text.clone(),
                        });
                    }
                    in_heading = false;
                }
                Event::Text(text) | Event::Code(text) if in_heading => {
                    heading_text.push_str(text.as_ref());
                }
                _ => {}
            }
        }

        toc
    }

    /// Get the table of contents
    pub fn toc(&self) -> &[TocEntry] {
        &self.toc
    }
    // Builder methods for configuration
    pub fn show_toc(mut self, show: bool) -> Self {
        self.config.show_toc = show;
        self.lines = self.parse_with_options();
        self
    }

    pub fn toc_title(mut self, title: impl Into<String>) -> Self {
        self.config.toc_title = title.into();
        self.lines = self.parse_with_options();
        self
    }

    pub fn toc_fg(mut self, color: Color) -> Self {
        self.config.toc_fg = color;
        self.lines = self.parse_with_options();
        self
    }

    pub fn figlet_headings(mut self, enable: bool) -> Self {
        self.config.figlet_font = if enable {
            Some(FigletFont::Block)
        } else {
            None
        };
        self.lines = self.parse_with_options();
        self
    }

    pub fn link_fg(mut self, color: Color) -> Self {
        self.config.link_fg = color;
        self.lines = self.parse_with_options();
        self
    }

    pub fn code_fg(mut self, color: Color) -> Self {
        self.config.code_fg = color;
        self.lines = self.parse_with_options();
        self
    }

    pub fn heading_fg(mut self, color: Color) -> Self {
        self.config.heading_fg = color;
        self.lines = self.parse_with_options();
        self
    }

    pub fn syntax_highlight(mut self, enable: bool) -> Self {
        self.config.syntax_highlight = enable;
        self.lines = self.parse_with_options();
        self
    }

    pub fn syntax_theme(mut self, theme: SyntaxTheme) -> Self {
        self.config.syntax_theme = theme;
        self.lines = self.parse_with_options();
        self
    }

    pub fn code_line_numbers(mut self, enable: bool) -> Self {
        self.config.code_line_numbers = enable;
        self.lines = self.parse_with_options();
        self
    }

    pub fn code_border(mut self, enable: bool) -> Self {
        self.config.code_border = enable;
        self.lines = self.parse_with_options();
        self
    }

    /// Get source markdown
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Get rendered line count
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

impl Default for Markdown {
    fn default() -> Self {
        Self::new("")
    }
}

impl View for Markdown {
    crate::impl_view_meta!("Markdown");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 1 || area.height < 1 {
            return;
        }

        for (y, line) in self.lines.iter().enumerate() {
            if y as u16 >= area.height {
                break;
            }

            let mut x: u16 = 0;
            for segment in &line.segments {
                for ch in segment.text.chars() {
                    let cw = crate::utils::char_width(ch) as u16;
                    if x + cw > area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = segment.fg;
                    cell.bg = segment.bg;
                    cell.modifier = segment.modifier;
                    ctx.set(x, y as u16, cell);
                    x += cw;
                }
            }
        }
    }
}

impl_styled_view!(Markdown);
impl_props_builders!(Markdown);

// KEEP HERE - Private implementation tests (AdmonitionType internals: from_marker, icon, label, color)
#[cfg(test)]
mod tests {
    //! Markdown widget private implementation tests

    use super::*;
    use crate::style::Color;

    #[test]
    fn test_admonition_type_from_marker() {
        assert_eq!(
            AdmonitionType::from_marker("[!NOTE]"),
            Some(AdmonitionType::Note)
        );
        assert_eq!(
            AdmonitionType::from_marker("[!TIP]"),
            Some(AdmonitionType::Tip)
        );
        assert_eq!(
            AdmonitionType::from_marker("[!IMPORTANT]"),
            Some(AdmonitionType::Important)
        );
        assert_eq!(
            AdmonitionType::from_marker("[!WARNING]"),
            Some(AdmonitionType::Warning)
        );
        assert_eq!(
            AdmonitionType::from_marker("[!CAUTION]"),
            Some(AdmonitionType::Caution)
        );
        assert_eq!(
            AdmonitionType::from_marker("[!note]"),
            Some(AdmonitionType::Note)
        );
        assert_eq!(AdmonitionType::from_marker("NOTE"), None);
        assert_eq!(AdmonitionType::from_marker("[NOTE]"), None);
        assert_eq!(AdmonitionType::from_marker("[!UNKNOWN]"), None);
    }

    #[test]
    fn test_admonition_icon() {
        assert_eq!(AdmonitionType::Note.icon(), "ℹ️ ");
        assert_eq!(AdmonitionType::Tip.icon(), "💡");
        assert_eq!(AdmonitionType::Important.icon(), "❗");
        assert_eq!(AdmonitionType::Warning.icon(), "⚠️ ");
        assert_eq!(AdmonitionType::Caution.icon(), "🔴");
    }

    #[test]
    fn test_admonition_label() {
        assert_eq!(AdmonitionType::Note.label(), "Note");
        assert_eq!(AdmonitionType::Tip.label(), "Tip");
        assert_eq!(AdmonitionType::Important.label(), "Important");
        assert_eq!(AdmonitionType::Warning.label(), "Warning");
        assert_eq!(AdmonitionType::Caution.label(), "Caution");
    }

    #[test]
    fn test_admonition_color() {
        assert_ne!(AdmonitionType::Note.color(), Color::BLACK);
        assert_ne!(AdmonitionType::Tip.color(), Color::BLACK);
        assert_ne!(AdmonitionType::Important.color(), Color::BLACK);
        assert_ne!(AdmonitionType::Warning.color(), Color::BLACK);
        assert_ne!(AdmonitionType::Caution.color(), Color::BLACK);
    }

    #[test]
    fn test_markdown_helper() {
        let md = markdown("Test content");
        assert_eq!(md.source(), "Test content");
    }
}
