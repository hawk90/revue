//! ToolbarAction and EditorViewMode values
//!
//! EditorViewMode's default is covered by the in-source view_mode tests.
//! The clone/copy/self-equality checks of the extracted file only
//! exercised derives and were dropped.

use revue::widget::{EditorViewMode, ToolbarAction};

#[test]
fn test_toolbar_action_variants_are_distinct() {
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

    for (i, a) in variants.iter().enumerate() {
        for (j, b) in variants.iter().enumerate() {
            assert_eq!(i == j, a == b, "{a:?} vs {b:?}");
        }
    }
}

#[test]
fn test_toolbar_action_debug() {
    assert_eq!(format!("{:?}", ToolbarAction::Bold), "Bold");
    assert_eq!(format!("{:?}", ToolbarAction::NumberedList), "NumberedList");
}

#[test]
fn test_editor_view_mode_variants_are_distinct() {
    assert_ne!(EditorViewMode::Editor, EditorViewMode::Preview);
    assert_ne!(EditorViewMode::Preview, EditorViewMode::Split);
    assert_ne!(EditorViewMode::Editor, EditorViewMode::Split);
}

#[test]
fn test_editor_view_mode_debug() {
    assert_eq!(format!("{:?}", EditorViewMode::Editor), "Editor");
    assert_eq!(format!("{:?}", EditorViewMode::Preview), "Preview");
    assert_eq!(format!("{:?}", EditorViewMode::Split), "Split");
}
