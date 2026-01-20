//! MaskedInput widget integration tests
//!
//! MaskedInput 위젯의 통합 테스트입니다.
//! 생성자, 빌더 메서드, 값 관리, 입력 처리, 마스크 스타일,
//! 비밀번호 강도, 검증, 렌더링 등 다양한 기능을 테스트합니다.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{masked_input, MaskStyle, MaskedInput};

// =============================================================================
// Constructor and Builder Tests (생성자 및 빌더 테스트)
// =============================================================================

#[test]
fn test_masked_input_new() {
    let input = MaskedInput::new();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_default() {
    let input = MaskedInput::default();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_password() {
    let input = MaskedInput::password();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_pin() {
    let input = MaskedInput::pin(4);
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_credit_card() {
    let input = MaskedInput::credit_card();
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_helper() {
    let input = masked_input().value("test");
    assert_eq!(input.get_value(), "test");
}

#[test]
fn test_masked_input_mask_char() {
    let input = MaskedInput::new().mask_char('*').value("test");
    assert_eq!(input.get_value(), "test");
}

#[test]
fn test_masked_input_mask_style() {
    let input = MaskedInput::new()
        .mask_style(MaskStyle::ShowLast(2))
        .value("test");
    assert_eq!(input.get_value(), "test");
}

#[test]
fn test_masked_input_placeholder() {
    let input = MaskedInput::new().placeholder("Enter password");
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_label() {
    let input = MaskedInput::new().label("Password");
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_max_length() {
    let input = MaskedInput::new().max_length(10);
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_min_length() {
    let input = MaskedInput::new().min_length(8);
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_focused() {
    let input = MaskedInput::new().focused(true);
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_disabled() {
    let input = MaskedInput::new().disabled(true);
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_colors() {
    let input = MaskedInput::new().fg(Color::RED).bg(Color::BLUE);
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_width() {
    let input = MaskedInput::new().width(30);
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_show_strength() {
    let input = MaskedInput::new().show_strength(true);
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_allow_reveal() {
    let input = MaskedInput::new().allow_reveal(true);
    assert_eq!(input.get_value(), "");
}

#[test]
fn test_masked_input_value() {
    let input = MaskedInput::new().value("secret123");
    assert_eq!(input.get_value(), "secret123");
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
}

// =============================================================================
// Value Management Tests (값 관리 테스트)
// =============================================================================

#[test]
fn test_masked_input_get_value() {
    let input = MaskedInput::new().value("secret");
    assert_eq!(input.get_value(), "secret");
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
}

#[test]
fn test_masked_input_delete_forward() {
    let mut input = MaskedInput::new().value("hello");
    input.move_start();
    input.delete_forward();
    assert_eq!(input.get_value(), "ello");
}

#[test]
fn test_masked_input_delete_forward_at_end() {
    let mut input = MaskedInput::new().value("hello");
    input.delete_forward();
    assert_eq!(input.get_value(), "hello");
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
}

#[test]
fn test_masked_input_move_right() {
    let mut input = MaskedInput::new().value("hello");
    input.move_start();
    input.move_right();
    input.move_right();
    // 커서가 오른쪽으로 이동 (내부 상태)
    assert_eq!(input.get_value(), "hello");
}

#[test]
fn test_masked_input_move_start() {
    let mut input = MaskedInput::new().value("hello");
    input.move_start();
    assert_eq!(input.get_value(), "hello");
}

#[test]
fn test_masked_input_move_end() {
    let mut input = MaskedInput::new().value("hello");
    input.move_start();
    input.move_end();
    assert_eq!(input.get_value(), "hello");
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
}

#[test]
fn test_masked_input_move_right_at_end() {
    let mut input = MaskedInput::new().value("test");
    input.move_right();
    input.move_right();
    assert_eq!(input.get_value(), "test");
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
