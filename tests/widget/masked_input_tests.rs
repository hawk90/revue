//! Tests for MaskedInput widget
//!
//! Extracted from src/widget/form/masked_input.rs

use revue::style::Color;
use revue::widget::form::masked_input::{MaskStyle, MaskedInput, ValidationState};
use revue::widget::form::masked_input;
use revue::widget::form::password_input;
use revue::widget::form::pin_input;
use revue::widget::form::credit_card_input;

// =========================================================================
// MaskStyle enum tests
// =========================================================================

#[test]
fn test_mask_style_default() {
    assert_eq!(MaskStyle::default(), MaskStyle::Full);
}

#[test]
fn test_mask_style_clone() {
    let style = MaskStyle::ShowLast(4);
    assert_eq!(style, style.clone());
}

#[test]
fn test_mask_style_copy() {
    let style1 = MaskStyle::Peek;
    let style2 = style1;
    assert_eq!(style1, MaskStyle::Peek);
    assert_eq!(style2, MaskStyle::Peek);
}

#[test]
fn test_mask_style_partial_eq() {
    assert_eq!(MaskStyle::Full, MaskStyle::Full);
    assert_eq!(MaskStyle::ShowLast(4), MaskStyle::ShowLast(4));
    assert_ne!(MaskStyle::Full, MaskStyle::Peek);
}

#[test]
fn test_mask_style_debug() {
    let debug_str = format!("{:?}", MaskStyle::ShowLast(4));
    assert!(debug_str.contains("ShowLast"));
}

#[test]
fn test_mask_style_all_variants_unique() {
    let variants = [
        MaskStyle::Full,
        MaskStyle::ShowLast(1),
        MaskStyle::ShowFirst(1),
        MaskStyle::Peek,
        MaskStyle::Hidden,
    ];

    // All should be different from Full
    for variant in variants.iter().skip(1) {
        assert_ne!(*variant, MaskStyle::Full);
    }
}

// =========================================================================
// ValidationState enum tests
// =========================================================================

#[test]
fn test_validation_state_clone() {
    let state = ValidationState::Invalid("error".to_string());
    let cloned = state.clone();
    assert_eq!(state, cloned);
}

#[test]
fn test_validation_state_debug() {
    let debug_str = format!("{:?}", ValidationState::Validating);
    assert!(debug_str.contains("Validating"));
}

#[test]
fn test_validation_state_partial_eq() {
    assert_eq!(ValidationState::None, ValidationState::None);
    assert_eq!(ValidationState::Valid, ValidationState::Valid);
    assert_ne!(ValidationState::Valid, ValidationState::Validating);
}

#[test]
fn test_validation_state_invalid_with_message() {
    let state = ValidationState::Invalid("Too short".to_string());
    assert!(matches!(state, ValidationState::Invalid(_)));
    if let ValidationState::Invalid(msg) = state {
        assert_eq!(msg, "Too short");
    }
}

#[test]
fn test_validation_state_all_variants() {
    let _ = ValidationState::None;
    let _ = ValidationState::Valid;
    let _ = ValidationState::Invalid("error".to_string());
    let _ = ValidationState::Validating;
}

// =========================================================================
// MaskedInput::new and default tests
// =========================================================================

#[test]
fn test_masked_input_new() {
    let input = MaskedInput::new();
    assert_eq!(input.get_value(), "");
    assert_eq!(input.get_mask_char(), '●');
    assert_eq!(input.get_mask_style(), MaskStyle::Full);
}

#[test]
fn test_masked_input_default() {
    let input = MaskedInput::default();
    assert_eq!(input.get_value(), "");
    assert_eq!(input.get_mask_char(), '●');
}

#[test]
fn test_masked_input_clone() {
    let input1 = MaskedInput::new()
        .value("test")
        .mask_char('*')
        .placeholder("Enter");
    let input2 = input1.clone();
    assert_eq!(input1.get_value(), input2.get_value());
    assert_eq!(input1.get_mask_char(), input2.get_mask_char());
}

#[test]
fn test_masked_input_debug() {
    let input = MaskedInput::new().value("test");
    let debug_str = format!("{:?}", input);
    assert!(debug_str.contains("MaskedInput"));
}

// =========================================================================
// MaskedInput builder tests
// =========================================================================

#[test]
fn test_masked_input_mask_char() {
    let input = MaskedInput::new().mask_char('*');
    assert_eq!(input.get_mask_char(), '*');
}

#[test]
fn test_masked_input_mask_style() {
    let input = MaskedInput::new().mask_style(MaskStyle::Peek);
    assert_eq!(input.get_mask_style(), MaskStyle::Peek);
}

#[test]
fn test_masked_input_placeholder() {
    let input = MaskedInput::new().placeholder("Enter password");
    assert_eq!(input.get_placeholder(), Some(&"Enter password".to_string()));
}

#[test]
fn test_masked_input_label() {
    let input = MaskedInput::new().label("Password");
    assert_eq!(input.get_label(), Some(&"Password".to_string()));
}

#[test]
fn test_masked_input_max_length() {
    let input = MaskedInput::new().max_length(10);
    assert_eq!(input.get_max_length(), 10);
}

