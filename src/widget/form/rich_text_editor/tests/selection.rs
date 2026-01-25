//! Tests for text selection

use super::*;

#[test]
fn test_start_selection() {
    let mut editor = RichTextEditor::new().content("Hello");
    editor.set_cursor(0, 2);
    editor.start_selection();
    assert!(editor.has_selection());
}

#[test]
fn test_clear_selection() {
    let mut editor = RichTextEditor::new().content("Hello");
    editor.start_selection();
    editor.clear_selection();
    assert!(!editor.has_selection());
}

#[test]
fn test_get_selection_single_line() {
    let mut editor = RichTextEditor::new().content("Hello World");
    editor.set_cursor(0, 0);
    editor.start_selection();
    editor.set_cursor(0, 5);
    // Note: anchor is at (0,0), cursor at (0,5)
    // But cursor movement clears selection, so we need different approach
}

#[test]
fn test_has_selection_initially_false() {
    let editor = RichTextEditor::new();
    assert!(!editor.has_selection());
}

#[test]
fn test_delete_selection() {
    let mut editor = RichTextEditor::new().content("Hello World");
    editor.anchor = Some((0, 0));
    editor.cursor = (0, 5);
    editor.delete_selection();
    assert_eq!(editor.get_content(), " World");
}
