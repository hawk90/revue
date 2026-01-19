//! Block Type tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

#[test]
fn test_block_type_default() {
    let editor = RichTextEditor::new().content("text");
    assert_eq!(editor.current_block_type(), BlockType::Paragraph);
}

#[test]
fn test_set_block_type() {
    let mut editor = RichTextEditor::new().content("text");
    editor.set_block_type(BlockType::Heading1);
    assert_eq!(editor.current_block_type(), BlockType::Heading1);
}

#[test]
fn test_block_types() {
    let mut editor = RichTextEditor::new().content("text");

    editor.set_block_type(BlockType::Quote);
    assert_eq!(editor.current_block_type(), BlockType::Quote);

    editor.set_block_type(BlockType::BulletList);
    assert_eq!(editor.current_block_type(), BlockType::BulletList);

    editor.set_block_type(BlockType::CodeBlock);
    assert_eq!(editor.current_block_type(), BlockType::CodeBlock);
}
