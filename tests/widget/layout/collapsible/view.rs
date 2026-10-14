//! Collapsible placement, its height against the drawn rows, and a model of
//! the expanded state under keys

use proptest::prelude::*;
use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::Collapsible;

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

fn draw(c: &Collapsible, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    c.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// Rows with at least one drawn cell (a glyph or a color).
fn drawn_rows(buffer: &Buffer) -> u16 {
    (0..buffer.height())
        .filter(|&y| {
            (0..buffer.width()).any(|x| {
                let c = buffer.get(x, y).unwrap();
                c.symbol != ' ' || c.fg.is_some() || c.bg.is_some()
            })
        })
        .count() as u16
}

#[test]
fn test_collapsible_zero_areas_and_offset() {
    let base = || Collapsible::new("한글 Title").content("one\ntwo 世界");
    for c in [
        base(),
        base().expanded(true),
        base().expanded(true).border(false),
        base()
            .expanded(true)
            .header_colors(Color::CYAN, Some(Color::BLACK))
            .content_colors(Color::YELLOW, Some(Color::BLUE)),
        base().expanded(true).focused(true),
    ] {
        assert_zero_areas_draw_nothing(&c);
        assert_same_when_offset(c.clone(), 20, 5);
    }
}

#[test]
fn test_collapsible_max_size_cuts_the_drawing() {
    let c = Collapsible::new("Details")
        .content("a\nb\nc")
        .expanded(true);
    // max_width: the title is cut to the narrower width
    let buffer = draw(&c.clone().max_width(6), 20, 6);
    assert_eq!(row(&buffer, 0), "▼ Det");
    assert_eq!(row(&buffer, 4), "└────");
    // max_height: one line, then the bottom border
    let buffer = draw(&c.max_height(3), 20, 6);
    assert_eq!(row(&buffer, 1), "│ a");
    assert_eq!(row(&buffer, 2).chars().next(), Some('└'));
    assert_eq!(row(&buffer, 3), "");
}

#[test]
fn test_collapsible_disabled_header_is_gray_and_bold_when_focused() {
    // Disabled grays the header; focus still makes it bold
    let c = Collapsible::new("X")
        .header_colors(Color::CYAN, None)
        .disabled(true)
        .focused(true);
    let buffer = draw(&c, 10, 1);
    let icon = buffer.get(0, 0).unwrap();
    assert_eq!(icon.fg, Some(Color::rgb(100, 100, 100)));
    assert!(icon.modifier.contains(revue::render::Modifier::BOLD));
    assert_eq!(
        buffer.get(2, 0).unwrap().fg,
        Some(Color::rgb(100, 100, 100))
    );
}

#[test]
fn test_collapsible_default_colors() {
    let c = Collapsible::new("T").content("x").expanded(true);
    let buffer = draw(&c, 10, 3);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::WHITE));
    assert_eq!(buffer.get(2, 0).unwrap().fg, Some(Color::WHITE));
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(Color::rgb(80, 80, 80)));
    assert_eq!(
        buffer.get(2, 1).unwrap().fg,
        Some(Color::rgb(200, 200, 200))
    );
    assert_eq!(buffer.get(0, 2).unwrap().fg, Some(Color::rgb(80, 80, 80)));
    assert_eq!(buffer.get(5, 2).unwrap().fg, Some(Color::rgb(80, 80, 80)));
    // Not focused: not bold
    assert!(!buffer
        .get(0, 0)
        .unwrap()
        .modifier
        .contains(revue::render::Modifier::BOLD));
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        Just(Key::Enter),
        Just(Key::Char(' ')),
        Just(Key::Right),
        Just(Key::Left),
        Just(Key::Char('l')),
        Just(Key::Char('h')),
        Just(Key::Up),
        Just(Key::Down),
        Just(Key::Tab),
        Just(Key::Char('x')),
    ]
}

proptest! {
    /// Enter and Space toggle, Right and `l` expand, Left and `h` collapse;
    /// other keys and every key while disabled are not handled and change
    /// nothing. The icon drawn follows the state.
    #[test]
    fn prop_collapsible_keys_follow_the_model(
        start in any::<bool>(),
        disabled in any::<bool>(),
        keys in prop::collection::vec(key(), 0..30),
    ) {
        let mut c = Collapsible::new("K").content("a").expanded(start).disabled(disabled);
        let mut open = start;
        for k in &keys {
            let handled = !disabled && match k {
                Key::Enter | Key::Char(' ') => { open = !open; true }
                Key::Right | Key::Char('l') => { open = true; true }
                Key::Left | Key::Char('h') => { open = false; true }
                _ => false,
            };
            prop_assert_eq!(c.handle_key(k), handled, "{:?}", k);
            prop_assert_eq!(c.is_expanded(), open);
        }
        let icon = draw(&c, 10, 4).get(0, 0).unwrap().symbol;
        prop_assert_eq!(icon, if open { '▼' } else { '▶' });
    }

    /// `height()` is the number of rows drawn when the area is tall enough,
    /// and the drawing is cut to the area when it is not: the header, as many
    /// lines as fit, and the bottom border on the last row.
    #[test]
    fn prop_collapsible_height_matches_the_drawing(
        lines in prop::collection::vec("[a-z]{1,8}", 0..6),
        expanded in any::<bool>(),
        border in any::<bool>(),
        h in 1u16..10,
    ) {
        let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
        let c = Collapsible::new("T").lines(&refs).expanded(expanded).border(border);
        let height = c.height();
        let expected = if expanded { 1 + lines.len() as u16 + border as u16 } else { 1 };
        prop_assert_eq!(height, expected);

        let buffer = draw(&c, 20, h);
        prop_assert_eq!(drawn_rows(&buffer), height.min(h));
        if expanded && border && h > 1 {
            let last = height.min(h) - 1;
            prop_assert_eq!(buffer.get(0, last).unwrap().symbol, '└');
        }
        if expanded {
            let shown = (height.min(h) - 1 - (border as u16).min(height.min(h) - 1)) as usize;
            for (i, line) in lines.iter().take(shown).enumerate() {
                let text = row(&buffer, 1 + i as u16);
                prop_assert!(text.ends_with(line.as_str()), "{text:?} {line:?}");
            }
        }
    }
}
