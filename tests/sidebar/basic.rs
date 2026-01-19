//! Basic tests

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
fn test_sidebar_new() {
    let sb = Sidebar::new();
    assert_eq!(sb.selected_id(), None);
    assert!(!sb.is_collapsed());
}

#[test]
fn test_sidebar_default() {
    let sb = Sidebar::default();
    assert_eq!(sb.item_count(), 0);
}

#[test]
fn test_sidebar_item_new() {
    let item = SidebarItem::new("home", "Home");
    assert_eq!(item.id, "home");
    assert_eq!(item.label, "Home");
    assert!(!item.disabled);
    assert!(item.icon.is_none());
    assert!(item.badge.is_none());
}

#[test]
fn test_sidebar_item_builder() {
    let item = SidebarItem::new("settings", "Settings")
        .icon('⚙')
        .badge("3")
        .disabled(true);

    assert_eq!(item.icon, Some('⚙'));
    assert_eq!(item.badge, Some("3".to_string()));
    assert!(item.disabled);
}

#[test]
fn test_sidebar_item_children() {
    let child1 = SidebarItem::new("child1", "Child 1");
    let child2 = SidebarItem::new("child2", "Child 2");
    let parent = SidebarItem::new("parent", "Parent").children(vec![child1, child2]);

    assert!(parent.has_children());
    assert_eq!(parent.children.len(), 2);
}

#[test]
fn test_sidebar_section_new() {
    let items = vec![
        SidebarItem::new("a", "Item A"),
        SidebarItem::new("b", "Item B"),
    ];
    let section = SidebarSection::new(items);

    assert!(section.title.is_none());
    assert_eq!(section.items.len(), 2);
}

#[test]
fn test_sidebar_section_titled() {
    let items = vec![SidebarItem::new("a", "Item A")];
    let section = SidebarSection::titled("My Section", items);

    assert_eq!(section.title, Some("My Section".to_string()));
}
