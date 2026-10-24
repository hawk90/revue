//! TextArea on the screen: placement, the gutter, the placeholder, colors,
//! the cursor, the selection, find matches and syntax colors

use proptest::prelude::*;
use revue::event::Key;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::syntax::Language;
use revue::widget::traits::{Interactive, RenderContext, View};
use revue::widget::TextArea;

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

fn render(ta: &TextArea, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    ta.render(&mut ctx);
    buffer
}

/// Row `y`, without wide glyphs' continuation cells and trailing blanks.
fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .filter(|&c| c != '\0')
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// Where the cursor is drawn: cells on a white background.
fn cursor_cells(buffer: &Buffer) -> Vec<(u16, u16)> {
    let mut cells = Vec::new();
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            if buffer.get(x, y).unwrap().bg == Some(Color::WHITE) {
                cells.push((x, y));
            }
        }
    }
    cells
}

#[test]
fn test_textarea_zero_areas_draw_nothing() {
    assert_zero_areas_draw_nothing(
        &TextArea::new()
            .content("one\ntwo")
            .line_numbers(true)
            .bg(Color::BLUE)
            .focused(true),
    );
}

#[test]
fn test_textarea_same_when_offset() {
    let mut ta = TextArea::new()
        .content("fn main() {\n    let 한 = 1;\n}")
        .line_numbers(true)
        .bg(Color::BLUE)
        .focused(true);
    ta.set_cursor(1, 6);
    assert_same_when_offset(ta, 14, 4);
}

#[test]
fn test_textarea_gutter_right_aligns_numbers() {
    let text: Vec<String> = (1..=10).map(|i| format!("l{i}")).collect();
    let ta = TextArea::new()
        .content(text.join("\n"))
        .line_numbers(true)
        .line_number_fg(Color::CYAN);
    let buffer = render(&ta, 10, 10);
    // Two digits for ten lines, then a space and a blank separator column
    assert_eq!(row(&buffer, 0), " 1  l1");
    assert_eq!(row(&buffer, 9), "10  l10");
    assert_eq!(buffer.get(1, 0).unwrap().fg, Some(Color::CYAN));
    assert_eq!(buffer.get(4, 0).unwrap().fg, None);
}

#[test]
fn test_textarea_placeholder_only_while_empty() {
    let mut ta = TextArea::new().placeholder("Write here").line_numbers(true);
    let buffer = render(&ta, 20, 3);
    assert_eq!(row(&buffer, 0), "1  Write here");
    assert_ne!(buffer.get(3, 0).unwrap().fg, None);
    ta.set_content("x");
    assert_eq!(row(&render(&ta, 20, 3), 0), "1  x");
}

#[test]
fn test_textarea_placeholder_clipped_to_the_text_width() {
    let ta = TextArea::new().placeholder("Write here").line_numbers(true);
    // 3 gutter columns leave 4 for the placeholder
    assert_eq!(row(&render(&ta, 7, 3), 0), "1  Writ");
}

#[test]
fn test_textarea_bg_fills_the_area_and_fg_colors_the_text() {
    let ta = TextArea::new()
        .content("ab")
        .fg(Color::GREEN)
        .bg(Color::BLUE);
    let buffer = render(&ta, 6, 3);
    for y in 0..3 {
        for x in 0..6 {
            assert_eq!(
                buffer.get(x, y).unwrap().bg,
                Some(Color::BLUE),
                "({x}, {y})"
            );
        }
    }
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::GREEN));
}

#[test]
fn test_textarea_cursor_drawn_only_when_focused() {
    let ta = TextArea::new().content("ab").cursor_fg(Color::RED);
    assert!(cursor_cells(&render(&ta, 6, 3)).is_empty());

    let ta = ta.focused(true);
    let buffer = render(&ta, 6, 3);
    assert_eq!(cursor_cells(&buffer), vec![(0, 0)]);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'a');
    assert_eq!(cell.fg, Some(Color::RED));
    assert!(cell.modifier.contains(Modifier::BOLD));
}

/// A zero-width char takes no cell, so a cursor before one was not drawn:
/// it is drawn on the next glyph, or past the end when none follows
#[test]
fn test_textarea_cursor_on_a_zero_width_char_is_drawn() {
    let mut ta = TextArea::new().content("a\u{200B}b").focused(true);
    ta.set_cursor(0, 1);
    let buffer = render(&ta, 6, 1);
    assert_eq!(cursor_cells(&buffer), vec![(1, 0)]);
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'b');

    let mut ta = TextArea::new().content("a\u{200B}").focused(true);
    ta.set_cursor(0, 1);
    assert_eq!(cursor_cells(&render(&ta, 6, 1)), vec![(1, 0)]);

    // A combining mark at the start of a line is a character of its own
    let ta = TextArea::new().content("\u{301}b").focused(true);
    let buffer = render(&ta, 6, 1);
    assert_eq!(cursor_cells(&buffer), vec![(0, 0)]);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'b');
}

#[test]
fn test_textarea_cursor_past_the_end_drawn_after_the_text() {
    let mut ta = TextArea::new().content("ab\n한").focused(true);
    ta.set_cursor(0, 2);
    assert_eq!(cursor_cells(&render(&ta, 6, 3)), vec![(2, 0)]);
    // After a wide glyph: two columns in
    ta.set_cursor(1, 1);
    assert_eq!(cursor_cells(&render(&ta, 6, 3)), vec![(2, 1)]);
}

