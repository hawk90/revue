//! Tests for link and image operations and dialogs

use super::*;

#[test]
fn test_insert_link() {
    let mut editor = RichTextEditor::new();
    editor.insert_link("Example", "https://example.com");
    assert_eq!(editor.get_content(), "[Example](https://example.com)");
}

#[test]
fn test_insert_image() {
    let mut editor = RichTextEditor::new();
    editor.insert_image("Alt text", "image.png");
    assert_eq!(editor.get_content(), "![Alt text](image.png)");
}

#[test]
fn test_open_link_dialog() {
    let mut editor = RichTextEditor::new();
    editor.open_link_dialog();
    assert!(editor.is_dialog_open());
}

#[test]
fn test_open_image_dialog() {
    let mut editor = RichTextEditor::new();
    editor.open_image_dialog();
    assert!(editor.is_dialog_open());
}

#[test]
fn test_close_dialog() {
    let mut editor = RichTextEditor::new();
    editor.open_link_dialog();
    editor.close_dialog();
    assert!(!editor.is_dialog_open());
}
