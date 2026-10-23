//! Input proptests: a model of the plain keys, undo and redo, selection,
//! and the cursor on the screen

use proptest::prelude::*;
use revue::event::{Key, KeyEvent};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::Input;

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        Just(Key::Char('a')),
        Just(Key::Char(' ')),
        Just(Key::Char('한')),
        Just(Key::Char('\t')),
        Just(Key::Backspace),
        Just(Key::Delete),
        Just(Key::Left),
        Just(Key::Right),
        Just(Key::Home),
        Just(Key::End),
        Just(Key::Enter),
    ]
}

fn shift(key: Key) -> KeyEvent {
    let mut e = KeyEvent::new(key);
    e.shift = true;
    e
}

#[test]
fn test_input_zero_areas_and_offset() {
    let mut input = Input::new().value("한글 abc").placeholder("p");
    input.handle_key(&Key::Left);
    assert_zero_areas_draw_nothing(&input);
    assert_same_when_offset(input, 10, 1);
}

proptest! {
    /// Any plain keys leave the text and the cursor a char model gives:
    /// printable chars insert (control chars such as Tab are dropped),
    /// Backspace and Delete remove one char, and the moves stop at the ends.
    #[test]
    fn prop_input_keys_follow_the_model(keys in prop::collection::vec(key(), 0..60)) {
        let mut input = Input::new();
        let (mut text, mut cur): (Vec<char>, usize) = (Vec::new(), 0);
        for k in &keys {
            let changed = match k {
                Key::Char(c) if !c.is_control() => { text.insert(cur, *c); cur += 1; true }
                Key::Backspace if cur > 0 => { cur -= 1; text.remove(cur); true }
                Key::Delete if cur < text.len() => { text.remove(cur); true }
                Key::Left => { cur = cur.saturating_sub(1); true }
                Key::Right => { cur = (cur + 1).min(text.len()); true }
                Key::Home => { cur = 0; true }
                Key::End => { cur = text.len(); true }
                _ => false,
            };
            prop_assert_eq!(input.handle_key(k), changed, "{:?}", k);
            prop_assert_eq!(input.text(), text.iter().collect::<String>(), "after {:?}", k);
            prop_assert_eq!(input.cursor(), cur, "after {:?}", k);
        }
    }

    /// Undoing every edit restores the empty start, and redoing every one
    /// restores the end.
    #[test]
    fn prop_input_undo_and_redo_round_trip(keys in prop::collection::vec(key(), 0..60)) {
        let mut input = Input::new();
        for k in &keys {
            input.handle_key(k);
        }
        let end = input.text().to_string();
        while input.undo() {}
        prop_assert_eq!(input.text(), "");
        while input.redo() {}
        prop_assert_eq!(input.text(), end);
    }

    /// Shift+Left and Shift+Right from a cursor select the chars between
    /// the anchor and the cursor; typing replaces them.
    #[test]
    fn prop_input_shift_selection(
        start in 0usize..8,
        moves in prop::collection::vec(any::<bool>(), 0..10),
    ) {
        let text = "ab 한글 cd";
        let chars: Vec<char> = text.chars().collect();
        let mut input = Input::new().value(text);
        input.handle_key(&Key::Home);
        for _ in 0..start {
            input.handle_key(&Key::Right);
        }
        let mut cur = start;
        for right in &moves {
            input.handle_key_event(&shift(if *right { Key::Right } else { Key::Left }));
            cur = if *right { (cur + 1).min(chars.len()) } else { cur.saturating_sub(1) };
        }
        let (lo, hi) = (start.min(cur), start.max(cur));
        let expected: String = chars[lo..hi].iter().collect();
        prop_assert_eq!(input.selected_text().unwrap_or(""), expected.as_str());
        input.handle_key(&Key::Char('X'));
        let mut after = chars.clone();
        after.splice(lo..hi, ['X']);
        prop_assert_eq!(input.text(), after.iter().collect::<String>());
    }

    /// After any keys, a focused input draws its cursor once in the area,
    /// one cell or a wide glyph's two, as the edits scroll the text.
    #[test]
    fn prop_input_cursor_on_screen(
        keys in prop::collection::vec(key(), 0..60),
        width in 3u16..12,
    ) {
        let mut input = Input::new().cursor_style(Color::BLACK, Color::YELLOW);
        let draw = |input: &Input| {
            let mut buffer = Buffer::new(width, 1);
            let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, 1));
            input.render(&mut ctx);
            buffer
        };
        // Drawn between keys, as an app does, so the kept scroll is used
        for k in &keys {
            input.handle_key(k);
            draw(&input);
        }
        let buffer = draw(&input);
        let cells: Vec<u16> = (0..width).filter(|&x| buffer.get(x, 0).unwrap().bg == Some(Color::YELLOW)).collect();
        prop_assert!(matches!(cells.len(), 1 | 2), "{:?}", cells);
        prop_assert!(cells.len() == 1 || cells[1] == cells[0] + 1);
    }
}

#[test]
fn test_input_unfocused_ignores_key_events() {
    let mut input = Input::new().value("ab").focused(false);
    for key in [Key::Char('x'), Key::Backspace, Key::Left, Key::Home] {
        assert!(!input.handle_key_event(&KeyEvent::new(key)), "{key:?}");
    }
    assert!(!input.handle_key_event(&KeyEvent::ctrl(Key::Char('a'))));
    assert_eq!((input.text(), input.cursor()), ("ab", 2));
}

#[test]
fn test_input_scroll_is_kept_while_the_cursor_moves_inside_the_view() {
    let mut input = Input::new().value("abcdefghij");
    let row = |input: &Input| {
        let mut buffer = Buffer::new(5, 1);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 5, 1));
        input.render(&mut ctx);
        (0..5)
            .map(|x| buffer.get(x, 0).unwrap().symbol)
            .collect::<String>()
    };
    // At the end: the last four chars and the cursor after them
    assert_eq!(row(&input), "ghij ");
    // Moving left inside the view does not move the view
    input.handle_key(&Key::Left);
    input.handle_key(&Key::Left);
    assert_eq!(row(&input), "ghij ");
}

// The cursor goes over grapheme clusters; positions are still characters
// (#1261)

#[test]
fn the_cursor_steps_over_a_whole_cluster() {
    // "e" + combining acute, then "x": the accent is never stepped into
    let mut input = Input::new().value("e\u{301}x");
    assert_eq!(input.cursor(), 3);
    input.handle_key(&Key::Left);
    assert_eq!(input.cursor(), 2);
    input.handle_key(&Key::Left);
    assert_eq!(input.cursor(), 0);
    input.handle_key(&Key::Delete);
    assert_eq!(input.text(), "x");
}

#[test]
fn backspace_takes_an_accent_but_an_emoji_sequence_whole() {
    let mut input = Input::new().value("e\u{301}");
    input.handle_key(&Key::Backspace);
    assert_eq!((input.text(), input.cursor()), ("e", 1));
    let mut input = Input::new().value("a👨\u{200D}👩\u{200D}👧");
    input.handle_key(&Key::Backspace);
    assert_eq!((input.text(), input.cursor()), ("a", 1));
}
