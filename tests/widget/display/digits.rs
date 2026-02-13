//! Digits widget tests extracted from src/widget/display/digits.rs

use revue::style::Color;
use revue::widget::display::digits::{digits, clock, timer, DigitStyle, Digits};

// =========================================================================
// Digits creation tests
// =========================================================================

#[test]
fn test_digits_new() {
    let d = Digits::new(42);
    // Public API available: format_value()
    assert_eq!(d.format_value(), "42");
}

#[test]
fn test_digits_from_float() {
    let d = Digits::from_float(12.345, 2);
    assert_eq!(d.format_value(), "12.35");
}

#[test]
fn test_digits_time() {
    let d = Digits::time(12, 34, 56);
    assert_eq!(d.format_value(), "12:34:56");
}

#[test]
fn test_digits_clock() {
    let d = Digits::clock(9, 30);
    assert_eq!(d.format_value(), "09:30");
}

#[test]
fn test_digits_timer() {
    let d = Digits::timer(3661); // 1h 1m 1s
    assert_eq!(d.format_value(), "01:01:01");

    let d2 = Digits::timer(65); // 1m 5s
    assert_eq!(d2.format_value(), "01:05");
}

#[test]
fn test_digits_style() {
    let d = Digits::new(0).style(DigitStyle::Thin);
    // Test that style affects height
    assert_eq!(d.height(), 5);
    let block = Digits::new(0).style(DigitStyle::Block);
    assert_eq!(block.height(), 5);
}

#[test]
fn test_digits_separator() {
    let d = Digits::new(1234567).separator(',');
    assert_eq!(d.format_value(), "1,234,567");
}

#[test]
fn test_digits_min_width() {
    let d = Digits::new(42).min_width(5);
    assert_eq!(d.format_value(), "00042");
}

#[test]
fn test_digits_height() {
    assert_eq!(Digits::new(0).style(DigitStyle::Block).height(), 5);
    assert_eq!(Digits::new(0).style(DigitStyle::Braille).height(), 4);
}

#[test]
fn test_digits_render_lines() {
    let d = Digits::new(1).style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    assert!(lines[0].contains("█"));
}

#[test]
fn test_digits_negative() {
    let d = Digits::new(-42).separator(',');
    let formatted = d.format_value();
    assert!(formatted.starts_with('-'));
}

#[test]
fn test_digits_decimal_separator() {
    let d = Digits::new("1234.56").separator(',');
    assert_eq!(d.format_value(), "1,234.56");
}

#[test]
fn test_helper_functions() {
    let d = digits(100);
    assert_eq!(d.format_value(), "100");

    let c = clock(12, 0);
    assert_eq!(c.format_value(), "12:00");

    let t = timer(90);
    assert_eq!(t.format_value(), "01:30");
}

// =========================================================================
// Thousands separator and format_value tests (public API)
// =========================================================================

#[test]
fn test_add_thousands_separator_small_number() {
    let d = Digits::new(123).separator(',');
    assert_eq!(d.format_value(), "123");
}

#[test]
fn test_thousands_separator_negative() {
    let d = Digits::new(-1234).separator(',');
    assert_eq!(d.format_value(), "-1,234");
}

#[test]
fn test_thousands_separator_large_negative() {
    let d = Digits::new(-1234567).separator(',');
    assert_eq!(d.format_value(), "-1,234,567");
}

#[test]
fn test_thousands_separator_with_decimal() {
    let d = Digits::new("1234.56").separator(',');
    assert_eq!(d.format_value(), "1,234.56");
}

#[test]
fn test_thousands_separator_negative_decimal() {
    let d = Digits::new("-1234.56").separator(',');
    assert_eq!(d.format_value(), "-1,234.56");
}

#[test]
fn test_thousands_separator_custom_char() {
    let d = Digits::new(1234567).separator('.');
    assert_eq!(d.format_value(), "1.234.567");

    let d = Digits::new(1234567).separator(' ');
    assert_eq!(d.format_value(), "1 234 567");
}

#[test]
fn test_thousands_separator_exact_thousands() {
    let d = Digits::new(1000).separator(',');
    assert_eq!(d.format_value(), "1,000");

    let d = Digits::new(1000000).separator(',');
    assert_eq!(d.format_value(), "1,000,000");
}

// =========================================================================
// format_value edge cases (public API)
// =========================================================================

#[test]
fn test_format_value_empty() {
    let d = Digits::new("");
    assert_eq!(d.format_value(), "");
}

#[test]
fn test_format_value_with_min_width_greater_than_length() {
    let d = Digits::new("42").min_width(6);
    assert_eq!(d.format_value(), "000042");
}

