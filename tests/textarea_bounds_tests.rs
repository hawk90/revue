//! Bounds checking regression tests for TextArea (#144)
//!
//! These tests verify that TextArea handles edge cases without panicking.

use revue::widget::TextArea;

#[test]
fn test_delete_selection_multiline() {
    let mut ta = TextArea::new().content("Line1\nLine2\nLine3");
    ta.set_cursor(0, 2);
    ta.select_all();
    // Should not panic even with full selection
    ta.delete_selection();
    assert_eq!(ta.get_content(), "");
}

#[test]
fn test_delete_char_before_at_start() {
    let mut ta = TextArea::new().content("Hello");
    ta.set_cursor(0, 0);
    // Should not panic when at start of document
    ta.delete_char_before();
    assert_eq!(ta.get_content(), "Hello");
}

#[test]
fn test_delete_char_before_merge_lines() {
    let mut ta = TextArea::new().content("Line1\nLine2");
    ta.set_cursor(1, 0);
    // Should merge lines without panic
    ta.delete_char_before();
    assert_eq!(ta.get_content(), "Line1Line2");
}

#[test]
fn test_delete_char_at_end() {
    let mut ta = TextArea::new().content("Hello");
    ta.set_cursor(0, 5);
    // Should not panic at end of line with no next line
    ta.delete_char_at();
    assert_eq!(ta.get_content(), "Hello");
}

#[test]
fn test_delete_char_at_merge_lines() {
    let mut ta = TextArea::new().content("Line1\nLine2");
    ta.set_cursor(0, 5);
    // Should merge with next line without panic
    ta.delete_char_at();
    assert_eq!(ta.get_content(), "Line1Line2");
}

#[test]
fn test_duplicate_line() {
    let mut ta = TextArea::new().content("Hello");
    ta.set_cursor(0, 0);
    // Should duplicate without panic
    ta.duplicate_line();
    assert_eq!(ta.line_count(), 2);
}

#[test]
fn test_move_word_left_empty() {
    let mut ta = TextArea::new().content("");
    ta.set_cursor(0, 0);
    // Should not panic on empty content
    ta.move_word_left();
    assert_eq!(ta.cursor_position(), (0, 0));
}

#[test]
fn test_move_word_right_empty() {
    let mut ta = TextArea::new().content("");
    ta.set_cursor(0, 0);
    // Should not panic on empty content
    ta.move_word_right();
    assert_eq!(ta.cursor_position(), (0, 0));
}

#[test]
fn test_find_in_empty() {
    let mut ta = TextArea::new().content("");
    ta.open_find();
    ta.set_find_query("test");
    // Should not panic searching in empty content
    let state = ta.find_state().unwrap();
    assert_eq!(state.matches.len(), 0);
}

#[test]
fn test_replace_multiline() {
    let mut ta = TextArea::new().content("Line1\nLine2\nLine3");
    ta.open_find();
    ta.set_find_query("Line");
    ta.set_replace_text("Row");
    // Should replace all without panic
    ta.replace_all();
    assert!(ta.get_content().contains("Row"));
}
