//! CodeEditor widget integration tests
//!
//! Extracted from src/widget/developer/code_editor/tests.rs

use revue::widget::developer::code_editor::{CodeEditor, EditorConfig};
use revue::widget::syntax::Language;

// =========================================================================
// Creation and initialization tests
// =========================================================================

#[test]
fn test_code_editor_new() {
    let editor = CodeEditor::new();
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_code_editor_default() {
    let editor = CodeEditor::default();
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_code_editor_content() {
    let editor = CodeEditor::new().content("Hello\nWorld");
    assert_eq!(editor.line_count(), 2);
    assert_eq!(editor.get_content(), "Hello\nWorld");
}

// =========================================================================
// Text insertion tests
// =========================================================================

#[test]
fn test_code_editor_insert_char() {
    let mut editor = CodeEditor::new();
    editor.insert_char('H');
    editor.insert_char('i');
    assert_eq!(editor.get_content(), "Hi");
}

#[test]
fn test_code_editor_insert_char_multiline() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.insert_char('\n');
    editor.insert_char('b');
    assert_eq!(editor.line_count(), 2);
    assert_eq!(editor.get_content(), "a\nb");
}

#[test]
fn test_code_editor_insert_char_in_middle() {
    let mut editor = CodeEditor::new().content("ac");
    editor.set_cursor(0, 1);
    editor.insert_char('b');
    assert_eq!(editor.get_content(), "abc");
}

#[test]
fn test_code_editor_insert_tab() {
    let mut editor = CodeEditor::new();
    editor.insert_char('\t');
    assert!(editor.get_content().len() > 0);
}

// =========================================================================
// Deletion tests
// =========================================================================

#[test]
fn test_code_editor_delete_char_before() {
    let mut editor = CodeEditor::new().content("abc");
    editor.set_cursor(0, 2);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "ac");
}

#[test]
fn test_code_editor_delete_char_at() {
    let mut editor = CodeEditor::new().content("abc");
    editor.set_cursor(0, 1);
    editor.delete_char_at();
    assert_eq!(editor.get_content(), "ac");
}

#[test]
fn test_code_editor_delete_line() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.delete_line(1);
    assert_eq!(editor.line_count(), 2);
    assert_eq!(editor.get_content(), "line1\nline3");
}

// =========================================================================
// Movement tests
// =========================================================================

#[test]
fn test_code_editor_movement() {
    let mut editor = CodeEditor::new().content("Hello\nWorld");
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 1));
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 1));
    editor.move_left();
    assert_eq!(editor.cursor_position(), (1, 0));
    editor.move_up();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_code_editor_move_home() {
    let mut editor = CodeEditor::new().content("  hello");
    editor.set_cursor(0, 5);
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_code_editor_move_end() {
    let mut editor = CodeEditor::new().content("hello");
    editor.move_end();
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_code_editor_move_word_forward() {
    let mut editor = CodeEditor::new().content("hello world");
    let before = editor.cursor_position();
    editor.move_word_right();
    assert_ne!(editor.cursor_position(), before);
}

#[test]
fn test_code_editor_move_word_back() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 11);
    editor.move_word_left();
    assert!(editor.cursor_position().1 < 11);
}

#[test]
fn test_code_editor_page_down() {
    let mut editor = CodeEditor::new();
    for i in 0..30 {
        editor.insert_char('\n');
        editor.insert_char((b'a' + (i % 26)) as char);
    }
    let initial = editor.cursor_position();
    editor.page_down(20);
    assert!(editor.cursor_position().0 > initial.0);
}

#[test]
fn test_code_editor_page_up() {
    let mut editor = CodeEditor::new();
    for i in 0..30 {
        editor.insert_char('\n');
        editor.insert_char((b'a' + (i % 26)) as char);
    }
    editor.set_cursor(25, 0);
    let initial = editor.cursor_position();
    editor.page_up(20);
    assert!(editor.cursor_position().0 < initial.0);
}

// =========================================================================
// Selection tests
// =========================================================================

#[test]
fn test_code_editor_select_none() {
    let editor = CodeEditor::new();
    assert!(!editor.has_selection());
}

#[test]
fn test_code_editor_selected_text() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.start_selection();
    editor.set_cursor(0, 5);
    let text = editor.get_selection();
    assert!(text.is_some());
}

#[test]
fn test_code_editor_delete_selection() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.start_selection();
    editor.set_cursor(0, 5);
    editor.delete_selection();
    assert_eq!(editor.get_content(), " world");
}

// =========================================================================
// Search tests
// =========================================================================

#[test]
fn test_code_editor_find_next() {
    let mut editor = CodeEditor::new().content("hello\nhello\nhello");
    editor.find_next("hello");
    let first = editor.cursor_position();
    editor.find_next("hello");
    assert_ne!(editor.cursor_position(), first);
}

