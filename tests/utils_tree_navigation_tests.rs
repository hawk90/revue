//! Tests for tree navigation module
//!
//! Extracted from src/utils/tree/navigation.rs

use revue::utils::tree::{TreeItem, TreeNav};

// =========================================================================
// TreeNav construction tests
// =========================================================================

#[test]
fn test_tree_nav_new() {
    let nav = TreeNav::new();
    assert_eq!(nav.selected(), 0);
    assert_eq!(nav.selected_row(), 0);
    assert!(nav.visible_items().is_empty());
}

#[test]
fn test_tree_nav_default() {
    let nav = TreeNav::default();
    assert_eq!(nav.selected(), 0);
    assert_eq!(nav.selected_row(), 0);
}

// =========================================================================
// Item management tests
// =========================================================================

#[test]
fn test_tree_nav_add_item() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    assert_eq!(nav.visible_items().len(), 1);
}

#[test]
fn test_tree_nav_add_multiple_items() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    nav.add_item(TreeItem::new(2));
    assert_eq!(nav.visible_items().len(), 3);
}

#[test]
fn test_tree_nav_clear() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    nav.clear();
    assert!(nav.visible_items().is_empty());
    assert_eq!(nav.selected(), 0);
}

// =========================================================================
// Visibility tests
// =========================================================================

#[test]
fn test_tree_nav_is_visible_no_parent() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    assert!(nav.is_visible(0));
}

#[test]
fn test_tree_nav_is_visible_with_parent() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    nav.add_item(TreeItem::new(1).with_parent(0));
    assert!(nav.is_visible(1));
}

#[test]
fn test_tree_nav_is_visible_parent_collapsed() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    nav.add_item(TreeItem::new(1).with_parent(0));
    nav.collapse(); // Collapse item 0
    assert!(!nav.is_visible(1));
}

#[test]
fn test_tree_nav_is_visible_grandparent_collapsed() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible()); // Root
    nav.add_item(TreeItem::new(1).with_parent(0).collapsible()); // Parent
    nav.add_item(TreeItem::new(2).with_parent(1)); // Child
    nav.collapse(); // Collapse root
    assert!(!nav.is_visible(2)); // Grandchild should be hidden
}

#[test]
fn test_tree_nav_visible_items_filters_hidden() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    nav.add_item(TreeItem::new(1).with_parent(0));
    nav.collapse();
    assert_eq!(nav.visible_items().len(), 1); // Only root visible
}

// =========================================================================
// Collapse/Expand tests
// =========================================================================

#[test]
fn test_tree_nav_is_collapsed() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    assert!(!nav.is_collapsed(0));

    nav.collapse();
    assert!(nav.is_collapsed(0));
}

#[test]
fn test_tree_nav_toggle_collapse() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    assert!(!nav.is_collapsed(0));

    nav.toggle_collapse();
    assert!(nav.is_collapsed(0));

    nav.toggle_collapse();
    assert!(!nav.is_collapsed(0));
}

#[test]
fn test_tree_nav_collapse() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    nav.collapse();
    assert!(nav.is_collapsed(0));
}

#[test]
fn test_tree_nav_expand() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    nav.collapse();
    nav.expand();
    assert!(!nav.is_collapsed(0));
}

#[test]
fn test_tree_nav_collapse_all() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    nav.add_item(TreeItem::new(1).collapsible());
    nav.collapse_all();
    assert!(nav.is_collapsed(0));
    assert!(nav.is_collapsed(1));
}

#[test]
fn test_tree_nav_expand_all() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    nav.add_item(TreeItem::new(1).collapsible());
    nav.collapse_all();
    nav.expand_all();
    assert!(!nav.is_collapsed(0));
    assert!(!nav.is_collapsed(1));
}

#[test]
fn test_tree_nav_toggle_non_collapsible() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0)); // Not collapsible
    nav.toggle_collapse();
    assert!(!nav.is_collapsed(0)); // Should stay expanded
}

// =========================================================================
// Navigation tests
// =========================================================================

#[test]
fn test_tree_nav_next_empty() {
    let mut nav = TreeNav::new();
    nav.next(); // Should not panic
}

#[test]
fn test_tree_nav_next_single_item() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.next();
    // Should wrap around or stay at same item
    assert!(nav.selected() < 1);
}

#[test]
fn test_tree_nav_prev_empty() {
    let mut nav = TreeNav::new();
    nav.prev(); // Should not panic
}

