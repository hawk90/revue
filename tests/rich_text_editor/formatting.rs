//! Formatting tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

#[test]
fn test_text_format_default() {
    let format = TextFormat::default();
    assert!(!format.bold);
    assert!(!format.italic);
    assert!(!format.underline);
    assert!(!format.strikethrough);
    assert!(!format.code);
}

#[test]
fn test_text_format_toggle() {
    let format = TextFormat::new()
        .toggle_bold()
        .toggle_italic()
        .toggle_code();
    assert!(format.bold);
    assert!(format.italic);
    assert!(!format.underline);
    assert!(format.code);
}

#[test]
fn test_toggle_bold() {
    let mut editor = RichTextEditor::new();
    assert!(!editor.current_format().bold);
    editor.toggle_bold();
    assert!(editor.current_format().bold);
    editor.toggle_bold();
    assert!(!editor.current_format().bold);
}

#[test]
fn test_toggle_italic() {
    let mut editor = RichTextEditor::new();
    editor.toggle_italic();
    assert!(editor.current_format().italic);
}
