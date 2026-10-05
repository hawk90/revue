//! Tests for RichTextEditor cursor movement

use revue::widget::RichTextEditor;

#[test]
fn test_move_right_stops_at_end_of_last_block() {
    let mut editor = RichTextEditor::new().content("hi");
    editor.move_right();
    editor.move_right();
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_move_up_from_second_block() {
    let mut editor = RichTextEditor::new().content("first\nsecond");
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 0));
    editor.move_up();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_move_document_end_empty() {
    let mut editor = RichTextEditor::new();
    editor.move_document_end();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_movement_sequence() {
    let mut editor = RichTextEditor::new().content("abc\ndef");
    editor.move_right();
    editor.move_right();
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 3));
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 3));
    editor.move_home();
    assert_eq!(editor.cursor_position(), (1, 0));
}

#[test]
fn test_cursor_navigation_full_sequence() {
    let mut editor = RichTextEditor::new().content("line 1\nline 2\nline 3");
    editor.move_left();
    assert_eq!(editor.cursor_position(), (0, 0));
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 1));
    editor.move_up();
    assert_eq!(editor.cursor_position(), (0, 1));
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 1));
    editor.move_home();
    assert_eq!(editor.cursor_position(), (1, 0));
    editor.move_end();
    assert_eq!(editor.cursor_position(), (1, 6));
    editor.move_document_start();
    assert_eq!(editor.cursor_position(), (0, 0));
    editor.move_document_end();
    assert_eq!(editor.cursor_position(), (2, 6));
}

#[test]
fn test_move_empty_editor() {
    let mut editor = RichTextEditor::new();
    editor.move_left();
    editor.move_right();
    editor.move_up();
    editor.move_down();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_move_long_line() {
    let mut editor = RichTextEditor::new().content("a".repeat(100));
    editor.move_end();
    assert_eq!(editor.cursor_position().1, 100);
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 0));
}
