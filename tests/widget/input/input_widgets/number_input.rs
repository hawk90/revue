//! Tests for the NumberInput widget, through its public API

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::View;
use revue::widget::{currency_input, integer_input, number_input, percentage_input, NumberInput};

fn render_row(input: &NumberInput, width: u16) -> (Buffer, String) {
    let mut buffer = Buffer::new(width, 1);
    let area = Rect::new(0, 0, width, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    input.render(&mut ctx);
    let text = (0..width)
        .map(|x| buffer.get(x, 0).map(|c| c.symbol).unwrap_or(' '))
        .collect::<String>();
    (buffer, text)
}

/// Press each character of `keys` as a key.
fn type_keys(input: &mut NumberInput, keys: &str) {
    for c in keys.chars() {
        input.handle_key(&Key::Char(c));
    }
}

/// Enter editing mode with exactly `buffer` in the edit buffer, cursor at its end.
fn editing(value: f64, buffer: &str) -> NumberInput {
    let mut input = NumberInput::new().value(value);
    type_keys(&mut input, buffer);
    assert!(input.is_editing());
    assert_eq!(input.display_string(), buffer);
    input
}

/// The edit buffer with a `9` typed at the cursor, which shows where it is.
fn at_cursor(input: &NumberInput) -> String {
    let mut probe = input.clone();
    probe.handle_key(&Key::Char('9'));
    probe.display_string()
}

// =========================================================================
// Constructor Tests
// =========================================================================

#[test]
fn test_number_input_new_creates_default_widget() {
    let mut input = NumberInput::new();
    assert_eq!(input.get_value(), 0.0);
    assert!(!input.is_editing());
    // Precision 0, no prefix or suffix.
    assert_eq!(input.display_string(), "0");
    // Buttons are shown.
    assert!(render_row(&input, 20).1.contains("-+"));
    // Step 1, no min or max.
    input.increment();
    assert_eq!(input.get_value(), 1.0);
    input.set_value(-1e9);
    assert_eq!(input.get_value(), -1e9);
    input.set_value(1e9);
    assert_eq!(input.get_value(), 1e9);
    input.set_to_min();
    input.set_to_max();
    assert_eq!(input.get_value(), 1e9);
}

#[test]
fn test_number_input_default_trait() {
    let mut input = NumberInput::default();
    assert_eq!(input.get_value(), 0.0);
    input.increment();
    assert_eq!(input.get_value(), 1.0);
}

// =========================================================================
// Builder Method Tests
// =========================================================================

#[test]
fn test_number_input_value_builder() {
    let input = NumberInput::new().value(42.0);
    assert_eq!(input.get_value(), 42.0);
}

#[test]
fn test_number_input_value_with_clamping() {
    let input = NumberInput::new().min(0.0).max(100.0).value(150.0);
    assert_eq!(input.get_value(), 100.0); // Clamped to max

    let input = NumberInput::new().min(0.0).max(100.0).value(-50.0);
    assert_eq!(input.get_value(), 0.0); // Clamped to min
}

#[test]
fn test_number_input_min_builder() {
    let mut input = NumberInput::new().min(10.0).value(50.0);
    input.set_to_min();
    assert_eq!(input.get_value(), 10.0);
}

#[test]
fn test_number_input_min_clamps_current_value() {
    let input = NumberInput::new().value(5.0).min(10.0);
    assert_eq!(input.get_value(), 10.0); // Clamped up to min
}

#[test]
fn test_number_input_max_builder() {
    let mut input = NumberInput::new().max(100.0);
    input.set_to_max();
    assert_eq!(input.get_value(), 100.0);
}

#[test]
fn test_number_input_max_clamps_current_value() {
    let input = NumberInput::new().value(150.0).max(100.0);
    assert_eq!(input.get_value(), 100.0); // Clamped down to max
}

#[test]
fn test_number_input_step_builder() {
    let mut input = NumberInput::new().step(5.0);
    input.increment();
    assert_eq!(input.get_value(), 5.0);

    let mut input = NumberInput::new().step(-5.0);
    input.increment();
    assert_eq!(input.get_value(), 5.0); // Absolute value applied
}

#[test]
fn test_number_input_precision_builder() {
    let input = NumberInput::new().value(1.0).precision(2);
    assert_eq!(input.display_string(), "1.00");
}

#[test]
fn test_number_input_prefix_builder() {
    let input = NumberInput::new().prefix("$");
    assert_eq!(input.display_string(), "$0");
}

#[test]
fn test_number_input_suffix_builder() {
    let input = NumberInput::new().suffix("%");
    assert_eq!(input.display_string(), "0%");
}

#[test]
fn test_number_input_show_buttons_builder() {
    let input = NumberInput::new().show_buttons(true);
    assert_eq!(render_row(&input, 10).1, "0      -+ ");

    let input = NumberInput::new().show_buttons(false);
    assert_eq!(render_row(&input, 10).1.trim_end(), "0");
}

#[test]
fn test_number_input_width_builder() {
    // A 10-wide field keeps its buttons inside its own 10 columns.
    let input = NumberInput::new().width(10);
    assert_eq!(render_row(&input, 30).1.trim_end(), "0      -+");
    // Nothing is drawn past the field: no background, no text.
    let (buffer, _) = render_row(&input, 30);
    assert!((10..30).all(|x| buffer.get(x, 0).unwrap().bg.is_none()));
    // Prefix and suffix are clipped to the field too.
    let input = NumberInput::new()
        .width(6)
        .show_buttons(false)
        .value(1.0)
        .suffix(" units");
    assert_eq!(render_row(&input, 30).1.trim_end(), "1 unit");
    // A field wider than the area is clipped to the area.
    let input = NumberInput::new().width(20);
    assert_eq!(render_row(&input, 10).1, "0      -+ ");
    // Widths below 5 are raised to 5.
    let input = NumberInput::new().width(3).show_buttons(false);
    let narrow = NumberInput::new().width(5).show_buttons(false);
    assert_eq!(render_row(&input, 30).1, render_row(&narrow, 30).1);
}

#[test]
fn test_number_input_buttons_need_room() {
    // The buttons only draw when they fit after the value.
    let input = NumberInput::new().value(12345.0);
    assert_eq!(render_row(&input, 8).1, "12345   ");
    assert_eq!(render_row(&input, 9).1, "12345 -+ ");
}

#[test]
fn test_number_input_builder_chaining() {
    let mut input = NumberInput::new()
        .value(50.0)
        .min(0.0)
        .max(100.0)
        .step(5.0)
        .precision(2)
        .prefix("$")
        .suffix(" USD")
        .width(15)
        .show_buttons(true);

    assert_eq!(input.get_value(), 50.0);
    assert_eq!(input.display_string(), "$50.00 USD");
    input.increment();
    assert_eq!(input.get_value(), 55.0);
    input.set_to_min();
    assert_eq!(input.get_value(), 0.0);
    input.set_to_max();
    assert_eq!(input.get_value(), 100.0);
}

// =========================================================================
// Value Getter Tests
// =========================================================================

#[test]
fn test_number_input_get_value() {
    let input = NumberInput::new().value(42.5);
    assert_eq!(input.get_value(), 42.5);
}

#[test]
fn test_number_input_get_int() {
    let input = NumberInput::new().value(42.7);
    assert_eq!(input.get_int(), 43);

    let input = NumberInput::new().value(42.2);
    assert_eq!(input.get_int(), 42);
}

// =========================================================================
// Value Setter Tests
// =========================================================================

#[test]
fn test_number_input_set_value() {
    let mut input = NumberInput::new();
    input.set_value(42.0);
    assert_eq!(input.get_value(), 42.0);
}

#[test]
fn test_number_input_set_value_with_clamping() {
    let mut input = NumberInput::new().min(0.0).max(100.0);
    input.set_value(150.0);
    assert_eq!(input.get_value(), 100.0);

    input.set_value(-50.0);
    assert_eq!(input.get_value(), 0.0);
}

#[test]
fn test_number_input_set_value_exits_editing_mode() {
    let mut input = NumberInput::new();
    input.start_editing();
    assert!(input.is_editing());
    input.set_value(42.0);
    assert!(!input.is_editing());
    assert_eq!(input.display_string(), "42");
}

// =========================================================================
// Increment/Decrement Tests
// =========================================================================

#[test]
fn test_number_input_increment() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    input.increment();
    assert_eq!(input.get_value(), 15.0);
}

