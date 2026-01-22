//! Sidebar tests

use super::types::{CollapseMode, FlattenedItem, SidebarItem, SidebarSection};
use super::Sidebar;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

// Re-export helpers from parent module
pub use super::sidebar;
pub use super::sidebar_item;
pub use super::sidebar_section;
pub use super::sidebar_section_titled;

// =========================================================================
// SidebarItem Tests
// =========================================================================

#[test]
fn test_sidebar_item_new() {
    let item = SidebarItem::new("home", "Home");
    assert_eq!(item.id, "home");
    assert_eq!(item.label, "Home");
    assert!(item.icon.is_none());
    assert!(!item.disabled);
    assert!(item.badge.is_none());
    assert!(item.children.is_empty());
    assert!(!item.expanded);
}

#[test]
fn test_sidebar_item_icon() {
    let item = SidebarItem::new("home", "Home").icon('🏠');
    assert_eq!(item.icon, Some('🏠'));
}

#[test]
fn test_sidebar_item_disabled() {
    let item = SidebarItem::new("home", "Home").disabled(true);
    assert!(item.disabled);
}

#[test]
fn test_sidebar_item_badge() {
    let item = SidebarItem::new("inbox", "Inbox").badge("5");
    assert_eq!(item.badge, Some("5".to_string()));
}

#[test]
fn test_sidebar_item_children() {
    let children = vec![
        SidebarItem::new("child1", "Child 1"),
        SidebarItem::new("child2", "Child 2"),
    ];
    let item = SidebarItem::new("parent", "Parent").children(children);
    assert_eq!(item.children.len(), 2);
}

#[test]
fn test_sidebar_item_expanded() {
    let item = SidebarItem::new("folder", "Folder").expanded(true);
    assert!(item.expanded);
}

#[test]
fn test_sidebar_item_has_children() {
    let item =
        SidebarItem::new("folder", "Folder").children(vec![SidebarItem::new("file", "File")]);
    assert!(item.has_children());

    let empty_item = SidebarItem::new("file", "File");
    assert!(!empty_item.has_children());
}

#[test]
fn test_sidebar_item_builder_chain() {
    let item = SidebarItem::new("nav", "Navigation")
        .icon('📁')
        .disabled(false)
        .badge("3")
        .expanded(true);

    assert_eq!(item.icon, Some('📁'));
    assert!(!item.disabled);
    assert_eq!(item.badge, Some("3".to_string()));
    assert!(item.expanded);
}

// =========================================================================
// SidebarSection Tests
// =========================================================================

#[test]
fn test_sidebar_section_new() {
    let items = vec![SidebarItem::new("home", "Home")];
    let section = SidebarSection::new(items);
    assert!(section.title.is_none());
    assert_eq!(section.items.len(), 1);
}

#[test]
fn test_sidebar_section_titled() {
    let items = vec![SidebarItem::new("home", "Home")];
    let section = SidebarSection::titled("Main", items);
    assert_eq!(section.title, Some("Main".to_string()));
    assert_eq!(section.items.len(), 1);
}

// =========================================================================
// CollapseMode Tests
// =========================================================================

#[test]
fn test_collapse_mode_default() {
    assert_eq!(CollapseMode::default(), CollapseMode::Expanded);
}

#[test]
fn test_collapse_mode_equality() {
    assert_eq!(CollapseMode::Expanded, CollapseMode::Expanded);
    assert_ne!(CollapseMode::Expanded, CollapseMode::Collapsed);
}

// =========================================================================
// Sidebar Creation Tests
// =========================================================================

#[test]
fn test_sidebar_new() {
    let sb = Sidebar::new();
    assert!(sb.selected_id().is_none());
    assert_eq!(sb.hovered_index(), 0);
    assert!(!sb.is_collapsed());
}

#[test]
fn test_sidebar_default() {
    let sb = Sidebar::default();
    assert!(!sb.is_collapsed());
}

#[test]
fn test_sidebar_helper() {
    let sb = sidebar();
    assert!(sb.selected_id().is_none());
}

#[test]
fn test_sidebar_item_helper() {
    let item = sidebar_item("test", "Test");
    assert_eq!(item.id, "test");
    assert_eq!(item.label, "Test");
}

#[test]
fn test_sidebar_section_helper() {
    let section = sidebar_section(vec![sidebar_item("a", "A")]);
    assert!(section.title.is_none());
    assert_eq!(section.items.len(), 1);
}

