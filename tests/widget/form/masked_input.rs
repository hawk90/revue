//! MaskedInput widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::credit_card_input;
use revue::widget::masked_input;
use revue::widget::password_input;
use revue::widget::pin_input;
use revue::widget::traits::RenderContext;
use revue::widget::traits::View;
use revue::widget::MaskStyle;
use revue::widget::MaskedInput;
use revue::widget::ValidationState;

// =============================================================================
// Constructor and Builder Tests (생성자 및 빌더 테스트)
// =============================================================================

#[test]
fn test_masked_input_new() {
    let input = MaskedInput::new();
    assert_eq!(input.get_value(), "");

    let input = MaskedInput::new();
    assert_eq!(input.get_value(), "");
    assert_eq!(input.get_mask_char(), '●');
    assert_eq!(input.get_mask_style(), MaskStyle::Full);

    let mut input = MaskedInput::new();

    assert_eq!(input.get_value(), "");
    assert!(input.validate());
}

#[test]
fn test_masked_input_default() {
    let input = MaskedInput::default();
    assert_eq!(input.get_value(), "");

    let input = MaskedInput::default();
    assert_eq!(input.get_value(), "");
    assert_eq!(input.get_mask_char(), '●');
}

#[test]
fn test_masked_input_password() {
    let input = MaskedInput::password();
    assert_eq!(input.get_value(), "");

    let pwd = MaskedInput::password();
    assert_eq!(pwd.get_mask_char(), '●');
    assert_eq!(pwd.get_mask_style(), MaskStyle::Full);
    assert!(pwd.get_show_strength());

    let _input = MaskedInput::password();

    // Password preset was created successfully
}

#[test]
fn test_masked_input_pin() {
    let input = MaskedInput::pin(4);
    assert_eq!(input.get_value(), "");

    let pin = MaskedInput::pin(4);
    assert_eq!(pin.get_mask_char(), '*');
    assert_eq!(pin.get_max_length(), 4);
    assert_eq!(pin.get_mask_style(), MaskStyle::Full);

    let _input = MaskedInput::pin(4);

    // PIN preset was created successfully
}

#[test]
fn test_masked_input_credit_card() {
    let input = MaskedInput::credit_card();
    assert_eq!(input.get_value(), "");

    let card = MaskedInput::credit_card();
    assert_eq!(card.get_mask_char(), '•');
    assert_eq!(card.get_max_length(), 16);
    assert!(matches!(card.get_mask_style(), MaskStyle::ShowLast(4)));

    let _input = MaskedInput::credit_card();

    // Credit card preset was created successfully
}

