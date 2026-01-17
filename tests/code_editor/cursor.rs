//! Cursor Navigation tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{code_editor, CodeEditor, EditorConfig, IndentStyle};

#[test]
fn test_cursor_move_right() {
    let mut editor = CodeEditor::new().content("hello");
    assert_eq!(editor.cursor_position(), (0, 0));
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 1));
    editor.move_right();
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 3));
}

#[test]
fn test_cursor_move_left() {
    let mut editor = CodeEditor::new().content("hello");
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
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
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
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
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
fn test_cursor_move_home() {
    let mut editor = CodeEditor::new().content("    hello");
    editor.set_cursor(0, 8);
    // First home goes to first non-whitespace (column 4)
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 4));
    // Second home goes to column 0
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_move_end() {
    let mut editor = CodeEditor::new().content("hello");
    editor.move_end();
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_cursor_document_navigation() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3\nline4");
    editor.move_document_end();
    assert_eq!(editor.cursor_position(), (3, 5));
    editor.move_document_start();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_word_navigation() {
    let mut editor = CodeEditor::new().content("hello world foo");
    editor.move_word_right();
    assert_eq!(editor.cursor_position(), (0, 6)); // After "hello "
    editor.move_word_right();
    assert_eq!(editor.cursor_position(), (0, 12)); // After "world "

    editor.move_word_left();
    assert_eq!(editor.cursor_position(), (0, 6));
}

#[test]
fn test_cursor_page_navigation() {
    let mut editor = CodeEditor::new().content("1\n2\n3\n4\n5\n6\n7\n8\n9\n10");
    editor.page_down(5);
    assert_eq!(editor.cursor_position(), (5, 0));
    editor.page_up(3);
    assert_eq!(editor.cursor_position(), (2, 0));
}

#[test]
fn test_cursor_wrap_between_lines() {
    let mut editor = CodeEditor::new().content("ab\ncd");
    editor.set_cursor(0, 2);
    editor.move_right();
    // Should wrap to next line
    assert_eq!(editor.cursor_position(), (1, 0));

    editor.move_left();
    // Should wrap back to previous line
    assert_eq!(editor.cursor_position(), (0, 2));
}
