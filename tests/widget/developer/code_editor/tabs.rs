//! CodeEditor with tabs in the text (#1378): a tab reaches the next multiple
//! of `indent_size`, in drawing, the cursor, horizontal scroll and moving
//! between lines alike.

use revue::event::Key;
use revue::widget::code_editor::{EditorConfig, IndentStyle};
use revue::widget::CodeEditor;

use super::render::{row, screen};

fn plain(content: &str) -> CodeEditor {
    CodeEditor::new()
        .content(content)
        .line_numbers(false)
        .highlight_current_line(false)
        .bracket_matching(false)
}

/// Columns of row 0 drawn on the cursor's background
fn cursor_cols(editor: &CodeEditor, w: u16) -> Vec<u16> {
    let buffer = screen(editor, w, 1);
    (0..w)
        .filter(|&x| buffer.get(x, 0).unwrap().bg == Some(editor.cursor_bg))
        .collect()
}

#[test]
fn a_tab_reaches_the_next_stop() {
    let editor = plain("\tif x {\nab\tc\nabcd\te").focused(false);
    let buffer = screen(&editor, 20, 3);
    // From the line start: a whole stop
    assert_eq!(row(&buffer, 0), "    if x {");
    // Partway: the rest of the stop
    assert_eq!(row(&buffer, 1), "ab  c");
    // On a stop: a whole stop again, never zero
    assert_eq!(row(&buffer, 2), "abcd    e");
}

#[test]
fn the_stop_is_indent_size() {
    let editor = plain("\tb\na\tb").indent_size(2).focused(false);
    let buffer = screen(&editor, 10, 2);
    assert_eq!(row(&buffer, 0), "  b");
    assert_eq!(row(&buffer, 1), "a b");
}

#[test]
fn the_cursor_on_a_tab_covers_it() {
    let mut editor = plain("\tx");
    editor.set_cursor(0, 0);
    assert_eq!(cursor_cols(&editor, 10), vec![0, 1, 2, 3]);
    // After it: on the glyph
    editor.set_cursor(0, 1);
    assert_eq!(cursor_cols(&editor, 10), vec![4]);
    // A partial tab: only the columns it takes
    let mut editor = plain("abc\tx");
    editor.set_cursor(0, 3);
    assert_eq!(cursor_cols(&editor, 10), vec![3]);
    // Past the end of a line ending in a tab: after the stop
    let mut editor = plain("a\t");
    editor.set_cursor(0, 2);
    assert_eq!(cursor_cols(&editor, 10), vec![4]);
}

#[test]
fn moving_through_indentation_keeps_the_cursor_drawn() {
    let mut editor = plain("\t\tx");
    editor.set_cursor(0, 3);
    let mut seen = Vec::new();
    for _ in 0..3 {
        editor.handle_key(&Key::Left);
        seen.push(cursor_cols(&editor, 12));
    }
    assert_eq!(seen, vec![vec![8], vec![4, 5, 6, 7], vec![0, 1, 2, 3]]);
}

#[test]
fn horizontal_scroll_counts_tab_columns() {
    // `x` at column 8: past a 6-column view, so the view scrolls by the
    // tabs' columns, not by their char count
    let mut editor = plain("\t\tx");
    editor.set_cursor(0, 2);
    let buffer = screen(&editor, 6, 1);
    assert_eq!(row(&buffer, 0), "     x");
    assert_eq!(cursor_cols(&editor, 6), vec![5]);
    // Back onto the first tab: the view starts at it, the whole tab shown
    editor.set_cursor(0, 0);
    assert_eq!(cursor_cols(&editor, 6), vec![0, 1, 2, 3]);
}

#[test]
fn a_tab_cut_by_the_left_edge_shows_its_remaining_cells() {
    // The cursor on `f` (column 7) in a 5-column view scrolls the view to
    // column 3, the last column of the tab: drawn blank, not left stale
    let mut editor = plain("ab\tcdef");
    editor.set_cursor(0, 6);
    let buffer = screen(&editor, 5, 1);
    assert_eq!(row(&buffer, 0), " cdef");
    assert_eq!(cursor_cols(&editor, 5), vec![4]);
}

#[test]
fn up_and_down_keep_the_screen_column_across_tabs() {
    let mut editor = plain("\tx\nabcdefgh");
    editor.set_cursor(0, 1); // `x`, column 4
    editor.handle_key(&Key::Down);
    assert_eq!(editor.cursor_position(), (1, 4)); // `e`, column 4
                                                  // From column 2, inside the tab above: onto the tab
    editor.set_cursor(1, 2);
    editor.handle_key(&Key::Up);
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn tab_indent_style_draws_the_inserted_tab() {
    let config = EditorConfig {
        indent_style: IndentStyle::Tabs,
        indent_size: 4,
        highlight_current_line: false,
        bracket_matching: false,
        ..Default::default()
    };
    let mut editor = CodeEditor::new()
        .line_numbers(false)
        .config(config)
        .focused(false);
    editor.insert_char('\t');
    editor.insert_char('y');
    assert_eq!(editor.get_content(), "\ty");
    assert_eq!(row(&screen(&editor, 10, 1), 0), "    y");
}
