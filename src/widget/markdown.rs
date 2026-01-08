//! Markdown widget for rendering markdown content

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::figlet::{figlet_with_font, FigletFont};
use crate::utils::syntax::{Language, SyntaxHighlighter, SyntaxTheme};
use crate::{impl_props_builders, impl_styled_view};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

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
    fn from_marker(text: &str) -> Option<Self> {
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
    fn icon(&self) -> &'static str {
        match self {
            AdmonitionType::Note => "ℹ️ ",
            AdmonitionType::Tip => "💡",
            AdmonitionType::Important => "❗",
            AdmonitionType::Warning => "⚠️ ",
            AdmonitionType::Caution => "🔴",
        }
    }

    /// Get color for this admonition type
    fn color(&self) -> Color {
        match self {
            AdmonitionType::Note => Color::rgb(88, 166, 255), // Blue
            AdmonitionType::Tip => Color::rgb(63, 185, 80),   // Green
            AdmonitionType::Important => Color::rgb(163, 113, 247), // Purple
            AdmonitionType::Warning => Color::rgb(210, 153, 34), // Yellow/Orange
            AdmonitionType::Caution => Color::rgb(248, 81, 73), // Red
        }
    }

    /// Get label for this admonition type
    fn label(&self) -> &'static str {
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
struct FootnoteDefinition {
    label: String,
    content: String,
}

/// Styled text segment
#[derive(Clone)]
struct StyledText {
    text: String,
    fg: Option<Color>,
    bg: Option<Color>,
    modifier: Modifier,
}

impl StyledText {
    fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            fg: None,
            bg: None,
            modifier: Modifier::empty(),
        }
    }

    fn with_fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    fn with_modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self
    }
}

/// A line of styled text
#[derive(Clone)]
struct Line {
    segments: Vec<StyledText>,
}

impl Line {
    fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    fn push(&mut self, segment: StyledText) {
        self.segments.push(segment);
    }

