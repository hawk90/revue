//! CodeEditor widget tests

mod bracket;
mod edge_cases;
mod editing;
mod key_handling;
mod modes;
mod navigation;
mod render;
mod selection;
mod types;
mod vertical_scroll;
mod wide_chars;

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::code_editor;
use revue::widget::traits::RenderContext;
use revue::widget::traits::View;
use revue::widget::CodeEditor;
use revue::widget::Language;

// =========================================================================
// Creation and initialization tests
// =========================================================================

#[test]
fn test_code_editor_default() {
    let editor = CodeEditor::default();
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.cursor_position(), (0, 0));
}

// =========================================================================
// Text insertion tests
// =========================================================================

#[test]
fn test_code_editor_insert_char_multiline() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.insert_char('\n');
    editor.insert_char('b');
    assert_eq!(editor.line_count(), 2);
    assert_eq!(editor.get_content(), "a\nb");
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

// =========================================================================
// Search tests
// =========================================================================

#[test]
fn test_code_editor_find_next() {
    let mut editor = CodeEditor::new().content("hello\nhello\nhello");
    editor.open_find();
    editor.set_find_query("hello");
    assert_eq!(editor.find_match_count(), 3);
    assert_eq!(editor.current_find_index(), 1);

    editor.find_next();
    assert_eq!(editor.cursor_position(), (1, 0));
    assert_eq!(editor.current_find_index(), 2);

    editor.find_next();
    assert_eq!(editor.cursor_position(), (2, 0));

    // Wraps around to the first match
    editor.find_next();
    assert_eq!(editor.cursor_position(), (0, 0));
    assert_eq!(editor.current_find_index(), 1);
}

#[test]
fn test_code_editor_find_prev() {
    let mut editor = CodeEditor::new().content("hello\nhello\nhello");
    editor.open_find();
    editor.set_find_query("hello");

    // From the first match, previous wraps to the last one
    editor.find_previous();
    assert_eq!(editor.cursor_position(), (2, 0));
    assert_eq!(editor.current_find_index(), 3);

    editor.find_previous();
    assert_eq!(editor.cursor_position(), (1, 0));
}

// =========================================================================
// Bracket matching tests
// =========================================================================

#[test]
fn test_bracket_matching_nested() {
    let mut editor = CodeEditor::new().content("((test))");
    // Inner opening paren matches the inner closing paren, not the outer one
    editor.set_cursor(0, 1);
    let m = editor
        .find_matching_bracket()
        .expect("inner paren has a match");
    assert_eq!(m.position, (0, 6));
    assert_eq!(m.char, ')');
}

#[test]
fn test_bracket_matching_no_match() {
    let mut editor = CodeEditor::new().content("(test");
    editor.set_cursor(0, 0);
    let m = editor.find_matching_bracket();
    assert!(m.is_none());
}

// =========================================================================
// Syntax highlighting tests
// =========================================================================

/// Render `editor` without line numbers or cursor and return the
/// foreground colors of the first `n` cells of the first row.
fn first_row_fgs(editor: &CodeEditor, n: u16) -> Vec<Option<Color>> {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
    (0..n).map(|x| buffer.get(x, 0).unwrap().fg).collect()
}

#[test]
fn test_code_editor_syntax_highlighting() {
    let editor = CodeEditor::new()
        .content("fn main() {}")
        .language(Language::Rust)
        .line_numbers(false)
        .focused(false);
    let fgs = first_row_fgs(&editor, 4);
    // The `fn` keyword is colored differently from the identifier `main`
    assert_eq!(fgs[0], fgs[1]);
    assert_ne!(fgs[0], fgs[3]);
}

#[test]
fn test_code_editor_no_syntax_highlighting() {
    let editor = CodeEditor::new()
        .content("fn main() {}")
        .language(Language::None)
        .line_numbers(false)
        .focused(false);
    let fgs = first_row_fgs(&editor, 4);
    // Without a language, keyword and identifier share one color
    assert_eq!(fgs[0], fgs[3]);
}

// =========================================================================
// Auto indent tests
// =========================================================================

#[test]
fn test_code_editor_auto_indent() {
    let mut editor = CodeEditor::new().content("fn main() {").auto_indent(true);
    editor.set_cursor(0, 11);
    editor.insert_char('\n');
    // A line ending in `{` indents the next line by one level
    assert_eq!(editor.get_line(1).as_deref(), Some("    "));
    assert_eq!(editor.cursor_position(), (1, 4));
}

#[test]
fn test_code_editor_auto_indent_keeps_leading_whitespace() {
    let mut editor = CodeEditor::new()
        .content("    let x = 1;")
        .auto_indent(true);
    editor.set_cursor(0, 14);
    editor.insert_char('\n');
    assert_eq!(editor.get_line(1).as_deref(), Some("    "));
    assert_eq!(editor.cursor_position(), (1, 4));
}

#[test]
fn test_code_editor_no_auto_indent() {
    let mut editor = CodeEditor::new().content("fn main() {").auto_indent(false);
    editor.set_cursor(0, 11);
    editor.insert_char('\n');
    assert_eq!(editor.get_line(1).as_deref(), Some(""));
    assert_eq!(editor.cursor_position(), (1, 0));
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
fn test_code_editor_new() {
    let editor = CodeEditor::new();
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.cursor_position(), (0, 0));
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_code_editor_constructor() {
    let editor = code_editor().content("hello");
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_code_editor_content() {
    let editor = CodeEditor::new().content("line1\nline2\nline3");
    assert_eq!(editor.line_count(), 3);
    assert_eq!(editor.get_content(), "line1\nline2\nline3");
}

#[test]
fn test_code_editor_set_content() {
    let mut editor = CodeEditor::new();
    editor.set_content("new content");
    assert_eq!(editor.get_content(), "new content");
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_code_editor_multiline_content() {
    let code = "fn main() {\n    println!(\"Hello\");\n}";
    let editor = CodeEditor::new().content(code);
    assert_eq!(editor.line_count(), 3);
}
