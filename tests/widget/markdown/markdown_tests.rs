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
use revue::widget::markdown::{markdown as markdown_fn, Markdown};
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
    let md = markdown_fn("Test content");
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
    assert!(md.line_count() >= 1);
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

// =========================================================================
// Rendering Regression Tests
// =========================================================================

/// Render `source` into a `width` x 24 buffer and return each row as text,
/// with trailing blanks trimmed.
fn render_rows(source: &str, width: u16) -> Vec<String> {
    let mut buffer = Buffer::new(width, 24);
    let area = Rect::new(0, 0, width, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    Markdown::new(source).render(&mut ctx);
    (0..24)
        .map(|y| {
            (0..width)
                .filter_map(|x| buffer.get(x, y).map(|c| c.symbol))
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect()
}

fn row_containing<'a>(rows: &'a [String], needle: &str) -> Option<&'a String> {
    rows.iter().find(|r| r.contains(needle))
}

#[test]
fn test_markdown_text_after_table_still_renders() {
    let rows = render_rows(
        "| A | B |\n|---|---|\n| 1 | 2 |\n\n## After\n\nTrailing paragraph.",
        40,
    );
    assert!(row_containing(&rows, "After").is_some(), "{rows:#?}");
    assert!(
        row_containing(&rows, "Trailing paragraph.").is_some(),
        "{rows:#?}"
    );
}

#[test]
fn test_markdown_table_cells_render() {
    let rows = render_rows(
        "| Name | Value |\n|------|-------|\n| alpha | 1 |\n| beta | 22 |",
        40,
    );
    let header = row_containing(&rows, "Name").expect("header row");
    assert!(header.contains("Value"), "{rows:#?}");
    let alpha = row_containing(&rows, "alpha").expect("first body row");
    assert!(alpha.contains('1'), "{rows:#?}");
    let beta = row_containing(&rows, "beta").expect("second body row");
    assert!(beta.contains("22"), "{rows:#?}");
    // Header and body are separate rows, with a separator between them
    let header_y = rows.iter().position(|r| r.contains("Name")).unwrap();
    let alpha_y = rows.iter().position(|r| r.contains("alpha")).unwrap();
    assert!(alpha_y > header_y + 1, "{rows:#?}");
    assert!(
        rows[header_y + 1].contains('─'),
        "separator under header: {rows:#?}"
    );
    // Columns line up: the second column starts at the same x on every row
    let col = |r: &String, s: &str| r.find(s).unwrap();
    assert_eq!(col(header, "Value"), col(alpha, "1"), "{rows:#?}");
    assert_eq!(col(header, "Value"), col(beta, "22"), "{rows:#?}");
}

#[test]
fn test_markdown_table_clipped_to_width() {
    let rows = render_rows(
        "| Column one | Column two | Column three |\n|---|---|---|\n| a | b | c |",
        12,
    );
    assert!(rows.iter().all(|r| r.chars().count() <= 12));
    assert!(row_containing(&rows, "Column").is_some(), "{rows:#?}");
}

#[test]
fn test_markdown_bullet_precedes_item_text() {
    let rows = render_rows("- Markdown support\n- Second", 40);
    let first = row_containing(&rows, "Markdown support").expect("item row");
    assert_eq!(first.trim(), "• Markdown support", "{rows:#?}");
    let second = row_containing(&rows, "Second").expect("item row");
    assert_eq!(second.trim(), "• Second", "{rows:#?}");
}

#[test]
fn test_markdown_nested_list_items_on_own_lines() {
    let rows = render_rows("- outer\n  - inner", 40);
    let outer = row_containing(&rows, "outer").expect("outer row");
    assert!(!outer.contains("inner"), "{rows:#?}");
    let inner = row_containing(&rows, "inner").expect("inner row");
    assert_eq!(inner.trim(), "• inner", "{rows:#?}");
}

#[test]
fn test_markdown_code_block_keeps_lines() {
    let rows = render_rows("```\nfirst line\nsecond line\nthird\n```", 40);
    let first_y = rows.iter().position(|r| r.contains("first line"));
    let second_y = rows.iter().position(|r| r.contains("second line"));
    let third_y = rows.iter().position(|r| r.contains("third"));
    let (first_y, second_y, third_y) = match (first_y, second_y, third_y) {
        (Some(a), Some(b), Some(c)) => (a, b, c),
        _ => panic!("missing code lines: {rows:#?}"),
    };
    assert_ne!(first_y, second_y, "{rows:#?}");
    assert!(first_y < second_y && second_y < third_y, "{rows:#?}");
}

#[test]
fn test_markdown_code_block_box_fits_content() {
    let rows = render_rows("```\nshort\na much longer line of code here\n```", 60);
    let top = row_containing(&rows, "┌").expect("top border");
    let bottom = row_containing(&rows, "└").expect("bottom border");
    let short = row_containing(&rows, "short").expect("content row");
    let long = row_containing(&rows, "a much longer").expect("content row");
    let w = top.chars().count();
    // Every row of the box has the same width and closes on the right
    for r in [bottom, short, long] {
        assert_eq!(r.chars().count(), w, "{rows:#?}");
    }
    assert!(short.ends_with('│') && long.ends_with('│'), "{rows:#?}");
    assert!(top.ends_with('┐') && bottom.ends_with('┘'), "{rows:#?}");
}

#[test]
fn test_markdown_plain_paragraph_has_no_quote_bar() {
    let rows = render_rows("Just a paragraph.", 40);
    assert_eq!(rows[0], "Just a paragraph.", "{rows:#?}");
}

#[test]
fn test_markdown_real_blockquote_keeps_quote_bar() {
    let rows = render_rows("Intro.\n\n> Quoted text\n\nOutro.", 40);
    let quote = row_containing(&rows, "Quoted text").expect("quote row");
    assert!(quote.starts_with("│ "), "{rows:#?}");
    let intro = row_containing(&rows, "Intro.").unwrap();
    assert!(!intro.contains('│'), "{rows:#?}");
    let outro = row_containing(&rows, "Outro.").expect("outro row");
    assert!(!outro.contains('│'), "{rows:#?}");
}

#[test]
fn test_markdown_blockquote_style_does_not_leak() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    Markdown::new("> quoted\n\nplain").render(&mut ctx);
    let y = (0..10)
        .find(|&y| buffer.get(0, y).map(|c| c.symbol) == Some('p'))
        .expect("plain row");
    let cell = buffer.get(0, y).unwrap();
    assert!(
        !cell.modifier.contains(revue::render::Modifier::ITALIC),
        "text after a blockquote must not stay italic"
    );
}
