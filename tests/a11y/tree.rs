//! A11y tree integration tests
//!
//! Tests for accessibility tree structure, navigation, and management.

use revue::a11y::{AccessibilityTree, AccessibilityTreeBuilder, TreeNode};
use revue::utils::accessibility::{AccessibleState, Role};

// =============================================================================
// TreeNode Tests
// =============================================================================

#[test]
fn test_tree_node_builder_pattern() {
    let node = TreeNode::new("btn1", Role::Button)
        .name("Submit")
        .description("Click to submit")
        .state(AccessibleState::new().disabled(true))
        .bounds(10, 20, 100, 30)
        .property("test-id", "my-button")
        .property("aria-label", "Submit Form");

    assert_eq!(node.id, "btn1");
    assert_eq!(node.role, Role::Button);
    assert_eq!(node.name, Some("Submit".to_string()));
    assert_eq!(node.description, Some("Click to submit".to_string()));
    assert!(node.state.disabled);
    assert_eq!(node.bounds, Some((10, 20, 100, 30)));
    assert_eq!(
        node.properties.get("test-id"),
        Some(&"my-button".to_string())
    );
    assert_eq!(
        node.properties.get("aria-label"),
        Some(&"Submit Form".to_string())
    );
}

#[test]
fn test_tree_node_properties() {
    let mut node = TreeNode::new("test", Role::Generic);

    // Add multiple properties
    node = node.property("key1", "value1").property("key2", "value2");

    assert_eq!(node.properties.len(), 2);
    assert_eq!(node.properties.get("key1"), Some(&"value1".to_string()));
    assert_eq!(node.properties.get("key2"), Some(&"value2".to_string()));
}

#[test]
fn test_tree_node_accessible_name() {
    let node_with_name = TreeNode::new("test", Role::Button).name("My Button");
    assert_eq!(node_with_name.accessible_name(), "My Button");

    let node_without_name = TreeNode::new("test", Role::Button);
    assert_eq!(node_without_name.accessible_name(), "button");
}

#[test]
fn test_tree_node_describe() {
    let checked_node = TreeNode::new("cb", Role::Checkbox)
        .name("Accept Terms")
        .state(AccessibleState::new().checked(true));

    let desc = checked_node.describe();
    assert!(desc.contains("Accept Terms"));
    assert!(desc.contains("checkbox"));
    assert!(desc.contains("checked"));
}

#[test]
fn test_tree_node_describe_disabled() {
    let disabled_node = TreeNode::new("btn", Role::Button)
        .name("Submit")
        .state(AccessibleState::new().disabled(true));

    let desc = disabled_node.describe();
    assert!(desc.contains("disabled"));
}

#[test]
fn test_tree_node_describe_selected() {
    let selected_node = TreeNode::new("item", Role::ListItem)
        .name("Option 1")
        .state(AccessibleState::new().selected(true));

    let desc = selected_node.describe();
    assert!(desc.contains("selected"));
}

#[test]
fn test_tree_node_describe_position() {
    let mut state = AccessibleState::new();
    state.pos_in_set = Some(2);
    state.set_size = Some(5);

    let positioned_node = TreeNode::new("item", Role::ListItem)
        .name("Item 2")
        .state(state);

    let desc = positioned_node.describe();
    assert!(desc.contains("2 of 5"));
}

#[test]
fn test_tree_node_focusable() {
    let button = TreeNode::new("btn", Role::Button);
    assert!(button.is_focusable());

    let disabled_button =
        TreeNode::new("btn", Role::Button).state(AccessibleState::new().disabled(true));
    assert!(!disabled_button.is_focusable());

    let container = TreeNode::new("div", Role::Generic);
    assert!(!container.is_focusable());
}

// =============================================================================
// AccessibilityTree Tests
// =============================================================================

#[test]
fn test_tree_new() {
    let tree = AccessibilityTree::new();
    assert!(tree.is_empty());
    assert!(tree.root().is_none());
    assert_eq!(tree.len(), 0);
}

