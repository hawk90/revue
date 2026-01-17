//! Text Editing tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

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
