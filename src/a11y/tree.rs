//! Accessibility tree for screen reader navigation
//!
//! Provides a hierarchical representation of UI elements that can be
//! exposed to assistive technologies.

use crate::utils::accessibility::{AccessibleNode, AccessibleState, Role};
use std::collections::HashMap;

/// Unique identifier for tree nodes
pub type TreeNodeId = String;

/// A node in the accessibility tree
#[derive(Clone, Debug)]
pub struct TreeNode {
    /// Unique identifier
    pub id: TreeNodeId,
    /// Role of this element
    pub role: Role,
    /// Accessible name/label
    pub name: Option<String>,
    /// Accessible description
    pub description: Option<String>,
    /// Current state
    pub state: AccessibleState,
    /// Bounding rectangle (x, y, width, height)
    pub bounds: Option<(u16, u16, u16, u16)>,
    /// Parent node ID
    pub parent: Option<TreeNodeId>,
    /// Child node IDs
    pub children: Vec<TreeNodeId>,
    /// Custom properties
    pub properties: HashMap<String, String>,
}

impl TreeNode {
    /// Create a new tree node
    pub fn new(id: impl Into<TreeNodeId>, role: Role) -> Self {
        Self {
            id: id.into(),
            role,
            name: None,
            description: None,
            state: AccessibleState::default(),
            bounds: None,
            parent: None,
            children: Vec::new(),
            properties: HashMap::new(),
        }
    }

    /// Set the accessible name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set the state
    pub fn state(mut self, state: AccessibleState) -> Self {
        self.state = state;
        self
    }

    /// Set bounds
    pub fn bounds(mut self, x: u16, y: u16, width: u16, height: u16) -> Self {
        self.bounds = Some((x, y, width, height));
        self
    }

    /// Add a property
    pub fn property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    /// Get the accessible name for announcement
    pub fn accessible_name(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| self.role.name().to_string())
    }

    /// Generate description for screen readers
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();

        // Name and role
        if let Some(name) = &self.name {
            parts.push(format!("{}, {}", name, self.role.name()));
        } else {
            parts.push(self.role.name().to_string());
        }

        // State information
        if self.state.disabled {
            parts.push("disabled".to_string());
        }
        if let Some(checked) = self.state.checked {
            parts.push(if checked { "checked" } else { "not checked" }.to_string());
        }
        if let Some(expanded) = self.state.expanded {
            parts.push(if expanded { "expanded" } else { "collapsed" }.to_string());
        }
        if self.state.selected {
            parts.push("selected".to_string());
        }
        if let (Some(pos), Some(size)) = (self.state.pos_in_set, self.state.set_size) {
            parts.push(format!("{} of {}", pos, size));
        }

        // Value
        if let Some(text) = &self.state.value_text {
            parts.push(text.clone());
        } else if let Some(now) = self.state.value_now {
            if let (Some(min), Some(max)) = (self.state.value_min, self.state.value_max) {
                if (max - min).abs() > f64::EPSILON {
                    let percent = ((now - min) / (max - min) * 100.0) as i32;
                    parts.push(format!("{}%", percent));
                }
            }
        }

        // Description
        if let Some(desc) = &self.description {
            parts.push(desc.clone());
        }

        parts.join(", ")
    }

    /// Check if this node is focusable
    pub fn is_focusable(&self) -> bool {
        self.role.is_interactive() && !self.state.disabled && !self.state.hidden
    }

    /// Convert from AccessibleNode
    pub fn from_accessible_node(node: &AccessibleNode) -> Self {
        Self {
            id: node.id.clone(),
            role: node.role,
            name: node.label.clone(),
            description: node.description.clone(),
            state: node.state.clone(),
            bounds: None,
            parent: node.parent.clone(),
            children: node.children.clone(),
            properties: node.properties.clone(),
        }
    }
}

