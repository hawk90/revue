//! Markdown widget tests

use super::*;
use crate::layout::Rect;
use crate::render::Buffer;

#[test]
fn test_markdown_new() {
    let md = Markdown::new("# Hello");
    assert_eq!(md.source(), "# Hello");
}

#[test]
fn test_markdown_heading() {
    let md = Markdown::new("# Heading 1");
    assert!(md.line_count() > 0);
}

#[test]
fn test_markdown_paragraph() {
    let md = Markdown::new("This is a paragraph.\n\nAnother paragraph.");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_bold() {
    let md = Markdown::new("This is **bold** text.");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_italic() {
    let md = Markdown::new("This is *italic* text.");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_code() {
    let md = Markdown::new("Inline `code` here.");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_list() {
    let md = Markdown::new("- Item 1\n- Item 2\n- Item 3");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_ordered_list() {
    let md = Markdown::new("1. First\n2. Second\n3. Third");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_render() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let md = Markdown::new("# Test\n\nHello world.");
    md.render(&mut ctx);

    // Check that something was rendered
    let mut found_hash = false;
    for x in 0..10 {
        if buffer.get(x, 0).unwrap().symbol == '#' {
            found_hash = true;
            break;
        }
    }
    assert!(found_hash);
}

#[test]
fn test_markdown_helper() {
    let md = markdown("Test content");
    assert_eq!(md.source(), "Test content");
}

#[test]
fn test_markdown_quote() {
    let md = Markdown::new("> This is a quote");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_link() {
    let md = Markdown::new("[Link](https://example.com)");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_rule() {
    let md = Markdown::new("Above\n\n---\n\nBelow");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_footnote_reference() {
    let md = Markdown::new("Text with footnote[^1]\n\n[^1]: This is the footnote.");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_multiple_footnotes() {
    let md =
        Markdown::new("First[^a] and second[^b].\n\n[^a]: First footnote.\n[^b]: Second footnote.");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_footnote_section_rendered() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let md = Markdown::new("Text[^note]\n\n[^note]: My footnote.");
    md.render(&mut ctx);

    let mut found_separator = false;
    for y in 0..24 {
        if buffer.get(0, y).unwrap().symbol == '─' {
            found_separator = true;
            break;
        }
    }
    assert!(found_separator);
}

#[test]
fn test_admonition_type_from_marker() {
    assert_eq!(
        AdmonitionType::from_marker("[!NOTE]"),
        Some(AdmonitionType::Note)
    );
    assert_eq!(
        AdmonitionType::from_marker("[!TIP]"),
        Some(AdmonitionType::Tip)
    );
    assert_eq!(
        AdmonitionType::from_marker("[!IMPORTANT]"),
        Some(AdmonitionType::Important)
    );
    assert_eq!(
        AdmonitionType::from_marker("[!WARNING]"),
        Some(AdmonitionType::Warning)
    );
    assert_eq!(
        AdmonitionType::from_marker("[!CAUTION]"),
        Some(AdmonitionType::Caution)
    );
    assert_eq!(
        AdmonitionType::from_marker("[!note]"),
        Some(AdmonitionType::Note)
    );
    assert_eq!(AdmonitionType::from_marker("NOTE"), None);
    assert_eq!(AdmonitionType::from_marker("[NOTE]"), None);
    assert_eq!(AdmonitionType::from_marker("[!UNKNOWN]"), None);
}

#[test]
fn test_admonition_icon() {
    assert_eq!(AdmonitionType::Note.icon(), "ℹ️ ");
    assert_eq!(AdmonitionType::Tip.icon(), "💡");
    assert_eq!(AdmonitionType::Important.icon(), "❗");
    assert_eq!(AdmonitionType::Warning.icon(), "⚠️ ");
    assert_eq!(AdmonitionType::Caution.icon(), "🔴");
}

#[test]
fn test_admonition_label() {
    assert_eq!(AdmonitionType::Note.label(), "Note");
    assert_eq!(AdmonitionType::Tip.label(), "Tip");
    assert_eq!(AdmonitionType::Important.label(), "Important");
    assert_eq!(AdmonitionType::Warning.label(), "Warning");
    assert_eq!(AdmonitionType::Caution.label(), "Caution");
}

#[test]
fn test_admonition_note() {
    let md = Markdown::new("> [!NOTE]\n> This is a note.");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_admonition_warning() {
    let md = Markdown::new("> [!WARNING]\n> Be careful!");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_admonition_all_types() {
    for (marker, label) in [
        ("[!NOTE]", "Note"),
        ("[!TIP]", "Tip"),
        ("[!IMPORTANT]", "Important"),
        ("[!WARNING]", "Warning"),
        ("[!CAUTION]", "Caution"),
    ] {
        let source = format!("> {}\n> Content for {}.", marker, label);
        let md = Markdown::new(source);
        assert!(
            md.line_count() >= 2,
            "Admonition {} should render at least 2 lines",
            label
        );
    }
}

#[test]
fn test_regular_blockquote_not_admonition() {
    let md = Markdown::new("> This is a regular quote\n> Not an admonition");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_admonition_render() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let md = Markdown::new("> [!NOTE]\n> Important information.");
    md.render(&mut ctx);

    let mut found_bar = false;
    for y in 0..24 {
        if buffer.get(0, y).unwrap().symbol == '│' {
            found_bar = true;
            break;
        }
    }
    assert!(found_bar);
}

#[test]
fn test_admonition_multiline_content() {
    let md = Markdown::new("> [!WARNING]\n> Line 1\n> Line 2\n> Line 3");
    assert!(
        md.line_count() >= 4,
        "Multi-line admonition should have multiple lines"
    );
}

#[test]
fn test_admonition_color() {
    assert_ne!(AdmonitionType::Note.color(), Color::BLACK);
    assert_ne!(AdmonitionType::Tip.color(), Color::BLACK);
    assert_ne!(AdmonitionType::Important.color(), Color::BLACK);
    assert_ne!(AdmonitionType::Warning.color(), Color::BLACK);
    assert_ne!(AdmonitionType::Caution.color(), Color::BLACK);
}

#[test]
fn test_blockquote_multiline() {
    let md = Markdown::new("> First line\n> Second line\n> Third line");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_footnote_with_multiline_content() {
    let md = Markdown::new("Text[^1]\n\n[^1]: This is a longer footnote with multiple words.");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_footnote_reference_before_definition() {
    let md = Markdown::new("See[^note] for details.\n\n[^note]: The footnote content here.");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_admonition_with_empty_content() {
    let md = Markdown::new("> [!TIP]");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_mixed_content_with_admonition() {
    let md = Markdown::new("Before.\n\n> [!IMPORTANT]\n> Content\n\nAfter.");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_footnote_number_ordering() {
    let md = Markdown::new("A[^z] B[^a]\n\n[^a]: Alpha\n[^z]: Zeta");
    assert!(md.line_count() >= 3);
}
