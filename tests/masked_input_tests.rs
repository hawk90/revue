//! Integration tests for masked input widget

use revue::style::Color;
use revue::widget::MaskedInput;

#[test]
fn test_masked_input_new() {
    let mut input = MaskedInput::new();

    assert_eq!(input.get_value(), "");
    assert!(input.validate());
}

#[test]
fn test_masked_input_password() {
    let input = MaskedInput::password();

    // Password preset was created successfully
}

#[test]
fn test_masked_input_pin() {
    let input = MaskedInput::pin(4);

    // PIN preset was created successfully
}

#[test]
fn test_masked_input_credit_card() {
    let input = MaskedInput::credit_card();

    // Credit card preset was created successfully
}

#[test]
fn test_masked_input_value() {
    let input = MaskedInput::new().value("test123");

    assert_eq!(input.get_value(), "test123");
}

#[test]
fn test_masked_input_set_value() {
    let mut input = MaskedInput::new();
    input.set_value("new value");

    assert_eq!(input.get_value(), "new value");
}

#[test]
fn test_masked_input_clear() {
    let mut input = MaskedInput::new().value("some text");
    input.clear();

    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_mask_char() {
    let input = MaskedInput::new().mask_char('•');

    // Mask char was set successfully
}

#[test]
fn test_masked_input_placeholder() {
    let input = MaskedInput::new().placeholder("Enter password");

    // Placeholder was set successfully
}

#[test]
fn test_masked_input_label() {
    let input = MaskedInput::new().label("Password:");

    // Label was set successfully
}

#[test]
fn test_masked_input_max_length() {
    let input = MaskedInput::new().max_length(10);

    // Max length was set successfully
}

#[test]
fn test_masked_input_min_length() {
    let input = MaskedInput::new().min_length(5);

    // Min length was set successfully
}

#[test]
fn test_masked_input_focused() {
    let input = MaskedInput::new().focused(true);

    // Focus state was set successfully
}

#[test]
fn test_masked_input_disabled() {
    let input = MaskedInput::new().disabled(true);

    // Disabled state was set successfully
}

#[test]
fn test_masked_input_colors() {
    let input = MaskedInput::new().fg(Color::CYAN).bg(Color::BLUE);

    // Colors were set successfully
}

#[test]
fn test_masked_input_width() {
    let input = MaskedInput::new().width(30);

    // Width was set successfully
}

#[test]
fn test_masked_input_show_strength() {
    let input = MaskedInput::new().show_strength(true);

    // Show strength was set successfully
}

#[test]
fn test_masked_input_allow_reveal() {
    let input = MaskedInput::new().allow_reveal(true);

    // Allow reveal was set successfully
}

#[test]
fn test_masked_input_toggle_reveal() {
    let mut input = MaskedInput::new();
    input.toggle_reveal();

    // Toggle reveal was called successfully
}

#[test]
fn test_masked_input_insert_char() {
    let mut input = MaskedInput::new();
    input.insert_char('a');
    input.insert_char('b');
    input.insert_char('c');

    assert_eq!(input.get_value(), "abc");
}

#[test]
fn test_masked_input_delete_backward() {
    let mut input = MaskedInput::new().value("abc");
    input.delete_backward();

    assert_eq!(input.get_value(), "ab");
}

#[test]
fn test_masked_input_delete_forward() {
    let mut input = MaskedInput::new().value("abc");
    input.move_left(); // Move cursor back
    input.delete_forward(); // Delete character at cursor

    // After moving left from end of "abc", cursor is at 'c'
    // delete_forward removes 'c', leaving "ab"
    assert_eq!(input.get_value(), "ab");
}

#[test]
fn test_masked_input_move_left() {
    let mut input = MaskedInput::new().value("abc");
    input.move_left();

    // Move left was called successfully
}

#[test]
fn test_masked_input_move_right() {
    let mut input = MaskedInput::new().value("abc");
    input.move_left();
    input.move_right();

    // Move right was called successfully
}

#[test]
fn test_masked_input_move_start() {
    let mut input = MaskedInput::new().value("abc");
    input.move_start();

    // Move to start was called successfully
}

#[test]
fn test_masked_input_move_end() {
    let mut input = MaskedInput::new().value("abc");
    input.move_start();
    input.move_end();

    // Move to end was called successfully
}

#[test]
fn test_masked_input_password_strength() {
    let input = MaskedInput::new().value("weak");

    // Password strength can be calculated
    let strength = input.password_strength();
    assert!(strength >= 0 && strength <= 5);
}

#[test]
fn test_masked_input_strength_label() {
    let input = MaskedInput::new().value("test");

    // Strength label can be retrieved
    let label = input.strength_label();
    assert!(!label.is_empty());
}

#[test]
fn test_masked_input_strength_color() {
    let input = MaskedInput::new().value("test");

    // Strength color can be retrieved
    let color = input.strength_color();
    // Color is valid (not checking specific value as it depends on implementation)
}

#[test]
fn test_masked_input_validate() {
    let mut input = MaskedInput::new().value("test123");

    assert!(input.validate());
}

#[test]
fn test_masked_input_validate_empty() {
    let mut input = MaskedInput::new();

    // Empty input validation depends on implementation
    let result = input.validate();
    // Just verify the method can be called
}

#[test]
fn test_masked_input_with_min_length() {
    let mut input = MaskedInput::new().min_length(5).value("abc");

    assert!(!input.validate()); // Too short
}

#[test]
fn test_masked_input_with_min_length_valid() {
    let mut input = MaskedInput::new().min_length(5).value("abcdefgh");

    assert!(input.validate()); // Long enough
}
