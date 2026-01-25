//! Tests for BlockType

use super::*;

#[test]
fn test_block_type_default() {
    assert_eq!(BlockType::default(), BlockType::Paragraph);
}

#[test]
fn test_block_type_markdown_prefix_paragraph() {
    assert_eq!(BlockType::Paragraph.markdown_prefix(), "");
}

#[test]
fn test_block_type_markdown_prefix_headings() {
    assert_eq!(BlockType::Heading1.markdown_prefix(), "# ");
    assert_eq!(BlockType::Heading2.markdown_prefix(), "## ");
    assert_eq!(BlockType::Heading3.markdown_prefix(), "### ");
    assert_eq!(BlockType::Heading4.markdown_prefix(), "#### ");
    assert_eq!(BlockType::Heading5.markdown_prefix(), "##### ");
    assert_eq!(BlockType::Heading6.markdown_prefix(), "###### ");
}

#[test]
fn test_block_type_markdown_prefix_quote() {
    assert_eq!(BlockType::Quote.markdown_prefix(), "> ");
}

#[test]
fn test_block_type_markdown_prefix_lists() {
    assert_eq!(BlockType::BulletList.markdown_prefix(), "- ");
    assert_eq!(BlockType::NumberedList.markdown_prefix(), "1. ");
}

#[test]
fn test_block_type_markdown_prefix_code_block() {
    assert_eq!(BlockType::CodeBlock.markdown_prefix(), "```\n");
}

#[test]
fn test_block_type_markdown_prefix_horizontal_rule() {
    assert_eq!(BlockType::HorizontalRule.markdown_prefix(), "---");
}
