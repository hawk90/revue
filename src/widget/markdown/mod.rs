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

mod helpers;
pub mod parser;
pub mod types;

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::figlet::FigletFont;
use crate::utils::syntax::{Language, SyntaxTheme};
use crate::utils::unicode::{center_to_width, display_width, pad_to_width, right_align_to_width};
use crate::widget::theme::{DARK_GRAY, DISABLED_FG, PLACEHOLDER_FG};
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

pub use types::{AdmonitionType, FootnoteDefinition, Line, StyledText, TocEntry};

// Re-export helpers
pub use helpers::markdown;

// Import pulldown-cmark types for parser
#[cfg(feature = "markdown")]
use pulldown_cmark::{CodeBlockKind, Tag, TagEnd};

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
                Event::Text(text) if in_heading => {
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

    /// Parse markdown into styled lines with current options
    fn parse_with_options(&self) -> Vec<Line> {
        #[allow(unused_imports)]
        #[cfg(feature = "markdown")]
        use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

        let parser = Parser::new_ext(&self.source, parser::ParserContext::parser_options());
        let mut ctx = parser::ParserContext::new(&self.source, &self.config);

        for event in parser {
            // Text held back as a possible callout marker is plain quote
            // text once anything but more text follows it
            if !matches!(event, Event::Text(_)) {
                ctx.flush_pending_quote();
            }
            match event {
                Event::Start(tag) => self.handle_start_tag(&mut ctx, tag),
                Event::End(tag_end) => self.handle_end_tag(&mut ctx, tag_end),
                Event::Text(text) => self.handle_text(&mut ctx, &text),
                Event::Code(text) => self.handle_code(&mut ctx, &text),
                Event::Html(text) => self.handle_html(&mut ctx, &text),
                Event::FootnoteReference(text) => self.handle_footnote_reference(&mut ctx, &text),
                Event::Rule => {
                    ctx.flush_line();
                    let rule_line = Line::new();
                    ctx.lines.push(rule_line);
                }
                Event::SoftBreak => {
                    if ctx.in_blockquote || ctx.current_admonition.is_some() {
                        ctx.flush_line();
                        ctx.new_line();
                    } else {
                        ctx.add_text(" ");
                    }
                }
                Event::HardBreak => {
                    ctx.flush_line();
                    ctx.new_line();
                }
                Event::TaskListMarker(checked) => {
                    // Task list item - the checkbox stands in for the bullet
                    ctx.item_needs_bullet = false;
                    if checked {
                        ctx.add_text("[x] ");
                    } else {
                        ctx.add_text("[ ] ");
                    }
                }
                // Handle remaining events
                _ => {}
            }
        }

        // Render footnotes at the end if any
        if !ctx.footnote_definitions.is_empty() {
            ctx.new_line();
            ctx.flush_line();

            // Separator
            let mut sep_line = Line::new();
            sep_line.push(
                StyledText::new("────────────────────────────────────────").with_fg(DARK_GRAY),
            );
            ctx.lines.push(sep_line);
            ctx.new_line();

            // Sort footnotes by reference number
            let mut sorted_definitions: Vec<_> = ctx.footnote_definitions.iter().collect();
            sorted_definitions
                .sort_by_key(|d| ctx.footnote_label_map.get(&d.label).copied().unwrap_or(999));

            for (idx, def) in sorted_definitions.iter().enumerate() {
                let mut footnote_line = Line::new();
                footnote_line.push(
                    StyledText::new(format!("[{}] ", idx + 1))
                        .with_fg(ctx.link_fg)
                        .with_modifier(Modifier::BOLD),
                );
                footnote_line.push(StyledText::new(&def.content));
                ctx.lines.push(footnote_line);
            }
        }

        while ctx.lines.last().map(|l| l.is_empty()).unwrap_or(false) {
            ctx.lines.pop();
        }

        ctx.lines
    }

    fn handle_start_tag(&self, ctx: &mut parser::ParserContext, tag: Tag) {
        match tag {
            Tag::Heading { level, .. } => {
                ctx.in_heading = true;
                ctx.heading_level = match level {
                    pulldown_cmark::HeadingLevel::H1 => 1,
                    pulldown_cmark::HeadingLevel::H2 => 2,
                    pulldown_cmark::HeadingLevel::H3 => 3,
                    pulldown_cmark::HeadingLevel::H4 => 4,
                    pulldown_cmark::HeadingLevel::H5 => 5,
                    pulldown_cmark::HeadingLevel::H6 => 6,
                };
                ctx.heading_text.clear();
                ctx.current_modifier |= Modifier::BOLD;
                ctx.current_fg = Some(ctx.heading_fg);
            }
            Tag::Strong => {
                ctx.current_modifier |= Modifier::BOLD;
            }
            Tag::Emphasis => {
                ctx.current_modifier |= Modifier::ITALIC;
            }
            Tag::Strikethrough => {
                // Strikethrough not supported by terminal modifier
            }
            Tag::Link { .. } => {
                ctx.current_fg = Some(ctx.link_fg);
                ctx.current_modifier |= Modifier::UNDERLINE;
            }
            Tag::Image { .. } => {
                ctx.current_fg = Some(ctx.link_fg);
            }
            Tag::CodeBlock(kind) => {
                ctx.in_code_block = true;
                match kind {
                    // The info string's first word names the language
                    CodeBlockKind::Fenced(info) => {
                        let lang = info.split_whitespace().next().unwrap_or("");
                        ctx.code_block_lang = Language::from_fence(lang);
                    }
                    CodeBlockKind::Indented => {
                        ctx.code_block_lang = Language::Unknown;
                    }
                }
            }
            Tag::List(num) => {
                ctx.list_depth += 1;
                ctx.ordered_list_num = num;
            }
            Tag::Item => {
                // A nested item must not continue its parent's line
                ctx.flush_line();
                let indent = "  ".repeat(ctx.list_depth.saturating_sub(1));
                ctx.add_text(&indent);

                // For ordered lists, add the number now
                if let Some(n) = ctx.ordered_list_num {
                    ctx.ordered_list_num = Some(n + 1);
                    ctx.add_text(&format!("{}. ", n));
                    ctx.item_needs_bullet = false;
                } else {
                    // For unordered lists, wait to see if it's a task list;
                    // `add_text` emits the bullet before the item's first text
                    ctx.item_needs_bullet = true;
                }
            }
            Tag::Paragraph => {
                // Start new line if not empty
                ctx.flush_line();
            }
            Tag::Table(alignments) => {
                ctx.flush_line();
                ctx.in_table = true;
                ctx.table_alignments = alignments;
                ctx.table_rows.clear();
            }
            Tag::TableHead => {
                ctx.in_table_head = true;
                ctx.table_row.clear();
            }
            Tag::TableRow => {
                ctx.table_row.clear();
            }
            Tag::TableCell => {
                ctx.current_cell.clear();
            }
            Tag::FootnoteDefinition(name) => {
                ctx.in_footnote_definition = true;
                ctx.current_footnote_label = name.to_string();
                ctx.current_footnote_content.clear();
            }
            Tag::BlockQuote(_) => {
                ctx.in_blockquote = true;
                ctx.blockquote_first_text = true;
                ctx.current_admonition = None;
            }
            _ => {}
        }
    }

    fn handle_end_tag(&self, ctx: &mut parser::ParserContext, tag_end: TagEnd) {
        match tag_end {
            TagEnd::Heading(_) => {
                // A heading line starts with its level's `#` marker, dimmed
                let marker = "#".repeat(ctx.heading_level as usize);
                let (fg, modifier) = (ctx.current_fg, ctx.current_modifier);
                ctx.current_fg = Some(PLACEHOLDER_FG);
                ctx.current_modifier = Modifier::empty();
                ctx.add_text(&format!("{marker} "));
                (ctx.current_fg, ctx.current_modifier) = (fg, modifier);
                if !ctx.heading_text.is_empty() {
                    let text = ctx.heading_text.clone();
                    ctx.add_text(&text);
                }
                ctx.in_heading = false;
                ctx.current_modifier &= !Modifier::BOLD;
                ctx.current_fg = None;
                ctx.new_line();
            }
            TagEnd::Paragraph => {
                ctx.flush_line();
                ctx.new_line();
            }
            TagEnd::Strong => {
                ctx.current_modifier &= !Modifier::BOLD;
            }
            TagEnd::Emphasis => {
                ctx.current_modifier &= !Modifier::ITALIC;
            }
            TagEnd::Strikethrough => {
                ctx.current_modifier &= !Modifier::CROSSED_OUT;
            }
            TagEnd::Link => {
                ctx.current_fg = None;
                ctx.current_modifier &= !Modifier::UNDERLINE;
            }
            TagEnd::CodeBlock => {
                self.render_code_block(ctx);
            }
            TagEnd::FootnoteDefinition => {
                ctx.footnote_definitions.push(FootnoteDefinition {
                    label: ctx.current_footnote_label.clone(),
                    content: ctx.current_footnote_content.clone(),
                });
                ctx.in_footnote_definition = false;
            }
            TagEnd::BlockQuote(_) => {
                ctx.in_blockquote = false;
                ctx.blockquote_first_text = false;
                ctx.flush_line();
                // Add empty line after admonition
                if ctx.current_admonition.is_some() {
                    ctx.lines.push(std::mem::take(&mut ctx.current_line));
                    let empty_line = Line::new();
                    ctx.lines.push(empty_line);
                }
                ctx.current_admonition = None;
                ctx.accumulated_blockquote.clear();
                // The quote / admonition styling ends with the quote
                ctx.current_fg = None;
                ctx.current_modifier &= !(Modifier::ITALIC | Modifier::BOLD);
            }
            TagEnd::List(_) => {
                ctx.list_depth = ctx.list_depth.saturating_sub(1);
                ctx.flush_line();
            }
            TagEnd::Item => {
                ctx.flush_line();
                ctx.item_needs_bullet = false;
            }
            TagEnd::TableCell => {
                let cell = std::mem::take(&mut ctx.current_cell);
                ctx.table_row.push(cell.trim().to_string());
            }
            TagEnd::TableHead | TagEnd::TableRow => {
                let row = std::mem::take(&mut ctx.table_row);
                ctx.table_rows.push(row);
                ctx.in_table_head = false;
            }
            TagEnd::Table => {
                self.render_table(ctx);
            }
            _ => {}
        }
    }

    fn handle_text(&self, ctx: &mut parser::ParserContext, text: &str) {
        if ctx.in_code_block {
            // Collected verbatim; split into lines when the block ends
            ctx.code_block_lines.push(text.to_string());
        } else if ctx.in_footnote_definition {
            ctx.current_footnote_content.push_str(text);
        } else if ctx.in_table {
            ctx.current_cell.push_str(text);
        } else if ctx.in_heading {
            ctx.heading_text.push_str(text);
        } else if ctx.in_blockquote && ctx.blockquote_first_text {
            // Hold the quote's opening text back only while it can still
            // become a callout marker like `[!NOTE]`
            ctx.accumulated_blockquote.push_str(text);
            let pending = ctx.accumulated_blockquote.clone();
            if let Some(admonition) = AdmonitionType::from_exact_marker(&pending) {
                ctx.current_admonition = Some(admonition);
                ctx.flush_line();
                // Render admonition header with icon and label
                ctx.current_fg = Some(admonition.color());
                ctx.current_modifier |= Modifier::BOLD;
                ctx.add_text(&format!("{} {}", admonition.icon(), admonition.label()));
                ctx.new_line();
                ctx.accumulated_blockquote.clear();
                ctx.blockquote_first_text = false;
            } else if !AdmonitionType::is_marker_prefix(&pending) {
                ctx.flush_pending_quote();
            }
        } else if ctx.in_blockquote {
            // Blockquote / admonition content
            ctx.add_quote_text(text);
        } else {
            ctx.add_text(text);
        }
    }

    fn handle_code(&self, ctx: &mut parser::ParserContext, text: &str) {
        if ctx.in_table {
            ctx.current_cell.push_str(text);
        } else if !ctx.in_code_block {
            ctx.add_text(text);
        }
    }

    fn handle_html(&self, ctx: &mut parser::ParserContext, text: &str) {
        if let Some(admonition) = AdmonitionType::from_marker(text) {
            ctx.current_admonition = Some(admonition);
            ctx.flush_line();
            // Render admonition header with icon and label
            let color = admonition.color();
            ctx.current_fg = Some(color);
            ctx.current_modifier |= Modifier::BOLD;
            ctx.add_text(&format!("{} {}", admonition.icon(), admonition.label()));
            ctx.new_line();
            ctx.blockquote_first_text = false;
        }
    }

    fn handle_footnote_reference(&self, ctx: &mut parser::ParserContext, text: &str) {
        // Track footnote references
        if !ctx.footnote_label_map.contains_key(text) {
            ctx.footnote_counter += 1;
            ctx.footnote_label_map
                .insert(text.to_string(), ctx.footnote_counter);
        }

        let num = ctx.footnote_label_map.get(text).copied().unwrap_or(1);
        ctx.add_text(&format!("[^{}]", num));
    }

    fn render_code_block(&self, ctx: &mut parser::ParserContext) {
        ctx.in_code_block = false;
        ctx.new_line();

        // pulldown-cmark may hand the block over in several text events;
        // join them and split on newlines so each source line is its own row.
        let code = std::mem::take(&mut ctx.code_block_lines).concat();
        let rows: Vec<(String, &str)> = code
            .lines()
            .enumerate()
            .map(|(line_num, line)| {
                let mut prefix = if ctx.code_border {
                    "│ ".to_string()
                } else {
                    String::new()
                };
                if ctx.code_line_numbers {
                    prefix.push_str(&format!("{:3} │ ", line_num + 1));
                }
                (prefix, line)
            })
            .collect();

        // The box is as wide as its widest line (prefix included)
        let inner_width = rows
            .iter()
            .map(|(prefix, line)| display_width(prefix) + display_width(line))
            .max()
            .unwrap_or(0)
            .max(2);

        let border = |left: &str, right: &str| {
            let mut line = Line::new();
            line.push(
                StyledText::new(format!("{left}{}{right}", "─".repeat(inner_width)))
                    .with_fg(DISABLED_FG),
            );
            line
        };

        if ctx.code_border {
            ctx.lines.push(border("┌", "┐"));
        }

        for (prefix, line) in &rows {
            let mut code_line = Line::new();

            if !prefix.is_empty() {
                code_line.push(StyledText::new(prefix.clone()).with_fg(DISABLED_FG));
            }

            // Apply syntax highlighting if enabled
            let tokens = if ctx.syntax_highlight && ctx.code_block_lang != Language::Unknown {
                ctx.highlighter.highlight_line(line, ctx.code_block_lang)
            } else {
                Vec::new()
            };
            if tokens.is_empty() {
                code_line.push(StyledText::new(*line).with_fg(ctx.code_fg));
            } else {
                // Render highlighted code - tokens contain the text directly
                for token in &tokens {
                    let fg = ctx.highlighter.token_color(token.token_type);
                    code_line.push(StyledText::new(token.text.clone()).with_fg(fg));
                }
            }

            if ctx.code_border {
                let pad = inner_width - display_width(prefix) - display_width(line);
                code_line
                    .push(StyledText::new(format!("{} │", " ".repeat(pad))).with_fg(DISABLED_FG));
            }

            ctx.lines.push(code_line);
        }

        if ctx.code_border {
            ctx.lines.push(border("└", "┘"));
        }

        ctx.new_line();
    }

    /// Lay out the collected table: a box with the header row, a separator,
    /// and the body rows, each column as wide as its widest cell. Rows wider
    /// than the widget are clipped at render time like any other line.
    fn render_table(&self, ctx: &mut parser::ParserContext) {
        use pulldown_cmark::Alignment;

        ctx.in_table = false;
        ctx.in_table_head = false;
        let rows = std::mem::take(&mut ctx.table_rows);
        let alignments = std::mem::take(&mut ctx.table_alignments);

        let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
        if columns == 0 {
            return;
        }
        let widths: Vec<usize> = (0..columns)
            .map(|col| {
                rows.iter()
                    .filter_map(|row| row.get(col))
                    .map(|cell| display_width(cell))
                    .max()
                    .unwrap_or(0)
            })
            .collect();

        let border = |left: &str, mid: &str, right: &str| {
            let segments: Vec<String> = widths.iter().map(|w| "─".repeat(w + 2)).collect();
            let mut line = Line::new();
            line.push(
                StyledText::new(format!("{left}{}{right}", segments.join(mid)))
                    .with_fg(DISABLED_FG),
            );
            line
        };

        ctx.lines.push(border("┌", "┬", "┐"));
        for (row_idx, row) in rows.iter().enumerate() {
            let is_header = row_idx == 0;
            let mut line = Line::new();
            line.push(StyledText::new("│").with_fg(DISABLED_FG));
            for (col, width) in widths.iter().enumerate() {
                let cell = row.get(col).map(String::as_str).unwrap_or("");
                let text = match alignments.get(col) {
                    Some(Alignment::Center) => center_to_width(cell, *width),
                    Some(Alignment::Right) => right_align_to_width(cell, *width),
                    _ => pad_to_width(cell, *width),
                };
                let text = StyledText::new(format!(" {text} "));
                line.push(if is_header {
                    text.with_fg(ctx.heading_fg).with_modifier(Modifier::BOLD)
                } else {
                    text
                });
                line.push(StyledText::new("│").with_fg(DISABLED_FG));
            }
            ctx.lines.push(line);
            if is_header && rows.len() > 1 {
                ctx.lines.push(border("├", "┼", "┤"));
            }
        }
        ctx.lines.push(border("└", "┴", "┘"));

        ctx.new_line();
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
            Some(crate::utils::figlet::FigletFont::Block)
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
// Public API tests extracted to tests/widget/markdown/markdown_tests.rs
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
