//! View Mode tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

#[test]
fn test_view_mode_default() {
    let editor = RichTextEditor::new();
    // Default should be editor mode
    let _md = editor.to_markdown(); // Just verify it works
}

#[test]
fn test_view_mode_builder() {
    let _editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Split)
        .toolbar(true)
        .focused(true);
}
