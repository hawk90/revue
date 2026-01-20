//! Block and Span Helper tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

#[test]
fn test_block_paragraph() {
    let block = Block::paragraph("hello");
    assert_eq!(block.block_type, BlockType::Paragraph);
    assert_eq!(block.text(), "hello");
    assert_eq!(block.len(), 5);
    assert!(!block.is_empty());
}

#[test]
fn test_block_new() {
    let mut block = Block::new(BlockType::Heading1);
    assert_eq!(block.block_type, BlockType::Heading1);
    assert!(block.is_empty());

    block.set_text("Title");
    assert_eq!(block.text(), "Title");
}

#[test]
fn test_formatted_span() {
    let span = FormattedSpan::new("text").with_format(TextFormat {
        bold: true,
        italic: false,
        underline: false,
        strikethrough: false,
        code: false,
    });
    assert_eq!(span.text, "text");
    assert!(span.format.bold);
}

#[test]
fn test_block_type_markdown_prefix() {
    assert_eq!(BlockType::Heading1.markdown_prefix(), "# ");
    assert_eq!(BlockType::Heading2.markdown_prefix(), "## ");
    assert_eq!(BlockType::Quote.markdown_prefix(), "> ");
    assert_eq!(BlockType::BulletList.markdown_prefix(), "- ");
    assert_eq!(BlockType::Paragraph.markdown_prefix(), "");
    assert_eq!(BlockType::NumberedList.markdown_prefix(), "1. ");
    assert_eq!(BlockType::CodeBlock.markdown_prefix(), "```\n");
    assert_eq!(BlockType::HorizontalRule.markdown_prefix(), "---");
}

#[test]
fn test_formatted_span_all_formats() {
    // Test span with all format combinations
    let span = FormattedSpan::new("text").with_format(TextFormat {
        bold: true,
        italic: true,
        underline: false,
        strikethrough: true,
        code: false,
    });
    assert_eq!(span.text, "text");
    assert!(span.format.bold);
    assert!(span.format.italic);
    assert!(span.format.strikethrough);
}

#[test]
fn test_block_to_markdown_with_code_language() {
    let mut block = Block::new(BlockType::CodeBlock);
    block.set_text("println!(\"hello\");");
    block.language = Some("rust".to_string());

    assert_eq!(block.to_markdown(), "```rust\nprintln!(\"hello\");\n```");
}

#[test]
fn test_block_to_markdown_with_multiple_spans() {
    let mut block = Block::new(BlockType::Paragraph);
    block.spans = vec![
        FormattedSpan::new("bold").with_format(TextFormat {
            bold: true,
            italic: false,
            underline: false,
            strikethrough: false,
            code: false,
        }),
        FormattedSpan::new(" and "),
        FormattedSpan::new("code").with_format(TextFormat {
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            code: true,
        }),
    ];

    let markdown = block.to_markdown();
    assert!(markdown.contains("**bold**"));
    assert!(markdown.contains("`code`"));
}
