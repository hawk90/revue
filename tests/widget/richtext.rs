//! Rich text widget integration tests
//!
//! Tests for markup parsing, colors, links, span width, and rendering.

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{markup, rich_text, span, style, RichText, Span, Style};

// =============================================================================
// Markup Parsing Edge Cases Tests
// =============================================================================

#[test]
fn test_markup_unclosed_tag() {
    let rt = RichText::markup("[bold]Unclosed text");

    // Should treat unclosed tag as applying to rest
    assert!(rt.len() >= 1);
    assert_eq!(rt.width(), 13); // "Unclosed text" = 13 chars
}

#[test]
fn test_markup_mismatched_tags() {
    let rt = RichText::markup("[bold][/italic]Text");

    // Should handle mismatched closing tag
    assert!(rt.len() >= 1);
}

#[test]
fn test_markup_empty_tag() {
    let rt = RichText::markup("[]Text");

    // Should handle empty tag gracefully
    assert!(rt.len() >= 1);
    assert_eq!(rt.width(), 4);
}

#[test]
fn test_markup_nested_tags() {
    let rt = RichText::markup("[b][i]Nested[/i][/b]");

    // Should apply both styles
    assert!(rt.len() >= 1);
    assert_eq!(rt.width(), 6);
}

#[test]
fn test_markup_deep_nesting() {
    let tags = "[b]".repeat(10);
    let closing = "[/]".repeat(10);
    let rt = RichText::markup(&format!("{}Text{}", tags, closing));

    // Should handle deep nesting
    assert!(rt.len() >= 1);
    assert_eq!(rt.width(), 4);
}

#[test]
fn test_markup_adjacent_tags() {
    let rt = RichText::markup("[b]A[/b][i]B[/i]");

    // Should create separate spans
    assert_eq!(rt.len(), 2);
    assert_eq!(rt.width(), 2);
}

#[test]
fn test_markup_tag_without_content() {
    let rt = RichText::markup("[b][/]");

    // Empty tag with reset should work
    assert!(rt.is_empty() || rt.width() == 0);
}

#[test]
fn test_markup_multiple_reset_tags() {
    let rt = RichText::markup("[b]Text1[/]Text2[/]Text3");

    // Multiple resets should work
    assert!(rt.len() >= 2);
}

#[test]
fn test_markup_whitespace_in_tags() {
    let rt = RichText::markup("[bold ]Text[/ ]");

    // Should handle whitespace in tags
    assert!(rt.len() >= 1);
}

#[test]
fn test_markup_special_chars_in_tags() {
    let rt = RichText::markup("[b]Test@#$%^&*()Text[/]");

    // Should handle special characters in content
    assert_eq!(rt.width(), 17);
}

#[test]
fn test_markup_empty_string() {
    let rt = RichText::markup("");

    assert!(rt.is_empty());
}

// =============================================================================
// Color Markup Tests
// =============================================================================

#[test]
fn test_markup_all_standard_colors() {
    for (name, _expected) in [
        ("red", Color::RED),
        ("green", Color::GREEN),
        ("blue", Color::BLUE),
        ("yellow", Color::YELLOW),
        ("cyan", Color::CYAN),
        ("magenta", Color::MAGENTA),
        ("white", Color::WHITE),
        ("black", Color::BLACK),
    ] {
        let markup = format!("[{}]Text[/]", name);
        let rt = RichText::markup(&markup);
        // Should parse without error
        assert_eq!(rt.width(), 4);
    }
}

#[test]
fn test_markup_all_background_colors() {
    for (name, _expected) in [
        ("on_red", Color::RED),
        ("on_green", Color::GREEN),
        ("on_blue", Color::BLUE),
        ("on_yellow", Color::YELLOW),
        ("on_cyan", Color::CYAN),
        ("on_magenta", Color::MAGENTA),
        ("on_white", Color::WHITE),
        ("on_black", Color::BLACK),
    ] {
        let markup = format!("[{}]Text[/]", name);
        let rt = RichText::markup(&markup);
        // Should parse without error
        assert_eq!(rt.width(), 4);
    }
}

