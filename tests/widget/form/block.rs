//! Tests for rich_text_editor block module

use revue::widget::{Block, BlockType, FormattedSpan, TextFormat};

// =========================================================================
// BlockType enum tests
// =========================================================================

#[test]
fn test_block_type_variants_are_distinct() {
    let variants = [
        BlockType::Paragraph,
        BlockType::Heading1,
        BlockType::Heading2,
        BlockType::Heading3,
        BlockType::Heading4,
        BlockType::Heading5,
        BlockType::Heading6,
        BlockType::Quote,
        BlockType::CodeBlock,
        BlockType::BulletList,
        BlockType::NumberedList,
        BlockType::HorizontalRule,
    ];
    for (i, a) in variants.iter().enumerate() {
        for (j, b) in variants.iter().enumerate() {
            assert_eq!(i == j, a == b, "{a:?} vs {b:?}");
        }
    }
}

// =========================================================================
// FormattedSpan tests
// =========================================================================

#[test]
fn test_formatted_span_empty() {
    let span = FormattedSpan::new("");
    assert_eq!(span.text, "");
}

// =========================================================================
// Block::paragraph tests
// =========================================================================

#[test]
fn test_block_paragraph_empty() {
    let block = Block::paragraph("");
    assert_eq!(block.block_type, BlockType::Paragraph);
    assert_eq!(block.text(), "");
    assert!(block.is_empty());
}

#[test]
fn test_block_paragraph_with_string() {
    let s = String::from("owned string");
    let block = Block::paragraph(s);
    assert_eq!(block.text(), "owned string");
}

// =========================================================================
// Block::new tests
// =========================================================================

#[test]
fn test_block_new_keeps_type_and_starts_empty() {
    for block_type in [
        BlockType::Quote,
        BlockType::CodeBlock,
        BlockType::BulletList,
    ] {
        let block = Block::new(block_type);
        assert_eq!(block.block_type, block_type);
        assert!(block.is_empty());
        assert_eq!(block.text(), "");
    }
}

// =========================================================================
// Block::text tests
// =========================================================================

#[test]
fn test_block_text_multiple_spans() {
    let mut block = Block::paragraph("");
    block.spans = vec![
        FormattedSpan::new("Hello"),
        FormattedSpan::new(" "),
        FormattedSpan::new("world"),
    ];
    assert_eq!(block.text(), "Hello world");
}

// =========================================================================
// Block::set_text tests
// =========================================================================

#[test]
fn test_block_set_text_to_empty() {
    let mut block = Block::paragraph("something");
    block.set_text("");
    assert_eq!(block.text(), "");
    assert!(block.is_empty());
}

// =========================================================================
// Block::len tests
// =========================================================================

#[test]
fn test_block_len_empty() {
    let block = Block::paragraph("");
    assert_eq!(block.len(), 0);
}

#[test]
fn test_block_len_multiple_spans() {
    let mut block = Block::paragraph("");
    block.spans = vec![
        FormattedSpan::new("Hi"),
        FormattedSpan::new(" "),
        FormattedSpan::new("there"),
    ];
    assert_eq!(block.len(), 8); // "Hi" + " " + "there" = 2 + 1 + 5 = 8
}

// =========================================================================
// Block::is_empty tests
// =========================================================================

#[test]
fn test_block_is_empty_multiple_empty_spans() {
    let mut block = Block::paragraph("");
    block.spans = vec![FormattedSpan::new(""), FormattedSpan::new("")];
    assert!(block.is_empty());
}

// =========================================================================
// Block::to_markdown tests
// =========================================================================

#[test]
fn test_block_to_markdown_heading2() {
    let mut block = Block::new(BlockType::Heading2);
    block.set_text("Subtitle");
    assert_eq!(block.to_markdown(), "## Subtitle");
}

#[test]
fn test_block_to_markdown_bullet_list() {
    let mut block = Block::new(BlockType::BulletList);
    block.set_text("Item");
    assert_eq!(block.to_markdown(), "- Item");
}

#[test]
fn test_block_to_markdown_numbered_list() {
    let mut block = Block::new(BlockType::NumberedList);
    block.set_text("Item");
    assert_eq!(block.to_markdown(), "1. Item");
}

#[test]
fn test_block_to_markdown_code_block_no_lang() {
    let mut block = Block::new(BlockType::CodeBlock);
    block.set_text("code here");
    assert_eq!(block.to_markdown(), "```\ncode here\n```");
}

#[test]
fn test_block_to_markdown_with_bold() {
    let mut block = Block::paragraph("");
    block.spans = vec![FormattedSpan::new("bold").with_format(TextFormat {
        bold: true,
        ..Default::default()
    })];
    assert_eq!(block.to_markdown(), "**bold**");
}

#[test]
fn test_block_to_markdown_with_italic() {
    let mut block = Block::paragraph("");
    block.spans = vec![FormattedSpan::new("italic").with_format(TextFormat {
        italic: true,
        ..Default::default()
    })];
    assert_eq!(block.to_markdown(), "*italic*");
}

#[test]
fn test_block_to_markdown_with_strikethrough() {
    let mut block = Block::paragraph("");
    block.spans = vec![FormattedSpan::new("deleted").with_format(TextFormat {
        strikethrough: true,
        ..Default::default()
    })];
    assert_eq!(block.to_markdown(), "~~deleted~~");
}

#[test]
fn test_block_to_markdown_with_code() {
    let mut block = Block::paragraph("");
    block.spans = vec![FormattedSpan::new("code").with_format(TextFormat {
        code: true,
        ..Default::default()
    })];
    assert_eq!(block.to_markdown(), "`code`");
}

#[test]
fn test_block_to_markdown_with_multiple_formats() {
    let mut block = Block::paragraph("");
    block.spans = vec![FormattedSpan::new("text").with_format(TextFormat {
        bold: true,
        italic: true,
        ..Default::default()
    })];
    // Bold is applied first (**text**), then italic wraps it.
    assert_eq!(block.to_markdown(), "***text***");
}
