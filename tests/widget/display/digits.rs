//! Digits widget tests extracted from src/widget/display/digits.rs

use revue::style::Color;
use revue::widget::display::digits::{digits, clock, timer, DigitStyle, Digits};

// =========================================================================
// Digits creation tests
// =========================================================================

#[test]
fn test_digits_new() {
    let d = Digits::new(42);
    assert_eq!(d.value(), "42");
}

#[test]
fn test_digits_from_float() {
    let d = Digits::from_float(12.345, 2);
    assert_eq!(d.value(), "12.35");
}

#[test]
fn test_digits_time() {
    let d = Digits::time(12, 34, 56);
    assert_eq!(d.value(), "12:34:56");
}

#[test]
fn test_digits_clock() {
    let d = Digits::clock(9, 30);
    assert_eq!(d.value(), "09:30");
}

#[test]
fn test_digits_timer() {
    let d = Digits::timer(3661); // 1h 1m 1s
    assert_eq!(d.value(), "01:01:01");

    let d2 = Digits::timer(65); // 1m 5s
    assert_eq!(d2.value(), "01:05");
}

#[test]
fn test_digits_style() {
    let d = Digits::new(0).style(DigitStyle::Thin);
    assert_eq!(d.style(), DigitStyle::Thin);
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
    assert_eq!(d.value(), "100");

    let c = clock(12, 0);
    assert_eq!(c.value(), "12:00");

    let t = timer(90);
    assert_eq!(t.value(), "01:30");
}

// =========================================================================
// Pattern getter tests (private methods)
// =========================================================================

#[test]
fn test_get_block_pattern_digits() {
    let d = Digits::new(0).style(DigitStyle::Block);

    // Test digit 0
    let pattern = d.test_get_block_pattern('0');
    assert_eq!(pattern.len(), 5);
    assert_eq!(pattern[0], "███");
    assert_eq!(pattern[4], "███");

    // Test digit 8
    let pattern = d.test_get_block_pattern('8');
    assert_eq!(pattern[2], "███");
}

#[test]
fn test_get_block_pattern_special_chars() {
    let d = Digits::new(0).style(DigitStyle::Block);

    let colon = d.test_get_block_pattern(':');
    assert_eq!(colon[1], "█");
    assert_eq!(colon[3], "█");

    let dot = d.test_get_block_pattern('.');
    assert_eq!(dot[4], "█");

    let minus = d.test_get_block_pattern('-');
    assert_eq!(minus[2], "███");

    let space = d.test_get_block_pattern(' ');
    assert_eq!(space[0], "   ");
}

#[test]
fn test_get_thin_pattern_digits() {
    let d = Digits::new(0).style(DigitStyle::Thin);

    let pattern = d.test_get_thin_pattern('0');
    assert_eq!(pattern.len(), 5);
    assert_eq!(pattern[0], "┌─┐");

    let pattern = d.test_get_thin_pattern('1');
    assert_eq!(pattern[0], "  │");
}

#[test]
fn test_get_thin_pattern_special_chars() {
    let d = Digits::new(0).style(DigitStyle::Thin);

    let colon = d.test_get_thin_pattern(':');
    assert_eq!(colon.len(), 5);
    assert!(colon[1].contains('●'));

    let dot = d.test_get_thin_pattern('.');
    assert_eq!(dot[4], "●");

    let minus = d.test_get_thin_pattern('-');
    assert_eq!(minus[2], "───");
}

#[test]
fn test_get_ascii_pattern() {
    let d = Digits::new(0).style(DigitStyle::Ascii);

    let pattern = d.test_get_ascii_pattern('0');
    assert_eq!(pattern[0], "+-+");

    let pattern = d.test_get_ascii_pattern('8');
    assert_eq!(pattern[2], "+-+");

    let colon = d.test_get_ascii_pattern(':');
    assert_eq!(colon[1], "o");
}

#[test]
fn test_get_braille_pattern() {
    let d = Digits::new(0).style(DigitStyle::Braille);

    let pattern = d.test_get_braille_pattern('0');
    assert_eq!(pattern.len(), 4);

    let colon = d.test_get_braille_pattern(':');
    assert_eq!(colon.len(), 4);
    assert!(colon[1].contains('⠂'));

    let dot = d.test_get_braille_pattern('.');
    assert_eq!(dot[3], "⠂");

    let minus = d.test_get_braille_pattern('-');
    assert!(minus[1].contains('⠤'));
}

#[test]
fn test_get_char_pattern_delegates() {
    let d_block = Digits::new(0).style(DigitStyle::Block);
    let pattern = d_block.test_get_char_pattern('0');
    assert_eq!(pattern.len(), 5);

    let d_thin = Digits::new(0).style(DigitStyle::Thin);
    let pattern = d_thin.test_get_char_pattern('1');
    assert_eq!(pattern.len(), 5);

    let d_braille = Digits::new(0).style(DigitStyle::Braille);
    let pattern = d_braille.test_get_char_pattern('0');
    assert_eq!(pattern.len(), 4);
}

// =========================================================================
// add_thousands_separator edge cases
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
// format_value edge cases
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
// render_lines with different styles
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
// digit_width tests
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
// Builder setter tests
// =========================================================================

#[test]
fn test_prefix_setter() {
    let d = Digits::new(100).prefix("$");
    assert_eq!(d.prefix(), Some("$".to_string()));
}

#[test]
fn test_suffix_setter() {
    let d = Digits::new(100).suffix("%");
    assert_eq!(d.suffix(), Some("%".to_string()));
}

#[test]
fn test_leading_zeros_setter() {
    let d = Digits::new(42).leading_zeros(true);
    assert!(d.is_leading_zeros());
}

#[test]
fn test_from_int() {
    let d = Digits::from_int(-12345);
    assert_eq!(d.value(), "-12345");
}

#[test]
fn test_timer_zero_seconds() {
    let d = Digits::timer(0);
    assert_eq!(d.value(), "00:00");
}

#[test]
fn test_timer_exactly_one_hour() {
    let d = Digits::timer(3600);
    assert_eq!(d.value(), "01:00:00");
}

#[test]
fn test_digit_style_default() {
    let style = DigitStyle::default();
    assert_eq!(style, DigitStyle::Block);
}
