//! Tests for text editing operations

use super::*;

#[test]
fn test_insert_char() {
    let mut editor = RichTextEditor::new();
    editor.insert_char('H');
    editor.insert_char('i');
    assert_eq!(editor.get_content(), "Hi");
}

#[test]
fn test_insert_char_at_position() {
    let mut editor = RichTextEditor::new().content("Hllo");
    editor.set_cursor(0, 1);
    editor.insert_char('e');
    assert_eq!(editor.get_content(), "Hello");
}

#[test]
fn test_insert_str() {
    let mut editor = RichTextEditor::new();
    editor.insert_str("Hello World");
    assert_eq!(editor.get_content(), "Hello World");
}

#[test]
fn test_insert_newline() {
    let mut editor = RichTextEditor::new().content("HelloWorld");
    editor.set_cursor(0, 5);
    editor.insert_char('\n');
    assert_eq!(editor.block_count(), 2);
    assert_eq!(editor.get_content(), "Hello\nWorld");
}

#[test]
fn test_delete_char_before() {
    let mut editor = RichTextEditor::new().content("Hello");
    editor.set_cursor(0, 5);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "Hell");
}

#[test]
fn test_delete_char_before_at_start() {
    let mut editor = RichTextEditor::new().content("Hello");
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "Hello");
}

#[test]
fn test_delete_char_before_merges_lines() {
    let mut editor = RichTextEditor::new().content("Hello\nWorld");
    editor.set_cursor(1, 0);
    editor.delete_char_before();
    assert_eq!(editor.block_count(), 1);
    assert_eq!(editor.get_content(), "HelloWorld");
}

#[test]
fn test_delete_char_at() {
    let mut editor = RichTextEditor::new().content("Hello");
    editor.set_cursor(0, 0);
    editor.delete_char_at();
    assert_eq!(editor.get_content(), "ello");
}

#[test]
fn test_delete_char_at_end() {
    let mut editor = RichTextEditor::new().content("Hello\nWorld");
    editor.set_cursor(0, 5);
    editor.delete_char_at();
    assert_eq!(editor.get_content(), "HelloWorld");
}

#[test]
fn test_delete_block() {
    let mut editor = RichTextEditor::new().content("Line 1\nLine 2\nLine 3");
    editor.set_cursor(1, 0);
    editor.delete_block();
    assert_eq!(editor.block_count(), 2);
}

#[test]
fn test_delete_block_single() {
    let mut editor = RichTextEditor::new().content("Only line");
    editor.delete_block();
    assert_eq!(editor.block_count(), 1);
    assert_eq!(editor.get_content(), "");
}