    fn is_empty(&self) -> bool {
        self.segments.is_empty() || self.segments.iter().all(|s| s.text.is_empty())
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

/// A markdown widget for rendering markdown content
pub struct Markdown {
    source: String,
    lines: Vec<Line>,
    link_fg: Color,
    code_fg: Color,
    _code_bg: Option<Color>,
    heading_fg: Color,
    quote_fg: Color,
    toc_fg: Color,
    /// Figlet font for H1 headings (None = no figlet)
    figlet_font: Option<FigletFont>,
    /// Which heading levels to render as figlet (default: only H1)
    figlet_max_level: u8,
    /// Extracted table of contents
    toc: Vec<TocEntry>,
    /// Show TOC at the beginning
    show_toc: bool,
    /// TOC title (default: "Table of Contents")
    toc_title: String,
    /// Enable syntax highlighting for code blocks
    syntax_highlight: bool,
    /// Syntax highlighting theme
    syntax_theme: SyntaxTheme,
    /// Show line numbers in code blocks
    code_line_numbers: bool,
    /// Code block border style
    code_border: bool,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Markdown {
    /// Create a new markdown widget
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        let toc = Self::extract_toc(&source);
        let mut md = Self {
            source,
            lines: Vec::new(),
            link_fg: Color::CYAN,
            code_fg: Color::YELLOW,
            _code_bg: None,
            heading_fg: Color::WHITE,
            quote_fg: Color::rgb(128, 128, 128),
            toc_fg: Color::CYAN,
            figlet_font: None,
            figlet_max_level: 1,
            toc,
            show_toc: false,
            toc_title: "Table of Contents".to_string(),
            syntax_highlight: true,
            syntax_theme: SyntaxTheme::monokai(),
            code_line_numbers: false,
            code_border: true,
            props: WidgetProps::new(),
        };
        md.lines = md.parse_with_options();
        md
    }

    /// Get parser options for extended markdown features
    fn parser_options() -> Options {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_FOOTNOTES);
        options
    }

    /// Extract table of contents from markdown source
    fn extract_toc(source: &str) -> Vec<TocEntry> {
        let parser = Parser::new_ext(source, Self::parser_options());
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

    /// Enable/disable showing TOC at the beginning
    pub fn show_toc(mut self, show: bool) -> Self {
        self.show_toc = show;
        self.lines = self.parse_with_options();
        self
    }

    /// Set TOC title
    pub fn toc_title(mut self, title: impl Into<String>) -> Self {
        self.toc_title = title.into();
        self.lines = self.parse_with_options();
        self
    }

    /// Set TOC color
    pub fn toc_fg(mut self, color: Color) -> Self {
        self.toc_fg = color;
        self.lines = self.parse_with_options();
        self
    }

    /// Get formatted TOC as string
    pub fn toc_string(&self) -> String {
        let mut result = String::new();
        for entry in &self.toc {
            let indent = "  ".repeat(entry.level.saturating_sub(1) as usize);
            result.push_str(&format!("{}- {}\n", indent, entry.text));
        }
        result
    }

    /// Enable figlet-style headings with default Block font
    pub fn figlet_headings(mut self, enable: bool) -> Self {
        self.figlet_font = if enable {
            Some(FigletFont::Block)
        } else {
            None
        };
        self.lines = self.parse_with_options();
        self
    }

    /// Set figlet font for headings
    pub fn figlet_font(mut self, font: FigletFont) -> Self {
        self.figlet_font = Some(font);
        self.lines = self.parse_with_options();
        self
    }

    /// Set maximum heading level to render as figlet (1-6)
    pub fn figlet_max_level(mut self, level: u8) -> Self {
        self.figlet_max_level = level.clamp(1, 6);
        self.lines = self.parse_with_options();
        self
    }

    /// Set link color
    pub fn link_fg(mut self, color: Color) -> Self {
        self.link_fg = color;
        self.lines = self.parse_with_options();
        self
    }

    /// Set code color
    pub fn code_fg(mut self, color: Color) -> Self {
        self.code_fg = color;
        self.lines = self.parse_with_options();
        self
    }

    /// Set heading color
    pub fn heading_fg(mut self, color: Color) -> Self {
        self.heading_fg = color;
        self.lines = self.parse_with_options();
        self
    }

    /// Enable/disable syntax highlighting for code blocks
    pub fn syntax_highlight(mut self, enable: bool) -> Self {
        self.syntax_highlight = enable;
        self.lines = self.parse_with_options();
        self
    }

    /// Set syntax highlighting theme
    pub fn syntax_theme(mut self, theme: SyntaxTheme) -> Self {
        self.syntax_theme = theme;
        self.lines = self.parse_with_options();
        self
    }

    /// Use Monokai theme (default)
    pub fn theme_monokai(mut self) -> Self {
        self.syntax_theme = SyntaxTheme::monokai();
        self.lines = self.parse_with_options();
        self
    }

    /// Use Nord theme
    pub fn theme_nord(mut self) -> Self {
        self.syntax_theme = SyntaxTheme::nord();
        self.lines = self.parse_with_options();
        self
    }

    /// Use Dracula theme
    pub fn theme_dracula(mut self) -> Self {
        self.syntax_theme = SyntaxTheme::dracula();
        self.lines = self.parse_with_options();
        self
    }

    /// Use One Dark theme
    pub fn theme_one_dark(mut self) -> Self {
        self.syntax_theme = SyntaxTheme::one_dark();
        self.lines = self.parse_with_options();
        self
    }

    /// Enable/disable line numbers in code blocks
    pub fn code_line_numbers(mut self, enable: bool) -> Self {
        self.code_line_numbers = enable;
        self.lines = self.parse_with_options();
        self
    }

    /// Enable/disable code block border
    pub fn code_border(mut self, enable: bool) -> Self {
        self.code_border = enable;
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

    /// Parse markdown into styled lines with current options
    fn parse_with_options(&self) -> Vec<Line> {
        let parser = Parser::new_ext(&self.source, Self::parser_options());
        let mut lines: Vec<Line> = Vec::new();
        let mut current_line = Line::new();
        let mut current_modifier = Modifier::empty();
        let mut current_fg: Option<Color> = None;
        let mut in_code_block = false;
        let mut code_block_lang = Language::Unknown;
        let mut code_block_lines: Vec<String> = Vec::new();
        let mut heading_level: u8 = 1;
        let mut heading_text = String::new();
        let mut in_heading = false;
        let mut list_depth: usize = 0;
        let mut ordered_list_num: Option<u64> = None;
        let highlighter = SyntaxHighlighter::with_theme(self.syntax_theme.clone());
        // Task list tracking
        let mut item_needs_bullet = false; // Track if item needs bullet (not task list)
                                           // Table tracking
        let mut in_table = false;
        let mut in_table_head = false;
        let mut table_row: Vec<String> = Vec::new();
        let mut _table_alignments: Vec<pulldown_cmark::Alignment> = Vec::new(); // Reserved for alignment
        let mut table_rows: Vec<Vec<String>> = Vec::new();
        let mut current_cell = String::new();

        // Footnote tracking
        let mut footnote_definitions: Vec<FootnoteDefinition> = Vec::new();
        let mut in_footnote_definition = false;
        let mut current_footnote_label = String::new();
        let mut current_footnote_content = String::new();
        let mut footnote_counter: usize = 0;
        let mut footnote_label_map: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        // Admonition tracking (GitHub-style callouts in blockquotes)
        let mut in_blockquote = false;
        let mut blockquote_first_text = true; // Is next text the first in blockquote?
        let mut current_admonition: Option<AdmonitionType> = None;

        // Render TOC at the beginning if enabled
        if self.show_toc && !self.toc.is_empty() {
            // TOC title
            let mut title_line = Line::new();
            title_line.push(
                StyledText::new(&self.toc_title)
                    .with_fg(self.heading_fg)
                    .with_modifier(Modifier::BOLD),
            );
            lines.push(title_line);
            lines.push(Line::new());

            // TOC entries
            for entry in &self.toc {
                let indent = "  ".repeat(entry.level.saturating_sub(1) as usize);
                let mut toc_line = Line::new();
                toc_line.push(StyledText::new(format!("{}- ", indent)));
                toc_line.push(
                    StyledText::new(&entry.text)
                        .with_fg(self.toc_fg)
                        .with_modifier(Modifier::UNDERLINE),
                );
                lines.push(toc_line);
            }

            // Separator
            lines.push(Line::new());
            let mut sep_line = Line::new();
            sep_line.push(
                StyledText::new("────────────────────────────────────────")
                    .with_fg(Color::rgb(80, 80, 80)),
            );
            lines.push(sep_line);
            lines.push(Line::new());
        }

        for event in parser {
            match event {
                Event::Start(tag) => {
                    match tag {
                        Tag::Heading { level, .. } => {
                            heading_level = match level {
                                HeadingLevel::H1 => 1,
                                HeadingLevel::H2 => 2,
                                HeadingLevel::H3 => 3,
                                HeadingLevel::H4 => 4,
                                HeadingLevel::H5 => 5,
                                HeadingLevel::H6 => 6,
                            };
                            in_heading = true;
                            heading_text.clear();
                            current_modifier |= Modifier::BOLD;
                            current_fg = Some(self.heading_fg);
                        }
                        Tag::Strong => {
                            current_modifier |= Modifier::BOLD;
                        }
                        Tag::Emphasis => {
                            current_modifier |= Modifier::ITALIC;
                        }
                        Tag::Strikethrough => {
                            current_modifier |= Modifier::CROSSED_OUT;
                        }
                        Tag::Link { .. } => {
                            current_fg = Some(self.link_fg);
                            current_modifier |= Modifier::UNDERLINE;
                        }
                        Tag::CodeBlock(kind) => {
                            in_code_block = true;
                            code_block_lines.clear();
                            // Extract language from fenced code block
                            code_block_lang = match kind {
                                CodeBlockKind::Fenced(lang) => {
                                    let lang_str =
                                        lang.as_ref().split_whitespace().next().unwrap_or("");
                                    Language::from_fence(lang_str)
                                }
                                CodeBlockKind::Indented => Language::Unknown,
                            };
                        }
                        Tag::List(start) => {
                            list_depth += 1;
                            ordered_list_num = start;
                        }
                        Tag::Item => {
                            let indent = "  ".repeat(list_depth.saturating_sub(1));
                            current_line.push(StyledText::new(&indent));

                            // For ordered lists, add the number now
                            if let Some(num) = ordered_list_num {
                                ordered_list_num = Some(num + 1);
                                current_line.push(StyledText::new(format!("{}. ", num)));
                                item_needs_bullet = false;
                            } else {
                                // For unordered lists, wait to see if it's a task list
                                item_needs_bullet = true;
                            }
                        }
                        Tag::Table(alignments) => {
                            in_table = true;
                            _table_alignments = alignments;
                            table_rows.clear();
                        }
                        Tag::TableHead => {
                            in_table_head = true;
                            table_row.clear();
                        }
                        Tag::TableRow => {
                            table_row.clear();
                        }
                        Tag::TableCell => {
                            current_cell.clear();
                        }
                        Tag::BlockQuote(_) => {
                            in_blockquote = true;
                            blockquote_first_text = true;
                            current_admonition = None;
                            // Don't render yet - wait to see if it's an admonition
                        }
                        Tag::FootnoteDefinition(label) => {
                            in_footnote_definition = true;
                            current_footnote_label = label.to_string();
                            current_footnote_content.clear();
                            // Assign number to this footnote if not already assigned
                            if !footnote_label_map.contains_key(&current_footnote_label) {
                                footnote_counter += 1;
                                footnote_label_map
                                    .insert(current_footnote_label.clone(), footnote_counter);
                            }
                        }
                        Tag::Paragraph => {
                            if !lines.is_empty() && !current_line.is_empty() {
                                lines.push(current_line);
                                current_line = Line::new();
                            }
                        }
                        _ => {}
                    }
                }
                Event::End(tag) => {
                    match tag {
                        TagEnd::Heading(_) => {
                            current_modifier = Modifier::empty();
                            current_fg = None;
                            in_heading = false;

                            // Check if we should render as figlet
                            let use_figlet = self.figlet_font.is_some()
                                && heading_level <= self.figlet_max_level;

                            if use_figlet {
                                let font = self.figlet_font.unwrap();
                                let figlet_art = figlet_with_font(&heading_text, font);

                                // Add each line of figlet art
                                for figlet_line in figlet_art.lines() {
                                    let mut line = Line::new();
                                    line.push(
                                        StyledText::new(figlet_line)
                                            .with_fg(self.heading_fg)
                                            .with_modifier(Modifier::BOLD),
                                    );
                                    lines.push(line);
                                }
                            } else {
                                // Regular heading with prefix
                                let prefix = "#".repeat(heading_level as usize);
                                let mut heading_line = Line::new();
                                heading_line.push(
                                    StyledText::new(format!("{} ", prefix))
                                        .with_fg(Color::rgb(128, 128, 128)),
                                );
                                for seg in current_line.segments.drain(..) {
                                    heading_line.push(seg);
                                }
                                lines.push(heading_line);
                            }

                            current_line = Line::new();
                            lines.push(Line::new()); // Empty line after heading
                        }
                        TagEnd::Strong => {
                            current_modifier &= !Modifier::BOLD;
                        }
                        TagEnd::Emphasis => {
                            current_modifier &= !Modifier::ITALIC;
                        }
                        TagEnd::Strikethrough => {
                            current_modifier &= !Modifier::CROSSED_OUT;
                        }
                        TagEnd::Link => {
                            current_fg = None;
                            current_modifier &= !Modifier::UNDERLINE;
                        }
                        TagEnd::CodeBlock => {
                            in_code_block = false;
                            current_fg = None;

                            // Render code block with syntax highlighting
                            let border_color = Color::rgb(60, 60, 60);
                            let line_num_color = Color::rgb(100, 100, 100);
                            let _bg_color = Color::rgb(30, 30, 30); // Reserved for background

                            // Calculate max line number width
                            let line_num_width = if self.code_line_numbers {
                                code_block_lines.len().to_string().len() + 1
                            } else {
                                0
                            };

                            // Top border
                            if self.code_border {
                                let mut border_line = Line::new();
                                border_line.push(StyledText::new("╭─").with_fg(border_color));
                                // Language label
                                let lang_label = match code_block_lang {
                                    Language::Rust => " rust ",
                                    Language::Python => " python ",
                                    Language::JavaScript => " javascript ",
                                    Language::TypeScript => " typescript ",
                                    Language::Go => " go ",
                                    Language::C => " c ",
                                    Language::Cpp => " c++ ",
                                    Language::Java => " java ",
                                    Language::Ruby => " ruby ",
                                    Language::Shell => " shell ",
                                    Language::Json => " json ",
                                    Language::Yaml => " yaml ",
                                    Language::Toml => " toml ",
                                    Language::Sql => " sql ",
                                    Language::Html => " html ",
                                    Language::Css => " css ",
                                    _ => "",
                                };
                                if !lang_label.is_empty() {
                                    border_line.push(
                                        StyledText::new(lang_label)
                                            .with_fg(self.syntax_theme.keyword),
                                    );
                                }
                                border_line
                                    .push(StyledText::new("─".repeat(40)).with_fg(border_color));
                                lines.push(border_line);
                            }

                            // Code lines with syntax highlighting
                            let mut in_block_comment = false;
                            for (idx, code_line) in code_block_lines.iter().enumerate() {
                                let mut line = Line::new();

                                // Left border
                                if self.code_border {
                                    line.push(StyledText::new("│ ").with_fg(border_color));
                                }

                                // Line numbers
                                if self.code_line_numbers {
                                    let num_str =
                                        format!("{:>width$} ", idx + 1, width = line_num_width);
                                    line.push(StyledText::new(num_str).with_fg(line_num_color));
                                }

                                // Apply syntax highlighting with block comment state tracking
                                if self.syntax_highlight {
                                    let (tokens, still_in_block) = highlighter
                                        .highlight_line_with_state(
                                            code_line,
                                            code_block_lang,
                                            in_block_comment,
                                        );
                                    in_block_comment = still_in_block;
                                    for token in tokens {
                                        let color = highlighter.token_color(token.token_type);
                                        line.push(StyledText::new(&token.text).with_fg(color));
                                    }
                                } else {
                                    // No highlighting, just use code_fg
                                    line.push(StyledText::new(code_line).with_fg(self.code_fg));
                                }

                                lines.push(line);
                            }

                            // Bottom border
                            if self.code_border {
                                let mut border_line = Line::new();
                                border_line.push(StyledText::new("╰─").with_fg(border_color));
                                border_line
                                    .push(StyledText::new("─".repeat(42)).with_fg(border_color));
                                lines.push(border_line);
                            }

                            code_block_lines.clear();
                            current_line = Line::new();
                        }
                        TagEnd::List(_) => {
                            list_depth = list_depth.saturating_sub(1);
                            if list_depth == 0 {
                                ordered_list_num = None;
                            }
                        }
                        TagEnd::Item => {
                            lines.push(current_line);
                            current_line = Line::new();
                        }
                        TagEnd::Table => {
                            in_table = false;
                            // Render the complete table
                            if !table_rows.is_empty() {
                                // Calculate column widths
                                let col_count =
                                    table_rows.iter().map(|r| r.len()).max().unwrap_or(0);
                                let mut col_widths: Vec<usize> = vec![0; col_count];
                                for row in &table_rows {
                                    for (i, cell) in row.iter().enumerate() {
                                        if i < col_widths.len() {
                                            col_widths[i] = col_widths[i].max(cell.chars().count());
                                        }
                                    }
                                }

                                let border_color = Color::rgb(80, 80, 80);
                                let header_fg = self.heading_fg;

                                // Top border
                                let mut top_border = Line::new();
                                top_border.push(StyledText::new("┌").with_fg(border_color));
                                for (i, &width) in col_widths.iter().enumerate() {
                                    top_border.push(
                                        StyledText::new("─".repeat(width + 2))
                                            .with_fg(border_color),
                                    );
                                    if i < col_widths.len() - 1 {
                                        top_border.push(StyledText::new("┬").with_fg(border_color));
                                    }
                                }
                                top_border.push(StyledText::new("┐").with_fg(border_color));
                                lines.push(top_border);

                                // Rows
                                for (row_idx, row) in table_rows.iter().enumerate() {
                                    let mut line = Line::new();
                                    line.push(StyledText::new("│").with_fg(border_color));
                                    for (i, width) in col_widths.iter().enumerate() {
                                        let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
                                        let padding = width.saturating_sub(cell.chars().count());
                                        let padded = format!(" {}{} ", cell, " ".repeat(padding));
                                        if row_idx == 0 {
                                            // Header row - bold
                                            line.push(
                                                StyledText::new(padded)
                                                    .with_fg(header_fg)
                                                    .with_modifier(Modifier::BOLD),
                                            );
                                        } else {
                                            line.push(StyledText::new(padded));
                                        }
                                        line.push(StyledText::new("│").with_fg(border_color));
                                    }
                                    lines.push(line);

                                    // Header separator
                                    if row_idx == 0 {
                                        let mut sep_line = Line::new();
                                        sep_line.push(StyledText::new("├").with_fg(border_color));
                                        for (i, &width) in col_widths.iter().enumerate() {
                                            sep_line.push(
                                                StyledText::new("─".repeat(width + 2))
                                                    .with_fg(border_color),
                                            );
                                            if i < col_widths.len() - 1 {
                                                sep_line.push(
                                                    StyledText::new("┼").with_fg(border_color),
                                                );
                                            }
                                        }
                                        sep_line.push(StyledText::new("┤").with_fg(border_color));
                                        lines.push(sep_line);
                                    }
                                }

                                // Bottom border
                                let mut bottom_border = Line::new();
                                bottom_border.push(StyledText::new("└").with_fg(border_color));
                                for (i, &width) in col_widths.iter().enumerate() {
                                    bottom_border.push(
                                        StyledText::new("─".repeat(width + 2))
                                            .with_fg(border_color),
                                    );
                                    if i < col_widths.len() - 1 {
                                        bottom_border
                                            .push(StyledText::new("┴").with_fg(border_color));
                                    }
                                }
                                bottom_border.push(StyledText::new("┘").with_fg(border_color));
                                lines.push(bottom_border);
                                lines.push(Line::new());
                            }
                            table_rows.clear();
                        }
                        TagEnd::TableHead => {
                            in_table_head = false;
                            if !table_row.is_empty() {
                                table_rows.push(table_row.clone());
                            }
                            table_row.clear();
                        }
                        TagEnd::TableRow => {
                            if !in_table_head && !table_row.is_empty() {
                                table_rows.push(table_row.clone());
                            }
                            table_row.clear();
                        }
                        TagEnd::TableCell => {
                            table_row.push(current_cell.trim().to_string());
                            current_cell.clear();
                        }
                        TagEnd::BlockQuote(_) => {
                            // Finish the blockquote/admonition
                            if !current_line.is_empty() {
                                lines.push(current_line);
                                current_line = Line::new();
                            }
                            // Add empty line after admonition
                            if current_admonition.is_some() {
                                lines.push(Line::new());
                            }
                            in_blockquote = false;
                            current_admonition = None;
                            current_fg = None;
                            current_modifier &= !Modifier::ITALIC;
                        }
                        TagEnd::FootnoteDefinition => {
                            // Save the footnote definition
                            if !current_footnote_label.is_empty() {
                                footnote_definitions.push(FootnoteDefinition {
                                    label: current_footnote_label.clone(),
                                    content: current_footnote_content.trim().to_string(),
                                });
                            }
                            in_footnote_definition = false;
                            current_footnote_label.clear();
                            current_footnote_content.clear();
                        }
                        TagEnd::Paragraph => {
                            if !current_line.is_empty() {
                                lines.push(current_line);
                                current_line = Line::new();
                            }
                            if !in_code_block {
                                lines.push(Line::new());
                            }
                        }
                        _ => {}
                    }
                }
                Event::Text(text) => {
                    if in_heading {
                        heading_text.push_str(text.as_ref());
                    }
                    if in_footnote_definition {
                        // Accumulate footnote content
                        current_footnote_content.push_str(text.as_ref());
                    } else if in_code_block {
                        // Accumulate code block lines for later highlighting
                        for line in text.as_ref().lines() {
                            code_block_lines.push(line.to_string());
                        }
                    } else if in_table {
                        // Accumulate text for table cell
                        current_cell.push_str(text.as_ref());
                    } else if in_blockquote && blockquote_first_text {
                        // Check for admonition marker (e.g., "[!NOTE]")
                        blockquote_first_text = false;
                        let text_str = text.as_ref();
                        if let Some(admonition) = AdmonitionType::from_marker(text_str) {
                            current_admonition = Some(admonition);
                            // Render admonition header with icon and label
                            let mut header_line = Line::new();
                            header_line.push(StyledText::new("│ ").with_fg(admonition.color()));
                            header_line.push(
                                StyledText::new(admonition.icon()).with_fg(admonition.color()),
                            );
                            header_line.push(
                                StyledText::new(format!(" {}", admonition.label()))
                                    .with_fg(admonition.color())
                                    .with_modifier(Modifier::BOLD),
                            );
                            lines.push(header_line);
                            // Set up for subsequent content
                            current_fg = Some(admonition.color());
                        } else {
                            // Regular blockquote - render with quote styling
                            current_fg = Some(self.quote_fg);
                            current_modifier |= Modifier::ITALIC;
                            current_line
                                .push(StyledText::new("│ ").with_fg(Color::rgb(100, 100, 100)));
                            let mut segment = StyledText::new(text_str);
                            segment.modifier = current_modifier;
                            segment.fg = current_fg;
                            current_line.push(segment);
                        }
                    } else if let Some(admonition) = current_admonition {
                        // Admonition content
                        let mut content_line = Line::new();
                        content_line.push(StyledText::new("│ ").with_fg(admonition.color()));
                        content_line.push(StyledText::new(text.as_ref()));
                        lines.push(content_line);
                    } else if in_blockquote {
                        // Regular blockquote continuation
                        current_line.push(StyledText::new("│ ").with_fg(Color::rgb(100, 100, 100)));
                        let mut segment = StyledText::new(text.as_ref());
                        segment.modifier = current_modifier;
                        segment.fg = current_fg;
                        current_line.push(segment);
                    } else {
                        // Add bullet for regular list items (not task list)
                        if item_needs_bullet {
                            current_line.push(StyledText::new("- "));
                            item_needs_bullet = false;
                        }
                        let mut segment = StyledText::new(text.as_ref());
                        segment.modifier = current_modifier;
                        segment.fg = current_fg;
                        current_line.push(segment);
                    }
                }
                Event::TaskListMarker(checked) => {
                    item_needs_bullet = false; // Task list has checkbox instead of bullet
                                               // Render checkbox
                    let checkbox = if checked {
                        "☑ " // Checked checkbox
                    } else {
                        "☐ " // Unchecked checkbox
                    };
                    let color = if checked {
                        Color::GREEN
                    } else {
                        Color::rgb(150, 150, 150)
                    };
                    current_line.push(StyledText::new(checkbox).with_fg(color));
                }
                Event::Code(code) => {
                    let segment = StyledText::new(format!("`{}`", code)).with_fg(self.code_fg);
                    current_line.push(segment);
                }
                Event::FootnoteReference(label) => {
                    // Render footnote reference as superscript number
                    let label_str = label.to_string();
                    let num = footnote_label_map
                        .entry(label_str.clone())
                        .or_insert_with(|| {
                            footnote_counter += 1;
                            footnote_counter
                        });
                    let ref_text = format!("[{}]", num);
                    current_line.push(
                        StyledText::new(ref_text)
                            .with_fg(self.link_fg)
                            .with_modifier(Modifier::BOLD),
                    );
                }
                Event::SoftBreak | Event::HardBreak => {
                    lines.push(current_line);
                    current_line = Line::new();
                }
                Event::Rule => {
                    if !current_line.is_empty() {
                        lines.push(current_line);
                        current_line = Line::new();
                    }
                    let mut rule_line = Line::new();
                    rule_line.push(
                        StyledText::new("────────────────────────────────────────")
                            .with_fg(Color::rgb(128, 128, 128)),
                    );
                    lines.push(rule_line);
                    lines.push(Line::new());
                }
                _ => {}
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        // Render footnotes section at the end if there are any
        if !footnote_definitions.is_empty() {
            lines.push(Line::new());

            // Footnotes separator
            let mut sep_line = Line::new();
            sep_line.push(
                StyledText::new("────────────────────────────────────────")
                    .with_fg(Color::rgb(80, 80, 80)),
            );
            lines.push(sep_line);
            lines.push(Line::new());

            // Sort footnote definitions by their assigned number
            let mut sorted_definitions: Vec<_> = footnote_definitions
                .iter()
                .filter_map(|def| footnote_label_map.get(&def.label).map(|num| (*num, def)))
                .collect();
            sorted_definitions.sort_by_key(|(num, _)| *num);

            // Render each footnote
            for (num, def) in sorted_definitions {
                let mut footnote_line = Line::new();
                footnote_line.push(
                    StyledText::new(format!("[{}] ", num))
                        .with_fg(self.link_fg)
                        .with_modifier(Modifier::BOLD),
                );
                footnote_line.push(StyledText::new(&def.content));
                lines.push(footnote_line);
            }
        }

        while lines.last().map(|l| l.is_empty()).unwrap_or(false) {
            lines.pop();
        }

        lines
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

/// Helper function to create a markdown widget
pub fn markdown(source: impl Into<String>) -> Markdown {
    Markdown::new(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_markdown_new() {
        let md = Markdown::new("# Hello");
        assert_eq!(md.source(), "# Hello");
    }

    #[test]
    fn test_markdown_heading() {
        let md = Markdown::new("# Heading 1");
        assert!(md.line_count() > 0);
    }

    #[test]
    fn test_markdown_paragraph() {
        let md = Markdown::new("This is a paragraph.\n\nAnother paragraph.");
        assert!(md.line_count() >= 2);
    }

    #[test]
    fn test_markdown_bold() {
        let md = Markdown::new("This is **bold** text.");
        assert!(md.line_count() >= 1);
    }

    #[test]
    fn test_markdown_italic() {
        let md = Markdown::new("This is *italic* text.");
        assert!(md.line_count() >= 1);
    }

    #[test]
    fn test_markdown_code() {
        let md = Markdown::new("Inline `code` here.");
        assert!(md.line_count() >= 1);
    }

    #[test]
    fn test_markdown_list() {
        let md = Markdown::new("- Item 1\n- Item 2\n- Item 3");
        assert!(md.line_count() >= 3);
    }

    #[test]
    fn test_markdown_ordered_list() {
        let md = Markdown::new("1. First\n2. Second\n3. Third");
        assert!(md.line_count() >= 3);
    }

    #[test]
    fn test_markdown_render() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let md = Markdown::new("# Test\n\nHello world.");
        md.render(&mut ctx);

        // Check that something was rendered
        // The heading prefix '#' should be visible
        let mut found_hash = false;
        for x in 0..10 {
            if buffer.get(x, 0).unwrap().symbol == '#' {
                found_hash = true;
                break;
            }
        }
        assert!(found_hash);
    }

    #[test]
    fn test_markdown_helper() {
        let md = markdown("Test content");
        assert_eq!(md.source(), "Test content");
    }

    #[test]
    fn test_markdown_quote() {
        let md = Markdown::new("> This is a quote");
        assert!(md.line_count() >= 1);
    }

    #[test]
    fn test_markdown_link() {
        let md = Markdown::new("[Link](https://example.com)");
        assert!(md.line_count() >= 1);
    }

    #[test]
    fn test_markdown_rule() {
        let md = Markdown::new("Above\n\n---\n\nBelow");
        assert!(md.line_count() >= 3);
    }

    // ════════════════════════════════════════════════════════════════════════════
    // Footnotes tests (#47)
    // ════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_footnote_reference() {
        let md = Markdown::new("Text with footnote[^1]\n\n[^1]: This is the footnote.");
        assert!(md.line_count() >= 2);
    }

    #[test]
    fn test_multiple_footnotes() {
        let md = Markdown::new(
            "First[^a] and second[^b].\n\n[^a]: First footnote.\n[^b]: Second footnote.",
        );
        // Should have content plus separator plus footnotes
        assert!(md.line_count() >= 3);
    }

    #[test]
    fn test_footnote_section_rendered() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let md = Markdown::new("Text[^note]\n\n[^note]: My footnote.");
        md.render(&mut ctx);

        // Look for the separator line (─) which indicates footnotes section
        let mut found_separator = false;
        for y in 0..24 {
            if buffer.get(0, y).unwrap().symbol == '─' {
                found_separator = true;
                break;
            }
        }
        assert!(found_separator, "Footnotes separator should be rendered");
    }

    // ════════════════════════════════════════════════════════════════════════════
    // Admonition/Callout tests (#49)
    // ════════════════════════════════════════════════════════════════════════════

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
        // Case insensitive
        assert_eq!(
            AdmonitionType::from_marker("[!note]"),
            Some(AdmonitionType::Note)
        );
        // Invalid markers
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
    fn test_admonition_note() {
        let md = Markdown::new("> [!NOTE]\n> This is a note.");
        assert!(md.line_count() >= 2);
    }

    #[test]
    fn test_admonition_warning() {
        let md = Markdown::new("> [!WARNING]\n> Be careful!");
        assert!(md.line_count() >= 2);
    }

    #[test]
    fn test_admonition_all_types() {
        for (marker, label) in [
            ("[!NOTE]", "Note"),
            ("[!TIP]", "Tip"),
            ("[!IMPORTANT]", "Important"),
            ("[!WARNING]", "Warning"),
            ("[!CAUTION]", "Caution"),
        ] {
            let source = format!("> {}\n> Content for {}.", marker, label);
            let md = Markdown::new(source);
            assert!(
                md.line_count() >= 2,
                "Admonition {} should render at least 2 lines",
                label
            );
        }
    }

    #[test]
    fn test_regular_blockquote_not_admonition() {
        let md = Markdown::new("> This is a regular quote\n> Not an admonition");
        // Should render as blockquote, not admonition
        assert!(md.line_count() >= 1);
    }

    #[test]
    fn test_admonition_render() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let md = Markdown::new("> [!NOTE]\n> Important information.");
        md.render(&mut ctx);

        // Look for the vertical bar (│) which is part of admonition styling
        let mut found_bar = false;
        for y in 0..24 {
            if buffer.get(0, y).unwrap().symbol == '│' {
                found_bar = true;
                break;
            }
        }
        assert!(found_bar, "Admonition border should be rendered");
    }

    #[test]
    fn test_admonition_multiline_content() {
        // Test multi-line admonition content (tests the content rendering branch)
        let md = Markdown::new("> [!WARNING]\n> Line 1\n> Line 2\n> Line 3");
        assert!(
            md.line_count() >= 4,
            "Multi-line admonition should have multiple lines"
        );
    }

    #[test]
    fn test_admonition_color() {
        // Test each admonition type has a color
        assert_ne!(AdmonitionType::Note.color(), Color::BLACK);
        assert_ne!(AdmonitionType::Tip.color(), Color::BLACK);
        assert_ne!(AdmonitionType::Important.color(), Color::BLACK);
        assert_ne!(AdmonitionType::Warning.color(), Color::BLACK);
        assert_ne!(AdmonitionType::Caution.color(), Color::BLACK);
    }

    #[test]
    fn test_blockquote_multiline() {
        // Test multi-line regular blockquote (tests continuation branch)
        let md = Markdown::new("> First line\n> Second line\n> Third line");
        assert!(md.line_count() >= 1, "Multi-line blockquote should render");
    }

    #[test]
    fn test_footnote_with_multiline_content() {
        // Test footnote with content accumulation
        let md = Markdown::new("Text[^1]\n\n[^1]: This is a longer footnote with multiple words.");
        assert!(md.line_count() >= 2);
    }

    #[test]
    fn test_footnote_reference_before_definition() {
        // Test that references work even when definition comes later
        let md = Markdown::new("See[^note] for details.\n\n[^note]: The footnote content here.");
        assert!(md.line_count() >= 2);
    }

    #[test]
    fn test_admonition_with_empty_content() {
        // Test admonition with just the marker
        let md = Markdown::new("> [!TIP]");
        assert!(md.line_count() >= 1);
    }

    #[test]
    fn test_mixed_content_with_admonition() {
        // Test that content before and after admonition renders correctly
        let md = Markdown::new("Before.\n\n> [!IMPORTANT]\n> Content\n\nAfter.");
        assert!(md.line_count() >= 3);
    }

    #[test]
    fn test_footnote_number_ordering() {
        // Test that footnotes are numbered in order of reference
        let md = Markdown::new("A[^z] B[^a]\n\n[^a]: Alpha\n[^z]: Zeta");
        // Both footnotes should be present
        assert!(md.line_count() >= 3);
    }
}
