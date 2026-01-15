//! Tests for CodeEditor widget
//!
//! These tests use only the public API of the widget.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{code_editor, CodeEditor, EditorConfig, IndentStyle};

// =============================================================================
// Basic Creation and Content Tests
// =============================================================================

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

// =============================================================================
// Cursor Navigation Tests
// =============================================================================

#[test]
fn test_cursor_move_right() {
    let mut editor = CodeEditor::new().content("hello");
    assert_eq!(editor.cursor_position(), (0, 0));
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 1));
    editor.move_right();
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 3));
}

#[test]
fn test_cursor_move_left() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 3);
    editor.move_left();
    assert_eq!(editor.cursor_position(), (0, 2));
    editor.move_left();
    editor.move_left();
    editor.move_left();
    // Should stop at 0
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_move_down() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 0));
    editor.move_down();
    assert_eq!(editor.cursor_position(), (2, 0));
    // Should stop at last line
    editor.move_down();
    assert_eq!(editor.cursor_position(), (2, 0));
}

#[test]
fn test_cursor_move_up() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(2, 0);
    editor.move_up();
    assert_eq!(editor.cursor_position(), (1, 0));
    editor.move_up();
    assert_eq!(editor.cursor_position(), (0, 0));
    // Should stop at first line
    editor.move_up();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_move_home() {
    let mut editor = CodeEditor::new().content("    hello");
    editor.set_cursor(0, 8);
    // First home goes to first non-whitespace (column 4)
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 4));
    // Second home goes to column 0
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_move_end() {
    let mut editor = CodeEditor::new().content("hello");
    editor.move_end();
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_cursor_document_navigation() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3\nline4");
    editor.move_document_end();
    assert_eq!(editor.cursor_position(), (3, 5));
    editor.move_document_start();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_word_navigation() {
    let mut editor = CodeEditor::new().content("hello world foo");
    editor.move_word_right();
    assert_eq!(editor.cursor_position(), (0, 6)); // After "hello "
    editor.move_word_right();
    assert_eq!(editor.cursor_position(), (0, 12)); // After "world "

    editor.move_word_left();
    assert_eq!(editor.cursor_position(), (0, 6));
}

#[test]
fn test_cursor_page_navigation() {
    let mut editor = CodeEditor::new().content("1\n2\n3\n4\n5\n6\n7\n8\n9\n10");
    editor.page_down(5);
    assert_eq!(editor.cursor_position(), (5, 0));
    editor.page_up(3);
    assert_eq!(editor.cursor_position(), (2, 0));
}

#[test]
fn test_cursor_wrap_between_lines() {
    let mut editor = CodeEditor::new().content("ab\ncd");
    editor.set_cursor(0, 2);
    editor.move_right();
    // Should wrap to next line
    assert_eq!(editor.cursor_position(), (1, 0));

    editor.move_left();
    // Should wrap back to previous line
    assert_eq!(editor.cursor_position(), (0, 2));
}

// =============================================================================
// Selection Tests
// =============================================================================

#[test]
fn test_selection_basic() {
    let mut editor = CodeEditor::new().content("hello world");
    assert!(!editor.has_selection());

    editor.start_selection();
    assert!(editor.has_selection());

    editor.clear_selection();
    assert!(!editor.has_selection());
}

#[test]
fn test_get_selection() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 0);
    editor.start_selection();
    editor.set_cursor(0, 5);
    // Selection from (0,0) to (0,5)
    // Note: get_selection returns text between anchor and cursor
    let sel = editor.get_selection();
    assert!(sel.is_some());
    assert_eq!(sel.unwrap(), "hello");
}

// =============================================================================
// Text Editing Tests
// =============================================================================

#[test]
fn test_insert_char() {
    let mut editor = CodeEditor::new().content("hllo");
    editor.set_cursor(0, 1);
    editor.insert_char('e');
    assert_eq!(editor.get_content(), "hello");
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_insert_str() {
    let mut editor = CodeEditor::new().content("hd");
    editor.set_cursor(0, 1);
    editor.insert_str("ello worl");
    assert_eq!(editor.get_content(), "hello world");
}

#[test]
fn test_delete_char_before() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "hell");
}

