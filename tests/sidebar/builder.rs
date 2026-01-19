//! Sidebar Builder tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    sidebar, sidebar_item, sidebar_section, sidebar_section_titled, CollapseMode, FlattenedItem,
    Sidebar, SidebarItem, SidebarSection,
};

#[test]
fn test_sidebar_items() {
    let sb = Sidebar::new().items(vec![
        SidebarItem::new("home", "Home"),
        SidebarItem::new("settings", "Settings"),
    ]);

    assert_eq!(sb.item_count(), 2);
}

#[test]
fn test_sidebar_section() {
    let sb = Sidebar::new()
        .section(SidebarSection::titled(
            "Navigation",
            vec![SidebarItem::new("home", "Home")],
        ))
        .section(SidebarSection::titled(
            "Settings",
            vec![SidebarItem::new("prefs", "Preferences")],
        ));

    // 2 items across 2 sections
    assert_eq!(sb.item_count(), 2);
}

#[test]
fn test_sidebar_selected() {
    let sb = Sidebar::new()
        .items(vec![SidebarItem::new("home", "Home")])
        .selected("home");

    assert_eq!(sb.selected_id(), Some("home"));
}

#[test]
fn test_sidebar_collapse_mode() {
    let sb_expanded = Sidebar::new().collapse_mode(CollapseMode::Expanded);
    assert!(!sb_expanded.is_collapsed());

    let sb_collapsed = Sidebar::new().collapse_mode(CollapseMode::Collapsed);
    assert!(sb_collapsed.is_collapsed());
}

#[test]
fn test_sidebar_dimensions() {
    let sb = Sidebar::new()
        .expanded_width(30)
        .collapsed_width(5)
        .collapse_threshold(15);

    assert_eq!(sb.current_width(), 30);
}

#[test]
fn test_sidebar_header_footer() {
    let sb = Sidebar::new().header("App Name").footer("v1.0.0");

    // Just verify it builds without error
    assert_eq!(sb.item_count(), 0);
}

#[test]
fn test_sidebar_styling() {
    let sb = Sidebar::new()
        .fg(Color::WHITE)
        .bg(Color::BLACK)
        .selected_style(Color::WHITE, Color::BLUE)
        .hover_style(Color::WHITE, Color::rgb(50, 50, 50))
        .disabled_color(Color::rgb(100, 100, 100))
        .section_color(Color::CYAN)
        .badge_style(Color::WHITE, Color::RED)
        .border_color(Color::rgb(60, 60, 60));

    assert_eq!(sb.item_count(), 0);
}
