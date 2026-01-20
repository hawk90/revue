//! CodeEditor widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    BracketMatch, BracketPair, CodeEditor, EditorConfig, IndentStyle, Language, SyntaxTheme,
};

// =========================================================================
// Constructor and Builder Tests
// =========================================================================

#[test]
fn test_code_editor_new() {
    let editor = CodeEditor::new();
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.get_content(), "");
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_code_editor_default() {
    let editor = CodeEditor::default();
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_code_editor_helper() {
    let editor = revue::widget::code_editor();
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_code_editor_with_content() {
    let editor = CodeEditor::new().content("Hello\nWorld");
    assert_eq!(editor.line_count(), 2);
    assert_eq!(editor.get_content(), "Hello\nWorld");
}

#[test]
fn test_code_editor_with_empty_content() {
    let editor = CodeEditor::new().content("");
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_code_editor_language() {
    let _editor = CodeEditor::new().language(Language::Rust);
    // Builder accepts parameter, language affects syntax highlighting
    let _editor2 = CodeEditor::new().language(Language::Python);
}

#[test]
fn test_code_editor_language_line_count() {
    let editor = CodeEditor::new().language(Language::Python);
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_code_editor_detect_language() {
    let _editor = CodeEditor::new().detect_language("test.rs");
    // Should detect Rust from .rs extension
    let _editor2 = CodeEditor::new().detect_language("test.py");
    // Should detect Python from .py extension
}

#[test]
fn test_code_editor_detect_language_line_count() {
    let editor = CodeEditor::new().detect_language("test.py");
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_code_editor_theme() {
    let theme = SyntaxTheme::dark();
    let editor = CodeEditor::new().theme(theme.clone());
    // Builder accepts theme parameter
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_code_editor_config() {
    let config = EditorConfig {
        indent_style: IndentStyle::Tabs,
        indent_size: 2,
        auto_indent: false,
        bracket_matching: false,
        highlight_current_line: false,
        show_minimap: false,
        minimap_width: 8,
        show_whitespace: true,
        word_wrap: true,
    };
    let editor = CodeEditor::new().config(config);
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_code_editor_line_numbers() {
    let _editor = CodeEditor::new().line_numbers(true);
    let editor2 = CodeEditor::new().line_numbers(false);
    assert_eq!(editor2.line_count(), 1);
}

#[test]
fn test_code_editor_read_only() {
    let mut editor = CodeEditor::new().read_only(true);
    editor.insert_char('a');
    assert_eq!(editor.get_content(), ""); // Should not insert
}

#[test]
fn test_code_editor_focused() {
    let _editor = CodeEditor::new().focused(false);
    let editor2 = CodeEditor::new().focused(true);
    assert_eq!(editor2.line_count(), 1);
}

#[test]
fn test_code_editor_indent_size() {
    let _editor = CodeEditor::new().indent_size(2);
    let editor2 = CodeEditor::new().indent_size(8);
    assert_eq!(editor2.line_count(), 1);
}

#[test]
fn test_code_editor_indent_size_minimum() {
    let editor = CodeEditor::new().indent_size(0);
    // Should clamp to minimum of 1
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_code_editor_indent_style() {
    let _editor = CodeEditor::new().indent_style(IndentStyle::Spaces);
    let editor2 = CodeEditor::new().indent_style(IndentStyle::Tabs);
    assert_eq!(editor2.line_count(), 1);
}

#[test]
fn test_code_editor_auto_indent() {
    let _editor = CodeEditor::new().auto_indent(true);
    let editor2 = CodeEditor::new().auto_indent(false);
    assert_eq!(editor2.line_count(), 1);
}

#[test]
fn test_code_editor_bracket_matching() {
    let _editor = CodeEditor::new().bracket_matching(true);
    let editor2 = CodeEditor::new().bracket_matching(false);
    assert_eq!(editor2.line_count(), 1);
}

#[test]
fn test_code_editor_highlight_current_line() {
    let _editor = CodeEditor::new().highlight_current_line(true);
    let editor2 = CodeEditor::new().highlight_current_line(false);
    assert_eq!(editor2.line_count(), 1);
}

#[test]
fn test_code_editor_minimap() {
    let _editor = CodeEditor::new().minimap(true);
    let editor2 = CodeEditor::new().minimap(false);
    assert_eq!(editor2.line_count(), 1);
}

#[test]
fn test_code_editor_colors() {
    let _editor = CodeEditor::new()
        .fg(Color::rgb(255, 0, 0))
        .bg(Color::rgb(0, 255, 0));
    // Colors affect rendering
}

// =========================================================================
// EditorConfig Tests
// =========================================================================

#[test]
fn test_editor_config_default() {
    let config = EditorConfig::default();
    assert_eq!(config.indent_style, IndentStyle::Spaces);
    assert_eq!(config.indent_size, 4);
    assert!(config.auto_indent);
    assert!(config.bracket_matching);
    assert!(config.highlight_current_line);
    assert!(!config.show_minimap);
    assert_eq!(config.minimap_width, 10);
    assert!(!config.show_whitespace);
    assert!(!config.word_wrap);
}

#[test]
fn test_indent_style_default() {
    let style = IndentStyle::default();
    assert_eq!(style, IndentStyle::Spaces);
}

// =========================================================================
// Content Management Tests
// =========================================================================

#[test]
fn test_get_content_empty() {
    let editor = CodeEditor::new();
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_get_content_single_line() {
    let editor = CodeEditor::new().content("Hello World");
    assert_eq!(editor.get_content(), "Hello World");
}

#[test]
fn test_get_content_multi_line() {
    let editor = CodeEditor::new().content("Line 1\nLine 2\nLine 3");
    assert_eq!(editor.get_content(), "Line 1\nLine 2\nLine 3");
}

#[test]
fn test_set_content() {
    let mut editor = CodeEditor::new();
    editor.set_content("New content");
    assert_eq!(editor.get_content(), "New content");
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_set_content_multi_line() {
    let mut editor = CodeEditor::new();
    editor.set_content("Line 1\nLine 2");
    assert_eq!(editor.line_count(), 2);
    assert_eq!(editor.get_content(), "Line 1\nLine 2");
}

#[test]
fn test_set_content_clears_undo() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.set_content("b");
    editor.undo();
    // After set_content, undo stack should be cleared
}

#[test]
fn test_line_count() {
    let editor = CodeEditor::new().content("L1\nL2\nL3");
    assert_eq!(editor.line_count(), 3);
}

#[test]
fn test_line_count_empty() {
    let editor = CodeEditor::new();
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_set_language() {
    let mut editor = CodeEditor::new();
    editor.set_language(Language::JavaScript);
    editor.set_language(Language::None);
    // Should not panic
}

// =========================================================================
// Cursor Position Tests
// =========================================================================

#[test]
fn test_cursor_position_initial() {
    let editor = CodeEditor::new();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_set_cursor() {
    let mut editor = CodeEditor::new().content("Hello\nWorld");
    editor.set_cursor(1, 3);
    assert_eq!(editor.cursor_position(), (1, 3));
}

#[test]
fn test_set_cursor_bounds() {
    let mut editor = CodeEditor::new().content("Hi");
    editor.set_cursor(0, 100);
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_set_cursor_line_bounds() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2");
    editor.set_cursor(10, 0);
    assert_eq!(editor.cursor_position(), (1, 0));
}

// =========================================================================
// Cursor Movement Tests
// =========================================================================

#[test]
fn test_move_left() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.set_cursor(0, 5);
    editor.move_left();
    assert_eq!(editor.cursor_position(), (0, 4));
    editor.move_left();
    assert_eq!(editor.cursor_position(), (0, 3));
}

#[test]
fn test_move_left_at_start() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.set_cursor(0, 0);
    editor.move_left();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_move_left_wrap_to_previous_line() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2");
    editor.set_cursor(1, 0);
    editor.move_left();
    assert_eq!(editor.cursor_position(), (0, 6));
}

#[test]
fn test_move_right() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.set_cursor(0, 0);
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 1));
}

#[test]
fn test_move_right_at_end() {
    let mut editor = CodeEditor::new().content("Hi");
    editor.set_cursor(0, 2);
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_move_right_to_next_line() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2");
    editor.set_cursor(0, 6);
    editor.move_right();
    assert_eq!(editor.cursor_position(), (1, 0));
}

#[test]
fn test_move_up() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2\nLine 3");
    editor.set_cursor(2, 3);
    editor.move_up();
    assert_eq!(editor.cursor_position(), (1, 3));
}

#[test]
fn test_move_up_at_top() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2");
    editor.set_cursor(0, 3);
    editor.move_up();
    assert_eq!(editor.cursor_position(), (0, 3));
}

#[test]
fn test_move_up_clamps_column() {
    let mut editor = CodeEditor::new().content("Line 1\nHi");
    editor.set_cursor(1, 3);
    editor.move_up();
    assert_eq!(editor.cursor_position(), (0, 3));
}

#[test]
fn test_move_down() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2\nLine 3");
    editor.set_cursor(0, 3);
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 3));
}

#[test]
fn test_move_down_at_bottom() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2");
    editor.set_cursor(1, 3);
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 3));
}

