//! Tests for undo/redo functionality

use super::*;

#[test]
fn test_undo_insert_char() {
    let mut editor = RichTextEditor::new();
    editor.insert_char('H');
    editor.insert_char('i');
    assert_eq!(editor.get_content(), "Hi");
    editor.undo();
    assert_eq!(editor.get_content(), "H");
}

#[test]
fn test_redo_insert_char() {
    let mut editor = RichTextEditor::new();
    editor.insert_char('H');
    editor.undo();
    assert_eq!(editor.get_content(), "");
    editor.redo();
    assert_eq!(editor.get_content(), "H");
}

#[test]
fn test_undo_delete_char() {
    let mut editor = RichTextEditor::new().content("Hi");
    editor.set_cursor(0, 2);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "H");
    editor.undo();
    assert_eq!(editor.get_content(), "Hi");
}

#[test]
fn test_undo_block_type_change() {
    let mut editor = RichTextEditor::new().content("Title");
    editor.set_block_type(BlockType::Heading1);
    assert_eq!(editor.current_block_type(), BlockType::Heading1);
    editor.undo();
    assert_eq!(editor.current_block_type(), BlockType::Paragraph);
}

#[test]
fn test_insert_clears_redo_stack() {
    let mut editor = RichTextEditor::new();
    editor.insert_char('A');
    editor.undo();
    editor.insert_char('B');
    editor.redo(); // Should do nothing
    assert_eq!(editor.get_content(), "B");
}
