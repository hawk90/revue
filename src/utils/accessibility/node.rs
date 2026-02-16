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
