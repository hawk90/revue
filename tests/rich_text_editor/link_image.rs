//! Link and Image tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

#[test]
fn test_insert_link() {
    let mut editor = RichTextEditor::new();
    editor.insert_link("Google", "https://google.com");
    assert_eq!(editor.get_content(), "[Google](https://google.com)");
}

#[test]
fn test_insert_image() {
    let mut editor = RichTextEditor::new();
    editor.insert_image("Logo", "logo.png");
    assert_eq!(editor.get_content(), "![Logo](logo.png)");
}

#[test]
fn test_dialog_open_close() {
    let mut editor = RichTextEditor::new();
    assert!(!editor.is_dialog_open());

    editor.open_link_dialog();
    assert!(editor.is_dialog_open());

    editor.close_dialog();
    assert!(!editor.is_dialog_open());

    editor.open_image_dialog();
    assert!(editor.is_dialog_open());

    editor.close_dialog();
    assert!(!editor.is_dialog_open());
}
