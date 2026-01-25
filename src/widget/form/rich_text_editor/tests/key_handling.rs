//! Tests for key handling

use super::*;

#[test]
fn test_handle_key_char() {
    let mut editor = RichTextEditor::new();
    assert!(editor.handle_key(&Key::Char('H')));
    assert_eq!(editor.get_content(), "H");
}

#[test]
fn test_handle_key_enter() {
    let mut editor = RichTextEditor::new().content("Hello");
    editor.set_cursor(0, 5);
    editor.handle_key(&Key::Enter);
    assert_eq!(editor.block_count(), 2);
}

#[test]
fn test_handle_key_backspace() {
    let mut editor = RichTextEditor::new().content("Hi");
    editor.set_cursor(0, 2);
    editor.handle_key(&Key::Backspace);
    assert_eq!(editor.get_content(), "H");
}

#[test]
fn test_handle_key_delete() {
    let mut editor = RichTextEditor::new().content("Hi");
    editor.set_cursor(0, 0);
    editor.handle_key(&Key::Delete);
    assert_eq!(editor.get_content(), "i");
}

#[test]
fn test_handle_key_tab() {
    let mut editor = RichTextEditor::new();
    editor.handle_key(&Key::Tab);
    assert_eq!(editor.get_content(), "    ");
}

#[test]
fn test_handle_key_left() {
    let mut editor = RichTextEditor::new().content("Hi");
    editor.set_cursor(0, 2);
    editor.handle_key(&Key::Left);
    assert_eq!(editor.cursor_position(), (0, 1));
}

#[test]
fn test_handle_key_right() {
    let mut editor = RichTextEditor::new().content("Hi");
    editor.handle_key(&Key::Right);
    assert_eq!(editor.cursor_position(), (0, 1));
}

#[test]
fn test_handle_key_up() {
    let mut editor = RichTextEditor::new().content("A\nB");
    editor.set_cursor(1, 0);
    editor.handle_key(&Key::Up);
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_handle_key_down() {
    let mut editor = RichTextEditor::new().content("A\nB");
    editor.handle_key(&Key::Down);
    assert_eq!(editor.cursor_position(), (1, 0));
}

#[test]
fn test_handle_key_home() {
    let mut editor = RichTextEditor::new().content("Hello");
    editor.set_cursor(0, 3);
    editor.handle_key(&Key::Home);
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_handle_key_end() {
    let mut editor = RichTextEditor::new().content("Hello");
    editor.handle_key(&Key::End);
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_handle_key_unknown() {
    let mut editor = RichTextEditor::new();
    assert!(!editor.handle_key(&Key::F(1)));
}
