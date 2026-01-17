//! Toolbar Action tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

#[test]
fn test_toolbar_action_formatting() {
    let mut editor = RichTextEditor::new();

    editor.toolbar_action(ToolbarAction::Bold);
    assert!(editor.current_format().bold);

    editor.toolbar_action(ToolbarAction::Italic);
    assert!(editor.current_format().italic);

    editor.toolbar_action(ToolbarAction::Code);
    assert!(editor.current_format().code);
}

#[test]
fn test_toolbar_action_block_type() {
    let mut editor = RichTextEditor::new().content("text");

    editor.toolbar_action(ToolbarAction::Heading1);
    assert_eq!(editor.current_block_type(), BlockType::Heading1);

    editor.toolbar_action(ToolbarAction::Quote);
    assert_eq!(editor.current_block_type(), BlockType::Quote);
}

#[test]
fn test_toolbar_action_undo_redo() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.insert_char('!');

    editor.toolbar_action(ToolbarAction::Undo);
    assert_eq!(editor.get_content(), "hello");

    editor.toolbar_action(ToolbarAction::Redo);
    assert_eq!(editor.get_content(), "hello!");
}
