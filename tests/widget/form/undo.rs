//! Tests for rich_text_editor undo module

use revue::widget::form::rich_text_editor::RichTextEditor;
use revue::widget::form::rich_text_editor::undo::MAX_UNDO_HISTORY;

    // =========================================================================
    // MAX_UNDO_HISTORY constant tests
    // =========================================================================

    #[test]
    fn test_max_undo_history_value() {
        assert_eq!(MAX_UNDO_HISTORY, 100);
    }

    // =========================================================================
    // undo tests
    // =========================================================================

    #[test]
    fn test_undo_empty() {
        let mut editor = RichTextEditor::new();
        editor.undo();
        // Should not panic on empty undo stack
        assert_eq!(editor.get_content(), "");
    }

    #[test]
    fn test_undo_insert_char() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('a');
        editor.insert_char('b');
        assert_eq!(editor.get_content(), "ab");
        editor.undo();
        assert_eq!(editor.get_content(), "a");
    }

    #[test]
    fn test_undo_multiple() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("hello");
        editor.undo();
        editor.undo();
        editor.undo();
        editor.undo();
        editor.undo();
        // All characters undone
        assert_eq!(editor.get_content(), "");
    }

    #[test]
    fn test_undo_then_redo() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("hi");
        editor.undo();
        // Undoes 'i', leaving 'h'
        assert_eq!(editor.get_content(), "h");
        editor.redo();
        // Redoes 'i', back to 'hi'
        assert_eq!(editor.get_content(), "hi");
    }

    #[test]
    fn test_undo_newline() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('a');
        editor.insert_char('\n');
        editor.insert_char('b');
        assert_eq!(editor.block_count(), 2);
        editor.undo();
        // Undoes 'b' insertion, still 2 blocks (newline remains)
        assert_eq!(editor.block_count(), 2);
    }

    #[test]
    fn test_undo_delete_char_before() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.move_end();
        editor.delete_char_before();
        assert_eq!(editor.get_content(), "hell");
        editor.undo();
        assert_eq!(editor.get_content(), "hello");
    }

    #[test]
    fn test_undo_clears_redo() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("ab");
        editor.undo();
        // Content is "a", redo stack has one item
        editor.insert_str("c");
        // After new operation, redo should be cleared
        // Content is "ac"
        editor.redo();
        // No redo available since we inserted after undoing
        assert_eq!(editor.get_content(), "ac");
    }

    // =========================================================================
    // redo tests
    // =========================================================================

    #[test]
    fn test_redo_empty() {
        let mut editor = RichTextEditor::new();
        editor.redo();
        // Should not panic on empty redo stack
        assert_eq!(editor.get_content(), "");
    }

    #[test]
    fn test_redo_after_undo() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("test");
        editor.undo();
        editor.undo();
        editor.undo();
        editor.undo();
        // All undone, content is ""
        editor.redo();
        // Redoes first 't'
        assert_eq!(editor.get_content(), "t");
    }

    #[test]
    fn test_redo_multiple() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("hi");
        editor.undo();
        editor.undo();
        // Both undone, content is ""
        editor.redo();
        editor.redo();
        // Both redone
        assert_eq!(editor.get_content(), "hi");
    }

    #[test]
    fn test_redo_newline() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('a');
        editor.insert_char('\n');
        editor.undo();
        // Undoes newline, back to 1 block
        assert_eq!(editor.block_count(), 1);
        editor.redo();
        // Redoes newline, back to 2 blocks
        assert_eq!(editor.block_count(), 2);
    }

    #[test]
    fn test_redo_delete_char() {
        let mut editor = RichTextEditor::new().content("ab");
        editor.delete_char_at();
        assert_eq!(editor.get_content(), "b");
        editor.undo();
        assert_eq!(editor.get_content(), "ab");
        editor.redo();
        assert_eq!(editor.get_content(), "b");
    }

    // =========================================================================
    // Undo/redo interaction tests
    // =========================================================================

    #[test]
    fn test_undo_redo_undo_cycle() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("xyz");
        editor.undo();
        // Content is "xy"
        editor.redo();
        // Content is "xyz"
        editor.undo();
        // Content is "xy"
        assert_eq!(editor.get_content(), "xy");
    }

    #[test]
    fn test_multiple_operations_undo_redo() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('a');
        editor.insert_char('b');
        editor.insert_char('c');
        editor.undo();
        editor.redo();
        editor.undo();
        editor.undo();
        editor.redo();
        editor.redo();
        assert_eq!(editor.get_content(), "abc");
    }

    #[test]
    fn test_undo_history_limit() {
        let mut editor = RichTextEditor::new();
        // Insert more than MAX_UNDO_HISTORY characters
        for _ in 0..MAX_UNDO_HISTORY + 10 {
            editor.insert_char('x');
        }
        // Should not crash, history should be limited
        editor.undo();
        editor.undo();
        // Just verify it works
    }

    #[test]
    fn test_undo_with_cursor_position() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("hello");
        editor.undo();
        // Cursor should be restored to position 4 (after "hell")
        let pos = editor.cursor_position();
        assert_eq!(pos, (0, 4));
    }

    #[test]
    fn test_redo_with_cursor_position() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("hi");
        editor.undo();
        editor.redo();
        // Cursor should be at position 2 (after "hi")
        let pos = editor.cursor_position();
        assert_eq!(pos, (0, 2));
    }
}
