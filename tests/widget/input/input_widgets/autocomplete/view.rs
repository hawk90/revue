//! Autocomplete rendering tests: the input row, the cursor, the dropdown, colors and areas

use super::{dropdown, fruits, key, render, render_rows, row, type_str};
use crate::traits::placement::assert_same_when_offset;
use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::utils::FilterMode;
use revue::widget::autocomplete::{Autocomplete, Suggestion};
use revue::widget::traits::{RenderContext, View};

#[test]
fn test_autocomplete_placeholder_hidden_by_value() {
    let a = Autocomplete::new().placeholder("Search...").value("abc");
    assert_eq!(render_rows(&a, 12, 1)[0].trim_end(), "abc");
}

#[test]
fn test_autocomplete_dropdown_clipped_to_height() {
    let mut a = fruits().filter_mode(FilterMode::None);
    type_str(&mut a, "x");
    // One row for the input, two for the dropdown
    assert_eq!(dropdown(&a, 20, 3), vec!["apple", "application"]);
}

#[test]
fn test_autocomplete_input_style() {
    let a = Autocomplete::new()
        .input_style(Color::CYAN, Color::BLACK)
        .value("ab");
    let buffer = render(&a, 5, 1);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::CYAN));
    assert_eq!(cell.bg, Some(Color::BLACK));
    // The rest of the input line is filled with the background
    assert_eq!(buffer.get(4, 0).unwrap().bg, Some(Color::BLACK));
}

#[test]
fn test_autocomplete_style_override() {
    let a = Autocomplete::new()
        .input_style(Color::RED, Color::BLACK)
        .input_style(Color::BLUE, Color::WHITE)
        .value("x");
    let buffer = render(&a, 5, 1);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::BLUE));
    assert_eq!(cell.bg, Some(Color::WHITE));
}

