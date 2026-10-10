//! CodeEditor modes (go-to-line, find) tests

use revue::widget::CodeEditor;

// =========================================================================
// Go-to-Line mode tests
// =========================================================================

#[test]
fn test_goto_line_open_close() {
    let mut editor = CodeEditor::default();
    assert!(!editor.is_goto_line_active());
    editor.open_goto_line();
    assert!(editor.is_goto_line_active());
    editor.close_goto_line();
    assert!(!editor.is_goto_line_active());
}

// =========================================================================
// Find mode tests
// =========================================================================

#[test]
fn test_find_open_close() {
    let mut editor = CodeEditor::default();
    assert!(!editor.is_find_active());
    editor.open_find();
    assert!(editor.is_find_active());
    editor.close_find();
    assert!(!editor.is_find_active());
}

#[test]
fn test_find_match_count_empty() {
    let editor = CodeEditor::default();
    assert_eq!(editor.find_match_count(), 0);
}

#[test]
fn test_find_current_index_empty() {
    let editor = CodeEditor::default();
    assert_eq!(editor.current_find_index(), 0);
}

#[test]
fn test_find_set_query_empty() {
    let mut editor = CodeEditor::default();
    editor.open_find();
    editor.set_find_query("");
    assert_eq!(editor.find_match_count(), 0);
}

#[test]
fn test_find_next_empty() {
    let mut editor = CodeEditor::default();
    editor.open_find();
    editor.find_next();
    assert_eq!(editor.cursor_position(), (0, 0));
    assert_eq!(editor.current_find_index(), 0);
}

#[test]
fn test_find_previous_empty() {
    let mut editor = CodeEditor::default();
    editor.open_find();
    editor.find_previous();
    assert_eq!(editor.cursor_position(), (0, 0));
    assert_eq!(editor.current_find_index(), 0);
}

#[test]
fn test_goto_line() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3\nline4");
    editor.goto_line(3);
    assert_eq!(editor.cursor_position().0, 2); // 0-indexed line 2
}

#[test]
fn test_goto_line_bounds() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.goto_line(100); // Beyond end
    assert_eq!(editor.cursor_position().0, 1); // Should go to last line

    editor.goto_line(0); // Line 0 treated as line 1
    assert_eq!(editor.cursor_position().0, 0);
}

#[test]
fn test_find_basic() {
    let mut editor = CodeEditor::new().content("hello world hello");
    editor.open_find();
    editor.set_find_query("hello");
    assert_eq!(editor.find_match_count(), 2);
}

#[test]
fn test_find_no_match() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.open_find();
    editor.set_find_query("foo");
    assert_eq!(editor.find_match_count(), 0);
}

#[test]
fn test_find_navigation() {
    let mut editor = CodeEditor::new().content("foo bar foo baz foo");
    editor.open_find();
    editor.set_find_query("foo");
    assert_eq!(editor.find_match_count(), 3);

    editor.find_next();
    editor.find_next();
    editor.find_previous();
}

#[test]
fn test_find_case_insensitive() {
    let mut editor = CodeEditor::new().content("Hello HELLO hello");
    editor.open_find();
    editor.set_find_query("hello");
    // Search is case-insensitive
    assert_eq!(editor.find_match_count(), 3);
}

#[test]
fn test_close_find() {
    let mut editor = CodeEditor::new().content("hello hello");
    editor.open_find();
    editor.set_find_query("hello");
    assert_eq!(editor.find_match_count(), 2);
    assert!(editor.is_find_active());

    editor.close_find();
    assert!(!editor.is_find_active());
}

// Found by tests/event_sequences.rs: the shrunk sequence was
// `<find 'n'> Backspace 'n' <set content> Enter` - the find matches were
// positions in the old text, so Enter jumped the cursor past the end of
// the new one.
#[test]
fn test_set_content_refreshes_the_find_matches() {
    use revue::event::Key;
    let mut editor = CodeEditor::new().content("fn main() {}");
    editor.open_find();
    editor.set_find_query("n");
    assert_eq!(editor.find_match_count(), 2);

    editor.set_content("x");
    assert_eq!(editor.find_match_count(), 0);
    editor.handle_key(&Key::Enter);
    assert_eq!(editor.cursor_position(), (0, 0));

    editor.set_content("a n b n");
    assert_eq!(editor.find_match_count(), 2);
}
