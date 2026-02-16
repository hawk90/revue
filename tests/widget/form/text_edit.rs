//! Tests for rich_text_editor text_edit module

use revue::widget::form::rich_text_editor::RichTextEditor;

    // =========================================================================
    // insert_char tests
    // =========================================================================

    #[test]
    fn test_insert_char_basic() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('a');
        assert_eq!(editor.get_content(), "a");
    }

    #[test]
    fn test_insert_char_multiple() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('h');
        editor.insert_char('e');
        editor.insert_char('l');
        editor.insert_char('l');
        editor.insert_char('o');
        assert_eq!(editor.get_content(), "hello");
    }

    #[test]
    fn test_insert_char_newline() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('a');
        editor.insert_char('\n');
        editor.insert_char('b');
        assert_eq!(editor.get_content(), "a\nb");
    }

    #[test]
    fn test_insert_char_unicode() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('你');
        editor.insert_char('好');
        assert_eq!(editor.get_content(), "你好");
    }

    #[test]
    fn test_insert_char_special_chars() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('@');
        editor.insert_char('#');
        editor.insert_char('$');
        assert_eq!(editor.get_content(), "@#$");
    }

    #[test]
    fn test_insert_char_empty() {
        let mut editor = RichTextEditor::new();
        editor.insert_char(' ');
        assert_eq!(editor.get_content(), " ");
    }

    // =========================================================================
    // insert_str tests
    // =========================================================================

    #[test]
    fn test_insert_str_basic() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("hello");
        assert_eq!(editor.get_content(), "hello");
    }

    #[test]
    fn test_insert_str_with_newline() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("line1\nline2");
        assert_eq!(editor.get_content(), "line1\nline2");
    }

    #[test]
    fn test_insert_str_multiple() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("hello");
        editor.insert_str(" ");
        editor.insert_str("world");
        assert_eq!(editor.get_content(), "hello world");
    }

    #[test]
    fn test_insert_str_empty() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("");
        assert_eq!(editor.get_content(), "");
    }

    #[test]
    fn test_insert_str_unicode() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("Hello世界");
        assert_eq!(editor.get_content(), "Hello世界");
    }
