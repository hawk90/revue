//! RichText widget tests extracted from src/widget/display/richtext.rs

use revue::prelude::*;

// =========================================================================
// Style tests
// =========================================================================

#[test]
fn test_style_builder() {
    let s = Style::new().bold().fg(Color::RED);
    assert!(s.is_bold());
    assert_eq!(s.fg(), Some(Color::RED));
}

#[test]
fn test_style_presets() {
    assert_eq!(Style::red().fg(), Some(Color::RED));
    assert_eq!(Style::green().fg(), Some(Color::GREEN));
    assert_eq!(Style::blue().fg(), Some(Color::BLUE));
}

// =========================================================================
// Span tests
// =========================================================================

#[test]
fn test_span() {
    let span = Span::new("Hello").style(Style::new().bold());
    assert_eq!(span.text(), "Hello");
    assert!(span.style().is_bold());
}

#[test]
fn test_span_link() {
    let span = Span::link("Click", "https://example.com");
    assert_eq!(span.text(), "Click");
    assert!(span.link().is_some());
    assert!(span.style().is_underline());
}

// =========================================================================
// RichText builder tests
// =========================================================================

#[test]
fn test_rich_text_builder() {
    let rt = RichText::new()
        .push("Hello ", Style::new().bold())
        .push("World", Style::green());

    assert_eq!(rt.len(), 2);
    assert_eq!(rt.width(), 11);
}

#[test]
fn test_rich_text_markup_bold() {
    let rt = RichText::markup("[bold]Hello[/] World");
    assert_eq!(rt.len(), 2);
    assert!(rt.spans()[0].style().is_bold());
    assert!(!rt.spans()[1].style().is_bold());
}

#[test]
fn test_rich_text_markup_color() {
    let rt = RichText::markup("[red]Error[/]");
    assert_eq!(rt.spans()[0].style().fg(), Some(Color::RED));
}

#[test]
fn test_rich_text_markup_combined() {
    let rt = RichText::markup("[bold red]Important[/]");
    assert!(rt.spans()[0].style().is_bold());
    assert_eq!(rt.spans()[0].style().fg(), Some(Color::RED));
}

#[test]
fn test_rich_text_markup_link() {
    let rt = RichText::markup("[link=https://example.com]Click here[/]");
    assert!(rt.spans()[0].link().is_some());
    assert_eq!(
        rt.spans()[0].link().as_ref().unwrap(),
        "https://example.com"
    );
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

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(6, 0).unwrap().symbol, 'W');
    assert!(buffer.get(6, 0).unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_rich_text_render_with_hyperlink() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let rt = RichText::new().push_link("Click", "https://example.com");

    rt.render(&mut ctx);

    // Check hyperlink was registered
    assert!(buffer.get(0, 0).unwrap().hyperlink_id.is_some());
    assert_eq!(buffer.get_hyperlink(0), Some("https://example.com"));
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_helper_functions() {
    let rt = rich_text();
    assert!(rt.is_empty());

    let rt = markup("[bold]test[/]");
    assert_eq!(rt.len(), 1);

    let s = span("test");
    assert_eq!(s.text(), "test");

    let st = style();
    assert!(!st.is_bold());
}
