//! Editable text is laid out in terminal columns, not chars.
//!
//! The text widgets keep their cursor as a char index, which is right for
//! editing, but a wide glyph (Hangul, CJK, emoji) spans two cells: the glyph,
//! then a continuation cell. Drawing glyph `i` without its continuation leaves
//! the second half of the cell to whatever was there before (the cursor or the
//! selection then covers only half the glyph), and a horizontal scroll that
//! counts chars instead of columns lets the cursor run off the edge.

use revue::event::{Key, KeyEvent};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{Input, TextArea};

fn render(view: &dyn View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    view.render(&mut ctx);
    buffer
}

/// A buffer row as the terminal shows it: a wide glyph is one `char`
/// spanning two cells, so its continuation cell contributes nothing.
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

fn press(input: &mut Input, key: Key) {
    input.handle_key_event(&KeyEvent::new(key));
}

fn shift(input: &mut Input, key: Key) {
    let mut event = KeyEvent::new(key);
    event.shift = true;
    input.handle_key_event(&event);
}

const CURSOR_BG: Option<Color> = Some(Color::WHITE);
const SELECTION_BG: Option<Color> = Some(Color::rgb(70, 130, 180));

// ─── Input: drawing ─────────────────────────────────────────────────────────

#[test]
fn input_draws_hangul_with_continuation_cells() {
    let input = Input::new().value("a한b");
    let buf = render(&input, 10, 1);

    assert_eq!(sym(&buf, 0, 0), 'a');
    assert_eq!(sym(&buf, 1, 0), '한');
    assert!(is_cont(&buf, 2, 0), "row: {:?}", row_text(&buf, 0));
    assert_eq!(sym(&buf, 3, 0), 'b');
    // The cursor sits after the text, at column 4 (not char index 3).
    assert_eq!(bg(&buf, 4, 0), CURSOR_BG);
    assert_ne!(bg(&buf, 3, 0), CURSOR_BG);
}

#[test]
fn input_cursor_on_a_wide_char_covers_both_cells() {
    let mut input = Input::new().value("a한b");
    press(&mut input, Key::Left);
    press(&mut input, Key::Left); // cursor index 1, on '한'
    assert_eq!(input.cursor(), 1);
    let buf = render(&input, 10, 1);

    assert_ne!(bg(&buf, 0, 0), CURSOR_BG);
    assert_eq!(sym(&buf, 1, 0), '한');
    assert_eq!(bg(&buf, 1, 0), CURSOR_BG);
    assert!(is_cont(&buf, 2, 0));
    assert_eq!(bg(&buf, 2, 0), CURSOR_BG, "second half of the cursor glyph");
    assert_ne!(bg(&buf, 3, 0), CURSOR_BG);
}

#[test]
fn input_cursor_after_hangul_is_at_its_column() {
    let mut input = Input::new().value("안녕하세요");
    press(&mut input, Key::Left); // cursor index 4, on '요' at column 8
    let buf = render(&input, 20, 1);

    assert_eq!(row_text(&buf, 0).trim_end(), "안녕하세요");
    assert_eq!(sym(&buf, 8, 0), '요');
    assert_eq!(bg(&buf, 8, 0), CURSOR_BG);
    assert_eq!(bg(&buf, 9, 0), CURSOR_BG);
    for x in 0..8 {
        assert_ne!(bg(&buf, x, 0), CURSOR_BG, "column {x}");
    }
}

#[test]
fn input_selection_covers_both_cells_of_wide_chars() {
    // Select from the start so the cursor (which paints over the selection)
    // ends up on 'b', clear of the selected cells.
    let mut input = Input::new().value("a한b");
    press(&mut input, Key::Home);
    shift(&mut input, Key::Right);
    shift(&mut input, Key::Right); // selects "a한", cursor on 'b'
    let buf = render(&input, 10, 1);

    assert_eq!(bg(&buf, 0, 0), SELECTION_BG);
    assert_eq!(bg(&buf, 1, 0), SELECTION_BG);
    assert!(is_cont(&buf, 2, 0));
    assert_eq!(bg(&buf, 2, 0), SELECTION_BG, "second half of '한'");
    assert_eq!(bg(&buf, 3, 0), CURSOR_BG);
}

