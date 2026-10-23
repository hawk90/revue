//! Rich text widget integration tests
//!
//! Tests for markup parsing, colors, links, span width, and rendering.

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{markup, rich_text, span, style, RichText, Span, Style};

mod view;

/// `rt` drawn into a one-row area `width` wide
fn draw(rt: &RichText, width: u16) -> Buffer {
    let mut buffer = Buffer::new(width, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, 1));
    rt.render(&mut ctx);
    buffer
}

// =============================================================================
// Markup Parsing Edge Cases Tests
// =============================================================================

#[test]
fn test_markup_unclosed_tag() {
    let rt = RichText::markup("[bold]Unclosed text");

    // Should treat unclosed tag as applying to rest
    assert!(!rt.is_empty());
    assert_eq!(rt.width(), 13); // "Unclosed text" = 13 chars
}

#[test]
fn test_markup_mismatched_tags() {
    let rt = RichText::markup("[bold][/italic]Text");

    // The closing tag names a style that is not on; the text stays bold
    let buffer = draw(&rt, 6);
    assert_eq!(row(&buffer, 0, 6), "Text");
    assert!(buffer.get(3, 0).unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_markup_empty_tag() {
    // Brackets with nothing in them are not a tag: they are text
    for (markup, shown) in [("[]Text", "[]Text"), ("[ ]Text", "[ ]Text")] {
        let rt = RichText::markup(markup);
        assert_eq!(row(&draw(&rt, 8), 0, 8), shown);
    }
}

#[test]
fn test_markup_nested_tags() {
    let rt = RichText::markup("[b][i]Nested[/i][/b]");

    // Should apply both styles
    assert_eq!(rt.width(), 6);
    let cell = *draw(&rt, 8).get(0, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::BOLD | Modifier::ITALIC));
}

#[test]
fn test_markup_deep_nesting() {
    let tags = "[b]".repeat(10);
    let closing = "[/]".repeat(10);
    let rt = RichText::markup(&format!("{}Text{}", tags, closing));

    // Should handle deep nesting
    assert!(!rt.is_empty());
    assert_eq!(rt.width(), 4);
}

#[test]
fn test_markup_adjacent_tags() {
    let rt = RichText::markup("[b]A[/b][i]B[/i]");

    // Each closing tag ends its own style: A is only bold, B only italic
    assert_eq!(rt.len(), 2);
    let buffer = draw(&rt, 3);
    assert_eq!(buffer.get(0, 0).unwrap().modifier, Modifier::BOLD);
    assert_eq!(buffer.get(1, 0).unwrap().modifier, Modifier::ITALIC);
}

#[test]
fn test_markup_tag_without_content() {
    let rt = RichText::markup("[b][/]");

    // Tags alone make no span
    assert!(rt.is_empty());
    assert_eq!(rt.width(), 0);
}

#[test]
fn test_markup_multiple_reset_tags() {
    let rt = RichText::markup("[b]Text1[/]Text2[/]Text3");

    assert_eq!(rt.len(), 3);
    let buffer = draw(&rt, 16);
    assert_eq!(row(&buffer, 0, 16), "Text1Text2Text3");
    assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
    // After a reset nothing is bold, and a second reset keeps it so
    assert!(buffer.get(5, 0).unwrap().modifier.is_empty());
    assert!(buffer.get(10, 0).unwrap().modifier.is_empty());
}

#[test]
fn test_markup_whitespace_in_tags() {
    let rt = RichText::markup("[bold ]Text[/ ]");

    // A trailing space inside a tag is ignored
    let buffer = draw(&rt, 6);
    assert_eq!(row(&buffer, 0, 6), "Text");
    assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
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
    for (name, expected) in [
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
        assert_eq!(rt.width(), 4);
        let cell = *draw(&rt, 6).get(0, 0).unwrap();
        assert_eq!((cell.fg, cell.bg), (Some(expected), None), "{name}");
    }
}

#[test]
fn test_markup_all_background_colors() {
    for (name, expected) in [
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
        assert_eq!(rt.width(), 4);
        let cell = *draw(&rt, 6).get(3, 0).unwrap();
        assert_eq!((cell.fg, cell.bg), (None, Some(expected)), "{name}");
    }
}

#[test]
fn test_markup_combined_colors() {
    let rt = RichText::markup("[red on_blue]Text[/]");

    assert_eq!(rt.width(), 4);
    let cell = *draw(&rt, 6).get(0, 0).unwrap();
    assert_eq!((cell.fg, cell.bg), (Some(Color::RED), Some(Color::BLUE)));
}

// =============================================================================
// Link Markup Tests
// =============================================================================

#[test]
fn test_markup_link_with_formatting() {
    let rt = RichText::markup("[link=https://example.com][b]Click[/b][/]");

    let buffer = draw(&rt, 8);
    assert_eq!(row(&buffer, 0, 8), "Click");
    let cell = buffer.get(4, 0).unwrap();
    let id = cell.hyperlink_id.expect("linked");
    assert_eq!(buffer.get_hyperlink(id), Some("https://example.com"));
    // A link is cyan and underlined unless told otherwise, and keeps the bold
    assert_eq!(cell.fg, Some(Color::CYAN));
    assert!(cell.modifier.contains(Modifier::BOLD | Modifier::UNDERLINE));
}

#[test]
fn test_markup_link_empty_url() {
    let rt = RichText::markup("[link=]Text[/]");

    // Should handle empty URL
    assert_eq!(rt.len(), 1);
    assert_eq!(row(&draw(&rt, 6), 0, 6), "Text");
}

#[test]
fn test_markup_link_special_chars() {
    let rt = RichText::markup("[link=https://example.com?param=value&other=test]Link[/]");

    let buffer = draw(&rt, 6);
    let id = buffer.get(0, 0).unwrap().hyperlink_id.expect("linked");
    assert_eq!(
        buffer.get_hyperlink(id),
        Some("https://example.com?param=value&other=test")
    );
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

    // Emoji are width 2
    assert_eq!(span.width(), 4);
}

#[test]
fn test_span_width_combining() {
    let span = Span::new("café"); // With combining accent

    // Should count correctly
    assert_eq!(span.width(), 4);
}

#[test]
fn test_span_width_mixed() {
    let span = Span::new("Hi 日本語 😀");

    // 2 + 1 + 6 + 1 + 2
    assert_eq!(span.width(), 12);
}

#[test]
fn test_span_width_ascii() {
    let span = Span::new("Hello");

    assert_eq!(span.width(), 5);
}

#[test]
fn test_span_width_tabs() {
    // A tab is drawn as nothing (#1198), so it takes no column either
    assert_eq!(Span::new("\t").width(), 0);
    assert_eq!(Span::new("a\tb").width(), 2);
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

    // First wide char at 0, continuation at 1
    // Second wide char at 2, continuation at 3
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '日');
    assert!(buffer.get(1, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '本');
    assert!(buffer.get(3, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(4, 0).unwrap().symbol, ' ');
}

#[test]
fn test_render_zero_area() {
    let rt = RichText::new().text("Test");

    let mut buffer = Buffer::new(4, 2);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    assert_eq!(row(&buffer, 0, 4), "");
    assert_eq!(row(&buffer, 1, 4), "");
}

#[test]
fn test_render_truncation() {
    let rt = RichText::new().text("This is very long text that exceeds area");

    let mut buffer = Buffer::new(12, 2);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);

    // Should truncate at area width, without wrapping
    assert_eq!(row(&buffer, 0, 12), "This is ve");
    assert_eq!(row(&buffer, 1, 12), "");
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
// Newline Tests
// =============================================================================

fn row(buffer: &Buffer, y: u16, width: u16) -> String {
    (0..width)
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

#[test]
fn test_render_newline_inside_span_starts_new_row() {
    let rt = RichText::new().text("ab\ncd");

    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    rt.render(&mut ctx);

    assert_eq!(row(&buffer, 0, 10), "ab");
    assert_eq!(row(&buffer, 1, 10), "cd");
}

#[test]
fn test_render_newline_span_keeps_following_styles() {
    // Same shape as examples/dashboard.rs render_json_syntax: styled
    // pieces with a separate "\n" span after each line.
    let rt = RichText::new()
        .push("{", Style::new().fg(Color::WHITE))
        .push("\n", Style::new())
        .push("  ", Style::new())
        .push("\"k\"", Style::new().fg(Color::BLUE))
        .push(": 1", Style::new().fg(Color::GREEN))
        .push("\n", Style::new())
        .push("}", Style::new().fg(Color::WHITE));

    let mut buffer = Buffer::new(12, 4);
    let area = Rect::new(0, 0, 12, 4);
    let mut ctx = RenderContext::new(&mut buffer, area);
    rt.render(&mut ctx);

    assert_eq!(row(&buffer, 0, 12), "{");
    assert_eq!(row(&buffer, 1, 12), "  \"k\": 1");
    assert_eq!(row(&buffer, 2, 12), "}");
    assert_eq!(buffer.get(2, 1).unwrap().fg, Some(Color::BLUE));
    assert_eq!(buffer.get(5, 1).unwrap().fg, Some(Color::GREEN));
    assert_eq!(buffer.get(0, 2).unwrap().fg, Some(Color::WHITE));
}

#[test]
fn test_render_newline_continues_span_style_on_next_row() {
    let rt = RichText::new().push("one\ntwo", Style::new().fg(Color::RED).bold());

    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);
    rt.render(&mut ctx);

    assert_eq!(row(&buffer, 1, 10), "two");
    let cell = buffer.get(0, 1).unwrap();
    assert_eq!(cell.fg, Some(Color::RED));
    assert!(cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_render_newline_starts_at_area_left_edge() {
    let rt = RichText::new().text("ab\ncd");

    let mut buffer = Buffer::new(10, 4);
    let area = Rect::new(3, 1, 5, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);
    rt.render(&mut ctx);

    assert_eq!(buffer.get(3, 1).unwrap().symbol, 'a');
    assert_eq!(buffer.get(3, 2).unwrap().symbol, 'c');
    assert_eq!(buffer.get(4, 2).unwrap().symbol, 'd');
}

#[test]
fn test_render_newline_clips_at_area_height() {
    let rt = RichText::new().text("1\n2\n3\n4");

    let mut buffer = Buffer::new(5, 4);
    let area = Rect::new(0, 0, 5, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);
    rt.render(&mut ctx);

    assert_eq!(row(&buffer, 0, 5), "1");
    assert_eq!(row(&buffer, 1, 5), "2");
    // Nothing spills below the area
    assert_eq!(row(&buffer, 2, 5), "");
    assert_eq!(row(&buffer, 3, 5), "");
}

#[test]
fn test_render_long_line_truncates_but_next_line_still_renders() {
    let rt = RichText::new().text("abcdefgh\nxy");

    let mut buffer = Buffer::new(4, 2);
    let area = Rect::new(0, 0, 4, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);
    rt.render(&mut ctx);

    assert_eq!(row(&buffer, 0, 4), "abcd");
    assert_eq!(row(&buffer, 1, 4), "xy");
}

#[test]
fn test_rich_text_width_is_widest_line() {
    let rt = RichText::new()
        .push("abc", Style::new())
        .push("\n", Style::new())
        .push("defgh", Style::new())
        .push("\nij", Style::new());

    assert_eq!(rt.width(), 5);
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

    assert!(!rt.is_empty());
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

    let meta = rt.meta();
    assert_eq!(meta.widget_type, "RichText");
    assert_eq!(meta.id, None);
    assert!(meta.classes.is_empty());
}

#[test]
fn test_rich_text_props_builders() {
    let rt = RichText::new().element_id("my-text").class("text-class");

    assert_eq!(rt.id(), Some("my-text"));
    assert!(View::classes(&rt).contains(&"text-class".to_string()));
}

#[test]
fn test_markup_short_tags() {
    let rt = RichText::markup("[b]Bold[/b][i]Italic[/i][u]Underline[/u] end");

    assert_eq!(rt.len(), 4);
    let buffer = draw(&rt, 24);
    for (x, want) in [
        (0, Modifier::BOLD),
        (4, Modifier::ITALIC),
        (10, Modifier::UNDERLINE),
        (20, Modifier::empty()),
    ] {
        assert_eq!(buffer.get(x, 0).unwrap().modifier, want, "x {x}");
    }
}

#[test]
fn test_markup_multiple_attributes() {
    let rt = RichText::markup("[bold red underline]Text[/]");

    let cell = *draw(&rt, 6).get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::RED));
    assert_eq!(cell.modifier, Modifier::BOLD | Modifier::UNDERLINE);
}

#[test]
fn test_markup_dim_modifier() {
    let rt = RichText::markup("[dim]Text[/]");

    assert_eq!(draw(&rt, 6).get(0, 0).unwrap().modifier, Modifier::DIM);
}

#[test]
fn test_markup_strike_modifier() {
    let rt = RichText::markup("[s]Text[/]");

    assert_eq!(
        draw(&rt, 6).get(0, 0).unwrap().modifier,
        Modifier::CROSSED_OUT
    );
}

#[test]
fn test_markup_reverse_modifier() {
    let rt = RichText::markup("[rev]Text[/]");

    assert_eq!(draw(&rt, 6).get(0, 0).unwrap().modifier, Modifier::REVERSE);
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
    let rt = RichText::new()
        .text("ab")
        .default_style(Style::new().fg(Color::CYAN));

    // Spans without a color take the default style's
    let buffer = draw(&rt, 4);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
    assert_eq!(buffer.get(1, 0).unwrap().fg, Some(Color::CYAN));
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
    let mut span2 = span1.clone();

    // Modifying one should not affect the other
    span2.text.push('!');
    span2.style.bold = false;
    assert_eq!(span1.text, "Test");
    assert!(span1.style.bold);
    assert_eq!(span2.text, "Test!");
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
    assert_eq!(rt.widget_type(), "RichText");
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
    // Each emoji takes two columns
    for (x, emoji) in [(0, '😀'), (2, '😃'), (4, '😄')] {
        assert_eq!(buffer.get(x, 0).unwrap().symbol, emoji);
        assert!(buffer.get(x + 1, 0).unwrap().is_continuation());
    }
    assert_eq!(buffer.get(6, 0).unwrap().symbol, ' ');
}

#[test]
fn test_render_empty_string() {
    let rt = RichText::new().text("");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);
    assert_eq!(row(&buffer, 0, 10), "");
    assert_eq!(rt.measure(10, 10), Some((0, 1)));
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
    let rt = RichText::new().text("AB");
    let mut buffer = Buffer::new(2, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);
    assert_eq!(row(&buffer, 0, 2), "A");
}

#[test]
fn test_render_zero_height() {
    let rt = RichText::new().text("Test");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);
    assert_eq!(row(&buffer, 0, 10), "");
}

