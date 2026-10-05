//! Tests for RichTextEditor construction

use revue::widget::RichTextEditor;

#[test]
fn test_rich_text_editor_from_markdown_blank_line_is_a_block() {
    let editor = RichTextEditor::new().from_markdown("# Heading\n\nParagraph");
    // Creates 3 blocks: heading, empty paragraph, actual paragraph
    assert_eq!(editor.block_count(), 3);
    assert_eq!(editor.get_content(), "Heading\n\nParagraph");
}