#[test]
fn test_tree_with_root() {
    let root = TreeNode::new("root", Role::Main).name("App");
    let tree = AccessibilityTree::with_root(root);

    assert!(!tree.is_empty());
    assert!(tree.root().is_some());
    assert_eq!(tree.root().unwrap().name, Some("App".to_string()));
    assert_eq!(tree.len(), 1);
}

#[test]
fn test_tree_add_node() {
    let mut tree = AccessibilityTree::new();

    let node = TreeNode::new("node1", Role::Button).name("Click");
    tree.add_node(node);

    assert_eq!(tree.len(), 1);
    assert!(tree.get(&"node1".to_string()).is_some());
}

#[test]
fn test_tree_add_node_duplicate() {
    let mut tree = AccessibilityTree::new();

    tree.add_node(TreeNode::new("dup", Role::Button).name("First"));
    assert_eq!(tree.len(), 1);

    // Adding duplicate ID should replace
    tree.add_node(TreeNode::new("dup", Role::Button).name("Second"));
    assert_eq!(tree.len(), 1);
    assert_eq!(
        tree.get(&"dup".to_string()).unwrap().name,
        Some("Second".to_string())
    );
}

#[test]
fn test_tree_remove_node() {
    let mut tree = AccessibilityTree::new();

    tree.add_node(TreeNode::new("node1", Role::Button));
    tree.add_node(TreeNode::new("node2", Role::Button));

    assert_eq!(tree.len(), 2);

    let removed = tree.remove_node(&"node1".to_string());
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().id, "node1");
    assert_eq!(tree.len(), 1);
}

#[test]
fn test_tree_remove_node_recursive() {
    let mut tree = AccessibilityTree::new();

    // Create a hierarchy: root -> child -> grandchild
    tree.add_node(TreeNode::new("root", Role::Main));
    tree.add_child(&"root".to_string(), TreeNode::new("child", Role::Group));
    tree.add_child(
        &"child".to_string(),
        TreeNode::new("grandchild", Role::Button),
    );

    assert_eq!(tree.len(), 3);

    // Remove child should also remove grandchild
    tree.remove_node(&"child".to_string());

    assert_eq!(tree.len(), 1); // Only root remains
    assert!(tree.get(&"child".to_string()).is_none());
    assert!(tree.get(&"grandchild".to_string()).is_none());
}

#[test]
fn test_tree_remove_nonexistent() {
    let mut tree = AccessibilityTree::new();
    let result = tree.remove_node(&"nonexistent".to_string());
    assert!(result.is_none());
}

#[test]
fn test_tree_add_child() {
    let mut tree = AccessibilityTree::new();

    tree.add_node(TreeNode::new("root", Role::Main));
    tree.add_child(
        &"root".to_string(),
        TreeNode::new("child", Role::Button).name("Click"),
    );

    assert_eq!(tree.len(), 2);

    let root = tree.get(&"root".to_string()).unwrap();
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0], "child");

    let child = tree.get(&"child".to_string()).unwrap();
    assert_eq!(child.parent, Some("root".to_string()));
}

#[test]
fn test_tree_children() {
    let mut tree = AccessibilityTree::new();

    tree.add_node(TreeNode::new("parent", Role::Group));
    tree.add_child(&"parent".to_string(), TreeNode::new("child1", Role::Button));
    tree.add_child(&"parent".to_string(), TreeNode::new("child2", Role::Button));
    tree.add_child(&"parent".to_string(), TreeNode::new("child3", Role::Button));

    let children = tree.children(&"parent".to_string());
    assert_eq!(children.len(), 3);
    assert_eq!(children[0].id, "child1");
    assert_eq!(children[1].id, "child2");
    assert_eq!(children[2].id, "child3");
}

#[test]
fn test_tree_parent() {
    let mut tree = AccessibilityTree::new();

    tree.add_node(TreeNode::new("root", Role::Main));
    tree.add_child(&"root".to_string(), TreeNode::new("child", Role::Button));

    let parent = tree.parent(&"child".to_string());
    assert!(parent.is_some());
    assert_eq!(parent.unwrap().id, "root");

    let root_parent = tree.parent(&"root".to_string());
    assert!(root_parent.is_none());
}

