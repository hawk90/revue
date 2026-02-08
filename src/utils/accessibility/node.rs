//! Accessible node representing a widget

use super::{generate_id, AccessibleState, Role};
use std::collections::HashMap;

/// Accessible node representing a widget
#[derive(Clone, Debug)]
pub struct AccessibleNode {
    /// Node ID
    pub id: String,
    /// Role
    pub role: Role,
    /// Label (accessible name)
    pub label: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Keyboard shortcut
    pub shortcut: Option<String>,
    /// State
    pub state: AccessibleState,
    /// Additional properties
    pub properties: HashMap<String, String>,
    /// Child node IDs
    pub children: Vec<String>,
    /// Parent node ID
    pub parent: Option<String>,
}

impl AccessibleNode {
    /// Create new accessible node
    pub fn new(role: Role) -> Self {
        Self {
            id: generate_id(),
            role,
            label: None,
            description: None,
            shortcut: None,
            state: AccessibleState::default(),
            properties: HashMap::new(),
            children: Vec::new(),
            parent: None,
        }
    }

    /// Create with specific ID
    pub fn with_id(id: impl Into<String>, role: Role) -> Self {
        Self {
            id: id.into(),
            role,
            label: None,
            description: None,
            shortcut: None,
            state: AccessibleState::default(),
            properties: HashMap::new(),
            children: Vec::new(),
            parent: None,
        }
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set keyboard shortcut
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set state
    pub fn state(mut self, state: AccessibleState) -> Self {
        self.state = state;
        self
    }

    /// Add property
    pub fn property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    /// Add child
    pub fn child(mut self, child_id: impl Into<String>) -> Self {
        self.children.push(child_id.into());
        self
    }

    /// Set parent
    pub fn parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent = Some(parent_id.into());
        self
    }

    /// Get accessible name (label or fallback)
    pub fn accessible_name(&self) -> &str {
        self.label.as_deref().unwrap_or(self.role.name())
    }

    /// Check if node can receive focus
    pub fn is_focusable(&self) -> bool {
        self.role.is_interactive() && !self.state.disabled && !self.state.hidden
    }

    /// Generate description for screen readers
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();

        // Role and label
        if let Some(label) = &self.label {
            parts.push(format!("{}, {}", label, self.role.name()));
        } else {
            parts.push(self.role.name().to_string());
        }

        // State
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

        // Position
        if let (Some(pos), Some(size)) = (self.state.pos_in_set, self.state.set_size) {
            parts.push(format!("{} of {}", pos, size));
        }

        // Value
        if let Some(value) = &self.state.value_text {
            parts.push(value.clone());
        } else if let Some(now) = self.state.value_now {
            if let (Some(min), Some(max)) = (self.state.value_min, self.state.value_max) {
                let percent = ((now - min) / (max - min) * 100.0) as i32;
                parts.push(format!("{}%", percent));
            }
        }

        // Description
        if let Some(desc) = &self.description {
            parts.push(desc.clone());
        }

        // Shortcut
        if let Some(shortcut) = &self.shortcut {
            parts.push(format!("Press {}", shortcut));
        }

        parts.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // AccessibleNode::new() tests
    // =========================================================================

    #[test]
    fn test_accessible_node_new() {
        let node = AccessibleNode::new(Role::Button);
        assert_eq!(node.role, Role::Button);
        assert!(node.label.is_none());
        assert!(node.description.is_none());
        assert!(node.shortcut.is_none());
        assert!(!node.state.disabled);
        assert!(!node.state.selected);
        assert!(node.properties.is_empty());
        assert!(node.children.is_empty());
        assert!(node.parent.is_none());
    }

    #[test]
    fn test_accessible_node_new_generates_id() {
        let node = AccessibleNode::new(Role::Button);
        assert!(!node.id.is_empty());
    }

    // =========================================================================
    // AccessibleNode::with_id() tests
    // =========================================================================

    #[test]
    fn test_accessible_node_with_id_str() {
        let node = AccessibleNode::with_id("test-id", Role::Button);
        assert_eq!(node.id, "test-id");
        assert_eq!(node.role, Role::Button);
    }

    #[test]
    fn test_accessible_node_with_id_string() {
        let node = AccessibleNode::with_id(String::from("my-id"), Role::Link);
        assert_eq!(node.id, "my-id");
    }

    // =========================================================================
    // AccessibleNode::label() tests
    // =========================================================================

    #[test]
    fn test_accessible_node_label_str() {
        let node = AccessibleNode::new(Role::Button).label("Submit");
        assert_eq!(node.label, Some("Submit".to_string()));
    }