#[test]
fn test_delete_char_at() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 0);
    editor.delete_char_at();
    assert_eq!(editor.get_content(), "ello");
}

#[test]
fn test_delete_line() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(1, 0);
    editor.delete_line();
    assert_eq!(editor.get_content(), "line1\nline3");
    assert_eq!(editor.line_count(), 2);
}

#[test]
fn test_newline_insertion() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 5);
    editor.insert_char('\n');
    assert_eq!(editor.line_count(), 2);
}

#[test]
fn test_delete_selection() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 0);
    editor.start_selection();
    editor.set_cursor(0, 6);
    editor.delete_selection();
    assert_eq!(editor.get_content(), "world");
}

#[test]
fn test_read_only_mode() {
    let editor = CodeEditor::new().content("hello").read_only(true);
    // In read-only mode, inserts should be ignored by the handle_key method
    // But insert_char itself doesn't check read_only (that's done in handle_key)
    assert_eq!(editor.get_content(), "hello");
}

// =============================================================================
// Undo/Redo Tests
// =============================================================================

#[test]
fn test_undo_insert() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.insert_char('!');
    assert_eq!(editor.get_content(), "hello!");

    editor.undo();
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_redo() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.insert_char('!');
    editor.undo();
    assert_eq!(editor.get_content(), "hello");

    editor.redo();
    assert_eq!(editor.get_content(), "hello!");
}

#[test]
fn test_undo_delete() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "hell");

    editor.undo();
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_multiple_undo() {
    let mut editor = CodeEditor::new().content("a");
    editor.set_cursor(0, 1);
    editor.insert_char('b');
    editor.insert_char('c');
    assert_eq!(editor.get_content(), "abc");

    editor.undo();
    assert_eq!(editor.get_content(), "ab");
    editor.undo();
    assert_eq!(editor.get_content(), "a");
}

// =============================================================================
// Bracket Matching Tests
// =============================================================================

#[test]
fn test_bracket_matching_paren() {
    let mut editor = CodeEditor::new().content("(hello)").bracket_matching(true);
    editor.set_cursor(0, 0); // On opening paren
    let bracket_match = editor.find_matching_bracket();
    assert!(bracket_match.is_some());
    let m = bracket_match.unwrap();
    assert_eq!(m.position, (0, 6)); // Closing paren position
    assert_eq!(m.char, ')');
}

#[test]
fn test_bracket_matching_curly() {
    let mut editor = CodeEditor::new().content("{ foo }").bracket_matching(true);
    editor.set_cursor(0, 0);
    let bracket_match = editor.find_matching_bracket();
    assert!(bracket_match.is_some());
    assert_eq!(bracket_match.unwrap().char, '}');
}

#[test]
fn test_bracket_matching_square() {
    let mut editor = CodeEditor::new()
        .content("[1, 2, 3]")
        .bracket_matching(true);
    editor.set_cursor(0, 0);
    let bracket_match = editor.find_matching_bracket();
    assert!(bracket_match.is_some());
    assert_eq!(bracket_match.unwrap().char, ']');
}

#[test]
fn test_bracket_matching_nested() {
    let mut editor = CodeEditor::new()
        .content("((inner))")
        .bracket_matching(true);
    editor.set_cursor(0, 0); // First opening paren
    let bracket_match = editor.find_matching_bracket();
    assert!(bracket_match.is_some());
    assert_eq!(bracket_match.unwrap().position, (0, 8)); // Outer closing paren
}

#[test]
fn test_bracket_matching_from_close() {
    let mut editor = CodeEditor::new().content("(hello)").bracket_matching(true);
    editor.set_cursor(0, 6); // On closing paren
    let bracket_match = editor.find_matching_bracket();
    assert!(bracket_match.is_some());
    assert_eq!(bracket_match.unwrap().position, (0, 0)); // Opening paren
    assert_eq!(bracket_match.unwrap().char, '(');
}

#[test]
fn test_bracket_matching_multiline() {
    let mut editor = CodeEditor::new()
        .content("{\n    foo\n}")
        .bracket_matching(true);
    editor.set_cursor(0, 0);
    let bracket_match = editor.find_matching_bracket();
    assert!(bracket_match.is_some());
    assert_eq!(bracket_match.unwrap().position, (2, 0));
}