#[test]
fn test_render_newline_in_text() {
    let rt = RichText::new().text("Line1\nLine2");
    let mut buffer = Buffer::new(20, 2);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);
    assert_eq!(row(&buffer, 0, 20), "Line1");
    assert_eq!(row(&buffer, 1, 20), "Line2");
    assert_eq!(rt.measure(20, 20), Some((5, 2)));
}

#[test]
fn test_render_tabs_in_text() {
    let rt = RichText::new().text("Item\tTabbed");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rt.render(&mut ctx);
    // A tab is a control character, so it takes no cell, as in `draw_text`
    // (#1198). Expanding tabs would be a rule for every text path (#982).
    assert_eq!(row(&buffer, 0, 20), "ItemTabbed");
    assert_eq!(rt.measure(20, 20), Some((10, 1)));
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
    // Emoji has width 2
    assert_eq!(rt.width(), 4);
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
    assert_eq!(rt.width(), 6);
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
    assert_eq!(span.width(), 3);
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
    assert_eq!(span.width(), 9);
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
    let buffer = draw(&rt, 4);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, '😀');
    assert!(cell.modifier.contains(Modifier::BOLD));
    assert!(buffer.get(1, 0).unwrap().is_continuation());
}

#[test]
fn test_markup_cjk_in_text() {
    let rt = RichText::markup("[red]日本語[/]");
    assert_eq!(rt.width(), 6);
    let buffer = draw(&rt, 8);
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '語');
    assert_eq!(buffer.get(4, 0).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_markup_rtl_text() {
    let rt = RichText::markup("[i]مرحبا[/i]");
    assert_eq!(rt.width(), 5);
    let cell = *draw(&rt, 6).get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'م');
    assert!(cell.modifier.contains(Modifier::ITALIC));
}

