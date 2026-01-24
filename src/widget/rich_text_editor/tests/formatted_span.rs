//! Tests for FormattedSpan

use super::*;

#[test]
fn test_formatted_span_new() {
    let span = FormattedSpan::new("Hello");
    assert_eq!(span.text, "Hello");
    assert!(!span.format.bold);
}

#[test]
fn test_formatted_span_with_format() {
    let format = TextFormat::new().toggle_bold();
    let span = FormattedSpan::new("Bold text").with_format(format);
    assert!(span.format.bold);
    assert_eq!(span.text, "Bold text");
}