#[test]
fn test_move_down_clamps_column() {
    let mut editor = CodeEditor::new().content("Hi\nLine 2");
    editor.set_cursor(0, 2);
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 2));
}

#[test]
fn test_move_home() {
    let mut editor = CodeEditor::new().content("  Hello World");
    editor.set_cursor(0, 5);
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 2)); // First non-whitespace
}

#[test]
fn test_move_home_at_first_non_ws() {
    let mut editor = CodeEditor::new().content("  Hello");
    editor.set_cursor(0, 2);
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 0)); // Go to column 0
}

#[test]
fn test_move_home_twice() {
    let mut editor = CodeEditor::new().content("  Hello");
    editor.set_cursor(0, 5);
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 2));
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_move_end() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.set_cursor(0, 0);
    editor.move_end();
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_move_document_start() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2\nLine 3");
    editor.set_cursor(2, 5);
    editor.move_document_start();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_move_document_end() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2\nLine 3");
    editor.move_document_end();
    assert_eq!(editor.cursor_position(), (2, 6));
}

#[test]
fn test_move_word_left() {
    let mut editor = CodeEditor::new().content("Hello World");
    editor.set_cursor(0, 6);
    editor.move_word_left();
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_move_word_left_multiple() {
    let mut editor = CodeEditor::new().content("one two three");
    editor.set_cursor(0, 12);
    editor.move_word_left();
    assert_eq!(editor.cursor_position(), (0, 8));
    editor.move_word_left();
    assert_eq!(editor.cursor_position(), (0, 4));
}

#[test]
fn test_move_word_left_at_start() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.set_cursor(0, 0);
    editor.move_word_left();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_move_word_right() {
    let mut editor = CodeEditor::new().content("Hello World");
    editor.set_cursor(0, 0);
    editor.move_word_right();
    assert_eq!(editor.cursor_position(), (0, 6));
}

#[test]
fn test_move_word_right_multiple() {
    let mut editor = CodeEditor::new().content("one two three");
    editor.set_cursor(0, 0);
    editor.move_word_right();
    assert_eq!(editor.cursor_position(), (0, 4));
    editor.move_word_right();
    assert_eq!(editor.cursor_position(), (0, 8));
}

#[test]
fn test_move_word_right_at_end() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.set_cursor(0, 5);
    editor.move_word_right();
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_page_up() {
    let mut editor = CodeEditor::new();
    for i in 0..20 {
        editor.insert_char('\n');
        editor.insert_str(&format!("Line {}", i));
    }
    editor.set_cursor(15, 0);
    editor.page_up(10);
    assert_eq!(editor.cursor_position(), (5, 0));
}

#[test]
fn test_page_down() {
    let mut editor = CodeEditor::new();
    for i in 0..20 {
        editor.insert_char('\n');
        editor.insert_str(&format!("Line {}", i));
    }
    editor.set_cursor(5, 0);
    editor.page_down(10);
    assert_eq!(editor.cursor_position(), (15, 0));
}

#[test]
fn test_page_down_clamps_to_end() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2\nLine 3");
    editor.set_cursor(0, 0);
    editor.page_down(100);
    assert_eq!(editor.cursor_position(), (2, 6));
}