#[test]
fn test_autocomplete_dropdown_style() {
    let mut a = Autocomplete::new()
        .suggestions(["ab", "ac"])
        .dropdown_style(Color::BLUE, Color::WHITE, Color::GREEN)
        .highlight_fg(Color::YELLOW);
    type_str(&mut a, "a");
    let buffer = render(&a, 10, 3);
    // Selected row: matched 'a' highlighted, the rest in selected colors
    let first = buffer.get(0, 1).unwrap();
    assert_eq!(first.fg, Some(Color::YELLOW));
    assert_eq!(first.bg, Some(Color::GREEN));
    let second = buffer.get(1, 1).unwrap();
    assert_eq!(second.fg, Some(Color::WHITE));
    assert_eq!(second.bg, Some(Color::GREEN));
    // Unselected row uses the dropdown background
    assert_eq!(buffer.get(1, 2).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_autocomplete_highlight_fg() {
    let mut a = Autocomplete::new()
        .suggestions(["apple"])
        .highlight_fg(Color::YELLOW);
    type_str(&mut a, "ae");
    let buffer = render(&a, 10, 2);
    let highlighted: Vec<bool> = (0..5)
        .map(|x| buffer.get(x, 1).unwrap().fg == Some(Color::YELLOW))
        .collect();
    assert_eq!(highlighted, vec![true, false, false, false, true]);
}

#[test]
fn test_autocomplete_set_value_unicode_cursor_rendered_at_end() {
    let mut a = Autocomplete::new().input_style(Color::WHITE, Color::BLACK);
    a.focus();
    a.set_value("Привет");
    let buffer = render(&a, 20, 1);
    // The cursor cell is drawn inverted, right after the text
    assert_eq!(buffer.get(6, 0).unwrap().bg, Some(Color::WHITE));
    assert_eq!(buffer.get(5, 0).unwrap().bg, Some(Color::BLACK));
}

#[test]
fn test_autocomplete_render_empty() {
    let a = Autocomplete::new();
    for r in render_rows(&a, 20, 5) {
        assert_eq!(r.trim(), "");
    }
}

#[test]
fn test_autocomplete_render_with_value() {
    let a = Autocomplete::new().value("test");
    assert_eq!(render_rows(&a, 20, 5)[0].trim_end(), "test");
}

#[test]
fn test_autocomplete_render_focused() {
    // The focused cursor is drawn with inverted colors.
    let mut a = Autocomplete::new()
        .input_style(Color::WHITE, Color::BLACK)
        .value("ab");
    a.focus();
    let buffer = render(&a, 20, 5);
    let cursor = buffer.get(2, 0).unwrap();
    assert_eq!(cursor.fg, Some(Color::BLACK));
    assert_eq!(cursor.bg, Some(Color::WHITE));

    a.blur();
    let buffer = render(&a, 20, 5);
    assert_eq!(buffer.get(2, 0).unwrap().bg, Some(Color::BLACK));
}

#[test]
fn test_autocomplete_render_cursor_on_char() {
    let mut a = Autocomplete::new()
        .input_style(Color::WHITE, Color::BLACK)
        .value("ab");
    a.focus();
    key(&mut a, Key::Home);
    let buffer = render(&a, 20, 1);
    let cursor = buffer.get(0, 0).unwrap();
    assert_eq!(cursor.symbol, 'a');
    assert_eq!(cursor.bg, Some(Color::WHITE));
}

#[test]
fn test_autocomplete_render_zero_width() {
    let a = Autocomplete::new().value("x");
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 0, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_autocomplete_render_zero_height() {
    let a = Autocomplete::new().value("x");
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_autocomplete_render_very_narrow() {
    // Below three columns nothing is drawn; at three the input appears.
    let a = Autocomplete::new().value("abcd");
    assert_eq!(render_rows(&a, 2, 1)[0], "  ");
    assert_eq!(render_rows(&a, 3, 1)[0], "abc");
}

#[test]
fn test_autocomplete_render_with_suggestions() {
    let mut a = fruits().value("app");
    a.focus();
    let rows = render_rows(&a, 30, 10);
    assert_eq!(rows[0].trim_end(), "app");
    assert_eq!(rows[1].trim_end(), "apple");
    assert_eq!(rows[2].trim_end(), "application");
    assert_eq!(rows[3].trim_end(), "");
}

#[test]
fn test_autocomplete_render_icon_and_description() {
    let mut a = Autocomplete::new().suggestions(vec![Suggestion::new("main.rs")
        .icon('#')
        .description("entry")]);
    type_str(&mut a, "main");
    assert_eq!(render_rows(&a, 20, 2)[1].trim_end(), "# main.rs entry");
}

#[test]
fn test_autocomplete_render_long_value() {
    let a = Autocomplete::new().value("this is a very long value");
    assert_eq!(render_rows(&a, 20, 5)[0], "this is a very long ");
}

#[test]
fn test_autocomplete_render_unicode_value() {
    let a = Autocomplete::new().value("こんにちは世界");
    assert!(render_rows(&a, 20, 5)[0].starts_with("こんにちは世界"));
}

#[test]
fn test_autocomplete_render_offset_area() {
    let a = Autocomplete::new().value("test");
    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(10, 2, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    assert_eq!(row(&buffer, 2, 50)[10..14].to_string(), "test");
    assert_eq!(row(&buffer, 0, 50).trim(), "");
}

#[test]
fn test_autocomplete_draws_the_same_when_offset() {
    let mut a = Autocomplete::new().suggestions(["alpha", "beta", "albatross"]);
    a.focus();
    type_str(&mut a, "al");
    assert_same_when_offset(a, 20, 5);
}

/// The dropdown drew the first suggestions that fit while Down moved over
/// all of them, so the highlighted one could be below the area (#882).
#[test]
fn test_autocomplete_dropdown_follows_the_highlight() {
    let mut a = Autocomplete::new()
        .suggestions((1..=10).map(|i| format!("item{i:02}")).collect::<Vec<_>>());
    type_str(&mut a, "item");
    for _ in 0..6 {
        key(&mut a, Key::Down);
    }
    // 4 rows below the input: item04..item07, item07 highlighted
    assert_eq!(
        dropdown(&a, 20, 5),
        ["item04", "item05", "item06", "item07"]
    );
    key(&mut a, Key::Up);
    assert_eq!(
        dropdown(&a, 20, 5),
        ["item04", "item05", "item06", "item07"]
    );
}

// The field keeps its scroll and draws the selection (#985)

/// The columns drawn with the cursor's colors (the input bg on the text
/// color) on row 0
fn cursor_cells(buffer: &Buffer, width: u16) -> Vec<u16> {
    (0..width)
        .filter(|&x| buffer.get(x, 0).unwrap().fg == Some(revue::widget::theme::EDITOR_BG))
        .collect()
}

#[test]
fn test_autocomplete_scroll_is_kept_while_the_cursor_moves_inside_the_view() {
    let mut a = Autocomplete::new().value("abcdefghij");
    a.focus();
    // 6 columns: the end and the cursor after it
    let buffer = render(&a, 6, 1);
    assert_eq!(row(&buffer, 0, 6), "fghij ");
    assert_eq!(cursor_cells(&buffer, 6), vec![5]);
    // Two steps left stay inside the view: it does not move
    key(&mut a, Key::Left);
    key(&mut a, Key::Left);
    let buffer = render(&a, 6, 1);
    assert_eq!(row(&buffer, 0, 6), "fghij ");
    assert_eq!(cursor_cells(&buffer, 6), vec![3]);
    // Unfocused, the value shows from its start
    a.blur();
    assert_eq!(row(&render(&a, 6, 1), 0, 6), "abcdef");
}

#[test]
fn test_autocomplete_selection_is_drawn() {
    let mut a = Autocomplete::new().value("abcd");
    a.focus();
    a.handle_key(revue::event::KeyEvent {
        shift: true,
        ..revue::event::KeyEvent::ctrl(Key::Left)
    });
    let buffer = render(&a, 8, 1);
    // The whole word is selected and the cursor sits on its first letter
    assert_eq!(cursor_cells(&buffer, 8), vec![0]);
    for x in 1..4 {
        assert_eq!(
            buffer.get(x, 0).unwrap().bg,
            Some(Color::rgb(60, 100, 180)),
            "{x}"
        );
    }
    assert_ne!(buffer.get(4, 0).unwrap().bg, Some(Color::rgb(60, 100, 180)));
}