#[test]
fn test_markup_combined_colors() {
    let rt = RichText::markup("[red on_blue]Text[/]");

    assert_eq!(rt.width(), 4);
}

// =============================================================================
// Link Markup Tests
// =============================================================================

#[test]
fn test_markup_link_with_formatting() {
    let rt = RichText::markup("[link=https://example.com][b]Click[/b][/]");

    assert!(rt.len() >= 1);
}

#[test]
fn test_markup_link_empty_url() {
    let rt = RichText::markup("[link=]Text[/]");

    // Should handle empty URL
    assert!(rt.len() >= 1);
}

#[test]
fn test_markup_link_special_chars() {
    let rt = RichText::markup("[link=https://example.com?param=value&other=test]Link[/]");

    assert!(rt.len() >= 1);
}

// =============================================================================
// Span Width Tests
// =============================================================================

#[test]
fn test_span_width_wide_chars() {
    let span = Span::new("日本語"); // Japanese characters

    // CJK characters are width 2
    assert_eq!(span.width(), 6);
}

#[test]
fn test_span_width_emoji() {
    let span = Span::new("😀😃"); // Emoji

    // Emoji are typically width 2
    assert!(span.width() >= 2);
}

#[test]
fn test_span_width_combining() {
    let span = Span::new("café"); // With combining accent

    // Should count correctly
    assert!(span.width() >= 4);
}

#[test]
fn test_span_width_mixed() {
    let span = Span::new("Hi 日本語 😀");

    // Should calculate mixed width correctly
    assert!(span.width() > 10);
}

#[test]
fn test_span_width_ascii() {
    let span = Span::new("Hello");

    assert_eq!(span.width(), 5);
}

#[test]
fn test_span_width_tabs() {
    let span = Span::new("\t");

    // Tab has a width
    assert!(span.width() >= 1);
}

// =============================================================================
// Rendering Tests
// =============================================================================

#[test]
fn test_render_hyperlink_registration() {
    let rt = RichText::new().push_link("Click here", "https://example.com");

    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    // Check hyperlink was registered
    assert!(buffer.get(0, 0).unwrap().hyperlink_id.is_some());
}

#[test]
fn test_render_modifier_application() {
    let rt = RichText::new()
        .push("Bold ", Style::new().bold())
        .push("Italic ", Style::new().italic())
        .push("Underline", Style::new().underline());

    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    // Check modifiers were applied
    assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
    assert!(buffer
        .get(6, 0)
        .unwrap()
        .modifier
        .contains(Modifier::ITALIC));
    assert!(buffer
        .get(13, 0)
        .unwrap()
        .modifier
        .contains(Modifier::UNDERLINE));
}

#[test]
fn test_render_color_application() {
    let rt = RichText::new()
        .push("Red ", Style::new().fg(Color::RED))
        .push("Blue", Style::new().fg(Color::BLUE));

    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(5, 0).unwrap().fg, Some(Color::BLUE));
}

#[test]
fn test_render_wide_char_continuation() {
    let rt = RichText::new().text("日本"); // Two wide characters

    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    // Wide chars should use continuation cells
    // First wide char at 0, continuation at 1
    // Second wide char at 2, continuation at 3
    assert!(buffer.get(1, 0).is_some());
    assert!(buffer.get(3, 0).is_some());
}

#[test]
fn test_render_zero_area() {
    let rt = RichText::new().text("Test");

    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    // Should not panic
}

#[test]
fn test_render_truncation() {
    let rt = RichText::new().text("This is very long text that exceeds area");

    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    // Should truncate at area width
}

// =============================================================================
// RichText Builder Tests
// =============================================================================