#[test]
fn input_scrolls_by_columns_to_keep_the_cursor_visible() {
    // Ten syllables are 20 columns; the box is 8 wide.
    let input = Input::new().value("안녕하세요반갑습니다");
    let buf = render(&input, 8, 1);

    // Cursor at the end: it must be on screen, right after the last glyph.
    let cursor_x = (0..8).find(|&x| bg(&buf, x, 0) == CURSOR_BG);
    assert_eq!(cursor_x, Some(7), "row: {:?}", row_text(&buf, 0));
    assert_eq!(sym(&buf, 5, 0), '다', "row: {:?}", row_text(&buf, 0));
    assert!(is_cont(&buf, 6, 0));
}

#[test]
fn input_scrolls_back_when_the_cursor_moves_left_of_the_view() {
    let mut input = Input::new().value("안녕하세요반갑습니다");
    let _ = render(&input, 8, 1);
    press(&mut input, Key::Home);
    let buf = render(&input, 8, 1);

    assert_eq!(sym(&buf, 0, 0), '안');
    assert_eq!(bg(&buf, 0, 0), CURSOR_BG);
    assert!(is_cont(&buf, 1, 0));
    assert_eq!(row_text(&buf, 0), "안녕하세");
}

#[test]
fn input_scrolling_ascii_keeps_the_cursor_visible() {
    let input = Input::new().value("abcdefghijklmnop");
    let buf = render(&input, 6, 1);
    assert_eq!(row_text(&buf, 0), "lmnop ");
    assert_eq!(bg(&buf, 5, 0), CURSOR_BG);
}

#[test]
fn input_cursor_on_a_wide_char_at_the_right_edge_is_scrolled_in_whole() {
    // "ab" then '한' at columns 2-3; a 3-wide box cannot show the cursor on
    // '한' without scrolling: half a glyph is not a cursor.
    let mut input = Input::new().value("ab한");
    press(&mut input, Key::Left);
    let buf = render(&input, 3, 1);

    let x = (0..3)
        .find(|&x| sym(&buf, x, 0) == '한')
        .unwrap_or_else(|| panic!("'한' not drawn: {:?}", row_text(&buf, 0)));
    assert!(x + 1 < 3);
    assert_eq!(bg(&buf, x, 0), CURSOR_BG);
    assert_eq!(bg(&buf, x + 1, 0), CURSOR_BG);
}

#[test]
fn input_placeholder_with_hangul_keeps_continuation_cells() {
    let input = Input::new().placeholder("이름").focused(false);
    let buf = render(&input, 10, 1);
    assert_eq!(sym(&buf, 0, 0), '이');
    assert!(is_cont(&buf, 1, 0));
    assert_eq!(sym(&buf, 2, 0), '름');
    assert!(is_cont(&buf, 3, 0));
}

// ─── Input: editing ─────────────────────────────────────────────────────────

#[test]
fn input_typing_and_backspacing_hangul_moves_by_whole_chars() {
    let mut input = Input::new();
    for ch in "안녕".chars() {
        press(&mut input, Key::Char(ch));
    }
    assert_eq!(input.text(), "안녕");
    assert_eq!(input.cursor(), 2);

    press(&mut input, Key::Backspace);
    assert_eq!(input.text(), "안");
    assert_eq!(input.cursor(), 1);

    press(&mut input, Key::Left);
    press(&mut input, Key::Char('a'));
    assert_eq!(input.text(), "a안");
    assert_eq!(input.cursor(), 1);

    press(&mut input, Key::Delete);
    assert_eq!(input.text(), "a");
    press(&mut input, Key::Right);
    assert_eq!(input.cursor(), 1);
}

