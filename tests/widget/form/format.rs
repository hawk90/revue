//! Tests for rich_text_editor format module

use revue::widget::form::rich_text_editor::RichTextEditor;

    // =========================================================================
    // toggle_bold tests
    // =========================================================================

    #[test]
    fn test_toggle_bold_from_default() {
        let mut editor = RichTextEditor::new();
        editor.toggle_bold();
        assert!(editor.current_format().bold);
    }

    #[test]
    fn test_toggle_bold_twice() {
        let mut editor = RichTextEditor::new();
        editor.toggle_bold();
        assert!(editor.current_format().bold);
        editor.toggle_bold();
        assert!(!editor.current_format().bold);
    }

    // =========================================================================
    // toggle_italic tests
    // =========================================================================

    #[test]
    fn test_toggle_italic_from_default() {
        let mut editor = RichTextEditor::new();
        editor.toggle_italic();
        assert!(editor.current_format().italic);
    }

    #[test]
    fn test_toggle_italic_twice() {
        let mut editor = RichTextEditor::new();
        editor.toggle_italic();
        assert!(editor.current_format().italic);
        editor.toggle_italic();
        assert!(!editor.current_format().italic);
    }

    // =========================================================================
    // toggle_underline tests
    // =========================================================================

    #[test]
    fn test_toggle_underline_from_default() {
        let mut editor = RichTextEditor::new();
        editor.toggle_underline();
        assert!(editor.current_format().underline);
    }

    #[test]
    fn test_toggle_underline_twice() {
        let mut editor = RichTextEditor::new();
        editor.toggle_underline();
        assert!(editor.current_format().underline);
        editor.toggle_underline();
        assert!(!editor.current_format().underline);
    }

    // =========================================================================
    // toggle_strikethrough tests
    // =========================================================================

    #[test]
    fn test_toggle_strikethrough_from_default() {
        let mut editor = RichTextEditor::new();
        editor.toggle_strikethrough();
        assert!(editor.current_format().strikethrough);
    }

    #[test]
    fn test_toggle_strikethrough_twice() {
        let mut editor = RichTextEditor::new();
        editor.toggle_strikethrough();
        assert!(editor.current_format().strikethrough);
        editor.toggle_strikethrough();
        assert!(!editor.current_format().strikethrough);
    }

    // =========================================================================
    // toggle_code tests
    // =========================================================================

    #[test]
    fn test_toggle_code_from_default() {
        let mut editor = RichTextEditor::new();
        editor.toggle_code();
        assert!(editor.current_format().code);
    }

    #[test]
    fn test_toggle_code_twice() {
        let mut editor = RichTextEditor::new();
        editor.toggle_code();
