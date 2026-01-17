//! Data widget snapshot tests (Table, Tabs, List, Tree)

#![allow(unused_imports)]

use revue::prelude::*;
use revue::testing::{Pilot, TestApp, TestConfig};

#[test]
fn test_table_basic() {
    let view = Table::new(vec![
        Column::new("Name"),
        Column::new("Age"),
        Column::new("City"),
    ])
    .row(vec!["Alice", "30", "NYC"])
    .row(vec!["Bob", "25", "LA"])
    .row(vec!["Charlie", "35", "Chicago"]);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("table_basic");
}

#[test]
fn test_table_with_header() {
    let view = Table::new(vec![
        Column::new("ID"),
        Column::new("Product"),
        Column::new("Price"),
    ])
    .row(vec!["1", "Widget", "$9.99"])
    .row(vec!["2", "Gadget", "$19.99"]);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("table_with_header");
}

// =============================================================================
// Tabs Widget Tests
// =============================================================================

#[test]
fn test_tabs_basic() {
    let view = Tabs::new().tab("Home").tab("Settings").tab("About");

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("tabs_basic");
}

// =============================================================================
// List Widget Tests
// =============================================================================

#[test]
fn test_list_basic() {
    let view = List::new(vec!["Item 1", "Item 2", "Item 3", "Item 4", "Item 5"]);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("list_basic");
}

#[test]
fn test_list_selected() {
    let view = List::new(vec!["First", "Second", "Third"]).selected(1);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("list_selected");
}

// =============================================================================
// Tree Widget Tests
// =============================================================================

#[test]
fn test_tree_basic() {
    let view = Tree::new().node(
        TreeNode::new("Root")
            .expanded(true)
            .child(TreeNode::new("Child 1"))
            .child(
                TreeNode::new("Child 2")
                    .expanded(true)
                    .child(TreeNode::new("Grandchild 1"))
                    .child(TreeNode::new("Grandchild 2")),
            )
            .child(TreeNode::new("Child 3")),
    );

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("tree_basic");
}
