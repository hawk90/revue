//! Markdown widget parsing and rendering

#![allow(unused_imports)]

use super::types::{AdmonitionType, FootnoteDefinition, Line, StyledText};
use crate::render::Modifier;
use crate::style::Color;
use crate::utils::figlet::FigletFont;
use crate::utils::syntax::{Language, SyntaxHighlighter, SyntaxTheme};
use crate::utils::unicode::display_width;
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
    /// The current heading is drawn as FIGlet big text at its end, so its
    /// inline content is collected rather than drawn as it arrives
    pub heading_is_figlet: bool,
    pub list_depth: usize,
    /// The next number of the innermost list, `None` for a bullet list
    pub ordered_list_num: Option<u64>,
    /// The outer lists' next numbers, innermost last: a nested list puts
    /// its parent's here and gives it back when it ends (#893)
    pub list_numbers: Vec<Option<u64>>,
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
    /// The width the lines are broken to, `None` for no limit
    width: Option<usize>,
    /// How many of the current line's first segments are its markers (a
    /// heading's `#`s), which the rest of the line wraps beside
    markers: usize,
    /// What starts each further row of the current line when it wraps:
    /// blank columns under its markers
    continuation: Vec<StyledText>,
    /// The list items and quotes the current block is in, outermost first
    containers: Vec<Container>,
}

/// A block that holds other blocks, a list item or a quote: every row drawn
/// inside it starts with its prefix, after the prefixes of the containers
/// around it (#1475)
struct Container {
    /// Starts the container's first row: the bullet, number or task box, or
    /// the quote bar
    first: Vec<StyledText>,
    /// Starts each further row: blank columns as wide as `first`, or the
    /// quote bar again
    rest: Vec<StyledText>,
    /// A row has been drawn inside it
    started: bool,
    /// A quote, whose bar takes a callout's color
    quote: bool,
}

impl Container {
    /// A list item: `marker` on its first row, blank columns under it after
    fn item(marker: &str) -> Self {
        Self {
            first: vec![StyledText::new(marker)],
            rest: vec![StyledText::new(" ".repeat(display_width(marker)))],
            started: false,
            quote: false,
        }
    }

