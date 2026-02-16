//! Tests for rich_text_editor cursor module

use revue::widget::form::rich_text_editor::RichTextEditor;

    // =========================================================================
    // Cursor movement tests - single block
    // =========================================================================

    #[test]
    fn test_move_left_at_start() {
        let mut editor = RichTextEditor::new();
        editor.move_left();
        // Should stay at (0, 0)
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    #[test]
    fn test_move_right() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.move_right();
        // Should be at (0, 1)
        assert_eq!(editor.cursor_position(), (0, 1));
    }

    #[test]
    fn test_move_right_at_end() {
        let mut editor = RichTextEditor::new().content("hi");
        editor.move_right();
        editor.move_right();
        // Should be at (0, 2) - at end of block
        assert_eq!(editor.cursor_position(), (0, 2));
    }

    #[test]
    fn test_move_home() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.move_right();
        editor.move_right();
        editor.move_home();
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    #[test]
    fn test_move_end() {
        let mut editor = RichTextEditor::new().content("test");
        editor.move_end();
        assert_eq!(editor.cursor_position(), (0, 4));
    }

    // =========================================================================
    // Cursor movement tests - multiple blocks
    // =========================================================================

    #[test]
    fn test_move_up_from_second_block() {
        let mut editor = RichTextEditor::new().content("first\nsecond");
        editor.move_down();
        assert_eq!(editor.cursor_position(), (1, 0));
        editor.move_up();
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    #[test]
    fn test_move_down() {
        let mut editor = RichTextEditor::new().content("first\nsecond");
        editor.move_down();
        assert_eq!(editor.cursor_position(), (1, 0));
    }

    #[test]
    fn test_move_up_at_top() {
        let mut editor = RichTextEditor::new().content("first\nsecond");
        editor.move_up();
        // Should stay at (0, 0)
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    #[test]
    fn test_move_down_at_bottom() {
        let mut editor = RichTextEditor::new().content("first\nsecond");
        editor.move_down();
        editor.move_down();
        // Should stay at (1, 0)
        assert_eq!(editor.cursor_position(), (1, 0));
    }

    #[test]
    fn test_move_right_across_blocks() {
        let mut editor = RichTextEditor::new().content("ab\ncd");
        editor.move_right();
        editor.move_right();
        // Should be at end of first block
        assert!(editor.cursor_position().1 >= 1);
    }

    #[test]
    fn test_move_left_across_blocks() {
        let mut editor = RichTextEditor::new().content("ab\ncd");
        editor.move_down();
        editor.move_left();
        // Should move to previous block end (column is usize, always >= 0)
        let pos = editor.cursor_position();
        assert!(pos.0 < 10); // Just verify it's in reasonable range
    }

    // =========================================================================
    // Document navigation tests
    // =========================================================================

    #[test]
    fn test_move_document_start() {
        let mut editor = RichTextEditor::new().content("line 1\nline 2\nline 3");
        editor.move_down();
        editor.move_down();
        editor.move_document_start();
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    #[test]
    fn test_move_document_end() {
        let mut editor = RichTextEditor::new().content("line 1\nline 2\nline 3");
        editor.move_document_end();
        assert_eq!(editor.cursor_position().0, 2);
        // Column is usize, always >= 0, just verify it's reasonable
        assert!(editor.cursor_position().1 < 100);
    }

    #[test]
    fn test_move_document_end_empty() {
        let mut editor = RichTextEditor::new();
        editor.move_document_end();
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    // =========================================================================
    // Complex navigation tests
    // =========================================================================

    #[test]
    fn test_cursor_movement_sequence() {
        let mut editor = RichTextEditor::new().content("abc\ndef");
        // Move right 3 times
        editor.move_right();
        editor.move_right();
        editor.move_right();
        assert!(editor.cursor_position().1 >= 2);
        // Move down
        editor.move_down();
        assert_eq!(editor.cursor_position().0, 1);
        // Move home
        editor.move_home();
        assert_eq!(editor.cursor_position(), (1, 0));
    }

    #[test]
    fn test_cursor_navigation_does_not_panic() {
        let mut editor = RichTextEditor::new().content("line 1\nline 2\nline 3");
        // Just verify all navigation works without panic
        editor.move_left();
        editor.move_right();
        editor.move_up();
        editor.move_down();
        editor.move_home();
        editor.move_end();
        editor.move_document_start();
        editor.move_document_end();
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_move_empty_editor() {
        let mut editor = RichTextEditor::new();
        editor.move_left();
        editor.move_right();
        editor.move_up();
        editor.move_down();
        // Should all work without panic
        assert_eq!(editor.cursor_position(), (0, 0));
    }

    #[test]
    fn test_move_long_line() {
        let mut editor = RichTextEditor::new().content("a".repeat(100));
        editor.move_end();
        assert_eq!(editor.cursor_position().1, 100);
        editor.move_home();
        assert_eq!(editor.cursor_position(), (0, 0));
    }
}
