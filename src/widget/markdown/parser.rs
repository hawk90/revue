//! Markdown widget parsing and rendering

#![allow(unused_imports)]

use super::types::{AdmonitionType, FootnoteDefinition, Line, StyledText};
use crate::render::Modifier;
use crate::style::Color;
use crate::utils::figlet::FigletFont;
use crate::utils::syntax::{Language, SyntaxHighlighter, SyntaxTheme};
#[cfg(feature = "markdown")]
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Markdown parser context - tracks state during parsing
#[allow(dead_code)]
pub struct ParserContext<'a> {
    pub syntax_theme: SyntaxTheme,
    pub syntax_highlight: bool,
    pub code_line_numbers: bool,
    pub code_border: bool,
    pub highlighter: SyntaxHighlighter,
    pub link_fg: Color,
    pub code_fg: Color,
    pub heading_fg: Color,
    pub quote_fg: Color,
    pub figlet_font: Option<FigletFont>,
    pub figlet_max_level: u8,
    pub in_code_block: bool,
    pub code_block_lang: Language,
    pub code_block_lines: Vec<String>,
    pub heading_level: u8,
    pub heading_text: String,
    pub in_heading: bool,
    pub list_depth: usize,
    pub ordered_list_num: Option<u64>,
    pub item_needs_bullet: bool,
    pub in_table: bool,
    pub in_table_head: bool,
    pub table_row: Vec<String>,
    pub table_alignments: Vec<pulldown_cmark::Alignment>,
    pub table_rows: Vec<Vec<String>>,
    pub current_cell: String,
    pub footnote_definitions: Vec<FootnoteDefinition>,
    pub in_footnote_definition: bool,
    pub current_footnote_label: String,
    pub current_footnote_content: String,
    pub footnote_counter: usize,
    pub footnote_label_map: std::collections::HashMap<String, usize>,
    pub in_blockquote: bool,
    pub blockquote_first_text: bool,
    pub current_admonition: Option<AdmonitionType>,
    pub accumulated_blockquote: String,
    pub current_modifier: Modifier,
    pub current_fg: Option<Color>,
    pub current_line: Line,
    pub lines: Vec<Line>,
    pub source: &'a str,
}

impl<'a> ParserContext<'a> {
    pub fn new(source: &'a str, config: &super::MarkdownConfig) -> Self {
        Self {
            syntax_theme: config.syntax_theme.clone(),
            syntax_highlight: config.syntax_highlight,
            code_line_numbers: config.code_line_numbers,
            code_border: config.code_border,
            highlighter: SyntaxHighlighter::with_theme(config.syntax_theme.clone()),
            link_fg: config.link_fg,
            code_fg: config.code_fg,
            heading_fg: config.heading_fg,
            quote_fg: config.quote_fg,
            figlet_font: config.figlet_font,
            figlet_max_level: config.figlet_max_level,
            in_code_block: false,
            code_block_lang: Language::Unknown,
            code_block_lines: Vec::new(),
            heading_level: 1,
            heading_text: String::new(),
            in_heading: false,
            list_depth: 0,
            ordered_list_num: None,
            item_needs_bullet: false,
            in_table: false,
            in_table_head: false,
            table_row: Vec::new(),
            table_alignments: Vec::new(),
            table_rows: Vec::new(),
            current_cell: String::new(),
            footnote_definitions: Vec::new(),
            in_footnote_definition: false,
            current_footnote_label: String::new(),
            current_footnote_content: String::new(),
            footnote_counter: 0,
            footnote_label_map: std::collections::HashMap::new(),
            in_blockquote: false,
            blockquote_first_text: true,
            current_admonition: None,
            accumulated_blockquote: String::new(),
            current_modifier: Modifier::empty(),
            current_fg: None,
            current_line: Line::new(),
            lines: Vec::new(),
            source,
        }
    }

    /// Flush the current line
    pub fn flush_line(&mut self) {
        if !self.current_line.is_empty() {
            self.lines.push(std::mem::take(&mut self.current_line));
        }
    }

    /// Start a new line
    pub fn new_line(&mut self) {
        if !self.current_line.is_empty() {
            self.lines.push(std::mem::take(&mut self.current_line));
        }
    }

    /// Add text to the current line
    pub fn add_text(&mut self, text: &str) {
        if !text.is_empty() {
            self.current_line.push(StyledText {
                text: text.to_string(),
                fg: self.current_fg,
                bg: None,
                modifier: self.current_modifier,
            });
        }
    }

    /// Get parser options for extended markdown features
    pub fn parser_options() -> Options {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_FOOTNOTES);
        options
    }
}