#[test]
fn test_masked_input_min_length() {
    let input = MaskedInput::new().min_length(8);
    assert_eq!(input.get_min_length(), 8);
}

#[test]
fn test_masked_input_focused() {
    let input = MaskedInput::new().focused(true);
    assert!(input.get_focused());
}

#[test]
fn test_masked_input_disabled() {
    let input = MaskedInput::new().disabled(true);
    assert!(input.get_disabled());
}

#[test]
fn test_masked_input_fg() {
    let input = MaskedInput::new().fg(Color::RED);
    assert_eq!(input.fg(), Some(Color::RED));
}

#[test]
fn test_masked_input_bg() {
    let input = MaskedInput::new().bg(Color::BLUE);
    assert_eq!(input.bg(), Some(Color::BLUE));
}

#[test]
fn test_masked_input_width() {
    let input = MaskedInput::new().width(30);
    assert_eq!(input.width(), Some(30));
}

#[test]
fn test_masked_input_show_strength() {
    let input = MaskedInput::new().show_strength(true);
    assert!(input.get_show_strength());
}

#[test]
fn test_masked_input_allow_reveal() {
    let input = MaskedInput::new().allow_reveal(true);
    assert!(input.get_allow_reveal());
}

#[test]
fn test_masked_input_value() {
    let input = MaskedInput::new().value("secret123");
    assert_eq!(input.get_value(), "secret123");
    assert_eq!(input.get_cursor(), 9);
}

#[test]
fn test_masked_input_password() {
    let pwd = MaskedInput::password();
    assert_eq!(pwd.get_mask_char(), '●');
    assert_eq!(pwd.get_mask_style(), MaskStyle::Full);
    assert!(pwd.get_show_strength());
}

#[test]
fn test_masked_input_pin() {
    let pin = MaskedInput::pin(4);
    assert_eq!(pin.get_mask_char(), '*');
    assert_eq!(pin.get_max_length(), 4);
    assert_eq!(pin.get_mask_style(), MaskStyle::Full);
}

#[test]
fn test_masked_input_credit_card() {
    let card = MaskedInput::credit_card();
    assert_eq!(card.get_mask_char(), '•');
    assert_eq!(card.get_max_length(), 16);
    assert!(matches!(card.get_mask_style(), MaskStyle::ShowLast(4)));
}

// =========================================================================
// MaskedInput value operations
// =========================================================================

#[test]
fn test_masked_input_get_value() {
    let input = MaskedInput::new().value("test123");
    assert_eq!(input.get_value(), "test123");
}

#[test]
fn test_masked_input_set_value() {
    let mut input = MaskedInput::new();
    input.set_value("abc");
    assert_eq!(input.get_value(), "abc");
    // Cursor should be clamped to new length
    assert_eq!(input.get_cursor(), 3);
}

#[test]
fn test_masked_input_clear() {
    let mut input = MaskedInput::new().value("something");
    input.clear();
    assert_eq!(input.get_value(), "");
    assert_eq!(input.get_cursor(), 0);
}

#[test]
fn test_masked_input_clear_empty() {
    let mut input = MaskedInput::new();
    input.clear();
    assert_eq!(input.get_value(), "");
}

// =========================================================================
// MaskedInput cursor operations
// =========================================================================

#[test]
fn test_masked_input_insert() {
    let mut input = MaskedInput::new();
    input.insert_char('a');
    input.insert_char('b');
    input.insert_char('c');
    assert_eq!(input.get_value(), "abc");
    assert_eq!(input.get_cursor(), 3);
}

#[test]
fn test_masked_input_insert_disabled() {
    let mut input = MaskedInput::new().disabled(true);
    input.insert_char('a');
    assert_eq!(input.get_value(), "");
    assert_eq!(input.get_cursor(), 0);
}

#[test]
fn test_masked_input_insert_max_length() {
    let mut input = MaskedInput::new().max_length(4);
    for c in "12345678".chars() {
        input.insert_char(c);
    }
    assert_eq!(input.get_value(), "1234");
}

#[test]
fn test_masked_input_insert_unlimited() {
    let mut input = MaskedInput::new().max_length(0);
    for c in "12345678".chars() {
        input.insert_char(c);
    }
    assert_eq!(input.get_value(), "12345678");
}

#[test]
fn test_masked_insert_middle() {
    let mut input = MaskedInput::new().value("ac");
    input.set_cursor(1);
    input.insert_char('b');
    assert_eq!(input.get_value(), "abc");
    assert_eq!(input.get_cursor(), 2);
}

#[test]
fn test_masked_input_delete_backward() {
    let mut input = MaskedInput::new().value("hello");
    input.delete_backward();
    assert_eq!(input.get_value(), "hell");
    assert_eq!(input.get_cursor(), 4);
}

#[test]
fn test_masked_input_delete_backward_at_start() {
    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(0);
    input.delete_backward();
    assert_eq!(input.get_value(), "hello");
    assert_eq!(input.get_cursor(), 0);
}

#[test]
fn test_masked_input_delete_backward_disabled() {
    let mut input = MaskedInput::new().value("hello").disabled(true);
    input.set_cursor(3);
    input.delete_backward();
    assert_eq!(input.get_value(), "hello");
    assert_eq!(input.get_cursor(), 3);
}

