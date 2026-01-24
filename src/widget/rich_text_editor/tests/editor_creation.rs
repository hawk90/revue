//! Tests for RichTextEditor creation and builder methods

use super::*;

#[test]
fn test_rich_text_editor_new() {
    let editor = RichTextEditor::new();
    assert_eq!(editor.block_count(), 1);
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_rich_text_editor_default() {
    let editor = RichTextEditor::default();
    assert_eq!(editor.block_count(), 1);
}

#[test]
fn test_rich_text_editor_helper() {
    let editor = rich_text_editor();
    assert_eq!(editor.block_count(), 1);
}

#[test]
fn test_rich_text_editor_content() {
    let editor = RichTextEditor::new().content("Line 1\nLine 2\nLine 3");
    assert_eq!(editor.block_count(), 3);
    assert_eq!(editor.get_content(), "Line 1\nLine 2\nLine 3");
}

#[test]
fn test_rich_text_editor_content_empty() {
    let editor = RichTextEditor::new().content("");
    assert_eq!(editor.block_count(), 1);
}

#[test]
fn test_rich_text_editor_set_content() {
    let mut editor = RichTextEditor::new();
    editor.set_content("New content\nSecond line");
    assert_eq!(editor.block_count(), 2);
    assert_eq!(editor.get_content(), "New content\nSecond line");
}

#[test]
fn test_rich_text_editor_view_mode() {
    let editor = RichTextEditor::new().view_mode(EditorViewMode::Split);
    assert_eq!(editor.view_mode, EditorViewMode::Split);
}

#[test]
fn test_rich_text_editor_toolbar() {
    let editor = RichTextEditor::new().toolbar(false);
    assert!(!editor.show_toolbar);
}

#[test]
fn test_rich_text_editor_focused() {
    let editor = RichTextEditor::new().focused(false);
    assert!(!editor.focused);
}
