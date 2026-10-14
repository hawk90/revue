//! Tests for text formatting

use super::*;

#[test]
fn test_toggle_underline() {
    let mut editor = RichTextEditor::new();
    editor.toggle_underline();
    assert!(editor.current_format().underline);
}

#[test]
fn test_toggle_strikethrough() {
    let mut editor = RichTextEditor::new();
    editor.toggle_strikethrough();
    assert!(editor.current_format().strikethrough);
}

#[test]
fn test_toggle_code() {
    let mut editor = RichTextEditor::new();
    editor.toggle_code();
    assert!(editor.current_format().code);
}

#[test]
fn test_set_block_type() {
    let mut editor = RichTextEditor::new().content("Title");
    editor.set_block_type(BlockType::Heading1);
    assert_eq!(editor.current_block_type(), BlockType::Heading1);
}

#[test]
fn test_current_block_type() {
    let editor = RichTextEditor::new();
    assert_eq!(editor.current_block_type(), BlockType::Paragraph);
}
