//! RichTextEditor undo and redo tests

use revue::widget::RichTextEditor;

// =========================================================================
// undo tests
// =========================================================================

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

// =========================================================================
// redo tests
// =========================================================================

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
    // The undo history keeps the last 100 edits.
    let mut editor = RichTextEditor::new();
    for _ in 0..110 {
        editor.insert_char('x');
    }
    for _ in 0..200 {
        editor.undo();
    }
    assert_eq!(editor.get_content(), "x".repeat(10));
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

#[test]
fn test_undo_insert() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.insert_char('!');
    assert_eq!(editor.get_content(), "hello!");

    editor.undo();
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_redo() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.insert_char('!');
    editor.undo();
    assert_eq!(editor.get_content(), "hello");

    editor.redo();
    assert_eq!(editor.get_content(), "hello!");
}

#[test]
fn test_undo_delete() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "hell");

    editor.undo();
    assert_eq!(editor.get_content(), "hello");
}