#[test]
fn test_number_input_decrement() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    input.decrement();
    assert_eq!(input.get_value(), 5.0);
}

#[test]
fn test_number_input_increment_with_max() {
    let mut input = NumberInput::new().value(95.0).max(100.0).step(10.0);
    input.increment();
    assert_eq!(input.get_value(), 100.0); // Clamped to max
}

#[test]
fn test_number_input_decrement_with_min() {
    let mut input = NumberInput::new().value(5.0).min(0.0).step(10.0);
    input.decrement();
    assert_eq!(input.get_value(), 0.0); // Clamped to min
}

#[test]
fn test_number_input_increment_large() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    input.increment_large();
    assert_eq!(input.get_value(), 60.0); // +10 * 5
}

#[test]
fn test_number_input_decrement_large() {
    let mut input = NumberInput::new().value(60.0).step(5.0);
    input.decrement_large();
    assert_eq!(input.get_value(), 10.0); // -10 * 5
}

#[test]
fn test_number_input_set_to_min() {
    let mut input = NumberInput::new().value(50.0).min(10.0);
    input.set_to_min();
    assert_eq!(input.get_value(), 10.0);
}

#[test]
fn test_number_input_set_to_min_without_min() {
    let mut input = NumberInput::new().value(50.0);
    input.set_to_min();
    assert_eq!(input.get_value(), 50.0); // No change when no min
}

