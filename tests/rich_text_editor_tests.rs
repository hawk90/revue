//! Tests for RichTextEditor widget
//!
//! These tests use only the public API of the widget.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

// =============================================================================
// Basic Creation and Content Tests
// =============================================================================

#[test]
fn test_rich_text_editor_new() {
    let editor = RichTextEditor::new();
    assert_eq!(editor.block_count(), 1);
    assert_eq!(editor.cursor_position(), (0, 0));
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_rich_text_editor_constructor() {
    let editor = rich_text_editor().content("hello");
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_rich_text_editor_content() {
    let editor = RichTextEditor::new().content("line1\nline2\nline3");
    assert_eq!(editor.block_count(), 3);
    assert_eq!(editor.get_content(), "line1\nline2\nline3");
}

#[test]
fn test_rich_text_editor_set_content() {
    let mut editor = RichTextEditor::new();
    editor.set_content("new content");
    assert_eq!(editor.get_content(), "new content");
    assert_eq!(editor.cursor_position(), (0, 0));
}

// =============================================================================
// Cursor Navigation Tests
// =============================================================================

#[test]
fn test_cursor_move_right() {
    let mut editor = RichTextEditor::new().content("hello");
    assert_eq!(editor.cursor_position(), (0, 0));
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 1));
    editor.move_right();
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 3));
}

#[test]
fn test_cursor_move_left() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 3);
    editor.move_left();
    assert_eq!(editor.cursor_position(), (0, 2));
    editor.move_left();
    editor.move_left();
    editor.move_left();
    // Should stop at 0
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_move_down() {
    let mut editor = RichTextEditor::new().content("line1\nline2\nline3");
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 0));
    editor.move_down();
    assert_eq!(editor.cursor_position(), (2, 0));
    // Should stop at last line
    editor.move_down();
    assert_eq!(editor.cursor_position(), (2, 0));
}

#[test]
fn test_cursor_move_up() {
    let mut editor = RichTextEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(2, 0);
    editor.move_up();
    assert_eq!(editor.cursor_position(), (1, 0));
    editor.move_up();
    assert_eq!(editor.cursor_position(), (0, 0));
    // Should stop at first line
    editor.move_up();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_move_home_end() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 3);
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 0));
    editor.move_end();
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_cursor_document_navigation() {
    let mut editor = RichTextEditor::new().content("line1\nline2\nline3\nline4");
    editor.move_document_end();
    assert_eq!(editor.cursor_position(), (3, 5));
    editor.move_document_start();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_wrap_between_blocks() {
    let mut editor = RichTextEditor::new().content("ab\ncd");
    editor.set_cursor(0, 2);
    editor.move_right();
    // Should wrap to next line
    assert_eq!(editor.cursor_position(), (1, 0));

    editor.move_left();
    // Should wrap back to previous line
    assert_eq!(editor.cursor_position(), (0, 2));
}

// =============================================================================
// Selection Tests
// =============================================================================

#[test]
fn test_selection_basic() {
    let mut editor = RichTextEditor::new().content("hello world");
    assert!(!editor.has_selection());

    editor.start_selection();
    assert!(editor.has_selection());

    editor.clear_selection();
    assert!(!editor.has_selection());
}

#[test]
fn test_get_selection() {
    let mut editor = RichTextEditor::new().content("hello world");
    editor.set_cursor(0, 0);
    editor.start_selection();
    editor.set_cursor(0, 5);
    let sel = editor.get_selection();
    assert!(sel.is_some());
    assert_eq!(sel.unwrap(), "hello");
}

// =============================================================================
// Text Editing Tests
// =============================================================================

#[test]
fn test_insert_char() {
    let mut editor = RichTextEditor::new().content("hllo");
    editor.set_cursor(0, 1);
    editor.insert_char('e');
    assert_eq!(editor.get_content(), "hello");
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_insert_str() {
    let mut editor = RichTextEditor::new().content("hd");
    editor.set_cursor(0, 1);
    editor.insert_str("ello worl");
    assert_eq!(editor.get_content(), "hello world");
}

#[test]
fn test_delete_char_before() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "hell");
}

#[test]
fn test_delete_char_at() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 0);
    editor.delete_char_at();
    assert_eq!(editor.get_content(), "ello");
}

#[test]
fn test_delete_block() {
    let mut editor = RichTextEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(1, 0);
    editor.delete_block();
    assert_eq!(editor.block_count(), 2);
}

#[test]
fn test_newline_insertion() {
    let mut editor = RichTextEditor::new().content("hello world");
    editor.set_cursor(0, 5);
    editor.insert_char('\n');
    assert_eq!(editor.block_count(), 2);
}

#[test]
fn test_delete_selection() {
    let mut editor = RichTextEditor::new().content("hello world");
    editor.set_cursor(0, 0);
    editor.start_selection();
    editor.set_cursor(0, 6);
    editor.delete_selection();
    assert_eq!(editor.get_content(), "world");
}