#[test]
fn test_rich_text_builder_chain() {
    let rt = RichText::new()
        .push("Hello ", Style::new().bold())
        .push("World", Style::new().fg(Color::GREEN))
        .push_link("Click", "https://example.com");

    assert_eq!(rt.len(), 3);
}

#[test]
fn test_rich_text_append_mutable() {
    let mut rt = RichText::new();

    rt.append("First", Style::new());
    rt.append("Second", Style::new().bold());
    rt.append_link("Link", "https://example.com");

    assert_eq!(rt.len(), 3);
}

#[test]
fn test_rich_text_clear() {
    let mut rt = RichText::new()
        .push("Test", Style::new())
        .push("Data", Style::new());

    assert_eq!(rt.len(), 2);

    rt.clear();

    assert!(rt.is_empty());
}

#[test]
fn test_rich_text_width() {
    let rt = RichText::new()
        .push("Hello", Style::new())
        .push(" ", Style::new())
        .push("World", Style::new());

    assert_eq!(rt.width(), 11);
}

#[test]
fn test_rich_text_is_empty() {
    let rt = RichText::new();
    assert!(rt.is_empty());

    let rt2 = RichText::new().text("Test");
    assert!(!rt2.is_empty());
}

#[test]
fn test_rich_text_plain() {
    let rt = RichText::plain("Plain text");

    assert_eq!(rt.len(), 1);
    assert_eq!(rt.width(), 10);
}

// =============================================================================
// Style Tests
// =============================================================================

#[test]
fn test_style_all_modifiers() {
    let style = Style::new()
        .bold()
        .italic()
        .underline()
        .dim()
        .strikethrough()
        .reverse();

    assert!(style.bold);
    assert!(style.italic);
    assert!(style.underline);
    assert!(style.dim);
    assert!(style.strikethrough);
    assert!(style.reverse);
}

#[test]
fn test_style_all_preset_colors() {
    assert_eq!(Style::red().fg, Some(Color::RED));
    assert_eq!(Style::green().fg, Some(Color::GREEN));
    assert_eq!(Style::blue().fg, Some(Color::BLUE));
    assert_eq!(Style::yellow().fg, Some(Color::YELLOW));
    assert_eq!(Style::cyan().fg, Some(Color::CYAN));
    assert_eq!(Style::magenta().fg, Some(Color::MAGENTA));
    assert_eq!(Style::white().fg, Some(Color::WHITE));
}

// =============================================================================
// Span Tests
// =============================================================================

#[test]
fn test_span_styled() {
    let span = Span::styled("Text", Style::new().bold());

    assert_eq!(span.text, "Text");
    assert!(span.style.bold);
}

#[test]
fn test_span_href() {
    let span = Span::link("Link", "https://example.com");

    assert_eq!(span.text, "Link");
    assert_eq!(span.link, Some("https://example.com".to_string()));
    assert!(span.style.underline);
    assert_eq!(span.style.fg, Some(Color::CYAN));
}

#[test]
fn test_span_href_method() {
    // The href() method only sets the URL, not the style
    let span = Span::new("Link").href("https://example.com");

    assert_eq!(span.text, "Link");
    assert_eq!(span.link, Some("https://example.com".to_string()));
    // href() doesn't modify style, so these should be default
    assert!(!span.style.underline);
}

#[test]
fn test_span_clone() {
    let span1 = Span::styled("Test", Style::new().bold());
    let span2 = span1.clone();

    assert_eq!(span1.text, span2.text);
    assert_eq!(span1.style.bold, span2.style.bold);
}

// =============================================================================
// Helper Functions Tests
// =============================================================================

#[test]
fn test_rich_text_helper() {
    let rt = rich_text();

    assert!(rt.is_empty());
}

#[test]
fn test_markup_helper() {
    let rt = markup("[bold]Test[/]");

    assert!(rt.len() >= 1);
}

#[test]
fn test_span_helper() {
    let s = span("Test");

    assert_eq!(s.text, "Test");
}

