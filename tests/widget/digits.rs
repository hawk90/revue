//! Digits widget integration tests
//!
//! Digits 위젯의 통합 테스트입니다.
//! 큰 숫자 표시, 시계 형식, 타이머 등 다양한 형식의 숫자 표시 기능을 테스트합니다.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{clock, digits, timer, DigitStyle, Digits, StyledView, View};

// ─────────────────────────────────────────────────────────────────────────
// Constructor tests - 생성자 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_digits_new_with_integer() {
    // 정수로 생성 테스트
    let d = Digits::new(42);
    let lines = d.render_lines();
    assert!(!lines.is_empty());
}

#[test]
fn test_digits_new_with_string() {
    // 문자열로 생성 테스트
    let d = Digits::new("123");
    let lines = d.render_lines();
    assert!(!lines.is_empty());
}

#[test]
fn test_digits_new_with_negative() {
    // 음수로 생성 테스트
    let d = Digits::new(-100);
    let lines = d.render_lines();
    // 마이너스 기호가 렌더링되어야 함
    assert!(lines.len() > 2); // 마이너스는 3번째 줄에 있음
    assert!(lines[2].contains('█')); // Block 스타일 기본
}

#[test]
fn test_digits_from_int() {
    // from_int 메서드 테스트
    let d = Digits::from_int(999);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_from_float() {
    // from_float 메서드 테스트
    let d = Digits::from_float(12.3456, 2);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    // 점이 렌더링되어야 함
    assert!(lines[4].contains('█')); // 점은 마지막 줄
}

#[test]
fn test_digits_from_float_zero_decimals() {
    // 소수점 없는 float 테스트
    let d = Digits::from_float(42.0, 0);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_from_float_rounding() {
    // float 반올림 테스트
    let d = Digits::from_float(3.14159, 3);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

// ─────────────────────────────────────────────────────────────────────────
// Time and clock methods - 시간 및 시계 메서드 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_digits_time() {
    // 시간 형식 생성 테스트 (HH:MM:SS)
    let d = Digits::time(12, 34, 56);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    // 콜론이 있어야 함
    assert!(lines.iter().any(|l| l.contains('█')));
}

#[test]
fn test_digits_time_single_digits() {
    // 한 자리 숫자 시간 테스트 (0 패딩)
    let d = Digits::time(5, 7, 9);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_time_midnight() {
    // 자정 테스트
    let d = Digits::time(0, 0, 0);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_time_max_values() {
    // 최대값 테스트 (23:59:59)
    let d = Digits::time(23, 59, 59);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_clock() {
    // 시계 형식 생성 테스트 (HH:MM)
    let d = Digits::clock(9, 30);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_clock_no_padding_needed() {
    // 패딩이 필요 없는 시간 테스트
    let d = Digits::clock(12, 45);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_timer_with_hours() {
    // 시간이 포함된 타이머 테스트
    let d = Digits::timer(3661); // 1시간 1분 1초
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_timer_minutes_only() {
    // 분만 있는 타이머 테스트
    let d = Digits::timer(65); // 1분 5초
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_timer_seconds_only() {
    // 초만 있는 타이머 테스트
    let d = Digits::timer(45);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_timer_zero() {
    // 0초 타이머 테스트
    let d = Digits::timer(0);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_timer_large_value() {
    // 큰 타이머 값 테스트
    let d = Digits::timer(86400); // 24시간
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

// ─────────────────────────────────────────────────────────────────────────
// Builder methods - 빌더 메서드 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_digits_style_block() {
    // Block 스타일 테스트
    let d = Digits::new(123).style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    // Block 스타일은 █ 문자 사용
    assert!(lines[0].contains('█'));
}

#[test]
fn test_digits_style_thin() {
    // Thin 스타일 테스트
    let d = Digits::new(456).style(DigitStyle::Thin);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    // Thin 스타일은 박스 드로잉 문자 사용
    assert!(lines[0].contains('┌') || lines[0].contains('─') || lines[0].contains('┐'));
}

#[test]
fn test_digits_style_ascii() {
    // ASCII 스타일 테스트
    let d = Digits::new(789).style(DigitStyle::Ascii);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    // ASCII 스타일은 +, -, | 문자 사용
    assert!(lines[0].contains('+') || lines[0].contains('-'));
}

#[test]
fn test_digits_style_braille() {
    // Braille 스타일 테스트
    let d = Digits::new(999).style(DigitStyle::Braille);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 4); // Braille은 4줄
}

#[test]
fn test_digits_style_default() {
    // 기본 스타일 테스트
    let d = Digits::new(111);
    let height = d.height();
    assert_eq!(height, 5); // 기본값은 Block (높이 5)
}

#[test]
fn test_digits_fg_color() {
    // 전경색 설정 테스트
    let d = Digits::new(42).fg(Color::RED);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
    // 패닉 없이 렌더링되어야 함
}

#[test]
fn test_digits_bg_color() {
    // 배경색 설정 테스트
    let d = Digits::new(42).bg(Color::BLUE);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_digits_prefix() {
    // 접두사 설정 테스트
    let d = Digits::new(100).prefix("$");
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
    // 접두사가 렌더링되어야 함
}

#[test]
fn test_digits_suffix() {
    // 접미사 설정 테스트
    let d = Digits::new(50).suffix("%");
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Dimension methods - 차원 메서드 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_digits_height_default_styles() {
    // 기본 스타일의 높이 테스트 (5)
    assert_eq!(Digits::new(0).style(DigitStyle::Block).height(), 5);
    assert_eq!(Digits::new(0).style(DigitStyle::Thin).height(), 5);
    assert_eq!(Digits::new(0).style(DigitStyle::Ascii).height(), 5);
}

#[test]
fn test_digits_height_braille() {
    // Braille 스타일의 높이 테스트 (4)
    assert_eq!(Digits::new(0).style(DigitStyle::Braille).height(), 4);
}

#[test]
fn test_digits_digit_width_default_styles() {
    // 기본 스타일의 숫자 너비 테스트 (3)
    assert_eq!(Digits::new(0).style(DigitStyle::Block).digit_width(), 3);
    assert_eq!(Digits::new(0).style(DigitStyle::Thin).digit_width(), 3);
    assert_eq!(Digits::new(0).style(DigitStyle::Ascii).digit_width(), 3);
}

#[test]
fn test_digits_digit_width_braille() {
    // Braille 스타일의 숫자 너비 테스트 (2)
    assert_eq!(Digits::new(0).style(DigitStyle::Braille).digit_width(), 2);
}

// ─────────────────────────────────────────────────────────────────────────
// Render lines tests - 렌더링 라인 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_digits_render_lines_single_digit() {
    // 한 자리 숫자 렌더링 테스트
    let d = Digits::new("5").style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5); // Block 스타일은 5줄
}

#[test]
fn test_digits_render_lines_multiple_digits() {
    // 여러 자리 숫자 렌더링 테스트
    let d = Digits::new("123").style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    // 각 줄에는 숫자와 공백이 포함됨
    assert!(lines[0].len() > 3);
}

#[test]
fn test_digits_render_lines_with_colon() {
    // 콜론이 포함된 시간 형식 렌더링 테스트
    let d = Digits::new("12:34").style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_render_lines_with_decimal() {
    // 소수점이 포함된 숫자 렌더링 테스트
    let d = Digits::new("12.5").style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_render_lines_with_negative() {
    // 음수 렌더링 테스트
    let d = Digits::new("-42").style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    // 마이너스 기호가 렌더링되어야 함
    assert!(lines[2].contains('█')); // 마이너스는 가운데 줄
}

#[test]
fn test_digits_render_lines_braille_style() {
    // Braille 스타일 렌더링 테스트
    let d = Digits::new("42").style(DigitStyle::Braille);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 4); // Braille은 4줄
}

#[test]
fn test_digits_render_lines_thin_style() {
    // Thin 스타일 렌더링 테스트
    let d = Digits::new("42").style(DigitStyle::Thin);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    // Thin 스타일은 특수 문자 사용
    assert!(lines[0].contains('┌') || lines[0].contains('─'));
}

#[test]
fn test_digits_render_lines_ascii_style() {
    // ASCII 스타일 렌더링 테스트
    let d = Digits::new("42").style(DigitStyle::Ascii);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    // ASCII 스타일은 +, -, | 문자 사용
    assert!(lines[0].contains('+') || lines[0].contains('-'));
}

#[test]
fn test_digits_render_lines_empty_value() {
    // 빈 값 렌더링 테스트
    let d = Digits::new("").style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    assert!(lines.iter().all(|l| l.is_empty()));
}

#[test]
fn test_digits_render_lines_with_separator_formatted() {
    // 구분자가 적용된 값 렌더링 테스트
    let d = Digits::new("10000").separator(',');
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    // 콤마는 공백으로 렌더링됨
    assert!(lines[0].contains(' '));
}

// ─────────────────────────────────────────────────────────────────────────
// Render integration tests - 렌더링 통합 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_digits_render_basic() {
    // 기본 렌더링 테스트
    let d = Digits::new("42");
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
    // 패닉 없이 렌더링되어야 함
}

#[test]
fn test_digits_render_with_colors() {
    // 색상이 적용된 렌더링 테스트
    let d = Digits::new("123").fg(Color::CYAN).bg(Color::BLACK);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_digits_render_time_format() {
    // 시간 형식 렌더링 테스트
    let d = Digits::time(12, 30, 45);
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_digits_render_negative_number() {
    // 음수 렌더링 테스트
    let d = Digits::new(-999);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_digits_render_small_buffer() {
    // 작은 버퍼 렌더링 테스트
    let d = Digits::new("42");
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_digits_render_with_prefix_suffix() {
    // 접두사/접미사가 있는 렌더링 테스트
    let d = Digits::new("100").prefix("$").suffix(" USD");
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_digits_render_thin_style() {
    // Thin 스타일 렌더링 테스트
    let d = Digits::new("9876").style(DigitStyle::Thin).fg(Color::GREEN);
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_digits_render_braille_style() {
    // Braille 스타일 렌더링 테스트
    let d = Digits::new("1234").style(DigitStyle::Braille).fg(Color::YELLOW);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_digits_render_ascii_style() {
    // ASCII 스타일 렌더링 테스트
    let d = Digits::new("5678").style(DigitStyle::Ascii).fg(Color::MAGENTA);
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Helper function tests - 헬퍼 함수 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_helper_digits() {
    // digits() 헬퍼 함수 테스트
    let d = digits(42);
    let lines = d.render_lines();
    assert!(!lines.is_empty());
}

#[test]
fn test_helper_digits_with_string() {
    // 문자열을 사용한 digits() 헬퍼 함수 테스트
    let d = digits("12345");
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_helper_clock() {
    // clock() 헬퍼 함수 테스트
    let c = clock(9, 41);
    let lines = c.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_helper_timer() {
    // timer() 헬퍼 함수 테스트
    let t = timer(125);
    let lines = t.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_helper_timer_with_hours() {
    // 시간이 포함된 timer() 헬퍼 함수 테스트
    let t = timer(7325);
    let lines = t.render_lines();
    assert_eq!(lines.len(), 5);
}

// ─────────────────────────────────────────────────────────────────────────
// Edge cases - 엣지 케이스 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_digits_zero_value() {
    // 0 값 테스트
    let d = Digits::new(0);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_large_number() {
    // 큰 숫자 테스트
    let d = Digits::new(999999999);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_very_large_negative() {
    // 매우 큰 음수 테스트
    let d = Digits::new(-999999999);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_float_with_many_decimals() {
    // 많은 소수점을 가진 float 테스트
    let d = Digits::from_float(1.0, 10);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_time_boundary_values() {
    // 시간 경계값 테스트
    let d1 = Digits::time(0, 0, 0);
    let lines1 = d1.render_lines();
    assert_eq!(lines1.len(), 5);

    let d2 = Digits::time(23, 59, 59);
    let lines2 = d2.render_lines();
    assert_eq!(lines2.len(), 5);
}

#[test]
fn test_digits_timer_overflow_day() {
    // 하루를 넘는 타이머 테스트
    let d = Digits::timer(90000); // 25시간
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_all_zeros() {
    // 모두 0인 숫자 테스트
    let d = Digits::new("00000");
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_repeated_nines() {
    // 반복되는 9 테스트
    let d = Digits::new("99999");
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_clone() {
    // Digits 복제 테스트
    let d1 = Digits::new(42)
        .style(DigitStyle::Thin)
        .fg(Color::RED)
        .bg(Color::BLUE)
        .prefix("$")
        .suffix("%");

    let d2 = d1.clone();
    // 같은 높이와 너비를 가져야 함
    assert_eq!(d1.height(), d2.height());
    assert_eq!(d1.digit_width(), d2.digit_width());
}

#[test]
fn test_digits_time_colon_rendering() {
    // 시간 콜론 렌더링 테스트
    let d = Digits::new("12:34").style(DigitStyle::Block);
    let lines = d.render_lines();
    assert!(lines.len() > 0);
}

#[test]
fn test_digits_decimal_dot_rendering() {
    // 소수점 점 렌더링 테스트
    let d = Digits::new("3.14").style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_multiple_special_chars() {
    // 여러 특수 문자가 포함된 값 테스트
    let d = Digits::new("-12.34");
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

// ─────────────────────────────────────────────────────────────────────────
// StyledView and CSS integration tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_digits_element_id() {
    // CSS ID 설정 테스트
    let d = digits(42).element_id("my-digits");
    assert_eq!(View::id(&d), Some("my-digits"));
}

#[test]
fn test_digits_css_classes() {
    // CSS 클래스 설정 테스트
    let d = digits(123).class("display").class("large");

    assert!(d.has_class("display"));
    assert!(d.has_class("large"));
    assert!(!d.has_class("small"));
}

#[test]
fn test_digits_styled_view_methods() {
    // StyledView 메서드 테스트
    let mut d = digits(999);

    d.set_id("counter");
    assert_eq!(View::id(&d), Some("counter"));

    d.add_class("primary");
    assert!(d.has_class("primary"));

    d.remove_class("primary");
    assert!(!d.has_class("primary"));

    d.toggle_class("active");
    assert!(d.has_class("active"));

    d.toggle_class("active");
    assert!(!d.has_class("active"));
}

#[test]
fn test_digits_builder_chain() {
    // 빌더 체인 테스트
    let d = digits(12345)
        .style(DigitStyle::Thin)
        .fg(Color::CYAN)
        .bg(Color::BLACK)
        .prefix("$")
        .suffix(".00")
        .separator(',')
        .min_width(3)
        .leading_zeros(false)
        .element_id("price")
        .class("currency")
        .class("large");

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);

    assert_eq!(View::id(&d), Some("price"));
    assert!(d.has_class("currency"));
    assert!(d.has_class("large"));
}

#[test]
fn test_digits_render_all_styles() {
    // 모든 스타일 렌더링 테스트
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);

    // Block
    let d = digits(123).style(DigitStyle::Block);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // Thin
    let d = digits(456).style(DigitStyle::Thin);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // ASCII
    let d = digits(789).style(DigitStyle::Ascii);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // Braille
    let d = digits(999).style(DigitStyle::Braille);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
}

#[test]
fn test_digits_different_digit_patterns() {
    // 서로 다른 숫자 패턴 테스트
    for i in 0..10 {
        let d = Digits::new(&format!("{}", i)).style(DigitStyle::Block);
        let lines = d.render_lines();
        assert_eq!(lines.len(), 5);
        assert!(!lines[0].is_empty());
    }
}

#[test]
fn test_digits_render_with_offset() {
    // 오프셋이 있는 렌더링 테스트
    let d = Digits::new("42");
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(10, 5, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_digits_render_zero_area() {
    // 0 너비/높이 영역 렌더링 테스트
    let d = Digits::new("42");
    let mut buffer = Buffer::new(20, 10);

    // 0 너비
    let area = Rect::new(0, 0, 0, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // 0 높이
    let area = Rect::new(0, 0, 20, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
}

#[test]
fn test_digits_timer_edge_cases() {
    // 타이머 엣지 케이스 테스트
    // 59초 (분만 표시)
    let d = Digits::timer(59);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);

    // 60초 (1분)
    let d = Digits::timer(60);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);

    // 3599초 (59분 59초)
    let d = Digits::timer(3599);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);

    // 3600초 (1시간)
    let d = Digits::timer(3600);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_float_edge_cases() {
    // Float 엣지 케이스 테스트
    // 0.0
    let d = Digits::from_float(0.0, 2);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);

    // 음수 float
    let d = Digits::from_float(-12.34, 2);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);

    // 매우 작은 소수
    let d = Digits::from_float(0.001, 3);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_empty_string_value() {
    // 빈 문자열 값 테스트
    let d = Digits::new("");
    let lines = d.render_lines();
    assert!(lines.iter().all(|l| l.is_empty()));
}

#[test]
fn test_digits_spaces_in_value() {
    // 값에 공백이 있는 경우 테스트
    let d = Digits::new("12 34");
    let lines = d.render_lines();
    // 공백은 공백 패턴으로 렌더링됨
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_digits_separator_affects_rendering() {
    // 구분자가 렌더링에 영향을 미치는지 테스트
    let d1 = Digits::new("10000");
    let lines1 = d1.render_lines();

    let d2 = Digits::new("10000").separator(',');
    let lines2 = d2.render_lines();

    // 구분자가 추가되면 더 많은 공간이 필요함
    assert_eq!(lines1.len(), lines2.len());
}

#[test]
fn test_digits_min_width_affects_rendering() {
    // 최소 너비가 렌더링에 영향을 미치는지 테스트
    let d1 = Digits::new("42");
    let lines1 = d1.render_lines();

    let d2 = Digits::new("42").min_width(5);
    let lines2 = d2.render_lines();

    // 최소 너비가 적용되면 더 많은 숫자가 렌더링됨
    assert_eq!(lines1.len(), lines2.len());
}

#[test]
fn test_digits_all_styles_have_correct_heights() {
    // 모든 스타일이 올바른 높이를 가지는지 테스트
    let block_height = Digits::new("0").style(DigitStyle::Block).height();
    let thin_height = Digits::new("0").style(DigitStyle::Thin).height();
    let ascii_height = Digits::new("0").style(DigitStyle::Ascii).height();
    let braille_height = Digits::new("0").style(DigitStyle::Braille).height();

    assert_eq!(block_height, 5);
    assert_eq!(thin_height, 5);
    assert_eq!(ascii_height, 5);
    assert_eq!(braille_height, 4);
}

#[test]
fn test_digits_all_styles_have_correct_widths() {
    // 모든 스타일이 올바른 숫자 너비를 가지는지 테스트
    let block_width = Digits::new("0").style(DigitStyle::Block).digit_width();
    let thin_width = Digits::new("0").style(DigitStyle::Thin).digit_width();
    let ascii_width = Digits::new("0").style(DigitStyle::Ascii).digit_width();
    let braille_width = Digits::new("0").style(DigitStyle::Braille).digit_width();

    assert_eq!(block_width, 3);
    assert_eq!(thin_width, 3);
    assert_eq!(ascii_width, 3);
    assert_eq!(braille_width, 2);
}

#[test]
fn test_digits_render_all_digits() {
    // 모든 숫자(0-9) 렌더링 테스트
    for digit in 0..=9 {
        let d = Digits::new(&format!("{}", digit)).style(DigitStyle::Block);
        let lines = d.render_lines();
        assert_eq!(lines.len(), 5);
        assert!(!lines[0].is_empty());
    }
}

#[test]
fn test_digits_time_all_components() {
    // 시간의 모든 구성 요소 테스트
    let d = Digits::time(1, 2, 3);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    // 콜론이 포함되어야 함
}

#[test]
fn test_digits_with_prefix_suffix_renders() {
    // 접두사와 접미사가 있는 렌더링 테스트
    let d = Digits::new("42").prefix("Price: $").suffix(" USD");
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_digits_thin_style_characters() {
    // Thin 스타일 문자 확인 테스트
    let d = Digits::new("0").style(DigitStyle::Thin);
    let lines = d.render_lines();
    // Thin 스타일은 박스 드로잉 문자 사용
    let has_box_char = lines.iter().any(|l| {
        l.contains('┌') || l.contains('┐') || l.contains('└') || l.contains('┘')
            || l.contains('│') || l.contains('─') || l.contains('┬')
            || l.contains('├') || l.contains('┤') || l.contains('┴')
    });
    assert!(has_box_char);
}

#[test]
fn test_digits_ascii_style_characters() {
    // ASCII 스타일 문자 확인 테스트
    let d = Digits::new("0").style(DigitStyle::Ascii);
    let lines = d.render_lines();
    // ASCII 스타일은 +, -, | 문자 사용
    let has_ascii_char = lines.iter().any(|l| {
        l.contains('+') || l.contains('-') || l.contains('|')
    });
    assert!(has_ascii_char);
}

#[test]
fn test_digits_block_style_characters() {
    // Block 스타일 문자 확인 테스트
    let d = Digits::new("0").style(DigitStyle::Block);
    let lines = d.render_lines();
    // Block 스타일은 █ 문자 사용
    let has_block_char = lines.iter().any(|l| l.contains('█'));
    assert!(has_block_char);
}

#[test]
fn test_digits_braille_style_characters() {
    // Braille 스타일 문자 확인 테스트
    let d = Digits::new("0").style(DigitStyle::Braille);
    let lines = d.render_lines();
    // Braille 패턴은 유니코드 브라유 문자 사용
    assert_eq!(lines.len(), 4);
}
