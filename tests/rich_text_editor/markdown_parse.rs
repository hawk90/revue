//! Markdown Parsing tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

#[test]
fn test_from_markdown_heading() {
    let editor = RichTextEditor::new().from_markdown("# Hello");
    assert_eq!(editor.current_block_type(), BlockType::Heading1);
    assert_eq!(editor.get_content(), "Hello");
}

#[test]
fn test_from_markdown_heading2() {
    let editor = RichTextEditor::new().from_markdown("## Section");
    assert_eq!(editor.current_block_type(), BlockType::Heading2);
    assert_eq!(editor.get_content(), "Section");
}

#[test]
fn test_from_markdown_quote() {
    let editor = RichTextEditor::new().from_markdown("> Quote");
    assert_eq!(editor.current_block_type(), BlockType::Quote);
    assert_eq!(editor.get_content(), "Quote");
}

#[test]
fn test_from_markdown_bullet_list() {
    let editor = RichTextEditor::new().from_markdown("- Item");
    assert_eq!(editor.current_block_type(), BlockType::BulletList);
    assert_eq!(editor.get_content(), "Item");
}

#[test]
fn test_from_markdown_horizontal_rule() {
    let editor = RichTextEditor::new().from_markdown("---");
    assert_eq!(editor.current_block_type(), BlockType::HorizontalRule);
}