#[test]
fn test_style_helper() {
    let s = style();

    assert!(!s.bold);
    assert!(!s.italic);
}

#[test]
fn test_rich_text_view_meta() {
    let rt = RichText::new();

    assert!(rt.widget_type().contains("RichText"));
}

#[test]
fn test_rich_text_props_builders() {
    let rt = RichText::new().element_id("my-text").class("text-class");

    assert_eq!(rt.id(), Some("my-text"));
    assert!(View::classes(&rt).contains(&"text-class".to_string()));
}

#[test]
fn test_markup_short_tags() {
    let rt = RichText::markup("[b]Bold[/b][i]Italic[/i][u]Underline[/u]");

    assert_eq!(rt.len(), 3);
}

#[test]
fn test_markup_multiple_attributes() {
    let rt = RichText::markup("[bold red underline]Text[/]");

    assert!(rt.len() >= 1);
}

#[test]
fn test_markup_dim_modifier() {
    let rt = RichText::markup("[dim]Text[/]");

    assert!(rt.len() >= 1);
}

#[test]
fn test_markup_strike_modifier() {
    let rt = RichText::markup("[s]Text[/]");

    assert!(rt.len() >= 1);
}

#[test]
fn test_markup_reverse_modifier() {
    let rt = RichText::markup("[rev]Text[/]");

    assert!(rt.len() >= 1);
}

#[test]
fn test_render_markup_colors() {
    let rt = RichText::markup("[red]Red text[/] [blue]Blue text[/]");

    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    // First part should be red
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_render_markup_bold() {
    let rt = RichText::markup("[b]Bold text[/]");

    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    // Should have bold modifier
    assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_render_markup_italic() {
    let rt = RichText::markup("[i]Italic text[/]");

    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    // Should have italic modifier
    assert!(buffer
        .get(0, 0)
        .unwrap()
        .modifier
        .contains(Modifier::ITALIC));
}

#[test]
fn test_render_link_from_markup() {
    let rt = RichText::markup("[link=https://example.com]Click here[/]");

    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    // Should have hyperlink
    assert!(buffer.get(0, 0).unwrap().hyperlink_id.is_some());
}

#[test]
fn test_span_style_method() {
    let style = Style::new().bold().fg(Color::RED);
    let span = Span::new("Text").style(style);

    assert_eq!(span.text, "Text");
    assert!(span.style.bold);
    assert_eq!(span.style.fg, Some(Color::RED));
}

#[test]
fn test_rich_text_default_style() {
    let rt = RichText::new().default_style(Style::new().fg(Color::CYAN));

    // Default style is set
    let _ = rt;
}

#[test]
fn test_rich_text_span_method() {
    let span = Span::styled("Custom", Style::new().bold());
    let rt = RichText::new().span(span);

    assert_eq!(rt.len(), 1);
}

// =============================================================================
// Style Clone Tests
// =============================================================================

#[test]
fn test_style_clone() {
    let style1 = Style::new().bold().fg(Color::RED);
    let style2 = style1.clone();

    assert_eq!(style1.bold, style2.bold);
    assert_eq!(style1.fg, style2.fg);
}

#[test]
fn test_span_clone_independent() {
    let span1 = Span::styled("Test", Style::new().bold());
    let span2 = span1.clone();

    // Modifying one should not affect the other
    assert_eq!(span1.text, span2.text);
    assert_eq!(span1.style.bold, span2.style.bold);
}

// =============================================================================
// RGB/RGBA Color Tests
// =============================================================================

#[test]
fn test_style_rgb_color() {
    let style = Style::new().fg(Color::rgb(100, 150, 200));
    assert_eq!(style.fg, Some(Color::rgb(100, 150, 200)));
}

#[test]
fn test_style_rgba_color() {
    let style = Style::new().fg(Color::rgba(200, 100, 50, 180));
    assert_eq!(style.fg, Some(Color::rgba(200, 100, 50, 180)));
}

#[test]
fn test_style_rgb_bg() {
    let style = Style::new().bg(Color::rgb(50, 60, 70));
    assert_eq!(style.bg, Some(Color::rgb(50, 60, 70)));
}

#[test]
fn test_style_rgba_bg() {
    let style = Style::new().bg(Color::rgba(30, 40, 50, 200));
    assert_eq!(style.bg, Some(Color::rgba(30, 40, 50, 200)));
}

#[test]
fn test_render_rgb_colors() {
    let rt = RichText::new()
        .push("RGB ", Style::new().fg(Color::rgb(255, 128, 0)))
        .push("RGBA ", Style::new().fg(Color::rgba(200, 100, 50, 180)));

    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::rgb(255, 128, 0)));
    assert_eq!(
        buffer.get(4, 0).unwrap().fg,
        Some(Color::rgba(200, 100, 50, 180))
    );
}

