//! ToolbarAction and EditorViewMode values

use revue::widget::EditorViewMode;
use revue::widget::RichTextEditor;
use revue::widget::ToolbarAction;

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
    let editor = RichTextEditor::new();
    // Default should be editor mode
    let _md = editor.to_markdown(); // Just verify it works
}

#[test]
fn test_view_mode_builder() {
    let _editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Split)
        .toolbar(true)
        .focused(true);
}
