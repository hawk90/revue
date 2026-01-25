//! Tests for TextFormat

use super::*;

#[test]
fn test_text_format_default() {
    let fmt = TextFormat::default();
    assert!(!fmt.bold);
    assert!(!fmt.italic);
    assert!(!fmt.underline);
    assert!(!fmt.strikethrough);
    assert!(!fmt.code);
}

#[test]
fn test_text_format_new() {
    let fmt = TextFormat::new();
    assert!(!fmt.bold);
    assert!(!fmt.italic);
}

#[test]
fn test_text_format_toggle_bold() {
    let fmt = TextFormat::new().toggle_bold();
    assert!(fmt.bold);
    let fmt = fmt.toggle_bold();
    assert!(!fmt.bold);
}

#[test]
fn test_text_format_toggle_italic() {
    let fmt = TextFormat::new().toggle_italic();
    assert!(fmt.italic);
    let fmt = fmt.toggle_italic();
    assert!(!fmt.italic);
}

#[test]
fn test_text_format_toggle_underline() {
    let fmt = TextFormat::new().toggle_underline();
    assert!(fmt.underline);
}

#[test]
fn test_text_format_toggle_strikethrough() {
    let fmt = TextFormat::new().toggle_strikethrough();
    assert!(fmt.strikethrough);
}

#[test]
fn test_text_format_toggle_code() {
    let fmt = TextFormat::new().toggle_code();
    assert!(fmt.code);
}

#[test]
fn test_text_format_multiple_toggles() {
    let fmt = TextFormat::new()
        .toggle_bold()
        .toggle_italic()
        .toggle_code();
    assert!(fmt.bold);
    assert!(fmt.italic);
    assert!(fmt.code);
    assert!(!fmt.underline);
    assert!(!fmt.strikethrough);
}