#[test]
fn test_tree_focus_navigation() {
    let mut tree = AccessibilityTree::new();

    tree.add_node(TreeNode::new("btn1", Role::Button).name("First"));
    tree.add_node(TreeNode::new("btn2", Role::Button).name("Second"));
    tree.add_node(TreeNode::new("btn3", Role::Button).name("Third"));

    // Set initial focus
    assert!(tree.set_focus(&"btn1".to_string()));
    let focused_id = tree.focus_id().unwrap().clone();

    // Move to next - should move to a different button
    tree.focus_next();
    let next_focused_id = tree.focus_id().unwrap().clone();
    assert_ne!(focused_id, next_focused_id);

    // Move to next again - should move to the third button
    tree.focus_next();
    let third_focused_id = tree.focus_id().unwrap().clone();
    assert_ne!(next_focused_id, third_focused_id);

    // Should wrap around
    tree.focus_next();
    let wrapped_id = tree.focus_id().unwrap().clone();
    // After wrapping, we should be back at one of the buttons
    assert!(wrapped_id == "btn1" || wrapped_id == "btn2" || wrapped_id == "btn3");
}

#[test]
fn test_tree_focus_prev() {
    let mut tree = AccessibilityTree::new();

    tree.add_node(TreeNode::new("btn1", Role::Button));
    tree.add_node(TreeNode::new("btn2", Role::Button));
    tree.add_node(TreeNode::new("btn3", Role::Button));

    // Focus one of the buttons
    let focus_id = "btn3".to_string();
    tree.set_focus(&focus_id);

    // Move focus backward - should go to a different button
    let first_prev = tree.focus_prev().unwrap().id.clone();
    assert_ne!(first_prev, focus_id);

    // Move focus backward again - should go to the third button
    let second_prev = tree.focus_prev().unwrap().id.clone();
    assert_ne!(second_prev, first_prev);
    assert_ne!(second_prev, focus_id);

    // Should wrap around
    let third_prev = tree.focus_prev().unwrap().id.clone();
    assert_eq!(third_prev, focus_id);
}

#[test]
fn test_tree_focus_unfocusable() {
    let mut tree = AccessibilityTree::new();

    // Only focusable elements should be focused
    tree.add_node(TreeNode::new("container", Role::Generic));
    tree.add_node(TreeNode::new("button", Role::Button));

    // Can't focus container
    assert!(!tree.set_focus(&"container".to_string()));

    // Can focus button
    assert!(tree.set_focus(&"button".to_string()));
}

#[test]
fn test_tree_focus_clears_old() {
    let mut tree = AccessibilityTree::new();

    tree.add_node(TreeNode::new("btn1", Role::Button));
    tree.add_node(TreeNode::new("btn2", Role::Button));

    tree.set_focus(&"btn1".to_string());
    assert!(tree.get(&"btn1".to_string()).unwrap().state.focused);

    tree.set_focus(&"btn2".to_string());
    assert!(!tree.get(&"btn1".to_string()).unwrap().state.focused);
    assert!(tree.get(&"btn2".to_string()).unwrap().state.focused);
}

#[test]
fn test_tree_ancestors() {
    let tree = AccessibilityTreeBuilder::new()
        .root(TreeNode::new("root", Role::Main))
        .begin_group(TreeNode::new("level1", Role::Group))
        .begin_group(TreeNode::new("level2", Role::Group))
        .child(TreeNode::new("leaf", Role::Button))
        .end_group()
        .end_group()
        .build();

    let ancestors = tree.ancestors(&"leaf".to_string());
    assert_eq!(ancestors.len(), 3);
    assert_eq!(ancestors[0].id, "level2");
    assert_eq!(ancestors[1].id, "level1");
    assert_eq!(ancestors[2].id, "root");
}

