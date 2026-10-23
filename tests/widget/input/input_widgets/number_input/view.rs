//! NumberInput on the screen: placement, colors, the cursor only while
//! focused, and a proptest that nothing is drawn past the field

use proptest::prelude::*;
use revue::event::Key;
use revue::style::Color;
use revue::widget::NumberInput;

use super::{render_row, type_keys};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

#[test]
fn test_number_input_zero_areas_draw_nothing() {
    let mut input = NumberInput::new()
        .value(12.0)
        .prefix("$")
        .suffix("%")
        .focused(true);
    assert_zero_areas_draw_nothing(&input);
    type_keys(&mut input, "34");
    assert_zero_areas_draw_nothing(&input);
}

#[test]
fn test_number_input_same_when_offset() {
    let mut input = NumberInput::new()
        .value(12.0)
        .prefix("$")
        .suffix("%")
        .width(12)
        .focused(true);
    assert_same_when_offset(input.clone(), 14, 1);
    type_keys(&mut input, "34");
    assert_same_when_offset(input, 14, 1);
}

#[test]
fn test_number_input_colors() {
    let input = NumberInput::new()
        .value(7.0)
        .prefix("$")
        .suffix("%")
        .width(10);
    let (buffer, text) = render_row(&input, 10);
    assert_eq!(text, "$7%    -+ ");
    let fg = |x| buffer.get(x, 0).unwrap().fg;
    // The prefix, the suffix and the buttons in one gray, the value white
    assert_eq!(fg(1), Some(Color::WHITE));
    assert!(buffer.get(1, 0).unwrap().bg.is_some());
    assert_ne!(fg(0), Some(Color::WHITE));
    assert_eq!(fg(0), fg(2));
    assert_eq!(fg(0), fg(7));

    let (custom, _) = render_row(&input.clone().fg(Color::GREEN).bg(Color::BLUE), 10);
    let cell = custom.get(1, 0).unwrap();
    assert_eq!((cell.fg, cell.bg), (Some(Color::GREEN), Some(Color::BLUE)));
}

#[test]
fn test_number_input_disabled_dims_the_value_and_the_buttons() {
    let input = NumberInput::new().value(7.0).width(10);
    let (enabled, _) = render_row(&input, 10);
    let (disabled, text) = render_row(&input.disabled(true), 10);
    assert_eq!(text, "7      -+ ");
    for x in [0, 7, 8] {
        assert_ne!(
            disabled.get(x, 0).unwrap().fg,
            enabled.get(x, 0).unwrap().fg,
            "{x}"
        );
    }
}

#[test]
fn test_number_input_cursor_only_while_focused() {
    let mut input = NumberInput::new().show_buttons(false);
    type_keys(&mut input, "12");
    assert!(input.is_editing());
    // The cursor past the end, then on the '2': not drawn either way
    for _ in 0..2 {
        let (buffer, text) = render_row(&input, 6);
        assert_eq!(text.trim_end(), "12");
        assert!((0..6).all(|x| buffer.get(x, 0).unwrap().bg != Some(Color::WHITE)));
        input.handle_key(&Key::Left);
    }
}

proptest! {
    /// Nothing is drawn past the field's width (or the area), and the
    /// value, the edit buffer and the buttons stay on row 0.
    #[test]
    fn prop_number_input_draws_inside_its_field(
        value in -1e6f64..1e6,
        precision in 0u8..4,
        prefix in proptest::option::of("[$a-z]{0,3}"),
        suffix in proptest::option::of("[%a-z ]{0,4}"),
        width in proptest::option::of(0u16..20),
        buttons in any::<bool>(),
        typed in "[0-9.-]{0,8}",
        focused in any::<bool>(),
    ) {
        let mut input = NumberInput::new().value(value).precision(precision).show_buttons(buttons).focused(focused);
        if let Some(p) = prefix {
            input = input.prefix(p);
        }
        if let Some(s) = suffix {
            input = input.suffix(s);
        }
        if let Some(w) = width {
            input = input.width(w);
        }
        type_keys(&mut input, &typed);
        let (buffer, _) = render_row(&input, 24);
        let field = width.map_or(24, |w| w.clamp(5, 24));
        for x in field..24 {
            let cell = buffer.get(x, 0).unwrap();
            prop_assert_eq!(cell.symbol, ' ', "{} past the field of {}", x, field);
            prop_assert_eq!(cell.bg, None);
        }
    }
}

// The typed text is drawn as every single-line field is (#985)

#[test]
fn test_number_input_selection_is_drawn() {
    use revue::event::KeyEvent;
    let mut input = NumberInput::new().focused(true).show_buttons(false);
    type_keys(&mut input, "12345");
    for _ in 0..2 {
        input.handle_key_event(&KeyEvent {
            shift: true,
            ..KeyEvent::new(Key::Left)
        });
    }
    let (buffer, text) = render_row(&input, 10);
    assert!(text.starts_with("12345"), "{text:?}");
    // The cursor on "4", "5" selected, "3" neither
    assert_eq!(buffer.get(3, 0).unwrap().bg, Some(Color::WHITE));
    assert_eq!(buffer.get(4, 0).unwrap().bg, Some(Color::BLUE));
    assert_ne!(buffer.get(2, 0).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_number_input_long_text_scrolls_to_the_cursor() {
    let mut input = NumberInput::new()
        .focused(true)
        .show_buttons(false)
        .width(6);
    type_keys(&mut input, "1234567890");
    let (buffer, text) = render_row(&input, 10);
    // The end and the cursor after it, in the six columns of the field
    assert_eq!(&text[..6], "67890 ");
    assert_eq!(buffer.get(5, 0).unwrap().bg, Some(Color::WHITE));
}
