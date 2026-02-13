use revue::widget::{number_input, NumberInput};
use revue::event::Key;

// =========================================================================
// Constructor Tests
// =========================================================================

#[test]
fn test_number_input_new_creates_default_widget() {
    let input = NumberInput::new();
    assert_eq!(input.value, 0.0);
    assert!(input.min.is_none());
    assert!(input.max.is_none());
    assert_eq!(input.step, 1.0);
    assert_eq!(input.precision, 0);
    assert!(input.prefix.is_none());
    assert!(input.suffix.is_none());
    assert!(!input.editing);
    assert_eq!(input.input_buffer, "");
    assert_eq!(input.cursor, 0);
    assert_eq!(input.width, 10);
    assert!(input.show_buttons);
}

#[test]
fn test_number_input_default_trait() {
    let input = NumberInput::default();
    assert_eq!(input.value, 0.0);
    assert_eq!(input.step, 1.0);
}

// =========================================================================
// Builder Method Tests
// =========================================================================

#[test]
fn test_number_input_value_builder() {
    let input = NumberInput::new().value(42.0);
    assert_eq!(input.value, 42.0);
}

#[test]
fn test_number_input_value_with_clamping() {
    let input = NumberInput::new().min(0.0).max(100.0).value(150.0);
    assert_eq!(input.value, 100.0); // Clamped to max

    let input = NumberInput::new().min(0.0).max(100.0).value(-50.0);
    assert_eq!(input.value, 0.0); // Clamped to min
}

#[test]
fn test_number_input_min_builder() {
    let input = NumberInput::new().min(10.0);
    assert_eq!(input.min, Some(10.0));
}

#[test]
fn test_number_input_min_clamps_current_value() {
    let input = NumberInput::new().value(5.0).min(10.0);
    assert_eq!(input.value, 10.0); // Clamped up to min
}

#[test]
fn test_number_input_max_builder() {
    let input = NumberInput::new().max(100.0);
    assert_eq!(input.max, Some(100.0));
}

#[test]
fn test_number_input_max_clamps_current_value() {
    let input = NumberInput::new().value(150.0).max(100.0);
    assert_eq!(input.value, 100.0); // Clamped down to max
}

#[test]
fn test_number_input_step_builder() {
    let input = NumberInput::new().step(5.0);
    assert_eq!(input.step, 5.0);

    let input = NumberInput::new().step(-5.0);
    assert_eq!(input.step, 5.0); // Absolute value applied
}

#[test]
fn test_number_input_precision_builder() {
    let input = NumberInput::new().precision(2);
    assert_eq!(input.precision, 2);
}

#[test]
fn test_number_input_prefix_builder() {
    let input = NumberInput::new().prefix("$");
    assert_eq!(input.prefix, Some("$".to_string()));
}

#[test]
fn test_number_input_suffix_builder() {
    let input = NumberInput::new().suffix("%");
    assert_eq!(input.suffix, Some("%".to_string()));
}

#[test]
fn test_number_input_width_builder() {
    let input = NumberInput::new().width(20);
    assert_eq!(input.width, 20);

    let input = NumberInput::new().width(3);
    assert_eq!(input.width, 5); // Minimum width enforced
}

#[test]
fn test_number_input_show_buttons_builder() {
    let input = NumberInput::new().show_buttons(true);
    assert!(input.show_buttons);

    let input = NumberInput::new().show_buttons(false);
    assert!(!input.show_buttons);
}

#[test]
fn test_number_input_builder_chaining() {
    let input = NumberInput::new()
        .value(50.0)
        .min(0.0)
        .max(100.0)
        .step(5.0)
        .precision(2)
        .prefix("$")
        .suffix(" USD")
        .width(15)
        .show_buttons(true);

    assert_eq!(input.value, 50.0);
    assert_eq!(input.min, Some(0.0));
    assert_eq!(input.max, Some(100.0));
    assert_eq!(input.step, 5.0);
    assert_eq!(input.precision, 2);
    assert_eq!(input.prefix, Some("$".to_string()));
    assert_eq!(input.suffix, Some(" USD".to_string()));
    assert_eq!(input.width, 15);
    assert!(input.show_buttons);
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
    assert_eq!(input.value, 42.0);
}

#[test]
fn test_number_input_set_value_with_clamping() {
    let mut input = NumberInput::new().min(0.0).max(100.0);
    input.set_value(150.0);
    assert_eq!(input.value, 100.0);

    input.set_value(-50.0);
    assert_eq!(input.value, 0.0);
}