#[test]
fn input_editing_mixed_text_keeps_valid_utf8() {
    let mut input = Input::new().value("a한b");
    press(&mut input, Key::Left);
    press(&mut input, Key::Backspace); // deletes '한'
    assert_eq!(input.text(), "ab");
    assert_eq!(input.cursor(), 1);
    press(&mut input, Key::Char('글'));
    assert_eq!(input.text(), "a글b");
    assert_eq!(input.cursor(), 2);

    input.handle_key_event(&KeyEvent::ctrl(Key::Backspace));
    assert_eq!(input.text(), "b");
    assert_eq!(input.cursor(), 0);
}

// ─── TextArea ───────────────────────────────────────────────────────────────

/// A focused TextArea with the cursor at the end of its content.
fn textarea(content: &str) -> TextArea {
    let mut ta = TextArea::new().content(content).focused(true);
    ta.set_cursor(usize::MAX, usize::MAX);
    ta
}

fn ta_key(ta: &mut TextArea, key: Key) {
    ta.handle_key_event(&KeyEvent::new(key));
}

#[test]
fn textarea_draws_hangul_with_continuation_cells_and_cursor_on_both() {
    let mut ta = textarea("a한b");
    ta.set_cursor(0, 1); // on '한'
    let buf = render(&ta, 10, 3);

    assert_eq!(sym(&buf, 0, 0), 'a');
    assert_eq!(sym(&buf, 1, 0), '한');
    assert!(is_cont(&buf, 2, 0), "row: {:?}", row_text(&buf, 0));
    assert_eq!(sym(&buf, 3, 0), 'b');
    assert_eq!(bg(&buf, 1, 0), CURSOR_BG);
    assert_eq!(bg(&buf, 2, 0), CURSOR_BG, "second half of the cursor glyph");
    assert_ne!(bg(&buf, 3, 0), CURSOR_BG);
}

#[test]
fn textarea_cursor_at_end_of_hangul_line_is_at_its_column() {
    let ta = textarea("안녕"); // cursor at end, char index 2, column 4
    let buf = render(&ta, 10, 3);
    assert_eq!(bg(&buf, 4, 0), CURSOR_BG);
    for x in 0..4 {
        assert_ne!(bg(&buf, x, 0), CURSOR_BG, "column {x}");
    }
}

#[test]
fn textarea_selection_covers_both_cells_of_wide_chars() {
    let mut ta = textarea("a한b");
    ta.set_cursor(0, 0);
    ta.start_selection();
    ta_key_shiftless_right(&mut ta, 2); // select "a한", cursor on 'b'
    let buf = render(&ta, 10, 3);

    let sel = Some(Color::rgb(50, 50, 150));
    assert_eq!(bg(&buf, 0, 0), sel);
    assert_eq!(bg(&buf, 1, 0), sel);
    assert!(is_cont(&buf, 2, 0));
    assert_eq!(bg(&buf, 2, 0), sel, "second half of '한'");
    assert_eq!(bg(&buf, 3, 0), CURSOR_BG);
}

/// Move right `n` times keeping the selection anchor (`move_right` itself
/// leaves the anchor alone; the plain Right key clears it).
fn ta_key_shiftless_right(ta: &mut TextArea, n: usize) {
    for _ in 0..n {
        ta.move_right();
    }
}

#[test]
fn textarea_wrapped_hangul_keeps_continuation_cells() {
    // 5 syllables, 10 columns, in a 6-wide box: "안녕하" then "세요".
    let ta = textarea("안녕하세요");
    let buf = render(&ta, 6, 3);
    assert_eq!(row_text(&buf, 0), "안녕하");
    assert!(is_cont(&buf, 1, 0) && is_cont(&buf, 3, 0) && is_cont(&buf, 5, 0));
    assert_eq!(row_text(&buf, 1).trim_end(), "세요");
    assert!(is_cont(&buf, 1, 1) && is_cont(&buf, 3, 1));
    assert_eq!(bg(&buf, 4, 1), CURSOR_BG);
}

