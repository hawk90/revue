//! Markdown widget for rendering markdown content

#![allow(missing_docs)]

mod helpers;
pub mod parser;
pub mod types;

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::figlet::FigletFont;
use crate::utils::syntax::{Language, SyntaxTheme};
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

pub use types::{AdmonitionType, FootnoteDefinition, Line, StyledText, TocEntry};

// Re-export helpers
pub use helpers::markdown;

// Import pulldown-cmark types for parser
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
            quote_fg: Color::rgb(128, 128, 128),
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
        use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

        let parser = Parser::new_ext(&self.source, parser::ParserContext::parser_options());
        let mut ctx = parser::ParserContext::new(&self.source, &self.config);

        for event in parser {
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
                    ctx.add_text(" ");
                }
                Event::HardBreak => {
                    ctx.flush_line();
                    ctx.new_line();
                }
                Event::TaskListMarker(checked) => {
                    // Task list item - add checkbox
                    if checked {
                        ctx.add_text("[x] ");
                    } else {
                        ctx.add_text("[ ] ");
                    }
                    ctx.item_needs_bullet = false;
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
                StyledText::new("────────────────────────────────────────")
                    .with_fg(Color::rgb(80, 80, 80)),
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
                    CodeBlockKind::Fenced(_) => {
                        // Language will be determined from the next event
                    }
                    CodeBlockKind::Indented => {}
                }
            }
            Tag::List(num) => {
                ctx.list_depth += 1;
                ctx.ordered_list_num = num;
            }
            Tag::Item => {
                let indent = "  ".repeat(ctx.list_depth.saturating_sub(1));
                ctx.add_text(&indent);

                // For ordered lists, add the number now
                if let Some(n) = ctx.ordered_list_num {
                    ctx.ordered_list_num = Some(n + 1);
                    ctx.add_text(&format!("{}. ", n));
                    ctx.item_needs_bullet = false;
                } else {
                    // For unordered lists, wait to see if it's a task list
                    ctx.item_needs_bullet = true;
                }
            }
            Tag::Paragraph => {
                // Start new line if not empty
                ctx.flush_line();
            }
            Tag::Table(_) | Tag::TableHead | Tag::TableRow | Tag::TableCell => {
                ctx.in_table = true;
                if matches!(tag, Tag::TableHead) {
                    ctx.in_table_head = true;
                }
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
                // Add the heading text to the current line
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
                ctx.flush_line();
                // Add empty line after admonition
                if ctx.current_admonition.is_some() {
                    ctx.lines.push(std::mem::take(&mut ctx.current_line));
                    let empty_line = Line::new();
                    ctx.lines.push(empty_line);
                }
                ctx.current_admonition = None;
            }
            TagEnd::List(_) => {
                ctx.list_depth = ctx.list_depth.saturating_sub(1);
                ctx.flush_line();
            }
            TagEnd::Item => {
                // Add bullet if needed (wasn't a task list)
                if ctx.item_needs_bullet {
                    ctx.add_text("• ");
                }
                ctx.flush_line();
                ctx.item_needs_bullet = false;
            }
            _ => {}
        }
    }

    fn handle_text(&self, ctx: &mut parser::ParserContext, text: &str) {
        if ctx.in_code_block {
            ctx.code_block_lines.push(text.to_string());
        } else if ctx.in_footnote_definition {
            ctx.current_footnote_content.push_str(text);
        } else if ctx.in_table {
            ctx.current_cell.push_str(text);
        } else if ctx.in_heading {
            ctx.heading_text.push_str(text);
        } else if ctx.blockquote_first_text {
            ctx.blockquote_first_text = false;
            // Check if this is an admonition marker
            if let Some(admonition) = AdmonitionType::from_marker(text) {
                ctx.current_admonition = Some(admonition);
                // Start new line for admonition header
                ctx.flush_line();
                // Render admonition header with icon and label
                ctx.add_text(&format!("{} {}", admonition.icon(), admonition.label()));
                ctx.current_fg = Some(admonition.color());
                ctx.current_modifier |= Modifier::BOLD;
            } else {
                // Regular blockquote - add quote prefix first
                ctx.current_modifier |= Modifier::ITALIC;
                ctx.add_text("│ ");
                ctx.current_fg = Some(ctx.quote_fg);
                ctx.add_text(text);
            }
        } else if let Some(_admonition) = ctx.current_admonition {
            // Admonition content - add quote prefix
            ctx.add_text(&format!("│ {}", text));
        } else if ctx.in_blockquote {
            // Regular blockquote continuation
            ctx.add_text(&format!("│ {}", text));
        } else {
            ctx.add_text(text);
        }
    }

    fn handle_code(&self, ctx: &mut parser::ParserContext, text: &str) {
        if !ctx.in_code_block {
            ctx.add_text(text);
        }
    }

    fn handle_html(&self, ctx: &mut parser::ParserContext, text: &str) {
        if let Some(admonition) = AdmonitionType::from_marker(text) {
            ctx.current_admonition = Some(admonition);
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

        // Code border
        if ctx.code_border {
            let mut border_line = Line::new();
            border_line.push(StyledText::new("┌").with_fg(Color::rgb(100, 100, 100)));
            for _ in 0..30 {
                border_line.push(StyledText::new("─").with_fg(Color::rgb(100, 100, 100)));
            }
            border_line.push(StyledText::new("┐").with_fg(Color::rgb(100, 100, 100)));
            ctx.lines.push(border_line);
        }

        for (line_num, line) in ctx.code_block_lines.iter().enumerate() {
            let mut code_line = Line::new();

            if ctx.code_line_numbers {
                code_line.push(
                    StyledText::new(format!("{:3} │ ", line_num + 1))
                        .with_fg(Color::rgb(100, 100, 100)),
                );
            } else if ctx.code_border {
                code_line.push(StyledText::new("│ ").with_fg(Color::rgb(100, 100, 100)));
            }

            // Apply syntax highlighting if enabled
            if ctx.syntax_highlight && ctx.code_block_lang != Language::Unknown {
                let tokens = ctx.highlighter.highlight_line(line, ctx.code_block_lang);
                if !tokens.is_empty() {
                    // Render highlighted code - tokens contain the text directly
                    for token in &tokens {
                        let fg = ctx.highlighter.token_color(token.token_type);
                        code_line.push(StyledText::new(token.text.clone()).with_fg(fg));
                    }
                } else {
                    code_line.push(StyledText::new(line.clone()).with_fg(ctx.code_fg));
                }
            } else {
                code_line.push(StyledText::new(line.clone()).with_fg(ctx.code_fg));
            }

            ctx.lines.push(code_line);
        }

        ctx.code_block_lines.clear();

        if ctx.code_border {
            let mut border_line = Line::new();
            border_line.push(StyledText::new("└").with_fg(Color::rgb(100, 100, 100)));
            for _ in 0..30 {
                border_line.push(StyledText::new("─").with_fg(Color::rgb(100, 100, 100)));
            }
            border_line.push(StyledText::new("┘").with_fg(Color::rgb(100, 100, 100)));
            ctx.lines.push(border_line);
        }

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

            let mut x = area.x;
            for segment in &line.segments {
                for ch in segment.text.chars() {
                    if x >= area.x + area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = segment.fg;
                    cell.bg = segment.bg;
                    cell.modifier = segment.modifier;
                    ctx.buffer.set(x, area.y + y as u16, cell);
                    x += 1;
                }
            }
        }
    }
}

impl_styled_view!(Markdown);
impl_props_builders!(Markdown);

#[cfg(test)]
mod tests;