/// Complete accessibility tree
#[derive(Clone, Debug, Default)]
pub struct AccessibilityTree {
    /// All nodes by ID
    nodes: HashMap<TreeNodeId, TreeNode>,
    /// Root node ID
    root: Option<TreeNodeId>,
    /// Currently focused node ID
    focus: Option<TreeNodeId>,
}

impl AccessibilityTree {
    /// Create a new empty tree
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a tree with a root node
    pub fn with_root(root: TreeNode) -> Self {
        let root_id = root.id.clone();
        let mut tree = Self::new();
        tree.add_node(root);
        tree.root = Some(root_id);
        tree
    }

    /// Set the root node
    pub fn set_root(&mut self, id: TreeNodeId) {
        self.root = Some(id);
    }

    /// Get the root node
    pub fn root(&self) -> Option<&TreeNode> {
        self.root.as_ref().and_then(|id| self.nodes.get(id))
    }

    /// Add a node to the tree
    pub fn add_node(&mut self, node: TreeNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Remove a node and its descendants
    pub fn remove_node(&mut self, id: &TreeNodeId) -> Option<TreeNode> {
        if let Some(node) = self.nodes.remove(id) {
            // Remove from parent's children
            if let Some(parent_id) = &node.parent {
                if let Some(parent) = self.nodes.get_mut(parent_id) {
                    parent.children.retain(|c| c != id);
                }
            }

            // Recursively remove children
            for child_id in &node.children {
                self.remove_node(child_id);
            }

            // Clear focus if focused node was removed
            if self.focus.as_ref() == Some(id) {
                self.focus = None;
            }

            Some(node)
        } else {
            None
        }
    }

    /// Get a node by ID
    pub fn get(&self, id: &TreeNodeId) -> Option<&TreeNode> {
        self.nodes.get(id)
    }

    /// Get a mutable node by ID
    pub fn get_mut(&mut self, id: &TreeNodeId) -> Option<&mut TreeNode> {
        self.nodes.get_mut(id)
    }

    /// Add a child to a parent node
    pub fn add_child(&mut self, parent_id: &TreeNodeId, mut child: TreeNode) {
        child.parent = Some(parent_id.clone());
        let child_id = child.id.clone();

        self.nodes.insert(child_id.clone(), child);

        if let Some(parent) = self.nodes.get_mut(parent_id) {
            parent.children.push(child_id);
        }
    }

    /// Set focus to a node
    pub fn set_focus(&mut self, id: &TreeNodeId) -> bool {
        if let Some(node) = self.nodes.get(id) {
            if node.is_focusable() {
                // Clear old focus
                if let Some(old_id) = &self.focus {
                    if let Some(old_node) = self.nodes.get_mut(old_id) {
                        old_node.state.focused = false;
                    }
                }

                // Set new focus
                if let Some(new_node) = self.nodes.get_mut(id) {
                    new_node.state.focused = true;
                }

                self.focus = Some(id.clone());
                return true;
            }
        }
        false
    }

    /// Get currently focused node
    pub fn focused(&self) -> Option<&TreeNode> {
        self.focus.as_ref().and_then(|id| self.nodes.get(id))
    }

    /// Get focused node ID
    pub fn focus_id(&self) -> Option<&TreeNodeId> {
        self.focus.as_ref()
    }

    /// Move focus to next focusable element
    pub fn focus_next(&mut self) -> Option<&TreeNode> {
        let focusable: Vec<TreeNodeId> = self
            .nodes
            .values()
            .filter(|n| n.is_focusable())
            .map(|n| n.id.clone())
            .collect();

        if focusable.is_empty() {
            return None;
        }

        let current_idx = self
            .focus
            .as_ref()
            .and_then(|id| focusable.iter().position(|fid| fid == id))
            .unwrap_or(0);

        let next_idx = (current_idx + 1) % focusable.len();
        let next_id = focusable[next_idx].clone();

        self.set_focus(&next_id);
        self.focused()
    }

    /// Move focus to previous focusable element
    pub fn focus_prev(&mut self) -> Option<&TreeNode> {
        let focusable: Vec<TreeNodeId> = self
            .nodes
            .values()
            .filter(|n| n.is_focusable())
            .map(|n| n.id.clone())
            .collect();

        if focusable.is_empty() {
            return None;
        }

        let current_idx = self
            .focus
            .as_ref()
            .and_then(|id| focusable.iter().position(|fid| fid == id))
            .unwrap_or(0);

        let prev_idx = if current_idx == 0 {
            focusable.len() - 1
        } else {
            current_idx - 1
        };

        let prev_id = focusable[prev_idx].clone();
        self.set_focus(&prev_id);
        self.focused()
    }

    /// Get all nodes
    pub fn nodes(&self) -> impl Iterator<Item = &TreeNode> {
        self.nodes.values()
    }

    /// Get all focusable nodes
    pub fn focusable_nodes(&self) -> Vec<&TreeNode> {
        self.nodes.values().filter(|n| n.is_focusable()).collect()
    }

    /// Get all landmark nodes
    pub fn landmarks(&self) -> Vec<&TreeNode> {
        self.nodes
            .values()
            .filter(|n| n.role.is_landmark())
            .collect()
    }

    /// Get children of a node
    pub fn children(&self, id: &TreeNodeId) -> Vec<&TreeNode> {
        self.nodes
            .get(id)
            .map(|node| {
                node.children
                    .iter()
                    .filter_map(|cid| self.nodes.get(cid))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get parent of a node
    pub fn parent(&self, id: &TreeNodeId) -> Option<&TreeNode> {
        self.nodes
            .get(id)
            .and_then(|node| node.parent.as_ref())
            .and_then(|pid| self.nodes.get(pid))
    }

    /// Get ancestors of a node (from immediate parent to root)
    pub fn ancestors(&self, id: &TreeNodeId) -> Vec<&TreeNode> {
        let mut result = Vec::new();
        let mut current = self.nodes.get(id);

        while let Some(node) = current {
            if let Some(parent) = node.parent.as_ref().and_then(|pid| self.nodes.get(pid)) {
                result.push(parent);
                current = Some(parent);
            } else {
                break;
            }
        }

        result
    }

    /// Get path from root to a node
    pub fn path_to(&self, id: &TreeNodeId) -> Vec<&TreeNode> {
        let mut ancestors = self.ancestors(id);
        ancestors.reverse();
        if let Some(node) = self.nodes.get(id) {
            ancestors.push(node);
        }
        ancestors
    }

    /// Clear the tree
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.root = None;
        self.focus = None;
    }

    /// Get node count
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Check if tree is empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Generate a text representation for debugging
    pub fn debug_string(&self) -> String {
        let mut output = String::new();

        fn print_node(
            tree: &AccessibilityTree,
            id: &TreeNodeId,
            depth: usize,
            output: &mut String,
        ) {
            let indent = "  ".repeat(depth);
            if let Some(node) = tree.get(id) {
                let focus_marker = if node.state.focused { " *" } else { "" };
                output.push_str(&format!(
                    "{}- {} [{}]{}\n",
                    indent,
                    node.accessible_name(),
                    node.role.name(),
                    focus_marker
                ));

                for child_id in &node.children {
                    print_node(tree, child_id, depth + 1, output);
                }
            }
        }

        if let Some(root_id) = &self.root {
            print_node(self, root_id, 0, &mut output);
        }

        output
    }
}

/// Builder for constructing accessibility trees
pub struct AccessibilityTreeBuilder {
    tree: AccessibilityTree,
    current_parent: Option<TreeNodeId>,
}

impl AccessibilityTreeBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            tree: AccessibilityTree::new(),
            current_parent: None,
        }
    }

