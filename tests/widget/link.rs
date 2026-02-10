//! Tests for Link widget
//!
//! Extracted from src/widget/link.rs

use revue::style::Color;
use revue::widget::{link, url_link, Link, LinkStyle};

#[test]
fn test_link_new() {
    let link = Link::new("https://example.com");
    assert_eq!(link.url(), "https://example.com");
    assert_eq!(link.display_text(), "https://example.com");
}

#[test]
fn test_link_with_text() {
    let link = Link::with_text("https://example.com", "Example");
    assert_eq!(link.url(), "https://example.com");
    assert_eq!(link.display_text(), "Example");
}

#[test]
fn test_link_style() {
    let link = Link::new("https://example.com").text("test");

    assert_eq!(
        link.clone().style(LinkStyle::Bracketed).format_display(),
        "[test]"
    );
    assert_eq!(
        link.clone().style(LinkStyle::Arrow).format_display(),
        "test →"
    );
    assert_eq!(
        link.clone().style(LinkStyle::Icon).format_display(),
        "🔗 test"
    );
}

#[test]
fn test_link_focused() {
    let link = Link::new("https://example.com").focused(true);
    assert!(link.is_focused());
}

#[test]
fn test_link_disabled() {
    let link = Link::new("https://example.com").disabled(true);
    assert!(link.is_disabled());
}

#[test]
fn test_link_osc8() {
    let link = Link::new("https://example.com").osc8(true);
    let start = link.osc8_start();
    assert!(start.contains("https://example.com"));
    assert!(start.starts_with("\x1b]8;;"));
}

#[test]
fn test_link_osc8_disabled_link() {
    let link = Link::new("https://example.com").disabled(true);
    assert!(link.osc8_start().is_empty());
}

#[test]
fn test_helper_functions() {
    let l = link("https://example.com", "Example");
    assert_eq!(l.display_text(), "Example");

    let u = url_link("https://example.com");
    assert_eq!(u.display_text(), "https://example.com");
}

#[test]
fn test_link_tooltip() {
    let link = Link::new("https://example.com").tooltip("Click to visit");
    assert_eq!(link.get_tooltip(), &Some("Click to visit".to_string()));
}

#[test]
fn test_link_style_default() {
    let style = LinkStyle::default();
    assert_eq!(style, LinkStyle::Underline);
}

#[test]
fn test_link_style_clone() {
    let style = LinkStyle::Arrow;
    let cloned = style;
    assert_eq!(style, cloned);
}

#[test]
fn test_link_style_copy() {
    let style1 = LinkStyle::Icon;
    let style2 = style1;
    assert_eq!(style1, LinkStyle::Icon);
    assert_eq!(style2, LinkStyle::Icon);
}

#[test]
fn test_link_style_partial_eq() {
    assert_eq!(LinkStyle::Underline, LinkStyle::Underline);
    assert_ne!(LinkStyle::Underline, LinkStyle::Bracketed);
}

#[test]
fn test_link_style_format_underline() {
    let link = Link::new("url").text("test").style(LinkStyle::Underline);
    assert_eq!(link.format_display(), "test");
}

#[test]
fn test_link_style_format_plain() {
    let link = Link::new("url").text("test").style(LinkStyle::Plain);
    assert_eq!(link.format_display(), "test");
}

#[test]
fn test_link_text_builder() {
    let link = Link::new("https://example.com").text("Custom Text");
    assert_eq!(link.display_text(), "Custom Text");
    assert_eq!(link.url(), "https://example.com");
}

#[test]
fn test_link_text_overrides() {
    let link = Link::with_text("https://example.com", "First").text("Second");
    assert_eq!(link.display_text(), "Second");
}

#[test]
fn test_link_fg() {
    let link = Link::new("url").fg(Color::RED);
    assert_eq!(link.get_fg(), Some(Color::RED));
}

#[test]
fn test_link_fg_none() {
    let link = Link::new("url");
    assert!(link.get_fg().is_none());
}

#[test]
fn test_link_bg() {
    let link = Link::new("url").bg(Color::BLUE);
    assert_eq!(link.get_bg(), Some(Color::BLUE));
}

#[test]
fn test_link_bg_none() {
    let link = Link::new("url");
    assert!(link.get_bg().is_none());
}

#[test]
fn test_link_colors_combined() {
    let link = Link::new("url").fg(Color::GREEN).bg(Color::BLACK);
    assert_eq!(link.get_fg(), Some(Color::GREEN));
    assert_eq!(link.get_bg(), Some(Color::BLACK));
}

#[test]
fn test_link_focused_builder() {
    let link = Link::new("url").focused(true);
    assert!(link.is_focused());
}