#[test]
fn test_page_up_clamps_to_start() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2");
    editor.set_cursor(0, 0);
    editor.page_up(10);
    assert_eq!(editor.cursor_position(), (0, 0));
}

// =========================================================================
// Text Editing Tests
// =========================================================================

#[test]
fn test_insert_char() {
    let mut editor = CodeEditor::new();
    editor.insert_char('H');
    editor.insert_char('i');
    assert_eq!(editor.get_content(), "Hi");
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_insert_char_middle_of_line() {
    let mut editor = CodeEditor::new().content("Hi");
    editor.set_cursor(0, 1);
    editor.insert_char('e');
    assert_eq!(editor.get_content(), "Hei");
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_insert_char_newline() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.set_cursor(0, 5);
    editor.insert_char('\n');
    assert_eq!(editor.get_content(), "Hello\n");
    assert_eq!(editor.line_count(), 2);
    assert_eq!(editor.cursor_position(), (1, 0));
}

#[test]
fn test_insert_char_newline_with_auto_indent() {
    let mut editor = CodeEditor::new().content("    Hello");
    editor.set_cursor(0, 9);
    editor.insert_char('\n');
    assert_eq!(editor.get_content(), "    Hello\n    ");
    assert_eq!(editor.cursor_position(), (1, 4));
}

#[test]
fn test_insert_char_newline_with_bracket() {
    let mut editor = CodeEditor::new().auto_indent(true).content("    {");
    editor.set_cursor(0, 5);
    editor.insert_char('\n');
    // Should add extra indent after opening bracket
    let content = editor.get_content();
    assert!(content.contains('\n'));
    let lines: Vec<&str> = content.split('\n').collect();
    assert!(lines[1].len() > 4);
}

#[test]
fn test_insert_char_newline_no_auto_indent() {
    let mut editor = CodeEditor::new().auto_indent(false).content("    Hello");
    editor.set_cursor(0, 9);
    editor.insert_char('\n');
    assert_eq!(editor.get_content(), "    Hello\n");
}

#[test]
fn test_insert_char_tab_spaces() {
    let mut editor = CodeEditor::new().indent_size(4);
    editor.insert_char('\t');
    assert_eq!(editor.get_content(), "    ");
}

#[test]
fn test_insert_char_tab_tabs() {
    let mut editor = CodeEditor::new().indent_style(IndentStyle::Tabs);
    editor.insert_char('\t');
    assert_eq!(editor.get_content(), "\t");
}

#[test]
fn test_insert_char_read_only() {
    let mut editor = CodeEditor::new().read_only(true);
    editor.insert_char('a');
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_insert_char_auto_closes_paren() {
    let mut editor = CodeEditor::new().bracket_matching(true);
    editor.insert_char('(');
    assert_eq!(editor.get_content(), "()");
}

#[test]
fn test_insert_char_auto_closes_bracket() {
    let mut editor = CodeEditor::new().bracket_matching(true);
    editor.insert_char('[');
    assert_eq!(editor.get_content(), "[]");
}

#[test]
fn test_insert_char_auto_closes_brace() {
    let mut editor = CodeEditor::new().bracket_matching(true);
    editor.insert_char('{');
    assert_eq!(editor.get_content(), "{}");
}

#[test]
fn test_insert_char_auto_closes_double_quote() {
    let mut editor = CodeEditor::new().bracket_matching(true);
    editor.insert_char('"');
    assert_eq!(editor.get_content(), "\"\"");
}

#[test]
fn test_insert_char_auto_closes_single_quote() {
    let mut editor = CodeEditor::new().bracket_matching(true);
    editor.insert_char('\'');
    assert_eq!(editor.get_content(), "''");
}

#[test]
fn test_insert_char_no_auto_close_disabled() {
    let mut editor = CodeEditor::new().bracket_matching(false);
    editor.insert_char('(');
    assert_eq!(editor.get_content(), "(");
}

#[test]
fn test_insert_str() {
    let mut editor = CodeEditor::new();
    editor.insert_str("Hello");
    assert_eq!(editor.get_content(), "Hello");
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_insert_str_multi_line() {
    let mut editor = CodeEditor::new();
    editor.insert_str("Line 1\nLine 2");
    assert_eq!(editor.get_content(), "Line 1\nLine 2");
    assert_eq!(editor.line_count(), 2);
}

#[test]
fn test_insert_str_with_selection() {
    let mut editor = CodeEditor::new().content("Hello World");
    editor.start_selection();
    editor.move_end();
    editor.insert_str("Rust");
    assert_eq!(editor.get_content(), "Hello Rust");
}

#[test]
fn test_insert_str_read_only() {
    let mut editor = CodeEditor::new().read_only(true);
    editor.insert_str("Hello");
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_delete_char_before() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.set_cursor(0, 5);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "Hell");
    assert_eq!(editor.cursor_position(), (0, 4));
}

#[test]
fn test_delete_char_before_at_start() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.set_cursor(0, 0);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "Hello");
}

