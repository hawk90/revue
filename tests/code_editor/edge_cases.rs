//! Edge Cases tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{code_editor, CodeEditor, EditorConfig, IndentStyle};

#[test]
fn test_empty_content() {
    let editor = CodeEditor::new().content("");
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_single_char_content() {
    let editor = CodeEditor::new().content("x");
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.get_content(), "x");
}

#[test]
fn test_unicode_content() {
    let editor = CodeEditor::new().content("你好世界");
    assert_eq!(editor.get_content(), "你好世界");
}

#[test]
fn test_cursor_clamp_on_content_change() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 11);
    editor.set_content("hi");
    // Cursor should be clamped to new content bounds
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_delete_at_start() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 0);
    editor.delete_char_before();
    // Should do nothing
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_delete_at_end() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.delete_char_at();
    // Should do nothing
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_merge_lines_on_backspace() {
    let mut editor = CodeEditor::new().content("hello\nworld");
    editor.set_cursor(1, 0);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "helloworld");
    assert_eq!(editor.line_count(), 1);
}