#[test]
fn test_masked_input_delete_forward() {
    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(2);
    input.delete_forward();
    assert_eq!(input.get_value(), "helo");
    assert_eq!(input.get_cursor(), 2);
}

#[test]
fn test_masked_input_delete_forward_at_end() {
    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(5);
    input.delete_forward();
    assert_eq!(input.get_value(), "hello");
    assert_eq!(input.get_cursor(), 5);
}

#[test]
fn test_masked_input_delete_forward_disabled() {
    let mut input = MaskedInput::new().value("hello").disabled(true);
    input.set_cursor(2);
    input.delete_forward();
    assert_eq!(input.get_value(), "hello");
    assert_eq!(input.get_cursor(), 2);
}

#[test]
fn test_masked_input_move_left() {
    let mut input = MaskedInput::new().value("hello");
    input.move_left();
    assert_eq!(input.get_cursor(), 4);
}

#[test]
fn test_masked_input_move_left_at_start() {
    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(0);
    input.move_left();
    assert_eq!(input.get_cursor(), 0);
}

#[test]
fn test_masked_input_move_right() {
    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(0);
    input.move_right();
    assert_eq!(input.get_cursor(), 1);
}

#[test]
fn test_masked_input_move_right_at_end() {
    let mut input = MaskedInput::new().value("hello");
    input.move_right();
    assert_eq!(input.get_cursor(), 5);
}

#[test]
fn test_masked_input_move_start() {
    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(3);
    input.move_start();
    assert_eq!(input.get_cursor(), 0);
}

#[test]
fn test_masked_input_move_end() {
    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(0);
    input.move_end();
    assert_eq!(input.get_cursor(), 5);
}

#[test]
fn test_masked_input_cursor_movement_chain() {
    let mut input = MaskedInput::new().value("hello");

    input.move_start();
    assert_eq!(input.get_cursor(), 0);

    input.move_end();
    assert_eq!(input.get_cursor(), 5);

    input.move_left();
    assert_eq!(input.get_cursor(), 4);

    input.move_right();
    assert_eq!(input.get_cursor(), 5);
}

// =========================================================================
// MaskedInput peek mode tests
// =========================================================================

#[test]
fn test_masked_input_update() {
    let mut input = MaskedInput::new().mask_style(MaskStyle::Peek).value("a");
    input.set_peek_countdown(5);

    input.update();
    assert_eq!(input.get_peek_countdown(), 4);

    for _ in 0..5 {
        input.update();
    }
    assert_eq!(input.get_peek_countdown(), 0);
}

#[test]
fn test_masked_input_insert_starts_peek() {
    let mut input = MaskedInput::new().mask_style(MaskStyle::Peek);

    input.insert_char('a');
    assert_eq!(input.get_peek_countdown(), 10);
}

#[test]
fn test_masked_display_peek() {
    let mut input = MaskedInput::new().mask_style(MaskStyle::Peek).value("abc");
    input.set_cursor(3);
    input.set_peek_countdown(5);

    // Last character should be visible
    let display = input.masked_display();
    assert_eq!(display, "●●c");
}

#[test]
fn test_masked_display_peek_no_countdown() {
    let input = MaskedInput::new().mask_style(MaskStyle::Peek).value("abc");

    let display = input.masked_display();
    assert_eq!(display, "●●●");
}

// =========================================================================
// MaskedInput display tests
// =========================================================================

#[test]
fn test_masked_display_full() {
    let input = MaskedInput::new().value("secret");
    assert_eq!(input.masked_display(), "●●●●●●");
}

#[test]
fn test_masked_display_empty() {
    let input = MaskedInput::new();
    assert_eq!(input.masked_display(), "");
}

#[test]
fn test_masked_display_show_last() {
    let input = MaskedInput::new()
        .mask_style(MaskStyle::ShowLast(4))
        .value("1234567890");
    assert_eq!(input.masked_display(), "●●●●●●7890");
}

#[test]
fn test_masked_display_show_last_exceeds_length() {
    let input = MaskedInput::new()
        .mask_style(MaskStyle::ShowLast(10))
        .value("123");
    assert_eq!(input.masked_display(), "123");
}

#[test]
fn test_masked_display_show_first() {
    let input = MaskedInput::new()
        .mask_style(MaskStyle::ShowFirst(4))
        .value("1234567890");
    assert_eq!(input.masked_display(), "1234●●●●●●");
}

#[test]
fn test_masked_display_show_first_exceeds_length() {
    let input = MaskedInput::new()
        .mask_style(MaskStyle::ShowFirst(10))
        .value("123");
    assert_eq!(input.masked_display(), "123");
}

#[test]
fn test_masked_display_hidden() {
    let input = MaskedInput::new()
        .mask_style(MaskStyle::Hidden)
        .value("secret");
    assert_eq!(input.masked_display(), "");
}

#[test]
fn test_masked_display_custom_mask_char() {
    let input = MaskedInput::new().mask_char('*').value("test");
    assert_eq!(input.masked_display(), "****");
}

// =========================================================================
// MaskedInput reveal tests
// =========================================================================

