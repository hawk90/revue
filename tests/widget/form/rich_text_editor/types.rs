//! ToolbarAction and EditorViewMode values

use revue::widget::EditorViewMode;
use revue::widget::RichTextEditor;
use revue::widget::ToolbarAction;

use super::{cells_with_bg, row, screen, CURSOR_BG, PREVIEW_BG, TOOLBAR};

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

#[test]
fn test_view_mode_default() {
    // Default should be editor mode
    assert_eq!(EditorViewMode::default(), EditorViewMode::Editor);
    let editor = RichTextEditor::new().content("x");
    let buffer = screen(&editor, 30, 5);
    // The editor: its cursor on the text, no preview background
    assert_eq!(row(&buffer, 1), "x");
    assert_eq!(cells_with_bg(&buffer, CURSOR_BG), vec![(0, 1)]);
    assert!(cells_with_bg(&buffer, PREVIEW_BG).is_empty());
}

#[test]
fn test_view_mode_builder() {
    let editor = RichTextEditor::new()
        .content("ab")
        .view_mode(EditorViewMode::Split)
        .toolbar(true)
        .focused(true);
    let buffer = screen(&editor, 40, 5);
    assert_eq!(row(&buffer, 0), TOOLBAR);
    // The editor with its cursor on the left, the preview on the right
    assert_eq!(row(&buffer, 1), format!("{:<20}ab", "ab"));
    assert_eq!(cells_with_bg(&buffer, CURSOR_BG), vec![(0, 1)]);
    assert_eq!(buffer.get(20, 1).unwrap().bg, Some(PREVIEW_BG));
}