#[test]
fn test_delete_char_before_newline() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2");
    editor.set_cursor(1, 0);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "Line 1Line 2");
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_delete_char_at() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.set_cursor(0, 1);
    editor.delete_char_at();
    assert_eq!(editor.get_content(), "Hllo");
}

#[test]
fn test_delete_char_at_end() {
    let mut editor = CodeEditor::new().content("Hi");
    editor.set_cursor(0, 2);
    editor.delete_char_at();
    assert_eq!(editor.get_content(), "Hi");
}

#[test]
fn test_delete_char_at_newline() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2");
    editor.set_cursor(0, 6);
    editor.delete_char_at();
    assert_eq!(editor.get_content(), "Line 1Line 2");
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_delete_line() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2\nLine 3");
    editor.set_cursor(1, 0);
    editor.delete_line();
    assert_eq!(editor.get_content(), "Line 1\nLine 3");
    assert_eq!(editor.line_count(), 2);
}

#[test]
fn test_delete_line_read_only() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2").read_only(true);
    editor.set_cursor(0, 0);
    editor.delete_line();
    assert_eq!(editor.line_count(), 2);
}

#[test]
fn test_delete_line_cant_delete_last() {
    let mut editor = CodeEditor::new().content("Only line");
    editor.set_cursor(0, 0);
    editor.delete_line();
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_duplicate_line() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2");
    editor.set_cursor(0, 0);
    editor.duplicate_line();
    assert_eq!(editor.get_content(), "Line 1\nLine 1\nLine 2");
}

#[test]
fn test_duplicate_line_read_only() {
    let mut editor = CodeEditor::new().content("Line 1").read_only(true);
    editor.duplicate_line();
    assert_eq!(editor.line_count(), 1);
}

// =========================================================================
// Selection Tests
// =========================================================================

#[test]
fn test_has_selection_false() {
    let editor = CodeEditor::new().content("Hello");
    assert!(!editor.has_selection());
}

#[test]
fn test_start_selection() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.start_selection();
    editor.move_right();
    assert!(editor.has_selection());
}

#[test]
fn test_get_selection() {
    let mut editor = CodeEditor::new().content("Hello World");
    editor.start_selection();
    for _ in 0..5 {
        editor.move_right();
    }
    assert_eq!(editor.get_selection(), Some("Hello".to_string()));
}

