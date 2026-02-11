//! Tree widget type tests

use revue::widget::data::tree::types::TreeNode;

// =========================================================================
// Constructor tests
// =========================================================================

#[test]
fn test_tree_node_new_with_string() {
    let node = TreeNode::new("Test Label");
    assert_eq!(node.label, "Test Label");
    assert!(node.children.is_empty());
    assert!(!node.expanded);
}

#[test]
fn test_tree_node_new_with_str() {
    let node = TreeNode::new("Test Label");
    assert_eq!(node.label, "Test Label");
}

#[test]
fn test_tree_node_new_empty_string() {
    let node = TreeNode::new("");
    assert_eq!(node.label, "");
    assert!(node.children.is_empty());
}

#[test]
fn test_tree_node_leaf() {
    let node = TreeNode::leaf("Leaf Node");
    assert_eq!(node.label, "Leaf Node");
    assert!(node.children.is_empty());
    assert!(!node.expanded);
}

#[test]
fn test_tree_node_leaf_no_children() {
    let node = TreeNode::leaf("Test");
    assert!(!node.has_children());
}

// =========================================================================
// Builder method tests
// =========================================================================

#[test]
fn test_tree_node_child_single() {
    let child = TreeNode::new("Child");
    let node = TreeNode::new("Parent").child(child);

    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0].label, "Child");
}

#[test]
fn test_tree_node_child_multiple_chained() {
    let node = TreeNode::new("Parent")
        .child(TreeNode::new("Child 1"))
        .child(TreeNode::new("Child 2"))
        .child(TreeNode::new("Child 3"));

    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[0].label, "Child 1");
    assert_eq!(node.children[1].label, "Child 2");
    assert_eq!(node.children[2].label, "Child 3");
}

#[test]
fn test_tree_node_children_empty_vec() {
    let node = TreeNode::new("Parent").children(vec![]);
    assert!(node.children.is_empty());
}

#[test]
fn test_tree_node_children_single() {
    let children = vec![TreeNode::new("Only Child")];
    let node = TreeNode::new("Parent").children(children);
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0].label, "Only Child");
}

#[test]
fn test_tree_node_children_multiple() {
    let children = vec![
        TreeNode::new("Child 1"),
        TreeNode::new("Child 2"),
        TreeNode::new("Child 3"),
    ];
    let node = TreeNode::new("Parent").children(children);
    assert_eq!(node.children.len(), 3);
}

#[test]
fn test_tree_node_children_replaces_previous() {
    let node = TreeNode::new("Parent")
        .child(TreeNode::new("First Child"))
        .children(vec![TreeNode::new("New Child")]);

    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0].label, "New Child");
}

#[test]
fn test_tree_node_expanded_true() {
    let node = TreeNode::new("Test").expanded(true);
    assert!(node.expanded);
}

#[test]
fn test_tree_node_expanded_false() {
    let node = TreeNode::new("Test").expanded(false);
    assert!(!node.expanded);
}

#[test]
fn test_tree_node_expanded_default_is_false() {
    let node = TreeNode::new("Test");
    assert!(!node.expanded);
}

// =========================================================================
// Getter method tests
// =========================================================================

#[test]
fn test_tree_node_has_children_true() {
    let node = TreeNode::new("Parent").child(TreeNode::new("Child"));
    assert!(node.has_children());
}

#[test]
fn test_tree_node_has_children_false() {
    let node = TreeNode::new("Leaf");
    assert!(!node.has_children());
}

#[test]
fn test_tree_node_has_children_empty_vec() {
    let node = TreeNode::new("Test").children(vec![]);
    assert!(!node.has_children());
}

#[test]
fn test_tree_node_has_children_nested() {
    let node = TreeNode::new("Grandparent")
        .child(TreeNode::new("Parent").child(TreeNode::new("Child")));
    assert!(node.has_children());
    assert!(node.children[0].has_children());
    assert!(!node.children[0].children[0].has_children());
}

// =========================================================================
// Clone tests
// =========================================================================

