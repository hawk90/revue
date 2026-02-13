//! CodeEditor cursor and navigation tests
//!
//! Extracted from src/widget/developer/code_editor/navigation.rs

use revue::widget::developer::code_editor::CodeEditor;

// =========================================================================
// cursor_position tests
// =========================================================================

#[test]
fn test_cursor_position_default() {
    let editor = CodeEditor::new();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_position_after_movement() {
    let mut editor = CodeEditor::new().content("ab\ncd");
    editor.move_right();
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 1));
}

// =========================================================================
// set_cursor tests
// =========================================================================

#[test]
fn test_set_cursor_basic() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(1, 2);
    assert_eq!(editor.cursor_position(), (1, 2));
}

#[test]
fn test_set_cursor_out_of_bounds_line() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.set_cursor(10, 0);
    assert_eq!(editor.cursor_position().0, 1); // Clamped to last line
}

#[test]
fn test_set_cursor_out_of_bounds_col() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 100);
    assert_eq!(editor.cursor_position().1, 5); // Clamped to line length
}

#[test]
fn test_set_cursor_extends_selection() {
    let mut editor = CodeEditor::new().content("test");
    editor.start_selection();
    editor.move_right();
    editor.set_cursor(0, 3);
    // set_cursor should extend selection when in selection mode
    assert!(editor.has_selection());
}

// =========================================================================
// line_count tests
// =========================================================================

#[test]
fn test_line_count_default() {
    let editor = CodeEditor::new();
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_line_count_multiple() {
    let editor = CodeEditor::new().content("a\nb\nc\nd");
    assert_eq!(editor.line_count(), 4);
}

#[test]
fn test_line_count_empty() {
    let editor = CodeEditor::new().content("");
    assert_eq!(editor.line_count(), 1);
}

// =========================================================================
// move_left tests
// =========================================================================

#[test]
fn test_move_left_basic() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 3);
    editor.move_left();
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_move_left_at_start() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 0);
    editor.move_left();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_move_left_to_previous_line() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.set_cursor(1, 0);
    editor.move_left();
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_move_left_extends_selection() {
    let mut editor = CodeEditor::new().content("test");
    editor.start_selection();
    editor.move_right();
    editor.move_right();
    editor.move_left();
    // Movement should extend selection when in selection mode
    assert!(editor.has_selection());
}

// =========================================================================
// move_right tests
// =========================================================================

#[test]
fn test_move_right_basic() {
    let mut editor = CodeEditor::new().content("hello");
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 1));
}

#[test]
fn test_move_right_at_end() {
    let mut editor = CodeEditor::new().content("hi");
    editor.set_cursor(0, 2);
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_move_right_to_next_line() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.set_cursor(0, 5);
    editor.move_right();
    assert_eq!(editor.cursor_position(), (1, 0));
}

#[test]
fn test_move_right_extends_selection() {
    let mut editor = CodeEditor::new().content("test");
    editor.start_selection();
    editor.move_right();
    editor.move_right();
    // Movement should extend selection when in selection mode
    assert!(editor.has_selection());
}

// =========================================================================
// move_up tests
// =========================================================================

#[test]
fn test_move_up_basic() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(2, 3);
    editor.move_up();
    assert_eq!(editor.cursor_position(), (1, 3));
}

#[test]
fn test_move_up_at_top() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.set_cursor(0, 2);
    editor.move_up();
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_move_up_clamps_col() {
    let mut editor = CodeEditor::new().content("short\nlonger line");
    editor.set_cursor(1, 10);
    editor.move_up();
    assert_eq!(editor.cursor_position(), (0, 5)); // Clamped to "short" length
}

#[test]
fn test_move_up_extends_selection() {
    let mut editor = CodeEditor::new().content("a\nb");
    editor.set_cursor(1, 0);
    editor.start_selection();
    editor.move_up();
    // Movement should extend selection when in selection mode
    assert!(editor.has_selection());
}

// =========================================================================
// move_down tests
// =========================================================================

#[test]
fn test_move_down_basic() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(0, 3);
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 3));
}

#[test]
fn test_move_down_at_bottom() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.set_cursor(1, 2);
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 2));
}

#[test]
fn test_move_down_clamps_col() {
    let mut editor = CodeEditor::new().content("longer line\nshort");
    editor.set_cursor(0, 10);
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 5)); // Clamped to "short" length
}

#[test]
fn test_move_down_extends_selection() {
    let mut editor = CodeEditor::new().content("a\nb");
    editor.start_selection();
    editor.move_down();
    // Movement should extend selection when in selection mode
    assert!(editor.has_selection());
}

// =========================================================================
// move_home tests
// =========================================================================

#[test]
fn test_move_home_from_middle() {
    let mut editor = CodeEditor::new().content("    hello");
    editor.set_cursor(0, 6);
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 4)); // First non-whitespace
}

#[test]
fn test_move_home_from_start() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 3);
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_move_home_toggle() {
    let mut editor = CodeEditor::new().content("    test");
    editor.set_cursor(0, 8);
    editor.move_home();
    let pos1 = editor.cursor_position();
    editor.move_home();
    let pos2 = editor.cursor_position();
    assert_ne!(pos1, pos2);
}

#[test]
fn test_move_home_extends_selection() {
    let mut editor = CodeEditor::new().content("  test");
    editor.start_selection();
    editor.move_home();
    // Movement should extend selection when in selection mode
    assert!(editor.has_selection());
}