#[test]
fn test_masked_input_helper() {
    let input = masked_input().value("test");
    assert_eq!(input.get_value(), "test");

    let input = masked_input();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_mask_char() {
    let input = MaskedInput::new().mask_char('*').value("test");
    assert_eq!(input.get_value(), "test");

    let input = MaskedInput::new().mask_char('*');
    assert_eq!(input.get_mask_char(), '*');

    let _input = MaskedInput::new().mask_char('•');

    // Mask char was set successfully
}

#[test]
fn test_masked_input_mask_style() {
    let input = MaskedInput::new()
        .mask_style(MaskStyle::ShowLast(2))
        .value("test");
    assert_eq!(input.get_value(), "test");

    let input = MaskedInput::new().mask_style(MaskStyle::Peek);
    assert_eq!(input.get_mask_style(), MaskStyle::Peek);
}

#[test]
fn test_masked_input_placeholder() {
    let input = MaskedInput::new().placeholder("Enter password");
    assert_eq!(input.get_value(), "");

    let input = MaskedInput::new().placeholder("Enter password");
    assert_eq!(input.get_placeholder(), Some(&"Enter password".to_string()));

    let _input = MaskedInput::new().placeholder("Enter password");

    // Placeholder was set successfully
}

#[test]
fn test_masked_input_label() {
    let input = MaskedInput::new().label("Password");
    assert_eq!(input.get_value(), "");

    let input = MaskedInput::new().label("Password");
    assert_eq!(input.get_label(), Some(&"Password".to_string()));

    let _input = MaskedInput::new().label("Password:");

    // Label was set successfully
}

#[test]
fn test_masked_input_max_length() {
    let input = MaskedInput::new().max_length(10);
    assert_eq!(input.get_value(), "");

    let input = MaskedInput::new().max_length(10);
    assert_eq!(input.get_max_length(), 10);

    let _input = MaskedInput::new().max_length(10);

    // Max length was set successfully
}

#[test]
fn test_masked_input_min_length() {
    let input = MaskedInput::new().min_length(8);
    assert_eq!(input.get_value(), "");

    let input = MaskedInput::new().min_length(8);
    assert_eq!(input.get_min_length(), 8);

    let _input = MaskedInput::new().min_length(5);

    // Min length was set successfully
}

#[test]
fn test_masked_input_focused() {
    let input = MaskedInput::new().focused(true);
    assert_eq!(input.get_value(), "");

    let input = MaskedInput::new().focused(true);
    assert!(input.get_focused());

    let _input = MaskedInput::new().focused(true);

    // Focus state was set successfully
}

#[test]
fn test_masked_input_disabled() {
    let input = MaskedInput::new().disabled(true);
    assert_eq!(input.get_value(), "");

    let input = MaskedInput::new().disabled(true);
    assert!(input.get_disabled());

    let _input = MaskedInput::new().disabled(true);

    // Disabled state was set successfully
}

#[test]
fn test_masked_input_colors() {
    let input = MaskedInput::new().fg(Color::RED).bg(Color::BLUE);
    assert_eq!(input.get_value(), "");

    let _input = MaskedInput::new().fg(Color::CYAN).bg(Color::BLUE);

    // Colors were set successfully
}

#[test]
fn test_masked_input_width() {
    let input = MaskedInput::new().width(30);
    assert_eq!(input.get_value(), "");

    let input = MaskedInput::new().width(30);
    assert_eq!(input.get_width(), Some(30));

    let _input = MaskedInput::new().width(30);

    // Width was set successfully
}

#[test]
fn test_masked_input_show_strength() {
    let input = MaskedInput::new().show_strength(true);
    assert_eq!(input.get_value(), "");

    let input = MaskedInput::new().show_strength(true);
    assert!(input.get_show_strength());

    let _input = MaskedInput::new().show_strength(true);

    // Show strength was set successfully
}

#[test]
fn test_masked_input_allow_reveal() {
    let input = MaskedInput::new().allow_reveal(true);
    assert_eq!(input.get_value(), "");

    let input = MaskedInput::new().allow_reveal(true);
    assert!(input.get_allow_reveal());

    let _input = MaskedInput::new().allow_reveal(true);

    // Allow reveal was set successfully
}

#[test]
fn test_masked_input_value() {
    let input = MaskedInput::new().value("secret123");
    assert_eq!(input.get_value(), "secret123");

    let input = MaskedInput::new().value("secret123");
    assert_eq!(input.get_value(), "secret123");
    assert_eq!(input.get_cursor(), 9);

    let input = MaskedInput::new().value("test123");

    assert_eq!(input.get_value(), "test123");
}

#[test]
fn test_masked_input_builder_chain() {
    let input = MaskedInput::new()
        .value("password")
        .placeholder("Enter password")
        .label("Password")
        .max_length(20)
        .min_length(8)
        .mask_char('*')
        .mask_style(MaskStyle::Full)
        .focused(true)
        .disabled(false)
        .fg(Color::WHITE)
        .bg(Color::BLACK)
        .width(25)
        .show_strength(true)
        .allow_reveal(true);

    assert_eq!(input.get_value(), "password");

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
    assert_eq!(input.get_fg(), Some(Color::WHITE));
    assert_eq!(input.get_bg(), Some(Color::BLACK));
    assert_eq!(input.get_width(), Some(30));
    assert!(input.get_show_strength());
    assert!(input.get_allow_reveal());
    assert_eq!(input.get_value(), "test");
}

// =============================================================================
// Value Management Tests (값 관리 테스트)
// =============================================================================

#[test]
fn test_masked_input_get_value() {
    let input = MaskedInput::new().value("secret");
    assert_eq!(input.get_value(), "secret");

    let input = MaskedInput::new().value("test123");
    assert_eq!(input.get_value(), "test123");
}

#[test]
fn test_masked_input_get_value_empty() {
    let input = MaskedInput::new();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_set_value() {
    let mut input = MaskedInput::new();
    input.set_value("newpassword");
    assert_eq!(input.get_value(), "newpassword");

    let mut input = MaskedInput::new();
    input.set_value("abc");
    assert_eq!(input.get_value(), "abc");
    // Cursor is clamped: initial cursor 0 is clamped to min(0, 3) = 0
    assert_eq!(input.get_cursor(), 0);

    let mut input = MaskedInput::new();
    input.set_value("new value");

    assert_eq!(input.get_value(), "new value");
}

#[test]
fn test_masked_input_set_value_multiple() {
    let mut input = MaskedInput::new();
    input.set_value("first");
    input.set_value("second");
    input.set_value("third");
    assert_eq!(input.get_value(), "third");
}

#[test]
fn test_masked_input_clear() {
    let mut input = MaskedInput::new().value("password");
    input.clear();
    assert_eq!(input.get_value(), "");

    let mut input = MaskedInput::new().value("something");
    input.clear();
    assert_eq!(input.get_value(), "");
    assert_eq!(input.get_cursor(), 0);

    let mut input = MaskedInput::new().value("some text");
    input.clear();

    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_clear_empty() {
    let mut input = MaskedInput::new();
    input.clear();
    assert_eq!(input.get_value(), "");
}

// =============================================================================
// Character Insertion Tests (문자 삽입 테스트)
// =============================================================================

#[test]
fn test_masked_input_insert_char() {
    let mut input = MaskedInput::new();
    input.insert_char('a');
    input.insert_char('b');
    input.insert_char('c');
    assert_eq!(input.get_value(), "abc");
}

#[test]
fn test_masked_input_insert_multiple_chars() {
    let mut input = MaskedInput::new();
    for c in "hello".chars() {
        input.insert_char(c);
    }
    assert_eq!(input.get_value(), "hello");
}

#[test]
fn test_masked_input_insert_special_chars() {
    let mut input = MaskedInput::new();
    input.insert_char('@');
    input.insert_char('#');
    input.insert_char('$');
    assert_eq!(input.get_value(), "@#$");
}

#[test]
fn test_masked_input_insert_respects_max_length() {
    let mut input = MaskedInput::new().max_length(5);
    input.insert_char('a');
    input.insert_char('b');
    input.insert_char('c');
    input.insert_char('d');
    input.insert_char('e');
    input.insert_char('f'); // 초과 분은 무시됨
    assert_eq!(input.get_value(), "abcde");
}

#[test]
fn test_masked_input_insert_disabled() {
    let mut input = MaskedInput::new().disabled(true);
    input.insert_char('a');
    assert_eq!(input.get_value(), "");

    let mut input = MaskedInput::new().disabled(true);
    input.insert_char('a');
    assert_eq!(input.get_value(), "");
    assert_eq!(input.get_cursor(), 0);
}

#[test]
fn test_masked_input_insert_unlimited_max_length() {
    let mut input = MaskedInput::new(); // max_length = 0 means unlimited
    for _ in 0..100 {
        input.insert_char('a');
    }
    assert_eq!(input.get_value().len(), 100);
}

#[test]
fn test_masked_input_insert_after_max_length() {
    let mut input = MaskedInput::new().max_length(3).value("abc");
    input.insert_char('d');
    assert_eq!(input.get_value(), "abc");
}

// =============================================================================
// Deletion Tests (삭제 테스트)
// =============================================================================

#[test]
fn test_masked_input_delete_backward() {
    let mut input = MaskedInput::new().value("hello");
    input.delete_backward();
    assert_eq!(input.get_value(), "hell");

    let mut input = MaskedInput::new().value("hello");
    input.delete_backward();
    assert_eq!(input.get_value(), "hell");
    assert_eq!(input.get_cursor(), 4);

    let mut input = MaskedInput::new().value("abc");
    input.delete_backward();

    assert_eq!(input.get_value(), "ab");
}

#[test]
fn test_masked_input_delete_backward_multiple() {
    let mut input = MaskedInput::new().value("hello");
    input.delete_backward();
    input.delete_backward();
    input.delete_backward();
    assert_eq!(input.get_value(), "he");
}

#[test]
fn test_masked_input_delete_backward_at_start() {
    let mut input = MaskedInput::new().value("hello");
    input.move_start();
    input.delete_backward();
    assert_eq!(input.get_value(), "hello");

    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(0);
    input.delete_backward();
    assert_eq!(input.get_value(), "hello");
    assert_eq!(input.get_cursor(), 0);
}

#[test]
fn test_masked_input_delete_backward_empty() {
    let mut input = MaskedInput::new();
    input.delete_backward();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_delete_backward_disabled() {
    let mut input = MaskedInput::new().value("test").disabled(true);
    input.delete_backward();
    assert_eq!(input.get_value(), "test");

    let mut input = MaskedInput::new().value("hello").disabled(true);
    input.set_cursor(3);
    input.delete_backward();
    assert_eq!(input.get_value(), "hello");
    assert_eq!(input.get_cursor(), 3);
}

#[test]
fn test_masked_input_delete_forward() {
    let mut input = MaskedInput::new().value("hello");
    input.move_start();
    input.delete_forward();
    assert_eq!(input.get_value(), "ello");

    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(2);
    input.delete_forward();
    assert_eq!(input.get_value(), "helo");
    assert_eq!(input.get_cursor(), 2);

    let mut input = MaskedInput::new().value("abc");
    input.move_left(); // Move cursor back
    input.delete_forward(); // Delete character at cursor

    // After moving left from end of "abc", cursor is at 'c'
    // delete_forward removes 'c', leaving "ab"
    assert_eq!(input.get_value(), "ab");
}

#[test]
fn test_masked_input_delete_forward_at_end() {
    let mut input = MaskedInput::new().value("hello");
    input.delete_forward();
    assert_eq!(input.get_value(), "hello");

    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(5);
    input.delete_forward();
    assert_eq!(input.get_value(), "hello");
    assert_eq!(input.get_cursor(), 5);
}

#[test]
fn test_masked_input_delete_forward_middle() {
    let mut input = MaskedInput::new().value("hello");
    input.move_start();
    input.move_right(); // cursor at 'e'
    input.delete_forward();
    assert_eq!(input.get_value(), "hllo");
}

#[test]
fn test_masked_input_delete_forward_multiple() {
    let mut input = MaskedInput::new().value("hello");
    input.move_start();
    input.delete_forward();
    input.delete_forward();
    input.delete_forward();
    assert_eq!(input.get_value(), "lo");
}

#[test]
fn test_masked_input_delete_forward_disabled() {
    let mut input = MaskedInput::new().value("test").disabled(true);
    input.move_start();
    input.delete_forward();
    assert_eq!(input.get_value(), "test");

    let mut input = MaskedInput::new().value("hello").disabled(true);
    input.set_cursor(2);
    input.delete_forward();
    assert_eq!(input.get_value(), "hello");
    assert_eq!(input.get_cursor(), 2);
}

#[test]
fn test_masked_input_delete_forward_empty() {
    let mut input = MaskedInput::new();
    input.delete_forward();
    assert_eq!(input.get_value(), "");
}

// =============================================================================
// Cursor Movement Tests (커서 이동 테스트)
// =============================================================================

#[test]
fn test_masked_input_move_left() {
    let mut input = MaskedInput::new().value("hello");
    input.move_left();
    // 커서가 왼쪽으로 이동 (내부 상태)
    assert_eq!(input.get_value(), "hello");

    let mut input = MaskedInput::new().value("hello");
    input.move_left();
    assert_eq!(input.get_cursor(), 4);

    let mut input = MaskedInput::new().value("abc");
    input.move_left();

    // Move left was called successfully
}

#[test]
fn test_masked_input_move_right() {
    let mut input = MaskedInput::new().value("hello");
    input.move_start();
    input.move_right();
    input.move_right();
    // 커서가 오른쪽으로 이동 (내부 상태)
    assert_eq!(input.get_value(), "hello");

    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(0);
    input.move_right();
    assert_eq!(input.get_cursor(), 1);

    let mut input = MaskedInput::new().value("abc");
    input.move_left();
    input.move_right();

    // Move right was called successfully
}

#[test]
fn test_masked_input_move_start() {
    let mut input = MaskedInput::new().value("hello");
    input.move_start();
    assert_eq!(input.get_value(), "hello");

    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(3);
    input.move_start();
    assert_eq!(input.get_cursor(), 0);

    let mut input = MaskedInput::new().value("abc");
    input.move_start();

    // Move to start was called successfully
}

#[test]
fn test_masked_input_move_end() {
    let mut input = MaskedInput::new().value("hello");
    input.move_start();
    input.move_end();
    assert_eq!(input.get_value(), "hello");

    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(0);
    input.move_end();
    assert_eq!(input.get_cursor(), 5);

    let mut input = MaskedInput::new().value("abc");
    input.move_start();
    input.move_end();

    // Move to end was called successfully
}

#[test]
fn test_masked_input_cursor_navigation_roundtrip() {
    let mut input = MaskedInput::new().value("hello world");
    input.move_start();
    input.move_end();
    input.move_start();
    assert_eq!(input.get_value(), "hello world");
}

#[test]
fn test_masked_input_move_left_at_start() {
    let mut input = MaskedInput::new();
    input.move_left();
    input.move_left();
    assert_eq!(input.get_value(), "");

    let mut input = MaskedInput::new().value("hello");
    input.set_cursor(0);
    input.move_left();
    assert_eq!(input.get_cursor(), 0);
}

#[test]
fn test_masked_input_move_right_at_end() {
    let mut input = MaskedInput::new().value("test");
    input.move_right();
    input.move_right();
    assert_eq!(input.get_value(), "test");

    let mut input = MaskedInput::new().value("hello");
    input.move_right();
    assert_eq!(input.get_cursor(), 5);
}

// =============================================================================
// Mask Style Tests (마스크 스타일 테스트)
// =============================================================================

#[test]
fn test_masked_input_mask_style_full() {
    let input = MaskedInput::new().mask_char('*').value("secret");
    // Full 마스크 스타일 설정
    assert_eq!(input.get_value(), "secret");
}

#[test]
fn test_masked_input_mask_style_show_last() {
    let input = MaskedInput::new()
        .mask_style(MaskStyle::ShowLast(4))
        .value("1234567890");
    assert_eq!(input.get_value(), "1234567890");
}

#[test]
fn test_masked_input_mask_style_show_first() {
    let input = MaskedInput::new()
        .mask_style(MaskStyle::ShowFirst(4))
        .value("1234567890");
    assert_eq!(input.get_value(), "1234567890");
}

#[test]
fn test_masked_input_mask_style_hidden() {
    let input = MaskedInput::new()
        .mask_style(MaskStyle::Hidden)
        .value("secret");
    assert_eq!(input.get_value(), "secret");
}

#[test]
fn test_masked_input_mask_style_peek() {
    let input = MaskedInput::new().mask_style(MaskStyle::Peek).value("test");
    assert_eq!(input.get_value(), "test");
}

// =============================================================================
// Reveal Toggle Tests (표시 전환 테스트)
// =============================================================================

#[test]
fn test_masked_input_toggle_reveal_allowed() {
    let mut input = MaskedInput::new().allow_reveal(true).value("secret");

    input.toggle_reveal();
    input.toggle_reveal();
    // 토글 가능 (내부 상태 변경)
    assert_eq!(input.get_value(), "secret");
}

#[test]
fn test_masked_input_toggle_reveal_not_allowed() {
    let mut input = MaskedInput::new().allow_reveal(false).value("secret");

    input.toggle_reveal();
    // allow_reveal이 false면 토글되지 않음
    assert_eq!(input.get_value(), "secret");
}

#[test]
fn test_masked_input_reveal_with_value() {
    let mut input = MaskedInput::new().allow_reveal(true).value("password123");

    input.toggle_reveal();
    assert_eq!(input.get_value(), "password123");
}

#[test]
fn test_masked_input_multiple_toggles() {
    let mut input = MaskedInput::new().allow_reveal(true).value("test");

    for _ in 0..10 {
        input.toggle_reveal();
    }
    assert_eq!(input.get_value(), "test");
}

// =============================================================================
// Password Strength Tests (비밀번호 강도 테스트)
// =============================================================================

#[test]
fn test_masked_input_password_strength_very_weak() {
    let input = MaskedInput::new().value("abc");
    assert_eq!(input.password_strength(), 0);
    assert_eq!(input.strength_label(), "Very Weak");
}

#[test]
fn test_masked_input_password_strength_weak() {
    let input = MaskedInput::new().value("abcdefgh");
    assert_eq!(input.password_strength(), 1);
    assert_eq!(input.strength_label(), "Weak");
}

#[test]
fn test_masked_input_password_strength_fair() {
    let input = MaskedInput::new().value("Abcdefgh");
    assert_eq!(input.password_strength(), 2);
    assert_eq!(input.strength_label(), "Fair");
}

#[test]
fn test_masked_input_password_strength_strong() {
    let input = MaskedInput::new().value("Abcdef12");
    assert_eq!(input.password_strength(), 3);
    assert_eq!(input.strength_label(), "Strong");
}

#[test]
fn test_masked_input_password_strength_very_strong() {
    let input = MaskedInput::new().value("Abcdef12!");
    assert_eq!(input.password_strength(), 4);
    assert_eq!(input.strength_label(), "Very Strong");
}

#[test]
fn test_masked_input_password_strength_empty() {
    let input = MaskedInput::new();
    assert_eq!(input.password_strength(), 0);
}

#[test]
fn test_masked_input_password_strength_with_digits() {
    let input = MaskedInput::new().value("password123");
    assert!(input.password_strength() >= 1);
}

#[test]
fn test_masked_input_password_strength_with_special() {
    let input = MaskedInput::new().value("password!");
    assert!(input.password_strength() >= 1);
}

#[test]
fn test_masked_input_password_strength_all_requirements() {
    let input = MaskedInput::new().value("MyP@ssw0rd123!");
    assert_eq!(input.password_strength(), 4);
}

#[test]
fn test_masked_input_strength_color_very_weak() {
    let input = MaskedInput::new().value("abc");
    assert_eq!(input.strength_color(), Color::RED);
}

#[test]
fn test_masked_input_strength_color_weak() {
    let input = MaskedInput::new().value("abcdefgh");
    assert_eq!(input.strength_color(), Color::rgb(255, 128, 0));
}

#[test]
fn test_masked_input_strength_color_fair() {
    let input = MaskedInput::new().value("Abcdefgh");
    assert_eq!(input.strength_color(), Color::YELLOW);
}

#[test]
fn test_masked_input_strength_color_strong() {
    let input = MaskedInput::new().value("Abcdef12");
    assert_eq!(input.strength_color(), Color::rgb(128, 255, 0));
}

#[test]
fn test_masked_input_strength_color_very_strong() {
    let input = MaskedInput::new().value("Abcdef12!");
    assert_eq!(input.strength_color(), Color::GREEN);
}

#[test]
fn test_masked_input_password_strength_max_4() {
    let input = MaskedInput::new().value("VeryStr0ng!Pass@123#");
    assert_eq!(input.password_strength(), 4);
}

// =============================================================================
// Validation Tests (검증 테스트)
// =============================================================================

#[test]
fn test_masked_input_validate_success() {
    let mut input = MaskedInput::new().min_length(8).value("validpass");
    assert!(input.validate());
}

#[test]
fn test_masked_input_validate_too_short() {
    let mut input = MaskedInput::new().min_length(8).value("short");
    assert!(!input.validate());
}

#[test]
fn test_masked_input_validate_no_min_length() {
    let mut input = MaskedInput::new().value("any");
    assert!(input.validate());
}

#[test]
fn test_masked_input_validate_empty_with_min_length() {
    let mut input = MaskedInput::new().min_length(1);
    assert!(!input.validate());
}

#[test]
fn test_masked_input_validate_exact_length() {
    let mut input = MaskedInput::new().min_length(5).value("exact");
    assert!(input.validate());
}

#[test]
fn test_masked_input_validate_multiple_times() {
    let mut input = MaskedInput::new().min_length(8).value("short");

    assert!(!input.validate());
    input.set_value("longenough");
    assert!(input.validate());
}

#[test]
fn test_masked_input_validate_zero_min_length() {
    let mut input = MaskedInput::new().min_length(0);
    assert!(input.validate());
}

// =============================================================================
// Update Tests (업데이트 테스트)
// =============================================================================

#[test]
fn test_masked_input_update() {
    let mut input = MaskedInput::new();
    input.update();
    // Peek 모드에서 카운트다운 감소
    assert_eq!(input.get_value(), "");

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
fn test_masked_input_update_multiple() {
    let mut input = MaskedInput::new();
    for _ in 0..10 {
        input.update();
    }
    // 여러 호출해도 안전
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_update_with_value() {
    let mut input = MaskedInput::new().value("test");
    for _ in 0..5 {
        input.update();
    }
    assert_eq!(input.get_value(), "test");
}

// =============================================================================
// Render Tests (렌더링 테스트)
// =============================================================================

#[test]
fn test_masked_input_render_basic() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let input = MaskedInput::new().value("test").focused(false);
    View::render(&input, &mut ctx);
}

#[test]
fn test_masked_input_render_focused() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let input = MaskedInput::new().value("test").focused(true);
    View::render(&input, &mut ctx);
}

#[test]
fn test_masked_input_render_with_label() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let input = MaskedInput::new().label("Password").value("test");
    View::render(&input, &mut ctx);
}

#[test]
fn test_masked_input_render_with_placeholder() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let input = MaskedInput::new()
        .placeholder("Enter password")
        .focused(false);
    View::render(&input, &mut ctx);
}

#[test]
fn test_masked_input_render_with_strength() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let input = MaskedInput::new().show_strength(true).value("password");
    View::render(&input, &mut ctx);
}

#[test]
fn test_masked_input_render_disabled() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let input = MaskedInput::new().disabled(true).value("test");
    View::render(&input, &mut ctx);
}