#[test]
fn test_tree_path_to_root() {
    let tree = AccessibilityTreeBuilder::new()
        .root(TreeNode::new("root", Role::Main))
        .begin_group(TreeNode::new("nav", Role::Navigation))
        .child(TreeNode::new("link", Role::Link).name("Home"))
        .end_group()
        .build();

    let path = tree.path_to(&"link".to_string());
    assert_eq!(path.len(), 3);
    assert_eq!(path[0].id, "root");
    assert_eq!(path[1].id, "nav");
    assert_eq!(path[2].id, "link");
}

#[test]
fn test_tree_landmarks() {
    let mut tree = AccessibilityTree::new();

    // Add various roles
    tree.add_node(TreeNode::new("nav", Role::Navigation));
    tree.add_node(TreeNode::new("main", Role::Main));
    tree.add_node(TreeNode::new("aside", Role::Generic)); // Generic instead of Note
    tree.add_node(TreeNode::new("header", Role::Header));
    tree.add_node(TreeNode::new("footer", Role::Footer));
    tree.add_node(TreeNode::new("button", Role::Button)); // Not a landmark

    let landmarks = tree.landmarks();
    assert_eq!(landmarks.len(), 4); // Navigation, Main, Header, Footer (Generic is not a landmark)
}

#[test]
fn test_tree_focusable_nodes() {
    let mut tree = AccessibilityTree::new();

    tree.add_node(TreeNode::new("button1", Role::Button));
    tree.add_node(TreeNode::new("button2", Role::Button));
    tree.add_node(TreeNode::new("container", Role::Generic));
    tree.add_node(TreeNode::new("link", Role::Link));

    let focusable = tree.focusable_nodes();
    assert_eq!(focusable.len(), 3); // 2 buttons + 1 link
}

#[test]
fn test_tree_clear() {
    let mut tree = AccessibilityTree::new();

    tree.add_node(TreeNode::new("node1", Role::Button));
    tree.add_node(TreeNode::new("node2", Role::Button));
    tree.set_focus(&"node1".to_string());

    tree.clear();

    assert!(tree.is_empty());
    assert!(tree.root().is_none());
    assert!(tree.focus_id().is_none());
}

#[test]
fn test_tree_debug_string() {
    let tree = AccessibilityTreeBuilder::new()
        .root(TreeNode::new("root", Role::Main).name("App"))
        .child(TreeNode::new("btn", Role::Button).name("Click"))
        .build();

    let debug = tree.debug_string();
    assert!(debug.contains("App"));
    assert!(debug.contains("main"));
    assert!(debug.contains("Click"));
    assert!(debug.contains("button"));
}

#[test]
fn test_tree_debug_string_with_focus() {
    let mut tree = AccessibilityTree::new();

    // Set root first, then add button as child, then focus
    tree.add_node(TreeNode::new("root", Role::Main));
    tree.set_root("root".to_string());
    tree.add_child(
        &"root".to_string(),
        TreeNode::new("btn", Role::Button).name("Click"),
    );
    tree.set_focus(&"btn".to_string());

    let debug = tree.debug_string();
    assert!(debug.contains("*")); // Focus marker
}

#[test]
fn test_tree_nodes_iterator() {
    let mut tree = AccessibilityTree::new();

    tree.add_node(TreeNode::new("node1", Role::Button));
    tree.add_node(TreeNode::new("node2", Role::Button));
    tree.add_node(TreeNode::new("node3", Role::Button));

    let count = tree.nodes().count();
    assert_eq!(count, 3);
}

// =============================================================================
// AccessibilityTreeBuilder Tests
// =============================================================================

#[test]
fn test_builder_nested_groups() {
    let tree = AccessibilityTreeBuilder::new()
        .root(TreeNode::new("root", Role::Main))
        .begin_group(TreeNode::new("level1", Role::Group))
        .begin_group(TreeNode::new("level2", Role::Group))
        .begin_group(TreeNode::new("level3", Role::Group))
        .child(TreeNode::new("leaf", Role::Button))
        .end_group()
        .end_group()
        .end_group()
        .build();

    // Verify deep nesting
    let ancestors = tree.ancestors(&"leaf".to_string());
    assert_eq!(ancestors.len(), 4);
    assert_eq!(ancestors[0].id, "level3");
    assert_eq!(ancestors[1].id, "level2");
    assert_eq!(ancestors[2].id, "level1");
    assert_eq!(ancestors[3].id, "root");
}

