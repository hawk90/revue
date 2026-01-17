//! Render tests

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
fn test_sidebar_render() {
    let mut buffer = Buffer::new(30, 20);
    let area = Rect::new(0, 0, 30, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .header("My App")
        .items(vec![
            SidebarItem::new("home", "Home").icon('🏠'),
            SidebarItem::new("settings", "Settings").icon('⚙'),
        ])
        .footer("v1.0");

    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_collapsed() {
    let mut buffer = Buffer::new(10, 20);
    let area = Rect::new(0, 0, 10, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .collapse_mode(CollapseMode::Collapsed)
        .items(vec![
            SidebarItem::new("home", "Home").icon('🏠'),
            SidebarItem::new("settings", "Settings").icon('⚙'),
        ]);

    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_badge() {
    let mut buffer = Buffer::new(30, 20);
    let area = Rect::new(0, 0, 30, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![
        SidebarItem::new("inbox", "Inbox").icon('📥').badge("99+"),
        SidebarItem::new("sent", "Sent").icon('📤'),
    ]);

    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_small_area() {
    let mut buffer = Buffer::new(2, 1);
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![SidebarItem::new("a", "A")]);

    // Should handle small area gracefully
    sb.render(&mut ctx);
}
