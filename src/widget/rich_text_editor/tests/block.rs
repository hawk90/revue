//! Tests for Block

use super::*;

#[test]
fn test_block_paragraph() {
    let block = Block::paragraph("Hello World");
    assert_eq!(block.block_type, BlockType::Paragraph);
    assert_eq!(block.text(), "Hello World");
}

#[test]
fn test_block_new() {
    let block = Block::new(BlockType::Heading1);
    assert_eq!(block.block_type, BlockType::Heading1);
    assert_eq!(block.text(), "");
}

#[test]
fn test_block_text() {
    let block = Block::paragraph("Test content");
    assert_eq!(block.text(), "Test content");
}

#[test]
fn test_block_set_text() {
    let mut block = Block::paragraph("Old");
    block.set_text("New");
    assert_eq!(block.text(), "New");
}

#[test]
fn test_block_len() {
    let block = Block::paragraph("Hello");
    assert_eq!(block.len(), 5);
}

#[test]
fn test_block_is_empty() {
    let block = Block::paragraph("");
    assert!(block.is_empty());

    let block = Block::paragraph("Not empty");
    assert!(!block.is_empty());
}

#[test]
fn test_block_to_markdown_paragraph() {
    let block = Block::paragraph("Plain text");
    assert_eq!(block.to_markdown(), "Plain text");
}

#[test]
fn test_block_to_markdown_heading() {
    let mut block = Block::new(BlockType::Heading1);
    block.set_text("Title");
    assert_eq!(block.to_markdown(), "# Title");
}

#[test]
fn test_block_to_markdown_quote() {
    let mut block = Block::new(BlockType::Quote);
    block.set_text("Quoted text");
    assert_eq!(block.to_markdown(), "> Quoted text");
}

#[test]
fn test_block_to_markdown_code_block() {
    let mut block = Block::new(BlockType::CodeBlock);
    block.set_text("let x = 1;");
    block.language = Some("rust".to_string());
    assert_eq!(block.to_markdown(), "```rust\nlet x = 1;\n```");
}

#[test]
fn test_block_to_markdown_horizontal_rule() {
    let block = Block::new(BlockType::HorizontalRule);
    assert_eq!(block.to_markdown(), "---");
}
