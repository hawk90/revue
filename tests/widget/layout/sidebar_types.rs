//! Tests for sidebar layout widget types

use crate::widget::layout::sidebar::{CollapseMode, SidebarItem, SidebarSection};

// =========================================================================
// SidebarItem::new tests
// =========================================================================

#[test]
fn test_sidebar_item_new() {
    let item = SidebarItem::new("test-id", "Test Label");
    assert_eq!(item.id, "test-id");
    assert_eq!(item.label, "Test Label");
}

#[test]
fn test_sidebar_item_new_default_values() {
    let item = SidebarItem::new("id", "label");
    assert!(item.icon.is_none());
    assert!(!item.disabled);
    assert!(item.badge.is_none());
    assert!(item.children.is_empty());
    assert!(!item.expanded);
}

#[test]
fn test_sidebar_item_new_with_strings() {
    let item = SidebarItem::new(String::from("my-id"), String::from("My Label"));
    assert_eq!(item.id, "my-id");
    assert_eq!(item.label, "My Label");
}

// =========================================================================
// SidebarItem builder methods tests
// =========================================================================

#[test]
fn test_sidebar_item_icon() {
    let item = SidebarItem::new("id", "label").icon('🏠');
    assert_eq!(item.icon, Some('🏠'));
}

#[test]
fn test_sidebar_item_disabled() {
    let item = SidebarItem::new("id", "label").disabled(true);
    assert!(item.disabled);
}

#[test]
fn test_sidebar_item_disabled_false() {
    let item = SidebarItem::new("id", "label").disabled(false);
    assert!(!item.disabled);
}

#[test]
fn test_sidebar_item_badge() {
    let item = SidebarItem::new("id", "label").badge("5");
    assert_eq!(item.badge.as_deref(), Some("5"));
}

#[test]
fn test_sidebar_item_badge_with_string() {
    let item = SidebarItem::new("id", "label").badge(String::from("new"));
    assert_eq!(item.badge.as_deref(), Some("new"));
}

#[test]
fn test_sidebar_item_children() {
    let child1 = SidebarItem::new("child1", "Child 1");
    let child2 = SidebarItem::new("child2", "Child 2");
    let item = SidebarItem::new("parent", "Parent").children(vec![child1, child2]);
    assert_eq!(item.children.len(), 2);
    assert_eq!(item.children[0].id, "child1");
    assert_eq!(item.children[1].id, "child2");
}

#[test]
fn test_sidebar_item_expanded() {
    let item = SidebarItem::new("id", "label").expanded(true);
    assert!(item.expanded);
}

#[test]
fn test_sidebar_item_expanded_false() {
    let item = SidebarItem::new("id", "label").expanded(false);
    assert!(!item.expanded);
}

#[test]
fn test_sidebar_item_chain() {
    let item = SidebarItem::new("id", "label")
        .icon('📄')
        .disabled(true)
        .badge("3")
        .expanded(true);
    assert_eq!(item.icon, Some('📄'));
    assert!(item.disabled);
    assert_eq!(item.badge.as_deref(), Some("3"));
    assert!(item.expanded);
}

// =========================================================================
// SidebarItem::has_children tests
// =========================================================================

#[test]
fn test_sidebar_item_has_children_true() {
    let child = SidebarItem::new("child", "Child");
    let item = SidebarItem::new("parent", "Parent").children(vec![child]);
    assert!(item.has_children());
}

#[test]
fn test_sidebar_item_has_children_false() {
    let item = SidebarItem::new("id", "label");
    assert!(!item.has_children());
}

#[test]
fn test_sidebar_item_has_children_empty_vec() {
    let item = SidebarItem::new("id", "label").children(vec![]);
    assert!(!item.has_children());
}

#[test]
fn test_sidebar_item_nested_children() {
    let grandchild = SidebarItem::new("grandchild", "Grandchild");
    let child = SidebarItem::new("child", "Child").children(vec![grandchild]);
    let parent = SidebarItem::new("parent", "Parent").children(vec![child]);
    assert!(parent.has_children());
    assert!(parent.children[0].has_children());
    assert!(!parent.children[0].children[0].has_children());
}

// =========================================================================
// SidebarSection::new tests
// =========================================================================