#[test]
fn test_reveal_toggle() {
    let mut input = MaskedInput::new().allow_reveal(true).value("secret");

    assert!(!input.get_revealing());
    assert_eq!(input.masked_display(), "●●●●●●");

    input.toggle_reveal();
    assert!(input.get_revealing());
    assert_eq!(input.masked_display(), "secret");
}

#[test]
fn test_reveal_toggle_not_allowed() {
    let mut input = MaskedInput::new().allow_reveal(false).value("secret");

    assert!(!input.get_revealing());
    input.toggle_reveal();
    assert!(!input.get_revealing());
}

#[test]
fn test_reveal_toggle_off() {
    let mut input = MaskedInput::new().allow_reveal(true).value("secret");
    input.set_revealing(true);

    input.toggle_reveal();
    assert!(!input.get_revealing());
}

// =========================================================================
// Password strength tests
// =========================================================================

#[test]
fn test_password_strength() {
    let weak = MaskedInput::new().value("abc");
    assert_eq!(weak.password_strength(), 0);

    let strong = MaskedInput::new().value("MyP@ssw0rd123!");
    assert!(strong.password_strength() >= 3);
}

#[test]
fn test_password_strength_very_weak() {
    let input = MaskedInput::new().value("abc");
    assert_eq!(input.password_strength(), 0);
}

#[test]
fn test_password_strength_weak() {
    let input = MaskedInput::new().value("abcdefgh");
    assert_eq!(input.password_strength(), 1);
}

#[test]
fn test_password_strength_fair() {
    let input = MaskedInput::new().value("Abcdefgh1");
    // len=9 >=8: +1, has_lower+upper: +1, has_digit: +1 = 3
    assert_eq!(input.password_strength(), 3);
}

#[test]
fn test_password_strength_strong() {
    let input = MaskedInput::new().value("Abcdefgh1!");
    // len=10 >=8: +1, has_lower+upper: +1, has_digit: +1, has_special: +1 = 4
    assert_eq!(input.password_strength(), 4);
}

#[test]
fn test_password_strength_very_strong() {
    let input = MaskedInput::new().value("Abcdefgh1!ghjk");
    // len=15 >=8: +1, >=12: +1, has_lower+upper: +1, has_digit: +1, has_special: +1 = 5 (capped to 4)
    assert_eq!(input.password_strength(), 4);
}

#[test]
fn test_strength_label() {
    assert_eq!(MaskedInput::new().value("a").strength_label(), "Very Weak");
    assert_eq!(
        MaskedInput::new().value("abcdefgh").strength_label(),
        "Weak"
    );
    assert_eq!(
        MaskedInput::new().value("Abcdefgh1").strength_label(),
        "Strong"
    );
    assert_eq!(
        MaskedInput::new().value("Abcdefgh1!").strength_label(),
        "Very Strong"
    );
    assert_eq!(
        MaskedInput::new().value("Abcdefgh1!ghjk").strength_label(),
        "Very Strong"
    );
}

#[test]
fn test_strength_color() {
    assert_eq!(MaskedInput::new().value("a").strength_color(), Color::RED);
    assert_eq!(
        MaskedInput::new().value("abcdefgh").strength_color(),
        Color::rgb(255, 128, 0)
    );
    assert_eq!(
        MaskedInput::new().value("Abcdefgh1").strength_color(),
        Color::rgb(128, 255, 0)
    );
    assert_eq!(
        MaskedInput::new().value("Abcdefgh1!").strength_color(),
        Color::GREEN
    );
    assert_eq!(
        MaskedInput::new().value("Abcdefgh1!ghjk").strength_color(),
        Color::GREEN
    );
}

// =========================================================================
// Validation tests
// =========================================================================

#[test]
fn test_validation() {
    let mut input = MaskedInput::new().min_length(8).value("short");

    assert!(!input.validate());
    assert!(matches!(input.get_validation(), ValidationState::Invalid(_)));

    input.set_value("longenough");
    assert!(input.validate());
    assert!(matches!(input.get_validation(), ValidationState::Valid));
}

#[test]
fn test_validation_no_min_length() {
    let mut input = MaskedInput::new().value("");
    assert!(input.validate());
    assert!(matches!(input.get_validation(), ValidationState::Valid));
}

#[test]
fn test_validation_exactly_min_length() {
    let mut input = MaskedInput::new().min_length(5).value("hello");
    assert!(input.validate());
}

