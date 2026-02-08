//! RichTextEditor core functionality
//!
//! This module contains the RichTextEditor struct definition and core builder methods.

use super::types::EditorViewMode;
use super::{Block, BlockType, DialogType, EditOp, TextFormat};
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Rich text editor widget
#[derive(Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // RichTextEditor construction tests
    // =========================================================================

    #[test]
    fn test_rich_text_editor_new() {
        let editor = RichTextEditor::new();
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_rich_text_editor_default() {
        let editor = RichTextEditor::default();
        assert_eq!(editor.blocks.len(), 1);
    }

    // =========================================================================
    // Content builder tests
    // =========================================================================

    #[test]
    fn test_rich_text_editor_content_plain() {
        let editor = RichTextEditor::new().content("Hello world");
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_rich_text_editor_content_multiline() {
        let editor = RichTextEditor::new().content("Line 1\nLine 2\nLine 3");
        assert_eq!(editor.blocks.len(), 3);
    }

    #[test]
    fn test_rich_text_editor_content_empty() {
        let editor = RichTextEditor::new().content("");
        // Empty content should still have one block
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_rich_text_editor_from_markdown() {
        let editor = RichTextEditor::new().from_markdown("# Heading\n\nParagraph");
        // Creates 3 blocks: heading, empty paragraph, actual paragraph
        assert_eq!(editor.blocks.len(), 3);
    }

    #[test]
    fn test_rich_text_editor_from_markdown_empty() {
        let editor = RichTextEditor::new().from_markdown("");
        assert_eq!(editor.blocks.len(), 1);
    }

    // =========================================================================
    // Markdown parsing tests
    // =========================================================================

    #[test]
    fn test_markdown_heading1() {
        let editor = RichTextEditor::new().from_markdown("# Title");
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_markdown_heading2() {
        let editor = RichTextEditor::new().from_markdown("## Subtitle");
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_markdown_heading3() {
        let editor = RichTextEditor::new().from_markdown("### Section");
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_markdown_bullet_list() {
        let editor = RichTextEditor::new().from_markdown("- Item 1\n- Item 2");
        assert_eq!(editor.blocks.len(), 2);
    }

    #[test]
    fn test_markdown_numbered_list() {
        let editor = RichTextEditor::new().from_markdown("1. First\n2. Second");
        assert_eq!(editor.blocks.len(), 2);
    }

    #[test]
    fn test_markdown_quote() {
        let editor = RichTextEditor::new().from_markdown("> Quote text");
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_markdown_code_block() {
        let editor = RichTextEditor::new().from_markdown("```\ncode here\n```");
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_markdown_code_block_with_language() {
        let editor = RichTextEditor::new().from_markdown("```rust\nfn main() {}\n```");
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_markdown_horizontal_rule() {
        let editor = RichTextEditor::new().from_markdown("---");
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_markdown_horizontal_rule_alt() {
        let editor = RichTextEditor::new().from_markdown("***");
        assert_eq!(editor.blocks.len(), 1);
    }

    // =========================================================================
    // Dialog tests
    // =========================================================================

    #[test]
    fn test_dialog_not_open_initially() {
        let editor = RichTextEditor::new();
        assert!(!editor.is_dialog_open());
    }

    #[test]
    fn test_open_link_dialog() {
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        assert!(editor.is_dialog_open());
    }

    #[test]
    fn test_open_image_dialog() {
        let mut editor = RichTextEditor::new();
        editor.open_image_dialog();
        assert!(editor.is_dialog_open());
    }

    #[test]
    fn test_close_dialog() {
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        editor.close_dialog();
        assert!(!editor.is_dialog_open());
    }

    // =========================================================================
    // Insert tests
    // =========================================================================

    #[test]
    fn test_insert_link() {
        let mut editor = RichTextEditor::new();
        editor.insert_link("text", "url");
        // Just verify it doesn't panic
    }

    #[test]
    fn test_insert_image() {
        let mut editor = RichTextEditor::new();
        editor.insert_image("alt", "src");
        // Just verify it doesn't panic
    }

    // =========================================================================
    // Builder tests
    // =========================================================================

    #[test]
    fn test_view_mode_builder() {
        let editor = RichTextEditor::new().view_mode(EditorViewMode::Preview);
        assert!(editor.blocks.len() >= 1);
    }

    #[test]
    fn test_toolbar_builder() {
        let editor = RichTextEditor::new().toolbar(false);
        assert!(editor.blocks.len() >= 1);
    }

    #[test]
    fn test_focused_builder() {
        let editor = RichTextEditor::new().focused(false);
        assert!(editor.blocks.len() >= 1);
    }

    #[test]
    fn test_bg_builder() {
        let editor = RichTextEditor::new().bg(Color::BLACK);
        assert!(editor.blocks.len() >= 1);
    }

    #[test]
    fn test_fg_builder() {
        let editor = RichTextEditor::new().fg(Color::WHITE);
        assert!(editor.blocks.len() >= 1);
    }

    // =========================================================================
    // Content access tests
    // =========================================================================

    #[test]
    fn test_get_content() {
        let editor = RichTextEditor::new().content("Hello world");
        let content = editor.get_content();
        assert_eq!(content, "Hello world");
    }

    #[test]
    fn test_get_content_empty() {
        let editor = RichTextEditor::new();
        let content = editor.get_content();
        assert_eq!(content, "");
    }

    #[test]
    fn test_to_markdown() {
        let editor = RichTextEditor::new().content("# Title");
        let markdown = editor.to_markdown();
        assert!(markdown.contains("# Title"));
    }

    // =========================================================================
    // Cursor tests
    // =========================================================================

    #[test]
    fn test_cursor_position() {
        let editor = RichTextEditor::new();
        let pos = editor.cursor_position();
        assert_eq!(pos, (0, 0));
    }

    #[test]
    fn test_set_cursor() {
        let mut editor = RichTextEditor::new();
        editor.set_cursor(0, 5);
        // Just verify it doesn't panic
    }

    // =========================================================================
    // Block tests
    // =========================================================================

    #[test]
    fn test_block_count() {
        let editor = RichTextEditor::new().content("Line 1\nLine 2");
        assert_eq!(editor.block_count(), 2);
    }

    #[test]
    fn test_block_count_empty() {
        let editor = RichTextEditor::new();
        assert_eq!(editor.block_count(), 1);
    }

    // =========================================================================
    // Format tests
    // =========================================================================

    #[test]
    fn test_toggle_bold() {
        let mut editor = RichTextEditor::new();
        editor.toggle_bold();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toggle_italic() {
        let mut editor = RichTextEditor::new();
        editor.toggle_italic();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toggle_code() {
        let mut editor = RichTextEditor::new();
        editor.toggle_code();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_current_format() {
        let editor = RichTextEditor::new();
        let format = editor.current_format();
        // Just verify it doesn't panic
        let _ = format;
    }

    // =========================================================================
    // Undo/Redo tests
    // =========================================================================

    #[test]
    fn test_undo_empty() {
        let mut editor = RichTextEditor::new();
        editor.undo();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_redo_empty() {
        let mut editor = RichTextEditor::new();
        editor.redo();
        // Just verify it doesn't panic
    }
}
