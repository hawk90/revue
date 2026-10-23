//! SearchBar on the screen: placement, the cursor, scrolling to it, and a
//! proptest of the field after any keys

use proptest::prelude::*;
use revue::event::Key;
use revue::style::Color;
use revue::widget::SearchBar;

use super::{row, screen, BG};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

/// Where the cursor is drawn: cells on the text color's background.
fn cursor_cells(buffer: &revue::render::Buffer) -> Vec<u16> {
    (0..buffer.width())
        .filter(|&x| buffer.get(x, 0).unwrap().bg == Some(Color::WHITE))
        .collect()
}

#[test]
fn test_search_bar_zero_areas_draw_nothing() {
    let mut s = SearchBar::new();
    s.set_query("author:john");
    s.focus();
    assert_zero_areas_draw_nothing(&s);
}

#[test]
fn test_search_bar_same_when_offset() {
    let mut s = SearchBar::new().width(20);
    s.set_query("한글 query");
    s.focus();
    assert_same_when_offset(s, 24, 2);
}

#[test]
fn test_search_bar_cursor_drawn_inverted_on_its_char() {
    let mut s = SearchBar::new();
    s.set_query("abc");
    s.cursor_left();
    assert!(cursor_cells(&screen(&s, 20, 1)).is_empty());
    s.focus();
    let buffer = screen(&s, 20, 1);
    // On 'c', in the background color
    assert_eq!(cursor_cells(&buffer), vec![6]);
    let cell = buffer.get(6, 0).unwrap();
    assert_eq!((cell.symbol, cell.fg), ('c', Some(BG)));
    // Past the end: a blank cell after the text
    s.cursor_end();
    assert_eq!(cursor_cells(&screen(&s, 20, 1)), vec![7]);
}

#[test]
fn test_search_bar_long_query_scrolls_to_the_cursor() {
    let mut s = SearchBar::new().width(12);
    s.set_query("abcdefghij한");
    s.focus();
    // 5 columns (the error mark keeps the sixth, #1014): the end of the
    // text and the cursor after it
    let buffer = screen(&s, 12, 1);
    assert_eq!(row(&buffer, 0), "│ 🔍ij한   │");
    assert_eq!(cursor_cells(&buffer), vec![8]);
    s.cursor_home();
    let buffer = screen(&s, 12, 1);
    assert_eq!(row(&buffer, 0), "│ 🔍abcde  │");
    assert_eq!(cursor_cells(&buffer), vec![4]);
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        Just(Key::Char('a')),
        Just(Key::Char(':')),
        Just(Key::Char(' ')),
        Just(Key::Char('한')),
        Just(Key::Backspace),
        Just(Key::Delete),
        Just(Key::Left),
        Just(Key::Right),
        Just(Key::Home),
        Just(Key::End),
        Just(Key::Escape),
    ]
}

proptest! {
    /// After any keys, a focused bar draws its cursor once in the field
    /// (columns 4 to width - 3), one cell or a wide glyph's two, and the
    /// field shows a part of the query. (Invalid queries are skipped: the
    /// error mark covers the last column, #1014.)
    #[test]
    fn prop_search_bar_cursor_in_the_field(
        keys in prop::collection::vec(key(), 0..40),
        width in 10u16..30,
    ) {
        let mut s = SearchBar::new().width(width);
        s.focus();
        for k in &keys {
            s.handle_key(k);
        }
        prop_assume!(s.is_valid());
        let buffer = screen(&s, width, 1);
        let cells = cursor_cells(&buffer);
        prop_assert!(matches!(cells.len(), 1 | 2), "{:?} {:?}", cells, row(&buffer, 0));
        prop_assert!(cells.iter().all(|&x| (4..width - 2).contains(&x)), "{:?}", cells);
        prop_assert!(cells.len() == 1 || cells[1] == cells[0] + 1);
        let field: String = row(&buffer, 0).chars().skip(3).take_while(|&c| c != '│').collect();
        if !s.get_input().is_empty() {
            prop_assert!(s.get_input().contains(field.trim()), "{:?} in {:?}", field, s.get_input());
        }
    }
}

// The error mark and the placeholder (#1014)

#[test]
fn the_error_mark_covers_neither_text_nor_cursor() {
    let mut s = SearchBar::new().width(12);
    s.set_query("sort:x:bad");
    assert!(!s.is_valid());
    s.cursor_home();
    s.focus();
    let buffer = screen(&s, 12, 1);
    // The field shows "sort:"; the mark has its own column after it
    assert_eq!(row(&buffer, 0), "│ 🔍sort:! │");
    s.cursor_end();
    let buffer = screen(&s, 12, 1);
    assert_eq!(cursor_cells(&buffer), vec![8]);
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '!');
}

#[test]
fn the_cursor_keeps_the_placeholders_first_letter() {
    let mut s = SearchBar::new();
    s.focus();
    let buffer = screen(&s, 30, 1);
    assert!(
        row(&buffer, 0).starts_with("│ 🔍Search..."),
        "{:?}",
        row(&buffer, 0)
    );
    assert_eq!(cursor_cells(&buffer), vec![4]);
}