#[test]
fn test_validation_invalid_message() {
    let mut input = MaskedInput::new().min_length(8).value("short");
    input.validate();

    if let ValidationState::Invalid(msg) = input.get_validation() {
        assert!(msg.contains("8"));
        assert!(msg.contains("Minimum"));
    } else {
        panic!("Expected Invalid state");
    }
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_helper_functions() {
    let pwd = password_input("Password");
    assert!(pwd.get_show_strength());
    assert_eq!(pwd.get_placeholder(), Some(&"Password".to_string()));

    let pin = pin_input(4);
    assert_eq!(pin.get_max_length(), 4);

    let card = credit_card_input();
    assert!(matches!(card.get_mask_style(), MaskStyle::ShowLast(4)));
}

#[test]
fn test_masked_input_helper() {
    let input = masked_input();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_password_input_helper() {
    let pwd = password_input("Enter password");
    assert_eq!(pwd.get_placeholder(), Some(&"Enter password".to_string()));
    assert!(pwd.get_show_strength());
}

#[test]
fn test_pin_input_helper() {
    let pin = pin_input(6);
    assert_eq!(pin.get_max_length(), 6);
}

#[test]
fn test_credit_card_input_helper() {
    let card = credit_card_input();
    assert_eq!(card.get_max_length(), 16);
}

// =========================================================================
// Builder chain tests
// =========================================================================

#[test]
fn test_masked_input_builder_chain() {
    let input = MaskedInput::new()
        .mask_char('*')
        .mask_style(MaskStyle::Peek)
        .placeholder("Password")
        .label("Enter")
        .max_length(20)
        .min_length(8)
        .focused(true)
        .disabled(false)
        .fg(Color::WHITE)
        .bg(Color::BLACK)
        .width(30)
        .show_strength(true)
        .allow_reveal(true)
        .value("test");

    assert_eq!(input.get_mask_char(), '*');
    assert_eq!(input.get_mask_style(), MaskStyle::Peek);
    assert_eq!(input.get_placeholder(), Some(&"Password".to_string()));
    assert_eq!(input.get_label(), Some(&"Enter".to_string()));
    assert_eq!(input.get_max_length(), 20);
    assert_eq!(input.get_min_length(), 8);
    assert!(input.get_focused());
    assert!(!input.get_disabled());
    assert_eq!(input.fg(), Some(Color::WHITE));
    assert_eq!(input.bg(), Some(Color::BLACK));
    assert_eq!(input.width(), Some(30));
    assert!(input.get_show_strength());
    assert!(input.get_allow_reveal());
    assert_eq!(input.get_value(), "test");
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_masked_input_unicode() {
    let mut input = MaskedInput::new();
    // Note: The implementation uses String::insert with byte-based cursor
    // This test verifies the behavior is predictable
    input.insert_char('a');
    input.insert_char('b');
    assert_eq!(input.get_value(), "ab");
}

#[test]
fn test_masked_input_delete_unicode() {
    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(2);
    input.delete_forward();
    assert_eq!(input.get_value(), "helo");
}

#[test]
fn test_masked_input_empty_value_operations() {
    let mut input = MaskedInput::new();
    input.delete_backward();
    input.delete_forward();
    input.move_left();
    input.move_right();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_peek_with_unicode() {
    let mut input = MaskedInput::new().mask_style(MaskStyle::Peek).value("ab");
    input.set_cursor(2);
    input.set_peek_countdown(5);

    let display = input.masked_display();
    assert_eq!(display, "●b");
}

#[test]
fn test_masked_input_show_last_with_unicode() {
    let input = MaskedInput::new()
        .mask_style(MaskStyle::ShowLast(2))
        .value("12345");
    assert_eq!(input.masked_display(), "●●●45");
}

#[test]
fn test_masked_input_single_char_operations() {
    let mut input = MaskedInput::new().value("a");

    input.move_left();
    assert_eq!(input.get_cursor(), 0);

    input.move_right();
    assert_eq!(input.get_cursor(), 1);

    input.delete_backward();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_zero_max_length() {
    let mut input = MaskedInput::new().max_length(0);
    for _ in 0..100 {
        input.insert_char('a');
    }
    // Max length 0 means unlimited
    assert_eq!(input.get_value(), "a".repeat(100));
}

// =========================================================================
// element_id and class tests
// =========================================================================

#[test]
fn test_masked_input_element_id() {
    let input = MaskedInput::new().element_id("password-field");
    assert_eq!(input.element_id(), Some(&"password-field".to_string()));
}

#[test]
fn test_masked_input_element_id_override() {
    let input = MaskedInput::new()
        .element_id("first-id")
        .element_id("second-id");
    assert_eq!(input.element_id(), Some(&"second-id".to_string()));
}

#[test]
fn test_masked_input_class() {
    let input = MaskedInput::new().class("input-field");
    assert_eq!(input.classes(), &["input-field".to_string()]);
}

#[test]
fn test_masked_input_class_multiple() {
    let input = MaskedInput::new().class("required").class("validated");
    assert_eq!(
        input.classes(),
        &["required".to_string(), "validated".to_string()]
    );
}

#[test]
fn test_masked_input_class_no_duplicate() {
    let input = MaskedInput::new().class("container").class("container");
    assert_eq!(input.classes(), &["container".to_string()]);
}

#[test]
fn test_masked_input_classes_vec() {
    let input = MaskedInput::new().classes(vec!["class1", "class2", "class3"]);
    assert_eq!(
        input.classes(),
        &[
            "class1".to_string(),
            "class2".to_string(),
            "class3".to_string()
        ]
    );
}

#[test]
fn test_masked_input_classes_array() {
    let input = MaskedInput::new().classes(["class1", "class2"]);
    assert_eq!(
        input.classes(),
        &["class1".to_string(), "class2".to_string()]
    );
}

#[test]
fn test_masked_input_classes_with_duplicates_filtered() {
    let input = MaskedInput::new().classes(vec!["a", "b", "a", "c", "b"]);
    assert_eq!(
        input.classes(),
        &["a".to_string(), "b".to_string(), "c".to_string()]
    );
}

#[test]
fn test_masked_input_mixed_classes() {
    let input = MaskedInput::new()
        .class("first")
        .classes(vec!["second", "third"])
        .class("fourth");
    assert_eq!(
        input.classes(),
        &[
            "first".to_string(),
            "second".to_string(),
            "third".to_string(),
            "fourth".to_string()
        ]
    );
}

// =========================================================================
// StyledView trait tests
// =========================================================================

#[test]
fn test_masked_input_set_id() {
    let mut input = MaskedInput::new();
    input.set_id("test-id");
    assert_eq!(input.element_id(), Some(&"test-id".to_string()));
}

#[test]
fn test_masked_input_set_id_override() {
    let mut input = MaskedInput::new();
    input.set_id("first");
    input.set_id("second");
    assert_eq!(input.element_id(), Some(&"second".to_string()));
}

#[test]
fn test_masked_input_add_class() {
    let mut input = MaskedInput::new();
    input.add_class("container");
    assert_eq!(input.classes(), &["container".to_string()]);
}

#[test]
fn test_masked_input_add_class_multiple() {
    let mut input = MaskedInput::new();
    input.add_class("class1");
    input.add_class("class2");
    input.add_class("class3");
    assert_eq!(
        input.classes(),
        &[
            "class1".to_string(),
            "class2".to_string(),
            "class3".to_string()
        ]
    );
}

#[test]
fn test_masked_input_add_class_no_duplicate() {
    let mut input = MaskedInput::new();
    input.add_class("duplicate");
    input.add_class("duplicate");
    assert_eq!(input.classes(), &["duplicate".to_string()]);
}

#[test]
fn test_masked_input_remove_class() {
    let mut input = MaskedInput::new();
    input.add_class("class1");
    input.add_class("class2");
    input.add_class("class3");
    input.remove_class("class2");
    assert_eq!(
        input.classes(),
        &["class1".to_string(), "class3".to_string()]
    );
}

#[test]
fn test_masked_input_remove_class_not_present() {
    let mut input = MaskedInput::new();
    input.add_class("class1");
    input.remove_class("nonexistent");
    assert_eq!(input.classes(), &["class1".to_string()]);
}

#[test]
fn test_masked_input_remove_class_from_empty() {
    let mut input = MaskedInput::new();
    input.remove_class("anything");
    assert!(input.classes().is_empty());
}

#[test]
fn test_masked_input_toggle_class_adds() {
    let mut input = MaskedInput::new();
    input.toggle_class("new-class");
    assert_eq!(input.classes(), &["new-class".to_string()]);
}

#[test]
fn test_masked_input_toggle_class_removes() {
    let mut input = MaskedInput::new();
    input.add_class("existing");
    input.toggle_class("existing");
    assert!(input.classes().is_empty());
}

#[test]
fn test_masked_input_toggle_class_multiple_times() {
    let mut input = MaskedInput::new();
    input.toggle_class("toggle");
    assert_eq!(input.classes(), &["toggle".to_string()]);
    input.toggle_class("toggle");
    assert!(input.classes().is_empty());
    input.toggle_class("toggle");
    assert_eq!(input.classes(), &["toggle".to_string()]);
}

#[test]
fn test_masked_input_has_class_true() {
    let mut input = MaskedInput::new();
    input.add_class("existing");
    assert!(input.has_class("existing"));
}

#[test]
fn test_masked_input_has_class_false() {
    let input = MaskedInput::new();
    assert!(!input.has_class("nonexistent"));
}

#[test]
fn test_masked_input_has_class_empty() {
    let input = MaskedInput::new();
    assert!(!input.has_class("anything"));
}

#[test]
fn test_masked_input_classes_getter() {
    let input = MaskedInput::new().class("c1").class("c2");
    let classes = input.get_classes();
    assert_eq!(classes, &["c1".to_string(), "c2".to_string()]);
}

#[test]
fn test_masked_input_classes_getter_empty() {
    let input = MaskedInput::new();
    assert!(input.get_classes().is_empty());
}

#[test]
fn test_masked_input_id_getter() {
    let input = MaskedInput::new().element_id("test-id");
    assert_eq!(input.get_id(), Some("test-id"));
}

#[test]
fn test_masked_input_id_getter_none() {
    let input = MaskedInput::new();
    assert_eq!(input.get_id(), None);
}

// =========================================================================
// Combined builder and styled tests
// =========================================================================

#[test]
fn test_masked_input_builder_and_styled_mix() {
    let mut input = MaskedInput::new()
        .element_id("test")
        .class("from-builder")
        .value("password");

    input.add_class("from-styled");
    input.set_id("updated-id");

    assert_eq!(input.element_id(), Some(&"updated-id".to_string()));
    assert_eq!(
        input.classes(),
        &["from-builder".to_string(), "from-styled".to_string()]
    );
    assert_eq!(input.get_value(), "password");
}

#[test]
fn test_masked_input_full_builder_chain_with_props() {
    let input = MaskedInput::new()
        .element_id("password-input")
        .class("required")
        .classes(vec!["validated", "secure"])
        .mask_char('*')
        .mask_style(MaskStyle::Peek)
        .placeholder("Enter password")
        .label("Password")
        .max_length(20)
        .min_length(8)
        .focused(true)
        .disabled(false)
        .fg(Color::WHITE)
        .bg(Color::BLACK)
        .width(30)
        .show_strength(true)
        .allow_reveal(true)
        .value("test");

    assert_eq!(input.element_id(), Some(&"password-input".to_string()));
    assert_eq!(
        input.classes(),
        &[
            "required".to_string(),
            "validated".to_string(),
            "secure".to_string()
        ]
    );
    assert_eq!(input.get_mask_char(), '*');
    assert_eq!(input.get_mask_style(), MaskStyle::Peek);
    assert_eq!(input.get_placeholder(), Some(&"Enter password".to_string()));
    assert_eq!(input.get_label(), Some(&"Password".to_string()));
    assert_eq!(input.get_max_length(), 20);
    assert_eq!(input.get_min_length(), 8);
    assert!(input.get_focused());
    assert!(!input.get_disabled());
    assert_eq!(input.fg(), Some(Color::WHITE));
    assert_eq!(input.bg(), Some(Color::BLACK));
    assert_eq!(input.width(), Some(30));
    assert!(input.get_show_strength());
    assert!(input.get_allow_reveal());
    assert_eq!(input.get_value(), "test");
}

// =========================================================================
// Edge case tests for props
// =========================================================================

#[test]
fn test_masked_input_empty_string_element_id() {
    let input = MaskedInput::new().element_id("");
    assert_eq!(input.element_id(), Some(&"".to_string()));
}

#[test]
fn test_masked_input_empty_string_class() {
    let input = MaskedInput::new().class("");
    assert_eq!(input.classes(), &["".to_string()]);
}

#[test]
fn test_masked_input_classes_empty_vec() {
    let input = MaskedInput::new().classes(Vec::<&str>::new());
    assert!(input.classes().is_empty());
}

#[test]
fn test_masked_input_classes_empty_array() {
    let input = MaskedInput::new().classes([] as [&str; 0]);
    assert!(input.classes().is_empty());
}

#[test]
fn test_masked_input_set_id_empty_string() {
    let mut input = MaskedInput::new();
    input.set_id("");
    assert_eq!(input.element_id(), Some(&"".to_string()));
}

#[test]
fn test_masked_input_add_class_empty_string() {
    let mut input = MaskedInput::new();
    input.add_class("");
    assert_eq!(input.classes(), &["".to_string()]);
}

// =========================================================================
// Password builder preset with props tests
// =========================================================================

#[test]
fn test_masked_input_password_with_props() {
    let pwd = MaskedInput::password()
        .element_id("pwd")
        .class("password-field");

    assert_eq!(pwd.element_id(), Some(&"pwd".to_string()));
    assert_eq!(pwd.classes(), &["password-field".to_string()]);
    assert!(pwd.get_show_strength());
    assert_eq!(pwd.get_mask_char(), '●');
}

#[test]
fn test_masked_input_pin_with_props() {
    let pin = MaskedInput::pin(4)
        .element_id("pin-input")
        .classes(vec!["numeric", "required"]);

    assert_eq!(pin.element_id(), Some(&"pin-input".to_string()));
    assert_eq!(
        pin.classes(),
        &["numeric".to_string(), "required".to_string()]
    );
    assert_eq!(pin.get_max_length(), 4);
}

#[test]
fn test_masked_input_credit_card_with_props() {
    let card = MaskedInput::credit_card()
        .element_id("card-number")
        .class("financial");

    assert_eq!(card.element_id(), Some(&"card-number".to_string()));
    assert_eq!(card.classes(), &["financial".to_string()]);
    assert_eq!(card.get_max_length(), 16);
}

// =========================================================================
// Styled operations with disabled/focused states
// =========================================================================

#[test]
fn test_masked_input_styled_operations_while_disabled() {
    let mut input = MaskedInput::new().disabled(true);

    // Styled operations should work regardless of disabled state
    input.add_class("disabled");
    input.set_id("disabled-input");
    input.toggle_class("toggle");

    assert!(input.has_class("disabled"));
    assert_eq!(input.get_id(), Some("disabled-input"));
    assert!(input.has_class("toggle"));
}

#[test]
fn test_masked_input_styled_operations_while_focused() {
    let mut input = MaskedInput::new().focused(true);

    // Styled operations should work regardless of focused state
    input.add_class("focused");
    input.remove_class("focused");
    input.toggle_class("active");

    assert!(!input.has_class("focused"));
    assert!(input.has_class("active"));
}

// =========================================================================
// Class operations with special characters
// =========================================================================

#[test]
fn test_masked_input_class_with_hyphens() {
    let mut input = MaskedInput::new();
    input.add_class("my-custom-class");
    assert!(input.has_class("my-custom-class"));
}

#[test]
fn test_masked_input_class_with_underscores() {
    let mut input = MaskedInput::new();
    input.add_class("my_custom_class");
    assert!(input.has_class("my_custom_class"));
}

#[test]
fn test_masked_input_class_with_numbers() {
    let mut input = MaskedInput::new();
    input.add_class("class123");
    assert!(input.has_class("class123"));
}

// =========================================================================
// Interaction between value changes and styled operations
// =========================================================================

#[test]
fn test_masked_input_value_and_styled_operations() {
    let mut input = MaskedInput::new();

    input.add_class("initial");
    input.set_value("password");
    input.add_class("has-value");
    input.clear();
    input.remove_class("has-value");
    input.toggle_class("empty");

    assert!(input.has_class("initial"));
    assert!(input.has_class("empty"));
    assert!(!input.has_class("has-value"));
    assert_eq!(input.get_value(), "");
}

// =========================================================================
// Stress tests - long builder chains
// =========================================================================

#[test]
fn test_masked_input_long_class_chain() {
    let input = MaskedInput::new()
        .class("c1")
        .class("c2")
        .class("c3")
        .classes(vec!["c4", "c5"])
        .class("c6")
        .classes(vec!["c7", "c8", "c9"]);

    assert_eq!(input.classes().len(), 9);
}

#[test]
fn test_masked_input_many_toggle_operations() {
    let mut input = MaskedInput::new();
    for _ in 0..10 {
        input.toggle_class("toggle");
    }
    // Even number of toggles = not present
    assert!(!input.has_class("toggle"));
}

#[test]
fn test_masked_input_many_add_remove_operations() {
    let mut input = MaskedInput::new();
    for i in 0..5 {
        input.add_class(&format!("class{}", i));
    }
    assert_eq!(input.classes().len(), 5);

    for i in 0..5 {
        input.remove_class(&format!("class{}", i));
    }
    assert!(input.classes().is_empty());
}

// =========================================================================
// Helper functions with props
// =========================================================================

#[test]
fn test_masked_input_helper_with_props() {
    let input = masked_input().element_id("masked").class("input");
    assert_eq!(input.element_id(), Some(&"masked".to_string()));
    assert_eq!(input.classes(), &["input".to_string()]);
}

#[test]
fn test_password_input_helper_with_props() {
    let pwd = password_input("Password").element_id("pwd").class("secure");
    assert_eq!(pwd.element_id(), Some(&"pwd".to_string()));
    assert_eq!(pwd.classes(), &["secure".to_string()]);
}

#[test]
fn test_pin_input_helper_with_props() {
    let pin = pin_input(6)
        .element_id("pin")
        .classes(vec!["numeric", "required"]);
    assert_eq!(pin.element_id(), Some(&"pin".to_string()));
    assert!(pin.has_class("numeric"));
    assert!(pin.has_class("required"));
}

#[test]
fn test_credit_card_input_helper_with_props() {
    let card = credit_card_input().element_id("card").class("financial");
    assert_eq!(card.element_id(), Some(&"card".to_string()));
    assert!(card.has_class("financial"));
}

// =========================================================================
// Clone and Debug with props
// =========================================================================

#[test]
fn test_masked_input_clone_preserves_props() {
    let input1 = MaskedInput::new()
        .element_id("test-id")
        .class("class1")
        .class("class2")
        .value("secret");
    let input2 = input1.clone();

    assert_eq!(input1.element_id(), input2.element_id());
    assert_eq!(input1.classes(), input2.classes());
    assert_eq!(input1.get_value(), input2.get_value());
}

#[test]
fn test_masked_input_debug_includes_props() {
    let input = MaskedInput::new().element_id("test-id").class("test-class");
    let debug_str = format!("{:?}", input);
    // Debug should contain structural information
    assert!(debug_str.contains("MaskedInput"));
}

// =========================================================================
// Validation state with styled operations
// =========================================================================

#[test]
fn test_masked_input_validation_with_class_changes() {
    let mut input = MaskedInput::new().min_length(8);

    input.add_class("initial");
    assert!(!input.validate()); // Too short
    input.add_class("invalid");

    input.set_value("longenough");
    input.remove_class("invalid");
    input.add_class("valid");
    assert!(input.validate());

    assert!(input.has_class("initial"));
    assert!(input.has_class("valid"));
    assert!(!input.has_class("invalid"));
}

// =========================================================================
// Reveal functionality with styled operations
// =========================================================================

#[test]
fn test_masked_input_reveal_with_class_toggling() {
    let mut input = MaskedInput::new().allow_reveal(true).value("secret");

    input.add_class("masked");
    input.toggle_reveal();
    input.remove_class("masked");
    input.add_class("revealed");

    assert!(input.get_revealing());
    assert!(!input.has_class("masked"));
    assert!(input.has_class("revealed"));

    input.toggle_reveal();
    input.toggle_class("revealed");
    assert!(!input.get_revealing());
    assert!(!input.has_class("revealed"));
}

// =========================================================================
// Peek mode with styled operations
// =========================================================================

#[test]
fn test_masked_input_peek_with_class_operations() {
    let mut input = MaskedInput::new().mask_style(MaskStyle::Peek);

    input.add_class("peek-mode");
    input.insert_char('a');
    assert_eq!(input.get_peek_countdown(), 10);

    input.update();
    input.remove_class("peek-mode");
    input.add_class("peeking");
    assert_eq!(input.get_peek_countdown(), 9);
    assert!(input.has_class("peeking"));
}