#[test]
fn test_link_not_focused() {
    let link = Link::new("url").focused(false);
    assert!(!link.is_focused());
}

#[test]
fn test_link_disabled_builder() {
    let link = Link::new("url").disabled(true);
    assert!(link.is_disabled());
}

#[test]
fn test_link_not_disabled() {
    let link = Link::new("url").disabled(false);
    assert!(!link.is_disabled());
}

#[test]
fn test_link_osc8_disabled() {
    let link = Link::new("url").osc8(false);
    assert!(link.osc8_start().is_empty());
    assert!(link.osc8_end().is_empty());
}

#[test]
fn test_link_osc8_enabled() {
    let link = Link::new("url").osc8(true);
    assert!(!link.osc8_start().is_empty());
    assert!(!link.osc8_end().is_empty());
}

#[test]
fn test_link_url() {
    let link = Link::new("https://example.com/path");
    assert_eq!(link.url(), "https://example.com/path");
}

#[test]
fn test_link_display_text_with_text_set() {
    let link = Link::new("url").text("Custom");
    assert_eq!(link.display_text(), "Custom");
}

#[test]
fn test_link_display_text_fallback_to_url() {
    let link = Link::new("https://example.com");
    assert_eq!(link.display_text(), "https://example.com");
}

#[test]
fn test_link_is_focused() {
    let link = Link::new("url").focused(true);
    assert!(link.is_focused());
}

#[test]
fn test_link_is_disabled() {
    let link = Link::new("url").disabled(true);
    assert!(link.is_disabled());
}

#[test]
fn test_link_clone() {
    let link1 = Link::new("url")
        .text("text")
        .fg(Color::RED)
        .bg(Color::BLUE)
        .focused(true)
        .disabled(false);
    let link2 = link1.clone();

    assert_eq!(link1.url(), link2.url());
    assert_eq!(link1.display_text(), link2.display_text());
    assert_eq!(link1.get_fg(), link2.get_fg());
    assert_eq!(link1.get_bg(), link2.get_bg());
    assert_eq!(link1.is_focused(), link2.is_focused());
    assert_eq!(link1.is_disabled(), link2.is_disabled());
}

#[test]
fn test_link_default_style() {
    let link = Link::new("url");
    assert_eq!(link.get_style(), LinkStyle::default());
}

#[test]
fn test_link_osc8_end_when_enabled() {
    let link = Link::new("url").osc8(true);
    assert_eq!(link.osc8_end(), "\x1b]8;;\x1b\\");
}

#[test]
fn test_link_osc8_end_when_disabled() {
    let link = Link::new("url").osc8(false);
    assert!(link.osc8_end().is_empty());
}

#[test]
fn test_link_osc8_end_when_link_disabled() {
    let link = Link::new("url").disabled(true);
    assert!(link.osc8_end().is_empty());
}

#[test]
fn test_format_display_empty_text() {
    let link = Link::new("url").text("");
    assert_eq!(link.format_display(), "");
}

#[test]
fn test_format_display_unicode() {
    let link = Link::new("url")
        .text("Hello 世界")
        .style(LinkStyle::Bracketed);
    assert_eq!(link.format_display(), "[Hello 世界]");
}

#[test]
fn test_url_link_helper() {
    let link = url_link("https://example.com");
    assert_eq!(link.url(), "https://example.com");
    assert_eq!(link.display_text(), "https://example.com");
}

#[test]
fn test_link_helper_with_text() {
    let link = link("https://example.com", "Click Here");
    assert_eq!(link.url(), "https://example.com");
    assert_eq!(link.display_text(), "Click Here");
}

#[test]
fn test_link_builder_chain() {
    let link = Link::new("https://example.com")
        .text("Example")
        .style(LinkStyle::Arrow)
        .fg(Color::CYAN)
        .bg(Color::BLACK)
        .focused(true)
        .disabled(false)
        .tooltip("Hover me")
        .osc8(true);

    assert_eq!(link.url(), "https://example.com");
    assert_eq!(link.display_text(), "Example");
    assert_eq!(link.get_style(), LinkStyle::Arrow);
    assert_eq!(link.get_fg(), Some(Color::CYAN));
    assert_eq!(link.get_bg(), Some(Color::BLACK));
    assert!(link.is_focused());
    assert!(!link.is_disabled());
    assert_eq!(link.get_tooltip(), &Some("Hover me".to_string()));
}

#[test]
fn test_link_debug() {
    let link = Link::new("url");
    let debug_str = format!("{:?}", link);
    assert!(debug_str.contains("url"));
}
