//! Public API tests for Markdown widget
//!
//! These tests use only the public API of the Markdown widget:
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
fn test_markdown_helper_function() {
    let md = revue::widget::markdown("Test content");
    assert_eq!(md.source(), "Test content");
}

// =========================================================================
// Parsing Tests - Headings
// =========================================================================

#[test]
fn test_markdown_heading_parsed() {
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
    let md = Markdown::new("First line.\nSecond line.\nThird line.");
    assert!(md.line_count() >= 3);
}

// =========================================================================
// Parsing Tests - Text Formatting
// =========================================================================

#[test]
fn test_markdown_bold_text() {
    let md = Markdown::new("This is **bold** text.");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_italic_text() {
    let md = Markdown::new("This is *italic* text.");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_bold_and_italic() {
    let md = Markdown::new("This is ***bold and italic*** text.");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_inline_code() {
    let md = Markdown::new("Inline `code` here.");
    assert!(md.line_count() >= 1);
}

// =========================================================================
// Parsing Tests - Lists
// =========================================================================

#[test]
fn test_markdown_unordered_list() {
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
fn test_markdown_blockquote() {
    let md = Markdown::new("> This is a quote");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_multiline_blockquote() {
    let md = Markdown::new(
        "> First line
> Second line
> Third line",
    );
    assert!(md.line_count() >= 1);
}

// =========================================================================
// Parsing Tests - Links
// =========================================================================

#[test]
fn test_markdown_link() {
    let md = Markdown::new("[Link](https://example.com)");
    assert!(md.line_count() >= 1);
}

// =========================================================================
// Parsing Tests - Horizontal Rules
// =========================================================================

#[test]
fn test_markdown_horizontal_rule() {
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

[^1]: This is the footnote.",
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
        "Text[^1]

[^1]: This is a longer footnote with multiple words.",
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

#[test]
fn test_markdown_footnote_number_ordering() {
    let md = Markdown::new(
        "A[^z] B[^a]

[^a]: Alpha
[^z]: Zeta",
    );
    assert!(md.line_count() >= 3);
}

// =========================================================================
// Parsing Tests - Admonitions
// =========================================================================

#[test]
fn test_markdown_admonition_note() {
    let md = Markdown::new(
        "> [!NOTE]
> This is a note.",
    );
    assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_admonition_warning() {
    let md = Markdown::new(
        "> [!WARNING]
> Be careful!",
    );
    assert!(md.line_count() >= 2);
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

#[test]
fn test_markdown_admonition_multiline_content() {
    let md = Markdown::new(
        "> [!WARNING]
> Line 1
> Line 2
> Line 3",
    );
    assert!(
        md.line_count() >= 4,
        "Multi-line admonition should have multiple lines"
    );
}

#[test]
fn test_markdown_admonition_with_empty_content() {
    let md = Markdown::new("> [!TIP]");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_mixed_content_with_admonition() {
    let md = Markdown::new(
        "Before.

> [!IMPORTANT]
> Content

After.",
    );
    assert!(md.line_count() >= 3);
}

// =========================================================================
// Rendering Tests
// =========================================================================

#[test]
fn test_markdown_render_heading_bold() {
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
            if cell.symbol == 'T' && cell.modifier.contains(revue::render::Modifier::BOLD) {
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
fn test_markdown_render_admonition_with_border() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let md = Markdown::new(
        "> [!NOTE]
> Important information.",
    );
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
    assert_eq!(md.toc().len(), 3);
}

#[test]
fn test_markdown_toc_levels() {
    let md = Markdown::new(
        "# Level 1
## Level 2
# Another Level 1",
    );
    let toc = md.toc();
    assert_eq!(toc[0].level, 1);
    assert_eq!(toc[1].level, 2);
    assert_eq!(toc[2].level, 1);
}