// =============================================================================
// Undo/Redo Tests
// =============================================================================

#[test]
fn test_undo_insert() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.insert_char('!');
    assert_eq!(editor.get_content(), "hello!");

    editor.undo();
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_redo() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.insert_char('!');
    editor.undo();
    assert_eq!(editor.get_content(), "hello");

    editor.redo();
    assert_eq!(editor.get_content(), "hello!");
}

#[test]
fn test_undo_delete() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "hell");

    editor.undo();
    assert_eq!(editor.get_content(), "hello");
}

// =============================================================================
// Formatting Tests
// =============================================================================

#[test]
fn test_text_format_default() {
    let format = TextFormat::default();
    assert!(!format.bold);
    assert!(!format.italic);
    assert!(!format.underline);
    assert!(!format.strikethrough);
    assert!(!format.code);
}

#[test]
fn test_text_format_toggle() {
    let format = TextFormat::new()
        .toggle_bold()
        .toggle_italic()
        .toggle_code();
    assert!(format.bold);
    assert!(format.italic);
    assert!(!format.underline);
    assert!(format.code);
}

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

// =============================================================================
// Block Type Tests
// =============================================================================

#[test]
fn test_block_type_default() {
    let editor = RichTextEditor::new().content("text");
    assert_eq!(editor.current_block_type(), BlockType::Paragraph);
}

#[test]
fn test_set_block_type() {
    let mut editor = RichTextEditor::new().content("text");
    editor.set_block_type(BlockType::Heading1);
    assert_eq!(editor.current_block_type(), BlockType::Heading1);
}

#[test]
fn test_block_types() {
    let mut editor = RichTextEditor::new().content("text");

    editor.set_block_type(BlockType::Quote);
    assert_eq!(editor.current_block_type(), BlockType::Quote);

    editor.set_block_type(BlockType::BulletList);
    assert_eq!(editor.current_block_type(), BlockType::BulletList);

    editor.set_block_type(BlockType::CodeBlock);
    assert_eq!(editor.current_block_type(), BlockType::CodeBlock);
}

// =============================================================================
// Toolbar Action Tests
// =============================================================================

#[test]
fn test_toolbar_action_formatting() {
    let mut editor = RichTextEditor::new();

    editor.toolbar_action(ToolbarAction::Bold);
    assert!(editor.current_format().bold);

    editor.toolbar_action(ToolbarAction::Italic);
    assert!(editor.current_format().italic);

    editor.toolbar_action(ToolbarAction::Code);
    assert!(editor.current_format().code);
}

#[test]
fn test_toolbar_action_block_type() {
    let mut editor = RichTextEditor::new().content("text");

    editor.toolbar_action(ToolbarAction::Heading1);
    assert_eq!(editor.current_block_type(), BlockType::Heading1);

    editor.toolbar_action(ToolbarAction::Quote);
    assert_eq!(editor.current_block_type(), BlockType::Quote);
}

#[test]
fn test_toolbar_action_undo_redo() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.insert_char('!');

    editor.toolbar_action(ToolbarAction::Undo);
    assert_eq!(editor.get_content(), "hello");

    editor.toolbar_action(ToolbarAction::Redo);
    assert_eq!(editor.get_content(), "hello!");
}

// =============================================================================
// Markdown Export Tests
// =============================================================================

#[test]
fn test_to_markdown_paragraph() {
    let editor = RichTextEditor::new().content("hello world");
    assert_eq!(editor.to_markdown(), "hello world");
}

#[test]
fn test_to_markdown_heading() {
    let mut editor = RichTextEditor::new().content("Title");
    editor.set_block_type(BlockType::Heading1);
    assert_eq!(editor.to_markdown(), "# Title");
}

#[test]
fn test_to_markdown_quote() {
    let mut editor = RichTextEditor::new().content("quoted text");
    editor.set_block_type(BlockType::Quote);
    assert_eq!(editor.to_markdown(), "> quoted text");
}

#[test]
fn test_to_markdown_bullet_list() {
    let mut editor = RichTextEditor::new().content("item");
    editor.set_block_type(BlockType::BulletList);
    assert_eq!(editor.to_markdown(), "- item");
}

// =============================================================================
// Markdown Parsing Tests
// =============================================================================

#[test]
fn test_from_markdown_heading() {
    let editor = RichTextEditor::new().from_markdown("# Hello");
    assert_eq!(editor.current_block_type(), BlockType::Heading1);
    assert_eq!(editor.get_content(), "Hello");
}

#[test]
fn test_from_markdown_heading2() {
    let editor = RichTextEditor::new().from_markdown("## Section");
    assert_eq!(editor.current_block_type(), BlockType::Heading2);
    assert_eq!(editor.get_content(), "Section");
}

#[test]
fn test_from_markdown_quote() {
    let editor = RichTextEditor::new().from_markdown("> Quote");
    assert_eq!(editor.current_block_type(), BlockType::Quote);
    assert_eq!(editor.get_content(), "Quote");
}

