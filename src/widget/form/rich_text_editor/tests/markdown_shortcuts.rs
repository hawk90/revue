//! Tests for markdown shortcuts

use super::*;

#[test]
fn test_markdown_shortcut_heading() {
    let mut editor = RichTextEditor::new();
    editor.insert_str("# ");
    editor.process_markdown_shortcuts();
    assert_eq!(editor.current_block_type(), BlockType::Heading1);
}

#[test]
fn test_markdown_shortcut_quote() {
    let mut editor = RichTextEditor::new();
    editor.insert_str("> ");
    editor.process_markdown_shortcuts();
    assert_eq!(editor.current_block_type(), BlockType::Quote);
}

#[test]
fn test_markdown_shortcut_bullet() {
    let mut editor = RichTextEditor::new();
    editor.insert_str("- ");
    editor.process_markdown_shortcuts();
    assert_eq!(editor.current_block_type(), BlockType::BulletList);
}

#[test]
fn test_markdown_shortcut_numbered() {
    let mut editor = RichTextEditor::new();
    editor.insert_str("1. ");
    editor.process_markdown_shortcuts();
    assert_eq!(editor.current_block_type(), BlockType::NumberedList);
}