#[test]
fn test_number_input_set_to_max() {
    let mut input = NumberInput::new().value(50.0).max(100.0);
    input.set_to_max();
    assert_eq!(input.get_value(), 100.0);
}

#[test]
fn test_number_input_set_to_max_without_max() {
    let mut input = NumberInput::new().value(50.0);
    input.set_to_max();
    assert_eq!(input.get_value(), 50.0); // No change when no max
}

// =========================================================================
// Editing Mode Tests
// =========================================================================

#[test]
fn test_number_input_start_editing() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    assert!(input.is_editing());
    // The buffer starts as the formatted value, cursor at its end.
    assert_eq!(input.display_string(), "42");
    assert_eq!(at_cursor(&input), "429");
}

#[test]
fn test_number_input_start_editing_already_editing() {
    let mut input = editing(42.0, "100");
    input.handle_key(&Key::Left);

    input.start_editing(); // Should not reset
    assert!(input.is_editing());
    assert_eq!(input.display_string(), "100");
    assert_eq!(at_cursor(&input), "1090");
}

#[test]
fn test_number_input_commit_edit_valid() {
    let mut input = editing(42.0, "100");
    input.commit_edit();
    assert!(!input.is_editing());
    assert_eq!(input.get_value(), 100.0);
    assert_eq!(input.display_string(), "100");
}

#[test]
fn test_number_input_commit_edit_invalid() {
    // "-" and "." are accepted keys but not a number on their own.
    let mut input = editing(42.0, "-.");
    input.commit_edit();
    assert!(!input.is_editing());
    assert_eq!(input.get_value(), 42.0); // Value unchanged
    assert_eq!(input.display_string(), "42");
}

#[test]
fn test_number_input_commit_edit_with_clamping() {
    let mut input = NumberInput::new().value(50.0).min(0.0).max(100.0);
    type_keys(&mut input, "150");
    input.commit_edit();
    assert_eq!(input.get_value(), 100.0); // Clamped to max
}

#[test]
fn test_number_input_cancel_edit() {
    let mut input = editing(42.0, "100");
    input.cancel_edit();
    assert!(!input.is_editing());
    assert_eq!(input.get_value(), 42.0); // Original value preserved
    assert_eq!(input.display_string(), "42");
}

#[test]
fn test_number_input_is_editing() {
    let mut input = NumberInput::new();
    assert!(!input.is_editing());

    input.start_editing();
    assert!(input.is_editing());

    input.cancel_edit();
    assert!(!input.is_editing());
}

// =========================================================================
// Display String Tests
// =========================================================================

#[test]
fn test_number_input_display_string_basic() {
    let input = NumberInput::new().value(42.0);
    assert_eq!(input.display_string(), "42");
}

#[test]
fn test_number_input_display_string_with_precision() {
    let input = NumberInput::new().value(42.5678).precision(2);
    assert_eq!(input.display_string(), "42.57");
}

#[test]
fn test_number_input_display_string_with_prefix() {
    let input = NumberInput::new().value(42.0).prefix("$");
    assert_eq!(input.display_string(), "$42");
}

#[test]
fn test_number_input_display_string_with_suffix() {
    let input = NumberInput::new().value(42.0).suffix("%");
    assert_eq!(input.display_string(), "42%");
}

#[test]
fn test_number_input_display_string_with_prefix_and_suffix() {
    let input = NumberInput::new().value(42.0).prefix("$").suffix(" USD");
    assert_eq!(input.display_string(), "$42 USD");
}

