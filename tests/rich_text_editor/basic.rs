//! Basic Creation and Content tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

#[test]
fn test_rich_text_editor_new() {
    let editor = RichTextEditor::new();
    assert_eq!(editor.block_count(), 1);
    assert_eq!(editor.cursor_position(), (0, 0));
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_rich_text_editor_constructor() {
    let editor = rich_text_editor().content("hello");
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_rich_text_editor_content() {
    let editor = RichTextEditor::new().content("line1\nline2\nline3");
    assert_eq!(editor.block_count(), 3);
    assert_eq!(editor.get_content(), "line1\nline2\nline3");
}

#[test]
fn test_rich_text_editor_set_content() {
    let mut editor = RichTextEditor::new();
    editor.set_content("new content");
    assert_eq!(editor.get_content(), "new content");
    assert_eq!(editor.cursor_position(), (0, 0));
}