#[test]
fn textarea_without_wrap_scrolls_by_columns_to_keep_the_cursor_visible() {
    let ta = textarea("안녕하세요반갑습니다").wrap(false);
    let buf = render(&ta, 8, 3);

    let cursor_x = (0..8).find(|&x| bg(&buf, x, 0) == CURSOR_BG);
    assert_eq!(cursor_x, Some(7), "row: {:?}", row_text(&buf, 0));
    assert_eq!(sym(&buf, 5, 0), '다', "row: {:?}", row_text(&buf, 0));
    assert!(is_cont(&buf, 6, 0));
}

#[test]
fn textarea_without_wrap_scrolls_back_to_the_line_start() {
    let mut ta = textarea("안녕하세요반갑습니다").wrap(false);
    let _ = render(&ta, 8, 3);
    ta_key(&mut ta, Key::Home);
    let buf = render(&ta, 8, 3);
    assert_eq!(row_text(&buf, 0), "안녕하세");
    assert_eq!(bg(&buf, 0, 0), CURSOR_BG);
}

#[test]
fn textarea_typing_and_backspacing_hangul_moves_by_whole_chars() {
    let mut ta = textarea("");
    for ch in "한글".chars() {
        ta_key(&mut ta, Key::Char(ch));
    }
    assert_eq!(ta.get_content(), "한글");
    assert_eq!(ta.cursor_position(), (0, 2));
    ta_key(&mut ta, Key::Backspace);
    assert_eq!(ta.get_content(), "한");
    ta_key(&mut ta, Key::Left);
    ta_key(&mut ta, Key::Char('a'));
    assert_eq!(ta.get_content(), "a한");
    ta_key(&mut ta, Key::Delete);
    assert_eq!(ta.get_content(), "a");
}

#[test]
fn textarea_up_down_keep_the_screen_column_across_wide_chars() {
    // 'b' on line 0 is char 2 but column 3; Down should land under it, on 'd'.
    let mut ta = textarea("a한b\nabcd");
    ta.set_cursor(0, 2);
    ta_key(&mut ta, Key::Down);
    assert_eq!(ta.cursor_position(), (1, 3));
    ta_key(&mut ta, Key::Up);
    assert_eq!(ta.cursor_position(), (0, 2));

    // Column 1 is the left half of '한': land on the glyph, not past it.
    let mut ta = textarea("a한b\nabcd");
    ta.set_cursor(1, 2); // 'c', column 2: the right half of '한'
    ta_key(&mut ta, Key::Up);
    assert_eq!(ta.cursor_position(), (0, 1));
}

#[test]
fn textarea_find_reports_char_columns_in_hangul_text() {
    let mut ta = textarea("안녕 안녕");
    ta.open_find();
    ta.set_find_query("녕");
    let matches: Vec<_> = ta
        .find_state()
        .unwrap()
        .matches
        .iter()
        .map(|m| (m.start.col, m.end.col))
        .collect();
    assert_eq!(matches, vec![(1, 2), (4, 5)]);
}

#[test]
fn textarea_find_does_not_slice_inside_a_hangul_syllable() {
    // The old search restarted one *byte* after each match.
    let mut ta = textarea("한한한");
    ta.open_find();
    ta.set_find_query("한");
    assert_eq!(ta.find_state().unwrap().match_count(), 3);
}

#[test]
fn textarea_find_whole_word_in_hangul_text() {
    let mut ta = textarea("한글 한글날");
    ta.open_find();
    ta.toggle_whole_word();
    ta.set_find_query("한글");
    let m = &ta.find_state().unwrap().matches;
    assert_eq!(m.len(), 1);
    assert_eq!((m[0].start.col, m[0].end.col), (0, 2));
}

#[test]
fn textarea_select_next_occurrence_in_hangul_text() {
    let mut ta = textarea("한글 한글");
    ta.set_cursor(0, 0);
    ta.handle_key_event(&KeyEvent::ctrl(Key::Char('d')));
    let positions = ta.cursor_positions();
    assert_eq!(positions.len(), 2, "{positions:?}");
    assert!(positions.contains(&(0, 5)), "{positions:?}");
}