#[test]
fn test_number_input_display_string_while_editing() {
    let input = editing(42.0, "50");
    assert_eq!(input.display_string(), "50");
    assert_eq!(input.get_value(), 42.0);
}

// =========================================================================
// Rendering Tests
// =========================================================================

#[test]
fn test_number_input_render_prefix_value_suffix() {
    let input = NumberInput::new()
        .value(7.0)
        .prefix("$")
        .suffix("%")
        .show_buttons(false);
    assert_eq!(render_row(&input, 10).1.trim_end(), "$7%");
}

#[test]
fn test_number_input_render_cursor_when_editing_and_focused() {
    let mut input = NumberInput::new().focused(true).show_buttons(false);
    type_keys(&mut input, "12");
    input.handle_key(&Key::Left);
    let (buffer, text) = render_row(&input, 10);
    assert_eq!(text.trim_end(), "12");
    // The cursor sits on the '2', drawn black on white.
    let cursor = buffer.get(1, 0).unwrap();
    assert_eq!(cursor.symbol, '2');
    assert_eq!(cursor.fg, Some(Color::BLACK));
    assert_eq!(cursor.bg, Some(Color::WHITE));
    assert_ne!(buffer.get(0, 0).unwrap().bg, Some(Color::WHITE));
}

#[test]
fn test_number_input_render_cursor_at_end() {
    let mut input = NumberInput::new().focused(true).show_buttons(false);
    type_keys(&mut input, "12");
    let (buffer, _) = render_row(&input, 10);
    assert_eq!(buffer.get(2, 0).unwrap().bg, Some(Color::WHITE));
}

// =========================================================================
// Key Handling Tests
// =========================================================================

#[test]
fn test_number_input_handle_key_up() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    let handled = input.handle_key(&Key::Up);
    assert!(handled);
    assert_eq!(input.get_value(), 15.0);
}

#[test]
fn test_number_input_handle_key_char_k() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    let handled = input.handle_key(&Key::Char('k'));
    assert!(handled);
    assert_eq!(input.get_value(), 15.0);
}

#[test]
fn test_number_input_handle_key_down() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    let handled = input.handle_key(&Key::Down);
    assert!(handled);
    assert_eq!(input.get_value(), 5.0);
}

#[test]
fn test_number_input_handle_key_char_j() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    let handled = input.handle_key(&Key::Char('j'));
    assert!(handled);
    assert_eq!(input.get_value(), 5.0);
}

#[test]
fn test_number_input_handle_key_up_commits_edit_first() {
    let mut input = editing(10.0, "20");
    assert!(input.handle_key(&Key::Up));
    assert!(!input.is_editing());
    assert_eq!(input.get_value(), 21.0);
}

#[test]
fn test_number_input_handle_key_page_up() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    let handled = input.handle_key(&Key::PageUp);
    assert!(handled);
    assert_eq!(input.get_value(), 60.0); // +10 * 5
}

#[test]
fn test_number_input_handle_key_page_down() {
    let mut input = NumberInput::new().value(60.0).step(5.0);
    let handled = input.handle_key(&Key::PageDown);
    assert!(handled);
    assert_eq!(input.get_value(), 10.0); // -10 * 5
}

#[test]
fn test_number_input_handle_key_home() {
    let mut input = NumberInput::new().value(50.0).min(10.0);
    let handled = input.handle_key(&Key::Home);
    assert!(handled);
    assert_eq!(input.get_value(), 10.0);
}

#[test]
fn test_number_input_handle_key_home_without_min() {
    let mut input = NumberInput::new().value(50.0);
    let handled = input.handle_key(&Key::Home);
    assert!(handled);
    assert_eq!(input.get_value(), 50.0); // No change
}

#[test]
fn test_number_input_handle_key_end() {
    let mut input = NumberInput::new().value(50.0).max(100.0);
    let handled = input.handle_key(&Key::End);
    assert!(handled);
    assert_eq!(input.get_value(), 100.0);
}

#[test]
fn test_number_input_handle_key_end_without_max() {
    let mut input = NumberInput::new().value(50.0);
    let handled = input.handle_key(&Key::End);
    assert!(handled);
    assert_eq!(input.get_value(), 50.0); // No change
}

#[test]
fn test_number_input_handle_key_enter_starts_editing() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::Enter);
    assert!(handled);
    assert!(input.is_editing());
    assert_eq!(input.display_string(), "42");
}

#[test]
fn test_number_input_handle_key_enter_commits_edit() {
    let mut input = editing(42.0, "100");
    let handled = input.handle_key(&Key::Enter);
    assert!(handled);
    assert!(!input.is_editing());
    assert_eq!(input.get_value(), 100.0);
}

