//! Flattened Items tests

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
fn test_sidebar_visible_items() {
    let sb = Sidebar::new()
        .section(SidebarSection::titled(
            "Section",
            vec![SidebarItem::new("a", "A")],
        ))
        .items(vec![SidebarItem::new("b", "B")]);

    let items = sb.visible_items();

    // Section header + item A + item B
    assert_eq!(items.len(), 3);

    // First should be section
    assert!(matches!(items[0], FlattenedItem::Section(_)));
}

#[test]
fn test_sidebar_nested_visible_items() {
    let sb = Sidebar::new().items(vec![SidebarItem::new("parent", "Parent")
        .children(vec![
            SidebarItem::new("c1", "Child 1"),
            SidebarItem::new("c2", "Child 2"),
        ])
        .expanded(true)]);

    let items = sb.visible_items();

    // Parent + 2 children
    assert_eq!(items.len(), 3);
}