#[test]
fn test_textarea_selection_drawn_with_selection_bg() {
    let mut ta = TextArea::new()
        .content("abcd")
        .selection_bg(Color::MAGENTA)
        .focused(true);
    ta.set_cursor(0, 1);
    ta.start_selection();
    ta.move_right();
    ta.move_right();
    assert_eq!(ta.get_selection().as_deref(), Some("bc"));
    let buffer = render(&ta, 6, 3);
    for x in 1..3 {
        let cell = buffer.get(x, 0).unwrap();
        assert_eq!(
            (cell.fg, cell.bg),
            (Some(Color::WHITE), Some(Color::MAGENTA)),
            "{x}"
        );
    }
    assert_eq!(buffer.get(0, 0).unwrap().bg, None);
    // The cursor, at the selection's end, is drawn over it
    assert_eq!(cursor_cells(&buffer), vec![(3, 0)]);
}

#[test]
fn test_textarea_find_matches_use_their_colors() {
    let mut ta = TextArea::new()
        .content("ab ab x")
        .match_highlight_bg(Color::YELLOW)
        .current_match_bg(Color::RED);
    ta.open_find();
    ta.set_find_query("ab");
    let buffer = render(&ta, 10, 3);
    for x in 0..2 {
        assert_eq!(buffer.get(x, 0).unwrap().bg, Some(Color::RED), "{x}");
        assert_eq!(buffer.get(x + 3, 0).unwrap().bg, Some(Color::YELLOW), "{x}");
    }
    assert_eq!(buffer.get(2, 0).unwrap().bg, None);
    assert_eq!(buffer.get(6, 0).unwrap().bg, None);
}

#[test]
fn test_textarea_syntax_colors_keywords() {
    let ta = TextArea::new().content("fn x").syntax(Language::Rust);
    let buffer = render(&ta, 6, 3);
    let keyword = buffer.get(0, 0).unwrap().fg;
    assert!(keyword.is_some());
    assert_eq!(buffer.get(1, 0).unwrap().fg, keyword);
    assert_ne!(buffer.get(3, 0).unwrap().fg, keyword);
}

#[test]
fn test_textarea_unwrapped_line_scrolls_to_the_cursor() {
    let mut ta = TextArea::new()
        .content("abcdefghij한")
        .wrap(false)
        .focused(true);
    ta.set_cursor(0, 10);
    let buffer = render(&ta, 6, 3);
    // The whole wide glyph under the cursor is in view, at the right edge
    assert_eq!(row(&buffer, 0), "ghij한");
    assert_eq!(cursor_cells(&buffer), vec![(4, 0), (5, 0)]);
    ta.handle_key(&Key::Home);
    assert_eq!(row(&render(&ta, 6, 3), 0), "abcdef");
}

#[test]
fn test_textarea_focus_and_blur_show_and_hide_the_cursor() {
    let mut ta = TextArea::new().content("ab");
    ta.on_focus();
    assert_eq!(cursor_cells(&render(&ta, 6, 3)), vec![(0, 0)]);
    ta.on_blur();
    assert!(cursor_cells(&render(&ta, 6, 3)).is_empty());
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        Just(Key::Char('a')),
        Just(Key::Char(' ')),
        Just(Key::Char('한')),
        Just(Key::Enter),
        Just(Key::Backspace),
        Just(Key::Delete),
        Just(Key::Left),
        Just(Key::Right),
        Just(Key::Up),
        Just(Key::Down),
        Just(Key::Home),
        Just(Key::End),
        Just(Key::PageUp),
        Just(Key::PageDown),
    ]
}

proptest! {
    /// After any edits and moves, the cursor is drawn once in the area,
    /// wrapped or not, with or without the gutter: past a full row, on a
    /// space past the width, and deep in a line taller than the view too
    /// (#1008). (`min_height(0)`: the default of 3 keeps the cursor in rows the
    /// area does not have, #799.)
    #[test]
    fn prop_textarea_cursor_always_on_screen(
        keys in prop::collection::vec(key(), 0..60),
        wrap in any::<bool>(),
        numbers in any::<bool>(),
        w in 6u16..14,
        h in 1u16..6,
    ) {
        let mut ta = TextArea::new()
            .wrap(wrap)
            .line_numbers(numbers)
            .min_height(0)
            .focused(true);
        for k in &keys {
            ta.handle_key(k);
            // Render between keys, as an app does, so the scroll is kept
            render(&ta, w, h);
        }
        let buffer = render(&ta, w, h);
        let cells = cursor_cells(&buffer);
        let shown: Vec<String> = (0..h).map(|y| row(&buffer, y)).collect();
        // One glyph: one cell, or two for a wide glyph, side by side
        prop_assert!(matches!(cells.len(), 1 | 2), "{:?} {:?}", cells, shown);
        prop_assert!(cells.iter().all(|&(x, y)| y == cells[0].1 && x - cells[0].0 < 2), "{:?} {:?}", cells, shown);
    }
}