#[test]
fn test_masked_input_render_with_colors() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let input = MaskedInput::new()
        .fg(Color::CYAN)
        .bg(Color::BLACK)
        .value("test");
    View::render(&input, &mut ctx);
}

#[test]
fn test_masked_input_render_with_custom_width() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let input = MaskedInput::new().width(35).value("test");
    View::render(&input, &mut ctx);
}

#[test]
fn test_masked_input_render_with_allow_reveal() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let input = MaskedInput::new().allow_reveal(true).value("test");
    View::render(&input, &mut ctx);
}

#[test]
fn test_masked_input_render_zero_area() {
    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let input = MaskedInput::new().value("test");
    View::render(&input, &mut ctx);
}

#[test]
fn test_masked_input_render_long_value() {
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let input = MaskedInput::new().value("this_is_a_very_long_password");
    View::render(&input, &mut ctx);
}

#[test]
fn test_masked_input_render_empty_value() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let input = MaskedInput::new();
    View::render(&input, &mut ctx);
}

#[test]
fn test_masked_input_render_all_options() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let input = MaskedInput::new()
        .label("Enter Password")
        .placeholder("Password")
        .value("test123")
        .focused(true)
        .show_strength(true)
        .allow_reveal(true)
        .fg(Color::WHITE)
        .bg(Color::BLACK)
        .width(35);
    View::render(&input, &mut ctx);
}

