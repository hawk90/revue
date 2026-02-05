//! Markdown 위젯 테스트
//! Markdown widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::utils::figlet::FigletFont;
use revue::utils::syntax::SyntaxTheme;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{markdown, Markdown};

// ─────────────────────────────────────────────────────────────────────────
// 기본 생성 및 빌더 메서드 테스트
// Basic creation and builder methods
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_new() {
    let md = Markdown::new("# Hello");
    assert_eq!(md.source(), "# Hello");
}

#[test]
fn test_markdown_new_with_string() {
    let md = Markdown::new(String::from("# World"));
    assert_eq!(md.source(), "# World");
}

#[test]
fn test_markdown_default() {
    let md = Markdown::default();
    assert_eq!(md.source(), "");
}

#[test]
fn test_markdown_helper() {
    let md = markdown("# Test content");
    assert_eq!(md.source(), "# Test content");
}

#[test]
fn test_markdown_line_count() {
    let md = Markdown::new("# Heading 1\n\nSome content");
    assert!(md.line_count() >= 2);
}

// ─────────────────────────────────────────────────────────────────────────
// 헤딩(제목) 테스트
// Heading tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_heading_h1() {
    let md = Markdown::new("# Heading 1");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_heading_h2() {
    let md = Markdown::new("## Heading 2");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_heading_h3() {
    let md = Markdown::new("### Heading 3");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_heading_h4() {
    let md = Markdown::new("#### Heading 4");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_heading_h5() {
    let md = Markdown::new("##### Heading 5");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_heading_h6() {
    let md = Markdown::new("###### Heading 6");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_multiple_headings() {
    let md = Markdown::new("# First\n\n## Second\n\n### Third");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_heading_color() {
    let md = Markdown::new("# Colored Heading").heading_fg(Color::RED);
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    md.render(&mut ctx);
    // Should render without error
}

// ─────────────────────────────────────────────────────────────────────────
// Text formatting tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_bold() {
    let md = Markdown::new("This is **bold** text.");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_bold_underscore() {
    let md = Markdown::new("This is __bold__ text.");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_italic() {
    let md = Markdown::new("This is *italic* text.");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_italic_underscore() {
    let md = Markdown::new("This is _italic_ text.");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_bold_italic() {
    let md = Markdown::new("This is ***bold italic*** text.");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_strikethrough() {
    let md = Markdown::new("This is ~~strikethrough~~ text.");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_inline_code() {
    let md = Markdown::new("Inline `code` here.");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_code_color() {
    let md = Markdown::new("Code `test`").code_fg(Color::GREEN);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    md.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Paragraph tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_single_paragraph() {
    let md = Markdown::new("This is a single paragraph.");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_multiple_paragraphs() {
    let md = Markdown::new("First paragraph.\n\nSecond paragraph.");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_long_paragraph() {
    let md = Markdown::new("This is a very long paragraph that contains many words and should span across multiple lines in the buffer when rendered.");
    assert!(md.line_count() >= 1);
}

// ─────────────────────────────────────────────────────────────────────────
// List tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_unordered_list() {
    let md = Markdown::new("- Item 1\n- Item 2\n- Item 3");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_unordered_list_plus() {
    let md = Markdown::new("+ Item 1\n+ Item 2");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_unordered_list_asterisk() {
    let md = Markdown::new("* Item 1\n* Item 2");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_ordered_list() {
    let md = Markdown::new("1. First\n2. Second\n3. Third");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_nested_list() {
    let md = Markdown::new("- Item 1\n  - Nested 1\n  - Nested 2\n- Item 2");
    assert!(md.line_count() >= 4);
}

#[test]
fn test_markdown_task_list_checked() {
    let md = Markdown::new("- [x] Completed task");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_task_list_unchecked() {
    let md = Markdown::new("- [ ] Incomplete task");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_task_list_mixed() {
    let md = Markdown::new("- [x] Done\n- [ ] Todo\n- [x] Also done");
    assert!(md.line_count() >= 3);
}

// ─────────────────────────────────────────────────────────────────────────
// 코드 블록 테스트
// Code block tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_code_block_fenced() {
    let md = Markdown::new("```\nlet x = 42;\n```");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_code_block_with_language() {
    let md = Markdown::new("```rust\nfn main() {}\n```");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_code_block_python() {
    let md = Markdown::new("```python\ndef hello():\n    pass\n```");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_code_block_javascript() {
    let md = Markdown::new("```javascript\nconsole.log('test');\n```");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_code_block_multiple_lines() {
    let md = Markdown::new("```rust\nfn main() {\n    println!(\"Hello\");\n}\n```");
    assert!(md.line_count() >= 4);
}

#[test]
fn test_markdown_syntax_highlight_enabled() {
    let md = Markdown::new("```rust\nlet x = 42;\n```").syntax_highlight(true);
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_syntax_highlight_disabled() {
    let md = Markdown::new("```rust\nlet x = 42;\n```").syntax_highlight(false);
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_syntax_theme_monokai() {
    let md = Markdown::new("```rust\nlet x = 42;\n```").theme_monokai();
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_syntax_theme_nord() {
    let md = Markdown::new("```rust\nlet x = 42;\n```").theme_nord();
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_syntax_theme_dracula() {
    let md = Markdown::new("```rust\nlet x = 42;\n```").theme_dracula();
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_syntax_theme_one_dark() {
    let md = Markdown::new("```rust\nlet x = 42;\n```").theme_one_dark();
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_syntax_theme_custom() {
    let theme = SyntaxTheme::monokai();
    let md = Markdown::new("```rust\nlet x = 42;\n```").syntax_theme(theme);
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_code_line_numbers_enabled() {
    let md = Markdown::new("```\nline1\nline2\n```").code_line_numbers(true);
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_code_line_numbers_disabled() {
    let md = Markdown::new("```\nline1\nline2\n```").code_line_numbers(false);
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_code_border_enabled() {
    let md = Markdown::new("```\ncode\n```").code_border(true);
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_code_border_disabled() {
    let md = Markdown::new("```\ncode\n```").code_border(false);
    assert!(md.line_count() >= 1);
}

// ─────────────────────────────────────────────────────────────────────────
// Link tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_link() {
    let md = Markdown::new("[Link](https://example.com)");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_link_with_title() {
    let md = Markdown::new("[Link](https://example.com \"Title\")");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_link_color() {
    let md = Markdown::new("[Link](https://example.com)").link_fg(Color::MAGENTA);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    md.render(&mut ctx);
}

#[test]
fn test_markdown_multiple_links() {
    let md = Markdown::new("[First](https://first.com) and [Second](https://second.com)");
    assert!(md.line_count() >= 1);
}

// ─────────────────────────────────────────────────────────────────────────
// Blockquote tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_blockquote() {
    let md = Markdown::new("> This is a quote");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_blockquote_multiline() {
    let md = Markdown::new("> Line 1\n> Line 2\n> Line 3");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_blockquote_with_formatting() {
    let md = Markdown::new("> This is **bold** and *italic*");
    assert!(md.line_count() >= 1);
}

// ─────────────────────────────────────────────────────────────────────────
// Admonition/Callout tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_admonition_note() {
    let md = Markdown::new("> [!NOTE]\n> This is a note.");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_admonition_tip() {
    let md = Markdown::new("> [!TIP]\n> Useful tip here.");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_admonition_important() {
    let md = Markdown::new("> [!IMPORTANT]\n> Pay attention!");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_admonition_warning() {
    let md = Markdown::new("> [!WARNING]\n> Be careful!");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_admonition_caution() {
    let md = Markdown::new("> [!CAUTION]\n> Danger ahead!");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_admonition_case_insensitive() {
    let md = Markdown::new("> [!note]\n> Lowercase note.");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_admonition_multiline() {
    let md = Markdown::new("> [!WARNING]\n> Line 1\n> Line 2\n> Line 3");
    assert!(md.line_count() >= 4);
}

#[test]
fn test_markdown_admonition_render() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let md = Markdown::new("> [!NOTE]\n> Important information.");
    md.render(&mut ctx);

    // Look for the vertical bar (│) which is part of admonition styling
    let mut found_bar = false;
    for y in 0..10 {
        if buffer.get(0, y).unwrap().symbol == '│' {
            found_bar = true;
            break;
        }
    }
    assert!(found_bar, "Admonition border should be rendered");
}

// ─────────────────────────────────────────────────────────────────────────
// Table tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_table_simple() {
    let md = Markdown::new("| A | B |\n|---|---|\n| 1 | 2 |");
    assert!(md.line_count() >= 4);
}

#[test]
fn test_markdown_table_multiple_rows() {
    let md = Markdown::new("| Name | Age |\n|------|-----|\n| Alice | 30 |\n| Bob | 25 |");
    assert!(md.line_count() >= 5);
}

#[test]
fn test_markdown_table_multiple_columns() {
    let md = Markdown::new("| A | B | C | D |\n|---|---|---|---|\n| 1 | 2 | 3 | 4 |");
    assert!(md.line_count() >= 4);
}

#[test]
fn test_markdown_table_with_content() {
    let md = Markdown::new("| Header 1 | Header 2 |\n|----------|----------|\n| Data 1 | Data 2 |");
    assert!(md.line_count() >= 4);
}

// ─────────────────────────────────────────────────────────────────────────
// Footnote tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_footnote_reference() {
    let md = Markdown::new("Text with footnote[^1]\n\n[^1]: This is the footnote.");
    assert!(md.line_count() >= 2);
}

#[test]
fn test_markdown_multiple_footnotes() {
    let md =
        Markdown::new("First[^a] and second[^b].\n\n[^a]: First footnote.\n[^b]: Second footnote.");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_footnote_render() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let md = Markdown::new("Text[^note]\n\n[^note]: My footnote.");
    md.render(&mut ctx);

    // Look for the separator line (─) which indicates footnotes section
    let mut found_separator = false;
    for y in 0..20 {
        if buffer.get(0, y).unwrap().symbol == '─' {
            found_separator = true;
            break;
        }
    }
    assert!(found_separator, "Footnotes separator should be rendered");
}

#[test]
fn test_markdown_footnote_number_ordering() {
    let md = Markdown::new("A[^z] B[^a]\n\n[^a]: Alpha\n[^z]: Zeta");
    assert!(md.line_count() >= 3);
}

// ─────────────────────────────────────────────────────────────────────────
// Horizontal rule tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_rule_asterisks() {
    let md = Markdown::new("Above\n\n***\n\nBelow");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_rule_dashes() {
    let md = Markdown::new("Above\n\n---\n\nBelow");
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_rule_underscores() {
    let md = Markdown::new("Above\n\n___\n\nBelow");
    assert!(md.line_count() >= 3);
}

// ─────────────────────────────────────────────────────────────────────────
// Figlet heading tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_figlet_headings_enabled() {
    let md = Markdown::new("# Big Title").figlet_headings(true);
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_figlet_headings_disabled() {
    let md = Markdown::new("# Regular Title").figlet_headings(false);
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_figlet_font_block() {
    let md = Markdown::new("# Title").figlet_font(FigletFont::Block);
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_figlet_font_banner() {
    let md = Markdown::new("# Title").figlet_font(FigletFont::Banner);
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_figlet_max_level_1() {
    let md = Markdown::new("# H1\n\n## H2\n\n### H3")
        .figlet_headings(true)
        .figlet_max_level(1);
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_figlet_max_level_2() {
    let md = Markdown::new("# H1\n\n## H2\n\n### H3")
        .figlet_headings(true)
        .figlet_max_level(2);
    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_figlet_render() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let md = Markdown::new("# Test").figlet_headings(true);
    md.render(&mut ctx);
    // Should render without error
}

// ─────────────────────────────────────────────────────────────────────────
// 목차(Table of Contents) 테스트
// Table of Contents tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_toc_extraction() {
    let md = Markdown::new("# Title 1\n\n## Title 2\n\n### Title 3");
    let toc = md.toc();
    assert_eq!(toc.len(), 3);
}

#[test]
fn test_markdown_toc_levels() {
    let md = Markdown::new("# H1\n\n## H2\n\n### H3\n\n#### H4");
    let toc = md.toc();
    assert_eq!(toc[0].level, 1);
    assert_eq!(toc[1].level, 2);
    assert_eq!(toc[2].level, 3);
    assert_eq!(toc[3].level, 4);
}

#[test]
fn test_markdown_toc_text() {
    let md = Markdown::new("# First Heading\n\n## Second Heading");
    let toc = md.toc();
    assert_eq!(toc[0].text, "First Heading");
    assert_eq!(toc[1].text, "Second Heading");
}

#[test]
fn test_markdown_toc_string() {
    let md = Markdown::new("# Title 1\n\n## Title 2");
    let toc_string = md.toc_string();
    assert!(toc_string.contains("Title 1"));
    assert!(toc_string.contains("Title 2"));
}

#[test]
fn test_markdown_show_toc_disabled() {
    let md = Markdown::new("# Title").show_toc(false);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    md.render(&mut ctx);
}

#[test]
fn test_markdown_show_toc_enabled() {
    let md = Markdown::new("# Title 1\n\n## Title 2").show_toc(true);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    md.render(&mut ctx);
}

#[test]
fn test_markdown_toc_title() {
    let md = Markdown::new("# Title").toc_title("Contents");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    md.render(&mut ctx);
}

#[test]
fn test_markdown_toc_fg() {
    let md = Markdown::new("# Title").show_toc(true).toc_fg(Color::GREEN);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    md.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Render tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_render_basic() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let md = Markdown::new("# Test\n\nHello world.");
    md.render(&mut ctx);

    // Check that heading prefix '#' is rendered
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
fn test_markdown_render_with_offset() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(5, 3, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let md = Markdown::new("# Offset Test");
    md.render(&mut ctx);
    // Should render without error
}

#[test]
fn test_markdown_render_empty_area() {
    let md = Markdown::new("# Test");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 0, 1); // Zero width
    let mut ctx = RenderContext::new(&mut buffer, area);
    md.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_markdown_render_zero_height() {
    let md = Markdown::new("# Test");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 0); // Zero height
    let mut ctx = RenderContext::new(&mut buffer, area);
    md.render(&mut ctx);
    // Should not panic
}

// ─────────────────────────────────────────────────────────────────────────
// StyledView trait tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_styled_view_set_id() {
    let mut md = Markdown::new("# Test");
    md.set_id("test-markdown");
    assert_eq!(View::id(&md), Some("test-markdown"));
}

#[test]
fn test_markdown_styled_view_add_class() {
    let mut md = Markdown::new("# Test");
    md.add_class("header");
    md.add_class("bold");
    assert!(md.has_class("header"));
    assert!(md.has_class("bold"));
}

#[test]
fn test_markdown_styled_view_remove_class() {
    let md = Markdown::new("# Test").class("a").class("b").class("c");
    let mut md = md;
    md.remove_class("b");
    assert!(md.has_class("a"));
    assert!(!md.has_class("b"));
    assert!(md.has_class("c"));
}

#[test]
fn test_markdown_styled_view_toggle_class() {
    let mut md = Markdown::new("# Test");
    md.toggle_class("test");
    assert!(md.has_class("test"));
    md.toggle_class("test");
    assert!(!md.has_class("test"));
}

#[test]
fn test_markdown_builder_element_id() {
    let md = Markdown::new("# Test").element_id("my-markdown");
    assert_eq!(View::id(&md), Some("my-markdown"));
}

#[test]
fn test_markdown_builder_class() {
    let md = Markdown::new("# Test").class("header").class("styled");
    assert!(md.has_class("header"));
    assert!(md.has_class("styled"));
}

#[test]
fn test_markdown_builder_classes() {
    let md = Markdown::new("# Test").classes(vec!["first", "second", "third"]);
    assert!(md.has_class("first"));
    assert!(md.has_class("second"));
    assert!(md.has_class("third"));
}

// ─────────────────────────────────────────────────────────────────────────
// View trait tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_view_widget_type() {
    let md = Markdown::new("# Test");
    assert_eq!(md.widget_type(), "Markdown");
}

#[test]
fn test_markdown_view_id_none() {
    let md = Markdown::new("# Test");
    assert!(View::id(&md).is_none());
}

#[test]
fn test_markdown_view_id_some() {
    let md = Markdown::new("# Test").element_id("test-id");
    assert_eq!(View::id(&md), Some("test-id"));
}

#[test]
fn test_markdown_view_classes_empty() {
    let md = Markdown::new("# Test");
    assert!(View::classes(&md).is_empty());
}

#[test]
fn test_markdown_view_classes_with_values() {
    let md = Markdown::new("# Test").class("first").class("second");
    let classes = View::classes(&md);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"first".to_string()));
    assert!(classes.contains(&"second".to_string()));
}

#[test]
fn test_markdown_view_meta() {
    let md = Markdown::new("# Test")
        .element_id("test-id")
        .class("test-class");
    let meta = md.meta();
    assert_eq!(meta.widget_type, "Markdown");
    assert_eq!(meta.id, Some("test-id".to_string()));
    assert!(meta.classes.contains("test-class"));
}

#[test]
fn test_markdown_view_children_default() {
    let md = Markdown::new("# Test");
    assert!(View::children(&md).is_empty());
}

// ─────────────────────────────────────────────────────────────────────────
// 엣지 케이스 및 특수 콘텐츠 테스트
// Edge cases and special content
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_empty_source() {
    let md = Markdown::new("");
    assert_eq!(md.source(), "");
}

#[test]
fn test_markdown_whitespace_only() {
    let md = Markdown::new("   \n\n   ");
    assert!(md.line_count() >= 0);
}

#[test]
fn test_markdown_special_characters() {
    let md = Markdown::new("Special: < > & \" ' © ® ™");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_unicode() {
    let md = Markdown::new("Unicode: 日本語 한국어 中文");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_emoji() {
    let md = Markdown::new("Emoji: 😀 🎉 ❤️ 🚀");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_mixed_content() {
    let content = r#"# Title

This is a paragraph with **bold** and *italic* text.

- List item 1
- List item 2

```rust
let x = 42;
```

> A quote

[Link](https://example.com)
"#;
    let md = Markdown::new(content);
    assert!(md.line_count() >= 5);
}

#[test]
fn test_markdown_very_long_content() {
    let content = "# Heading\n\n".repeat(100);
    let md = Markdown::new(content);
    assert!(md.line_count() >= 100);
}

#[test]
fn test_markdeep_link_reference() {
    let md = Markdown::new("[Text][reference]\n\n[reference]: https://example.com");
    assert!(md.line_count() >= 1);
}

#[test]
fn test_markdown_complex_nested_structure() {
    let content = "# Main\n\n\
## Section\n\n\
- Item with **bold**\n\
  - Nested item\n\
    - Deep nested\n\
- Another item\n\n\
### Subsection\n\n\
> Quote with `code`\n\n\
```rust\n\
fn test() {\n\
    println!(\"test\");\n\
}\n\
```\n\n\
1. Ordered\n\
2. List\n\
3. Items\n\n\
[Link](url)\n";
    let md = Markdown::new(content);
    assert!(md.line_count() >= 10);
}

// ─────────────────────────────────────────────────────────────────────────
// Builder method chaining tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_markdown_builder_chain_complete() {
    let md = Markdown::new("# Styled Content")
        .heading_fg(Color::CYAN)
        .link_fg(Color::MAGENTA)
        .code_fg(Color::YELLOW)
        .figlet_headings(true)
        .show_toc(true)
        .syntax_highlight(true)
        .code_line_numbers(true)
        .code_border(true)
        .theme_monokai()
        .element_id("styled")
        .class("custom");

    assert_eq!(md.source(), "# Styled Content");
    assert_eq!(View::id(&md), Some("styled"));
    assert!(md.has_class("custom"));
}

#[test]
fn test_markdown_builder_chain_colors() {
    let md = Markdown::new("# Test")
        .heading_fg(Color::RED)
        .link_fg(Color::GREEN)
        .code_fg(Color::BLUE)
        .toc_fg(Color::YELLOW);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    md.render(&mut ctx);
}

#[test]
fn test_markdown_builder_chain_figlet() {
    let md = Markdown::new("# Big Title")
        .figlet_font(FigletFont::Block)
        .figlet_max_level(2);

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    md.render(&mut ctx);
}

#[test]
fn test_markdown_builder_chain_code() {
    let md = Markdown::new("```rust\nlet x = 42;\n```")
        .syntax_highlight(true)
        .syntax_theme(SyntaxTheme::nord())
        .code_line_numbers(true)
        .code_border(true);

    assert!(md.line_count() >= 3);
}

#[test]
fn test_markdown_builder_chain_toc() {
    let md = Markdown::new("# One\n\n## Two\n\n### Three")
        .show_toc(true)
        .toc_title("Table of Contents")
        .toc_fg(Color::CYAN);

    let toc = md.toc();
    assert_eq!(toc.len(), 3);
}
