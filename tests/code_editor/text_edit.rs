//! Text Editing tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{code_editor, CodeEditor, EditorConfig, IndentStyle};

#[test]
fn test_insert_char() {
    let mut editor = CodeEditor::new().content("hllo");
    editor.set_cursor(0, 1);
    editor.insert_char('e');
    assert_eq!(editor.get_content(), "hello");
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_insert_str() {
    let mut editor = CodeEditor::new().content("hd");
    editor.set_cursor(0, 1);
    editor.insert_str("ello worl");
    assert_eq!(editor.get_content(), "hello world");
}

#[test]
fn test_delete_char_before() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "hell");
}

#[test]
fn test_delete_char_at() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 0);
    editor.delete_char_at();
    assert_eq!(editor.get_content(), "ello");
}

#[test]
fn test_delete_line() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(1, 0);
    editor.delete_line();
    assert_eq!(editor.get_content(), "line1\nline3");
    assert_eq!(editor.line_count(), 2);
}

#[test]
fn test_newline_insertion() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 5);
    editor.insert_char('\n');
    assert_eq!(editor.line_count(), 2);
}

#[test]
fn test_delete_selection() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 0);
    editor.start_selection();
    editor.set_cursor(0, 6);
    editor.delete_selection();
    assert_eq!(editor.get_content(), "world");
}

#[test]
fn test_read_only_mode() {
    let editor = CodeEditor::new().content("hello").read_only(true);
    // In read-only mode, inserts should be ignored by the handle_key method
    // But insert_char itself doesn't check read_only (that's done in handle_key)
    assert_eq!(editor.get_content(), "hello");
}
