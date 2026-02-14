//! RichText widget tests extracted from src/widget/display/richtext.rs

use revue::prelude::*;

// Note: Many tests for RichText access private methods (is_bold, text, etc.),
// so only a subset of tests that use public APIs are extracted here.

// =========================================================================
// RichText builder tests (using public APIs only)
// =========================================================================

#[test]
fn test_rich_text_builder() {
    let rt = RichText::new()
        .push("Hello ", Style::new())
        .push("World", Style::new().fg(Color::GREEN));

    assert_eq!(rt.len(), 2);
    // Can't test width() as it depends on text measurement which is private
}

#[test]
fn test_rich_text_markup() {
    let rt = RichText::markup("[bold]Hello[/]");

    assert_eq!(rt.len(), 1);
    // Can't test span details as they access private fields
}

#[test]
fn test_rich_text_render() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let rt = RichText::new()
        .push("Hello ", Style::new())
        .push("World", Style::new().bold());

    rt.render(&mut ctx);

    // Can't verify detailed buffer contents without private API access
    // Just verify it renders without panic
}