#[test]
fn test_code_editor_find_prev() {
    let mut editor = CodeEditor::new().content("hello\nhello\nhello");
    editor.set_cursor(2, 0);
    editor.find_previous("hello");
    assert_eq!(editor.cursor_position().0, 1);
}

// =========================================================================
// Undo/Redo tests
// =========================================================================

#[test]
fn test_code_editor_undo() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.insert_char('b');
    editor.undo();
    assert_eq!(editor.get_content(), "a");
}

#[test]
fn test_code_editor_redo() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.insert_char('b');
    editor.undo();
    editor.redo();
    assert_eq!(editor.get_content(), "ab");
}

// =========================================================================
// Bracket matching tests
// =========================================================================

#[test]
fn test_bracket_matching() {
    let editor = CodeEditor::new()
        .content("fn main() {}")
        .bracket_matching(true);
    let mut ed = editor;
    ed.set_cursor(0, 10);
    let m = ed.find_matching_bracket();
    assert!(m.is_some());
}

#[test]
fn test_bracket_matching_parentheses() {
    let mut editor = CodeEditor::new().content("(test)");
    editor.set_cursor(0, 0);
    let m = editor.find_matching_bracket();
    assert!(m.is_some());
}

#[test]
fn test_bracket_matching_square() {
    let mut editor = CodeEditor::new().content("[test]");
    editor.set_cursor(0, 0);
    let m = editor.find_matching_bracket();
    assert!(m.is_some());
}

#[test]
fn test_bracket_matching_curly() {
    let mut editor = CodeEditor::new().content("{test}");
    editor.set_cursor(0, 0);
    let m = editor.find_matching_bracket();
    assert!(m.is_some());
}

#[test]
fn test_bracket_matching_nested() {
    let mut editor = CodeEditor::new().content("((test))");
    editor.set_cursor(0, 1);
    let m = editor.find_matching_bracket();
    assert!(m.is_some());
}

#[test]
fn test_bracket_matching_no_match() {
    let mut editor = CodeEditor::new().content("(test");
    editor.set_cursor(0, 0);
    let m = editor.find_matching_bracket();
    assert!(m.is_none());
}

// =========================================================================
// Goto line tests
// =========================================================================

#[test]
fn test_code_editor_goto_line() {
    let mut editor = CodeEditor::new().content("1\n2\n3\n4\n5");
    editor.goto_line(3);
    assert_eq!(editor.cursor_position().0, 2);
}

#[test]
fn test_code_editor_goto_line_out_of_bounds() {
    let mut editor = CodeEditor::new().content("1\n2\n3");
    editor.goto_line(100);
    assert!(editor.cursor_position().0 < 3);
}

#[test]
fn test_code_editor_open_goto_line() {
    let mut editor = CodeEditor::new();
    editor.open_goto_line();
    assert!(editor.is_goto_line_active());
}

#[test]
fn test_code_editor_close_goto_line() {
    let mut editor = CodeEditor::new();
    editor.open_goto_line();
    editor.close_goto_line();
    assert!(!editor.is_goto_line_active());
}

// =========================================================================
// Syntax highlighting tests
// =========================================================================

#[test]
fn test_code_editor_syntax_highlighting() {
    let _editor = CodeEditor::new()
        .content("fn main() {}")
        .language(Language::Rust)
        .auto_indent(true);
    // Syntax highlighting is applied internally
}

#[test]
fn test_code_editor_no_syntax_highlighting() {
    let _editor = CodeEditor::new()
        .content("fn main() {}")
        .language(Language::None);
    // No syntax highlighting
}

// =========================================================================
// Auto indent tests
// =========================================================================

#[test]
fn test_code_editor_auto_indent() {
    let mut editor = CodeEditor::new()
        .content("fn main() {")
        .auto_indent(true);
    editor.insert_char('\n');
    // Should auto-indent
}

#[test]
fn test_code_editor_no_auto_indent() {
    let mut editor = CodeEditor::new()
        .content("fn main() {")
        .auto_indent(false);
    editor.insert_char('\n');
    // Should not auto-indent
}

// =========================================================================
// Content operations tests
// =========================================================================

#[test]
fn test_code_editor_clear() {
    let mut editor = CodeEditor::new().content("some content");
    editor.set_content("");
    assert_eq!(editor.get_content(), "");
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_code_editor_set_content() {
    let mut editor = CodeEditor::new();
    editor.set_content("new content");
    assert_eq!(editor.get_content(), "new content");
}

// =========================================================================
// Cursor position tests
// =========================================================================

#[test]
fn test_code_editor_set_cursor() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.set_cursor(1, 3);
    assert_eq!(editor.cursor_position(), (1, 3));
}

#[test]
fn test_code_editor_set_cursor_clamped() {
    let mut editor = CodeEditor::new().content("short");
    editor.set_cursor(0, 100);
    assert!(editor.cursor_position().1 <= 5);
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_code_editor_config() {
    let config = EditorConfig::default();
    let _editor = CodeEditor::new().config(config);
}
