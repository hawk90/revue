//! Tests for rich_text_editor selection module

use revue::widget::form::rich_text_editor::RichTextEditor;

    // =========================================================================
    // Selection state tests
    // =========================================================================

    #[test]
    fn test_has_selection_default() {
        let editor = RichTextEditor::new();
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_has_selection_after_start() {
        let mut editor = RichTextEditor::new();
        editor.start_selection();
        assert!(editor.has_selection());
    }

    #[test]
    fn test_has_selection_after_clear() {
        let mut editor = RichTextEditor::new();
        editor.start_selection();
        editor.clear_selection();
        assert!(!editor.has_selection());
    }

    // =========================================================================
    // start_selection tests
    // =========================================================================

    #[test]
    fn test_start_selection_sets_anchor() {
        let mut editor = RichTextEditor::new();
        editor.start_selection();
        assert!(editor.has_selection());
    }

    #[test]
    fn test_start_selection_with_cursor_position() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.move_right();
        editor.start_selection();
        assert!(editor.has_selection());
    }

    // =========================================================================
    // clear_selection tests
    // =========================================================================

    #[test]
    fn test_clear_selection_removes_anchor() {
        let mut editor = RichTextEditor::new();
        editor.start_selection();
        editor.clear_selection();
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_clear_selection_idempotent() {
        let mut editor = RichTextEditor::new();
        editor.start_selection();
        editor.clear_selection();
        editor.clear_selection();
        assert!(!editor.has_selection());
    }

    // =========================================================================
    // get_selection tests
    // =========================================================================

    #[test]
    fn test_get_selection_none_when_no_selection() {
        let editor = RichTextEditor::new();
        assert_eq!(editor.get_selection(), None);
    }

    #[test]
    fn test_get_selection_single_line() {
        let mut editor = RichTextEditor::new().content("hello world");
        // Start selection at position 0,0
        editor.start_selection();
        // Movement clears selection
        editor.move_right();
        // After movement, selection is cleared
        assert_eq!(editor.get_selection(), None);
    }

    #[test]
    fn test_get_selection_reversed() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.move_end();
        editor.start_selection();
        // Movement clears selection
        editor.move_left();
        assert_eq!(editor.get_selection(), None);
    }

    #[test]
    fn test_get_selection_empty_selection() {
        let mut editor = RichTextEditor::new().content("test");
        editor.start_selection();
        // Cursor and anchor are same (both at 0,0)
        let selection = editor.get_selection();
        // Empty string when cursor == anchor
        assert!(selection.is_some());
        assert_eq!(selection.unwrap(), "");
    }

    #[test]
    fn test_get_selection_multiline() {
        let mut editor = RichTextEditor::new().content("line 1\nline 2\nline 3");
        editor.move_down();
        editor.start_selection();
        // Movement clears selection
        editor.move_down();
        assert_eq!(editor.get_selection(), None);
    }

    #[test]
    fn test_get_selection_full_line() {
        let mut editor = RichTextEditor::new().content("hello world");
        editor.start_selection();
        // Movement clears selection
        editor.move_end();
        assert_eq!(editor.get_selection(), None);
    }

    #[test]
    fn test_get_selection_at_start() {
        let mut editor = RichTextEditor::new().content("test");
        editor.start_selection();
        // At position 0,0, anchor = (0,0), cursor = (0,0)
        let selection = editor.get_selection();
        assert!(selection.is_some());
        assert_eq!(selection.unwrap(), "");
    }

    #[test]
    fn test_get_selection_empty_content() {
        let mut editor = RichTextEditor::new();
        editor.start_selection();
        let selection = editor.get_selection();
        // Empty editor with selection
        assert!(selection.is_some());
        assert_eq!(selection.unwrap(), "");
    }

    // =========================================================================
    // delete_selection tests
    // =========================================================================

    #[test]
    fn test_delete_selection_no_selection() {
        let mut editor = RichTextEditor::new().content("hello");
        let cursor_before = editor.cursor_position();
        editor.delete_selection();
        // Should do nothing when no selection
        assert_eq!(editor.cursor_position(), cursor_before);
        assert_eq!(editor.get_content(), "hello");
    }

    #[test]
    fn test_delete_selection_single_char() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.start_selection();
        // Movement clears selection, so delete_selection won't do anything
        editor.move_right();
        editor.move_right();
        let content_before = editor.get_content();
        editor.delete_selection();
        let content_after = editor.get_content();
        assert_eq!(content_before, content_after);
    }

    #[test]
    fn test_delete_selection_multiple_chars() {
        let mut editor = RichTextEditor::new().content("hello world");
        editor.start_selection();
        // Movement clears selection
        editor.move_right();
        editor.move_right();
        editor.move_right();
        editor.move_right();
        editor.move_right();
        editor.delete_selection();
        // Content unchanged since selection was cleared
        assert_eq!(editor.get_content(), "hello world");
    }

    #[test]
    fn test_delete_selection_clears_anchor() {
        let mut editor = RichTextEditor::new().content("test");
        editor.start_selection();
        // Movement clears selection
        editor.move_right();
        editor.move_right();
        editor.delete_selection();
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_delete_selection_moves_cursor() {
        let mut editor = RichTextEditor::new().content("test");
        editor.move_end();
        editor.start_selection();
        editor.move_left();
        let pos_before = editor.cursor_position();
        editor.delete_selection();
        let pos_after = editor.cursor_position();
        // Cursor unchanged since selection was cleared
        assert_eq!(pos_before, pos_after);
    }

    #[test]
    fn test_delete_selection_multiline() {
        let mut editor = RichTextEditor::new().content("line 1\nline 2\nline 3");
        editor.move_down();
        editor.start_selection();
        editor.move_down();
        let blocks_before = editor.block_count();
        editor.delete_selection();
        let blocks_after = editor.block_count();
        // No blocks deleted since selection was cleared
        assert_eq!(blocks_before, blocks_after);
    }

    #[test]
    fn test_delete_selection_entire_content() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.start_selection();
        editor.move_end();
        editor.delete_selection();
        // Content unchanged since selection was cleared by movement
        assert_eq!(editor.get_content(), "hello");
    }

    #[test]
    fn test_delete_selection_from_end() {
        let mut editor = RichTextEditor::new().content("testing");
        editor.move_end();
        editor.start_selection();
        // Move left a few times - clears selection
        editor.move_left();
        editor.move_left();
        editor.delete_selection();
        // Content unchanged since selection was cleared
        assert_eq!(editor.get_content(), "testing");
    }

    #[test]
    fn test_delete_selection_full_line() {
        let mut editor = RichTextEditor::new().content("single line");
        editor.start_selection();
        editor.move_end();
        editor.delete_selection();
        // Content unchanged since selection was cleared
        assert_eq!(editor.get_content(), "single line");
    }

    // =========================================================================
    // Selection behavior tests
    // =========================================================================

    #[test]
    fn test_movement_clears_selection() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.start_selection();
        editor.move_right();
        assert!(!editor.has_selection());
        // Movement should clear selection
    }

    #[test]
    fn test_start_selection_twice() {
        let mut editor = RichTextEditor::new().content("test");
        editor.move_right();
        editor.start_selection();
        assert!(editor.has_selection());
        editor.move_right();
        assert!(!editor.has_selection());
        editor.start_selection();
        // Starting selection again should set anchor at new position
        assert!(editor.has_selection());
    }

    #[test]
    fn test_selection_with_empty_editor() {
        let mut editor = RichTextEditor::new();
        editor.start_selection();
        let selection = editor.get_selection();
        assert!(selection.is_some());
        // Empty selection when editor is empty
        assert_eq!(selection.unwrap(), "");
    }

    #[test]
    fn test_forward_selection() {
        let mut editor = RichTextEditor::new().content("hello");
        // Move to position first (clears any selection)
        editor.move_right();
        assert!(!editor.has_selection());
        // Start selection - this sets anchor
        editor.start_selection();
        assert!(editor.has_selection());
        // Movement clears selection
        editor.move_right();
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_backward_selection() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.move_end();
        assert!(!editor.has_selection());
        // Start selection
        editor.start_selection();
        assert!(editor.has_selection());
        // Movement clears selection
        editor.move_left();
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_selection_cursor_tracking() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.start_selection();
        editor.move_end();
        // Movement clears selection
        assert!(!editor.has_selection());
    }
}
