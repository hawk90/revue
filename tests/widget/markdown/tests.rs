//! Public API tests for Markdown widget
//!
//! These tests use only public API of Markdown widget:
//! - `Markdown::new()` - constructor
//! - `md.source()` - get source text
//! - `md.line_count()` - get parsed line count
//! - `md.toc()` - get table of contents
//! - `md.render()` - render to buffer
//! - `markdown()` - helper function

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::markdown::{Markdown, AdmonitionType};
use revue::widget::traits::{RenderContext, View};

// =========================================================================
// Constructor Tests
// =========================================================================

#[test]
fn test_markdown_new_creates_widget() {
    let md = Markdown::new("# Hello");
    assert_eq!(md.source(), "# Hello");
}

#[test]
fn test_markdown_helper() {
    let md = markdown("Test content");
    assert_eq!(md.source(), "Test content");
}

// =========================================================================
// Parsing Tests - Headings
// =========================================================================

#[test]
fn test_markdown_heading() {
    let md = Markdown::new("# Heading 1");
    assert!(md.line_count() > 0);
}

#[test]
fn test_markdown_multiple_headings() {
    let md = Markdown::new(
        "# Title 1
## Title 2
### Title 3",
    );
    assert!(md.line_count() >= 3);
}

// =========================================================================
// Parsing Tests - Paragraphs
// =========================================================================

#[test]
fn test_markdown_paragraph() {
    let md = Markdown::new(
        "This is a paragraph.

Another paragraph.",
    );
    assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_paragraph_with_multiple_lines() {
    let md = Markdown::new("First line.\nSecond line.");
    assert!(md.line_count() >= 2);
}

// =========================================================================
// Parsing Tests - Text Formatting
// =========================================================================

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
fn test_markdown_bold_and_italic() {
    let md = Markdown::new("This is ***bold and italic*** text.");
    assert!(md.line_count() >= 1);
}

// =========================================================================
// Parsing Tests - Lists
// =========================================================================

#[test]
fn test_markdown_list() {
    let md = Markdown::new(
        "- Item 1
- Item 2
- Item 3",
    );
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_ordered_list() {
    let md = Markdown::new(
        "1. First
2. Second
3. Third",
    );
    assert!(md.line_count() >= 3);
}

// =========================================================================
// Parsing Tests - Blockquotes
// =========================================================================

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
    let md = Markdown::new(
        "Above
---

Below",
        );
    assert!(md.line_count() >= 3);
}

// =========================================================================
// Parsing Tests - Footnotes
// =========================================================================

#[test]
fn test_markdown_footnote_reference() {
    let md = Markdown::new(
        "Text with footnote[^1]

[^1]: This is a footnote.",
        );
    assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_multiple_footnotes() {
    let md = Markdown::new(
        "First[^a] and second[^b].

[^a]: First footnote.
[^b]: Second footnote.",
        );
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_footnote_with_multiline_content() {
    let md = Markdown::new(
        "Text[^note]

[^note]: My footnote.",
        );
        assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_footnote_reference_before_definition() {
    let md = Markdown::new(
        "See[^note] for details.

[^note]: The footnote content here.",
        );
        assert!(md.line_count() >= 2);
}

// =========================================================================
// Parsing Tests - Admonitions
// =========================================================================

#[test]
fn test_markdown_admonition_type_from_marker() {
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
        assert_eq!(AdmonitionType::from_marker("[!UNKNOWN]"), None);
    assert_eq!(AdmonitionType::from_marker("[!NOTE]"), None);
        assert_eq!(AdmonitionType::from_marker("NOTE"), None);
        assert_eq!(AdmonitionType::from_marker("[!TIP]"), None);
    }

#[test]
fn test_markdown_admonition_icon() {
    assert_eq!(AdmonitionType::Note.icon(), "ℹ️ ");
}

#[test]
fn test_markdown_admonition_label() {
    assert_eq!(AdmonitionType::Note.label(), "Note");
}

#[test]
fn test_markdown_admonition_warning() {
    assert_eq!(AdmonitionType::Warning.icon(), "⚠️ ");
}

#[test]
fn test_markdown_admonition_important() {
    assert_eq!(AdmonitionType::Important.icon(), "❗");
}

#[test]
fn test_markdown_admonition_caution() {
    assert_eq!(AdmonitionType::Caution.icon(), "🔴");
}

#[test]
fn test_markdown_admonition_color() {
    assert_ne!(AdmonitionType::Note.color(), Color::BLACK);
    assert_ne!(AdmonitionType::Tip.color(), Color::BLACK);
    assert_ne!(AdmonitionType::Important.color(), Color::BLACK);
    assert_ne!(AdmonitionType::Warning.color(), Color::BLACK);
    assert_ne!(AdmonitionType::Caution.color(), Color::BLACK);
    }

#[test]
fn test_markdown_admonition_all_types() {
    for (marker, label) in [
        ("[!NOTE]", "Note"),
        ("[!TIP]", "Tip"),
        ("[!IMPORTANT]", "Important"),
        ("[!WARNING]", "Warning"),
        ("[!CAUTION]", "Caution"),
    ] {
        let source = format!(
                "> {}
> Content for {}.",
                marker, label
            );
        let md = Markdown::new(source);
        assert!(
            md.line_count() >= 2,
            "Admonition {} should render at least 2 lines",
            label
        );
    }
}

#[test]
fn test_markdown_regular_blockquote_not_admonition() {
    let md = Markdown::new(
        "> This is a regular quote
> Not an admonition",
    );
    assert!(md.line_count() >= 1);
}

// =========================================================================
// Rendering Tests
// =========================================================================

#[test]
fn test_markdown_render() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let md = Markdown::new(
        "# Test

Hello world.",
    );
    md.render(&mut ctx);

    // Check that heading was rendered with bold modifier
    let mut found_bold = false;
    for x in 0..10 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'T' && cell.modifier.contains(crate::render::Modifier::BOLD) {
                found_bold = true;
                break;
            }
        }
    }
    assert!(found_bold);
}

#[test]
fn test_markdown_render_footnote_section() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let md = Markdown::new(
        "Text[^note]

[^note]: My footnote.",
        );
    md.render(&mut ctx);

    let mut found_bar = false;
    for y in 0..24 {
        if buffer.get(0, y).unwrap().symbol == '─' {
            found_bar = true;
            break;
            }
        }
    assert!(found_bar);
}

#[test]
fn test_markdown_render() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let md = Markdown::new("# Test");
    md.render(&mut ctx);

    // Just ensure render doesn't panic
}

// =========================================================================
// Table of Contents Tests
// =========================================================================

#[test]
fn test_markdown_toc_empty_for_no_headings() {
    let md = Markdown::new("Just some text without headings.");
    assert_eq!(md.toc().len(), 0);
}

#[test]
fn test_markdown_toc_contains_headings() {
    let md = Markdown::new(
        "# Title 1
## Title 2
### Title 3",
    );
    let toc = md.toc();
    assert_eq!(toc.len(), 3);
}

#[test]
fn test_markdown_toc_levels() {
    let toc = md.toc();
    assert!(toc[0].level, 1);
    assert!(toc[1].level, 2);
    assert_eq!(toc[2].level, 1);
}
