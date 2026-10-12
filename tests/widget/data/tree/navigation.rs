//! Tree navigation: non-selectable nodes, the multi-selection, keys, and an
//! empty tree
//!
//! Only behaviour that holds today is asserted. The multi-selection and the
//! matches drifting after expand/collapse (#859, #975) and the view not
//! following the selection (#973) are left to their issues.

use revue::event::Key;
use revue::widget::data::tree::{Tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn leaf(label: &str) -> TreeNode {
    TreeNode::new(label).id(label)
}

// =========================================================================
// Empty input
// =========================================================================

#[test]
fn test_tree_empty_ignores_every_action() {
    let mut tree = Tree::new().multi_select(true);
    tree.select_next();
    tree.select_prev();
    tree.select_first();
    tree.select_last();
    tree.toggle_expand();
    tree.expand();
    tree.collapse();
    tree.toggle_select();
    for key in [
        Key::Up,
        Key::Down,
        Key::Left,
        Key::Right,
        Key::Enter,
        Key::Home,
        Key::End,
    ] {
        assert!(!tree.handle_key(&key), "{key:?}");
    }
    assert_eq!(tree.selected_label(), None);
    assert!(tree.selected_nodes().is_empty());
    assert_eq!(tree.visible_count(), 0);
}

#[test]
fn test_tree_empty_searchable_takes_a_query_and_finds_nothing() {
    let mut tree = Tree::new().searchable(true);
    assert!(tree.handle_key(&Key::Char('a')));
    assert_eq!(tree.query(), "a");
    assert_eq!(tree.match_count(), 0);
    // With a query, `n` means "next match", not a letter; there is none
    assert!(!tree.handle_key(&Key::Char('n')));
    assert_eq!(tree.query(), "a");
}

// =========================================================================
// Non-selectable nodes
// =========================================================================

/// A, (B not selectable), C
fn with_gap() -> Tree {
    Tree::new().nodes(vec![leaf("A"), leaf("B").selectable(false), leaf("C")])
}

#[test]
fn test_tree_moves_skip_non_selectable_nodes() {
    let mut tree = with_gap();
    tree.select_next();
    assert_eq!(tree.selected_label(), Some("C"));
    tree.select_prev();
    assert_eq!(tree.selected_label(), Some("A"));
}

#[test]
fn test_tree_selection_settles_off_a_non_selectable_first_node() {
    let tree = Tree::new().nodes(vec![leaf("A").selectable(false), leaf("B")]);
    assert_eq!(tree.selected_label(), Some("B"));
}

#[test]
fn test_tree_first_and_last_skip_non_selectable_ends() {
    let mut tree = Tree::new().nodes(vec![
        leaf("A").selectable(false),
        leaf("B"),
        leaf("C"),
        leaf("D").selectable(false),
    ]);
    tree.select_last();
    assert_eq!(tree.selected_label(), Some("C"));
    tree.select_first();
    assert_eq!(tree.selected_label(), Some("B"));
}

#[test]
fn test_tree_left_goes_to_the_nearest_selectable_ancestor() {
    let mut tree = Tree::new().node(
        leaf("root").expanded(true).child(
            leaf("group")
                .selectable(false)
                .expanded(true)
                .child(leaf("item")),
        ),
    );
    tree.select_last();
    assert_eq!(tree.selected_label(), Some("item"));
    assert!(tree.handle_key(&Key::Left));
    assert_eq!(tree.selected_label(), Some("root"));
}

#[test]
fn test_tree_all_non_selectable_up_and_down_stay_put() {
    let mut tree = Tree::new().nodes(vec![
        leaf("A").selectable(false),
        leaf("B").selectable(false),
    ]);
    tree.select_next();
    assert_eq!(tree.selected_index(), 0);
    tree.select_prev();
    assert_eq!(tree.selected_index(), 0);
}

// =========================================================================
// Multi-selection
// =========================================================================

#[test]
fn test_tree_toggle_select_adds_and_removes() {
    let mut tree = Tree::new()
        .multi_select(true)
        .nodes(vec![leaf("A"), leaf("B"), leaf("C")]);
    tree.toggle_select();
    tree.select_last();
    tree.toggle_select();
    assert_eq!(tree.selected_ids(), ["A", "C"]);
    assert!(tree.is_multi_selected(0) && tree.is_multi_selected(2));
    tree.toggle_select();
    assert_eq!(tree.selected_ids(), ["A"]);
}

#[test]
fn test_tree_toggle_select_skips_non_selectable() {
    let mut tree = Tree::new().multi_select(true).nodes(vec![
        leaf("A").selectable(false),
        leaf("B").selectable(false),
    ]);
    tree.toggle_select();
    assert!(tree.selected_nodes().is_empty());
}

#[test]
fn test_tree_selected_ids_skip_nodes_without_an_id() {
    let mut tree = Tree::new()
        .multi_select(true)
        .nodes(vec![TreeNode::new("no id"), leaf("B")]);
    tree.toggle_select();
    tree.select_next();
    tree.toggle_select();
    assert_eq!(tree.selected_nodes().len(), 2);
    assert_eq!(tree.selected_ids(), ["B"]);
}

#[test]
fn test_tree_on_select_reports_the_toggled_node() {
    let seen = Rc::new(RefCell::new(Vec::new()));
    let log = seen.clone();
    let mut tree = Tree::new()
        .multi_select(true)
        .nodes(vec![leaf("A"), leaf("B")])
        .on_select(move |node| log.borrow_mut().push(node.label.clone()));
    tree.handle_key(&Key::Char(' '));
    tree.handle_key(&Key::Down);
    tree.handle_key(&Key::Char(' '));
    tree.handle_key(&Key::Char(' '));
    assert_eq!(*seen.borrow(), ["A", "B", "B"]);
}

// =========================================================================
// Keys
// =========================================================================

#[test]
fn test_tree_space_toggles_selection_in_multi_select_and_expands_otherwise() {
    let parent = || leaf("P").child(leaf("c"));
    let mut multi = Tree::new().multi_select(true).node(parent());
    assert!(multi.handle_key(&Key::Char(' ')));
    assert_eq!(multi.selected_ids(), ["P"]);
    assert_eq!(multi.visible_count(), 1, "not expanded");

    let mut single = Tree::new().node(parent());
    assert!(single.handle_key(&Key::Char(' ')));
    assert_eq!(single.visible_count(), 2);
}

#[test]
fn test_tree_searchable_keeps_space_and_letters_out_of_navigation() {
    let mut tree = Tree::new()
        .searchable(true)
        .multi_select(true)
        .nodes(vec![leaf("A").child(leaf("a1")), leaf("B")]);
    assert!(!tree.handle_key(&Key::Char(' ')));
    assert!(tree.selected_nodes().is_empty());
    assert!(tree.handle_key(&Key::Char('j')));
    assert_eq!(tree.query(), "j");
    assert!(tree.handle_key(&Key::Escape));
    assert!(tree.handle_key(&Key::Down), "arrows still move");
    assert_eq!(tree.selected_label(), Some("B"));
    tree.handle_key(&Key::Up);
    assert!(tree.handle_key(&Key::Right), "arrows still expand");
    assert_eq!(tree.visible_count(), 3);
}

#[test]
fn test_tree_key_results_report_changes_only() {
    let mut tree = Tree::new().nodes(vec![leaf("A").child(leaf("a1")), leaf("B")]);
    assert!(!tree.handle_key(&Key::Up), "already at the top");
    assert!(!tree.handle_key(&Key::Home));
    assert!(tree.handle_key(&Key::Right));
    assert!(!tree.handle_key(&Key::Right), "already expanded");
    assert!(tree.handle_key(&Key::Left), "collapses");
    assert!(!tree.handle_key(&Key::Left), "a root has no parent");
    assert!(tree.handle_key(&Key::End));
    assert!(!tree.handle_key(&Key::Down), "already at the bottom");
    assert!(!tree.handle_key(&Key::Char('x')));
}