#[test]
fn test_number_input_handle_key_escape_cancels_edit() {
    let mut input = editing(42.0, "100");
    let handled = input.handle_key(&Key::Escape);
    assert!(handled);
    assert!(!input.is_editing());
    assert_eq!(input.get_value(), 42.0); // Original value
}

#[test]
fn test_number_input_handle_key_escape_not_editing() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::Escape);
    assert!(!handled);
}

#[test]
fn test_number_input_handle_key_digit_starts_editing() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::Char('5'));
    assert!(handled);
    assert!(input.is_editing());
    // The first digit replaces the value rather than appending to it.
    assert_eq!(input.display_string(), "5");
}

#[test]
fn test_number_input_handle_key_digit_while_editing() {
    let mut input = NumberInput::new().value(0.0);
    input.handle_key(&Key::Char('5'));
    assert_eq!(input.display_string(), "5");
    input.handle_key(&Key::Char('0'));
    assert_eq!(input.display_string(), "50");
}

#[test]
fn test_number_input_handle_key_decimal_point() {
    let mut input = NumberInput::new().value(0.0);
    type_keys(&mut input, "5.5");
    assert_eq!(input.display_string(), "5.5");
    input.commit_edit();
    assert_eq!(input.get_value(), 5.5);
}

#[test]
fn test_number_input_handle_key_decimal_point_only_once() {
    let mut input = editing(42.0, "5.5");
    assert!(input.handle_key(&Key::Char('.')));
    assert_eq!(input.display_string(), "5.5"); // No second decimal
}

#[test]
fn test_number_input_handle_key_minus_at_start() {
    let mut input = NumberInput::new().value(42.0);
    input.handle_key(&Key::Char('-'));
    assert_eq!(input.display_string(), "-");
    type_keys(&mut input, "7");
    input.commit_edit();
    assert_eq!(input.get_value(), -7.0);
}

#[test]
fn test_number_input_handle_key_minus_not_at_start() {
    let mut input = editing(42.0, "10");
    input.handle_key(&Key::Char('-'));
    assert_eq!(input.display_string(), "10"); // No minus added
}

#[test]
fn test_number_input_handle_key_minus_only_once() {
    let mut input = editing(42.0, "-10");
    input.handle_key(&Key::Left);
    input.handle_key(&Key::Left);
    input.handle_key(&Key::Left);
    input.handle_key(&Key::Char('-'));
    assert_eq!(input.display_string(), "-10"); // No second minus
}

#[test]
fn test_number_input_handle_key_other_char_not_editing() {
    let mut input = NumberInput::new().value(42.0);
    assert!(!input.handle_key(&Key::Char('a')));
    assert!(!input.is_editing());
}

#[test]
fn test_number_input_handle_key_backspace_while_editing() {
    let mut input = editing(42.0, "123");
    assert!(input.handle_key(&Key::Backspace));
    assert_eq!(input.display_string(), "12");
    assert_eq!(at_cursor(&input), "129");
}

#[test]
fn test_number_input_handle_key_backspace_mid_buffer() {
    let mut input = editing(42.0, "123");
    input.handle_key(&Key::Left);
    input.handle_key(&Key::Backspace);
    assert_eq!(input.display_string(), "13");
    assert_eq!(at_cursor(&input), "193");
}

#[test]
fn test_number_input_handle_key_backspace_at_start() {
    let mut input = editing(42.0, "12");
    input.handle_key(&Key::Left);
    input.handle_key(&Key::Left);
    assert!(input.handle_key(&Key::Backspace));
    assert_eq!(input.display_string(), "12");
}

#[test]
fn test_number_input_handle_key_backspace_not_editing() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::Backspace);
    assert!(!handled);
}

#[test]
fn test_number_input_handle_key_delete_while_editing() {
    let mut input = editing(42.0, "123");
    input.handle_key(&Key::Left);
    input.handle_key(&Key::Left);
    assert!(input.handle_key(&Key::Delete));
    assert_eq!(input.display_string(), "13");
}

#[test]
fn test_number_input_handle_key_delete_at_end() {
    let mut input = editing(42.0, "123");
    assert!(input.handle_key(&Key::Delete));
    assert_eq!(input.display_string(), "123");
}

#[test]
fn test_number_input_handle_key_delete_not_editing() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::Delete);
    assert!(!handled);
}