    /// Add the root node
    pub fn root(mut self, node: TreeNode) -> Self {
        let id = node.id.clone();
        self.tree.add_node(node);
        self.tree.set_root(id.clone());
        self.current_parent = Some(id);
        self
    }

    /// Add a child to the current parent
    pub fn child(mut self, node: TreeNode) -> Self {
        if let Some(parent_id) = &self.current_parent {
            self.tree.add_child(parent_id, node);
        } else {
            self.tree.add_node(node);
        }
        self
    }

    /// Start a new group (push current as parent)
    pub fn begin_group(mut self, node: TreeNode) -> Self {
        let id = node.id.clone();
        if let Some(parent_id) = &self.current_parent {
            self.tree.add_child(parent_id, node);
        } else {
            self.tree.add_node(node);
        }
        self.current_parent = Some(id);
        self
    }

    /// End current group (pop to parent)
    pub fn end_group(mut self) -> Self {
        if let Some(current_id) = &self.current_parent {
            if let Some(node) = self.tree.get(current_id) {
                self.current_parent = node.parent.clone();
            }
        }
        self
    }

    /// Build the tree
    pub fn build(self) -> AccessibilityTree {
        self.tree
    }
}

impl Default for AccessibilityTreeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_new() {
        let node = TreeNode::new("btn1", Role::Button).name("Submit");

