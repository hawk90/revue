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
    /// Optional unique identifier
    pub id: Option<String>,
    /// Optional icon character displayed before the label
    pub icon: Option<char>,
    /// Whether this node can be selected (default: true)
    pub selectable: bool,
}

impl TreeNode {
    /// Create a new tree node
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            children: Vec::new(),
            expanded: false,
            id: None,
            icon: None,
            selectable: true,
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

    /// Set node ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set icon character displayed before the label
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set whether this node can be selected
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
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