    #[test]
    fn test_accessible_node_label_string() {
        let node = AccessibleNode::new(Role::Button).label(String::from("Cancel"));
        assert_eq!(node.label, Some("Cancel".to_string()));
    }

    // =========================================================================
    // AccessibleNode::description() tests
    // =========================================================================

    #[test]
    fn test_accessible_node_description_str() {
        let node = AccessibleNode::new(Role::Button).description("Submit the form");
        assert_eq!(node.description, Some("Submit the form".to_string()));
    }

    #[test]
    fn test_accessible_node_description_string() {
        let node = AccessibleNode::new(Role::Button).description(String::from("Click to activate"));
        assert_eq!(node.description, Some("Click to activate".to_string()));
    }

    // =========================================================================
    // AccessibleNode::shortcut() tests
    // =========================================================================

    #[test]
    fn test_accessible_node_shortcut_str() {
        let node = AccessibleNode::new(Role::Button).shortcut("Ctrl+Enter");
        assert_eq!(node.shortcut, Some("Ctrl+Enter".to_string()));
    }

    #[test]
    fn test_accessible_node_shortcut_string() {
        let node = AccessibleNode::new(Role::Button).shortcut(String::from("Alt+S"));
        assert_eq!(node.shortcut, Some("Alt+S".to_string()));
    }

    // =========================================================================
    // AccessibleNode::state() tests
    // =========================================================================

    #[test]
    fn test_accessible_node_state() {
        let state = AccessibleState::new().disabled(true);
        let node = AccessibleNode::new(Role::Button).state(state.clone());
        assert!(node.state.disabled);
    }

    // =========================================================================
    // AccessibleNode::property() tests
    // =========================================================================

    #[test]
    fn test_accessible_node_property() {
        let node = AccessibleNode::new(Role::Button).property("data-test", "value");
        assert_eq!(node.properties.get("data-test"), Some(&"value".to_string()));
    }

    #[test]
    fn test_accessible_node_property_multiple() {
        let node = AccessibleNode::new(Role::Button)
            .property("data-test", "value1")
            .property("aria-describedby", "desc1");
        assert_eq!(node.properties.len(), 2);
    }

    // =========================================================================
    // AccessibleNode::child() tests
    // =========================================================================

    #[test]
    fn test_accessible_node_child() {
        let node = AccessibleNode::new(Role::Group).child("child-id");
        assert_eq!(node.children, vec!["child-id".to_string()]);
    }

    #[test]
    fn test_accessible_node_child_multiple() {
        let node = AccessibleNode::new(Role::Group)
            .child("child1")
            .child("child2");
        assert_eq!(node.children.len(), 2);
    }

    // =========================================================================
    // AccessibleNode::parent() tests
    // =========================================================================

    #[test]
    fn test_accessible_node_parent_str() {
        let node = AccessibleNode::new(Role::Button).parent("parent-id");
        assert_eq!(node.parent, Some("parent-id".to_string()));
    }

    #[test]
    fn test_accessible_node_parent_string() {
        let node = AccessibleNode::new(Role::Button).parent(String::from("parent-id"));
        assert_eq!(node.parent, Some("parent-id".to_string()));
    }

    // =========================================================================
    // AccessibleNode::accessible_name() tests
    // =========================================================================

    #[test]
    fn test_accessible_node_accessible_name_with_label() {
        let node = AccessibleNode::new(Role::Button).label("Submit");
        assert_eq!(node.accessible_name(), "Submit");
    }

    #[test]
    fn test_accessible_node_accessible_name_without_label() {
        let node = AccessibleNode::new(Role::Button);
        assert_eq!(node.accessible_name(), "button");
    }

    // =========================================================================
    // AccessibleNode::is_focusable() tests
    // =========================================================================

    #[test]
    fn test_accessible_node_is_focusable_interactive() {
        let node = AccessibleNode::new(Role::Button);
        assert!(node.is_focusable());
    }

    #[test]
    fn test_accessible_node_not_focusable_non_interactive() {
        let node = AccessibleNode::new(Role::Generic);
        assert!(!node.is_focusable());
    }

    #[test]
    fn test_accessible_node_not_focusable_disabled() {
        let node = AccessibleNode::new(Role::Button).state(AccessibleState::new().disabled(true));
        assert!(!node.is_focusable());
    }

    #[test]
    fn test_accessible_node_not_focusable_hidden() {
        let mut state = AccessibleState::new();
        state.hidden = true;
        let node = AccessibleNode::new(Role::Button).state(state);
        assert!(!node.is_focusable());
    }

    // =========================================================================
    // AccessibleNode::describe() tests
    // =========================================================================