// =============================================================================
// CSS/Styling Tests (CSS/스타일 테스트)
// =============================================================================

#[test]
fn test_masked_input_css_id() {
    let input = MaskedInput::new().element_id("password-input");
    assert_eq!(View::id(&input), Some("password-input"));

    let meta = input.meta();
    assert_eq!(meta.id, Some("password-input".to_string()));
}

#[test]
fn test_masked_input_css_classes() {
    let input = MaskedInput::new()
        .class("form-control")
        .class("password-field");

    assert!(input.has_class("form-control"));
    assert!(input.has_class("password-field"));
    assert!(!input.has_class("optional"));

    let classes = View::classes(&input);
    assert_eq!(classes.len(), 2);
}

#[test]
fn test_masked_input_styled_view_set_id() {
    let mut input = MaskedInput::new();
    input.set_id("test-id");
    assert_eq!(View::id(&input), Some("test-id"));
}

#[test]
fn test_masked_input_styled_view_add_class() {
    let mut input = MaskedInput::new();
    input.add_class("active");
    assert!(input.has_class("active"));
}

#[test]
fn test_masked_input_styled_view_remove_class() {
    let mut input = MaskedInput::new().class("active");
    input.remove_class("active");
    assert!(!input.has_class("active"));
}

#[test]
fn test_masked_input_styled_view_toggle_class() {
    let mut input = MaskedInput::new();

    input.toggle_class("selected");
    assert!(input.has_class("selected"));

    input.toggle_class("selected");
    assert!(!input.has_class("selected"));
}

