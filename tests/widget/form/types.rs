//! Tests for rich_text_editor types module

use revue::widget::form::rich_text_editor::types::{EditorViewMode, ToolbarAction};

// =========================================================================
// ToolbarAction enum tests
// =========================================================================

#[test]
fn test_toolbar_action_clone() {
    let action = ToolbarAction::Bold;
    assert_eq!(action, action.clone());
}

#[test]
fn test_toolbar_action_copy() {
    let action1 = ToolbarAction::Italic;
    let action2 = action1;
    assert_eq!(action1, ToolbarAction::Italic);
    assert_eq!(action2, ToolbarAction::Italic);
}

#[test]
fn test_toolbar_action_equality() {
    assert_eq!(ToolbarAction::Bold, ToolbarAction::Bold);
    assert_eq!(ToolbarAction::Link, ToolbarAction::Link);
    assert_ne!(ToolbarAction::Bold, ToolbarAction::Italic);
}

#[test]
fn test_toolbar_action_debug() {
    let debug_str = format!("{:?}", ToolbarAction::Bold);
    assert!(debug_str.contains("Bold"));
}

#[test]
fn test_toolbar_action_formatting_variants() {
    assert_eq!(ToolbarAction::Bold, ToolbarAction::Bold);
    assert_eq!(ToolbarAction::Italic, ToolbarAction::Italic);
    assert_eq!(ToolbarAction::Underline, ToolbarAction::Underline);
    assert_eq!(ToolbarAction::Strikethrough, ToolbarAction::Strikethrough);
    assert_eq!(ToolbarAction::Code, ToolbarAction::Code);
}

#[test]
fn test_toolbar_action_insert_variants() {
    assert_eq!(ToolbarAction::Link, ToolbarAction::Link);
    assert_eq!(ToolbarAction::Image, ToolbarAction::Image);
    assert_eq!(ToolbarAction::HorizontalRule, ToolbarAction::HorizontalRule);
}

#[test]
fn test_toolbar_action_heading_variants() {
    assert_eq!(ToolbarAction::Heading1, ToolbarAction::Heading1);
    assert_eq!(ToolbarAction::Heading2, ToolbarAction::Heading2);
    assert_eq!(ToolbarAction::Heading3, ToolbarAction::Heading3);
    assert_ne!(ToolbarAction::Heading1, ToolbarAction::Heading2);
    assert_ne!(ToolbarAction::Heading2, ToolbarAction::Heading3);
}

#[test]
fn test_toolbar_action_block_variants() {
    assert_eq!(ToolbarAction::Quote, ToolbarAction::Quote);
    assert_eq!(ToolbarAction::BulletList, ToolbarAction::BulletList);
    assert_eq!(ToolbarAction::NumberedList, ToolbarAction::NumberedList);
    assert_eq!(ToolbarAction::CodeBlock, ToolbarAction::CodeBlock);
}

#[test]
fn test_toolbar_action_history_variants() {
    assert_eq!(ToolbarAction::Undo, ToolbarAction::Undo);
    assert_eq!(ToolbarAction::Redo, ToolbarAction::Redo);
    assert_ne!(ToolbarAction::Undo, ToolbarAction::Redo);
}

#[test]
fn test_toolbar_action_all_variants_unique() {
    let variants = [
        ToolbarAction::Bold,
        ToolbarAction::Italic,
        ToolbarAction::Underline,
        ToolbarAction::Strikethrough,
        ToolbarAction::Code,
        ToolbarAction::Link,
        ToolbarAction::Image,
        ToolbarAction::Heading1,
        ToolbarAction::Heading2,
        ToolbarAction::Heading3,
        ToolbarAction::Quote,
        ToolbarAction::BulletList,
        ToolbarAction::NumberedList,
        ToolbarAction::CodeBlock,
        ToolbarAction::HorizontalRule,
        ToolbarAction::Undo,
        ToolbarAction::Redo,
    ];

    // Check all are different from Bold
    for variant in variants.iter().skip(1) {
        assert_ne!(*variant, ToolbarAction::Bold);
    }
}

// =========================================================================
// EditorViewMode enum tests
// =========================================================================

#[test]
fn test_editor_view_mode_default() {
    assert_eq!(EditorViewMode::default(), EditorViewMode::Editor);
}

#[test]
fn test_editor_view_mode_clone() {
    let mode = EditorViewMode::Split;
    assert_eq!(mode, mode.clone());
}

#[test]
fn test_editor_view_mode_copy() {
    let mode1 = EditorViewMode::Preview;
    let mode2 = mode1;
    assert_eq!(mode1, EditorViewMode::Preview);
    assert_eq!(mode2, EditorViewMode::Preview);
}

#[test]
fn test_editor_view_mode_equality() {
    assert_eq!(EditorViewMode::Editor, EditorViewMode::Editor);
    assert_eq!(EditorViewMode::Preview, EditorViewMode::Preview);
    assert_eq!(EditorViewMode::Split, EditorViewMode::Split);
    assert_ne!(EditorViewMode::Editor, EditorViewMode::Preview);
    assert_ne!(EditorViewMode::Preview, EditorViewMode::Split);
    assert_ne!(EditorViewMode::Editor, EditorViewMode::Split);
}

#[test]
fn test_editor_view_mode_debug() {
    let debug_str = format!("{:?}", EditorViewMode::Editor);
    assert!(debug_str.contains("Editor"));

    let debug_str = format!("{:?}", EditorViewMode::Preview);
    assert!(debug_str.contains("Preview"));

    let debug_str = format!("{:?}", EditorViewMode::Split);
    assert!(debug_str.contains("Split"));
}

#[test]
fn test_editor_view_mode_all_variants() {
    let _ = EditorViewMode::Editor;
    let _ = EditorViewMode::Preview;
    let _ = EditorViewMode::Split;
}