#[test]
fn test_number_input_handle_key_left_while_editing() {
    let mut input = editing(42.0, "123");
    assert!(input.handle_key(&Key::Left));
    assert_eq!(at_cursor(&input), "1293");
}

#[test]
fn test_number_input_handle_key_left_not_editing() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::Left);
    assert!(!handled);
}

#[test]
fn test_number_input_handle_key_left_at_start() {
    let mut input = editing(42.0, "123");
    for _ in 0..3 {
        assert!(input.handle_key(&Key::Left));
    }
    let handled = input.handle_key(&Key::Left);
    assert!(!handled);
    assert_eq!(at_cursor(&input), "9123");
}

#[test]
fn test_number_input_handle_key_right_while_editing() {
    let mut input = editing(42.0, "123");
    input.handle_key(&Key::Left);
    input.handle_key(&Key::Left);
    assert!(input.handle_key(&Key::Right));
    assert_eq!(at_cursor(&input), "1293");
}

#[test]
fn test_number_input_handle_key_right_not_editing() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::Right);
    assert!(!handled);
}

#[test]
fn test_number_input_handle_key_right_at_end() {
    let mut input = editing(42.0, "123");
    let handled = input.handle_key(&Key::Right);
    assert!(!handled);
    assert_eq!(at_cursor(&input), "1239");
}

#[test]
fn test_number_input_handle_key_insert_mid_buffer() {
    let mut input = editing(42.0, "13");
    input.handle_key(&Key::Left);
    input.handle_key(&Key::Char('2'));
    assert_eq!(input.display_string(), "123");
    assert_eq!(at_cursor(&input), "1293");
}

#[test]
fn test_number_input_handle_key_f1() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::F(1));
    assert!(!handled);
}

#[test]
fn test_number_input_disabled_ignores_keys() {
    let mut input = NumberInput::new().value(42.0).disabled(true);
    assert!(!input.handle_key(&Key::Up));
    assert!(!input.handle_key(&Key::Char('5')));
    assert_eq!(input.get_value(), 42.0);
    assert!(!input.is_editing());
}

// =========================================================================
// Clone Tests
// =========================================================================

#[test]
fn test_number_input_clone() {
    let input = NumberInput::new()
        .value(42.0)
        .min(0.0)
        .max(100.0)
        .step(5.0)
        .precision(2)
        .prefix("$")
        .suffix("%");

    let mut cloned = input.clone();
    assert_eq!(cloned.get_value(), 42.0);
    assert_eq!(cloned.display_string(), "$42.00%");
    cloned.increment();
    assert_eq!(cloned.get_value(), 47.0);
    cloned.set_to_min();
    assert_eq!(cloned.get_value(), 0.0);
    cloned.set_to_max();
    assert_eq!(cloned.get_value(), 100.0);
    // The original is untouched.
    assert_eq!(input.get_value(), 42.0);
}

// =========================================================================
// Helper Function Tests
// =========================================================================

#[test]
fn test_number_input_helper_basic() {
    let input = number_input().value(42.0);
    assert_eq!(input.get_value(), 42.0);
    assert_eq!(input.display_string(), "42");
}

#[test]
fn test_integer_input_helper() {
    let mut input = integer_input().value(10.0).min(0.0);
    assert_eq!(input.get_value(), 10.0);
    assert_eq!(input.display_string(), "10");
    input.increment();
    assert_eq!(input.get_value(), 11.0);
    input.set_to_min();
    assert_eq!(input.get_value(), 0.0);
}

#[test]
fn test_currency_input_helper() {
    let mut input = currency_input("$").value(19.99);
    assert_eq!(input.get_value(), 19.99);
    assert_eq!(input.display_string(), "$19.99");
    input.increment();
    assert_eq!(input.display_string(), "$20.00");
    // Currency cannot go negative.
    input.set_value(-5.0);
    assert_eq!(input.get_value(), 0.0);
}

#[test]
fn test_currency_input_symbol() {
    assert_eq!(currency_input("€").value(15.5).display_string(), "€15.50");
    assert_eq!(currency_input("£").display_string(), "£0.00");
}

#[test]
fn test_percentage_input_helper() {
    let mut input = percentage_input().value(75.0);
    assert_eq!(input.get_value(), 75.0);
    assert_eq!(input.display_string(), "75%");
    input.increment();
    assert_eq!(input.get_value(), 76.0);
    input.set_value(150.0);
    assert_eq!(input.get_value(), 100.0);
    input.set_value(-1.0);
    assert_eq!(input.get_value(), 0.0);
}
