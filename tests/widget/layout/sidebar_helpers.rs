//! Tests for sidebar layout widget helper functions

use crate::widget::layout::sidebar::{sidebar, sidebar_item, sidebar_section, sidebar_section_titled};

// =========================================================================
// sidebar helper tests
// =========================================================================

#[test]
fn test_sidebar_helper_creates_sidebar() {
    let _sidebar = sidebar();
}

// =========================================================================
// sidebar_item helper tests
// =========================================================================

#[test]
fn test_sidebar_item_helper_with_string_id() {
    let item = sidebar_item("test-id", "Test Label");
    assert_eq!(item.id, "test-id");
    assert_eq!(item.label, "Test Label");
}

#[test]
fn test_sidebar_item_helper_default_values() {
    let item = sidebar_item("id", "label");
    assert_eq!(item.id, "id");
    assert_eq!(item.label, "label");
    assert!(item.icon.is_none());
    assert!(!item.disabled);
    assert!(item.badge.is_none());
    assert!(item.children.is_empty());
    assert!(!item.expanded);
}

#[test]
fn test_sidebar_item_helper_with_string_types() {
    let item = sidebar_item(String::from("my-id"), String::from("My Label"));
    assert_eq!(item.id, "my-id");
    assert_eq!(item.label, "My Label");
}

#[test]
fn test_sidebar_item_helper_chainable() {
    let item = sidebar_item("id", "label")
        .icon('🏠')
        .disabled(false)
        .badge("5")
        .expanded(true);
    assert_eq!(item.icon, Some('🏠'));
    assert!(!item.disabled);
    assert_eq!(item.badge.as_deref(), Some("5"));
    assert!(item.expanded);
}

// =========================================================================
// sidebar_section helper tests
// =========================================================================

#[test]
fn test_sidebar_section_helper_no_title() {
    let section = sidebar_section(vec![sidebar_item("a", "A"), sidebar_item("b", "B")]);
    assert!(section.title.is_none());
    assert_eq!(section.items.len(), 2);
}

#[test]
fn test_sidebar_section_helper_empty() {
    let section = sidebar_section(vec![]);
    assert!(section.title.is_none());
    assert!(section.items.is_empty());
}

#[test]
fn test_sidebar_section_helper_preserves_items() {
    let items = vec![
        sidebar_item("1", "One"),
        sidebar_item("2", "Two").icon('📄'),
    ];
    let section = sidebar_section(items);
    assert_eq!(section.items[0].id, "1");
    assert_eq!(section.items[1].icon, Some('📄'));
}

// =========================================================================
// sidebar_section_titled helper tests
// =========================================================================

#[test]
fn test_sidebar_section_titled_helper_has_title() {
    let section = sidebar_section_titled("My Section", vec![sidebar_item("x", "X")]);
    assert_eq!(section.title.as_deref(), Some("My Section"));
}

#[test]
fn test_sidebar_section_titled_with_string_types() {
    let section =
        sidebar_section_titled(String::from("Section Title"), vec![sidebar_item("a", "A")]);
    assert_eq!(section.title.as_deref(), Some("Section Title"));
}

#[test]
fn test_sidebar_section_titled_with_empty_items() {
    let section = sidebar_section_titled("Empty Section", vec![]);
    assert_eq!(section.title.as_deref(), Some("Empty Section"));
    assert!(section.items.is_empty());
}

#[test]
fn test_sidebar_section_titled_vs_untitled() {
    let titled = sidebar_section_titled("Title", vec![sidebar_item("a", "A")]);
    let untitled = sidebar_section(vec![sidebar_item("a", "A")]);

    assert!(titled.title.is_some());
    assert!(untitled.title.is_none());
}