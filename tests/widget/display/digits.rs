//! Digits widget tests extracted from src/widget/display/digits.rs

use revue::widget::{clock, digits, DigitStyle, Digits};

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

    let t = Digits::timer(90);
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
fn test_render_lines_unknown_chars_use_space() {
    let d = Digits::new("abc").style(DigitStyle::Block);
    let lines = d.render_lines();
    assert_eq!(lines.len(), 5);
    // Unknown chars should render as spaces
    assert!(lines[0].contains("   "));
}

// =========================================================================
// Builder setter tests (public API)
// =========================================================================

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
    assert_eq!(DigitStyle::default(), DigitStyle::Block);
}

#[test]
fn test_leading_zeros_changes_output() {
    let with = Digits::new(42).min_width(5).leading_zeros(true);
    let without = Digits::new(42).min_width(5).leading_zeros(false);
    assert_ne!(with.format_value(), without.format_value());
    assert_eq!(with.format_value(), "00042");
    assert_eq!(without.format_value(), "   42");
    // Zero padding stays the default
    assert_eq!(Digits::new(42).min_width(5).format_value(), "00042");
    // Blank padding goes before the sign and the separators
    assert_eq!(
        Digits::new(-42)
            .min_width(5)
            .leading_zeros(false)
            .format_value(),
        "  -42"
    );
    assert_eq!(
        Digits::new(1234)
            .min_width(7)
            .separator(',')
            .leading_zeros(false)
            .format_value(),
        "  1,234"
    );
}

#[test]
fn test_leading_zeros_off_renders_blank_columns() {
    let lines = Digits::new(7)
        .min_width(3)
        .leading_zeros(false)
        .render_lines();
    let zeros = Digits::new(7).min_width(3).render_lines();
    assert_ne!(lines, zeros);
    // The two padding digits are blank, only the 7 draws
    assert!(lines[0].trim_start().starts_with("███"));
    assert!(zeros[0].starts_with("███"));
}
