//! A hex color with non-ASCII characters or a sign is no color: CSS
//! ignores it, and ColorPicker::set_hex rejects it instead of panicking on a
//! char boundary (#863)

use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{ColorPicker, Text};

fn css_color(value: &str) -> Option<Color> {
    let css = format!(".a {{ color: {value}; }}");
    let mut h = PipelineHarness::with_css(&css, 10, 1);
    h.draw(&Text::new("x").element_id("a").class("a"));
    h.computed_color("a")
}

#[test]
fn css_ignores_a_short_hex_with_non_ascii_characters() {
    assert_eq!(css_color("#€"), None); // 3 bytes, 1 char
    assert_eq!(css_color("#é1"), None);
}

#[test]
fn css_ignores_a_hex_with_a_sign() {
    assert_eq!(css_color("#+fffff"), None);
}

#[test]
fn css_still_reads_hex_colors() {
    assert_eq!(css_color("#f00"), Some(Color::rgb(255, 0, 0)));
    assert_eq!(css_color("#00ff00"), Some(Color::rgb(0, 255, 0)));
}

#[test]
fn color_picker_rejects_non_ascii_hex() {
    let mut picker = ColorPicker::new();
    let before = picker.get_color();
    assert!(!picker.set_hex("1é345"));
    assert!(!picker.set_hex("+12345"));
    assert_eq!(picker.get_color(), before);
    assert!(picker.set_hex("#123456"));
}