#[test]
fn test_number_input_set_value_exits_editing_mode() {
    let mut input = NumberInput::new();
    input.editing = true;
    input.set_value(42.0);
    assert!(!input.editing);
}

// =========================================================================
// Increment/Decrement Tests
// =========================================================================

#[test]
fn test_number_input_increment() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    input.increment();
    assert_eq!(input.value, 15.0);
}

#[test]
fn test_number_input_decrement() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    input.decrement();
    assert_eq!(input.value, 5.0);
}

#[test]
fn test_number_input_increment_with_max() {
    let mut input = NumberInput::new().value(95.0).max(100.0).step(10.0);
    input.increment();
    assert_eq!(input.value, 100.0); // Clamped to max
}

#[test]
fn test_number_input_decrement_with_min() {
    let mut input = NumberInput::new().value(5.0).min(0.0).step(10.0);
    input.decrement();
    assert_eq!(input.value, 0.0); // Clamped to min
}

#[test]
fn test_number_input_increment_large() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    input.increment_large();
    assert_eq!(input.value, 60.0); // +10 * 5
}

#[test]
fn test_number_input_decrement_large() {
    let mut input = NumberInput::new().value(60.0).step(5.0);
    input.decrement_large();
    assert_eq!(input.value, 10.0); // -10 * 5
}

#[test]
fn test_number_input_set_to_min() {
    let mut input = NumberInput::new().value(50.0).min(10.0);
    input.set_to_min();
    assert_eq!(input.value, 10.0);
}

#[test]
fn test_number_input_set_to_min_without_min() {
    let mut input = NumberInput::new().value(50.0);
    input.set_to_min();
    assert_eq!(input.value, 50.0); // No change when no min
}

#[test]
fn test_number_input_set_to_max() {
    let mut input = NumberInput::new().value(50.0).max(100.0);
    input.set_to_max();
    assert_eq!(input.value, 100.0);
}

#[test]
fn test_number_input_set_to_max_without_max() {
    let mut input = NumberInput::new().value(50.0);
    input.set_to_max();
    assert_eq!(input.value, 50.0); // No change when no max
}

// =========================================================================
// Editing Mode Tests
// =========================================================================

#[test]
fn test_number_input_start_editing() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    assert!(input.editing);
    assert_eq!(input.input_buffer, "42");
    assert_eq!(input.cursor, 2);
}

#[test]
fn test_number_input_start_editing_already_editing() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "100".to_string();
    input.cursor = 3;

    input.start_editing(); // Should not reset
    assert!(input.editing);
    assert_eq!(input.input_buffer, "100");
    assert_eq!(input.cursor, 3);
}

#[test]
fn test_number_input_commit_edit_valid() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "100".to_string();
    input.commit_edit();
    assert!(!input.editing);
    assert_eq!(input.value, 100.0);
    assert_eq!(input.input_buffer, "");
}

#[test]
fn test_number_input_commit_edit_invalid() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "abc".to_string();
    input.commit_edit();
    assert!(!input.editing);
    assert_eq!(input.value, 42.0); // Value unchanged
    assert_eq!(input.input_buffer, "");
}

#[test]
fn test_number_input_commit_edit_with_clamping() {
    let mut input = NumberInput::new().value(50.0).min(0.0).max(100.0);
    input.start_editing();
    input.input_buffer = "150".to_string();
    input.commit_edit();
    assert_eq!(input.value, 100.0); // Clamped to max
}

#[test]
fn test_number_input_cancel_edit() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "100".to_string();
    input.cancel_edit();
    assert!(!input.editing);
    assert_eq!(input.value, 42.0); // Original value preserved
    assert_eq!(input.input_buffer, "");
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
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "50".to_string();
    assert_eq!(input.display_string(), "50");
}

// =========================================================================
// Key Handling Tests
// =========================================================================

#[test]
fn test_number_input_handle_key_up() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    let handled = input.handle_key(&Key::Up);
    assert!(handled);
    assert_eq!(input.value, 15.0);
}

#[test]
fn test_number_input_handle_key_char_k() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    let handled = input.handle_key(&Key::Char('k'));
    assert!(handled);
    assert_eq!(input.value, 15.0);
}

#[test]
fn test_number_input_handle_key_down() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    let handled = input.handle_key(&Key::Down);
    assert!(handled);
    assert_eq!(input.value, 5.0);
}