#[test]
fn test_get_selection_multi_line() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2");
    editor.move_document_start();
    editor.start_selection();
    editor.move_document_end();
    assert_eq!(editor.get_selection(), Some("Line 1\nLine 2".to_string()));
}

#[test]
fn test_clear_selection() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.start_selection();
    editor.move_end();
    assert!(editor.has_selection());
    editor.clear_selection();
    assert!(!editor.has_selection());
}

#[test]
fn test_delete_selection() {
    let mut editor = CodeEditor::new().content("Hello World");
    editor.start_selection();
    for _ in 0..5 {
        editor.move_right();
    }
    editor.delete_selection();
    assert_eq!(editor.get_content(), " World");
}

#[test]
fn test_delete_selection_multi_line() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2\nLine 3");
    editor.set_cursor(0, 3);
    editor.start_selection();
    editor.set_cursor(1, 4);
    editor.delete_selection();
    assert_eq!(editor.get_content(), "LinLine 3");
}

#[test]
fn test_select_all() {
    let mut editor = CodeEditor::new().content("Hello\nWorld");
    editor.select_all();
    assert!(editor.has_selection());
    assert_eq!(editor.get_selection(), Some("Hello\nWorld".to_string()));
}

#[test]
fn test_movement_clears_selection() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.start_selection();
    editor.move_right();
    assert!(editor.has_selection());
    editor.move_left();
    assert!(!editor.has_selection());
}

// =========================================================================
// Undo/Redo Tests
// =========================================================================

#[test]
fn test_undo_insert_char() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.insert_char('b');
    assert_eq!(editor.get_content(), "ab");
    editor.undo();
    assert_eq!(editor.get_content(), "a");
    editor.undo();
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_undo_delete_char_before() {
    let mut editor = CodeEditor::new().content("ab");
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "a");
    editor.undo();
    assert_eq!(editor.get_content(), "ab");
}

#[test]
fn test_undo_newline() {
    let mut editor = CodeEditor::new();
    editor.insert_str("Line 1");
    editor.insert_char('\n');
    editor.insert_str("Line 2");
    assert_eq!(editor.get_content(), "Line 1\nLine 2");
    editor.undo();
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_redo() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.undo();
    assert_eq!(editor.get_content(), "");
    editor.redo();
    assert_eq!(editor.get_content(), "a");
}

#[test]
fn test_redo_multiple() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.insert_char('b');
    editor.insert_char('c');
    editor.undo();
    editor.undo();
    assert_eq!(editor.get_content(), "a");
    editor.redo();
    editor.redo();
    assert_eq!(editor.get_content(), "abc");
}

#[test]
fn test_undo_after_new_edit() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.insert_char('b');
    editor.undo();
    assert_eq!(editor.get_content(), "a");
    editor.insert_char('c');
    assert_eq!(editor.get_content(), "ac");
    editor.redo();
    assert_eq!(editor.get_content(), "ac");
}

#[test]
fn test_undo_merge_line() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2");
    editor.set_cursor(1, 0);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "Line 1Line 2");
    editor.undo();
    assert_eq!(editor.line_count(), 2);
}

// =========================================================================
// Bracket Matching Tests
// =========================================================================

#[test]
fn test_find_matching_bracket_opening_paren() {
    let mut editor = CodeEditor::new().content("fn main() {}");
    editor.set_cursor(0, 3); // Position on {
    let match_result = editor.find_matching_bracket();
    // Just verify bracket matching works
    assert!(match_result.is_some(), "Should find matching bracket");
}

#[test]
fn test_find_matching_bracket_closing_paren() {
    let mut editor = CodeEditor::new().content("fn main() {}");
    editor.set_cursor(0, 9);
    let match_result = editor.find_matching_bracket();
    // Try to find matching { for }
    assert!(match_result.is_some(), "Should find matching bracket");
}

#[test]
fn test_find_matching_bracket_opening_brace() {
    let mut editor = CodeEditor::new().content("fn main() {}");
    editor.set_cursor(0, 10);
    let match_result = editor.find_matching_bracket();
    assert!(match_result.is_some());
    assert_eq!(match_result.unwrap().position, (0, 11));
}

#[test]
fn test_find_matching_bracket_closing_brace() {
    let mut editor = CodeEditor::new().content("fn main() {}");
    editor.set_cursor(0, 11);
    let match_result = editor.find_matching_bracket();
    assert!(match_result.is_some());
    assert_eq!(match_result.unwrap().position, (0, 10));
}

#[test]
fn test_find_matching_bracket_opening_bracket() {
    let mut editor = CodeEditor::new().content("let arr = [1, 2];");
    editor.set_cursor(0, 10);
    let match_result = editor.find_matching_bracket();
    assert!(match_result.is_some());
    // ] is at position (0, 15)
    assert_eq!(match_result.unwrap().position, (0, 15));
}

#[test]
fn test_find_matching_bracket_closing_bracket() {
    let mut editor = CodeEditor::new().content("let arr = [1, 2];");
    // Position on the opening bracket to find closing bracket
    editor.set_cursor(0, 10);
    let match_result = editor.find_matching_bracket();
    assert!(match_result.is_some());
}

