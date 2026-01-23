#![allow(unused_imports)]

//! Tests for tree utilities

use crate::utils::tree::navigation::TreeNav;
use crate::utils::tree::prefix::tree_chars;
use crate::utils::tree::prefix::TreePrefix;
use crate::utils::tree::types::Indent;
use crate::utils::tree::types::TreeIcons;
use crate::utils::tree::types::TreeItem;

// ========================================================================
// TreePrefix tests
// ========================================================================

#[test]
fn test_tree_prefix_new() {
    let prefix = TreePrefix::new();
    assert_eq!(prefix.depth(), 0);
}

#[test]
fn test_tree_prefix_push_pop() {
    let mut prefix = TreePrefix::new();
    assert_eq!(prefix.depth(), 0);

    prefix.push(true);
    assert_eq!(prefix.depth(), 1);

    prefix.push(false);
    assert_eq!(prefix.depth(), 2);

    prefix.pop();
    assert_eq!(prefix.depth(), 1);

    prefix.pop();
    assert_eq!(prefix.depth(), 0);
}

#[test]
fn test_tree_prefix_basic() {
    let prefix = TreePrefix::new();
    assert_eq!(prefix.prefix(false), "├─");
    assert_eq!(prefix.prefix(true), "└─");
}

#[test]
fn test_tree_prefix_nested() {
    let mut prefix = TreePrefix::new();
    prefix.push(true); // Parent has more siblings
    assert_eq!(prefix.prefix(false), "│ ├─");
    assert_eq!(prefix.prefix(true), "│ └─");
}

#[test]
fn test_tree_prefix_no_more_siblings() {
    let mut prefix = TreePrefix::new();
    prefix.push(false); // Parent is last
    assert_eq!(prefix.prefix(false), "  ├─");
    assert_eq!(prefix.prefix(true), "  └─");
}

#[test]
fn test_tree_prefix_deep_nesting() {
    let mut prefix = TreePrefix::new();
    prefix.push(true);
    prefix.push(false);
    prefix.push(true);
    assert_eq!(prefix.depth(), 3);
    assert!(prefix.prefix(false).contains("│"));
}

#[test]
fn test_tree_prefix_continuation() {
    let prefix = TreePrefix::new();
    assert_eq!(prefix.continuation(), "  ");

    let mut prefix2 = TreePrefix::new();
    prefix2.push(true);
    assert_eq!(prefix2.continuation(), "│   ");
}

#[test]
fn test_tree_prefix_clear() {
    let mut prefix = TreePrefix::new();
    prefix.push(true);
    prefix.push(false);
    assert_eq!(prefix.depth(), 2);

    prefix.clear();
    assert_eq!(prefix.depth(), 0);
}

#[test]
fn test_tree_prefix_default() {
    let prefix = TreePrefix::default();
    assert_eq!(prefix.depth(), 0);
}

// ========================================================================
// Indent tests
// ========================================================================

#[test]
fn test_indent_new() {
    let indent = Indent::new(4);
    assert_eq!(indent.unit, 4);
}

#[test]
fn test_indent_default() {
    let indent = Indent::default();
    assert_eq!(indent.unit, 2);
}

#[test]
fn test_indent_level() {
    let indent = Indent::new(4);
    assert_eq!(indent.level(0), 0);
    assert_eq!(indent.level(1), 4);
    assert_eq!(indent.level(2), 8);
    assert_eq!(indent.level(5), 20);
}

#[test]
fn test_indent_at() {
    let indent = Indent::new(3);
    assert_eq!(indent.at(0), 0);
    assert_eq!(indent.at(1), 3);
    assert_eq!(indent.at(4), 12);
}

// ========================================================================
// TreeIcons tests
// ========================================================================

#[test]
fn test_tree_icons_default() {
    let icons = TreeIcons::default();
    assert_eq!(icons.selected, ">");
    assert_eq!(icons.collapsed, "▶");
    assert_eq!(icons.expanded, "▼");
    assert_eq!(icons.blank, " ");
}

#[test]
fn test_tree_icons_selection() {
    let icons = TreeIcons::default();
    assert_eq!(icons.selection(true), ">");
    assert_eq!(icons.selection(false), " ");
}

#[test]
fn test_tree_icons_collapse() {
    let icons = TreeIcons::default();
    assert_eq!(icons.collapse(true), "▶");
    assert_eq!(icons.collapse(false), "▼");
}

// ========================================================================
// TreeItem tests
// ========================================================================

#[test]
fn test_tree_item_new() {
    let item = TreeItem::new(5);
    assert_eq!(item.id, 5);
    assert_eq!(item.parent, None);
    assert_eq!(item.depth, 0);
    assert!(!item.collapsible);
    assert_eq!(item.row_count, 1);
}

#[test]
fn test_tree_item_with_parent() {
    let item = TreeItem::new(1).with_parent(0);
    assert_eq!(item.parent, Some(0));
}

#[test]
fn test_tree_item_with_depth() {
    let item = TreeItem::new(1).with_depth(3);
    assert_eq!(item.depth, 3);
}

#[test]
fn test_tree_item_collapsible() {
    let item = TreeItem::new(0).collapsible();
    assert!(item.collapsible);
}

#[test]
fn test_tree_item_with_row_count() {
    let item = TreeItem::new(0).with_row_count(5);
    assert_eq!(item.row_count, 5);
}

#[test]
fn test_tree_item_builder_chain() {
    let item = TreeItem::new(1)
        .with_parent(0)
        .with_depth(2)
        .collapsible()
        .with_row_count(3);

    assert_eq!(item.id, 1);
    assert_eq!(item.parent, Some(0));
    assert_eq!(item.depth, 2);
    assert!(item.collapsible);
    assert_eq!(item.row_count, 3);
}