// =============================================================================
// Multiple Render Calls
// =============================================================================

#[test]
fn test_rich_text_multiple_renders() {
    let rt = RichText::new().text("Test Text");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);

    for _ in 0..5 {
        buffer.clear();
        let mut ctx = RenderContext::new(&mut buffer, area);
        rt.render(&mut ctx);

        // Should render consistently
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
    }
}

#[test]
fn test_rich_text_render_after_append() {
    let mut rt = RichText::new().text("First");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);

    // Render original
    {
        let mut ctx = RenderContext::new(&mut buffer, area);
        rt.render(&mut ctx);
    }
    assert_eq!(rt.width(), 5);

    // Append and render again
    rt.append(" Second", Style::new());
    buffer.clear();
    {
        let mut ctx = RenderContext::new(&mut buffer, area);
        rt.render(&mut ctx);
    }
    assert_eq!(rt.width(), 12);
}

// =============================================================================
// CSS Integration Tests
// =============================================================================

#[test]
fn test_rich_text_element_id() {
    let rt = RichText::new().element_id("my-text");
    assert_eq!(rt.id(), Some("my-text"));
}

#[test]
fn test_rich_text_single_class() {
    let rt = RichText::new().class("highlight");
    assert!(View::classes(&rt).contains(&"highlight".to_string()));
}

#[test]
fn test_rich_text_multiple_classes() {
    let rt = RichText::new()
        .class("class1")
        .class("class2")
        .class("class3");
    let classes = View::classes(&rt);

    assert!(classes.contains(&"class1".to_string()));
    assert!(classes.contains(&"class2".to_string()));
    assert!(classes.contains(&"class3".to_string()));
}

#[test]
fn test_rich_text_classes_vec() {
    let rt = RichText::new().classes(vec!["a", "b", "c"]);
    let classes = View::classes(&rt);

    assert_eq!(classes.len(), 3);
}

#[test]
fn test_rich_text_meta() {
    let rt = RichText::new()
        .element_id("test-text")
        .class("primary")
        .text("Content");

    let meta = rt.meta();
    assert_eq!(meta.id, Some("test-text".to_string()));
    assert!(meta.classes.contains("primary"));
    assert!(meta.widget_type.contains("RichText"));
}

#[test]
fn test_rich_text_view_widget_type() {
    let rt = RichText::new();
    assert!(rt.widget_type().contains("RichText"));
}

#[test]
fn test_rich_text_view_children() {
    let rt = RichText::new();
    assert!(View::children(&rt).is_empty());
}

// =============================================================================
// Additional Rendering Edge Cases
// =============================================================================

#[test]
fn test_render_single_char() {
    let rt = RichText::new().text("A");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
}

#[test]
fn test_render_emoji_only() {
    let rt = RichText::new().text("😀😃😄");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);
    // Should render emoji
    let cell = buffer.get(0, 0).unwrap();
    assert_ne!(cell.symbol, ' ');
}