        assert_eq!(node.id, "btn1");
        assert_eq!(node.role, Role::Button);
        assert_eq!(node.name, Some("Submit".to_string()));
    }

    #[test]
    fn test_tree_node_describe() {
        let node = TreeNode::new("cb1", Role::Checkbox)
            .name("Accept terms")
            .state(AccessibleState::new().checked(true));

        let desc = node.describe();
        assert!(desc.contains("Accept terms"));
        assert!(desc.contains("checkbox"));
        assert!(desc.contains("checked"));
    }

    #[test]
    fn test_tree_node_focusable() {
        let button = TreeNode::new("btn", Role::Button);
        assert!(button.is_focusable());

        let disabled_button =
            TreeNode::new("btn2", Role::Button).state(AccessibleState::new().disabled(true));
        assert!(!disabled_button.is_focusable());

        let container = TreeNode::new("div", Role::Generic);
        assert!(!container.is_focusable());
    }

    #[test]
    fn test_accessibility_tree_new() {
        let tree = AccessibilityTree::new();
        assert!(tree.is_empty());
        assert!(tree.root().is_none());
    }

    #[test]
    fn test_accessibility_tree_with_root() {
        let root = TreeNode::new("root", Role::Main).name("Application");
        let tree = AccessibilityTree::with_root(root);

        assert!(!tree.is_empty());
        assert!(tree.root().is_some());
        assert_eq!(tree.root().unwrap().name, Some("Application".to_string()));
    }

    #[test]
    fn test_accessibility_tree_add_child() {
        let mut tree = AccessibilityTree::new();

        let root = TreeNode::new("root", Role::Main);
        tree.add_node(root);
        tree.set_root("root".to_string());

        let button = TreeNode::new("btn", Role::Button).name("Click me");
        tree.add_child(&"root".to_string(), button);

        assert_eq!(tree.len(), 2);
        assert!(tree.get(&"btn".to_string()).is_some());

        let children = tree.children(&"root".to_string());
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id, "btn");
    }

    #[test]
    fn test_accessibility_tree_focus() {
        let mut tree = AccessibilityTree::new();

        let btn1 = TreeNode::new("btn1", Role::Button).name("First");
        let btn2 = TreeNode::new("btn2", Role::Button).name("Second");

        tree.add_node(btn1);
        tree.add_node(btn2);

        assert!(tree.set_focus(&"btn1".to_string()));
        assert_eq!(tree.focus_id(), Some(&"btn1".to_string()));
        assert!(tree.focused().unwrap().state.focused);

        tree.focus_next();
        assert_eq!(tree.focus_id(), Some(&"btn2".to_string()));
    }

    #[test]
    fn test_accessibility_tree_focus_prev() {
        let mut tree = AccessibilityTree::new();

        let btn1 = TreeNode::new("btn1", Role::Button);
        let btn2 = TreeNode::new("btn2", Role::Button);

        tree.add_node(btn1);
        tree.add_node(btn2);

        tree.set_focus(&"btn2".to_string());
        tree.focus_prev();

        assert_eq!(tree.focus_id(), Some(&"btn1".to_string()));
    }

    #[test]
    fn test_accessibility_tree_remove_node() {
        let mut tree = AccessibilityTree::new();

        let root = TreeNode::new("root", Role::Main);
        tree.add_node(root);
        tree.set_root("root".to_string());

        let button = TreeNode::new("btn", Role::Button);
        tree.add_child(&"root".to_string(), button);

        assert_eq!(tree.len(), 2);

        tree.remove_node(&"btn".to_string());
        assert_eq!(tree.len(), 1);
        assert!(tree.get(&"btn".to_string()).is_none());
    }

    #[test]
    fn test_accessibility_tree_landmarks() {
        let mut tree = AccessibilityTree::new();

        tree.add_node(TreeNode::new("nav", Role::Navigation));
        tree.add_node(TreeNode::new("main", Role::Main));
        tree.add_node(TreeNode::new("btn", Role::Button));

        let landmarks = tree.landmarks();
        assert_eq!(landmarks.len(), 2);
    }

    #[test]
    fn test_accessibility_tree_builder() {
        let tree = AccessibilityTreeBuilder::new()
            .root(TreeNode::new("root", Role::Main).name("App"))
            .begin_group(TreeNode::new("form", Role::Form).name("Login Form"))
            .child(TreeNode::new("username", Role::TextInput).name("Username"))
            .child(TreeNode::new("password", Role::TextInput).name("Password"))
            .child(TreeNode::new("submit", Role::Button).name("Login"))
            .end_group()
            .build();

        assert_eq!(tree.len(), 5);
        assert!(tree.root().is_some());

        let form_children = tree.children(&"form".to_string());
        assert_eq!(form_children.len(), 3);
    }

    #[test]
    fn test_accessibility_tree_ancestors() {
        let tree = AccessibilityTreeBuilder::new()
            .root(TreeNode::new("root", Role::Main))
            .begin_group(TreeNode::new("level1", Role::Group))
            .begin_group(TreeNode::new("level2", Role::Group))
            .child(TreeNode::new("leaf", Role::Button))
            .end_group()
            .end_group()
            .build();

        let ancestors = tree.ancestors(&"leaf".to_string());
        assert_eq!(ancestors.len(), 3); // level2, level1, root
        assert_eq!(ancestors[0].id, "level2");
        assert_eq!(ancestors[1].id, "level1");
        assert_eq!(ancestors[2].id, "root");
    }

    #[test]
    fn test_accessibility_tree_path_to() {
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
    fn test_accessibility_tree_debug_string() {
        let tree = AccessibilityTreeBuilder::new()
            .root(TreeNode::new("root", Role::Main).name("App"))
            .child(TreeNode::new("btn", Role::Button).name("Click"))
            .build();

        let debug = tree.debug_string();
        assert!(debug.contains("App"));
        assert!(debug.contains("main"));
        assert!(debug.contains("Click"));
    }

    #[test]
    fn test_tree_node_from_accessible_node() {
        let accessible = AccessibleNode::with_id("test", Role::Button)
            .label("Test Button")
            .description("A test button");

        let tree_node = TreeNode::from_accessible_node(&accessible);

        assert_eq!(tree_node.id, "test");
        assert_eq!(tree_node.role, Role::Button);
        assert_eq!(tree_node.name, Some("Test Button".to_string()));
        assert_eq!(tree_node.description, Some("A test button".to_string()));
    }

    #[test]
    fn test_tree_clear() {
        let mut tree = AccessibilityTreeBuilder::new()
            .root(TreeNode::new("root", Role::Main))
            .child(TreeNode::new("btn", Role::Button))
            .build();

        tree.set_focus(&"btn".to_string());

        tree.clear();
        assert!(tree.is_empty());
        assert!(tree.root().is_none());
        assert!(tree.focus_id().is_none());
    }

    // =========================================================================
    // Additional a11y tree tests
    // =========================================================================

    #[test]
    fn test_tree_node_new_with_string_id() {
        let id = String::from("test_id");
        let node = TreeNode::new(id.clone(), Role::Button);
        assert_eq!(node.id, id);
    }

    #[test]
    fn test_tree_node_description() {
        let node = TreeNode::new("test", Role::Button).description("Click to submit");
        assert_eq!(node.description, Some("Click to submit".to_string()));
    }

    #[test]
    fn test_tree_node_state_builder() {
        let state = AccessibleState::new().disabled(true).selected(true);
        let node = TreeNode::new("test", Role::Button).state(state);
        assert!(node.state.disabled);
        assert!(node.state.selected);
    }

    #[test]
    fn test_tree_node_bounds() {
        let node = TreeNode::new("test", Role::Button).bounds(10, 20, 100, 50);
        assert_eq!(node.bounds, Some((10, 20, 100, 50)));
    }

    #[test]
    fn test_tree_node_property() {
        let node = TreeNode::new("test", Role::Button)
            .property("custom", "value")
            .property("data", "123");
        assert_eq!(node.properties.get("custom"), Some(&"value".to_string()));
        assert_eq!(node.properties.get("data"), Some(&"123".to_string()));
    }

    #[test]
    fn test_tree_node_accessible_name_fallback() {
        let node = TreeNode::new("test", Role::Button);
        assert_eq!(node.accessible_name(), "button");
    }

    #[test]
    fn test_tree_node_describe_with_value() {
        let node = TreeNode::new("slider", Role::Slider)
            .name("Volume")
            .state(AccessibleState::new().value_range(0.5, 0.0, 1.0));
        let desc = node.describe();
        assert!(desc.contains("Volume"));
        assert!(desc.contains("slider"));
        assert!(desc.contains("50%"));
    }

    #[test]
    fn test_tree_node_describe_with_position() {
        let node =
            TreeNode::new("item3", Role::ListItem).state(AccessibleState::new().position(3, 10));
        let desc = node.describe();
        assert!(desc.contains("3 of 10"));
    }

    #[test]
    fn test_tree_node_describe_expanded() {
        let node = TreeNode::new("group", Role::Group).state(AccessibleState::new().expanded(true));
        let desc = node.describe();
        assert!(desc.contains("expanded"));
    }

    #[test]
    fn test_tree_node_describe_collapsed() {
        let node =
            TreeNode::new("group", Role::Group).state(AccessibleState::new().expanded(false));
        let desc = node.describe();
        assert!(desc.contains("collapsed"));
    }

    #[test]
    fn test_tree_node_describe_disabled() {
        let node = TreeNode::new("btn", Role::Button).state(AccessibleState::new().disabled(true));
        let desc = node.describe();
        assert!(desc.contains("disabled"));
    }

    #[test]
    fn test_tree_node_describe_selected() {
        let node =
            TreeNode::new("item", Role::ListItem).state(AccessibleState::new().selected(true));
        let desc = node.describe();
        assert!(desc.contains("selected"));
    }

    #[test]
    fn test_tree_node_is_focusable_hidden() {
        let mut state = AccessibleState::new();
        state.hidden = true;
        let node = TreeNode::new("btn", Role::Button).state(state);
        assert!(!node.is_focusable());
    }

    #[test]
    fn test_accessibility_tree_default() {
        let tree = AccessibilityTree::default();
        assert!(tree.is_empty());
    }

    #[test]
    fn test_accessibility_tree_set_root() {
        let mut tree = AccessibilityTree::new();
        tree.set_root("root_id".to_string());
        assert_eq!(tree.root, Some("root_id".to_string()));
    }

    #[test]
    fn test_accessibility_tree_get() {
        let mut tree = AccessibilityTree::new();
        let node = TreeNode::new("test", Role::Button);
        tree.add_node(node);
        assert!(tree.get(&"test".to_string()).is_some());
    }

    #[test]
    fn test_accessibility_tree_get_mut() {
        let mut tree = AccessibilityTree::new();
        let node = TreeNode::new("test", Role::Button);
        tree.add_node(node);
        if let Some(node_mut) = tree.get_mut(&"test".to_string()) {
            node_mut.name = Some("Modified".to_string());
        }
        assert_eq!(
            tree.get(&"test".to_string()).unwrap().name,
            Some("Modified".to_string())
        );
    }

    #[test]
    fn test_accessibility_tree_add_node() {
        let mut tree = AccessibilityTree::new();
        let node = TreeNode::new("test", Role::Button);
        tree.add_node(node);
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_accessibility_tree_remove_nonexistent() {
        let mut tree = AccessibilityTree::new();
        let result = tree.remove_node(&"nonexistent".to_string());
        assert!(result.is_none());
    }

    #[test]
    fn test_accessibility_tree_focus_nonexistent() {
        let mut tree = AccessibilityTree::new();
        assert!(!tree.set_focus(&"nonexistent".to_string()));
    }

    #[test]
    fn test_accessibility_tree_focus_next_empty() {
        let mut tree = AccessibilityTree::new();
        assert!(tree.focus_next().is_none());
    }

    #[test]
    fn test_accessibility_tree_focus_prev_empty() {
        let mut tree = AccessibilityTree::new();
        assert!(tree.focus_prev().is_none());
    }

    #[test]
    fn test_accessibility_tree_focus_next_wraps() {
        let mut tree = AccessibilityTree::new();
        tree.add_node(TreeNode::new("btn1", Role::Button));
        tree.add_node(TreeNode::new("btn2", Role::Button));
        tree.set_focus(&"btn2".to_string());
        tree.focus_next();
        assert_eq!(tree.focus_id(), Some(&"btn1".to_string()));
    }

    #[test]
    fn test_accessibility_tree_focus_prev_wraps() {
        let mut tree = AccessibilityTree::new();
        tree.add_node(TreeNode::new("btn1", Role::Button));
        tree.add_node(TreeNode::new("btn2", Role::Button));
        tree.set_focus(&"btn1".to_string());
        tree.focus_prev();
        assert_eq!(tree.focus_id(), Some(&"btn2".to_string()));
    }

    #[test]
    fn test_accessibility_tree_nodes_iteration() {
        let mut tree = AccessibilityTree::new();
        tree.add_node(TreeNode::new("a", Role::Button));
        tree.add_node(TreeNode::new("b", Role::Button));
        let count = tree.nodes().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_accessibility_tree_focusable_nodes() {
        let mut tree = AccessibilityTree::new();
        tree.add_node(TreeNode::new("btn1", Role::Button));
        tree.add_node(TreeNode::new("btn2", Role::Button));
        tree.add_node(TreeNode::new("div", Role::Generic));
        let focusable = tree.focusable_nodes();
        assert_eq!(focusable.len(), 2);
    }

    #[test]
    fn test_accessibility_tree_parent() {
        let mut tree = AccessibilityTree::new();
        tree.add_node(TreeNode::new("root", Role::Main));
        tree.add_child(&"root".to_string(), TreeNode::new("child", Role::Button));
        let parent = tree.parent(&"child".to_string());
        assert!(parent.is_some());
        assert_eq!(parent.unwrap().id, "root");
    }

    #[test]
    fn test_accessibility_tree_parent_of_root() {
        let tree = AccessibilityTreeBuilder::new()
            .root(TreeNode::new("root", Role::Main))
            .build();
        let parent = tree.parent(&"root".to_string());
        assert!(parent.is_none());
    }

    #[test]
    fn test_accessibility_tree_ancestors_of_root() {
        let tree = AccessibilityTreeBuilder::new()
            .root(TreeNode::new("root", Role::Main))
            .build();
        let ancestors = tree.ancestors(&"root".to_string());
        assert!(ancestors.is_empty());
    }

    #[test]
    fn test_accessibility_tree_path_to_root() {
        let tree = AccessibilityTreeBuilder::new()
            .root(TreeNode::new("root", Role::Main))
            .build();
        let path = tree.path_to(&"root".to_string());
        assert_eq!(path.len(), 1);
        assert_eq!(path[0].id, "root");
    }

    #[test]
    fn test_accessibility_tree_len() {
        let mut tree = AccessibilityTree::new();
        tree.add_node(TreeNode::new("a", Role::Button));
        tree.add_node(TreeNode::new("b", Role::Button));
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn test_accessibility_tree_set_focus_clears_old() {
        let mut tree = AccessibilityTree::new();
        tree.add_node(TreeNode::new("btn1", Role::Button));
        tree.add_node(TreeNode::new("btn2", Role::Button));
        tree.set_focus(&"btn1".to_string());
        tree.set_focus(&"btn2".to_string());
        assert!(tree.get(&"btn1".to_string()).unwrap().state.focused == false);
        assert!(tree.get(&"btn2".to_string()).unwrap().state.focused == true);
    }

    #[test]
    fn test_accessibility_tree_builder_default() {
        let builder = AccessibilityTreeBuilder::default();
        assert!(builder.tree.is_empty());
    }

    #[test]
    fn test_accessibility_tree_builder_without_root() {
        let tree = AccessibilityTreeBuilder::new()
            .child(TreeNode::new("orphan", Role::Button))
            .build();
        assert!(tree.get(&"orphan".to_string()).is_some());
    }

    #[test]
    fn test_accessibility_tree_remove_clears_focus() {
        let mut tree = AccessibilityTree::new();
        tree.add_node(TreeNode::new("btn1", Role::Button));
        tree.add_node(TreeNode::new("btn2", Role::Button));
        tree.set_focus(&"btn1".to_string());
        tree.remove_node(&"btn1".to_string());
        assert!(tree.focus_id().is_none());
    }

    #[test]
    fn test_tree_node_clone() {
        let node = TreeNode::new("test", Role::Button)
            .name("Test")
            .description("A button");
        let cloned = node.clone();
        assert_eq!(node.id, cloned.id);
        assert_eq!(node.name, cloned.name);
    }

    #[test]
    fn test_accessibility_tree_clone() {
        let tree = AccessibilityTreeBuilder::new()
            .root(TreeNode::new("root", Role::Main))
            .child(TreeNode::new("btn", Role::Button))
            .build();
        let cloned = tree.clone();
        assert_eq!(tree.len(), cloned.len());
    }

    #[test]
    fn test_tree_node_describe_with_value_text() {
        let mut state = AccessibleState::new();
        state.value_text = Some("50%".to_string());
        let node = TreeNode::new("slider", Role::Slider).state(state);
        let desc = node.describe();
        assert!(desc.contains("50%"));
    }

    #[test]
    fn test_accessibility_tree_debug_string_with_focus() {
        let mut tree = AccessibilityTreeBuilder::new()
            .root(TreeNode::new("root", Role::Main).name("App"))
            .child(TreeNode::new("btn", Role::Button).name("Click"))
            .build();
        tree.set_focus(&"btn".to_string());
        let debug = tree.debug_string();
        assert!(debug.contains("*"));
    }

    #[test]
    fn test_accessibility_tree_children_of_nonexistent() {
        let tree = AccessibilityTree::new();
        let children = tree.children(&"nonexistent".to_string());
        assert!(children.is_empty());
    }

    #[test]
    fn test_accessibility_tree_children_empty() {
        let mut tree = AccessibilityTree::new();
        tree.add_node(TreeNode::new("childless", Role::Group));
        let children = tree.children(&"childless".to_string());
        assert!(children.is_empty());
    }

    #[test]
    fn test_accessibility_tree_multiple_children() {
        let mut tree = AccessibilityTree::new();
        tree.add_node(TreeNode::new("root", Role::Main));
        tree.add_child(&"root".to_string(), TreeNode::new("child1", Role::Button));
        tree.add_child(&"root".to_string(), TreeNode::new("child2", Role::Button));
        let children = tree.children(&"root".to_string());
        assert_eq!(children.len(), 2);
    }
}
