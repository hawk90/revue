//! RichTextEditor widget for rich text editing with markdown support
//!
//! A WYSIWYG-style text editor with formatting toolbar, markdown shortcuts,
//! live preview, and export capabilities.

use super::traits::{RenderContext, View, WidgetProps};
use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Maximum undo history size
const MAX_UNDO_HISTORY: usize = 100;

/// Text formatting options
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextFormat {
    /// Bold text
    pub bold: bool,
    /// Italic text
    pub italic: bool,
    /// Underline text
    pub underline: bool,
    /// Strikethrough text
    pub strikethrough: bool,
    /// Code/monospace text
    pub code: bool,
}

impl TextFormat {
    /// Create default format
    pub fn new() -> Self {
        Self::default()
    }

    /// Toggle bold
    pub fn toggle_bold(mut self) -> Self {
        self.bold = !self.bold;
        self
    }

    /// Toggle italic
    pub fn toggle_italic(mut self) -> Self {
        self.italic = !self.italic;
        self
    }

    /// Toggle underline
    pub fn toggle_underline(mut self) -> Self {
        self.underline = !self.underline;
        self
    }

    /// Toggle strikethrough
    pub fn toggle_strikethrough(mut self) -> Self {
        self.strikethrough = !self.strikethrough;
        self
    }

    /// Toggle code
    pub fn toggle_code(mut self) -> Self {
        self.code = !self.code;
        self
    }
}

/// Block type for paragraphs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BlockType {
    /// Normal paragraph
    #[default]
    Paragraph,
    /// Heading level 1
    Heading1,
    /// Heading level 2
    Heading2,
    /// Heading level 3
    Heading3,
    /// Heading level 4
    Heading4,
    /// Heading level 5
    Heading5,
    /// Heading level 6
    Heading6,
    /// Blockquote
    Quote,
    /// Code block
    CodeBlock,
    /// Unordered list item
    BulletList,
    /// Ordered list item
    NumberedList,
    /// Horizontal rule
    HorizontalRule,
}

impl BlockType {
    /// Get markdown prefix for this block type
    pub fn markdown_prefix(&self) -> &'static str {
        match self {
            BlockType::Paragraph => "",
            BlockType::Heading1 => "# ",
            BlockType::Heading2 => "## ",
            BlockType::Heading3 => "### ",
            BlockType::Heading4 => "#### ",
            BlockType::Heading5 => "##### ",
            BlockType::Heading6 => "###### ",
            BlockType::Quote => "> ",
            BlockType::CodeBlock => "```\n",
            BlockType::BulletList => "- ",
            BlockType::NumberedList => "1. ",
            BlockType::HorizontalRule => "---",
        }
    }
}

/// A formatted text span
#[derive(Clone, Debug)]
pub struct FormattedSpan {
    /// Text content
    pub text: String,
    /// Format applied
    pub format: TextFormat,
}

impl FormattedSpan {
    /// Create new span
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            format: TextFormat::default(),
        }
    }

    /// Set format
    pub fn with_format(mut self, format: TextFormat) -> Self {
        self.format = format;
        self
    }
}

/// A line/block in the document
#[derive(Clone, Debug)]
pub struct Block {
    /// Block type
    pub block_type: BlockType,
    /// Spans of formatted text
    pub spans: Vec<FormattedSpan>,
    /// Language for code blocks
    pub language: Option<String>,
}

impl Block {
    /// Create new paragraph
    pub fn paragraph(text: impl Into<String>) -> Self {
        Self {
            block_type: BlockType::Paragraph,
            spans: vec![FormattedSpan::new(text)],
            language: None,
        }
    }

    /// Create new block with type
    pub fn new(block_type: BlockType) -> Self {
        Self {
            block_type,
            spans: vec![FormattedSpan::new("")],
            language: None,
        }
    }

    /// Get plain text content
    pub fn text(&self) -> String {
        self.spans.iter().map(|s| s.text.as_str()).collect()
    }

    /// Set text content (single span)
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.spans = vec![FormattedSpan::new(text)];
    }

    /// Get text length
    pub fn len(&self) -> usize {
        self.spans.iter().map(|s| s.text.len()).sum()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Convert to markdown
    pub fn to_markdown(&self) -> String {
        let prefix = self.block_type.markdown_prefix();
        let text = self.spans_to_markdown();

        match self.block_type {
            BlockType::CodeBlock => {
                let lang = self.language.as_deref().unwrap_or("");
                format!("```{}\n{}\n```", lang, text)
            }
            BlockType::HorizontalRule => "---".to_string(),
            _ => format!("{}{}", prefix, text),
        }
    }

    /// Convert spans to markdown
    fn spans_to_markdown(&self) -> String {
        self.spans
            .iter()
            .map(|span| {
                let mut text = span.text.clone();
                if span.format.code {
                    text = format!("`{}`", text);
                }
                if span.format.bold {
                    text = format!("**{}**", text);
                }
                if span.format.italic {
                    text = format!("*{}*", text);
                }
                if span.format.strikethrough {
                    text = format!("~~{}~~", text);
                }
                text
            })
            .collect()
    }
}

/// Link data
#[derive(Clone, Debug)]
pub struct Link {
    /// Display text
    pub text: String,
    /// URL
    pub url: String,
    /// Title (optional)
    pub title: Option<String>,
}

impl Link {
    /// Create new link
    pub fn new(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: url.into(),
            title: None,
        }
    }

    /// Set title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Convert to markdown
    pub fn to_markdown(&self) -> String {
        match &self.title {
            Some(title) => format!("[{}]({} \"{}\")", self.text, self.url, title),
            None => format!("[{}]({})", self.text, self.url),
        }
    }
}

/// Image data
#[derive(Clone, Debug)]
pub struct ImageRef {
    /// Alt text
    pub alt: String,
    /// Image URL/path
    pub src: String,
    /// Title (optional)
    pub title: Option<String>,
}

impl ImageRef {
    /// Create new image
    pub fn new(alt: impl Into<String>, src: impl Into<String>) -> Self {
        Self {
            alt: alt.into(),
            src: src.into(),
            title: None,
        }
    }

    /// Set title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Convert to markdown
    pub fn to_markdown(&self) -> String {
        match &self.title {
            Some(title) => format!("![{}]({} \"{}\")", self.alt, self.src, title),
            None => format!("![{}]({})", self.alt, self.src),
        }
    }
}