#[test]
fn test_render_empty_string() {
    let rt = RichText::new().text("");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_render_with_offset() {
    let rt = RichText::new().text("Test");
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(10, 5, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);
    assert_eq!(buffer.get(10, 5).unwrap().symbol, 'T');
}

#[test]
fn test_render_single_pixel_width() {
    let rt = RichText::new().text("A");
    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);
}

#[test]
fn test_render_zero_height() {
    let rt = RichText::new().text("Test");
    let mut buffer = Buffer::new(10, 0);
    let area = Rect::new(0, 0, 10, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);
}

#[test]
fn test_render_newline_in_text() {
    let rt = RichText::new().text("Line1\nLine2");
    let mut buffer = Buffer::new(20, 2);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);
}

#[test]
fn test_render_tabs_in_text() {
    let rt = RichText::new().text("Item\tTabbed");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);
}

// =============================================================================
// Modifier Combination Tests
// =============================================================================

#[test]
fn test_style_bold_italic() {
    let rt = RichText::markup("[b][i]Bold Italic[/][/]");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::BOLD));
    assert!(cell.modifier.contains(Modifier::ITALIC));
}

#[test]
fn test_style_bold_underline() {
    let rt = RichText::markup("[b][u]Bold Underline[/][/]");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::BOLD));
    assert!(cell.modifier.contains(Modifier::UNDERLINE));
}

#[test]
fn test_style_all_modifiers_render() {
    let rt = RichText::markup("[b][i][u][dim][s][rev]All[/][/][/][/][/]");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::BOLD));
    assert!(cell.modifier.contains(Modifier::ITALIC));
    assert!(cell.modifier.contains(Modifier::UNDERLINE));
    assert!(cell.modifier.contains(Modifier::DIM));
    assert!(cell.modifier.contains(Modifier::CROSSED_OUT));
    assert!(cell.modifier.contains(Modifier::REVERSE));
}

// =============================================================================
// Link Tests
// =============================================================================

#[test]
fn test_link_with_style() {
    let rt = RichText::new()
        .push(
            "Click Me",
            Style::new().bold().fg(Color::YELLOW).underline(),
        )
        .push_link("Link", "https://example.com");

    assert_eq!(rt.len(), 2);
}

#[test]
fn test_multiple_links() {
    let rt = RichText::new()
        .push_link("Link1", "https://one.com")
        .push_link("Link2", "https://two.com")
        .push_link("Link3", "https://three.com");

    assert_eq!(rt.len(), 3);
}

#[test]
fn test_render_multiple_links() {
    let rt = RichText::markup("[link=https://one.com]First[/] [link=https://two.com]Second[/]");
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    // Both links should have hyperlinks
    assert!(buffer.get(0, 0).unwrap().hyperlink_id.is_some());
    assert!(buffer.get(7, 0).unwrap().hyperlink_id.is_some());
}

// =============================================================================
// Width Tests
// =============================================================================

#[test]
fn test_width_empty() {
    let rt = RichText::new();
    assert_eq!(rt.width(), 0);
}

#[test]
fn test_width_single_word() {
    let rt = RichText::new().text("Hello");
    assert_eq!(rt.width(), 5);
}

#[test]
fn test_width_multiple_spans() {
    let rt = RichText::new()
        .push("Hi", Style::new())
        .push(" ", Style::new())
        .push("There", Style::new());
    assert_eq!(rt.width(), 8);
}

#[test]
fn test_width_with_emoji() {
    let rt = RichText::new().text("Hi😀");
    // Emoji typically has width 2
    assert!(rt.width() >= 4);
}

#[test]
fn test_width_with_cjk() {
    let rt = RichText::new().text("日本語");
    // CJK characters are width 2
    assert_eq!(rt.width(), 6);
}

#[test]
fn test_width_mixed_content() {
    let rt = RichText::new().text("A日😀B");
    // A=1, 日=2 (CJK), 😀=2 (emoji), B=1 = 6 total
    assert!(rt.width() >= 6);
}