#[test]
fn test_auto_close_bracket() {
    let mut editor = CodeEditor::new().bracket_matching(true);
    editor.insert_char('(');
    // Auto-close should insert matching bracket
    assert_eq!(editor.get_content(), "()");
}

// =============================================================================
// Go-to-line Tests
// =============================================================================

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

// =============================================================================
// Find Tests
// =============================================================================

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

// =============================================================================
// Configuration Tests
// =============================================================================

#[test]
fn test_editor_config_default() {
    let config = EditorConfig::default();
    assert_eq!(config.indent_style, IndentStyle::Spaces);
    assert_eq!(config.indent_size, 4);
    assert!(config.auto_indent);
    assert!(config.bracket_matching);
    assert!(config.highlight_current_line);
    assert!(!config.show_minimap);
}

#[test]
fn test_editor_config_custom() {
    let config = EditorConfig {
        indent_style: IndentStyle::Tabs,
        indent_size: 2,
        auto_indent: false,
        bracket_matching: false,
        highlight_current_line: false,
        show_minimap: true,
        minimap_width: 15,
        show_whitespace: true,
        word_wrap: true,
    };

    let editor = CodeEditor::new().config(config);
    // Config is applied - we can't easily verify internals but it shouldn't panic
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_builder_methods() {
    let editor = CodeEditor::new()
        .content("code")
        .line_numbers(true)
        .read_only(false)
        .focused(true)
        .indent_size(2)
        .indent_style(IndentStyle::Tabs)
        .auto_indent(true)
        .bracket_matching(true)
        .highlight_current_line(true)
        .minimap(true);

    assert_eq!(editor.get_content(), "code");
}

// =============================================================================
// Language Detection Tests
// =============================================================================

#[test]
fn test_detect_language_rust() {
    let editor = CodeEditor::new()
        .content("fn main() {}")
        .detect_language("main.rs");
    // Language should be detected (can't easily verify, but shouldn't panic)
    assert_eq!(editor.get_content(), "fn main() {}");
}

#[test]
fn test_detect_language_javascript() {
    let editor = CodeEditor::new()
        .content("const x = 1;")
        .detect_language("app.js");
    assert_eq!(editor.get_content(), "const x = 1;");
}

#[test]
fn test_detect_language_python() {
    let editor = CodeEditor::new()
        .content("def foo(): pass")
        .detect_language("script.py");
    assert_eq!(editor.get_content(), "def foo(): pass");
}

// =============================================================================
// Rendering Tests
// =============================================================================

#[test]
fn test_render_empty() {
    let editor = CodeEditor::new();
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_render_with_content() {
    let editor = CodeEditor::new().content("hello\nworld");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_render_with_line_numbers() {
    let editor = CodeEditor::new().content("line1\nline2").line_numbers(true);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}

#[test]
fn test_render_with_minimap() {
    let editor = CodeEditor::new()
        .content("fn main() {\n    println!(\"Hello\");\n}")
        .minimap(true);
    let mut buffer = Buffer::new(60, 10);
    let area = Rect::new(0, 0, 60, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}

#[test]
fn test_render_with_selection() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.start_selection();
    editor.set_cursor(0, 5);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}

// =============================================================================
// Edge Cases
// =============================================================================

#[test]
fn test_empty_content() {
    let editor = CodeEditor::new().content("");
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_single_char_content() {
    let editor = CodeEditor::new().content("x");
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.get_content(), "x");
}

#[test]
fn test_unicode_content() {
    let editor = CodeEditor::new().content("你好世界");
    assert_eq!(editor.get_content(), "你好世界");
}

#[test]
fn test_cursor_clamp_on_content_change() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 11);
    editor.set_content("hi");
    // Cursor should be clamped to new content bounds
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_delete_at_start() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 0);
    editor.delete_char_before();
    // Should do nothing
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_delete_at_end() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.delete_char_at();
    // Should do nothing
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_merge_lines_on_backspace() {
    let mut editor = CodeEditor::new().content("hello\nworld");
    editor.set_cursor(1, 0);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "helloworld");
    assert_eq!(editor.line_count(), 1);
}