#[test]
fn test_markup_very_long_text() {
    let long_text = "A".repeat(1000);
    let rt = RichText::markup(&format!("[b]{}[/]", long_text));
    assert_eq!(rt.len(), 1);
    assert_eq!(rt.width(), 1000);
}

#[test]
fn test_markup_many_nested_tags() {
    let tags = "[b]".repeat(20);
    let text = "T";
    let closes = "[/]".repeat(20);
    let rt = RichText::markup(&format!("{}{}{}", tags, text, closes));
    assert_eq!(rt.len(), 1);
    let cell = *draw(&rt, 2).get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'T');
    assert_eq!(cell.modifier, Modifier::BOLD);
}

#[test]
fn test_markup_tag_uppercase() {
    let rt = RichText::markup("[B]Bold[/B]");
    // Tags should work regardless of case
    assert!(draw(&rt, 6)
        .get(0, 0)
        .unwrap()
        .modifier
        .contains(Modifier::BOLD));
}

#[test]
fn test_markup_numeric_colors() {
    let rt = RichText::markup("[#ff0000]Hex Color[/]");
    // Only named colors are tags; the text is still drawn
    assert_eq!(row(&draw(&rt, 12), 0, 12), "Hex Color");
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

// Closing tags, brackets that are not tags, and background and hex colors
// (#1069, #1203, #1276)

/// Modifier, fg and bg of the cell at `x`
fn look(buffer: &Buffer, x: u16) -> (Modifier, Option<Color>, Option<Color>) {
    let c = buffer.get(x, 0).unwrap();
    (c.modifier, c.fg, c.bg)
}

#[test]
fn a_named_closing_tag_ends_its_style() {
    let buffer = draw(&RichText::markup("[b]A[/b] B [red]x[/red] y"), 12);
    assert_eq!(row(&buffer, 0, 12), "A B x y");
    assert_eq!(look(&buffer, 0), (Modifier::BOLD, None, None));
    assert_eq!(look(&buffer, 2), (Modifier::empty(), None, None));
    assert_eq!(
        look(&buffer, 4),
        (Modifier::empty(), Some(Color::RED), None)
    );
    assert_eq!(look(&buffer, 6), (Modifier::empty(), None, None));
}

#[test]
fn a_closing_tag_turns_off_only_what_it_names() {
    // From a tag of several words, and from a tag under a newer one
    let buffer = draw(&RichText::markup("[bold red]a[/red]b[/bold]c"), 4);
    assert_eq!(look(&buffer, 0), (Modifier::BOLD, Some(Color::RED), None));
    assert_eq!(look(&buffer, 1), (Modifier::BOLD, None, None));
    assert_eq!(look(&buffer, 2), (Modifier::empty(), None, None));

    let buffer = draw(&RichText::markup("[b]A[i]B[/b]C[/i]D"), 5);
    assert_eq!(buffer.get(0, 0).unwrap().modifier, Modifier::BOLD);
    assert_eq!(
        buffer.get(1, 0).unwrap().modifier,
        Modifier::BOLD | Modifier::ITALIC
    );
    assert_eq!(buffer.get(2, 0).unwrap().modifier, Modifier::ITALIC);
    assert_eq!(buffer.get(3, 0).unwrap().modifier, Modifier::empty());

    // Nested alike: the inner one closes first, and the outer one is still on
    let buffer = draw(&RichText::markup("[b]a[bold]b[/b]c[/bold]d"), 5);
    for (x, bold) in [(0, true), (1, true), (2, true), (3, false)] {
        assert_eq!(
            buffer.get(x, 0).unwrap().modifier.contains(Modifier::BOLD),
            bold,
            "x {x}"
        );
    }
}

#[test]
fn closing_a_link_keeps_an_underline_turned_on_apart() {
    let buffer = draw(&RichText::markup("[u][link=https://x.example]a[/link]b"), 3);
    assert!(buffer.get(0, 0).unwrap().hyperlink_id.is_some());
    let b = buffer.get(1, 0).unwrap();
    assert_eq!(
        (b.hyperlink_id, b.fg, b.modifier),
        (None, None, Modifier::UNDERLINE)
    );
}

#[test]
fn brackets_that_are_not_a_tag_are_text() {
    for (markup, shown) in [
        ("[x] done", "[x] done"),
        ("[orange-ish]a", "[orange-ish]a"),
        ("[bold on]a", "[bold on]a"),
        ("[on nonsense]a", "[on nonsense]a"),
        ("[rgb(1,2,3)]a", "[rgb(1,2,3)]a"),
        ("a[/nonsense]b", "a[/nonsense]b"),
        ("[link]a", "[link]a"),
    ] {
        let buffer = draw(&RichText::markup(markup), 20);
        assert_eq!(row(&buffer, 0, 20), shown, "{markup:?}");
        assert_eq!(
            look(&buffer, 0),
            (Modifier::empty(), None, None),
            "{markup:?}"
        );
    }
    // A `[` before a tag is text, and the tag still applies
    let buffer = draw(&RichText::markup("[a[b]x"), 4);
    assert_eq!(row(&buffer, 0, 4), "[ax");
    assert_eq!(buffer.get(2, 0).unwrap().modifier, Modifier::BOLD);
    // Text that is not a tag takes the style in effect
    let buffer = draw(&RichText::markup("[b][x]"), 4);
    assert_eq!(buffer.get(1, 0).unwrap().modifier, Modifier::BOLD);
}

#[test]
fn backgrounds_and_hex_colors() {
    let navy = Color::rgb(0x00, 0x33, 0x66);
    for (markup, fg, bg) in [
        ("[on blue]a", None, Some(Color::BLUE)),
        ("[on_blue]a", None, Some(Color::BLUE)),
        ("[ON #003366]a", None, Some(navy)),
        ("[#003366]a", Some(navy), None),
        ("[#f00]a", Some(Color::RED), None),
        (
            "[orange on navy]a",
            Some(Color::rgb(255, 165, 0)),
            Some(Color::rgb(0, 0, 128)),
        ),
    ] {
        let buffer = draw(&RichText::markup(markup), 2);
        assert_eq!(look(&buffer, 0), (Modifier::empty(), fg, bg), "{markup:?}");
    }
    let buffer = draw(&RichText::markup("[red on blue]a[/on blue]b"), 2);
    assert_eq!(
        look(&buffer, 1),
        (Modifier::empty(), Some(Color::RED), None)
    );
}

// Found by review

#[test]
fn a_word_given_twice_in_a_tag_is_closed_once() {
    let buffer = draw(&RichText::markup("[bold bold]x[/bold]y"), 3);
    assert_eq!(buffer.get(1, 0).unwrap().modifier, Modifier::empty());
}

#[test]
fn transparent_is_not_a_markup_color() {
    // There is no see-through text: the brackets are shown as they are
    let buffer = draw(&RichText::markup("[transparent]a"), 16);
    assert_eq!(row(&buffer, 0, 16), "[transparent]a");
}

#[test]
fn link_is_read_in_any_case() {
    let buffer = draw(&RichText::markup("[LINK=https://x.example]a[/LINK]b"), 3);
    assert!(buffer.get(0, 0).unwrap().hyperlink_id.is_some());
    assert!(buffer.get(1, 0).unwrap().hyperlink_id.is_none());
}

#[test]
fn colors_closed_oldest_first_keep_the_newest_in_effect() {
    let fg = |markup: &str, x: u16| draw(&RichText::markup(markup), 6).get(x, 0).unwrap().fg;
    let markup = "[red]a[blue]b[/red]c[/blue]d";
    assert_eq!(fg(markup, 0), Some(Color::RED));
    assert_eq!(fg(markup, 1), Some(Color::BLUE));
    // Red closed under blue: blue stays
    assert_eq!(fg(markup, 2), Some(Color::BLUE));
    assert_eq!(fg(markup, 3), None);
    // A red closed under blue is not back when a newer red closes
    let markup = "[red]a[blue]b[/red]c[red]d[/red]e";
    assert_eq!(fg(markup, 3), Some(Color::RED));
    assert_eq!(fg(markup, 4), Some(Color::BLUE));
    // The same color opened twice: one close leaves it on
    assert_eq!(fg("[red]a[red]b[/red]c", 2), Some(Color::RED));
}

#[test]
fn a_link_url_from_markup_cannot_carry_terminal_controls() {
    let buffer = draw(&RichText::markup("[link=x\x1bc]a[/link]"), 2);
    let id = buffer.get(0, 0).unwrap().hyperlink_id.unwrap();
    assert_eq!(buffer.get_hyperlink(id), Some("x%1Bc"));
}