    #[test]
    fn test_accessible_node_describe_role_only() {
        let node = AccessibleNode::new(Role::Button);
        assert_eq!(node.describe(), "button");
    }

    #[test]
    fn test_accessible_node_describe_with_label() {
        let node = AccessibleNode::new(Role::Button).label("Submit");
        assert_eq!(node.describe(), "Submit, button");
    }

    #[test]
    fn test_accessible_node_describe_disabled() {
        let node = AccessibleNode::new(Role::Button).state(AccessibleState::new().disabled(true));
        assert_eq!(node.describe(), "button, disabled");
    }

    #[test]
    fn test_accessible_node_describe_checked() {
        let node = AccessibleNode::new(Role::Checkbox).state(AccessibleState::new().checked(true));
        assert_eq!(node.describe(), "checkbox, checked");
    }

    #[test]
    fn test_accessible_node_describe_not_checked() {
        let node = AccessibleNode::new(Role::Checkbox).state(AccessibleState::new().checked(false));
        assert_eq!(node.describe(), "checkbox, not checked");
    }

    #[test]
    fn test_accessible_node_describe_expanded() {
        let node = AccessibleNode::new(Role::Tree).state(AccessibleState::new().expanded(true));
        assert_eq!(node.describe(), "tree, expanded");
    }

    #[test]
    fn test_accessible_node_describe_collapsed() {
        let node = AccessibleNode::new(Role::Tree).state(AccessibleState::new().expanded(false));
        assert_eq!(node.describe(), "tree, collapsed");
    }

    #[test]
    fn test_accessible_node_describe_selected() {
        let node = AccessibleNode::new(Role::ListItem).state(AccessibleState::new().selected(true));
        assert_eq!(node.describe(), "listitem, selected");
    }

    #[test]
    fn test_accessible_node_describe_position() {
        let node = AccessibleNode::new(Role::ListItem).state(AccessibleState::new().position(2, 5));
        assert_eq!(node.describe(), "listitem, 2 of 5");
    }

    #[test]
    fn test_accessible_node_describe_value_text() {
        let node = AccessibleNode::new(Role::Slider).state(AccessibleState {
            value_text: Some("50%".to_string()),
            ..Default::default()
        });
        assert_eq!(node.describe(), "slider, 50%");
    }

    #[test]
    fn test_accessible_node_describe_value_percent() {
        let node = AccessibleNode::new(Role::Slider)
            .state(AccessibleState::new().value_range(50.0, 0.0, 100.0));
        assert_eq!(node.describe(), "slider, 50%");
    }

    #[test]
    fn test_accessible_node_describe_with_description() {
        let node = AccessibleNode::new(Role::Button).description("Click to submit");
        assert_eq!(node.describe(), "button, Click to submit");
    }

    #[test]
    fn test_accessible_node_describe_with_shortcut() {
        let node = AccessibleNode::new(Role::Button).shortcut("Ctrl+Enter");
        assert_eq!(node.describe(), "button, Press Ctrl+Enter");
    }

    #[test]
    fn test_accessible_node_describe_comprehensive() {
        let node = AccessibleNode::new(Role::Button)
            .label("Submit")
            .description("Submit the form")
            .shortcut("Enter")
            .state(AccessibleState::new().disabled(true));
        assert_eq!(
            node.describe(),
            "Submit, button, disabled, Submit the form, Press Enter"
        );
    }

    // =========================================================================
    // AccessibleNode builder chain tests
    // =========================================================================

    #[test]
    fn test_accessible_node_builder_chain() {
        let node = AccessibleNode::new(Role::Button)
            .label("Test")
            .description("Description")
            .shortcut("Ctrl+S")
            .state(AccessibleState::new().disabled(false))
            .property("data-test", "value")
            .child("child1")
            .parent("parent1");

        assert_eq!(node.label, Some("Test".to_string()));
        assert_eq!(node.description, Some("Description".to_string()));
        assert_eq!(node.shortcut, Some("Ctrl+S".to_string()));
        assert!(!node.state.disabled);
        assert_eq!(node.properties.get("data-test"), Some(&"value".to_string()));
        assert_eq!(node.children, vec!["child1".to_string()]);
        assert_eq!(node.parent, Some("parent1".to_string()));
    }

    // =========================================================================
    // AccessibleNode clone tests
    // =========================================================================

    #[test]
    fn test_accessible_node_clone() {
        let node = AccessibleNode::new(Role::Button).label("Test");
        let cloned = node.clone();
        assert_eq!(node.id, cloned.id);
        assert_eq!(node.role, cloned.role);
        assert_eq!(node.label, cloned.label);
    }
}