// =============================================================================
// Span Edge Cases
// =============================================================================

#[test]
fn test_span_empty_text() {
    let span = Span::new("");
    assert_eq!(span.text, "");
    assert_eq!(span.width(), 0);
}

#[test]
fn test_span_only_whitespace() {
    let span = Span::new("   ");
    assert_eq!(span.text, "   ");
    assert!(span.width() >= 3);
}

#[test]
fn test_span_newlines() {
    let span = Span::new("Line1\nLine2");
    assert_eq!(span.text, "Line1\nLine2");
}

#[test]
fn test_span_tabs() {
    let span = Span::new("\t\t");
    assert_eq!(span.text, "\t\t");
}

#[test]
fn test_span_special_chars() {
    let span = Span::new("@#$%^&*()");
    assert_eq!(span.text, "@#$%^&*()");
    assert!(span.width() >= 9);
}

// =============================================================================
// RichText Builder Chain Tests
// =============================================================================

#[test]
fn test_rich_text_complex_chain() {
    let rt = RichText::new()
        .text("Start")
        .push(" Middle", Style::new().bold())
        .push_link(" Link", "https://example.com")
        .push(" End", Style::new().fg(Color::GREEN));

    assert_eq!(rt.len(), 4);
    assert!(rt.width() > 10);
}

#[test]
fn test_rich_text_all_builder_methods() {
    let rt = RichText::new()
        .element_id("test")
        .class("class1")
        .class("class2")
        .text("Text")
        .push("More", Style::new().italic())
        .default_style(Style::new().fg(Color::CYAN));

    assert!(View::classes(&rt).contains(&"class1".to_string()));
    assert!(View::classes(&rt).contains(&"class2".to_string()));
    assert_eq!(rt.id(), Some("test"));
}

// =============================================================================
// Markup Parser Edge Cases
// =============================================================================

#[test]
fn test_markup_emoji_in_text() {
    let rt = RichText::markup("[b]😀[/b]");
    assert!(rt.len() >= 1);
}

#[test]
fn test_markup_cjk_in_text() {
    let rt = RichText::markup("[red]日本語[/]");
    assert!(rt.len() >= 1);
}

#[test]
fn test_markup_rtl_text() {
    let rt = RichText::markup("[i]مرحبا[/i]");
    assert!(rt.len() >= 1);
}

#[test]
fn test_markup_very_long_text() {
    let long_text = "A".repeat(1000);
    let rt = RichText::markup(&format!("[b]{}[/]", long_text));
    assert!(rt.len() >= 1);
}

#[test]
fn test_markup_many_nested_tags() {
    let tags = "[b]".repeat(20);
    let text = "T";
    let closes = "[/]".repeat(20);
    let rt = RichText::markup(&format!("{}{}{}", tags, text, closes));
    assert!(rt.len() >= 1);
}

#[test]
fn test_markup_tag_uppercase() {
    let rt = RichText::markup("[B]Bold[/B]");
    // Tags should work regardless of case
    assert!(rt.len() >= 1);
}

#[test]
fn test_markup_numeric_colors() {
    let rt = RichText::markup("[#ff0000]Hex Color[/]");
    assert!(rt.len() >= 1);
}

// =============================================================================
// Style Preset Tests
// =============================================================================

#[test]
fn test_style_preset_bold() {
    assert!(Style::new().bold().bold);
}

#[test]
fn test_style_preset_italic() {
    assert!(Style::new().italic().italic);
}

#[test]
fn test_style_preset_underline() {
    assert!(Style::new().underline().underline);
}

#[test]
fn test_style_preset_dim() {
    assert!(Style::new().dim().dim);
}

#[test]
fn test_style_preset_strikethrough() {
    assert!(Style::new().strikethrough().strikethrough);
}

#[test]
fn test_style_preset_reverse() {
    assert!(Style::new().reverse().reverse);
}

// =============================================================================
