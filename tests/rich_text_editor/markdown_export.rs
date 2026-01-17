//! Markdown Export tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

#[test]
fn test_to_markdown_paragraph() {
    let editor = RichTextEditor::new().content("hello world");
    assert_eq!(editor.to_markdown(), "hello world");
}

#[test]
fn test_to_markdown_heading() {
    let mut editor = RichTextEditor::new().content("Title");
    editor.set_block_type(BlockType::Heading1);
    assert_eq!(editor.to_markdown(), "# Title");
}

#[test]
fn test_to_markdown_quote() {
    let mut editor = RichTextEditor::new().content("quoted text");
    editor.set_block_type(BlockType::Quote);
    assert_eq!(editor.to_markdown(), "> quoted text");
}

#[test]
fn test_to_markdown_bullet_list() {
    let mut editor = RichTextEditor::new().content("item");
    editor.set_block_type(BlockType::BulletList);
    assert_eq!(editor.to_markdown(), "- item");
}