    /// A quote: the bar on every row
    fn quote(bar: StyledText) -> Self {
        Self {
            first: vec![bar.clone()],
            rest: vec![bar],
            started: false,
            quote: true,
        }
    }
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
            heading_is_figlet: false,
            list_depth: 0,
            ordered_list_num: None,
            list_numbers: Vec::new(),
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
            blockquote_first_text: false,
            current_admonition: None,
            accumulated_blockquote: String::new(),
            current_modifier: Modifier::empty(),
            current_fg: None,
            current_line: Line::new(),
            lines: Vec::new(),
            source,
            width: None,
            markers: 0,
            continuation: Vec::new(),
            containers: Vec::new(),
        }
    }

    /// Break the lines to `width` columns as they are added (`None`: no
    /// limit)
    pub(super) fn set_width(&mut self, width: Option<usize>) {
        self.width = width;
    }

    /// The columns a block inside the open containers has: the width less
    /// their prefixes (`None`: no limit)
    pub(super) fn room(&self) -> Option<usize> {
        let prefix = self.containers.iter().map(|c| segments_width(&c.rest));
        self.width.map(|width| width.saturating_sub(prefix.sum()))
    }

    /// The prefixes the next row starts with, outermost container first; the
    /// containers count it as drawn
    fn take_prefix(&mut self) -> Vec<StyledText> {
        let mut prefix = Vec::new();
        for container in &mut self.containers {
            let part = if container.started {
                &container.rest
            } else {
                &container.first
            };
            prefix.extend(part.iter().cloned());
            container.started = true;
        }
        prefix
    }

    /// Add `line` as one row, after the open containers' prefixes; it is
    /// not wrapped
    pub(super) fn push_row(&mut self, line: Line) {
        let mut segments = self.take_prefix();
        segments.extend(line.segments);
        self.lines.push(Line { segments });
    }

    /// Add `line` as rows of at most the width, each after the open
    /// containers' prefixes: its first `markers` segments start the first
    /// row and `continuation` the others, and the rest wraps beside them at
    /// word boundaries.
    pub(super) fn push_wrapped(
        &mut self,
        mut line: Line,
        markers: usize,
        continuation: &[StyledText],
    ) {
        let Some(room) = self.room() else {
            self.push_row(line);
            return;
        };
        let markers = markers.min(line.segments.len());
        let body = line.segments.split_off(markers);
        let lead = line.segments;
        let room = room.saturating_sub(segments_width(&lead));
        let mut rows = super::wrap::wrap_words(&body, room).into_iter();
        let mut first = lead;
        first.extend(rows.next().unwrap_or_default());
        self.push_row(Line { segments: first });
        for row in rows {
            let mut segments = continuation.to_vec();
            segments.extend(row);
            self.push_row(Line { segments });
        }
    }

    /// Open a list item whose first row starts with `marker`
    pub(super) fn open_item(&mut self, marker: &str) {
        self.flush_line();
        self.containers.push(Container::item(marker));
    }

    /// Mark the innermost list item as a task: its box stands in for the
    /// bullet. A box that comes after the item's first row is plain text.
    pub(super) fn mark_task(&mut self, checked: bool) {
        let marker = if checked { "[x] " } else { "[ ] " };
        match self.containers.last_mut() {
            Some(item) if !item.quote && !item.started => *item = Container::item(marker),
            _ => self.add_text(marker),
        }
    }

    /// Close the innermost list item; an item with nothing in it still
    /// shows its bullet
    pub(super) fn close_item(&mut self) {
        self.flush_line();
        if self.containers.last().is_some_and(|item| !item.started) {
            self.push_row(Line::new());
        }
        self.containers.pop();
    }

    /// Open a quote: every row inside it starts with the bar
    pub(super) fn open_quote(&mut self) {
        self.flush_line();
        let bar = StyledText::new("│ ").with_fg(self.quote_fg);
        self.containers.push(Container::quote(bar));
    }

    /// Draw the innermost quote's bar in a callout's `color`
    pub(super) fn color_quote_bar(&mut self, color: Color) {
        if let Some(quote) = self.containers.iter_mut().rev().find(|c| c.quote) {
            *quote = Container {
                started: quote.started,
                ..Container::quote(StyledText::new("│ ").with_fg(color))
            };
        }
    }

    /// Close the innermost quote; `in_blockquote` stays set while an outer
    /// quote is open
    pub(super) fn close_quote(&mut self) {
        self.flush_line();
        if let Some(at) = self.containers.iter().rposition(|c| c.quote) {
            self.containers.truncate(at);
        }
        self.in_blockquote = self.containers.iter().any(|c| c.quote);
    }

    /// Add the current line, wrapped beside its markers, and start an empty
    /// one
    fn push_current(&mut self) {
        let line = std::mem::take(&mut self.current_line);
        let markers = std::mem::take(&mut self.markers);
        let continuation = std::mem::take(&mut self.continuation);
        self.push_wrapped(line, markers, &continuation);
    }

    /// Start the current line with a marker, a heading's `#`s: the rows
    /// after the first leave blank columns under it. The line is empty when
    /// it comes; a heading starts a row of its own.
    pub(super) fn add_marker(&mut self, text: &str) {
        self.add_text(text);
        self.continuation
            .push(StyledText::new(" ".repeat(display_width(text))));
        self.markers = self.current_line.segments.len();
    }

    /// Flush the current line
    pub fn flush_line(&mut self) {
        if !self.current_line.is_empty() {
            self.push_current();
        }
    }

    /// Start a new line
    pub fn new_line(&mut self) {
        if !self.current_line.is_empty() {
            self.push_current();
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

    /// Add blockquote text. The quote bar comes from the open quote, which
    /// starts every row drawn inside it, so this is `add_text`.
    pub fn add_quote_text(&mut self, text: &str) {
        self.add_text(text);
    }

    /// Emit the text held back at the start of a blockquote while waiting to
    /// see whether it forms a callout marker. It did not, so it is quote text.
    pub fn flush_pending_quote(&mut self) {
        if !(self.in_blockquote && self.blockquote_first_text) {
            return;
        }
        let pending = std::mem::take(&mut self.accumulated_blockquote);
        if pending.is_empty() {
            return;
        }
        self.blockquote_first_text = false;
        if self.current_admonition.is_none() {
            self.current_modifier |= Modifier::ITALIC;
            self.current_fg = Some(self.quote_fg);
        }
        self.flush_line();
        self.add_quote_text(pending.trim_start());
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

/// Columns `segments` take
fn segments_width(segments: &[StyledText]) -> usize {
    segments.iter().map(|s| display_width(&s.text)).sum()
}
