//! Single-line text fields with wide and multibyte text: SearchBar, the
//! Autocomplete input, Form, the RichTextEditor link/image dialog and
//! MaskedInput.
//!
//! A wide glyph (Hangul, CJK, emoji) spans two cells, the second a
//! continuation cell; placing glyph `i` at column `x + i` drops it and lets
//! the next glyph land on top. A cursor kept as a byte index lands inside a
//! multibyte char and panics on the next edit.

use revue::event::{Key, KeyEvent};
use revue::layout::Rect;
use revue::patterns::form::FormState;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::autocomplete::Autocomplete;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{Form, MaskStyle, MaskedInput, RichTextEditor, SearchBar};

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

/// First cell in row `y` holding `ch`.
fn find_in_row(buffer: &Buffer, y: u16, ch: char) -> u16 {
    (0..buffer.width())
        .find(|&x| sym(buffer, x, y) == ch)
        .unwrap_or_else(|| panic!("{ch:?} not in row {y}: {:?}", row_text(buffer, y)))
}

// ─── SearchBar ──────────────────────────────────────────────────────────────

/// Text starts at column 4, after the border and the icon.
fn search_bar(text: &str) -> SearchBar {
    let mut bar = SearchBar::new().show_hints(false);
    bar.focus();
    for ch in text.chars() {
        bar.handle_key(&Key::Char(ch));
    }
    bar
}

const SEARCH_CURSOR_BG: Option<Color> = Some(Color::WHITE);

#[test]
fn search_bar_draws_hangul_with_continuation_cells() {
    let bar = search_bar("a한b");
    let buf = render(&bar, 40, 1);
    assert_eq!(sym(&buf, 4, 0), 'a');
    assert_eq!(sym(&buf, 5, 0), '한');
    assert!(is_cont(&buf, 6, 0), "row: {:?}", row_text(&buf, 0));
    assert_eq!(sym(&buf, 7, 0), 'b');
    assert_eq!(bg(&buf, 8, 0), SEARCH_CURSOR_BG);
}

#[test]
fn search_bar_cursor_on_a_wide_char_covers_both_cells() {
    let mut bar = search_bar("a한b");
    bar.handle_key(&Key::Left);
    bar.handle_key(&Key::Left);
    let buf = render(&bar, 40, 1);
    assert_eq!(sym(&buf, 5, 0), '한');
    assert_eq!(bg(&buf, 5, 0), SEARCH_CURSOR_BG);
    assert!(is_cont(&buf, 6, 0));
    assert_eq!(bg(&buf, 6, 0), SEARCH_CURSOR_BG);
    assert_ne!(bg(&buf, 7, 0), SEARCH_CURSOR_BG);
}

#[test]
fn search_bar_icon_keeps_its_continuation_cell() {
    let bar = search_bar("");
    let buf = render(&bar, 40, 1);
    assert_eq!(sym(&buf, 2, 0), '🔍');
    assert!(is_cont(&buf, 3, 0));
}

#[test]
fn search_bar_scrolls_by_columns_and_keeps_text_and_cursor_aligned() {
    // 12 wide leaves 6 columns for text (4..10); five syllables are 10.
    let bar = search_bar("안녕하세요");
    let buf = render(&bar.width(12), 12, 1);
    assert_eq!(
        bg(&buf, 9, 0),
        SEARCH_CURSOR_BG,
        "row: {:?}",
        row_text(&buf, 0)
    );
    assert_eq!(sym(&buf, 7, 0), '요', "row: {:?}", row_text(&buf, 0));
    assert!(is_cont(&buf, 8, 0));
}

// ─── Autocomplete ───────────────────────────────────────────────────────────

fn autocomplete(value: &str) -> Autocomplete {
    let mut ac = Autocomplete::new().value(value);
    ac.focus();
    ac
}

