//! Tests for cursor navigation

use super::*;

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
