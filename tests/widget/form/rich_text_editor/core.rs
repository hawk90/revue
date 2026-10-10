//! RichTextEditor construction and content tests

use revue::widget::rich_text_editor;
use revue::widget::RichTextEditor;

#[test]
fn test_rich_text_editor_from_markdown_blank_line_is_a_block() {
    let editor = RichTextEditor::new().from_markdown("# Heading\n\nParagraph");
    // Creates 3 blocks: heading, empty paragraph, actual paragraph
    assert_eq!(editor.block_count(), 3);
    assert_eq!(editor.get_content(), "Heading\n\nParagraph");
}

#[test]
fn test_rich_text_editor_new() {
    let editor = RichTextEditor::new();
    assert_eq!(editor.block_count(), 1);
    assert_eq!(editor.cursor_position(), (0, 0));
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_rich_text_editor_constructor() {
    let editor = rich_text_editor().content("hello");
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_rich_text_editor_content() {
    let editor = RichTextEditor::new().content("line1\nline2\nline3");
    assert_eq!(editor.block_count(), 3);
    assert_eq!(editor.get_content(), "line1\nline2\nline3");
}

#[test]
fn test_rich_text_editor_set_content() {
    let mut editor = RichTextEditor::new();
    editor.set_content("new content");
    assert_eq!(editor.get_content(), "new content");
    assert_eq!(editor.cursor_position(), (0, 0));
}
