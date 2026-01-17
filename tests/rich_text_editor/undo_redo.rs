//! Undo/Redo tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

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