#[test]
fn test_format_value_with_min_width_equal_to_length() {
    let d = Digits::new("123456").min_width(6);
    assert_eq!(d.format_value(), "123456");
}

#[test]
fn test_format_value_with_min_width_less_than_length() {
    let d = Digits::new("123456").min_width(3);
    assert_eq!(d.format_value(), "123456");
}

#[test]
fn test_format_value_both_min_width_and_separator() {
    let d = Digits::new("123456").min_width(6).separator(',');
    assert_eq!(d.format_value(), "123,456");
}

#[test]
fn test_format_value_leading_zeros_with_separator() {
    let d = Digits::new("123").min_width(6).separator(',');
    // First pad with zeros, then add separator
    let result = d.format_value();
    assert_eq!(result, "000,123");
}

// =========================================================================
// render_lines with different styles (public API)
// =========================================================================

#[test]
fn test_render_lines_block_style() {
    let d = Digits::new("12").style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    // Each line should have patterns for both digits with spacing
    assert!(lines[0].len() > 0);
}

#[test]
fn test_render_lines_thin_style() {
    let d = Digits::new("05").style(DigitStyle::Thin);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    assert!(lines[0].contains('┌'));
}

#[test]
fn test_render_lines_ascii_style() {
    let d = Digits::new("56").style(DigitStyle::Ascii);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    assert!(lines[0].contains('+'));
}

#[test]
fn test_render_lines_braille_style() {
    let d = Digits::new("78").style(DigitStyle::Braille);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 4); // Braille is 4 rows high
    assert!(lines[0].contains('⣰'));
}

#[test]
fn test_render_lines_with_colon() {
    let d = Digits::new("1:23").style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_render_lines_with_dot() {
    let d = Digits::new("12.34").style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_render_lines_with_minus() {
    let d = Digits::new("-42").style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_render_lines_unknown_chars_use_space() {
    let d = Digits::new("abc").style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    // Unknown chars should render as spaces
    assert!(lines[0].contains("   "));
}

// =========================================================================
// digit_width tests (public API)
// =========================================================================

#[test]
fn test_digit_width_braille() {
    let d = Digits::new(0).style(DigitStyle::Braille);
    assert_eq!(d.digit_width(), 2);
}

#[test]
fn test_digit_width_non_braille() {
    let d_block = Digits::new(0).style(DigitStyle::Block);
    assert_eq!(d_block.digit_width(), 3);

    let d_thin = Digits::new(0).style(DigitStyle::Thin);
    assert_eq!(d_thin.digit_width(), 3);

    let d_ascii = Digits::new(0).style(DigitStyle::Ascii);
    assert_eq!(d_ascii.digit_width(), 3);
}

// =========================================================================
// Builder setter tests (public API)
// =========================================================================

#[test]
fn test_prefix_setter() {
    // Note: There's no public prefix() getter in current Digits implementation
    // This test demonstrates what a public getter would test
    let d = Digits::new(100).prefix("$");
    // Can't test the prefix value directly with public API
    // But we can test that render_lines works
    let lines = d.render_lines();
    assert!(!lines.is_empty());
}

#[test]
fn test_suffix_setter() {
    // Note: There's no public suffix() getter in current Digits implementation
    let d = Digits::new(100).suffix("%");
    let lines = d.render_lines();
    assert!(!lines.is_empty());
}

#[test]
fn test_leading_zeros_setter() {
    // Note: There's no public is_leading_zeros() getter in current Digits implementation
    let d = Digits::new(42).leading_zeros(true);
    // Test that leading zeros affect the output through format_value
    assert_eq!(d.format_value(), "42"); // No effect without min_width
    let d_with_width = d.min_width(5);
    assert_eq!(d_with_width.format_value(), "00042");
}

#[test]
fn test_from_int() {
    let d = Digits::from_int(-12345);
    assert_eq!(d.format_value(), "-12345");
}

#[test]
fn test_timer_zero_seconds() {
    let d = Digits::timer(0);
    assert_eq!(d.format_value(), "00:00");
}

#[test]
fn test_timer_exactly_one_hour() {
    let d = Digits::timer(3600);
    assert_eq!(d.format_value(), "01:00:00");
}

#[test]
fn test_digit_style_default() {
    let style = DigitStyle::default();
    assert_eq!(style, DigitStyle::Block);
}

// =========================================================================
// PRIVATE TESTS - MARKED WITH "# KEEP HERE" COMMENT
// These tests access private methods and should remain in source files
// =========================================================================

// Example of private test that would remain in source:
/*
#[test]
#[keep_here]
fn test_private_render_logic() {
    // This test accesses private methods
    // These should remain in the source file
}
*/