//! Layout tree data structure
//!
//! Stores all layout nodes in a flat HashMap for efficient access.

use super::node::LayoutNode;
use std::collections::HashMap;

/// A tree structure for layout nodes
#[derive(Debug, Default)]
pub struct LayoutTree {
    /// All nodes indexed by ID
    nodes: HashMap<u64, LayoutNode>,
    /// Root node ID
    root: Option<u64>,
}

impl LayoutTree {
    /// Create a new empty layout tree
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root: None,
        }
    }

    /// Insert a node into the tree
    pub fn insert(&mut self, node: LayoutNode) {
        let id = node.id;
        self.nodes.insert(id, node);
    }

    /// Remove a node from the tree
    pub fn remove(&mut self, id: u64) -> Option<LayoutNode> {
        // Also remove from parent's children list
        if let Some(node) = self.nodes.get(&id) {
            if let Some(parent_id) = node.parent {
                if let Some(parent) = self.nodes.get_mut(&parent_id) {
                    parent.children.retain(|&child_id| child_id != id);
                }
            }
        }

        // Clear root if removing root node
        if self.root == Some(id) {
            self.root = None;
        }

        self.nodes.remove(&id)
    }

    /// Get a node by ID
    pub fn get(&self, id: u64) -> Option<&LayoutNode> {
        self.nodes.get(&id)
    }

    /// Get a mutable reference to a node by ID
    pub fn get_mut(&mut self, id: u64) -> Option<&mut LayoutNode> {
        self.nodes.get_mut(&id)
    }

    /// Check if a node exists
    pub fn contains(&self, id: u64) -> bool {
        self.nodes.contains_key(&id)
    }

    /// Set the root node ID
    #[allow(dead_code)]
    pub fn set_root(&mut self, id: u64) {
        self.root = Some(id);
    }

    /// Get the root node ID
    #[allow(dead_code)]
    pub fn root(&self) -> Option<u64> {
        self.root
    }

    /// Get children IDs for a node
    #[allow(dead_code)]
    pub fn children(&self, id: u64) -> &[u64] {
        self.nodes
            .get(&id)
            .map(|n| n.children.as_slice())
            .unwrap_or(&[])
    }

    /// Clear all nodes
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.root = None;
    }

    /// Get the number of nodes
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Check if the tree is empty
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Add a child to a parent node
    pub fn add_child(&mut self, parent_id: u64, child_id: u64) {
        // Set child's parent
        if let Some(child) = self.nodes.get_mut(&child_id) {
            child.parent = Some(parent_id);
        }

        // Add to parent's children list
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            if !parent.children.contains(&child_id) {
                parent.children.push(child_id);
            }
        }
    }

    /// Set children for a node (replaces existing children)
    #[allow(dead_code)]
    pub fn set_children(&mut self, parent_id: u64, children: Vec<u64>) {
        // Update parent references for all children
        for &child_id in &children {
            if let Some(child) = self.nodes.get_mut(&child_id) {
                child.parent = Some(parent_id);
            }
        }

        // Set children list
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children = children;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(id: u64) -> LayoutNode {
        let mut node = LayoutNode::default();
        node.id = id;
        node
    }

    #[test]
    fn test_tree_new() {
        let tree = LayoutTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert!(tree.root().is_none());
    }

    #[test]
    fn test_tree_insert_get() {
        let mut tree = LayoutTree::new();
        tree.insert(make_node(1));

        assert!(tree.contains(1));
        assert!(!tree.contains(2));

        let node = tree.get(1).unwrap();
        assert_eq!(node.id, 1);
    }

    #[test]
    fn test_tree_remove() {
        let mut tree = LayoutTree::new();
        tree.insert(make_node(1));
        tree.insert(make_node(2));

        assert_eq!(tree.len(), 2);

        let removed = tree.remove(1);
        assert!(removed.is_some());
        assert_eq!(tree.len(), 1);
        assert!(!tree.contains(1));
        assert!(tree.contains(2));
    }

    #[test]
    fn test_tree_root() {
        let mut tree = LayoutTree::new();
        tree.insert(make_node(1));
        tree.set_root(1);

        assert_eq!(tree.root(), Some(1));

        tree.remove(1);
        assert!(tree.root().is_none());
    }

    #[test]
    fn test_tree_parent_child() {
        let mut tree = LayoutTree::new();
        tree.insert(make_node(1));
        tree.insert(make_node(2));
        tree.insert(make_node(3));

        tree.add_child(1, 2);
        tree.add_child(1, 3);

        let parent = tree.get(1).unwrap();
        assert_eq!(parent.children, vec![2, 3]);

        let child = tree.get(2).unwrap();
        assert_eq!(child.parent, Some(1));
    }

    #[test]
    fn test_tree_set_children() {
        let mut tree = LayoutTree::new();
        tree.insert(make_node(1));
        tree.insert(make_node(2));
        tree.insert(make_node(3));

        tree.set_children(1, vec![2, 3]);

        assert_eq!(tree.children(1), &[2, 3]);
        assert_eq!(tree.get(2).unwrap().parent, Some(1));
        assert_eq!(tree.get(3).unwrap().parent, Some(1));
    }

    #[test]
    fn test_tree_clear() {
        let mut tree = LayoutTree::new();
        tree.insert(make_node(1));
        tree.insert(make_node(2));
        tree.set_root(1);

        tree.clear();

        assert!(tree.is_empty());
        assert!(tree.root().is_none());
    }

    #[test]
    fn test_remove_updates_parent() {
        let mut tree = LayoutTree::new();
        tree.insert(make_node(1));
        tree.insert(make_node(2));
        tree.add_child(1, 2);

        assert_eq!(tree.get(1).unwrap().children, vec![2]);

        tree.remove(2);

        assert!(tree.get(1).unwrap().children.is_empty());
    }
}
