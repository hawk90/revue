//! Tests for toolbar actions

use super::*;

#[test]
fn test_toolbar_action_bold() {
    let mut editor = RichTextEditor::new();
    editor.toolbar_action(ToolbarAction::Bold);
    assert!(editor.current_format().bold);
}

#[test]
fn test_toolbar_action_heading() {
    let mut editor = RichTextEditor::new().content("Title");
    editor.toolbar_action(ToolbarAction::Heading1);
    assert_eq!(editor.current_block_type(), BlockType::Heading1);
}

#[test]
fn test_toolbar_action_quote() {
    let mut editor = RichTextEditor::new().content("Quote");
    editor.toolbar_action(ToolbarAction::Quote);
    assert_eq!(editor.current_block_type(), BlockType::Quote);
}

#[test]
fn test_toolbar_action_bullet_list() {
    let mut editor = RichTextEditor::new().content("Item");
    editor.toolbar_action(ToolbarAction::BulletList);
    assert_eq!(editor.current_block_type(), BlockType::BulletList);
}

#[test]
fn test_toolbar_action_undo() {
    let mut editor = RichTextEditor::new();
    editor.insert_char('A');
    editor.toolbar_action(ToolbarAction::Undo);
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_toolbar_action_redo() {
    let mut editor = RichTextEditor::new();
    editor.insert_char('A');
    editor.undo();
    editor.toolbar_action(ToolbarAction::Redo);
    assert_eq!(editor.get_content(), "A");
}