#[test]
fn test_sidebar_section_titled_helper() {
    let section = sidebar_section_titled("Title", vec![sidebar_item("a", "A")]);
    assert_eq!(section.title, Some("Title".to_string()));
}

// =========================================================================
// Sidebar Builder Tests
// =========================================================================

#[test]
fn test_sidebar_section_builder() {
    let sb = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new("home", "Home")]));
    assert_eq!(sb.item_count(), 1);
}

#[test]
fn test_sidebar_sections_builder() {
    let sb = Sidebar::new().sections(vec![
        SidebarSection::new(vec![SidebarItem::new("a", "A")]),
        SidebarSection::new(vec![SidebarItem::new("b", "B")]),
    ]);
    assert_eq!(sb.item_count(), 2);
}

#[test]
fn test_sidebar_items_builder() {
    let sb = Sidebar::new().items(vec![
        SidebarItem::new("home", "Home"),
        SidebarItem::new("settings", "Settings"),
    ]);
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
    let sb = Sidebar::new().collapse_mode(CollapseMode::Collapsed);
    assert!(sb.is_collapsed());
}

#[test]
fn test_sidebar_collapse_threshold() {
    let sb = Sidebar::new().collapse_threshold(30);
    assert_eq!(sb.collapse_threshold, 30);
}

#[test]
fn test_sidebar_expanded_width() {
    let sb = Sidebar::new().expanded_width(32);
    assert_eq!(sb.expanded_width, 32);
}

#[test]
fn test_sidebar_collapsed_width() {
    let sb = Sidebar::new().collapsed_width(6);
    assert_eq!(sb.collapsed_width, 6);
}

#[test]
fn test_sidebar_header() {
    let sb = Sidebar::new().header("App Name");
    assert_eq!(sb.header, Some("App Name".to_string()));
}

#[test]
fn test_sidebar_footer() {
    let sb = Sidebar::new().footer("v1.0.0");
    assert_eq!(sb.footer, Some("v1.0.0".to_string()));
}

#[test]
fn test_sidebar_fg() {
    let sb = Sidebar::new().fg(Color::WHITE);
    assert_eq!(sb.fg, Some(Color::WHITE));
}

#[test]
fn test_sidebar_bg() {
    let sb = Sidebar::new().bg(Color::BLACK);
    assert_eq!(sb.bg, Some(Color::BLACK));
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

// =========================================================================
// State Getter Tests
// =========================================================================

#[test]
fn test_sidebar_current_width_expanded() {
    let sb = Sidebar::new()
        .expanded_width(30)
        .collapse_mode(CollapseMode::Expanded);
    assert_eq!(sb.current_width(), 30);
}

#[test]
fn test_sidebar_current_width_collapsed() {
    let sb = Sidebar::new()
        .collapsed_width(5)
        .collapse_mode(CollapseMode::Collapsed);
    assert_eq!(sb.current_width(), 5);
}

#[test]
fn test_sidebar_current_width_auto() {
    let sb = Sidebar::new()
        .expanded_width(25)
        .collapse_mode(CollapseMode::Auto);
    // Auto mode returns expanded_width (actual collapse determined at render)
    assert_eq!(sb.current_width(), 25);
}

#[test]
fn test_sidebar_visible_items_empty() {
    let sb = Sidebar::new();
    assert!(sb.visible_items().is_empty());
}

#[test]
fn test_sidebar_visible_items_flat() {
    let sb = Sidebar::new().items(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
    let items = sb.visible_items();
    assert_eq!(items.len(), 2);
}

#[test]
fn test_sidebar_visible_items_with_section_title() {
    let sb = Sidebar::new().section(SidebarSection::titled(
        "Section",
        vec![SidebarItem::new("a", "A")],
    ));
    let items = sb.visible_items();
    assert_eq!(items.len(), 2); // Section header + item
}

#[test]
fn test_sidebar_visible_items_nested_collapsed() {
    let sb =
        Sidebar::new()
            .items(vec![SidebarItem::new("parent", "Parent")
                .children(vec![SidebarItem::new("child", "Child")])]);
    let items = sb.visible_items();
    // Parent not expanded, so child is hidden
    assert_eq!(items.len(), 1);
}

#[test]
fn test_sidebar_visible_items_nested_expanded() {
    let sb = Sidebar::new().items(vec![SidebarItem::new("parent", "Parent")
        .expanded(true)
        .children(vec![SidebarItem::new("child", "Child")])]);
    let items = sb.visible_items();
    // Parent expanded, so child is visible
    assert_eq!(items.len(), 2);
}

#[test]
fn test_sidebar_item_count() {
    let sb = Sidebar::new()
        .section(SidebarSection::titled(
            "Main",
            vec![SidebarItem::new("a", "A")],
        ))
        .items(vec![SidebarItem::new("b", "B")]);
    // item_count excludes sections
    assert_eq!(sb.item_count(), 2);
}

// =========================================================================
// Navigation Tests
// =========================================================================

#[test]
fn test_sidebar_hover_down() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "A"),
        SidebarItem::new("b", "B"),
        SidebarItem::new("c", "C"),
    ]);
    assert_eq!(sb.hovered_index(), 0);
    sb.hover_down();
    assert_eq!(sb.hovered_index(), 1);
    sb.hover_down();
    assert_eq!(sb.hovered_index(), 2);
}