#[test]
fn test_sidebar_section_new_no_title() {
    let section =
        SidebarSection::new(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
    assert!(section.title.is_none());
    assert_eq!(section.items.len(), 2);
}

#[test]
fn test_sidebar_section_new_empty() {
    let section = SidebarSection::new(vec![]);
    assert!(section.title.is_none());
    assert!(section.items.is_empty());
}

// =========================================================================
// SidebarSection::titled tests
// =========================================================================

#[test]
fn test_sidebar_section_titled() {
    let section = SidebarSection::titled("My Section", vec![SidebarItem::new("x", "X")]);
    assert_eq!(section.title.as_deref(), Some("My Section"));
    assert_eq!(section.items.len(), 1);
}

#[test]
fn test_sidebar_section_titled_with_string() {
    let section = SidebarSection::titled(String::from("Title"), vec![]);
    assert_eq!(section.title.as_deref(), Some("Title"));
}

#[test]
fn test_sidebar_section_titled_empty_items() {
    let section = SidebarSection::titled("Empty", vec![]);
    assert_eq!(section.title.as_deref(), Some("Empty"));
    assert!(section.items.is_empty());
}

// =========================================================================
// CollapseMode tests
// =========================================================================

#[test]
fn test_collapse_mode_default_is_expanded() {
    let mode = CollapseMode::default();
    assert_eq!(mode, CollapseMode::Expanded);
}

#[test]
fn test_collapse_mode_partial_eq() {
    assert_eq!(CollapseMode::Expanded, CollapseMode::Expanded);
    assert_eq!(CollapseMode::Collapsed, CollapseMode::Collapsed);
    assert_eq!(CollapseMode::Auto, CollapseMode::Auto);
    assert_ne!(CollapseMode::Expanded, CollapseMode::Collapsed);
}

// =========================================================================
// FlattenedItem tests
// =========================================================================

#[test]
fn test_flattened_item_section() {
    let item = crate::widget::layout::sidebar::types::FlattenedItem::Section(Some("Header".to_string()));
    match item {
        crate::widget::layout::sidebar::types::FlattenedItem::Section(Some(title)) => assert_eq!(title, "Header"),
        _ => panic!("Expected Section with title"),
    }
}

#[test]
fn test_flattened_item_section_none() {
    let item = crate::widget::layout::sidebar::types::FlattenedItem::Section(None);
    match item {
        crate::widget::layout::sidebar::types::FlattenedItem::Section(None) => {}
        _ => panic!("Expected Section with None"),
    }
}

#[test]
fn test_flattened_item_item() {
    let sidebar_item = SidebarItem::new("id", "Label");
    let item = crate::widget::layout::sidebar::types::FlattenedItem::Item {
        item: sidebar_item,
        depth: 2,
    };
    match item {
        crate::widget::layout::sidebar::types::FlattenedItem::Item { item, depth } => {
            assert_eq!(item.id, "id");
            assert_eq!(depth, 2);
        }
        _ => panic!("Expected Item"),
    }
}

#[test]
fn test_flattened_item_item_depth_zero() {
    let sidebar_item = SidebarItem::new("root", "Root");
    let item = crate::widget::layout::sidebar::types::FlattenedItem::Item {
        item: sidebar_item,
        depth: 0,
    };
    match item {
        crate::widget::layout::sidebar::types::FlattenedItem::Item { depth, .. } => assert_eq!(depth, 0),
        _ => panic!("Expected Item"),
    }
}

// =========================================================================
// SidebarItem Clone tests
// =========================================================================

#[test]
fn test_sidebar_item_clone() {
    let item1 = SidebarItem::new("id", "label").icon('🏠');
    let item2 = item1.clone();
    assert_eq!(item1.id, item2.id);
    assert_eq!(item1.icon, item2.icon);
}

// =========================================================================
// SidebarSection Clone tests
// =========================================================================

#[test]
fn test_sidebar_section_clone() {
    let section1 = SidebarSection::titled("Title", vec![SidebarItem::new("a", "A")]);
    let section2 = section1.clone();
    assert_eq!(section1.title, section2.title);
    assert_eq!(section1.items.len(), section2.items.len());
}

// =========================================================================
// Integration tests
// =========================================================================

#[test]
fn test_sidebar_item_complete() {
    let item = SidebarItem::new("folder", "Documents")
        .icon('📁')
        .badge("12")
        .expanded(true)
        .children(vec![
            SidebarItem::new("file1", "File 1"),
            SidebarItem::new("file2", "File 2"),
        ]);

    assert_eq!(item.id, "folder");
    assert_eq!(item.label, "Documents");
    assert_eq!(item.icon, Some('📁'));
    assert_eq!(item.badge.as_deref(), Some("12"));
    assert!(item.expanded);
    assert!(item.has_children());
    assert_eq!(item.children.len(), 2);
}

#[test]
fn test_sidebar_section_complete() {
    let section = SidebarSection::titled(
        "Navigation",
        vec![
            SidebarItem::new("home", "Home").icon('🏠'),
            SidebarItem::new("docs", "Documents").icon('📁').badge("5"),
            SidebarItem::new("settings", "Settings").icon('⚙'),
        ],
    );

    assert_eq!(section.title.as_deref(), Some("Navigation"));
    assert_eq!(section.items.len(), 3);
    assert_eq!(section.items[0].icon, Some('🏠'));
    assert_eq!(section.items[1].badge.as_deref(), Some("5"));
}