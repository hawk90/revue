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
    // set_cursor keeps the anchor; the arrow keys would clear it
    assert_eq!(editor.get_selection().as_deref(), Some("Hello"));
    editor.set_cursor(0, 11);
    assert_eq!(editor.get_selection().as_deref(), Some("Hello World"));
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

#[test]
fn test_selection_left_past_the_text_stays_inside_it() {
    // Every edit clears the anchor; one left past the blocks or a block's
    // end is read as the text's end instead of indexing past it (#1074)
    let mut editor = RichTextEditor::new().content("ab\ncd");
    editor.anchor = Some((5, 9));
    assert_eq!(editor.get_selection().as_deref(), Some("ab\ncd"));
    editor.anchor = Some((0, 9));
    editor.set_cursor(0, 1);
    assert_eq!(editor.get_selection().as_deref(), Some("b"));
    editor.delete_selection();
    assert_eq!(editor.get_content(), "a\ncd");
}
