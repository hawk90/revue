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

// Found by tests/event_sequences.rs: the shrunk sequence was
// `[2x1] <link dialog>` - an open dialog in an area too narrow for it.
#[test]
fn test_dialog_in_a_tiny_area() {
    for (w, h) in [(0, 0), (1, 1), (2, 1), (4, 3), (5, 2), (6, 8)] {
        let mut editor = RichTextEditor::new().content("text");
        editor.open_link_dialog();
        let mut buffer = Buffer::new(w.max(1), h.max(1));
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
        editor.render(&mut ctx);
        editor.close_dialog();
        editor.open_image_dialog();
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
        editor.render(&mut ctx);
    }
}