#[test]
fn test_from_markdown_bullet_list() {
    let editor = RichTextEditor::new().from_markdown("- Item");
    assert_eq!(editor.current_block_type(), BlockType::BulletList);
    assert_eq!(editor.get_content(), "Item");
}

#[test]
fn test_from_markdown_horizontal_rule() {
    let editor = RichTextEditor::new().from_markdown("---");
    assert_eq!(editor.current_block_type(), BlockType::HorizontalRule);
}

// =============================================================================
// View Mode Tests
// =============================================================================

#[test]
fn test_view_mode_default() {
    let editor = RichTextEditor::new();
    // Default should be editor mode
    let _md = editor.to_markdown(); // Just verify it works
}

#[test]
fn test_view_mode_builder() {
    let _editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Split)
        .toolbar(true)
        .focused(true);
}

// =============================================================================
// Link and Image Tests
// =============================================================================

#[test]
fn test_insert_link() {
    let mut editor = RichTextEditor::new();
    editor.insert_link("Google", "https://google.com");
    assert_eq!(editor.get_content(), "[Google](https://google.com)");
}

#[test]
fn test_insert_image() {
    let mut editor = RichTextEditor::new();
    editor.insert_image("Logo", "logo.png");
    assert_eq!(editor.get_content(), "![Logo](logo.png)");
}

#[test]
fn test_dialog_open_close() {
    let mut editor = RichTextEditor::new();
    assert!(!editor.is_dialog_open());

    editor.open_link_dialog();
    assert!(editor.is_dialog_open());

    editor.close_dialog();
    assert!(!editor.is_dialog_open());

    editor.open_image_dialog();
    assert!(editor.is_dialog_open());

    editor.close_dialog();
    assert!(!editor.is_dialog_open());
}

// =============================================================================
// Block and Span Helper Tests
// =============================================================================

#[test]
fn test_block_paragraph() {
    let block = Block::paragraph("hello");
    assert_eq!(block.block_type, BlockType::Paragraph);
    assert_eq!(block.text(), "hello");
    assert_eq!(block.len(), 5);
    assert!(!block.is_empty());
}

#[test]
fn test_block_new() {
    let mut block = Block::new(BlockType::Heading1);
    assert_eq!(block.block_type, BlockType::Heading1);
    assert!(block.is_empty());

    block.set_text("Title");
    assert_eq!(block.text(), "Title");
}

#[test]
fn test_formatted_span() {
    let span = FormattedSpan::new("text").with_format(TextFormat {
        bold: true,
        italic: false,
        underline: false,
        strikethrough: false,
        code: false,
    });
    assert_eq!(span.text, "text");
    assert!(span.format.bold);
}

#[test]
fn test_block_type_markdown_prefix() {
    assert_eq!(BlockType::Heading1.markdown_prefix(), "# ");
    assert_eq!(BlockType::Heading2.markdown_prefix(), "## ");
    assert_eq!(BlockType::Quote.markdown_prefix(), "> ");
    assert_eq!(BlockType::BulletList.markdown_prefix(), "- ");
    assert_eq!(BlockType::Paragraph.markdown_prefix(), "");
}

// =============================================================================
// Rendering Tests
// =============================================================================

#[test]
fn test_render_empty() {
    let editor = RichTextEditor::new();
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_render_with_content() {
    let editor = RichTextEditor::new().content("hello\nworld");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}

#[test]
fn test_render_with_toolbar() {
    let editor = RichTextEditor::new().content("text").toolbar(true);
    let mut buffer = Buffer::new(60, 10);
    let area = Rect::new(0, 0, 60, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}

#[test]
fn test_render_split_view() {
    let editor = RichTextEditor::new()
        .content("# Heading\nParagraph")
        .view_mode(EditorViewMode::Split);
    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}

#[test]
fn test_render_preview_only() {
    let editor = RichTextEditor::new()
        .content("Text")
        .view_mode(EditorViewMode::Preview);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}

// =============================================================================
// Edge Cases
// =============================================================================

#[test]
fn test_empty_content() {
    let editor = RichTextEditor::new().content("");
    assert_eq!(editor.block_count(), 1);
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_single_char_content() {
    let editor = RichTextEditor::new().content("x");
    assert_eq!(editor.block_count(), 1);
    assert_eq!(editor.get_content(), "x");
}

#[test]
fn test_unicode_content() {
    let editor = RichTextEditor::new().content("你好世界");
    assert_eq!(editor.get_content(), "你好世界");
}

#[test]
fn test_delete_at_start() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 0);
    editor.delete_char_before();
    // Should do nothing
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_delete_at_end() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.delete_char_at();
    // Should do nothing (no more chars to delete)
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_merge_blocks_on_backspace() {
    let mut editor = RichTextEditor::new().content("hello\nworld");
    editor.set_cursor(1, 0);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "helloworld");
    assert_eq!(editor.block_count(), 1);
}