// =========================================================================
// move_end tests
// =========================================================================

#[test]
fn test_move_end_basic() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 2);
    editor.move_end();
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_move_end_empty_line() {
    let mut editor = CodeEditor::new().content("hello\n\nworld");
    editor.set_cursor(1, 0);
    editor.move_end();
    assert_eq!(editor.cursor_position(), (1, 0));
}

#[test]
fn test_move_end_extends_selection() {
    let mut editor = CodeEditor::new().content("test");
    editor.start_selection();
    editor.move_end();
    // Movement should extend selection when in selection mode
    assert!(editor.has_selection());
}

// =========================================================================
// move_document_start tests
// =========================================================================

#[test]
fn test_move_document_start_basic() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(2, 5);
    editor.move_document_start();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_move_document_start_extends_selection() {
    let mut editor = CodeEditor::new().content("a\nb\nc");
    editor.set_cursor(2, 0);
    editor.start_selection();
    editor.move_document_start();
    // Movement should extend selection when in selection mode
    assert!(editor.has_selection());
}

// =========================================================================
// move_document_end tests
// =========================================================================

#[test]
fn test_move_document_end_basic() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.move_document_end();
    assert_eq!(editor.cursor_position(), (2, 5));
}

#[test]
fn test_move_document_end_empty() {
    let mut editor = CodeEditor::new();
    editor.move_document_end();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_move_document_end_extends_selection() {
    let mut editor = CodeEditor::new().content("a\nb");
    editor.start_selection();
    editor.move_document_end();
    // Movement should extend selection when in selection mode
    assert!(editor.has_selection());
}

// =========================================================================
// move_word_left tests
// =========================================================================

#[test]
fn test_move_word_left_basic() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 8);
    editor.move_word_left();
    assert_eq!(editor.cursor_position(), (0, 6)); // Start of "world"
}

#[test]
fn test_move_word_left_at_start() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 0);
    editor.move_word_left();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_move_word_left_over_whitespace() {
    let mut editor = CodeEditor::new().content("hello   world");
    editor.set_cursor(0, 10);
    editor.move_word_left();
    // From position 10 ('r'), move_word_left goes to start of "world" (position 8)
    assert_eq!(editor.cursor_position(), (0, 8));
}

#[test]
fn test_move_word_left_to_previous_line() {
    let mut editor = CodeEditor::new().content("hello\nworld");
    editor.set_cursor(1, 0);
    editor.move_word_left();
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_move_word_left_extends_selection() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 11);
    editor.start_selection();
    editor.move_word_left();
    // Movement should extend selection when in selection mode
    assert!(editor.has_selection());
}

// =========================================================================
// move_word_right tests
// =========================================================================

#[test]
fn test_move_word_right_basic() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 2);
    editor.move_word_right();
    assert_eq!(editor.cursor_position(), (0, 6)); // After "hello"
}

#[test]
fn test_move_word_right_at_end() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.move_word_right();
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_move_word_right_over_whitespace() {
    let mut editor = CodeEditor::new().content("hello   world");
    editor.set_cursor(0, 2);
    editor.move_word_right();
    // move_word_right lands at start of next word ("world" at position 8)
    assert_eq!(editor.cursor_position(), (0, 8));
}

#[test]
fn test_move_word_right_to_next_line() {
    let mut editor = CodeEditor::new().content("hello\nworld");
    editor.set_cursor(0, 5);
    editor.move_word_right();
    assert_eq!(editor.cursor_position(), (1, 0)); // Start of next line
}

#[test]
fn test_move_word_right_extends_selection() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.start_selection();
    editor.move_word_right();
    // Movement should extend selection when in selection mode
    assert!(editor.has_selection());
}

// =========================================================================
// page_up tests
// =========================================================================

#[test]
fn test_page_up_basic() {
    let mut editor = CodeEditor::new().content("1\n2\n3\n4\n5\n6\n7\n8\n9\n10");
    editor.set_cursor(9, 0);
    editor.page_up(5);
    assert_eq!(editor.cursor_position(), (4, 0));
}

#[test]
fn test_page_up_clamps_to_top() {
    let mut editor = CodeEditor::new().content("1\n2\n3");
    editor.set_cursor(2, 0);
    editor.page_up(10);
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_page_up_extends_selection() {
    let mut editor = CodeEditor::new().content("1\n2\n3\n4\n5");
    editor.set_cursor(4, 0);
    editor.start_selection();
    editor.page_up(2);
    // Movement should extend selection when in selection mode
    assert!(editor.has_selection());
}

// =========================================================================
// page_down tests
// =========================================================================

#[test]
fn test_page_down_basic() {
    let mut editor = CodeEditor::new().content("1\n2\n3\n4\n5\n6\n7\n8\n9\n10");
    editor.set_cursor(0, 0);
    editor.page_down(5);
    assert_eq!(editor.cursor_position(), (5, 0));
}

#[test]
fn test_page_down_clamps_to_bottom() {
    let mut editor = CodeEditor::new().content("1\n2\n3");
    editor.set_cursor(0, 0);
    editor.page_down(10);
    assert_eq!(editor.cursor_position(), (2, 0));
}

#[test]
fn test_page_down_extends_selection() {
    let mut editor = CodeEditor::new().content("1\n2\n3\n4\n5");
    editor.start_selection();
    editor.page_down(2);
    // Movement should extend selection when in selection mode
    assert!(editor.has_selection());
}
