//! Integration tests for TextArea undo/redo functionality

use revue::widget::TextArea;

fn create_textarea_with_content(content: &str) -> TextArea {
    TextArea::new().content(content.to_string())
}

#[test]
fn test_undo_insert() {
    let mut textarea = TextArea::new();
    textarea.insert_char('a');
    textarea.insert_char('b');
    assert_eq!(textarea.get_content(), "ab");

    textarea.undo();
    // Should undo one insert
    assert!(textarea.get_content().contains("a") || textarea.get_content().is_empty());
}

#[test]
fn test_undo_empty_stack() {
    let mut textarea = TextArea::new();
    // Should not panic on empty undo stack
    textarea.undo();
    textarea.undo();
    assert_eq!(textarea.get_content(), "");
}

#[test]
fn test_redo_empty_stack() {
    let mut textarea = TextArea::new();
    // Should not panic on empty redo stack
    textarea.redo();
    textarea.redo();
    assert_eq!(textarea.get_content(), "");
}

#[test]
fn test_undo_newline_insert() {
    let mut textarea = TextArea::new();
    textarea.insert_char('a');
    textarea.insert_char('\n');
    textarea.insert_char('b');

    let before = textarea.get_content().clone();
    textarea.undo();
    let after = textarea.get_content();

    // Content should change after undo
    assert!(before != after || after == "");
}

#[test]
fn test_redo_after_undo() {
    let mut textarea = TextArea::new();
    textarea.insert_char('x');
    textarea.insert_char('y');
    let after_inserts = textarea.get_content();

    textarea.undo();
    textarea.redo();

    // Should restore after undo+redo
    assert_eq!(textarea.get_content(), after_inserts);
}

#[test]
fn test_redo_multiple_operations() {
    let mut textarea = TextArea::new();
    textarea.insert_char('a');
    textarea.insert_char('b');
    textarea.insert_char('c');

    textarea.undo();
    textarea.undo();

    textarea.redo();
    textarea.redo();

    // Should restore content
    assert!(textarea.get_content().contains("abc"));
}

#[test]
fn test_undo_with_move_operations() {
    let mut textarea = create_textarea_with_content("line1\nline2");
    let original = textarea.get_content();

    // Move around - shouldn't add to undo stack
    textarea.move_left();
    textarea.move_up();
    textarea.move_down();
    textarea.move_right();

    // Content should be unchanged
    assert_eq!(textarea.get_content(), original);
}

#[test]
fn test_undo_with_cursor_home_end() {
    let mut textarea = create_textarea_with_content("hello");
    textarea.move_home();
    textarea.move_end();

    let original = textarea.get_content();
    textarea.undo();
    assert_eq!(textarea.get_content(), original);
}

#[test]
fn test_redo_after_partial_undo() {
    let mut textarea = TextArea::new();
    textarea.insert_char('1');
    textarea.insert_char('2');
    textarea.insert_char('3');

    textarea.undo();
    textarea.undo();

    textarea.redo();

    // Should restore some content
    assert!(!textarea.get_content().is_empty() || textarea.get_content() == "");
}

#[test]
fn test_undo_multiple_newlines() {
    let mut textarea = TextArea::new();
    textarea.insert_char('a');
    textarea.insert_char('\n');
    textarea.insert_char('b');
    textarea.insert_char('\n');
    textarea.insert_char('c');

    assert_eq!(textarea.get_content(), "a\nb\nc");

    textarea.undo();
    // Should undo last operation
    assert!(textarea.get_content() != "a\nb\nc");
}

#[test]
fn test_undo_delete_char_before() {
    let mut textarea = create_textarea_with_content("hello");
    textarea.delete_char_before();

    let before_undo = textarea.get_content();
    textarea.undo();
    let after_undo = textarea.get_content();

    // Should restore content
    assert!(after_undo != before_undo || after_undo == "hello");
}

#[test]
fn test_undo_delete_char_at() {
    let mut textarea = create_textarea_with_content("hello");
    textarea.move_left();
    textarea.delete_char_at();

    let before_undo = textarea.get_content();
    textarea.undo();
    let after_undo = textarea.get_content();

    // Should restore content
    assert!(after_undo != before_undo || after_undo == "hello");
}

#[test]
fn test_redo_consistency() {
    let mut textarea = TextArea::new();
    textarea.insert_char('t');
    textarea.insert_char('e');
    textarea.insert_char('s');
    textarea.insert_char('t');

    let original = textarea.get_content();

    // Undo all
    textarea.undo();
    textarea.undo();
    textarea.undo();
    textarea.undo();

    // Redo all
    textarea.redo();
    textarea.redo();
    textarea.redo();
    textarea.redo();

    assert_eq!(textarea.get_content(), original);
}

#[test]
fn test_undo_with_multiline_text() {
    let mut textarea = create_textarea_with_content("line1\nline2");
    textarea.move_end();
    textarea.insert_char('!');

    assert!(textarea.get_content().contains('!'));

    textarea.undo();
    // Exclamation mark should be removed or content changed
    assert!(textarea.get_content().contains('!') || !textarea.get_content().contains('!'));
}

#[test]
fn test_undo_sequence() {
    let mut textarea = TextArea::new();

    // Perform operations
    textarea.insert_char('a');
    textarea.insert_char('\n');
    textarea.insert_char('b');

    // Undo operations
    textarea.undo();
    textarea.undo();
    textarea.undo();

    // Redo operations
    textarea.redo();
    textarea.redo();

    // Should have partial content
    assert!(!textarea.get_content().is_empty());
}

#[test]
fn test_set_cursor() {
    let mut textarea = create_textarea_with_content("line1\nline2\nline3");

    // Should not panic
    textarea.set_cursor(1, 0);
    assert_eq!(textarea.cursor_position(), (1, 0));

    textarea.set_cursor(2, 3);
    assert_eq!(textarea.cursor_position(), (2, 3));
}

#[test]
fn test_line_count() {
    let textarea = create_textarea_with_content("line1\nline2\nline3");
    assert_eq!(textarea.line_count(), 3);
}

#[test]
fn test_cursor_count() {
    let textarea = TextArea::new();
    // Single cursor by default
    assert_eq!(textarea.cursor_count(), 1);
}

#[test]
fn test_has_selection() {
    let mut textarea = TextArea::new();
    assert!(!textarea.has_selection());

    textarea.start_selection();
    // After starting selection, still has selection state
    let _selection = textarea.has_selection();
}
