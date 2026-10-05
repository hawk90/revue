//! CodeEditor with wide and multibyte text.
//!
//! The cursor column is a char index, as navigation and rendering already
//! assumed; the editing side indexed lines by bytes, so typing a second
//! Hangul syllable inserted it one byte into the first and panicked. Drawing
//! is in terminal columns, where a wide glyph takes two cells.

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::CodeEditor;

fn render(view: &dyn View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    view.render(&mut ctx);
    buffer
}

fn row_text(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .filter_map(|x| buffer.get(x, y))
        .filter(|cell| !cell.is_continuation())
        .map(|cell| cell.symbol)
        .collect()
}

fn sym(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

fn bg(buffer: &Buffer, x: u16, y: u16) -> Option<Color> {
    buffer.get(x, y).unwrap().bg
}

fn is_cont(buffer: &Buffer, x: u16, y: u16) -> bool {
    buffer.get(x, y).unwrap().is_continuation()
}

const CURSOR_BG: Option<Color> = Some(Color::rgb(166, 227, 161));

/// An editor with plain cells (no gutter, line or bracket highlight) and the
/// cursor at the end of its content.
fn editor(content: &str) -> CodeEditor {
    let mut ed = CodeEditor::new()
        .content(content)
        .line_numbers(false)
        .highlight_current_line(false)
        .bracket_matching(false);
    ed.set_cursor(usize::MAX, usize::MAX);
    ed
}

fn press(ed: &mut CodeEditor, key: Key) {
    ed.handle_key(&key);
}

// ─── Editing ────────────────────────────────────────────────────────────────

#[test]
fn typing_and_deleting_hangul_moves_by_whole_chars() {
    let mut ed = editor("");
    for ch in "안녕".chars() {
        press(&mut ed, Key::Char(ch));
    }
    assert_eq!(ed.get_content(), "안녕");
    assert_eq!(ed.cursor_position(), (0, 2));

    press(&mut ed, Key::Backspace);
    assert_eq!(ed.get_content(), "안");
    assert_eq!(ed.cursor_position(), (0, 1));

    press(&mut ed, Key::Left);
    press(&mut ed, Key::Char('a'));
    assert_eq!(ed.get_content(), "a안");
    press(&mut ed, Key::Delete);
    assert_eq!(ed.get_content(), "a");
}

#[test]
fn end_enter_and_merge_on_a_hangul_line() {
    let mut ed = editor("한글");
    press(&mut ed, Key::End);
    assert_eq!(ed.cursor_position(), (0, 2));
    press(&mut ed, Key::Left);
    press(&mut ed, Key::Enter);
    assert_eq!(ed.get_content(), "한\n글");
    press(&mut ed, Key::Backspace); // merge back
    assert_eq!(ed.get_content(), "한글");
    assert_eq!(ed.cursor_position(), (0, 1));
}

#[test]
fn tab_after_hangul_inserts_at_the_cursor() {
    let mut ed = editor("한");
    press(&mut ed, Key::Tab);
    assert_eq!(ed.get_content(), "한    ");
    assert_eq!(ed.cursor_position(), (0, 5));
}

#[test]
fn auto_closed_bracket_after_hangul() {
    let mut ed = editor("한").bracket_matching(true);
    ed.set_cursor(0, 1);
    press(&mut ed, Key::Char('('));
    assert_eq!(ed.get_content(), "한()");
    assert_eq!(ed.cursor_position(), (0, 2));
}

#[test]
fn up_down_keep_the_screen_column_across_wide_chars() {
    // 'b' on line 0 is char 2 but column 3; Down lands under it, on 'd'.
    let mut ed = editor("a한b\nabcd");
    ed.set_cursor(0, 2);
    press(&mut ed, Key::Down);
    assert_eq!(ed.cursor_position(), (1, 3));
    press(&mut ed, Key::Up);
    assert_eq!(ed.cursor_position(), (0, 2));
}

#[test]
fn undo_redo_hangul_edits() {
    let mut ed = editor("가");
    press(&mut ed, Key::Char('나'));
    press(&mut ed, Key::Char('다'));
    ed.undo();
    assert_eq!(ed.get_content(), "가나");
    ed.undo();
    assert_eq!(ed.get_content(), "가");
    ed.redo();
    assert_eq!(ed.get_content(), "가나");
    assert_eq!(ed.cursor_position(), (0, 2));

    press(&mut ed, Key::Backspace);
    assert_eq!(ed.get_content(), "가");
    ed.undo();
    assert_eq!(ed.get_content(), "가나");
    assert_eq!(ed.cursor_position(), (0, 2));
}

#[test]
fn undo_redo_line_split_after_hangul() {
    let mut ed = editor("한글");
    ed.set_cursor(0, 1);
    press(&mut ed, Key::Enter);
    assert_eq!(ed.get_content(), "한\n글");
    ed.undo();
    assert_eq!(ed.get_content(), "한글");
    ed.redo();
    assert_eq!(ed.get_content(), "한\n글");
}

#[test]
fn selecting_and_deleting_hangul_text() {
    let mut ed = editor("a한글b");
    ed.set_cursor(0, 1);
    ed.start_selection();
    ed.set_cursor(0, 3);
    assert_eq!(ed.get_selection().as_deref(), Some("한글"));
    ed.delete_selection();
    assert_eq!(ed.get_content(), "ab");
    ed.undo();
    assert_eq!(ed.get_content(), "a한글b");
}

#[test]
fn bracket_match_backward_across_a_hangul_line() {
    // Stepping back a line, the scan started at the line's *byte* length
    // and indexed its chars with it.
    let mut ed = CodeEditor::new().content("(한글\n)").bracket_matching(true);
    ed.set_cursor(1, 0);
    let m = ed.find_matching_bracket().expect("matching '('");
    assert_eq!(m.position, (0, 0));
}

#[test]
fn find_reports_char_columns_in_hangul_text() {
    let mut ed = editor("안녕 안녕\n한한한");
    ed.open_find();
    ed.set_find_query("녕");
    assert_eq!(ed.find_match_count(), 2);
    ed.find_next();
    assert_eq!(ed.cursor_position(), (0, 4));

    // The old search restarted one byte after each match.
    ed.set_find_query("한");
    assert_eq!(ed.find_match_count(), 3);
}

// ─── Drawing ────────────────────────────────────────────────────────────────

#[test]
fn draws_hangul_with_continuation_cells_and_cursor_on_both() {
    let mut ed = editor("a한b");
    ed.set_cursor(0, 1);
    let buf = render(&ed, 10, 2);

    assert_eq!(sym(&buf, 1, 0), '한');
    assert!(is_cont(&buf, 2, 0), "row: {:?}", row_text(&buf, 0));
    assert_eq!(sym(&buf, 3, 0), 'b');
    assert_eq!(bg(&buf, 1, 0), CURSOR_BG);
    assert_eq!(bg(&buf, 2, 0), CURSOR_BG);
    assert_ne!(bg(&buf, 3, 0), CURSOR_BG);
}

#[test]
fn scrolls_by_columns_to_keep_the_cursor_visible() {
    let ed = editor("안녕하세요반갑습니다");
    let buf = render(&ed, 8, 2);
    let cursor_x = (0..8).find(|&x| bg(&buf, x, 0) == CURSOR_BG);
    assert_eq!(cursor_x, Some(7), "row: {:?}", row_text(&buf, 0));
    assert_eq!(sym(&buf, 5, 0), '다');
    assert!(is_cont(&buf, 6, 0));
}

#[test]
fn find_dialog_draws_a_hangul_query_in_columns() {
    let mut ed = editor("한글");
    ed.open_find();
    for ch in "한글".chars() {
        press(&mut ed, Key::Char(ch));
    }
    // The 30-wide dialog centred in 40 columns starts at 5; "Find: " is 6.
    let buf = render(&ed, 40, 3);
    assert_eq!(sym(&buf, 11, 0), '한', "row: {:?}", row_text(&buf, 0));
    assert!(is_cont(&buf, 12, 0));
    assert_eq!(sym(&buf, 13, 0), '글');
    assert!(is_cont(&buf, 14, 0));
}