#[test]
fn test_sidebar_hover_down_at_end() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
    sb.hover_down();
    sb.hover_down();
    sb.hover_down(); // Should stay at last
    assert_eq!(sb.hovered_index(), 1);
}

#[test]
fn test_sidebar_hover_down_skips_disabled() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "A"),
        SidebarItem::new("b", "B").disabled(true),
        SidebarItem::new("c", "C"),
    ]);
    sb.hover_down();
    // Should skip disabled item B and go to C
    assert_eq!(sb.hovered_index(), 2);
}

#[test]
fn test_sidebar_hover_up() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "A"),
        SidebarItem::new("b", "B"),
        SidebarItem::new("c", "C"),
    ]);
    sb.hovered = 2;
    sb.hover_up();
    assert_eq!(sb.hovered_index(), 1);
    sb.hover_up();
    assert_eq!(sb.hovered_index(), 0);
}

#[test]
fn test_sidebar_hover_up_at_start() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
    sb.hover_up(); // Already at 0
    assert_eq!(sb.hovered_index(), 0);
}

#[test]
fn test_sidebar_select_hovered() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
    sb.hovered = 1;
    sb.select_hovered();
    assert_eq!(sb.selected_id(), Some("b"));
}

#[test]
fn test_sidebar_select_hovered_disabled() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A").disabled(true)]);
    sb.select_hovered();
    // Should not select disabled item
    assert!(sb.selected_id().is_none());
}

#[test]
fn test_sidebar_toggle_hovered() {
    let mut sb =
        Sidebar::new()
            .items(vec![SidebarItem::new("parent", "Parent")
                .children(vec![SidebarItem::new("child", "Child")])]);
    assert_eq!(sb.visible_items().len(), 1);
    sb.toggle_hovered();
    assert_eq!(sb.visible_items().len(), 2);
    sb.toggle_hovered();
    assert_eq!(sb.visible_items().len(), 1);
}

#[test]
fn test_sidebar_toggle_item() {
    let mut sb =
        Sidebar::new()
            .items(vec![SidebarItem::new("folder", "Folder")
                .children(vec![SidebarItem::new("file", "File")])]);
    sb.toggle_item("folder");
    let items = sb.visible_items();
    assert_eq!(items.len(), 2);
}

#[test]
fn test_sidebar_expand_all() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "A").children(vec![SidebarItem::new("a1", "A1")]),
        SidebarItem::new("b", "B").children(vec![SidebarItem::new("b1", "B1")]),
    ]);
    sb.expand_all();
    assert_eq!(sb.visible_items().len(), 4);
}

#[test]
fn test_sidebar_collapse_all() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "A")
            .expanded(true)
            .children(vec![SidebarItem::new("a1", "A1")]),
        SidebarItem::new("b", "B")
            .expanded(true)
            .children(vec![SidebarItem::new("b1", "B1")]),
    ]);
    assert_eq!(sb.visible_items().len(), 4);
    sb.collapse_all();
    assert_eq!(sb.visible_items().len(), 2);
}

#[test]
fn test_sidebar_toggle_collapse() {
    let mut sb = Sidebar::new().collapse_mode(CollapseMode::Expanded);
    assert!(!sb.is_collapsed());
    sb.toggle_collapse();
    assert!(sb.is_collapsed());
    sb.toggle_collapse();
    assert!(!sb.is_collapsed());
}

