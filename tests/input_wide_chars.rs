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
use revue::widget::Input;

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