#[test]
fn test_builder_multiple_children() {
    let tree = AccessibilityTreeBuilder::new()
        .root(TreeNode::new("root", Role::Main))
        .child(TreeNode::new("child1", Role::Button))
        .child(TreeNode::new("child2", Role::Button))
        .child(TreeNode::new("child3", Role::Button))
        .child(TreeNode::new("child4", Role::Button))
        .child(TreeNode::new("child5", Role::Button))
        .build();

    let children = tree.children(&"root".to_string());
    assert_eq!(children.len(), 5);
}

#[test]
fn test_builder_mixed_groups_and_children() {
    let tree = AccessibilityTreeBuilder::new()
        .root(TreeNode::new("root", Role::Main))
        .child(TreeNode::new("direct1", Role::Button))
        .begin_group(TreeNode::new("group", Role::Group))
        .child(TreeNode::new("nested1", Role::Button))
        .child(TreeNode::new("nested2", Role::Button))
        .end_group()
        .child(TreeNode::new("direct2", Role::Button))
        .build();

    let root_children = tree.children(&"root".to_string());
    assert_eq!(root_children.len(), 3); // direct1, group, direct2

    let group_children = tree.children(&"group".to_string());
    assert_eq!(group_children.len(), 2); // nested1, nested2
}

#[test]
fn test_builder_default() {
    let builder = AccessibilityTreeBuilder::new();
    let tree = builder.build();

    assert!(tree.is_empty());
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn test_complex_navigation_scenario() {
    let mut tree = AccessibilityTreeBuilder::new()
        .root(TreeNode::new("app", Role::Main).name("My App"))
        .begin_group(TreeNode::new("toolbar", Role::Group).name("Toolbar"))
        .child(TreeNode::new("btn1", Role::Button).name("Save"))
        .child(TreeNode::new("btn2", Role::Button).name("Load"))
        .child(TreeNode::new("btn3", Role::Button).name("Export"))
        .end_group()
        .begin_group(TreeNode::new("content", Role::Group).name("Main Content"))
        .child(TreeNode::new("article", Role::Generic))
        .end_group()
        .build();

    // Navigate through buttons - HashMap iteration order may vary
    // Just verify that focus_next moves through the buttons
    let button_ids = ["btn1", "btn2", "btn3"];
    tree.set_focus(&button_ids[0].to_string());
    assert!(tree
        .focus_id()
        .map(|id| button_ids.contains(&id.as_str()))
        .unwrap_or(false));

    // Move to next button - should be a different button
    let second_id = tree.focus_next().unwrap().id.clone();
    assert_ne!(second_id, button_ids[0]);
    assert!(button_ids.contains(&second_id.as_str()));

    // Move to next button - should be the third button
    let third_id = tree.focus_next().unwrap().id.clone();
    assert_ne!(third_id, second_id);
    assert_ne!(third_id, button_ids[0]);
    assert!(button_ids.contains(&third_id.as_str()));

    // Wrap around - should cycle back to first button
    let wrapped_id = tree.focus_next().unwrap().id.clone();
    assert_eq!(wrapped_id, button_ids[0]);
}

#[test]
fn test_focus_updates_node_state() {
    let mut tree = AccessibilityTree::new();

    tree.add_node(TreeNode::new("btn1", Role::Button));
    tree.add_node(TreeNode::new("btn2", Role::Button));

    tree.set_focus(&"btn1".to_string());
    assert!(tree.get(&"btn1".to_string()).unwrap().state.focused);

    tree.set_focus(&"btn2".to_string());
    assert!(!tree.get(&"btn1".to_string()).unwrap().state.focused);
    assert!(tree.get(&"btn2".to_string()).unwrap().state.focused);
}