/// Toolbar action
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolbarAction {
    /// Toggle bold formatting
    Bold,
    /// Toggle italic formatting
    Italic,
    /// Toggle underline formatting
    Underline,
    /// Toggle strikethrough formatting
    Strikethrough,
    /// Toggle inline code formatting
    Code,
    /// Insert link
    Link,
    /// Insert image
    Image,
    /// Set block to heading 1
    Heading1,
    /// Set block to heading 2
    Heading2,
    /// Set block to heading 3
    Heading3,
    /// Set block to quote
    Quote,
    /// Set block to bullet list
    BulletList,
    /// Set block to numbered list
    NumberedList,
    /// Set block to code block
    CodeBlock,
    /// Insert horizontal rule
    HorizontalRule,
    /// Undo last action
    Undo,
    /// Redo last undone action
    Redo,
}

/// Edit operation for undo/redo
#[derive(Clone, Debug)]
#[allow(dead_code)]
enum EditOp {
    InsertChar {
        block: usize,
        col: usize,
        ch: char,
    },
    DeleteChar {
        block: usize,
        col: usize,
        ch: char,
    },
    InsertBlock {
        index: usize,
        block: Block,
    },
    DeleteBlock {
        index: usize,
        block: Block,
    },
    MergeBlocks {
        index: usize,
    },
    SplitBlock {
        block: usize,
        col: usize,
    },
    ChangeBlockType {
        block: usize,
        old: BlockType,
        new: BlockType,
    },
    SetFormat {
        block: usize,
        start: usize,
        end: usize,
        old: TextFormat,
        new: TextFormat,
    },
}

/// View mode for the editor
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EditorViewMode {
    /// Editor only
    #[default]
    Editor,
    /// Preview only
    Preview,
    /// Split view (editor + preview)
    Split,
}

/// Dialog type
#[derive(Clone, Debug)]
enum DialogType {
    None,
    InsertLink {
        text: String,
        url: String,
        field: usize,
    },
    InsertImage {
        alt: String,
        src: String,
        field: usize,
    },
}

/// Rich text editor widget
pub struct RichTextEditor {
    /// Document blocks
    blocks: Vec<Block>,
    /// Cursor position (block, col)
    cursor: (usize, usize),
    /// Selection anchor (if selecting)
    anchor: Option<(usize, usize)>,
    /// Scroll offset
    scroll: usize,
    /// Current format for new text
    current_format: TextFormat,
    /// Undo stack
    undo_stack: Vec<EditOp>,
    /// Redo stack
    redo_stack: Vec<EditOp>,
    /// View mode
    view_mode: EditorViewMode,
    /// Show toolbar
    show_toolbar: bool,
    /// Focused state
    focused: bool,
    /// Active dialog
    dialog: DialogType,
    /// Colors
    bg: Option<Color>,
    fg: Option<Color>,
    toolbar_bg: Color,
    toolbar_fg: Color,
    toolbar_active_bg: Color,
    cursor_bg: Color,
    selection_bg: Color,
    preview_bg: Color,
    heading_fg: Color,
    code_bg: Color,
    quote_fg: Color,
    #[allow(dead_code)]
    link_fg: Color,
    /// Widget props
    props: WidgetProps,
}

