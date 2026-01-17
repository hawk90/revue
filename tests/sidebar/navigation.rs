//! Navigation tests

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
fn test_sidebar_hover_navigation() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "Item A"),
        SidebarItem::new("b", "Item B"),
        SidebarItem::new("c", "Item C"),
    ]);

    assert_eq!(sb.hovered_index(), 0);

    sb.hover_down();
    assert_eq!(sb.hovered_index(), 1);

    sb.hover_down();
    assert_eq!(sb.hovered_index(), 2);

    sb.hover_up();
    assert_eq!(sb.hovered_index(), 1);
}

#[test]
fn test_sidebar_select_hovered() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "Item A"),
        SidebarItem::new("b", "Item B"),
    ]);

    sb.hover_down();
    sb.select_hovered();

    assert_eq!(sb.selected_id(), Some("b"));
}

#[test]
fn test_sidebar_skip_disabled() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "Item A"),
        SidebarItem::new("b", "Item B").disabled(true),
        SidebarItem::new("c", "Item C"),
    ]);

    sb.hover_down();
    // Should skip disabled item and go to "c"
    assert_eq!(sb.hovered_index(), 2);
}
