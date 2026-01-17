//! Expand/Collapse tests

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
fn test_sidebar_toggle_item() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("parent", "Parent")
        .children(vec![SidebarItem::new("child", "Child")])
        .expanded(false)]);

    // Initially 1 item visible (parent collapsed)
    assert_eq!(sb.item_count(), 1);

    sb.toggle_item("parent");

    // After expanding, 2 items visible
    assert_eq!(sb.item_count(), 2);
}

#[test]
fn test_sidebar_expand_collapse_all() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("p1", "Parent 1")
            .children(vec![SidebarItem::new("c1", "Child 1")])
            .expanded(false),
        SidebarItem::new("p2", "Parent 2")
            .children(vec![SidebarItem::new("c2", "Child 2")])
            .expanded(false),
    ]);

    // Initially 2 items (both collapsed)
    assert_eq!(sb.item_count(), 2);

    sb.expand_all();
    assert_eq!(sb.item_count(), 4);

    sb.collapse_all();
    assert_eq!(sb.item_count(), 2);
}

#[test]
fn test_sidebar_toggle_collapse() {
    let mut sb = Sidebar::new();

    assert!(!sb.is_collapsed());

    sb.toggle_collapse();
    assert!(sb.is_collapsed());

    sb.toggle_collapse();
    assert!(!sb.is_collapsed());
}