#[test]
fn test_tree_node_clone_simple() {
    let node1 = TreeNode::new("Test");
    let node2 = node1.clone();
    assert_eq!(node1.label, node2.label);
    assert_eq!(node1.expanded, node2.expanded);
}

#[test]
fn test_tree_node_clone_with_children() {
    let node1 = TreeNode::new("Parent")
        .child(TreeNode::new("Child 1"))
        .child(TreeNode::new("Child 2"));
    let node2 = node1.clone();

    assert_eq!(node1.label, node2.label);
    assert_eq!(node1.children.len(), node2.children.len());
    assert_eq!(node1.children[0].label, node2.children[0].label);
    assert_eq!(node1.children[1].label, node2.children[1].label);
}

#[test]
fn test_tree_node_clone_with_expanded() {
    let node1 = TreeNode::new("Test").expanded(true);
    let node2 = node1.clone();
    assert!(node2.expanded);
}

#[test]
fn test_tree_node_clone_nested() {
    let node1 = TreeNode::new("Root").child(
        TreeNode::new("Level 1")
            .expanded(true)
            .child(TreeNode::new("Level 2")),
    );
    let node2 = node1.clone();

    assert_eq!(node1.children.len(), node2.children.len());
    assert_eq!(node1.children[0].expanded, node2.children[0].expanded);
    assert_eq!(
        node1.children[0].children.len(),
        node2.children[0].children.len()
    );
}

#[test]
fn test_tree_node_clone_independence() {
    let node1 = TreeNode::new("Parent").child(TreeNode::new("Child"));
    let mut node2 = node1.clone();

    node2.label = "Modified Parent".to_string();
    node2.children[0].label = "Modified Child".to_string();

    assert_eq!(node1.label, "Parent");
    assert_eq!(node1.children[0].label, "Child");
    assert_eq!(node2.label, "Modified Parent");
    assert_eq!(node2.children[0].label, "Modified Child");
}

// =========================================================================
// Complex scenarios
// =========================================================================

#[test]
fn test_tree_node_deep_hierarchy() {
    let node = TreeNode::new("Level 0").child(
        TreeNode::new("Level 1")
            .child(TreeNode::new("Level 2").child(TreeNode::new("Level 3"))),
    );

    assert!(node.has_children());
    assert!(node.children[0].has_children());
    assert!(node.children[0].children[0].has_children());
    assert!(!node.children[0].children[0].children[0].has_children());
}

#[test]
fn test_tree_node_multiple_siblings() {
    let node = TreeNode::new("Parent").children(vec![
        TreeNode::new("Child 1"),
        TreeNode::new("Child 2"),
        TreeNode::new("Child 3"),
        TreeNode::new("Child 4"),
    ]);

    assert_eq!(node.children.len(), 4);
    assert!(node.has_children());
}

#[test]
fn test_tree_node_builder_chain_complex() {
    let node = TreeNode::new("Root")
        .expanded(true)
        .child(
            TreeNode::new("Branch 1")
                .expanded(true)
                .child(TreeNode::new("Leaf 1.1"))
                .child(TreeNode::new("Leaf 1.2")),
        )
        .child(
            TreeNode::new("Branch 2")
                .expanded(false)
                .child(TreeNode::new("Leaf 2.1")),
        )
        .child(TreeNode::new("Leaf 3"));

    assert!(node.expanded);
    assert_eq!(node.children.len(), 3);
    assert!(node.children[0].expanded);
    assert!(!node.children[1].expanded);
    assert_eq!(node.children[0].children.len(), 2);
    assert_eq!(node.children[1].children.len(), 1);
    assert_eq!(node.children[2].children.len(), 0);
}

#[test]
fn test_tree_node_label_with_special_chars() {
    let node = TreeNode::new("path/to/file.txt");
    assert_eq!(node.label, "path/to/file.txt");
}

#[test]
fn test_tree_node_label_with_unicode() {
    let node = TreeNode::new("📁 文件夹");
    assert_eq!(node.label, "📁 文件夹");
}

#[test]
fn test_tree_node_label_with_whitespace() {
    let node = TreeNode::new("  spaced out  ");
    assert_eq!(node.label, "  spaced out  ");
}
