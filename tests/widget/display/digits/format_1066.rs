//! #1066: `min_width` gives one width with zeros or blanks, the label shows
//! the formatted value on the background, and a separator keeps the value

use revue::style::Color;
use revue::widget::Digits;

use super::view::{render, row};

#[test]
fn min_width_is_the_same_width_with_zeros_and_blanks() {
    let zeros = Digits::new(1234).min_width(7).separator(',');
    let blanks = zeros.clone().leading_zeros(false);
    assert_eq!(zeros.format_value(), "0,001,234");
    assert_eq!(blanks.format_value(), "    1,234");
}

#[test]
fn blank_padding_keeps_the_sign_by_the_number() {
    // The sign is one of the 7 characters: "-001234", grouped
    let d = Digits::new(-1234).min_width(7).separator(',');
    assert_eq!(d.format_value(), "-001,234");
    assert_eq!(d.leading_zeros(false).format_value(), "  -1,234");
}

#[test]
fn a_value_as_wide_as_min_width_is_not_padded() {
    for zeros in [true, false] {
        let d = Digits::new(1234567).min_width(7).separator(',');
        assert_eq!(d.leading_zeros(zeros).format_value(), "1,234,567");
    }
}

#[test]
fn min_width_counts_chars_not_bytes() {
    // Two chars, four bytes: padded to 4 chars
    assert_eq!(Digits::new("é1").min_width(4).format_value(), "00é1");
}

#[test]
fn the_label_shows_the_formatted_value() {
    let b = render(&Digits::new("1234.56").prefix("$").separator(','), 40, 6);
    assert_eq!(row(&b, 5).trim_end(), "$1,234.56");
    let b = render(&Digits::new(42).min_width(4).suffix("s"), 40, 6);
    assert_eq!(row(&b, 5).trim_end(), "0042s");
    let b = render(
        &Digits::new(42)
            .min_width(4)
            .leading_zeros(false)
            .prefix(">"),
        40,
        6,
    );
    assert_eq!(row(&b, 5).trim_end(), ">  42");
}

#[test]
fn measure_fits_the_formatted_label() {
    use revue::widget::traits::View;
    // A label wider than the glyphs: "$" + "0,001" + a long suffix
    let d = Digits::new(1)
        .min_width(4)
        .separator(',')
        .prefix("$")
        .suffix(" dollars total");
    let (w, _) = d.measure(200, 20).unwrap();
    assert_eq!(usize::from(w), "$0,001 dollars total".len());
}

#[test]
fn the_label_takes_the_background() {
    let b = render(&Digits::new(42).bg(Color::BLUE).suffix("%"), 10, 6);
    assert_eq!(b.get(0, 0).unwrap().bg, Some(Color::BLUE));
    assert_eq!(b.get(0, 5).unwrap().bg, Some(Color::BLUE));
    assert_eq!(b.get(2, 5).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn a_separator_keeps_a_value_that_is_not_a_number() {
    for value in ["1.2.3", "12:34:56", "1e5", "--1", "1.", ".5", "", "-"] {
        let d = Digits::new(value).separator(',');
        assert_eq!(d.format_value(), value, "{value:?}");
    }
}

#[test]
fn a_clock_with_a_separator_is_not_grouped() {
    let d = Digits::time(12, 34, 56).separator(',');
    assert_eq!(d.format_value(), "12:34:56");
}

#[test]
fn a_number_is_grouped_in_its_integer_part_only() {
    let d = Digits::new("-1234567.891").separator(',');
    assert_eq!(d.format_value(), "-1,234,567.891");
}