#[test]
fn test_number_input_handle_key_char_j() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    let handled = input.handle_key(&Key::Char('j'));
    assert!(handled);
    assert_eq!(input.value, 5.0);
}

#[test]
fn test_number_input_handle_key_page_up() {
    let mut input = NumberInput::new().value(10.0).step(5.0);
    let handled = input.handle_key(&Key::PageUp);
    assert!(handled);
    assert_eq!(input.value, 60.0); // +10 * 5
}

#[test]
fn test_number_input_handle_key_page_down() {
    let mut input = NumberInput::new().value(60.0).step(5.0);
    let handled = input.handle_key(&Key::PageDown);
    assert!(handled);
    assert_eq!(input.value, 10.0); // -10 * 5
}

#[test]
fn test_number_input_handle_key_home() {
    let mut input = NumberInput::new().value(50.0).min(10.0);
    let handled = input.handle_key(&Key::Home);
    assert!(handled);
    assert_eq!(input.value, 10.0);
}

#[test]
fn test_number_input_handle_key_home_without_min() {
    let mut input = NumberInput::new().value(50.0);
    let handled = input.handle_key(&Key::Home);
    assert!(handled);
    assert_eq!(input.value, 50.0); // No change
}

#[test]
fn test_number_input_handle_key_end() {
    let mut input = NumberInput::new().value(50.0).max(100.0);
    let handled = input.handle_key(&Key::End);
    assert!(handled);
    assert_eq!(input.value, 100.0);
}

#[test]
fn test_number_input_handle_key_end_without_max() {
    let mut input = NumberInput::new().value(50.0);
    let handled = input.handle_key(&Key::End);
    assert!(handled);
    assert_eq!(input.value, 50.0); // No change
}

#[test]
fn test_number_input_handle_key_enter_starts_editing() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::Enter);
    assert!(handled);
    assert!(input.editing);
}

#[test]
fn test_number_input_handle_key_enter_commits_edit() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "100".to_string();
    let handled = input.handle_key(&Key::Enter);
    assert!(handled);
    assert!(!input.editing);
    assert_eq!(input.value, 100.0);
}

#[test]
fn test_number_input_handle_key_escape_cancels_edit() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "100".to_string();
    let handled = input.handle_key(&Key::Escape);
    assert!(handled);
    assert!(!input.editing);
    assert_eq!(input.value, 42.0); // Original value
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
    assert!(input.editing);
    assert_eq!(input.input_buffer, "5");
}

#[test]
fn test_number_input_handle_key_digit_while_editing() {
    let mut input = NumberInput::new().value(0.0);
    input.start_editing();
    input.input_buffer.clear(); // Clear for clean test
    input.handle_key(&Key::Char('5'));
    assert_eq!(input.input_buffer, "5");
    input.handle_key(&Key::Char('0'));
    assert_eq!(input.input_buffer, "50");
}

#[test]
fn test_number_input_handle_key_decimal_point() {
    let mut input = NumberInput::new().value(0.0);
    input.start_editing();
    input.input_buffer.clear(); // Clear for clean test
    input.handle_key(&Key::Char('5'));
    input.handle_key(&Key::Char('.'));
    input.handle_key(&Key::Char('5'));
    assert_eq!(input.input_buffer, "5.5");
}

#[test]
fn test_number_input_handle_key_decimal_point_only_once() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "5.5".to_string();
    input.cursor = 3;
    input.handle_key(&Key::Char('.'));
    assert_eq!(input.input_buffer, "5.5"); // No second decimal
}

#[test]
fn test_number_input_handle_key_minus_at_start() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer.clear();
    input.cursor = 0; // Reset cursor to start
    input.handle_key(&Key::Char('-'));
    assert_eq!(input.input_buffer, "-");
}

#[test]
fn test_number_input_handle_key_minus_not_at_start() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "10".to_string();
    input.handle_key(&Key::Char('-'));
    assert_eq!(input.input_buffer, "10"); // No minus added
}

#[test]
fn test_number_input_handle_key_minus_only_once() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "-10".to_string();
    input.handle_key(&Key::Char('-'));
    assert_eq!(input.input_buffer, "-10"); // No second minus
}

#[test]
fn test_number_input_handle_key_backspace_while_editing() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "123".to_string();
    input.cursor = 3;
    input.handle_key(&Key::Backspace);
    assert_eq!(input.input_buffer, "12");
    assert_eq!(input.cursor, 2);
}

#[test]
fn test_number_input_handle_key_backspace_not_editing() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::Backspace);
    assert!(!handled);
}