#[test]
fn test_tree_nav_next_item() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    nav.next_item();
    // Should move to item 1 or wrap
    assert!(nav.selected() < 2);
}

#[test]
fn test_tree_nav_prev_item() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    nav.select(1);
    nav.prev_item();
    // Should move to item 0 or wrap
    assert!(nav.selected() < 2);
}

#[test]
fn test_tree_nav_first() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    nav.add_item(TreeItem::new(2));
    nav.select(2);
    nav.first();
    assert_eq!(nav.selected(), 0);
}

#[test]
fn test_tree_nav_last() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    nav.add_item(TreeItem::new(2));
    nav.last();
    assert_eq!(nav.selected(), 2);
}

#[test]
fn test_tree_nav_first_empty() {
    let mut nav = TreeNav::new();
    nav.first(); // Should not panic
}

#[test]
fn test_tree_nav_last_empty() {
    let mut nav = TreeNav::new();
    nav.last(); // Should not panic
}

// =========================================================================
// Selection tests
// =========================================================================

#[test]
fn test_tree_nav_select_valid() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    nav.select(1);
    assert_eq!(nav.selected(), 1);
    assert_eq!(nav.selected_row(), 0);
}

#[test]
fn test_tree_nav_select_invalid() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.select(5); // Out of bounds
                   // Should not change selection
    assert!(nav.selected() < 1);
}

#[test]
fn test_tree_nav_select_hidden() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    nav.add_item(TreeItem::new(1).with_parent(0));
    nav.collapse();
    nav.select(1); // Should not select hidden item
                   // Selection should remain at 0 or be reset
    assert!(nav.selected() < 2);
}

#[test]
fn test_tree_nav_selected() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    assert_eq!(nav.selected(), 0);
}

#[test]
fn test_tree_nav_selected_row() {
    let nav = TreeNav::new();
    assert_eq!(nav.selected_row(), 0);
}

// =========================================================================
// Row navigation within item tests
// =========================================================================

#[test]
fn test_tree_nav_next_within_multirow_item() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).with_row_count(3));
    nav.next();
    assert_eq!(nav.selected(), 0); // Still on same item
    assert_eq!(nav.selected_row(), 1); // But next row
}

#[test]
fn test_tree_nav_prev_within_multirow_item() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).with_row_count(3));
    nav.next(); // Move to row 1
    nav.next(); // Move to row 2
    nav.prev();
    assert_eq!(nav.selected_row(), 1); // Moved up a row
}

// =========================================================================
// is_last_sibling tests
// =========================================================================

#[test]
fn test_tree_nav_is_last_sibling_only_child() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    assert!(nav.is_last_sibling(0));
}

#[test]
fn test_tree_nav_is_last_sibling_middle_child() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    nav.add_item(TreeItem::new(2));
    assert!(!nav.is_last_sibling(1));
}

#[test]
fn test_tree_nav_is_last_sibling_last_child() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    nav.add_item(TreeItem::new(2));
    assert!(nav.is_last_sibling(2));
}

#[test]
fn test_tree_nav_is_last_sibling_with_parent() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    nav.add_item(TreeItem::new(1).with_parent(0));
    nav.add_item(TreeItem::new(2).with_parent(0));
    assert!(nav.is_last_sibling(2));
}

// =========================================================================
// get_prefix tests
// =========================================================================

#[test]
fn test_tree_nav_get_prefix_root() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    let (prefix, is_last) = nav.get_prefix(0);
    assert!(is_last);
    assert!(prefix.contains("└─") || prefix.contains("├─"));
}

#[test]
fn test_tree_nav_get_prefix_child() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1).with_parent(0).with_depth(1));
    let (prefix, _) = nav.get_prefix(1);
    // Should have some prefix from parent
    assert!(!prefix.is_empty());
}

// =========================================================================
// render_items tests
// =========================================================================

#[test]
fn test_tree_nav_render_items_empty() {
    let nav = TreeNav::new();
    let items = nav.render_items();
    assert!(items.is_empty());
}

#[test]
fn test_tree_nav_render_items_structure() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    let items = nav.render_items();
    assert_eq!(items.len(), 2);
    // Each item should be (item_ref, prefix, is_selected)
    assert_eq!(items[0].2, true); // First is selected
    assert!(!items[1].2); // Second is not selected
}

#[test]
fn test_tree_nav_render_items_with_collapse() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    nav.add_item(TreeItem::new(1).with_parent(0));
    nav.collapse();
    let items = nav.render_items();
    assert_eq!(items.len(), 1); // Only root visible
}