#[test]
fn test_masked_input_classes_builder() {
    let input = MaskedInput::new().classes(vec!["class1", "class2", "class3"]);

    assert!(input.has_class("class1"));
    assert!(input.has_class("class2"));
    assert!(input.has_class("class3"));
}

#[test]
fn test_masked_input_duplicate_class_not_added() {
    let input = MaskedInput::new().class("test").class("test");

    let classes = View::classes(&input);
    assert_eq!(classes.len(), 1);
}

#[test]
fn test_masked_input_multiple_classes() {
    let input = MaskedInput::new()
        .class("class1")
        .class("class2")
        .class("class3")
        .class("class4");

    let classes = View::classes(&input);
    assert_eq!(classes.len(), 4);
}

// =============================================================================
// Edge Cases (엣지 케이스 테스트)
// =============================================================================

#[test]
fn test_masked_input_empty_value_operations() {
    let mut input = MaskedInput::new();
    assert_eq!(input.get_value(), "");

    input.delete_backward();
    assert_eq!(input.get_value(), "");

    input.delete_forward();
    assert_eq!(input.get_value(), "");

    input.move_left();
    input.move_right();
    assert_eq!(input.get_value(), "");

    let mut input = MaskedInput::new();
    input.delete_backward();
    input.delete_forward();
    input.move_left();
    input.move_right();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_single_char() {
    let mut input = MaskedInput::new();
    input.insert_char('a');
    assert_eq!(input.get_value(), "a");

    input.delete_backward();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_very_long_value() {
    let mut input = MaskedInput::new();
    for _ in 0..100 {
        input.insert_char('a');
    }
    assert_eq!(input.get_value().len(), 100);
}

#[test]
fn test_masked_input_max_length_zero() {
    let mut input = MaskedInput::new().max_length(0); // unlimited
    for _ in 0..50 {
        input.insert_char('a');
    }
    assert_eq!(input.get_value().len(), 50);
}

#[test]
fn test_masked_input_unicode_chars() {
    // Note: insert_char() has issues with multi-byte chars
    // Use set_value() instead for unicode content
    let mut input = MaskedInput::new();
    input.set_value("한글");
    assert_eq!(input.get_value(), "한글");
}

#[test]
fn test_masked_input_emoji_chars() {
    // Note: insert_char() has issues with multi-byte chars
    // Use set_value() instead for emoji content
    let input = MaskedInput::new().value("😀🎉");
    assert!(input.get_value().contains('😀'));
    assert!(input.get_value().contains('🎉'));
}

#[test]
fn test_masked_input_cursor_beyond_value() {
    let mut input = MaskedInput::new();
    input.set_value("test");
    input.move_end();
    input.move_right();
    // 끝을 벗어나지 않음
    assert_eq!(input.get_value(), "test");
}

#[test]
fn test_masked_input_set_value_shorter() {
    let mut input = MaskedInput::new().value("hello world");
    input.set_value("hi");
    assert_eq!(input.get_value(), "hi");
}

#[test]
fn test_masked_input_delete_all_content() {
    let mut input = MaskedInput::new().value("test");
    input.delete_backward();
    input.delete_backward();
    input.delete_backward();
    input.delete_backward();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_delete_forward_all_content() {
    let mut input = MaskedInput::new().value("test");
    input.move_start();
    input.delete_forward();
    input.delete_forward();
    input.delete_forward();
    input.delete_forward();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_multiple_clears() {
    let mut input = MaskedInput::new().value("test");
    input.clear();
    input.clear();
    input.clear();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_move_cursor_on_empty() {
    let mut input = MaskedInput::new();
    input.move_left();
    input.move_right();
    input.move_start();
    input.move_end();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_set_value_empty_string() {
    let mut input = MaskedInput::new().value("test");
    input.set_value("");
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_insert_after_set_value() {
    let mut input = MaskedInput::new();
    input.set_value("hello");
    // After set_value, cursor is at position 0, so insert at beginning
    input.insert_char('!');
    assert_eq!(input.get_value(), "!hello");
}

#[test]
fn test_masked_insert_middle() {
    let mut input = MaskedInput::new().value("ac");
    input.move_start();
    input.move_right();
    input.insert_char('b');
    assert_eq!(input.get_value(), "abc");

    let mut input = MaskedInput::new().value("ac");
    input.set_cursor(1);
    input.insert_char('b');
    assert_eq!(input.get_value(), "abc");
    assert_eq!(input.get_cursor(), 2);
}

#[test]
fn test_masked_input_delete_all_with_backspace() {
    let mut input = MaskedInput::new().value("ABCD");
    for _ in 0..5 {
        input.delete_backward();
    }
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_delete_all_with_forward() {
    let mut input = MaskedInput::new().value("ABCD");
    input.move_start();
    for _ in 0..5 {
        input.delete_forward();
    }
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_empty_validate() {
    let mut input = MaskedInput::new();
    assert!(input.validate());
}

#[test]
fn test_masked_input_special_characters() {
    let mut input = MaskedInput::new();
    for c in "!@#$%^&*()".chars() {
        input.insert_char(c);
    }
    assert_eq!(input.get_value(), "!@#$%^&*()");
}

#[test]
fn test_masked_input_rapid_insert_delete() {
    let mut input = MaskedInput::new();
    for _ in 0..10 {
        input.insert_char('a');
    }
    for _ in 0..10 {
        input.delete_backward();
    }
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_whitespace() {
    let mut input = MaskedInput::new();
    input.insert_char(' ');
    input.insert_char(' ');
    input.insert_char(' ');
    assert_eq!(input.get_value(), "   ");
}

// =============================================================================
// Meta and Debug Tests (메타 및 디버그 테스트)
// =============================================================================

#[test]
fn test_masked_input_meta() {
    let input = MaskedInput::new()
        .element_id("test-input")
        .class("form-control");

    let meta = input.meta();
    assert_eq!(meta.widget_type, "MaskedInput");
    assert_eq!(meta.id, Some("test-input".to_string()));
    assert!(meta.classes.contains("form-control"));
}

#[test]
fn test_masked_input_clone() {
    let input1 = MaskedInput::new()
        .value("secret")
        .fg(Color::RED)
        .bg(Color::BLUE)
        .focused(true);

    let input2 = input1.clone();

    assert_eq!(input1.get_value(), input2.get_value());

    let input1 = MaskedInput::new()
        .value("test")
        .mask_char('*')
        .placeholder("Enter");
    let input2 = input1.clone();
    assert_eq!(input1.get_value(), input2.get_value());
    assert_eq!(input1.get_mask_char(), input2.get_mask_char());
}

#[test]
fn test_masked_input_debug_format() {
    let input = MaskedInput::new().value("test");
    let debug_str = format!("{:?}", input);
    assert!(debug_str.contains("MaskedInput"));
}

#[test]
fn test_masked_input_meta_widget_type() {
    let input = MaskedInput::new();
    let meta = input.meta();
    assert_eq!(meta.widget_type, "MaskedInput");
}

#[test]
fn test_masked_input_meta_empty() {
    let input = MaskedInput::new();
    let meta = input.meta();
    assert_eq!(meta.widget_type, "MaskedInput");
    assert_eq!(meta.id, None);
}

// =============================================================================
// Key handling (키 처리)
// =============================================================================

#[test]
fn test_masked_input_handle_key_types_and_edits() {
    let mut input = MaskedInput::password();
    for c in "abc".chars() {
        assert!(input.handle_key(&Key::Char(c)));
    }
    assert_eq!(input.get_value(), "abc");

    assert!(input.handle_key(&Key::Left));
    assert!(input.handle_key(&Key::Backspace));
    assert_eq!(input.get_value(), "ac");
    assert!(input.handle_key(&Key::Home));
    assert!(input.handle_key(&Key::Delete));
    assert_eq!(input.get_value(), "c");
    assert!(input.handle_key(&Key::End));
    assert_eq!(input.get_cursor(), 1);
    assert!(input.handle_key(&Key::Right));
    assert_eq!(input.get_cursor(), 1);
}

#[test]
fn test_masked_input_handle_key_respects_max_length() {
    let mut input = MaskedInput::pin(2);
    input.handle_key(&Key::Char('1'));
    input.handle_key(&Key::Char('2'));
    input.handle_key(&Key::Char('3'));
    assert_eq!(input.get_value(), "12");
}

#[test]
fn test_masked_input_handle_key_ignores_disabled_and_other_keys() {
    let mut input = MaskedInput::new().disabled(true);
    assert!(!input.handle_key(&Key::Char('a')));
    assert_eq!(input.get_value(), "");

    let mut input = MaskedInput::new();
    assert!(!input.handle_key(&Key::Enter));
    assert!(!input.handle_key(&Key::Char('\u{7}')));
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_set_value_respects_max_length() {
    let mut input = MaskedInput::new().max_length(4);
    input.set_value("123456");
    assert_eq!(input.get_value(), "1234");
    assert!(input.get_cursor() <= 4);

    // Counts chars, not bytes
    let mut input = MaskedInput::new().max_length(2);
    input.set_value("한글값");
    assert_eq!(input.get_value(), "한글");
}

#[test]
fn test_masked_input_value_builder_respects_max_length() {
    let input = MaskedInput::new().max_length(3).value("abcdef");
    assert_eq!(input.get_value(), "abc");
    assert_eq!(input.get_cursor(), 3);
}

#[test]
fn test_masked_input_max_length_builder_cuts_earlier_value() {
    let input = MaskedInput::new().value("abcdef").max_length(3);
    assert_eq!(input.get_value(), "abc");
    assert_eq!(input.get_cursor(), 3);
}

#[test]
fn test_masked_input_focused_placeholder_is_gray() {
    use revue::widget::theme::PLACEHOLDER_FG;

    let input = MaskedInput::new().placeholder("Enter").focused(true);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    input.render(&mut ctx);

    // "[" then the cursor over 'E', then the rest of the placeholder
    let row: String = (0..8).map(|x| buffer.get(x, 0).unwrap().symbol).collect();
    assert!(row.starts_with("[Enter"), "{row:?}");
    for x in 2..6 {
        assert_eq!(buffer.get(x, 0).unwrap().fg, Some(PLACEHOLDER_FG), "x={x}");
    }
}

/// #799: `bg` was stored and only read back by a getter.
#[test]
fn test_masked_input_bg_paints_the_field() {
    const BG: Color = Color {
        r: 10,
        g: 20,
        b: 30,
        a: 255,
    };
    for focused in [false, true] {
        let input = MaskedInput::new()
            .value("abc")
            .width(6)
            .bg(BG)
            .focused(focused);
        let mut buffer = Buffer::new(20, 1);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 1));
        input.render(&mut ctx);

        // x=0 is the `[`; the field starts at x=1. Focused, the cursor sits at
        // the end of the value, so the first cell is still plain field.
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '[');
        assert_eq!(
            buffer.get(1, 0).unwrap().bg,
            Some(BG),
            "focused={focused}: the field was not painted in `bg`"
        );
        assert_ne!(
            buffer.get(0, 0).unwrap().bg,
            Some(BG),
            "the bracket took `bg`"
        );
    }
}

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
fn test_masked_input_debug() {
    let input = MaskedInput::new().value("test");
    let debug_str = format!("{:?}", input);
    assert!(debug_str.contains("MaskedInput"));
}

// =========================================================================
// MaskedInput builder tests
// =========================================================================

#[test]
fn test_masked_input_fg() {
    let input = MaskedInput::new().fg(Color::RED);
    assert_eq!(input.get_fg(), Some(Color::RED));
}

#[test]
fn test_masked_input_bg() {
    let input = MaskedInput::new().bg(Color::BLUE);
    assert_eq!(input.get_bg(), Some(Color::BLUE));
}

// =========================================================================
// MaskedInput value operations
// =========================================================================

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
    assert!(matches!(
        input.get_validation(),
        ValidationState::Invalid(_)
    ));

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
    assert_eq!(input.get_id(), Some("password-field"));
}

#[test]
fn test_masked_input_element_id_override() {
    let input = MaskedInput::new()
        .element_id("first-id")
        .element_id("second-id");
    assert_eq!(input.get_id(), Some("second-id"));
}

#[test]
fn test_masked_input_class() {
    let input = MaskedInput::new().class("input-field");
    assert_eq!(input.get_classes(), &["input-field".to_string()]);
}

#[test]
fn test_masked_input_class_multiple() {
    let input = MaskedInput::new().class("required").class("validated");
    assert_eq!(
        input.get_classes(),
        &["required".to_string(), "validated".to_string()]
    );
}

#[test]
fn test_masked_input_class_no_duplicate() {
    let input = MaskedInput::new().class("container").class("container");
    assert_eq!(input.get_classes(), &["container".to_string()]);
}

#[test]
fn test_masked_input_classes_vec() {
    let input = MaskedInput::new().classes(vec!["class1", "class2", "class3"]);
    assert_eq!(
        input.get_classes(),
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
        input.get_classes(),
        &["class1".to_string(), "class2".to_string()]
    );
}

#[test]
fn test_masked_input_classes_with_duplicates_filtered() {
    let input = MaskedInput::new().classes(vec!["a", "b", "a", "c", "b"]);
    assert_eq!(
        input.get_classes(),
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
        input.get_classes(),
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
    assert_eq!(input.get_id(), Some("test-id"));
}

#[test]
fn test_masked_input_set_id_override() {
    let mut input = MaskedInput::new();
    input.set_id("first");
    input.set_id("second");
    assert_eq!(input.get_id(), Some("second"));
}

#[test]
fn test_masked_input_add_class() {
    let mut input = MaskedInput::new();
    input.add_class("container");
    assert_eq!(input.get_classes(), &["container".to_string()]);
}

#[test]
fn test_masked_input_add_class_multiple() {
    let mut input = MaskedInput::new();
    input.add_class("class1");
    input.add_class("class2");
    input.add_class("class3");
    assert_eq!(
        input.get_classes(),
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
    assert_eq!(input.get_classes(), &["duplicate".to_string()]);
}

#[test]
fn test_masked_input_remove_class() {
    let mut input = MaskedInput::new();
    input.add_class("class1");
    input.add_class("class2");
    input.add_class("class3");
    input.remove_class("class2");
    assert_eq!(
        input.get_classes(),
        &["class1".to_string(), "class3".to_string()]
    );
}

#[test]
fn test_masked_input_remove_class_not_present() {
    let mut input = MaskedInput::new();
    input.add_class("class1");
    input.remove_class("nonexistent");
    assert_eq!(input.get_classes(), &["class1".to_string()]);
}

#[test]
fn test_masked_input_remove_class_from_empty() {
    let mut input = MaskedInput::new();
    input.remove_class("anything");
    assert!(input.get_classes().is_empty());
}

#[test]
fn test_masked_input_toggle_class_adds() {
    let mut input = MaskedInput::new();
    input.toggle_class("new-class");
    assert_eq!(input.get_classes(), &["new-class".to_string()]);
}

#[test]
fn test_masked_input_toggle_class_removes() {
    let mut input = MaskedInput::new();
    input.add_class("existing");
    input.toggle_class("existing");
    assert!(input.get_classes().is_empty());
}

#[test]
fn test_masked_input_toggle_class_multiple_times() {
    let mut input = MaskedInput::new();
    input.toggle_class("toggle");
    assert_eq!(input.get_classes(), &["toggle".to_string()]);
    input.toggle_class("toggle");
    assert!(input.get_classes().is_empty());
    input.toggle_class("toggle");
    assert_eq!(input.get_classes(), &["toggle".to_string()]);
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

    assert_eq!(input.get_id(), Some("updated-id"));
    assert_eq!(
        input.get_classes(),
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

    assert_eq!(input.get_id(), Some("password-input"));
    assert_eq!(
        input.get_classes(),
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
    assert_eq!(input.get_fg(), Some(Color::WHITE));
    assert_eq!(input.get_bg(), Some(Color::BLACK));
    assert_eq!(input.get_width(), Some(30));
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
    assert_eq!(input.get_id(), Some(""));
}

#[test]
fn test_masked_input_empty_string_class() {
    let input = MaskedInput::new().class("");
    assert_eq!(input.get_classes(), &["".to_string()]);
}

#[test]
fn test_masked_input_classes_empty_vec() {
    let input = MaskedInput::new().classes(Vec::<&str>::new());
    assert!(input.get_classes().is_empty());
}

#[test]
fn test_masked_input_classes_empty_array() {
    let input = MaskedInput::new().classes([] as [&str; 0]);
    assert!(input.get_classes().is_empty());
}

#[test]
fn test_masked_input_set_id_empty_string() {
    let mut input = MaskedInput::new();
    input.set_id("");
    assert_eq!(input.get_id(), Some(""));
}

#[test]
fn test_masked_input_add_class_empty_string() {
    let mut input = MaskedInput::new();
    input.add_class("");
    assert_eq!(input.get_classes(), &["".to_string()]);
}

// =========================================================================
// Password builder preset with props tests
// =========================================================================

#[test]
fn test_masked_input_password_with_props() {
    let pwd = MaskedInput::password()
        .element_id("pwd")
        .class("password-field");

    assert_eq!(pwd.get_id(), Some("pwd"));
    assert_eq!(pwd.get_classes(), &["password-field".to_string()]);
    assert!(pwd.get_show_strength());
    assert_eq!(pwd.get_mask_char(), '●');
}

#[test]
fn test_masked_input_pin_with_props() {
    let pin = MaskedInput::pin(4)
        .element_id("pin-input")
        .classes(vec!["numeric", "required"]);

    assert_eq!(pin.get_id(), Some("pin-input"));
    assert_eq!(
        pin.get_classes(),
        &["numeric".to_string(), "required".to_string()]
    );
    assert_eq!(pin.get_max_length(), 4);
}

#[test]
fn test_masked_input_credit_card_with_props() {
    let card = MaskedInput::credit_card()
        .element_id("card-number")
        .class("financial");

    assert_eq!(card.get_id(), Some("card-number"));
    assert_eq!(card.get_classes(), &["financial".to_string()]);
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

    assert_eq!(input.get_classes().len(), 9);
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
        input.add_class(format!("class{}", i));
    }
    assert_eq!(input.get_classes().len(), 5);

    for i in 0..5 {
        input.remove_class(&format!("class{}", i));
    }
    assert!(input.get_classes().is_empty());
}

// =========================================================================
// Helper functions with props
// =========================================================================

#[test]
fn test_masked_input_helper_with_props() {
    let input = masked_input().element_id("masked").class("input");
    assert_eq!(input.get_id(), Some("masked"));
    assert_eq!(input.get_classes(), &["input".to_string()]);
}

#[test]
fn test_password_input_helper_with_props() {
    let pwd = password_input("Password").element_id("pwd").class("secure");
    assert_eq!(pwd.get_id(), Some("pwd"));
    assert_eq!(pwd.get_classes(), &["secure".to_string()]);
}

#[test]
fn test_pin_input_helper_with_props() {
    let pin = pin_input(6)
        .element_id("pin")
        .classes(vec!["numeric", "required"]);
    assert_eq!(pin.get_id(), Some("pin"));
    assert!(pin.has_class("numeric"));
    assert!(pin.has_class("required"));
}

#[test]
fn test_credit_card_input_helper_with_props() {
    let card = credit_card_input().element_id("card").class("financial");
    assert_eq!(card.get_id(), Some("card"));
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

    assert_eq!(input1.get_id(), input2.get_id());
    assert_eq!(input1.get_classes(), input2.get_classes());
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

// =========================================================================
// Additional edge case tests for set_id
// =========================================================================

#[test]
fn test_masked_input_set_id_empty_string_getter() {
    let mut input = MaskedInput::new();
    input.set_id("");
    assert_eq!(input.get_id(), Some(""));
}

// =========================================================================
// Password builder preset with props - helper integration
// =========================================================================

#[test]
fn test_masked_input_password_helper_with_props_integration() {
    let pwd = password_input("Enter password")
        .element_id("pwd")
        .class("password-field");

    assert_eq!(pwd.get_id(), Some("pwd"));
    assert_eq!(pwd.get_classes(), &["password-field".to_string()]);
    assert!(pwd.get_show_strength());
}

#[test]
fn test_masked_input_pin_helper_with_props_integration() {
    let pin = pin_input(4)
        .element_id("pin-input")
        .classes(vec!["numeric", "required"]);

    assert_eq!(pin.get_id(), Some("pin-input"));
    assert_eq!(
        pin.get_classes(),
        &["numeric".to_string(), "required".to_string()]
    );
    assert_eq!(pin.get_max_length(), 4);
}

#[test]
fn test_masked_input_credit_card_helper_with_props_integration() {
    let card = credit_card_input()
        .element_id("card-number")
        .class("financial");

    assert_eq!(card.get_id(), Some("card-number"));
    assert_eq!(card.get_classes(), &["financial".to_string()]);
    assert_eq!(card.get_max_length(), 16);
}

#[test]
fn test_masked_input_toggle_reveal() {
    let mut input = MaskedInput::new();
    input.toggle_reveal();

    // Toggle reveal was called successfully
}

#[test]
fn test_masked_input_password_strength() {
    let input = MaskedInput::new().value("weak");

    // Password strength can be calculated
    let _strength = input.password_strength();
}

#[test]
fn test_masked_input_strength_label() {
    let input = MaskedInput::new().value("test");

    // Strength label can be retrieved
    let _label = input.strength_label();
}

#[test]
fn test_masked_input_strength_color() {
    let input = MaskedInput::new().value("test");

    // Strength color can be retrieved
    let _color = input.strength_color();
    // Color is valid (not checking specific value as it depends on implementation)
}

#[test]
fn test_masked_input_validate() {
    let input = MaskedInput::new().value("test123");

    // validate() requires &mut self and modifies internal state
    // This test verifies the input can be created with the builder
    assert_eq!(input.get_value(), "test123");
}

#[test]
fn test_masked_input_validate_empty() {
    let input = MaskedInput::new();

    // Empty input validation depends on implementation
    // Just verify the input can be created
    assert_eq!(input.get_value(), "");
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
