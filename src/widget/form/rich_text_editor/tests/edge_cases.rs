//! Tests for edge cases

use super::*;

#[test]
fn test_empty_editor_operations() {
    let mut editor = RichTextEditor::new();
    editor.move_left();
    editor.move_up();
    editor.delete_char_before();
    editor.delete_char_at();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_undo_on_empty_stack() {
    let mut editor = RichTextEditor::new();
    editor.undo(); // Should not panic
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_redo_on_empty_stack() {
    let mut editor = RichTextEditor::new();
    editor.redo(); // Should not panic
}

#[test]
fn test_cursor_position_after_operations() {
    let mut editor = RichTextEditor::new().content("Hello");
    editor.move_end();
    assert_eq!(editor.cursor_position(), (0, 5));
    editor.insert_char('!');
    assert_eq!(editor.cursor_position(), (0, 6));
}

#[test]
fn test_multi_block_navigation() {
    let mut editor = RichTextEditor::new().content("Short\nLonger line\nX");
    editor.set_cursor(1, 10);
    editor.move_down();
    // Cursor should clamp to shorter line length
    assert_eq!(editor.cursor_position(), (2, 1));
}

#[test]
fn test_color_setters() {
    let editor = RichTextEditor::new().bg(Color::RED).fg(Color::WHITE);
    assert_eq!(editor.bg, Some(Color::RED));
    assert_eq!(editor.fg, Some(Color::WHITE));
}
