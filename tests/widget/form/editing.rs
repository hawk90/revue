//! Tests for rich_text_editor editing module

use revue::event::Key;
use revue::widget::form::rich_text_editor::RichTextEditor;
use revue::widget::form::rich_text_editor::types::ToolbarAction;
    use crate::event::Key;

    // =========================================================================
    // toolbar_action tests
    // =========================================================================

    #[test]
    fn test_toolbar_action_bold() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Bold);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_italic() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Italic);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_underline() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Underline);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_strikethrough() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Strikethrough);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_code() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Code);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_link() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Link);
        assert!(editor.is_dialog_open());
    }

    #[test]
    fn test_toolbar_action_image() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Image);
        assert!(editor.is_dialog_open());
    }

    #[test]
    fn test_toolbar_action_heading1() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Heading1);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_heading2() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Heading2);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_heading3() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Heading3);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_quote() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Quote);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_bullet_list() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::BulletList);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_numbered_list() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::NumberedList);
        // Just verify it doesn't panic
    }