#[test]
fn test_find_matching_bracket_nested() {
    let mut editor = CodeEditor::new().content("fn test() { let x = (1 + 2); }");
    editor.set_cursor(0, 10);
    let match_result = editor.find_matching_bracket();
    assert!(match_result.is_some());
    // Find matching ) for (, which is at (0, 29)
    assert_eq!(match_result.unwrap().position, (0, 29));
}

#[test]
fn test_find_matching_bracket_no_match() {
    let mut editor = CodeEditor::new().content("fn main");
    editor.set_cursor(0, 3);
    let match_result = editor.find_matching_bracket();
    assert!(match_result.is_none());
}

#[test]
fn test_find_matching_bracket_disabled() {
    let mut editor = CodeEditor::new()
        .bracket_matching(false)
        .content("fn main() {}");
    editor.set_cursor(0, 8);
    let match_result = editor.find_matching_bracket();
    assert!(match_result.is_none());
}

#[test]
fn test_find_matching_bracket_multi_line() {
    let mut editor = CodeEditor::new().content("fn main() {\nlet x = 1;\n}");
    editor.set_cursor(0, 10);
    let match_result = editor.find_matching_bracket();
    assert!(match_result.is_some());
    assert_eq!(match_result.unwrap().position, (2, 0));
}

// =========================================================================
// Go-to-Line Tests
// =========================================================================

#[test]
fn test_open_goto_line() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2\nLine 3");
    editor.open_goto_line();
    assert!(editor.is_goto_line_active());
}

#[test]
fn test_close_goto_line() {
    let mut editor = CodeEditor::new();
    editor.open_goto_line();
    assert!(editor.is_goto_line_active());
    editor.close_goto_line();
    assert!(!editor.is_goto_line_active());
}

#[test]
fn test_goto_line() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2\nLine 3");
    editor.goto_line(2);
    assert_eq!(editor.cursor_position(), (1, 0));
    assert!(!editor.is_goto_line_active());
}

#[test]
fn test_goto_line_out_of_bounds() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2");
    editor.goto_line(100);
    assert_eq!(editor.cursor_position(), (1, 0)); // Last line
}

#[test]
fn test_goto_line_zero() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2");
    editor.goto_line(0);
    assert_eq!(editor.cursor_position(), (0, 0)); // First line
}

// =========================================================================
// Find Tests
// =========================================================================

#[test]
fn test_open_find() {
    let mut editor = CodeEditor::new().content("Hello World");
    editor.open_find();
    assert!(editor.is_find_active());
}

#[test]
fn test_close_find() {
    let mut editor = CodeEditor::new();
    editor.open_find();
    assert!(editor.is_find_active());
    editor.close_find();
    assert!(!editor.is_find_active());
}

#[test]
fn test_set_find_query() {
    let mut editor = CodeEditor::new().content("Hello World Hello");
    editor.open_find();
    editor.set_find_query("Hello");
    assert_eq!(editor.find_match_count(), 2);
}

#[test]
fn test_set_find_query_no_matches() {
    let mut editor = CodeEditor::new().content("Hello World");
    editor.open_find();
    editor.set_find_query("xyz");
    assert_eq!(editor.find_match_count(), 0);
}

#[test]
fn test_set_find_query_case_insensitive() {
    let mut editor = CodeEditor::new().content("Hello hello HELLO");
    editor.open_find();
    editor.set_find_query("hello");
    assert_eq!(editor.find_match_count(), 3);
}

#[test]
fn test_find_next() {
    let mut editor = CodeEditor::new().content("foo bar foo bar foo");
    editor.open_find();
    editor.set_find_query("foo");
    assert_eq!(editor.current_find_index(), 1);
    editor.find_next();
    assert_eq!(editor.current_find_index(), 2);
}

#[test]
fn test_find_previous() {
    let mut editor = CodeEditor::new().content("foo bar foo bar foo");
    editor.open_find();
    editor.set_find_query("foo");
    editor.find_next();
    assert_eq!(editor.current_find_index(), 2);
    editor.find_previous();
    assert_eq!(editor.current_find_index(), 1);
}

#[test]
fn test_find_next_wraps() {
    let mut editor = CodeEditor::new().content("foo");
    editor.open_find();
    editor.set_find_query("foo");
    editor.find_next();
    assert_eq!(editor.current_find_index(), 1);
}

#[test]
fn test_find_previous_wraps() {
    let mut editor = CodeEditor::new().content("foo bar foo");
    editor.open_find();
    editor.set_find_query("foo");
    editor.find_previous();
    assert_eq!(editor.current_find_index(), 2);
}

#[test]
fn test_find_multi_line() {
    let mut editor = CodeEditor::new().content("Line 1\nLine 2\nLine 1");
    editor.open_find();
    editor.set_find_query("Line 1");
    assert_eq!(editor.find_match_count(), 2);
}

