//! RichTextEditor core functionality
//!
//! This module contains the RichTextEditor struct definition and core builder methods.

use super::types::EditorViewMode;
use super::{Block, BlockType, DialogType, EditOp, TextFormat};
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Rich text editor widget
pub struct RichTextEditor {
    /// Document blocks
    pub(super) blocks: Vec<Block>,
    /// Cursor position (block, col)
    pub(super) cursor: (usize, usize),
    /// Selection anchor (if selecting)
    pub(super) anchor: Option<(usize, usize)>,
    /// Scroll offset
    pub(super) scroll: usize,
    /// Current format for new text
    pub(super) current_format: TextFormat,
    /// Undo stack
    pub(super) undo_stack: Vec<EditOp>,
    /// Redo stack
    pub(super) redo_stack: Vec<EditOp>,
    /// View mode
    pub(super) view_mode: EditorViewMode,
    /// Show toolbar
    pub(super) show_toolbar: bool,
    /// Focused state
    pub(super) focused: bool,
    /// Active dialog
    pub(super) dialog: DialogType,
    /// Colors
    pub(super) bg: Option<Color>,
    pub(super) fg: Option<Color>,
    pub(super) toolbar_bg: Color,
    pub(super) toolbar_fg: Color,
    pub(super) toolbar_active_bg: Color,
    pub(super) cursor_bg: Color,
    pub(super) selection_bg: Color,
    pub(super) preview_bg: Color,
    pub(super) heading_fg: Color,
    pub(super) code_bg: Color,
    pub(super) quote_fg: Color,
    #[allow(dead_code)]
    pub(super) link_fg: Color,
    /// Widget props
    pub(super) props: crate::widget::traits::WidgetProps,
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
            props: crate::widget::traits::WidgetProps::new(),
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

    /// Ensure cursor is visible
    pub(super) fn ensure_cursor_visible(&mut self) {
        if self.cursor.0 < self.scroll {
            self.scroll = self.cursor.0;
        }
    }

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
}

impl Default for RichTextEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(RichTextEditor);
impl_props_builders!(RichTextEditor);