#[test]
fn test_number_input_handle_key_delete_while_editing() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "123".to_string();
    input.cursor = 1;
    input.handle_key(&Key::Delete);
    assert_eq!(input.input_buffer, "13");
}

#[test]
fn test_number_input_handle_key_delete_not_editing() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::Delete);
    assert!(!handled);
}

#[test]
fn test_number_input_handle_key_left_while_editing() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "123".to_string();
    input.cursor = 3;
    input.handle_key(&Key::Left);
    assert_eq!(input.cursor, 2);
}

#[test]
fn test_number_input_handle_key_left_not_editing() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::Left);
    assert!(!handled);
}

#[test]
fn test_number_input_handle_key_left_at_start() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "123".to_string();
    input.cursor = 0;
    let handled = input.handle_key(&Key::Left);
    assert!(!handled);
    assert_eq!(input.cursor, 0);
}

#[test]
fn test_number_input_handle_key_right_while_editing() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "123".to_string();
    input.cursor = 1;
    input.handle_key(&Key::Right);
    assert_eq!(input.cursor, 2);
}

#[test]
fn test_number_input_handle_key_right_not_editing() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::Right);
    assert!(!handled);
}

#[test]
fn test_number_input_handle_key_right_at_end() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "123".to_string();
    input.cursor = 3;
    let handled = input.handle_key(&Key::Right);
    assert!(!handled);
    assert_eq!(input.cursor, 3);
}

#[test]
fn test_number_input_handle_key_unknown() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::Escape);
    assert!(!handled);
}

#[test]
fn test_number_input_handle_key_f1() {
    let mut input = NumberInput::new().value(42.0);
    let handled = input.handle_key(&Key::F(1));
    assert!(!handled);
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

    let cloned = input.clone();
    assert_eq!(cloned.value, 42.0);
    assert_eq!(cloned.min, Some(0.0));
    assert_eq!(cloned.max, Some(100.0));
    assert_eq!(cloned.step, 5.0);
    assert_eq!(cloned.precision, 2);
    assert_eq!(cloned.prefix, Some("$".to_string()));
    assert_eq!(cloned.suffix, Some("%".to_string()));
}

// =========================================================================
// UTF-8 Tests
// =========================================================================

#[test]
fn test_number_input_utf8_char_count() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "123".to_string();
    assert_eq!(input.buffer_char_count(), 3);
}

#[test]
fn test_number_input_insert_char_at_cursor() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "13".to_string();
    input.cursor = 1;
    input.insert_char_at_cursor('2');
    assert_eq!(input.input_buffer, "123");
    assert_eq!(input.cursor, 2);
}

#[test]
fn test_number_input_delete_char_before_cursor() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "123".to_string();
    input.cursor = 2;
    input.delete_char_before_cursor();
    assert_eq!(input.input_buffer, "13");
    assert_eq!(input.cursor, 1);
}

#[test]
fn test_number_input_delete_char_at_cursor() {
    let mut input = NumberInput::new().value(42.0);
    input.start_editing();
    input.input_buffer = "123".to_string();
    input.cursor = 1;
    input.delete_char_at_cursor();
    assert_eq!(input.input_buffer, "13");
}

// =========================================================================
// Helper Function Tests
// =========================================================================

#[test]
fn test_number_input_helper_basic() {
    let input = number_input().value(42.0);
    assert_eq!(input.value, 42.0);
}

#[test]
fn test_integer_input_helper() {
    let input = super::integer_input().value(10.0).min(0.0);
    assert_eq!(input.precision, 0);
    assert_eq!(input.step, 1.0);
    assert_eq!(input.value, 10.0);
    assert_eq!(input.min, Some(0.0));
}

#[test]
fn test_currency_input_helper() {
    let input = super::currency_input("$").value(19.99);
    assert_eq!(input.precision, 2);
    assert_eq!(input.step, 0.01);
    assert_eq!(input.prefix, Some("$".to_string()));
    assert_eq!(input.value, 19.99);
    assert_eq!(input.min, Some(0.0));
}

#[test]
fn test_percentage_input_helper() {
    let input = super::percentage_input().value(75.0);
    assert_eq!(input.precision, 0);
    assert_eq!(input.step, 1.0);
    assert_eq!(input.suffix, Some("%".to_string()));
    assert_eq!(input.value, 75.0);
    assert_eq!(input.min, Some(0.0));
    assert_eq!(input.max, Some(100.0));
}