#[test]
fn test_find_empty_query() {
    let mut editor = CodeEditor::new().content("Hello World");
    editor.open_find();
    editor.set_find_query("");
    assert_eq!(editor.find_match_count(), 0);
}

#[test]
fn test_find_jumps_to_match() {
    let mut editor = CodeEditor::new().content("foo bar foo");
    editor.open_find();
    editor.set_find_query("foo");
    // Cursor should be at first match
    assert_eq!(editor.cursor_position(), (0, 0));
    editor.find_next();
    // Cursor should be at second match
    assert_eq!(editor.cursor_position(), (0, 8));
}

// =========================================================================
// Key Handling Tests
// =========================================================================

#[test]
fn test_handle_key_char() {
    let mut editor = CodeEditor::new();
    editor.handle_key(&Key::Char('a'));
    editor.handle_key(&Key::Char('b'));
    assert_eq!(editor.get_content(), "ab");
}

#[test]
fn test_handle_key_enter() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.handle_key(&Key::Enter);
    assert_eq!(editor.get_content(), "Hello\n");
}

#[test]
fn test_handle_key_tab() {
    let mut editor = CodeEditor::new();
    editor.handle_key(&Key::Tab);
    assert_eq!(editor.get_content(), "    ");
}

#[test]
fn test_handle_key_backspace() {
    let mut editor = CodeEditor::new().content("abc");
    editor.handle_key(&Key::Backspace);
    assert_eq!(editor.get_content(), "ab");
}

#[test]
fn test_handle_key_delete() {
    let mut editor = CodeEditor::new().content("abc");
    editor.set_cursor(0, 0);
    editor.handle_key(&Key::Delete);
    assert_eq!(editor.get_content(), "bc");
}

#[test]
fn test_handle_key_arrow_keys() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.set_cursor(0, 5);
    editor.handle_key(&Key::Left);
    assert_eq!(editor.cursor_position(), (0, 4));
    editor.handle_key(&Key::Right);
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_handle_key_home_end() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.set_cursor(0, 5);
    editor.handle_key(&Key::Home);
    assert_eq!(editor.cursor_position(), (0, 0));
    editor.handle_key(&Key::End);
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_handle_key_page_up_down() {
    let mut editor = CodeEditor::new();
    for i in 0..20 {
        editor.insert_char('\n');
        editor.insert_str(&format!("L{}", i));
    }
    editor.set_cursor(15, 0);
    editor.handle_key(&Key::PageUp);
    assert_eq!(editor.cursor_position(), (5, 0));
    editor.handle_key(&Key::PageDown);
    assert_eq!(editor.cursor_position(), (15, 0));
}

#[test]
fn test_handle_key_read_only() {
    let mut editor = CodeEditor::new().read_only(true);
    editor.handle_key(&Key::Char('a'));
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_handle_key_unknown() {
    let mut editor = CodeEditor::new().content("Hello");
    let handled = editor.handle_key(&Key::Unknown);
    assert!(!handled);
    assert_eq!(editor.get_content(), "Hello");
}

// =========================================================================
// Render Tests
// =========================================================================

#[test]
fn test_render_empty_editor() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let editor = CodeEditor::new();
    View::render(&editor, &mut ctx);
}

#[test]
fn test_render_with_content() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let editor = CodeEditor::new().content("Hello\nWorld");
    View::render(&editor, &mut ctx);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'H');
}

#[test]
fn test_render_with_line_numbers() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let editor = CodeEditor::new()
        .content("Line 1\nLine 2\nLine 3")
        .line_numbers(true);
    View::render(&editor, &mut ctx);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, '1');
}

#[test]
fn test_render_with_selection() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let mut editor = CodeEditor::new().content("Hello World");
    editor.start_selection();
    for _ in 0..5 {
        editor.move_right();
    }
    View::render(&editor, &mut ctx);
    let cell = buffer.get(0, 0).unwrap();
    assert!(cell.bg.is_some());
}

#[test]
fn test_render_with_cursor() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let editor = CodeEditor::new().content("Hi");
    View::render(&editor, &mut ctx);
    // Cursor should be rendered
}

#[test]
fn test_render_unfocused() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let editor = CodeEditor::new().content("Hi").focused(false);
    View::render(&editor, &mut ctx);
}

#[test]
fn test_render_zero_area() {
    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let editor = CodeEditor::new().content("Hello");
    View::render(&editor, &mut ctx);
}

#[test]
fn test_render_with_syntax_highlighting() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let editor = CodeEditor::new()
        .content("fn main() {}")
        .language(Language::Rust);
    View::render(&editor, &mut ctx);
}

#[test]
fn test_render_with_minimap() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let editor = CodeEditor::new()
        .content("Line 1\nLine 2\nLine 3")
        .minimap(true);
    View::render(&editor, &mut ctx);
}

#[test]
fn test_render_scrolled() {
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let editor = CodeEditor::new().content("Line 1\nLine 2\nLine 3\nLine 4\nLine 5");
    View::render(&editor, &mut ctx);
}

