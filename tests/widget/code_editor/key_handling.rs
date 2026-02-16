//! CodeEditor key handling tests
//!
//! Extracted from src/widget/developer/code_editor/key_handling.rs

use revue::event::Key;
use revue::widget::developer::code_editor::CodeEditor;

// =========================================================================
// handle_key tests
// =========================================================================

#[test]
fn test_handle_key_char() {
    let mut editor = CodeEditor::new();
    let handled = editor.handle_key(&Key::Char('a'));
    assert!(handled);
    assert_eq!(editor.get_content(), "a");
}

#[test]
fn test_handle_key_enter() {
    let mut editor = CodeEditor::new().content("line1");
    let handled = editor.handle_key(&Key::Enter);
    assert!(handled);
    assert_eq!(editor.line_count(), 2);
}

#[test]
fn test_handle_key_tab() {
    let mut editor = CodeEditor::new();
    let handled = editor.handle_key(&Key::Tab);
    assert!(handled);
    assert!(editor.get_content().len() > 0);
}

#[test]
fn test_handle_key_backspace() {
    let mut editor = CodeEditor::new().content("abc");
    editor.set_cursor(0, 2);
    let handled = editor.handle_key(&Key::Backspace);
    assert!(handled);
    assert_eq!(editor.get_content(), "ac");
}

#[test]
fn test_handle_key_delete() {
    let mut editor = CodeEditor::new().content("abc");
    let handled = editor.handle_key(&Key::Delete);
    assert!(handled);
    assert_eq!(editor.get_content(), "bc");
}

#[test]
fn test_handle_key_left() {
    let mut editor = CodeEditor::new().content("test");
    editor.set_cursor(0, 4);
    let handled = editor.handle_key(&Key::Left);
    assert!(handled);
    assert_eq!(editor.cursor_position(), (0, 3));
}

#[test]
fn test_handle_key_right() {
    let mut editor = CodeEditor::new().content("test");
    let handled = editor.handle_key(&Key::Right);
    assert!(handled);
    assert_eq!(editor.cursor_position(), (0, 1));
}

#[test]
fn test_handle_key_up() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.set_cursor(1, 2);
    let handled = editor.handle_key(&Key::Up);
    assert!(handled);
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_handle_key_down() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    let handled = editor.handle_key(&Key::Down);
    assert!(handled);
    assert_eq!(editor.cursor_position(), (1, 0));
}

#[test]
fn test_handle_key_home() {
    let mut editor = CodeEditor::new().content("  hello");
    editor.set_cursor(0, 5);
    let handled = editor.handle_key(&Key::Home);
    assert!(handled);
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_handle_key_end() {
    let mut editor = CodeEditor::new().content("hello");
    let handled = editor.handle_key(&Key::End);
    assert!(handled);
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_handle_key_page_up() {
    let mut editor = CodeEditor::new().content(
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25",
    );
    editor.set_cursor(24, 0);
    let handled = editor.handle_key(&Key::PageUp);
    assert!(handled);
    assert_eq!(editor.cursor_position(), (4, 0));
}

#[test]
fn test_handle_key_page_down() {
    let mut editor = CodeEditor::new().content(
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25",
    );
    let handled = editor.handle_key(&Key::PageDown);
    assert!(handled);
    assert_eq!(editor.cursor_position(), (20, 0));
}

#[test]
fn test_handle_key_unhandled() {
    let mut editor = CodeEditor::new();
    let handled = editor.handle_key(&Key::Null);
    assert!(!handled);
}

#[test]
fn test_handle_key_read_only() {
    let mut editor = CodeEditor::new().read_only(true);
    editor.handle_key(&Key::Char('a'));
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_handle_key_in_goto_mode() {
    let mut editor = CodeEditor::new();
    editor.open_goto_line();
    let handled = editor.handle_key(&Key::Char('5'));
    assert!(handled);
    assert!(editor.is_goto_line_active());
}

#[test]
fn test_handle_key_in_find_mode() {
    let mut editor = CodeEditor::new();
    editor.open_find();
    let handled = editor.handle_key(&Key::Char('a'));
    assert!(handled);
    assert!(editor.is_find_active());
}

#[test]
fn test_handle_key_escape_exits_goto() {
    let mut editor = CodeEditor::new();
    editor.open_goto_line();
    editor.handle_key(&Key::Escape);
    assert!(!editor.is_goto_line_active());
}

#[test]
fn test_handle_key_escape_exits_find() {
    let mut editor = CodeEditor::new();
    editor.open_find();
    editor.handle_key(&Key::Escape);
    assert!(!editor.is_find_active());
}