#[test]
fn autocomplete_draws_hangul_with_continuation_cells_and_cursor_after() {
    let ac = autocomplete("a한b");
    let buf = render(&ac, 20, 1);
    assert_eq!(sym(&buf, 0, 0), 'a');
    assert_eq!(sym(&buf, 1, 0), '한');
    assert!(is_cont(&buf, 2, 0), "row: {:?}", row_text(&buf, 0));
    assert_eq!(sym(&buf, 3, 0), 'b');
    // The cursor (inverted colors) is after 'b', at column 4, not 3.
    let cursor = buf.get(4, 0).unwrap();
    let text = buf.get(3, 0).unwrap();
    assert_eq!(cursor.bg, text.fg, "cursor at column 4");
    assert_ne!(text.bg, text.fg);
}

#[test]
fn autocomplete_scrolls_to_keep_the_cursor_visible() {
    let ac = autocomplete("안녕하세요반갑습니다");
    let buf = render(&ac, 8, 1);
    assert_eq!(sym(&buf, 5, 0), '다', "row: {:?}", row_text(&buf, 0));
    let cursor = buf.get(7, 0).unwrap();
    assert_eq!(cursor.bg, buf.get(5, 0).unwrap().fg);
}

#[test]
fn autocomplete_set_value_puts_the_cursor_after_the_last_char() {
    // The cursor was set to the byte length: 6 for two syllables, so the
    // next Backspace removed at a byte offset past the end and panicked.
    let mut ac = Autocomplete::new();
    ac.set_value("한글");
    ac.handle_key(KeyEvent::new(Key::Backspace));
    assert_eq!(ac.get_value(), "한");
}

#[test]
fn autocomplete_accepting_a_hangul_suggestion_then_editing() {
    let mut ac = Autocomplete::new().suggestions(vec!["서울", "부산"]);
    ac.focus();
    ac.handle_key(KeyEvent::new(Key::Char('서')));
    assert!(ac.accept_selection());
    assert_eq!(ac.get_value(), "서울");
    ac.handle_key(KeyEvent::new(Key::Char('시')));
    assert_eq!(ac.get_value(), "서울시");
}

#[test]
fn autocomplete_min_chars_counts_chars() {
    let mut ac = Autocomplete::new().suggestions(vec!["서울"]).min_chars(2);
    ac.focus();
    ac.handle_key(KeyEvent::new(Key::Char('서')));
    // One syllable is three bytes but one char: below the minimum.
    let buf = render(&ac, 20, 3);
    assert_eq!(row_text(&buf, 1).trim(), "", "dropdown shown for one char");
}

#[test]
fn autocomplete_dropdown_draws_hangul_labels_in_columns() {
    let mut ac = Autocomplete::new().suggestions(vec!["서울특별시"]);
    ac.focus();
    ac.handle_key(KeyEvent::new(Key::Char('서')));
    let buf = render(&ac, 20, 3);
    assert_eq!(sym(&buf, 0, 1), '서');
    assert!(is_cont(&buf, 1, 1), "row: {:?}", row_text(&buf, 1));
    assert_eq!(sym(&buf, 2, 1), '울');
    assert_eq!(row_text(&buf, 1).trim_end(), "서울특별시");
}

// ─── Form ───────────────────────────────────────────────────────────────────

#[test]
fn form_draws_hangul_label_placeholder_and_value_in_columns() {
    let state = FormState::new()
        .field("name", |f| f.label("이름").placeholder("홍길동"))
        .field("city", |f| f.label("도시"))
        .build();
    state
        .get("city")
        .unwrap()
        .value_signal()
        .set("서울".to_string());
    let form = Form::new(state);
    let buf = render(&form, 30, 12);

    let y = (0..12)
        .find(|&y| row_text(&buf, y).contains('이'))
        .expect("label row");
    let x = find_in_row(&buf, y, '이');
    assert!(is_cont(&buf, x + 1, y), "row: {:?}", row_text(&buf, y));
    assert_eq!(sym(&buf, x + 2, y), '름');

    let x = find_in_row(&buf, y + 1, '홍');
    assert!(is_cont(&buf, x + 1, y + 1));
    assert_eq!(sym(&buf, x + 2, y + 1), '길');

    let y = (0..12)
        .find(|&y| row_text(&buf, y).contains('서'))
        .expect("value row");
    let x = find_in_row(&buf, y, '서');
    assert!(is_cont(&buf, x + 1, y));
    assert_eq!(sym(&buf, x + 2, y), '울');
}