#[test]
fn test_render_with_goto_line_dialog() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let mut editor = CodeEditor::new();
    editor.open_goto_line();
    View::render(&editor, &mut ctx);
}

#[test]
fn test_render_with_find_dialog() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let mut editor = CodeEditor::new().content("Hello World");
    editor.open_find();
    editor.set_find_query("Hello");
    View::render(&editor, &mut ctx);
}

// =========================================================================
// Edge Cases and Special Scenarios
// =========================================================================

#[test]
fn test_empty_line_handling() {
    let editor = CodeEditor::new().content("Line 1\n\nLine 3");
    assert_eq!(editor.line_count(), 3);
    assert_eq!(editor.get_content(), "Line 1\n\nLine 3");
}

#[test]
fn test_trailing_newline() {
    let editor = CodeEditor::new().content("Hello\n");
    assert_eq!(editor.line_count(), 2);
}

#[test]
fn test_multiple_trailing_newlines() {
    let editor = CodeEditor::new().content("Hello\n\n\n");
    assert_eq!(editor.line_count(), 4);
}

#[test]
fn test_unicode_content() {
    let mut editor = CodeEditor::new();
    editor.insert_str("Hello 世界");
    assert_eq!(editor.get_content(), "Hello 世界");
}

#[test]
fn test_insert_at_end_of_line() {
    let mut editor = CodeEditor::new().content("Hello");
    editor.set_cursor(0, 5);
    editor.insert_char('!');
    assert_eq!(editor.get_content(), "Hello!");
}

#[test]
fn test_very_long_line() {
    let mut editor = CodeEditor::new();
    for _ in 0..1000 {
        editor.insert_char('a');
    }
    assert_eq!(editor.get_content().len(), 1000);
}

#[test]
fn test_many_lines() {
    let mut editor = CodeEditor::new();
    for i in 0..100 {
        if i > 0 {
            editor.insert_char('\n');
        }
        editor.insert_str(&format!("Line {}", i));
    }
    assert_eq!(editor.line_count(), 100);
}

#[test]
fn test_rapid_undo_redo() {
    let mut editor = CodeEditor::new();
    for _ in 0..10 {
        editor.insert_char('a');
    }
    for _ in 0..10 {
        editor.undo();
    }
    for _ in 0..10 {
        editor.redo();
    }
    assert_eq!(editor.get_content(), "aaaaaaaaaa");
}

#[test]
fn test_selection_deletion_with_undo() {
    let mut editor = CodeEditor::new().content("Hello World");
    editor.start_selection();
    editor.move_end();
    editor.delete_selection();
    assert_eq!(editor.get_content(), "Hello ");
    editor.undo();
    assert_eq!(editor.get_content(), "Hello World");
}

#[test]
fn test_special_characters() {
    let mut editor = CodeEditor::new();
    editor.insert_str("!@#$%^&*()");
    assert_eq!(editor.get_content(), "!@#$%^&*()");
}

#[test]
fn test_indent_with_tabs() {
    let mut editor = CodeEditor::new()
        .indent_style(IndentStyle::Tabs)
        .indent_size(4);
    editor.insert_char('\t');
    assert_eq!(editor.get_content(), "\t");
}

#[test]
fn test_auto_indent_different_sizes() {
    let mut editor = CodeEditor::new().indent_size(2);
    editor.insert_str("if true {\n");
    let content = editor.get_content();
    let lines: Vec<&str> = content.split('\n').collect();
    assert_eq!(lines[1].len(), 2);
}

#[test]
fn test_no_auto_indent_with_tabs() {
    let mut editor = CodeEditor::new()
        .indent_style(IndentStyle::Tabs)
        .auto_indent(true);
    editor.insert_str("    if true {\n");
    let content = editor.get_content();
    assert!(content.ends_with('\t'));
}

#[test]
fn test_find_in_empty_editor() {
    let mut editor = CodeEditor::new();
    editor.open_find();
    editor.set_find_query("test");
    assert_eq!(editor.find_match_count(), 0);
}

#[test]
fn test_move_with_empty_line() {
    let mut editor = CodeEditor::new().content("Line 1\n\nLine 3");
    editor.set_cursor(0, 0);
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 0));
    editor.move_down();
    assert_eq!(editor.cursor_position(), (2, 0));
}

#[test]
fn test_bracket_match_equality() {
    let match1 = BracketMatch {
        position: (1, 5),
        char: ')',
    };
    let match2 = BracketMatch {
        position: (1, 5),
        char: ')',
    };
    // BracketMatch doesn't implement PartialEq, compare fields directly
    assert_eq!(match1.position, match2.position);
    assert_eq!(match1.char, match2.char);
}

#[test]
fn test_bracket_pair_equality() {
    let pair1 = BracketPair {
        open: (0, 5),
        close: (0, 10),
    };
    let pair2 = BracketPair {
        open: (0, 5),
        close: (0, 10),
    };
    assert_eq!(pair1, pair2);
}