impl RichTextEditor {
    /// Create a new rich text editor
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::paragraph("")],
            cursor: (0, 0),
            anchor: None,
            scroll: 0,
            current_format: TextFormat::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            view_mode: EditorViewMode::Editor,
            show_toolbar: true,
            focused: true,
            dialog: DialogType::None,
            bg: Some(Color::rgb(30, 30, 46)),
            fg: Some(Color::rgb(205, 214, 244)),
            toolbar_bg: Color::rgb(49, 50, 68),
            toolbar_fg: Color::rgb(166, 173, 200),
            toolbar_active_bg: Color::rgb(137, 180, 250),
            cursor_bg: Color::rgb(166, 227, 161),
            selection_bg: Color::rgb(69, 71, 90),
            preview_bg: Color::rgb(24, 24, 37),
            heading_fg: Color::rgb(137, 180, 250),
            code_bg: Color::rgb(49, 50, 68),
            quote_fg: Color::rgb(166, 173, 200),
            link_fg: Color::rgb(137, 180, 250),
            props: WidgetProps::new(),
        }
    }

    /// Set content from plain text
    pub fn content(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        self.blocks = text.lines().map(Block::paragraph).collect();
        if self.blocks.is_empty() {
            self.blocks.push(Block::paragraph(""));
        }
        self.cursor = (0, 0);
        self.scroll = 0;
        self
    }

    /// Set content from markdown
    pub fn from_markdown(mut self, markdown: impl Into<String>) -> Self {
        self.parse_markdown(&markdown.into());
        self
    }

    /// Parse markdown content
    fn parse_markdown(&mut self, markdown: &str) {
        self.blocks.clear();
        let mut in_code_block = false;
        let mut code_block_lang = String::new();
        let mut code_block_content = String::new();

        for line in markdown.lines() {
            if let Some(lang_suffix) = line.strip_prefix("```") {
                if in_code_block {
                    // End code block
                    let mut block = Block::new(BlockType::CodeBlock);
                    block.set_text(&code_block_content);
                    block.language = if code_block_lang.is_empty() {
                        None
                    } else {
                        Some(code_block_lang.clone())
                    };
                    self.blocks.push(block);
                    code_block_content.clear();
                    code_block_lang.clear();
                    in_code_block = false;
                } else {
                    // Start code block
                    in_code_block = true;
                    code_block_lang = lang_suffix.to_string();
                }
                continue;
            }

            if in_code_block {
                if !code_block_content.is_empty() {
                    code_block_content.push('\n');
                }
                code_block_content.push_str(line);
                continue;
            }

            // Parse block type from line
            let block = self.parse_markdown_line(line);
            self.blocks.push(block);
        }

        if self.blocks.is_empty() {
            self.blocks.push(Block::paragraph(""));
        }
        self.cursor = (0, 0);
        self.scroll = 0;
    }

    /// Parse a single markdown line
    fn parse_markdown_line(&self, line: &str) -> Block {
        // Horizontal rule
        if line == "---" || line == "***" || line == "___" {
            return Block::new(BlockType::HorizontalRule);
        }

        // Headings
        if let Some(rest) = line.strip_prefix("###### ") {
            let mut block = Block::new(BlockType::Heading6);
            block.set_text(rest);
            return block;
        }
        if let Some(rest) = line.strip_prefix("##### ") {
            let mut block = Block::new(BlockType::Heading5);
            block.set_text(rest);
            return block;
        }
        if let Some(rest) = line.strip_prefix("#### ") {
            let mut block = Block::new(BlockType::Heading4);
            block.set_text(rest);
            return block;
        }
        if let Some(rest) = line.strip_prefix("### ") {
            let mut block = Block::new(BlockType::Heading3);
            block.set_text(rest);
            return block;
        }
        if let Some(rest) = line.strip_prefix("## ") {
            let mut block = Block::new(BlockType::Heading2);
            block.set_text(rest);
            return block;
        }
        if let Some(rest) = line.strip_prefix("# ") {
            let mut block = Block::new(BlockType::Heading1);
            block.set_text(rest);
            return block;
        }

        // Quote
        if let Some(rest) = line.strip_prefix("> ") {
            let mut block = Block::new(BlockType::Quote);
            block.set_text(rest);
            return block;
        }

        // Bullet list
        if let Some(rest) = line.strip_prefix("- ") {
            let mut block = Block::new(BlockType::BulletList);
            block.set_text(rest);
            return block;
        }
        if let Some(rest) = line.strip_prefix("* ") {
            let mut block = Block::new(BlockType::BulletList);
            block.set_text(rest);
            return block;
        }

        // Numbered list
        if line.len() > 2 {
            let first_char = line.chars().next().unwrap_or(' ');
            if first_char.is_ascii_digit() {
                if let Some(idx) = line.find(". ") {
                    let mut block = Block::new(BlockType::NumberedList);
                    block.set_text(&line[idx + 2..]);
                    return block;
                }
            }
        }

        // Regular paragraph
        Block::paragraph(line)
    }

    /// Set content (mutable)
    pub fn set_content(&mut self, text: &str) {
        self.blocks = text.lines().map(Block::paragraph).collect();
        if self.blocks.is_empty() {
            self.blocks.push(Block::paragraph(""));
        }
        self.cursor = (0, 0);
        self.scroll = 0;
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get plain text content
    pub fn get_content(&self) -> String {
        self.blocks
            .iter()
            .map(|b| b.text())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Export as markdown
    pub fn to_markdown(&self) -> String {
        self.blocks
            .iter()
            .map(|b| b.to_markdown())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Set view mode
    pub fn view_mode(mut self, mode: EditorViewMode) -> Self {
        self.view_mode = mode;
        self
    }

    /// Show/hide toolbar
    pub fn toolbar(mut self, show: bool) -> Self {
        self.show_toolbar = show;
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    // =========================================================================
    // Cursor and Navigation
    // =========================================================================

    /// Get cursor position
    pub fn cursor_position(&self) -> (usize, usize) {
        self.cursor
    }

    /// Set cursor position
    pub fn set_cursor(&mut self, block: usize, col: usize) {
        let block = block.min(self.blocks.len().saturating_sub(1));
        let col = col.min(self.blocks[block].len());
        self.cursor = (block, col);
        self.ensure_cursor_visible();
    }

    /// Get block count
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        } else if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.blocks[self.cursor.0].len();
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        let block_len = self.blocks[self.cursor.0].len();
        if self.cursor.1 < block_len {
            self.cursor.1 += 1;
        } else if self.cursor.0 + 1 < self.blocks.len() {
            self.cursor.0 += 1;
            self.cursor.1 = 0;
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move cursor up
    pub fn move_up(&mut self) {
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.cursor.1.min(self.blocks[self.cursor.0].len());
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move cursor down
    pub fn move_down(&mut self) {
        if self.cursor.0 + 1 < self.blocks.len() {
            self.cursor.0 += 1;
            self.cursor.1 = self.cursor.1.min(self.blocks[self.cursor.0].len());
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to start of line
    pub fn move_home(&mut self) {
        self.cursor.1 = 0;
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to end of line
    pub fn move_end(&mut self) {
        self.cursor.1 = self.blocks[self.cursor.0].len();
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to document start
    pub fn move_document_start(&mut self) {
        self.cursor = (0, 0);
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to document end
    pub fn move_document_end(&mut self) {
        let last_block = self.blocks.len().saturating_sub(1);
        self.cursor = (last_block, self.blocks[last_block].len());
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Ensure cursor is visible
    fn ensure_cursor_visible(&mut self) {
        if self.cursor.0 < self.scroll {
            self.scroll = self.cursor.0;
        }
    }

    // =========================================================================
    // Selection
    // =========================================================================

    /// Start selection
    pub fn start_selection(&mut self) {
        self.anchor = Some(self.cursor);
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.anchor = None;
    }

    /// Check if there's a selection
    pub fn has_selection(&self) -> bool {
        self.anchor.is_some()
    }

    /// Get selected text
    pub fn get_selection(&self) -> Option<String> {
        let anchor = self.anchor?;
        let (start, end) = if anchor < self.cursor {
            (anchor, self.cursor)
        } else {
            (self.cursor, anchor)
        };

        let mut result = String::new();
        for block_idx in start.0..=end.0 {
            let block = &self.blocks[block_idx];
            let text = block.text();

            let start_col = if block_idx == start.0 { start.1 } else { 0 };
            let end_col = if block_idx == end.0 {
                end.1
            } else {
                text.len()
            };

            if block_idx > start.0 {
                result.push('\n');
            }

            let chars: Vec<char> = text.chars().collect();
            let selected: String = chars[start_col..end_col.min(chars.len())].iter().collect();
            result.push_str(&selected);
        }

        Some(result)
    }

    /// Delete selection
    pub fn delete_selection(&mut self) {
        let anchor = match self.anchor {
            Some(a) => a,
            None => return,
        };

        let (start, end) = if anchor < self.cursor {
            (anchor, self.cursor)
        } else {
            (self.cursor, anchor)
        };

        if start.0 == end.0 {
            // Single block deletion
            let block = &mut self.blocks[start.0];
            let text = block.text();
            let chars: Vec<char> = text.chars().collect();
            let new_text: String = chars[..start.1].iter().chain(&chars[end.1..]).collect();
            block.set_text(new_text);
        } else {
            // Multi-block deletion
            let first_text = {
                let block = &self.blocks[start.0];
                let text = block.text();
                let chars: Vec<char> = text.chars().collect();
                chars[..start.1].iter().collect::<String>()
            };

            let last_text = {
                let block = &self.blocks[end.0];
                let text = block.text();
                let chars: Vec<char> = text.chars().collect();
                chars[end.1..].iter().collect::<String>()
            };

            // Merge first and last into first
            self.blocks[start.0].set_text(format!("{}{}", first_text, last_text));

            // Remove blocks in between
            for _ in start.0 + 1..=end.0 {
                if start.0 + 1 < self.blocks.len() {
                    self.blocks.remove(start.0 + 1);
                }
            }
        }

        self.cursor = start;
        self.anchor = None;
        self.ensure_cursor_visible();
    }

    // =========================================================================
    // Text Editing
    // =========================================================================

    /// Insert character at cursor
    pub fn insert_char(&mut self, ch: char) {
        if ch == '\n' {
            self.split_block();
            return;
        }

        // Record for undo
        self.undo_stack.push(EditOp::InsertChar {
            block: self.cursor.0,
            col: self.cursor.1,
            ch,
        });
        self.redo_stack.clear();
        if self.undo_stack.len() > MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }

        let block = &mut self.blocks[self.cursor.0];
        let text = block.text();
        let chars: Vec<char> = text.chars().collect();
        let new_text: String = chars[..self.cursor.1]
            .iter()
            .chain(std::iter::once(&ch))
            .chain(&chars[self.cursor.1..])
            .collect();
        block.set_text(new_text);
        self.cursor.1 += 1;
    }

    /// Insert string at cursor
    pub fn insert_str(&mut self, s: &str) {
        for ch in s.chars() {
            self.insert_char(ch);
        }
    }

    /// Delete character before cursor
    pub fn delete_char_before(&mut self) {
        if self.cursor.1 > 0 {
            let block = &mut self.blocks[self.cursor.0];
            let text = block.text();
            let chars: Vec<char> = text.chars().collect();
            let deleted = chars[self.cursor.1 - 1];

            // Record for undo
            self.undo_stack.push(EditOp::DeleteChar {
                block: self.cursor.0,
                col: self.cursor.1 - 1,
                ch: deleted,
            });
            self.redo_stack.clear();

            let new_text: String = chars[..self.cursor.1 - 1]
                .iter()
                .chain(&chars[self.cursor.1..])
                .collect();
            block.set_text(new_text);
            self.cursor.1 -= 1;
        } else if self.cursor.0 > 0 {
            // Merge with previous block
            self.merge_with_previous();
        }
    }

    /// Delete character at cursor
    pub fn delete_char_at(&mut self) {
        let block = &self.blocks[self.cursor.0];
        if self.cursor.1 < block.len() {
            let text = block.text();
            let chars: Vec<char> = text.chars().collect();
            let deleted = chars[self.cursor.1];

            // Record for undo
            self.undo_stack.push(EditOp::DeleteChar {
                block: self.cursor.0,
                col: self.cursor.1,
                ch: deleted,
            });
            self.redo_stack.clear();

            let new_text: String = chars[..self.cursor.1]
                .iter()
                .chain(&chars[self.cursor.1 + 1..])
                .collect();
            self.blocks[self.cursor.0].set_text(new_text);
        } else if self.cursor.0 + 1 < self.blocks.len() {
            // Merge with next block
            self.merge_with_next();
        }
    }

    /// Split block at cursor
    fn split_block(&mut self) {
        let block = &self.blocks[self.cursor.0];
        let text = block.text();
        let chars: Vec<char> = text.chars().collect();

        let first_text: String = chars[..self.cursor.1].iter().collect();
        let second_text: String = chars[self.cursor.1..].iter().collect();

        // Record for undo
        self.undo_stack.push(EditOp::SplitBlock {
            block: self.cursor.0,
            col: self.cursor.1,
        });
        self.redo_stack.clear();

        self.blocks[self.cursor.0].set_text(first_text);
        self.blocks
            .insert(self.cursor.0 + 1, Block::paragraph(second_text));
        self.cursor.0 += 1;
        self.cursor.1 = 0;
        self.ensure_cursor_visible();
    }

    /// Merge current block with previous
    fn merge_with_previous(&mut self) {
        if self.cursor.0 == 0 {
            return;
        }

        let prev_len = self.blocks[self.cursor.0 - 1].len();
        let current_text = self.blocks[self.cursor.0].text();

        // Record for undo
        self.undo_stack.push(EditOp::MergeBlocks {
            index: self.cursor.0 - 1,
        });
        self.redo_stack.clear();

        let prev_text = self.blocks[self.cursor.0 - 1].text();
        self.blocks[self.cursor.0 - 1].set_text(format!("{}{}", prev_text, current_text));
        self.blocks.remove(self.cursor.0);
        self.cursor.0 -= 1;
        self.cursor.1 = prev_len;
    }

    /// Merge current block with next
    fn merge_with_next(&mut self) {
        if self.cursor.0 + 1 >= self.blocks.len() {
            return;
        }

        let next_text = self.blocks[self.cursor.0 + 1].text();
        let current_text = self.blocks[self.cursor.0].text();
        self.blocks[self.cursor.0].set_text(format!("{}{}", current_text, next_text));
        self.blocks.remove(self.cursor.0 + 1);
    }

    /// Delete current block
    pub fn delete_block(&mut self) {
        if self.blocks.len() > 1 {
            self.blocks.remove(self.cursor.0);
            if self.cursor.0 >= self.blocks.len() {
                self.cursor.0 = self.blocks.len() - 1;
            }
            self.cursor.1 = 0;
        } else {
            self.blocks[0].set_text("");
            self.cursor.1 = 0;
        }
    }

    // =========================================================================
    // Formatting
    // =========================================================================

    /// Toggle bold format
    pub fn toggle_bold(&mut self) {
        self.current_format.bold = !self.current_format.bold;
    }

    /// Toggle italic format
    pub fn toggle_italic(&mut self) {
        self.current_format.italic = !self.current_format.italic;
    }

    /// Toggle underline format
    pub fn toggle_underline(&mut self) {
        self.current_format.underline = !self.current_format.underline;
    }

    /// Toggle strikethrough format
    pub fn toggle_strikethrough(&mut self) {
        self.current_format.strikethrough = !self.current_format.strikethrough;
    }

    /// Toggle code format
    pub fn toggle_code(&mut self) {
        self.current_format.code = !self.current_format.code;
    }

    /// Get current format
    pub fn current_format(&self) -> TextFormat {
        self.current_format
    }

    /// Set block type for current block
    pub fn set_block_type(&mut self, block_type: BlockType) {
        let old_type = self.blocks[self.cursor.0].block_type;

        // Record for undo
        self.undo_stack.push(EditOp::ChangeBlockType {
            block: self.cursor.0,
            old: old_type,
            new: block_type,
        });
        self.redo_stack.clear();

        self.blocks[self.cursor.0].block_type = block_type;
    }

    /// Get current block type
    pub fn current_block_type(&self) -> BlockType {
        self.blocks[self.cursor.0].block_type
    }

    // =========================================================================
    // Link and Image
    // =========================================================================

    /// Open link dialog
    pub fn open_link_dialog(&mut self) {
        let text = self.get_selection().unwrap_or_default();
        self.dialog = DialogType::InsertLink {
            text,
            url: String::new(),
            field: 0,
        };
    }

    /// Open image dialog
    pub fn open_image_dialog(&mut self) {
        self.dialog = DialogType::InsertImage {
            alt: String::new(),
            src: String::new(),
            field: 0,
        };
    }

    /// Check if dialog is open
    pub fn is_dialog_open(&self) -> bool {
        !matches!(self.dialog, DialogType::None)
    }

    /// Close dialog
    pub fn close_dialog(&mut self) {
        self.dialog = DialogType::None;
    }

    /// Insert link at cursor
    pub fn insert_link(&mut self, text: &str, url: &str) {
        let md = format!("[{}]({})", text, url);
        self.insert_str(&md);
    }

    /// Insert image at cursor
    pub fn insert_image(&mut self, alt: &str, src: &str) {
        let md = format!("![{}]({})", alt, src);
        self.insert_str(&md);
    }

    // =========================================================================
    // Undo/Redo
    // =========================================================================

    /// Undo last operation
    pub fn undo(&mut self) {
        if let Some(op) = self.undo_stack.pop() {
            match op {
                EditOp::InsertChar { block, col, ch } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let new_text: String = chars[..col].iter().chain(&chars[col + 1..]).collect();
                    self.blocks[block].set_text(new_text);
                    self.cursor = (block, col);
                    self.redo_stack.push(EditOp::InsertChar { block, col, ch });
                }
                EditOp::DeleteChar { block, col, ch } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let new_text: String = chars[..col]
                        .iter()
                        .chain(std::iter::once(&ch))
                        .chain(&chars[col..])
                        .collect();
                    self.blocks[block].set_text(new_text);
                    self.cursor = (block, col + 1);
                    self.redo_stack.push(EditOp::DeleteChar { block, col, ch });
                }
                EditOp::SplitBlock { block, col } => {
                    // Merge blocks back
                    let next_text = self.blocks[block + 1].text();
                    let current_text = self.blocks[block].text();
                    self.blocks[block].set_text(format!("{}{}", current_text, next_text));
                    self.blocks.remove(block + 1);
                    self.cursor = (block, col);
                    self.redo_stack.push(EditOp::SplitBlock { block, col });
                }
                EditOp::MergeBlocks { index } => {
                    // Split block back - this is complex, skip for now
                    self.redo_stack.push(EditOp::MergeBlocks { index });
                }
                EditOp::ChangeBlockType { block, old, new } => {
                    self.blocks[block].block_type = old;
                    self.redo_stack.push(EditOp::ChangeBlockType {
                        block,
                        old: new,
                        new: old,
                    });
                }
                _ => {}
            }
        }
    }

    /// Redo last undone operation
    pub fn redo(&mut self) {
        if let Some(op) = self.redo_stack.pop() {
            match op {
                EditOp::InsertChar { block, col, ch } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let new_text: String = chars[..col]
                        .iter()
                        .chain(std::iter::once(&ch))
                        .chain(&chars[col..])
                        .collect();
                    self.blocks[block].set_text(new_text);
                    self.cursor = (block, col + 1);
                    self.undo_stack.push(EditOp::InsertChar { block, col, ch });
                }
                EditOp::DeleteChar { block, col, ch } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let new_text: String = chars[..col].iter().chain(&chars[col + 1..]).collect();
                    self.blocks[block].set_text(new_text);
                    self.cursor = (block, col);
                    self.undo_stack.push(EditOp::DeleteChar { block, col, ch });
                }
                EditOp::SplitBlock { block, col } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let first_text: String = chars[..col].iter().collect();
                    let second_text: String = chars[col..].iter().collect();
                    self.blocks[block].set_text(first_text);
                    self.blocks.insert(block + 1, Block::paragraph(second_text));
                    self.cursor = (block + 1, 0);
                    self.undo_stack.push(EditOp::SplitBlock { block, col });
                }
                EditOp::ChangeBlockType { block, old, new } => {
                    self.blocks[block].block_type = new;
                    self.undo_stack
                        .push(EditOp::ChangeBlockType { block, old, new });
                }
                _ => {}
            }
        }
    }

    // =========================================================================
    // Toolbar
    // =========================================================================

    /// Execute toolbar action
    pub fn toolbar_action(&mut self, action: ToolbarAction) {
        match action {
            ToolbarAction::Bold => self.toggle_bold(),
            ToolbarAction::Italic => self.toggle_italic(),
            ToolbarAction::Underline => self.toggle_underline(),
            ToolbarAction::Strikethrough => self.toggle_strikethrough(),
            ToolbarAction::Code => self.toggle_code(),
            ToolbarAction::Link => self.open_link_dialog(),
            ToolbarAction::Image => self.open_image_dialog(),
            ToolbarAction::Heading1 => self.set_block_type(BlockType::Heading1),
            ToolbarAction::Heading2 => self.set_block_type(BlockType::Heading2),
            ToolbarAction::Heading3 => self.set_block_type(BlockType::Heading3),
            ToolbarAction::Quote => self.set_block_type(BlockType::Quote),
            ToolbarAction::BulletList => self.set_block_type(BlockType::BulletList),
            ToolbarAction::NumberedList => self.set_block_type(BlockType::NumberedList),
            ToolbarAction::CodeBlock => self.set_block_type(BlockType::CodeBlock),
            ToolbarAction::HorizontalRule => self.set_block_type(BlockType::HorizontalRule),
            ToolbarAction::Undo => self.undo(),
            ToolbarAction::Redo => self.redo(),
        }
    }

    // =========================================================================
    // Markdown Shortcuts
    // =========================================================================

    /// Process markdown shortcuts (called after typing space)
    pub fn process_markdown_shortcuts(&mut self) {
        let block = &self.blocks[self.cursor.0];
        let text = block.text();

        // Check for shortcuts at line start
        let prefix = text.trim_start();

        // Heading shortcuts
        if prefix.starts_with("# ") {
            self.apply_shortcut(BlockType::Heading1, 2);
        } else if prefix.starts_with("## ") {
            self.apply_shortcut(BlockType::Heading2, 3);
        } else if prefix.starts_with("### ") {
            self.apply_shortcut(BlockType::Heading3, 4);
        }
        // Quote shortcut
        else if prefix.starts_with("> ") {
            self.apply_shortcut(BlockType::Quote, 2);
        }
        // Bullet list shortcuts
        else if prefix.starts_with("- ") || prefix.starts_with("* ") {
            self.apply_shortcut(BlockType::BulletList, 2);
        }
        // Numbered list shortcut
        else if prefix.starts_with("1. ") {
            self.apply_shortcut(BlockType::NumberedList, 3);
        }
        // Horizontal rule
        else if text == "---" || text == "***" {
            self.blocks[self.cursor.0].block_type = BlockType::HorizontalRule;
            self.blocks[self.cursor.0].set_text("");
        }
    }

    /// Apply markdown shortcut
    fn apply_shortcut(&mut self, block_type: BlockType, prefix_len: usize) {
        let block = &mut self.blocks[self.cursor.0];
        let text = block.text();
        let new_text = text[prefix_len..].to_string();
        block.set_text(new_text);
        block.block_type = block_type;
        self.cursor.1 = self.cursor.1.saturating_sub(prefix_len);
    }

    // =========================================================================
    // Key Handling
    // =========================================================================

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        // Handle dialog input
        if self.is_dialog_open() {
            return self.handle_dialog_key(key);
        }

        match key {
            // Navigation
            Key::Left => self.move_left(),
            Key::Right => self.move_right(),
            Key::Up => self.move_up(),
            Key::Down => self.move_down(),
            Key::Home => self.move_home(),
            Key::End => self.move_end(),

            // Editing
            Key::Backspace => self.delete_char_before(),
            Key::Delete => self.delete_char_at(),
            Key::Enter => self.insert_char('\n'),
            Key::Char(ch) => {
                self.insert_char(*ch);
                if *ch == ' ' {
                    self.process_markdown_shortcuts();
                }
            }
            Key::Tab => self.insert_str("    "),

            _ => return false,
        }
        true
    }

    /// Handle dialog key input
    fn handle_dialog_key(&mut self, key: &Key) -> bool {
        match &mut self.dialog {
            DialogType::InsertLink { text, url, field } => match key {
                Key::Tab => {
                    *field = (*field + 1) % 2;
                }
                Key::Enter => {
                    let t = text.clone();
                    let u = url.clone();
                    self.dialog = DialogType::None;
                    self.insert_link(&t, &u);
                }
                Key::Escape => {
                    self.dialog = DialogType::None;
                }
                Key::Char(ch) => {
                    if *field == 0 {
                        text.push(*ch);
                    } else {
                        url.push(*ch);
                    }
                }
                Key::Backspace => {
                    if *field == 0 {
                        text.pop();
                    } else {
                        url.pop();
                    }
                }
                _ => return false,
            },
            DialogType::InsertImage { alt, src, field } => match key {
                Key::Tab => {
                    *field = (*field + 1) % 2;
                }
                Key::Enter => {
                    let a = alt.clone();
                    let s = src.clone();
                    self.dialog = DialogType::None;
                    self.insert_image(&a, &s);
                }
                Key::Escape => {
                    self.dialog = DialogType::None;
                }
                Key::Char(ch) => {
                    if *field == 0 {
                        alt.push(*ch);
                    } else {
                        src.push(*ch);
                    }
                }
                Key::Backspace => {
                    if *field == 0 {
                        alt.pop();
                    } else {
                        src.pop();
                    }
                }
                _ => return false,
            },
            DialogType::None => return false,
        }
        true
    }
}

impl Default for RichTextEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(RichTextEditor);
impl_props_builders!(RichTextEditor);

impl View for RichTextEditor {
    crate::impl_view_meta!("RichTextEditor");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 2 || area.height < 1 {
            return;
        }

        // Fill background
        if let Some(bg) = self.bg {
            for y in 0..area.height {
                for x in 0..area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(area.x + x, area.y + y, cell);
                }
            }
        }

        let mut y = area.y;

        // Render toolbar if enabled
        if self.show_toolbar {
            self.render_toolbar(ctx, area.x, y, area.width);
            y += 1;
        }

        let content_height = area
            .height
            .saturating_sub(if self.show_toolbar { 1 } else { 0 });

        match self.view_mode {
            EditorViewMode::Editor => {
                self.render_editor(ctx, area.x, y, area.width, content_height);
            }
            EditorViewMode::Preview => {
                self.render_preview(ctx, area.x, y, area.width, content_height);
            }
            EditorViewMode::Split => {
                let half_width = area.width / 2;
                self.render_editor(ctx, area.x, y, half_width, content_height);
                self.render_preview(
                    ctx,
                    area.x + half_width,
                    y,
                    area.width - half_width,
                    content_height,
                );
            }
        }

        // Render dialog if open
        if self.is_dialog_open() {
            self.render_dialog(ctx, area.x, area.y, area.width, area.height);
        }
    }
}

impl RichTextEditor {
    /// Render toolbar
    fn render_toolbar(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16) {
        // Fill toolbar background
        for col in 0..width {
            ctx.buffer
                .set(x + col, y, Cell::new(' ').bg(self.toolbar_bg));
        }

        let toolbar_items = [
            ("B", self.current_format.bold),
            ("I", self.current_format.italic),
            ("U", self.current_format.underline),
            ("S", self.current_format.strikethrough),
            ("`", self.current_format.code),
            ("|", false),
            ("H1", false),
            ("H2", false),
            ("H3", false),
            ("|", false),
            ("\"", false),
            ("•", false),
            ("1.", false),
            ("|", false),
            ("🔗", false),
            ("📷", false),
        ];

        let mut col = x + 1;
        for (label, active) in toolbar_items {
            if label == "|" {
                ctx.buffer.set(
                    col,
                    y,
                    Cell::new('│').fg(self.toolbar_fg).bg(self.toolbar_bg),
                );
                col += 1;
            } else {
                let (bg, fg) = if active {
                    (self.toolbar_active_bg, Color::rgb(30, 30, 46))
                } else {
                    (self.toolbar_bg, self.toolbar_fg)
                };

                for ch in label.chars() {
                    if col < x + width {
                        ctx.buffer.set(col, y, Cell::new(ch).fg(fg).bg(bg));
                        col += 1;
                    }
                }
                col += 1; // Space between items
            }
        }
    }

    /// Render editor
    fn render_editor(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16, height: u16) {
        let bg = self.bg.unwrap_or(Color::rgb(30, 30, 46));
        let fg = self.fg.unwrap_or(Color::rgb(205, 214, 244));

        // Fill editor background
        for row in 0..height {
            for col in 0..width {
                ctx.buffer.set(x + col, y + row, Cell::new(' ').bg(bg));
            }
        }

        // Render visible blocks
        for (row, block_idx) in (self.scroll..).take(height as usize).enumerate() {
            if block_idx >= self.blocks.len() {
                break;
            }

            let block = &self.blocks[block_idx];
            let row_y = y + row as u16;

            // Block type indicator
            let prefix = match block.block_type {
                BlockType::Heading1 => "# ",
                BlockType::Heading2 => "## ",
                BlockType::Heading3 => "### ",
                BlockType::Heading4 => "#### ",
                BlockType::Heading5 => "##### ",
                BlockType::Heading6 => "###### ",
                BlockType::Quote => "> ",
                BlockType::BulletList => "• ",
                BlockType::NumberedList => "1. ",
                BlockType::CodeBlock => "` ",
                BlockType::HorizontalRule => "──",
                BlockType::Paragraph => "",
            };

            let prefix_fg = match block.block_type {
                BlockType::Heading1
                | BlockType::Heading2
                | BlockType::Heading3
                | BlockType::Heading4
                | BlockType::Heading5
                | BlockType::Heading6 => self.heading_fg,
                BlockType::Quote => self.quote_fg,
                BlockType::CodeBlock => self.code_bg,
                _ => fg,
            };

            // Render prefix
            let mut col = x;
            for ch in prefix.chars() {
                if col < x + width {
                    ctx.buffer
                        .set(col, row_y, Cell::new(ch).fg(prefix_fg).bg(bg));
                    col += 1;
                }
            }

            // Render block content
            let text = block.text();
            for (char_idx, ch) in text.chars().enumerate() {
                if col >= x + width {
                    break;
                }

                let is_cursor =
                    self.focused && block_idx == self.cursor.0 && char_idx == self.cursor.1;

                let is_selected = self.anchor.is_some_and(|anchor| {
                    let (start, end) = if anchor < self.cursor {
                        (anchor, self.cursor)
                    } else {
                        (self.cursor, anchor)
                    };
                    block_idx >= start.0
                        && block_idx <= end.0
                        && (block_idx > start.0 || char_idx >= start.1)
                        && (block_idx < end.0 || char_idx < end.1)
                });

                let cell_bg = if is_cursor {
                    self.cursor_bg
                } else if is_selected {
                    self.selection_bg
                } else {
                    bg
                };

                ctx.buffer.set(col, row_y, Cell::new(ch).fg(fg).bg(cell_bg));
                col += 1;
            }

            // Render cursor at end of line
            if self.focused
                && block_idx == self.cursor.0
                && self.cursor.1 >= text.len()
                && col < x + width
            {
                ctx.buffer
                    .set(col, row_y, Cell::new(' ').bg(self.cursor_bg));
            }
        }
    }

    /// Render preview
    fn render_preview(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16, height: u16) {
        let fg = self.fg.unwrap_or(Color::rgb(205, 214, 244));

        // Fill preview background
        for row in 0..height {
            for col in 0..width {
                ctx.buffer
                    .set(x + col, y + row, Cell::new(' ').bg(self.preview_bg));
            }
        }

        // Render blocks as formatted text
        for (row, block_idx) in (self.scroll..).take(height as usize).enumerate() {
            if block_idx >= self.blocks.len() {
                break;
            }

            let block = &self.blocks[block_idx];
            let row_y = y + row as u16;

            match block.block_type {
                BlockType::Heading1 => {
                    self.render_heading(ctx, x, row_y, width, &block.text(), 1);
                }
                BlockType::Heading2 => {
                    self.render_heading(ctx, x, row_y, width, &block.text(), 2);
                }
                BlockType::Heading3 => {
                    self.render_heading(ctx, x, row_y, width, &block.text(), 3);
                }
                BlockType::Quote => {
                    let mut col = x;
                    ctx.buffer.set(
                        col,
                        row_y,
                        Cell::new('│').fg(self.quote_fg).bg(self.preview_bg),
                    );
                    col += 2;
                    for ch in block.text().chars() {
                        if col >= x + width {
                            break;
                        }
                        ctx.buffer.set(
                            col,
                            row_y,
                            Cell::new(ch).fg(self.quote_fg).bg(self.preview_bg),
                        );
                        col += 1;
                    }
                }
                BlockType::BulletList => {
                    let mut col = x;
                    ctx.buffer
                        .set(col, row_y, Cell::new('•').fg(fg).bg(self.preview_bg));
                    col += 2;
                    for ch in block.text().chars() {
                        if col >= x + width {
                            break;
                        }
                        ctx.buffer
                            .set(col, row_y, Cell::new(ch).fg(fg).bg(self.preview_bg));
                        col += 1;
                    }
                }
                BlockType::NumberedList => {
                    let num = (block_idx + 1).to_string();
                    let mut col = x;
                    for ch in num.chars() {
                        if col < x + width {
                            ctx.buffer
                                .set(col, row_y, Cell::new(ch).fg(fg).bg(self.preview_bg));
                            col += 1;
                        }
                    }
                    ctx.buffer
                        .set(col, row_y, Cell::new('.').fg(fg).bg(self.preview_bg));
                    col += 2;
                    for ch in block.text().chars() {
                        if col >= x + width {
                            break;
                        }
                        ctx.buffer
                            .set(col, row_y, Cell::new(ch).fg(fg).bg(self.preview_bg));
                        col += 1;
                    }
                }
                BlockType::CodeBlock => {
                    for col in 0..width {
                        ctx.buffer
                            .set(x + col, row_y, Cell::new(' ').bg(self.code_bg));
                    }
                    let mut col = x + 1;
                    for ch in block.text().chars() {
                        if col >= x + width - 1 {
                            break;
                        }
                        ctx.buffer
                            .set(col, row_y, Cell::new(ch).fg(fg).bg(self.code_bg));
                        col += 1;
                    }
                }
                BlockType::HorizontalRule => {
                    for col in 0..width {
                        ctx.buffer.set(
                            x + col,
                            row_y,
                            Cell::new('─').fg(self.quote_fg).bg(self.preview_bg),
                        );
                    }
                }
                _ => {
                    let mut col = x;
                    for ch in block.text().chars() {
                        if col >= x + width {
                            break;
                        }
                        ctx.buffer
                            .set(col, row_y, Cell::new(ch).fg(fg).bg(self.preview_bg));
                        col += 1;
                    }
                }
            }
        }
    }

    /// Render heading in preview
    fn render_heading(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        width: u16,
        text: &str,
        level: usize,
    ) {
        let modifier = if level == 1 {
            Modifier::BOLD | Modifier::UNDERLINE
        } else {
            Modifier::BOLD
        };

        let mut col = x;
        for ch in text.chars() {
            if col >= x + width {
                break;
            }
            let mut cell = Cell::new(ch).fg(self.heading_fg).bg(self.preview_bg);
            cell.modifier = modifier;
            ctx.buffer.set(col, y, cell);
            col += 1;
        }
    }

    /// Render dialog
    fn render_dialog(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16, height: u16) {
        // Calculate dialog position (centered)
        let dialog_width = 40.min(width.saturating_sub(4));
        let dialog_height = 7;
        let dialog_x = x + (width.saturating_sub(dialog_width)) / 2;
        let dialog_y = y + (height.saturating_sub(dialog_height)) / 2;

        let bg = Color::rgb(49, 50, 68);
        let fg = Color::rgb(205, 214, 244);

        // Draw dialog background
        for row in 0..dialog_height {
            for col in 0..dialog_width {
                ctx.buffer
                    .set(dialog_x + col, dialog_y + row, Cell::new(' ').bg(bg));
            }
        }

        // Draw border
        ctx.buffer
            .set(dialog_x, dialog_y, Cell::new('┌').fg(fg).bg(bg));
        ctx.buffer.set(
            dialog_x + dialog_width - 1,
            dialog_y,
            Cell::new('┐').fg(fg).bg(bg),
        );
        ctx.buffer.set(
            dialog_x,
            dialog_y + dialog_height - 1,
            Cell::new('└').fg(fg).bg(bg),
        );
        ctx.buffer.set(
            dialog_x + dialog_width - 1,
            dialog_y + dialog_height - 1,
            Cell::new('┘').fg(fg).bg(bg),
        );
        for col in 1..dialog_width - 1 {
            ctx.buffer
                .set(dialog_x + col, dialog_y, Cell::new('─').fg(fg).bg(bg));
            ctx.buffer.set(
                dialog_x + col,
                dialog_y + dialog_height - 1,
                Cell::new('─').fg(fg).bg(bg),
            );
        }
        for row in 1..dialog_height - 1 {
            ctx.buffer
                .set(dialog_x, dialog_y + row, Cell::new('│').fg(fg).bg(bg));
            ctx.buffer.set(
                dialog_x + dialog_width - 1,
                dialog_y + row,
                Cell::new('│').fg(fg).bg(bg),
            );
        }

        match &self.dialog {
            DialogType::InsertLink { text, url, field } => {
                // Title
                let title = "Insert Link";
                let title_x = dialog_x + (dialog_width - title.len() as u16) / 2;
                for (i, ch) in title.chars().enumerate() {
                    ctx.buffer.set(
                        title_x + i as u16,
                        dialog_y + 1,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }

                // Text field
                let label = "Text: ";
                let input_bg = if *field == 0 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.buffer.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 3,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                for (i, ch) in text.chars().enumerate() {
                    if dialog_x + 8 + (i as u16) < dialog_x + dialog_width - 2 {
                        ctx.buffer.set(
                            dialog_x + 8 + i as u16,
                            dialog_y + 3,
                            Cell::new(ch).fg(fg).bg(input_bg),
                        );
                    }
                }

                // URL field
                let label = "URL:  ";
                let input_bg = if *field == 1 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.buffer.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 4,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                for (i, ch) in url.chars().enumerate() {
                    if dialog_x + 8 + (i as u16) < dialog_x + dialog_width - 2 {
                        ctx.buffer.set(
                            dialog_x + 8 + i as u16,
                            dialog_y + 4,
                            Cell::new(ch).fg(fg).bg(input_bg),
                        );
                    }
                }
            }
            DialogType::InsertImage { alt, src, field } => {
                // Title
                let title = "Insert Image";
                let title_x = dialog_x + (dialog_width - title.len() as u16) / 2;
                for (i, ch) in title.chars().enumerate() {
                    ctx.buffer.set(
                        title_x + i as u16,
                        dialog_y + 1,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }

                // Alt field
                let label = "Alt:  ";
                let input_bg = if *field == 0 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.buffer.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 3,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                for (i, ch) in alt.chars().enumerate() {
                    if dialog_x + 8 + (i as u16) < dialog_x + dialog_width - 2 {
                        ctx.buffer.set(
                            dialog_x + 8 + i as u16,
                            dialog_y + 3,
                            Cell::new(ch).fg(fg).bg(input_bg),
                        );
                    }
                }

                // Src field
                let label = "Src:  ";
                let input_bg = if *field == 1 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.buffer.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 4,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                for (i, ch) in src.chars().enumerate() {
                    if dialog_x + 8 + (i as u16) < dialog_x + dialog_width - 2 {
                        ctx.buffer.set(
                            dialog_x + 8 + i as u16,
                            dialog_y + 4,
                            Cell::new(ch).fg(fg).bg(input_bg),
                        );
                    }
                }
            }
            DialogType::None => {}
        }
    }
}

/// Create a new rich text editor
pub fn rich_text_editor() -> RichTextEditor {
    RichTextEditor::new()
}