// ========================================================================
// TreeNav tests
// ========================================================================

#[test]
fn test_tree_nav_new() {
    let nav = TreeNav::new();
    assert_eq!(nav.selected(), 0);
    assert_eq!(nav.selected_row(), 0);
}

#[test]
fn test_tree_nav_default() {
    let nav = TreeNav::default();
    assert_eq!(nav.selected(), 0);
}

#[test]
fn test_tree_nav_add_item() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    assert_eq!(nav.visible_items().len(), 2);
}

#[test]
fn test_tree_nav_clear() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    nav.next_item();

    nav.clear();
    assert_eq!(nav.selected(), 0);
    assert_eq!(nav.visible_items().len(), 0);
}

#[test]
fn test_tree_nav_basic_navigation() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    nav.add_item(TreeItem::new(2));

    assert_eq!(nav.selected(), 0);
    nav.next_item();
    assert_eq!(nav.selected(), 1);
    nav.next_item();
    assert_eq!(nav.selected(), 2);
    nav.next_item();
    assert_eq!(nav.selected(), 0); // Wrap around
}

#[test]
fn test_tree_nav_prev_item() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    nav.add_item(TreeItem::new(2));

    assert_eq!(nav.selected(), 0);
    nav.prev_item();
    assert_eq!(nav.selected(), 2); // Wrap to last

    nav.prev_item();
    assert_eq!(nav.selected(), 1);
}

#[test]
fn test_tree_nav_collapse() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    nav.add_item(TreeItem::new(1).with_parent(0));
    nav.add_item(TreeItem::new(2).with_parent(0));
    nav.add_item(TreeItem::new(3));

    assert!(nav.is_visible(1));
    assert!(nav.is_visible(2));
    nav.toggle_collapse(); // Collapse item 0
    assert!(!nav.is_visible(1));
    assert!(!nav.is_visible(2));
    assert!(nav.is_visible(3));
}

#[test]
fn test_tree_nav_expand() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    nav.add_item(TreeItem::new(1).with_parent(0));

    nav.collapse();
    assert!(nav.is_collapsed(0));

    nav.expand();
    assert!(!nav.is_collapsed(0));
}

#[test]
fn test_tree_nav_collapse_all_expand_all() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).collapsible());
    nav.add_item(TreeItem::new(1).with_parent(0).collapsible());
    nav.add_item(TreeItem::new(2).with_parent(1));
    nav.add_item(TreeItem::new(3).collapsible());

    nav.collapse_all();
    assert!(nav.is_collapsed(0));
    assert!(nav.is_collapsed(1));
    assert!(nav.is_collapsed(3));

    nav.expand_all();
    assert!(!nav.is_collapsed(0));
    assert!(!nav.is_collapsed(1));
    assert!(!nav.is_collapsed(3));
}

#[test]
fn test_tree_nav_select() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    nav.add_item(TreeItem::new(2));

    nav.select(2);
    assert_eq!(nav.selected(), 2);

    nav.select(0);
    assert_eq!(nav.selected(), 0);
}

#[test]
fn test_tree_nav_first_last() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));
    nav.add_item(TreeItem::new(2));

    nav.last();
    assert_eq!(nav.selected(), 2);

    nav.first();
    assert_eq!(nav.selected(), 0);
}

#[test]
fn test_tree_nav_multi_row_items() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).with_row_count(3));
    nav.add_item(TreeItem::new(1));

    assert_eq!(nav.selected_row(), 0);

    // Next should move within item first
    nav.next();
    assert_eq!(nav.selected(), 0);
    assert_eq!(nav.selected_row(), 1);

    nav.next();
    assert_eq!(nav.selected(), 0);
    assert_eq!(nav.selected_row(), 2);

    // Now should move to next item
    nav.next();
    assert_eq!(nav.selected(), 1);
    assert_eq!(nav.selected_row(), 0);
}

#[test]
fn test_tree_nav_prev_multi_row() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).with_row_count(2));
    nav.add_item(TreeItem::new(1));

    nav.next_item(); // Move to item 1
    assert_eq!(nav.selected(), 1);

    nav.prev(); // Should go to last row of item 0
    assert_eq!(nav.selected(), 0);
    assert_eq!(nav.selected_row(), 1);
}

#[test]
fn test_tree_nav_render_items() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0));
    nav.add_item(TreeItem::new(1));

    let items = nav.render_items();
    assert_eq!(items.len(), 2);

    // First item should be selected
    assert!(items[0].2);
    assert!(!items[1].2);
}

#[test]
fn test_tree_nav_is_last_sibling() {
    let mut nav = TreeNav::new();
    nav.add_item(TreeItem::new(0).with_depth(0));
    nav.add_item(TreeItem::new(1).with_depth(0));
    nav.add_item(TreeItem::new(2).with_depth(0));

    assert!(!nav.is_last_sibling(0));
    assert!(!nav.is_last_sibling(1));
    assert!(nav.is_last_sibling(2));
}

#[test]
fn test_tree_nav_empty() {
    let mut nav = TreeNav::new();
    nav.next_item();
    nav.prev_item();
    nav.first();
    nav.last();
    // Should not crash on empty nav
    assert_eq!(nav.selected(), 0);
}

// ========================================================================
// tree_chars tests
// ========================================================================

#[test]
fn test_tree_chars() {
    assert_eq!(tree_chars::BRANCH, "├─");
    assert_eq!(tree_chars::LAST, "└─");
    assert_eq!(tree_chars::PIPE, "│ ");
    assert_eq!(tree_chars::SPACE, "  ");
}
