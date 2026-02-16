//! Tree widget types

/// A tree node
#[derive(Clone)]
pub struct TreeNode {
    /// Node label
    pub label: String,
    /// Child nodes
    pub children: Vec<TreeNode>,
    /// Whether node is expanded
    pub expanded: bool,
}

impl TreeNode {
    /// Create a new tree node
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            children: Vec::new(),
            expanded: false,
        }
    }

    /// Add a child node
    pub fn child(mut self, node: TreeNode) -> Self {
        self.children.push(node);
        self
    }

    /// Add multiple children
    pub fn children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    /// Set expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Check if node has children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Create a leaf node (no children)
    pub fn leaf(label: impl Into<String>) -> Self {
        Self::new(label)
    }
}

// Tests moved to tests/widget/data/tree_types.rs
