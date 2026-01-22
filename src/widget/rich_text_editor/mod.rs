//! RichTextEditor widget for rich text editing with markdown support
//!
//! A WYSIWYG-style text editor with formatting toolbar, markdown shortcuts,
//! live preview, and export capabilities.

mod block;
mod core;
mod editing;
mod link;
mod rendering;
mod text_format;
mod types;

use super::traits::{RenderContext, View};
use crate::render::Cell;

// Public exports (also used internally)
pub use block::{Block, BlockType, FormattedSpan};
pub use core::RichTextEditor;
pub use link::{ImageRef, Link};
pub use text_format::TextFormat;
pub use types::{EditorViewMode, ToolbarAction};

/// Edit operation for undo/redo
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(super) enum EditOp {
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

/// Dialog type
#[derive(Clone, Debug)]
pub(super) enum DialogType {
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

/// Create a new rich text editor
pub fn rich_text_editor() -> RichTextEditor {
    RichTextEditor::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Key;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::Color;

    // =========================================================================
    // TextFormat Tests
    // =========================================================================

    #[test]
    fn test_text_format_default() {
        let fmt = TextFormat::default();
        assert!(!fmt.bold);
        assert!(!fmt.italic);
        assert!(!fmt.underline);
        assert!(!fmt.strikethrough);
        assert!(!fmt.code);
    }

    #[test]
    fn test_text_format_new() {
        let fmt = TextFormat::new();
        assert!(!fmt.bold);
        assert!(!fmt.italic);
    }

    #[test]
    fn test_text_format_toggle_bold() {
        let fmt = TextFormat::new().toggle_bold();
        assert!(fmt.bold);
        let fmt = fmt.toggle_bold();
        assert!(!fmt.bold);
    }

    #[test]
    fn test_text_format_toggle_italic() {
        let fmt = TextFormat::new().toggle_italic();
        assert!(fmt.italic);
        let fmt = fmt.toggle_italic();
        assert!(!fmt.italic);
    }

    #[test]
    fn test_text_format_toggle_underline() {
        let fmt = TextFormat::new().toggle_underline();
        assert!(fmt.underline);
    }

    #[test]
    fn test_text_format_toggle_strikethrough() {
        let fmt = TextFormat::new().toggle_strikethrough();
        assert!(fmt.strikethrough);
    }

    #[test]
    fn test_text_format_toggle_code() {
        let fmt = TextFormat::new().toggle_code();
        assert!(fmt.code);
    }

    #[test]
    fn test_text_format_multiple_toggles() {
        let fmt = TextFormat::new()
            .toggle_bold()
            .toggle_italic()
            .toggle_code();
        assert!(fmt.bold);
        assert!(fmt.italic);
        assert!(fmt.code);
        assert!(!fmt.underline);
        assert!(!fmt.strikethrough);
    }

    // =========================================================================
    // BlockType Tests
    // =========================================================================

    #[test]
    fn test_block_type_default() {
        assert_eq!(BlockType::default(), BlockType::Paragraph);
    }

    #[test]
    fn test_block_type_markdown_prefix_paragraph() {
        assert_eq!(BlockType::Paragraph.markdown_prefix(), "");
    }

    #[test]
    fn test_block_type_markdown_prefix_headings() {
        assert_eq!(BlockType::Heading1.markdown_prefix(), "# ");
        assert_eq!(BlockType::Heading2.markdown_prefix(), "## ");
        assert_eq!(BlockType::Heading3.markdown_prefix(), "### ");
        assert_eq!(BlockType::Heading4.markdown_prefix(), "#### ");
        assert_eq!(BlockType::Heading5.markdown_prefix(), "##### ");
        assert_eq!(BlockType::Heading6.markdown_prefix(), "###### ");
    }

    #[test]
    fn test_block_type_markdown_prefix_quote() {
        assert_eq!(BlockType::Quote.markdown_prefix(), "> ");
    }

    #[test]
    fn test_block_type_markdown_prefix_lists() {
        assert_eq!(BlockType::BulletList.markdown_prefix(), "- ");
        assert_eq!(BlockType::NumberedList.markdown_prefix(), "1. ");
    }

    #[test]
    fn test_block_type_markdown_prefix_code_block() {
        assert_eq!(BlockType::CodeBlock.markdown_prefix(), "```\n");
    }

    #[test]
    fn test_block_type_markdown_prefix_horizontal_rule() {
        assert_eq!(BlockType::HorizontalRule.markdown_prefix(), "---");
    }

    // =========================================================================
    // FormattedSpan Tests
    // =========================================================================

    #[test]
    fn test_formatted_span_new() {
        let span = FormattedSpan::new("Hello");
        assert_eq!(span.text, "Hello");
        assert!(!span.format.bold);
    }

    #[test]
    fn test_formatted_span_with_format() {
        let format = TextFormat::new().toggle_bold();
        let span = FormattedSpan::new("Bold text").with_format(format);
        assert!(span.format.bold);
        assert_eq!(span.text, "Bold text");
    }

    // =========================================================================
    // Block Tests
    // =========================================================================

    #[test]
    fn test_block_paragraph() {
        let block = Block::paragraph("Hello World");
        assert_eq!(block.block_type, BlockType::Paragraph);
        assert_eq!(block.text(), "Hello World");
    }

    #[test]
    fn test_block_new() {
        let block = Block::new(BlockType::Heading1);
        assert_eq!(block.block_type, BlockType::Heading1);
        assert_eq!(block.text(), "");
    }

    #[test]
    fn test_block_text() {
        let block = Block::paragraph("Test content");
        assert_eq!(block.text(), "Test content");
    }

    #[test]
    fn test_block_set_text() {
        let mut block = Block::paragraph("Old");
        block.set_text("New");
        assert_eq!(block.text(), "New");
    }

    #[test]
    fn test_block_len() {
        let block = Block::paragraph("Hello");
        assert_eq!(block.len(), 5);
    }

    #[test]
    fn test_block_is_empty() {
        let block = Block::paragraph("");
        assert!(block.is_empty());

        let block = Block::paragraph("Not empty");
        assert!(!block.is_empty());
    }

    #[test]
    fn test_block_to_markdown_paragraph() {
        let block = Block::paragraph("Plain text");
        assert_eq!(block.to_markdown(), "Plain text");
    }

    #[test]
    fn test_block_to_markdown_heading() {
        let mut block = Block::new(BlockType::Heading1);
        block.set_text("Title");
        assert_eq!(block.to_markdown(), "# Title");
    }

    #[test]
    fn test_block_to_markdown_quote() {
        let mut block = Block::new(BlockType::Quote);
        block.set_text("Quoted text");
        assert_eq!(block.to_markdown(), "> Quoted text");
    }

    #[test]
    fn test_block_to_markdown_code_block() {
        let mut block = Block::new(BlockType::CodeBlock);
        block.set_text("let x = 1;");
        block.language = Some("rust".to_string());
        assert_eq!(block.to_markdown(), "```rust\nlet x = 1;\n```");
    }

    #[test]
    fn test_block_to_markdown_horizontal_rule() {
        let block = Block::new(BlockType::HorizontalRule);
        assert_eq!(block.to_markdown(), "---");
    }

    // =========================================================================
    // Link Tests
    // =========================================================================

    #[test]
    fn test_link_new() {
        let link = Link::new("Example", "https://example.com");
        assert_eq!(link.text, "Example");
        assert_eq!(link.url, "https://example.com");
        assert!(link.title.is_none());
    }

    #[test]
    fn test_link_with_title() {
        let link = Link::new("Example", "https://example.com").with_title("My Title");
        assert_eq!(link.title, Some("My Title".to_string()));
    }

    #[test]
    fn test_link_to_markdown() {
        let link = Link::new("Example", "https://example.com");
        assert_eq!(link.to_markdown(), "[Example](https://example.com)");
    }

    #[test]
    fn test_link_to_markdown_with_title() {
        let link = Link::new("Example", "https://example.com").with_title("Title");
        assert_eq!(
            link.to_markdown(),
            "[Example](https://example.com \"Title\")"
        );
    }

    // =========================================================================
    // ImageRef Tests
    // =========================================================================

    #[test]
    fn test_image_ref_new() {
        let img = ImageRef::new("Alt text", "/path/to/image.png");
        assert_eq!(img.alt, "Alt text");
        assert_eq!(img.src, "/path/to/image.png");
        assert!(img.title.is_none());
    }

    #[test]
    fn test_image_ref_with_title() {
        let img = ImageRef::new("Alt", "img.png").with_title("Image Title");
        assert_eq!(img.title, Some("Image Title".to_string()));
    }

    #[test]
    fn test_image_ref_to_markdown() {
        let img = ImageRef::new("Alt", "img.png");
        assert_eq!(img.to_markdown(), "![Alt](img.png)");
    }

    #[test]
    fn test_image_ref_to_markdown_with_title() {
        let img = ImageRef::new("Alt", "img.png").with_title("Title");
        assert_eq!(img.to_markdown(), "![Alt](img.png \"Title\")");
    }

    // =========================================================================
    // EditorViewMode Tests
    // =========================================================================

    #[test]
    fn test_editor_view_mode_default() {
        assert_eq!(EditorViewMode::default(), EditorViewMode::Editor);
    }

    // =========================================================================
    // RichTextEditor Creation Tests
    // =========================================================================

    #[test]
    fn test_rich_text_editor_new() {
        let editor = RichTextEditor::new();
        assert_eq!(editor.block_count(), 1);
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    #[test]
    fn test_rich_text_editor_default() {
        let editor = RichTextEditor::default();
        assert_eq!(editor.block_count(), 1);
    }

    #[test]
    fn test_rich_text_editor_helper() {
        let editor = rich_text_editor();
        assert_eq!(editor.block_count(), 1);
    }

    #[test]
    fn test_rich_text_editor_content() {
        let editor = RichTextEditor::new().content("Line 1\nLine 2\nLine 3");
        assert_eq!(editor.block_count(), 3);
        assert_eq!(editor.get_content(), "Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_rich_text_editor_content_empty() {
        let editor = RichTextEditor::new().content("");
        assert_eq!(editor.block_count(), 1);
    }

    #[test]
    fn test_rich_text_editor_set_content() {
        let mut editor = RichTextEditor::new();
        editor.set_content("New content\nSecond line");
        assert_eq!(editor.block_count(), 2);
        assert_eq!(editor.get_content(), "New content\nSecond line");
    }

    #[test]
    fn test_rich_text_editor_view_mode() {
        let editor = RichTextEditor::new().view_mode(EditorViewMode::Split);
        assert_eq!(editor.view_mode, EditorViewMode::Split);
    }

    #[test]
    fn test_rich_text_editor_toolbar() {
        let editor = RichTextEditor::new().toolbar(false);
        assert!(!editor.show_toolbar);
    }

    #[test]
    fn test_rich_text_editor_focused() {
        let editor = RichTextEditor::new().focused(false);
        assert!(!editor.focused);
    }

    // =========================================================================
    // Cursor Navigation Tests
    // =========================================================================

    #[test]
    fn test_move_right() {
        let mut editor = RichTextEditor::new().content("Hello");
        assert_eq!(editor.cursor_position(), (0, 0));
        editor.move_right();
        assert_eq!(editor.cursor_position(), (0, 1));
    }

    #[test]
    fn test_move_left() {
        let mut editor = RichTextEditor::new().content("Hello");
        editor.set_cursor(0, 3);
        editor.move_left();
        assert_eq!(editor.cursor_position(), (0, 2));
    }

    #[test]
    fn test_move_left_at_start() {
        let mut editor = RichTextEditor::new().content("Hello");
        editor.move_left();
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    #[test]
    fn test_move_right_to_next_line() {
        let mut editor = RichTextEditor::new().content("Hi\nThere");
        editor.set_cursor(0, 2); // End of first line
        editor.move_right();
        assert_eq!(editor.cursor_position(), (1, 0));
    }

    #[test]
    fn test_move_left_to_previous_line() {
        let mut editor = RichTextEditor::new().content("Hi\nThere");
        editor.set_cursor(1, 0);
        editor.move_left();
        assert_eq!(editor.cursor_position(), (0, 2));
    }

    #[test]
    fn test_move_up() {
        let mut editor = RichTextEditor::new().content("Line 1\nLine 2");
        editor.set_cursor(1, 3);
        editor.move_up();
        assert_eq!(editor.cursor_position(), (0, 3));
    }

    #[test]
    fn test_move_up_at_first_line() {
        let mut editor = RichTextEditor::new().content("Only line");
        editor.set_cursor(0, 5);
        editor.move_up();
        assert_eq!(editor.cursor_position(), (0, 5));
    }

    #[test]
    fn test_move_down() {
        let mut editor = RichTextEditor::new().content("Line 1\nLine 2");
        editor.set_cursor(0, 3);
        editor.move_down();
        assert_eq!(editor.cursor_position(), (1, 3));
    }

    #[test]
    fn test_move_down_at_last_line() {
        let mut editor = RichTextEditor::new().content("Only line");
        editor.set_cursor(0, 5);
        editor.move_down();
        assert_eq!(editor.cursor_position(), (0, 5));
    }

    #[test]
    fn test_move_home() {
        let mut editor = RichTextEditor::new().content("Hello World");
        editor.set_cursor(0, 6);
        editor.move_home();
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    #[test]
    fn test_move_end() {
        let mut editor = RichTextEditor::new().content("Hello");
        editor.move_end();
        assert_eq!(editor.cursor_position(), (0, 5));
    }

    #[test]
    fn test_move_document_start() {
        let mut editor = RichTextEditor::new().content("Line 1\nLine 2\nLine 3");
        editor.set_cursor(2, 3);
        editor.move_document_start();
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    #[test]
    fn test_move_document_end() {
        let mut editor = RichTextEditor::new().content("Line 1\nLine 2\nEnd");
        editor.move_document_end();
        assert_eq!(editor.cursor_position(), (2, 3));
    }

    #[test]
    fn test_set_cursor() {
        let mut editor = RichTextEditor::new().content("Hello\nWorld");
        editor.set_cursor(1, 3);
        assert_eq!(editor.cursor_position(), (1, 3));
    }

    #[test]
    fn test_set_cursor_clamps_block() {
        let mut editor = RichTextEditor::new().content("Hello");
        editor.set_cursor(100, 0);
        assert_eq!(editor.cursor_position().0, 0);
    }

    #[test]
    fn test_set_cursor_clamps_col() {
        let mut editor = RichTextEditor::new().content("Hi");
        editor.set_cursor(0, 100);
        assert_eq!(editor.cursor_position().1, 2);
    }

    // =========================================================================
    // Text Editing Tests
    // =========================================================================

    #[test]
    fn test_insert_char() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('H');
        editor.insert_char('i');
        assert_eq!(editor.get_content(), "Hi");
    }

    #[test]
    fn test_insert_char_at_position() {
        let mut editor = RichTextEditor::new().content("Hllo");
        editor.set_cursor(0, 1);
        editor.insert_char('e');
        assert_eq!(editor.get_content(), "Hello");
    }

    #[test]
    fn test_insert_str() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("Hello World");
        assert_eq!(editor.get_content(), "Hello World");
    }

    #[test]
    fn test_insert_newline() {
        let mut editor = RichTextEditor::new().content("HelloWorld");
        editor.set_cursor(0, 5);
        editor.insert_char('\n');
        assert_eq!(editor.block_count(), 2);
        assert_eq!(editor.get_content(), "Hello\nWorld");
    }

    #[test]
    fn test_delete_char_before() {
        let mut editor = RichTextEditor::new().content("Hello");
        editor.set_cursor(0, 5);
        editor.delete_char_before();
        assert_eq!(editor.get_content(), "Hell");
    }

    #[test]
    fn test_delete_char_before_at_start() {
        let mut editor = RichTextEditor::new().content("Hello");
        editor.delete_char_before();
        assert_eq!(editor.get_content(), "Hello");
    }

    #[test]
    fn test_delete_char_before_merges_lines() {
        let mut editor = RichTextEditor::new().content("Hello\nWorld");
        editor.set_cursor(1, 0);
        editor.delete_char_before();
        assert_eq!(editor.block_count(), 1);
        assert_eq!(editor.get_content(), "HelloWorld");
    }

    #[test]
    fn test_delete_char_at() {
        let mut editor = RichTextEditor::new().content("Hello");
        editor.set_cursor(0, 0);
        editor.delete_char_at();
        assert_eq!(editor.get_content(), "ello");
    }

    #[test]
    fn test_delete_char_at_end() {
        let mut editor = RichTextEditor::new().content("Hello\nWorld");
        editor.set_cursor(0, 5);
        editor.delete_char_at();
        assert_eq!(editor.get_content(), "HelloWorld");
    }

    #[test]
    fn test_delete_block() {
        let mut editor = RichTextEditor::new().content("Line 1\nLine 2\nLine 3");
        editor.set_cursor(1, 0);
        editor.delete_block();
        assert_eq!(editor.block_count(), 2);
    }

    #[test]
    fn test_delete_block_single() {
        let mut editor = RichTextEditor::new().content("Only line");
        editor.delete_block();
        assert_eq!(editor.block_count(), 1);
        assert_eq!(editor.get_content(), "");
    }

    // =========================================================================
    // Selection Tests
    // =========================================================================

    #[test]
    fn test_start_selection() {
        let mut editor = RichTextEditor::new().content("Hello");
        editor.set_cursor(0, 2);
        editor.start_selection();
        assert!(editor.has_selection());
    }

    #[test]
    fn test_clear_selection() {
        let mut editor = RichTextEditor::new().content("Hello");
        editor.start_selection();
        editor.clear_selection();
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_get_selection_single_line() {
        let mut editor = RichTextEditor::new().content("Hello World");
        editor.set_cursor(0, 0);
        editor.start_selection();
        editor.set_cursor(0, 5);
        // Note: anchor is at (0,0), cursor at (0,5)
        // But cursor movement clears selection, so we need different approach
    }

    #[test]
    fn test_has_selection_initially_false() {
        let editor = RichTextEditor::new();
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_delete_selection() {
        let mut editor = RichTextEditor::new().content("Hello World");
        editor.anchor = Some((0, 0));
        editor.cursor = (0, 5);
        editor.delete_selection();
        assert_eq!(editor.get_content(), " World");
    }

    // =========================================================================
    // Formatting Tests
    // =========================================================================

    #[test]
    fn test_toggle_bold() {
        let mut editor = RichTextEditor::new();
        assert!(!editor.current_format().bold);
        editor.toggle_bold();
        assert!(editor.current_format().bold);
        editor.toggle_bold();
        assert!(!editor.current_format().bold);
    }

    #[test]
    fn test_toggle_italic() {
        let mut editor = RichTextEditor::new();
        editor.toggle_italic();
        assert!(editor.current_format().italic);
    }

    #[test]
    fn test_toggle_underline() {
        let mut editor = RichTextEditor::new();
        editor.toggle_underline();
        assert!(editor.current_format().underline);
    }

    #[test]
    fn test_toggle_strikethrough() {
        let mut editor = RichTextEditor::new();
        editor.toggle_strikethrough();
        assert!(editor.current_format().strikethrough);
    }

    #[test]
    fn test_toggle_code() {
        let mut editor = RichTextEditor::new();
        editor.toggle_code();
        assert!(editor.current_format().code);
    }

    #[test]
    fn test_set_block_type() {
        let mut editor = RichTextEditor::new().content("Title");
        editor.set_block_type(BlockType::Heading1);
        assert_eq!(editor.current_block_type(), BlockType::Heading1);
    }

    #[test]
    fn test_current_block_type() {
        let editor = RichTextEditor::new();
        assert_eq!(editor.current_block_type(), BlockType::Paragraph);
    }

    // =========================================================================
    // Undo/Redo Tests
    // =========================================================================

    #[test]
    fn test_undo_insert_char() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('H');
        editor.insert_char('i');
        assert_eq!(editor.get_content(), "Hi");
        editor.undo();
        assert_eq!(editor.get_content(), "H");
    }

    #[test]
    fn test_redo_insert_char() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('H');
        editor.undo();
        assert_eq!(editor.get_content(), "");
        editor.redo();
        assert_eq!(editor.get_content(), "H");
    }

    #[test]
    fn test_undo_delete_char() {
        let mut editor = RichTextEditor::new().content("Hi");
        editor.set_cursor(0, 2);
        editor.delete_char_before();
        assert_eq!(editor.get_content(), "H");
        editor.undo();
        assert_eq!(editor.get_content(), "Hi");
    }

    #[test]
    fn test_undo_block_type_change() {
        let mut editor = RichTextEditor::new().content("Title");
        editor.set_block_type(BlockType::Heading1);
        assert_eq!(editor.current_block_type(), BlockType::Heading1);
        editor.undo();
        assert_eq!(editor.current_block_type(), BlockType::Paragraph);
    }

    #[test]
    fn test_insert_clears_redo_stack() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('A');
        editor.undo();
        editor.insert_char('B');
        editor.redo(); // Should do nothing
        assert_eq!(editor.get_content(), "B");
    }

    // =========================================================================
    // Markdown Parsing Tests
    // =========================================================================

    #[test]
    fn test_from_markdown_headings() {
        let editor = RichTextEditor::new().from_markdown("# Heading 1\n## Heading 2");
        assert_eq!(editor.block_count(), 2);
    }

    #[test]
    fn test_from_markdown_quote() {
        let editor = RichTextEditor::new().from_markdown("> This is a quote");
        assert_eq!(editor.blocks[0].block_type, BlockType::Quote);
    }

    #[test]
    fn test_from_markdown_bullet_list() {
        let editor = RichTextEditor::new().from_markdown("- Item 1\n- Item 2");
        assert_eq!(editor.blocks[0].block_type, BlockType::BulletList);
        assert_eq!(editor.blocks[1].block_type, BlockType::BulletList);
    }

    #[test]
    fn test_from_markdown_bullet_list_asterisk() {
        let editor = RichTextEditor::new().from_markdown("* Item");
        assert_eq!(editor.blocks[0].block_type, BlockType::BulletList);
    }

    #[test]
    fn test_from_markdown_numbered_list() {
        let editor = RichTextEditor::new().from_markdown("1. First\n2. Second");
        assert_eq!(editor.blocks[0].block_type, BlockType::NumberedList);
    }

    #[test]
    fn test_from_markdown_horizontal_rule() {
        let editor = RichTextEditor::new().from_markdown("---");
        assert_eq!(editor.blocks[0].block_type, BlockType::HorizontalRule);
    }

    #[test]
    fn test_from_markdown_code_block() {
        let editor = RichTextEditor::new().from_markdown("```rust\nlet x = 1;\n```");
        assert_eq!(editor.blocks[0].block_type, BlockType::CodeBlock);
        assert_eq!(editor.blocks[0].language, Some("rust".to_string()));
    }

    #[test]
    fn test_from_markdown_empty() {
        let editor = RichTextEditor::new().from_markdown("");
        assert_eq!(editor.block_count(), 1);
    }

    #[test]
    fn test_to_markdown() {
        let mut editor = RichTextEditor::new().content("Title");
        editor.set_block_type(BlockType::Heading1);
        assert_eq!(editor.to_markdown(), "# Title");
    }

    // =========================================================================
    // Link and Image Tests
    // =========================================================================

    #[test]
    fn test_insert_link() {
        let mut editor = RichTextEditor::new();
        editor.insert_link("Example", "https://example.com");
        assert_eq!(editor.get_content(), "[Example](https://example.com)");
    }

    #[test]
    fn test_insert_image() {
        let mut editor = RichTextEditor::new();
        editor.insert_image("Alt text", "image.png");
        assert_eq!(editor.get_content(), "![Alt text](image.png)");
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
    // Toolbar Action Tests
    // =========================================================================

    #[test]
    fn test_toolbar_action_bold() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Bold);
        assert!(editor.current_format().bold);
    }

    #[test]
    fn test_toolbar_action_heading() {
        let mut editor = RichTextEditor::new().content("Title");
        editor.toolbar_action(ToolbarAction::Heading1);
        assert_eq!(editor.current_block_type(), BlockType::Heading1);
    }

    #[test]
    fn test_toolbar_action_quote() {
        let mut editor = RichTextEditor::new().content("Quote");
        editor.toolbar_action(ToolbarAction::Quote);
        assert_eq!(editor.current_block_type(), BlockType::Quote);
    }

    #[test]
    fn test_toolbar_action_bullet_list() {
        let mut editor = RichTextEditor::new().content("Item");
        editor.toolbar_action(ToolbarAction::BulletList);
        assert_eq!(editor.current_block_type(), BlockType::BulletList);
    }

    #[test]
    fn test_toolbar_action_undo() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('A');
        editor.toolbar_action(ToolbarAction::Undo);
        assert_eq!(editor.get_content(), "");
    }

    #[test]
    fn test_toolbar_action_redo() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('A');
        editor.undo();
        editor.toolbar_action(ToolbarAction::Redo);
        assert_eq!(editor.get_content(), "A");
    }

    // =========================================================================
    // Key Handling Tests
    // =========================================================================

    #[test]
    fn test_handle_key_char() {
        let mut editor = RichTextEditor::new();
        assert!(editor.handle_key(&Key::Char('H')));
        assert_eq!(editor.get_content(), "H");
    }

    #[test]
    fn test_handle_key_enter() {
        let mut editor = RichTextEditor::new().content("Hello");
        editor.set_cursor(0, 5);
        editor.handle_key(&Key::Enter);
        assert_eq!(editor.block_count(), 2);
    }

    #[test]
    fn test_handle_key_backspace() {
        let mut editor = RichTextEditor::new().content("Hi");
        editor.set_cursor(0, 2);
        editor.handle_key(&Key::Backspace);
        assert_eq!(editor.get_content(), "H");
    }

    #[test]
    fn test_handle_key_delete() {
        let mut editor = RichTextEditor::new().content("Hi");
        editor.set_cursor(0, 0);
        editor.handle_key(&Key::Delete);
        assert_eq!(editor.get_content(), "i");
    }

    #[test]
    fn test_handle_key_tab() {
        let mut editor = RichTextEditor::new();
        editor.handle_key(&Key::Tab);
        assert_eq!(editor.get_content(), "    ");
    }

    #[test]
    fn test_handle_key_left() {
        let mut editor = RichTextEditor::new().content("Hi");
        editor.set_cursor(0, 2);
        editor.handle_key(&Key::Left);
        assert_eq!(editor.cursor_position(), (0, 1));
    }

    #[test]
    fn test_handle_key_right() {
        let mut editor = RichTextEditor::new().content("Hi");
        editor.handle_key(&Key::Right);
        assert_eq!(editor.cursor_position(), (0, 1));
    }

    #[test]
    fn test_handle_key_up() {
        let mut editor = RichTextEditor::new().content("A\nB");
        editor.set_cursor(1, 0);
        editor.handle_key(&Key::Up);
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    #[test]
    fn test_handle_key_down() {
        let mut editor = RichTextEditor::new().content("A\nB");
        editor.handle_key(&Key::Down);
        assert_eq!(editor.cursor_position(), (1, 0));
    }

    #[test]
    fn test_handle_key_home() {
        let mut editor = RichTextEditor::new().content("Hello");
        editor.set_cursor(0, 3);
        editor.handle_key(&Key::Home);
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    #[test]
    fn test_handle_key_end() {
        let mut editor = RichTextEditor::new().content("Hello");
        editor.handle_key(&Key::End);
        assert_eq!(editor.cursor_position(), (0, 5));
    }

    #[test]
    fn test_handle_key_unknown() {
        let mut editor = RichTextEditor::new();
        assert!(!editor.handle_key(&Key::F(1)));
    }

    // =========================================================================
    // Markdown Shortcuts Tests
    // =========================================================================

    #[test]
    fn test_markdown_shortcut_heading() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("# ");
        editor.process_markdown_shortcuts();
        assert_eq!(editor.current_block_type(), BlockType::Heading1);
    }

    #[test]
    fn test_markdown_shortcut_quote() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("> ");
        editor.process_markdown_shortcuts();
        assert_eq!(editor.current_block_type(), BlockType::Quote);
    }

    #[test]
    fn test_markdown_shortcut_bullet() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("- ");
        editor.process_markdown_shortcuts();
        assert_eq!(editor.current_block_type(), BlockType::BulletList);
    }

    #[test]
    fn test_markdown_shortcut_numbered() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("1. ");
        editor.process_markdown_shortcuts();
        assert_eq!(editor.current_block_type(), BlockType::NumberedList);
    }

    // =========================================================================
    // Render Tests
    // =========================================================================

    #[test]
    fn test_render_basic() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let editor = RichTextEditor::new().content("Hello");
        editor.render(&mut ctx);
        // Should not panic
    }

    #[test]
    fn test_render_with_toolbar() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let editor = RichTextEditor::new().toolbar(true);
        editor.render(&mut ctx);
    }

    #[test]
    fn test_render_without_toolbar() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let editor = RichTextEditor::new().toolbar(false);
        editor.render(&mut ctx);
    }

    #[test]
    fn test_render_split_view() {
        let mut buffer = Buffer::new(80, 20);
        let area = Rect::new(0, 0, 80, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let editor = RichTextEditor::new()
            .view_mode(EditorViewMode::Split)
            .content("# Title");
        editor.render(&mut ctx);
    }

    #[test]
    fn test_render_preview_mode() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let editor = RichTextEditor::new()
            .view_mode(EditorViewMode::Preview)
            .content("Preview text");
        editor.render(&mut ctx);
    }

    #[test]
    fn test_render_small_area() {
        let mut buffer = Buffer::new(1, 1);
        let area = Rect::new(0, 0, 1, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let editor = RichTextEditor::new();
        editor.render(&mut ctx); // Should handle gracefully
    }

    #[test]
    fn test_render_with_dialog() {
        let mut buffer = Buffer::new(60, 20);
        let area = Rect::new(0, 0, 60, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        editor.render(&mut ctx);
    }

    #[test]
    fn test_render_all_block_types() {
        let mut buffer = Buffer::new(50, 20);
        let area = Rect::new(0, 0, 50, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let md = "# H1\n## H2\n### H3\n> Quote\n- Bullet\n1. Number\n```\ncode\n```\n---";
        let editor = RichTextEditor::new().from_markdown(md);
        editor.render(&mut ctx);
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_empty_editor_operations() {
        let mut editor = RichTextEditor::new();
        editor.move_left();
        editor.move_up();
        editor.delete_char_before();
        editor.delete_char_at();
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    #[test]
    fn test_undo_on_empty_stack() {
        let mut editor = RichTextEditor::new();
        editor.undo(); // Should not panic
        assert_eq!(editor.get_content(), "");
    }

    #[test]
    fn test_redo_on_empty_stack() {
        let mut editor = RichTextEditor::new();
        editor.redo(); // Should not panic
    }

    #[test]
    fn test_cursor_position_after_operations() {
        let mut editor = RichTextEditor::new().content("Hello");
        editor.move_end();
        assert_eq!(editor.cursor_position(), (0, 5));
        editor.insert_char('!');
        assert_eq!(editor.cursor_position(), (0, 6));
    }

    #[test]
    fn test_multi_block_navigation() {
        let mut editor = RichTextEditor::new().content("Short\nLonger line\nX");
        editor.set_cursor(1, 10);
        editor.move_down();
        // Cursor should clamp to shorter line length
        assert_eq!(editor.cursor_position(), (2, 1));
    }

    #[test]
    fn test_color_setters() {
        let editor = RichTextEditor::new().bg(Color::RED).fg(Color::WHITE);
        assert_eq!(editor.bg, Some(Color::RED));
        assert_eq!(editor.fg, Some(Color::WHITE));
    }
}
