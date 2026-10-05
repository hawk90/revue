//! Toolbar actions of the rich text editor
//!
//! Bold, Italic, Code, Heading1, Quote, BulletList, Undo and Redo are
//! covered by tests/rich_text_editor/toolbar.rs and the in-source toolbar
//! tests; these are the remaining actions.

use revue::widget::{BlockType, RichTextEditor, ToolbarAction};

#[test]
fn test_toolbar_action_underline() {
    let mut editor = RichTextEditor::new();
    editor.toolbar_action(ToolbarAction::Underline);
    assert!(editor.current_format().underline);
}

#[test]
fn test_toolbar_action_strikethrough() {
    let mut editor = RichTextEditor::new();
    editor.toolbar_action(ToolbarAction::Strikethrough);
    assert!(editor.current_format().strikethrough);
    assert!(!editor.current_format().bold);
}

#[test]
fn test_toolbar_action_link_opens_dialog() {
    let mut editor = RichTextEditor::new();
    assert!(!editor.is_dialog_open());
    editor.toolbar_action(ToolbarAction::Link);
    assert!(editor.is_dialog_open());
}

#[test]
fn test_toolbar_action_image_opens_dialog() {
    let mut editor = RichTextEditor::new();
    editor.toolbar_action(ToolbarAction::Image);
    assert!(editor.is_dialog_open());
}

#[test]
fn test_toolbar_action_heading2_and_3() {
    let mut editor = RichTextEditor::new().content("Title");
    editor.toolbar_action(ToolbarAction::Heading2);
    assert_eq!(editor.current_block_type(), BlockType::Heading2);
    editor.toolbar_action(ToolbarAction::Heading3);
    assert_eq!(editor.current_block_type(), BlockType::Heading3);
}

#[test]
fn test_toolbar_action_numbered_list() {
    let mut editor = RichTextEditor::new().content("Item");
    editor.toolbar_action(ToolbarAction::NumberedList);
    assert_eq!(editor.current_block_type(), BlockType::NumberedList);
}

#[test]
fn test_toolbar_action_code_block() {
    let mut editor = RichTextEditor::new().content("let x = 1;");
    editor.toolbar_action(ToolbarAction::CodeBlock);
    assert_eq!(editor.current_block_type(), BlockType::CodeBlock);
}

#[test]
fn test_toolbar_action_horizontal_rule() {
    let mut editor = RichTextEditor::new().content("text");
    editor.toolbar_action(ToolbarAction::HorizontalRule);
    assert_eq!(editor.current_block_type(), BlockType::HorizontalRule);
}