#[test]
fn test_sidebar_toggle_collapse_from_auto() {
    let mut sb = Sidebar::new().collapse_mode(CollapseMode::Auto);
    sb.toggle_collapse();
    assert!(sb.is_collapsed());
}

// =========================================================================
// FlattenedItem Tests
// =========================================================================

#[test]
fn test_flattened_item_section() {
    let flat = FlattenedItem::Section(Some("Title".to_string()));
    if let FlattenedItem::Section(title) = flat {
        assert_eq!(title, Some("Title".to_string()));
    } else {
        panic!("Expected Section");
    }
}

#[test]
fn test_flattened_item_item() {
    let flat = FlattenedItem::Item {
        item: SidebarItem::new("test", "Test"),
        depth: 2,
    };
    if let FlattenedItem::Item { item, depth } = flat {
        assert_eq!(item.id, "test");
        assert_eq!(depth, 2);
    } else {
        panic!("Expected Item");
    }
}

// =========================================================================
// Render Tests
// =========================================================================

#[test]
fn test_sidebar_render_basic() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![
        SidebarItem::new("home", "Home").icon('🏠'),
        SidebarItem::new("settings", "Settings").icon('⚙'),
    ]);
    sb.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_sidebar_render_with_header() {
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .header("My App")
        .items(vec![SidebarItem::new("home", "Home")]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_footer() {
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .footer("v1.0.0")
        .items(vec![SidebarItem::new("home", "Home")]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_collapsed() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .collapse_mode(CollapseMode::Collapsed)
        .items(vec![SidebarItem::new("home", "Home").icon('🏠')]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_sections() {
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .section(SidebarSection::titled(
            "Main",
            vec![SidebarItem::new("home", "Home")],
        ))
        .section(SidebarSection::titled(
            "Settings",
            vec![SidebarItem::new("prefs", "Preferences")],
        ));
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_nested() {
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![SidebarItem::new("folder", "Folder")
        .expanded(true)
        .children(vec![SidebarItem::new("file", "File")])]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_badges() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![SidebarItem::new("inbox", "Inbox").badge("5")]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_disabled() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![
        SidebarItem::new("active", "Active"),
        SidebarItem::new("disabled", "Disabled").disabled(true),
    ]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_selected() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .items(vec![
            SidebarItem::new("a", "Item A"),
            SidebarItem::new("b", "Item B"),
        ])
        .selected("b");
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_small_area() {
    let mut buffer = Buffer::new(2, 1);
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![SidebarItem::new("home", "Home")]);
    sb.render(&mut ctx); // Should handle gracefully
}

#[test]
fn test_sidebar_render_auto_collapse() {
    let mut buffer = Buffer::new(15, 10);
    let area = Rect::new(0, 0, 15, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .collapse_mode(CollapseMode::Auto)
        .collapse_threshold(20)
        .items(vec![SidebarItem::new("home", "Home").icon('🏠')]);
    // Width (15) < threshold (20), so should render collapsed
    sb.render(&mut ctx);
}

// =========================================================================
// Edge Cases
// =========================================================================

#[test]
fn test_sidebar_empty() {
    let sb = Sidebar::new();
    assert_eq!(sb.item_count(), 0);
    assert!(sb.visible_items().is_empty());
}

#[test]
fn test_sidebar_navigation_on_empty() {
    let mut sb = Sidebar::new();
    sb.hover_up();
    sb.hover_down();
    sb.select_hovered();
    // Should not panic
    assert_eq!(sb.hovered_index(), 0);
}

#[test]
fn test_sidebar_toggle_nonexistent_item() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A")]);
    sb.toggle_item("nonexistent");
    // Should not panic
}

#[test]
fn test_sidebar_deeply_nested() {
    let sb = Sidebar::new().items(vec![SidebarItem::new("l1", "Level 1")
        .expanded(true)
        .children(vec![SidebarItem::new("l2", "Level 2")
            .expanded(true)
            .children(vec![SidebarItem::new("l3", "Level 3")
                .expanded(true)
                .children(vec![SidebarItem::new("l4", "Level 4")])])])]);

    let items = sb.visible_items();
    assert_eq!(items.len(), 4);

    // Check depths
    if let FlattenedItem::Item { depth, .. } = &items[3] {
        assert_eq!(*depth, 3);
    }
}
