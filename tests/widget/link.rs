//! Tests for Link widget
//!
//! Rendering is checked with OSC 8 off: the hyperlink itself is checked at
//! the terminal-output level in tests/osc8_hyperlinks.rs.

use revue::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::utils::browser::NO_LAUNCH_ENV;
use revue::widget::traits::RenderContext;
use revue::widget::{link, url_link, EventResult, Interactive, Link, LinkStyle, View};
use serial_test::serial;

/// Render `link` (with OSC 8 off) into a one-row buffer.
fn render(link: &Link, width: u16) -> Buffer {
    let mut buffer = Buffer::new(width, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, 1));
    link.clone().osc8(false).render(&mut ctx);
    buffer
}

/// The rendered text, wide characters once, trailing blanks trimmed.
fn rendered(link: &Link, width: u16) -> String {
    let buffer = render(link, width);
    (0..width)
        .filter_map(|x| buffer.get(x, 0))
        .filter(|c| !c.is_continuation())
        .map(|c| c.symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

fn left_click() -> MouseEvent {
    MouseEvent::new(0, 0, MouseEventKind::Down(MouseButton::Left))
}

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
        rendered(&link.clone().style(LinkStyle::Bracketed), 20),
        "[test]"
    );
    assert_eq!(
        rendered(&link.clone().style(LinkStyle::Arrow), 20),
        "test →"
    );
    assert_eq!(
        rendered(&link.clone().style(LinkStyle::Icon), 20),
        "🔗 test"
    );
}

#[test]
fn test_link_focused() {
    let link = Link::new("https://example.com").focused(true);
    assert!(link.is_focused());
    assert!(!Link::new("url").focused(false).is_focused());
    // Focus overrides the link color
    let link = Link::new("url").fg(Color::RED).focused(true);
    assert_eq!(
        render(&link, 5).get(0, 0).unwrap().fg,
        Some(Color::rgb(100, 200, 255))
    );
}

#[test]
fn test_link_disabled() {
    let link = Link::new("https://example.com").disabled(true);
    assert!(link.is_disabled());
    assert!(!Link::new("url").disabled(false).is_disabled());
}

#[test]
fn test_link_disabled_render() {
    // Grayed out and not underlined, whatever its color
    let link = Link::new("url").text("off").fg(Color::RED).disabled(true);
    assert_eq!(rendered(&link, 5), "off");
    let cell = *render(&link, 5).get(0, 0).unwrap();
    assert_ne!(cell.fg, Some(Color::RED));
    assert_ne!(cell.fg, Some(Color::CYAN));
    assert!(!cell.modifier.contains(Modifier::UNDERLINE));
}

#[test]
fn test_link_osc8_flag() {
    assert!(Link::new("url").get_osc8());
    assert!(!Link::new("url").osc8(false).get_osc8());
    assert!(Link::new("url").osc8(false).osc8(true).get_osc8());
}

#[test]
fn test_link_render_clipped() {
    let link = Link::new("https://example.com/a/long/path");
    assert_eq!(rendered(&link, 8), "https://");
}

#[test]
fn test_link_space_does_not_activate() {
    let mut link = Link::new("https://example.com").focused(true);
    assert_eq!(
        Interactive::handle_key(&mut link, &KeyEvent::new(Key::Char(' '))),
        EventResult::Ignored
    );
    assert_eq!(
        Interactive::handle_key(&mut link, &KeyEvent::new(Key::Char('x'))),
        EventResult::Ignored
    );
}

#[test]
#[serial]
fn test_link_left_click_activates() {
    std::env::set_var(NO_LAUNCH_ENV, "1");
    let mut link = Link::new("https://example.com");
    let result = Interactive::handle_mouse(&mut link, &left_click(), Rect::new(0, 0, 10, 1));
    std::env::remove_var(NO_LAUNCH_ENV);
    assert_eq!(result, EventResult::ConsumedAndRender);
}

#[test]
fn test_link_other_mouse_events_ignored() {
    let mut link = Link::new("https://example.com");
    let area = Rect::new(0, 0, 10, 1);
    for kind in [
        MouseEventKind::Down(MouseButton::Right),
        MouseEventKind::Up(MouseButton::Left),
        MouseEventKind::ScrollDown,
    ] {
        let event = MouseEvent::new(0, 0, kind);
        assert_eq!(
            Interactive::handle_mouse(&mut link, &event, area),
            EventResult::Ignored,
            "{kind:?}"
        );
    }
}

#[test]
fn test_link_disabled_ignores_click() {
    // Disabled links never reach the browser, so no guard is needed here
    let mut link = Link::new("https://example.com").disabled(true);
    assert_eq!(
        Interactive::handle_mouse(&mut link, &left_click(), Rect::new(0, 0, 10, 1)),
        EventResult::Ignored
    );
    assert!(link.open().is_ok());
}

#[test]
fn test_link_focus_handlers() {
    let mut link = Link::new("url");
    assert!(Interactive::focusable(&link));
    link.on_focus();
    assert!(link.is_focused());
    link.on_blur();
    assert!(!link.is_focused());
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
    assert_eq!(rendered(&link, 10), "test");
    let buffer = render(&link, 10);
    assert!(buffer
        .get(0, 0)
        .unwrap()
        .modifier
        .contains(Modifier::UNDERLINE));
}

#[test]
fn test_link_style_format_plain() {
    let link = Link::new("url").text("test").style(LinkStyle::Plain);
    assert_eq!(rendered(&link, 10), "test");
    let buffer = render(&link, 10);
    assert!(!buffer
        .get(0, 0)
        .unwrap()
        .modifier
        .contains(Modifier::UNDERLINE));
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
    assert_eq!(render(&link, 5).get(0, 0).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_link_fg_none() {
    let link = Link::new("url");
    assert!(link.get_fg().is_none());
    // Links are cyan unless told otherwise
    assert_eq!(render(&link, 5).get(0, 0).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_link_bg() {
    let link = Link::new("url").bg(Color::BLUE);
    assert_eq!(link.get_bg(), Some(Color::BLUE));
    assert_eq!(render(&link, 5).get(0, 0).unwrap().bg, Some(Color::BLUE));
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
fn test_link_not_focused() {
    let link = Link::new("url").focused(false);
    assert!(!link.is_focused());
}

#[test]
fn test_link_not_disabled() {
    let link = Link::new("url").disabled(false);
    assert!(!link.is_disabled());
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
fn test_format_display_empty_text() {
    let link = Link::new("url").text("");
    assert_eq!(link.display_text(), "");
    assert_eq!(rendered(&link, 5), "");
}

#[test]
fn test_format_display_unicode() {
    let link = Link::new("url")
        .text("Hello 世界")
        .style(LinkStyle::Bracketed);
    assert_eq!(rendered(&link, 20), "[Hello 世界]");
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
