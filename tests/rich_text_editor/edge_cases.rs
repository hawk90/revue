//! Edge Cases tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

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