// ─── RichTextEditor dialog ──────────────────────────────────────────────────

#[test]
fn rich_text_link_dialog_draws_hangul_text_in_columns() {
    let mut editor = RichTextEditor::new().focused(true);
    editor.open_link_dialog();
    for ch in "한글".chars() {
        editor.handle_key(&Key::Char(ch));
    }
    let buf = render(&editor, 50, 12);

    let y = (0..12)
        .find(|&y| row_text(&buf, y).contains("Text:"))
        .expect("text field row");
    let x = find_in_row(&buf, y, '한');
    assert!(is_cont(&buf, x + 1, y), "row: {:?}", row_text(&buf, y));
    assert_eq!(sym(&buf, x + 2, y), '글');
    assert!(is_cont(&buf, x + 3, y));
}

#[test]
fn rich_text_link_dialog_keeps_the_end_of_long_text_in_view() {
    let mut editor = RichTextEditor::new().focused(true);
    editor.open_link_dialog();
    let text = "가나다라마바사아자차카타파하";
    for ch in text.chars() {
        editor.handle_key(&Key::Char(ch));
    }
    // 30 wide: the dialog is 26, its field 16 columns; the text is 28.
    let buf = render(&editor, 30, 12);
    let y = (0..12)
        .find(|&y| row_text(&buf, y).contains("Text:"))
        .expect("text field row");
    let x = find_in_row(&buf, y, '하');
    assert!(is_cont(&buf, x + 1, y), "row: {:?}", row_text(&buf, y));
    assert_eq!(sym(&buf, x - 2, y), '파', "row: {:?}", row_text(&buf, y));
    assert!(is_cont(&buf, x - 1, y));
}

// ─── MaskedInput ────────────────────────────────────────────────────────────

#[test]
fn masked_input_typing_and_deleting_hangul_moves_by_chars() {
    // The cursor was a byte index: the second syllable was inserted one
    // byte into the first and panicked.
    let mut input = MaskedInput::new();
    for ch in "비밀".chars() {
        input.insert_char(ch);
    }
    assert_eq!(input.get_value(), "비밀");
    assert_eq!(input.get_cursor(), 2);
    input.move_left();
    input.insert_char('a');
    assert_eq!(input.get_value(), "비a밀");
    input.delete_backward();
    input.delete_forward();
    assert_eq!(input.get_value(), "비");
    input.move_end();
    assert_eq!(input.get_cursor(), 1);
}

#[test]
fn masked_input_masks_one_char_per_hangul_syllable() {
    let input = MaskedInput::new().value("비밀번호");
    assert_eq!(input.masked_display(), "●●●●");
    assert_eq!(input.get_cursor(), 4);
}

#[test]
fn masked_input_show_last_slices_by_chars() {
    let input = MaskedInput::new()
        .mask_style(MaskStyle::ShowLast(2))
        .value("비밀번호");
    assert_eq!(input.masked_display(), "●●번호");
    let input = MaskedInput::new()
        .mask_style(MaskStyle::ShowFirst(1))
        .value("비밀번호");
    assert_eq!(input.masked_display(), "비●●●");
}

#[test]
fn masked_input_lengths_count_chars() {
    let mut input = MaskedInput::new().max_length(2);
    for ch in "비밀번".chars() {
        input.insert_char(ch);
    }
    assert_eq!(input.get_value(), "비밀");

    let mut input = MaskedInput::new().min_length(3).value("비밀");
    assert!(!input.validate(), "two chars pass a three-char minimum");
}
