//! Tests for markdown parsing

use super::*;

#[test]
fn test_from_markdown_headings() {
    let editor = RichTextEditor::new().from_markdown("# Heading 1\n## Heading 2");
    assert_eq!(editor.block_count(), 2);
}

#[test]
fn test_from_markdown_quote() {
    let editor = RichTextEditor::new().from_markdown("> This is a quote");
    assert_eq!(editor.blocks[0].block_type, BlockType::Quote);
}

#[test]
fn test_from_markdown_bullet_list() {
    let editor = RichTextEditor::new().from_markdown("- Item 1\n- Item 2");
    assert_eq!(editor.blocks[0].block_type, BlockType::BulletList);
    assert_eq!(editor.blocks[1].block_type, BlockType::BulletList);
}

#[test]
fn test_from_markdown_bullet_list_asterisk() {
    let editor = RichTextEditor::new().from_markdown("* Item");
    assert_eq!(editor.blocks[0].block_type, BlockType::BulletList);
}

#[test]
fn test_from_markdown_numbered_list() {
    let editor = RichTextEditor::new().from_markdown("1. First\n2. Second");
    assert_eq!(editor.blocks[0].block_type, BlockType::NumberedList);
}

#[test]
fn test_from_markdown_horizontal_rule() {
    let editor = RichTextEditor::new().from_markdown("---");
    assert_eq!(editor.blocks[0].block_type, BlockType::HorizontalRule);
}

#[test]
fn test_from_markdown_code_block() {
    let editor = RichTextEditor::new().from_markdown("```rust\nlet x = 1;\n```");
    assert_eq!(editor.blocks[0].block_type, BlockType::CodeBlock);
    assert_eq!(editor.blocks[0].language, Some("rust".to_string()));
}

#[test]
fn test_from_markdown_empty() {
    let editor = RichTextEditor::new().from_markdown("");
    assert_eq!(editor.block_count(), 1);
}

#[test]
fn test_to_markdown() {
    let mut editor = RichTextEditor::new().content("Title");
    editor.set_block_type(BlockType::Heading1);
    assert_eq!(editor.to_markdown(), "# Title");
}
