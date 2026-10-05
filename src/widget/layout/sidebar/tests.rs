//! Sidebar unit tests for builder-set colors
//!
//! The color builders only store an Option<Color>; there is no public getter,
//! so these read the private fields. Everything else about Sidebar is tested
//! through the public API in tests/sidebar/, tests/widget/sidebar_tests.rs and
//! tests/widget/layout/sidebar*.rs.

use super::Sidebar;
use crate::style::Color;

#[test]
fn test_sidebar_fg() {
    let sb = Sidebar::new().fg(Color::WHITE);
    assert_eq!(sb.fg, Some(Color::WHITE));
}

#[test]
fn test_sidebar_selected_style() {
    let sb = Sidebar::new().selected_style(Color::WHITE, Color::BLUE);
    assert_eq!(sb.selected_fg, Some(Color::WHITE));
    assert_eq!(sb.selected_bg, Some(Color::BLUE));
}

#[test]
fn test_sidebar_hover_style() {
    let sb = Sidebar::new().hover_style(Color::YELLOW, Color::CYAN);
    assert_eq!(sb.hover_fg, Some(Color::YELLOW));
    assert_eq!(sb.hover_bg, Some(Color::CYAN));
}

#[test]
fn test_sidebar_disabled_color() {
    let sb = Sidebar::new().disabled_color(Color::rgb(128, 128, 128));
    assert_eq!(sb.disabled_fg, Some(Color::rgb(128, 128, 128)));
}

#[test]
fn test_sidebar_section_color() {
    let sb = Sidebar::new().section_color(Color::CYAN);
    assert_eq!(sb.section_fg, Some(Color::CYAN));
}

#[test]
fn test_sidebar_badge_style() {
    let sb = Sidebar::new().badge_style(Color::WHITE, Color::RED);
    assert_eq!(sb.badge_fg, Some(Color::WHITE));
    assert_eq!(sb.badge_bg, Some(Color::RED));
}

#[test]
fn test_sidebar_border_color() {
    let sb = Sidebar::new().border_color(Color::MAGENTA);
    assert_eq!(sb.border_fg, Some(Color::MAGENTA));
}
