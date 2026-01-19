//! Undo/Redo tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{code_editor, CodeEditor, EditorConfig, IndentStyle};

#[test]
fn test_undo_insert() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.insert_char('!');
    assert_eq!(editor.get_content(), "hello!");

    editor.undo();
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_redo() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.insert_char('!');
    editor.undo();
    assert_eq!(editor.get_content(), "hello");

    editor.redo();
    assert_eq!(editor.get_content(), "hello!");
}

#[test]
fn test_undo_delete() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "hell");

    editor.undo();
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_multiple_undo() {
    let mut editor = CodeEditor::new().content("a");
    editor.set_cursor(0, 1);
    editor.insert_char('b');
    editor.insert_char('c');
    assert_eq!(editor.get_content(), "abc");

    editor.undo();
    assert_eq!(editor.get_content(), "ab");
    editor.undo();
    assert_eq!(editor.get_content(), "a");
}
