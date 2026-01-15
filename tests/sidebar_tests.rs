//! Tests for Sidebar Navigation widget
//!
//! These tests use only the public API of the widget.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    sidebar, sidebar_item, sidebar_section, sidebar_section_titled, CollapseMode, FlattenedItem,
    Sidebar, SidebarItem, SidebarSection,
};

// =============================================================================
// Basic Tests
// =============================================================================

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

// =============================================================================
// Sidebar Builder Tests
// =============================================================================

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

// =============================================================================
// Navigation Tests
// =============================================================================

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

// =============================================================================
// Expand/Collapse Tests
// =============================================================================

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

// =============================================================================
// Flattened Items Tests
// =============================================================================

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

// =============================================================================
// Render Tests
// =============================================================================

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

// =============================================================================
// Helper Function Tests
// =============================================================================

#[test]
fn test_sidebar_helper() {
    let sb = sidebar()
        .header("Test")
        .items(vec![sidebar_item("home", "Home").icon('🏠')]);

    assert_eq!(sb.item_count(), 1);
}

#[test]
fn test_sidebar_section_helpers() {
    let sb = sidebar()
        .section(sidebar_section_titled(
            "Navigation",
            vec![sidebar_item("home", "Home")],
        ))
        .section(sidebar_section(vec![sidebar_item("other", "Other")]));

    assert_eq!(sb.item_count(), 2);
}
