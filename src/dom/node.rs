//! DOM node representation

use std::collections::HashSet;
use super::NodeId;
use crate::style::Style;

/// Widget metadata for CSS matching
#[derive(Debug, Clone, Default)]
pub struct WidgetMeta {
    /// Widget type name (e.g., "Button", "Input", "Text")
    pub widget_type: String,
    /// Element ID (unique identifier, e.g., "submit-btn")
    pub id: Option<String>,
    /// CSS classes (e.g., ["primary", "large"])
    pub classes: HashSet<String>,
}

impl WidgetMeta {
    /// Create new widget metadata
    pub fn new(widget_type: impl Into<String>) -> Self {
        Self {
            widget_type: widget_type.into(),
            id: None,
            classes: HashSet::new(),
        }
    }

    /// Set element ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add a CSS class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.insert(class.into());
        self
    }

    /// Add multiple CSS classes
    pub fn classes<I, S>(mut self, classes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for class in classes {
            self.classes.insert(class.into());
        }
        self
    }

    /// Check if has a class
    pub fn has_class(&self, class: &str) -> bool {
        self.classes.contains(class)
    }
}

/// Node state for pseudo-class matching
#[derive(Debug, Clone, Default)]
pub struct NodeState {
    /// Node has keyboard focus
    pub focused: bool,
    /// Mouse is hovering (if mouse support enabled)
    pub hovered: bool,
    /// Node is disabled
    pub disabled: bool,
    /// Node is selected
    pub selected: bool,
    /// Node is checked (for checkboxes, radio buttons)
    pub checked: bool,
    /// Node is active (being pressed)
    pub active: bool,
    /// Node is empty (no content)
    pub empty: bool,
    /// Node's content, style or layout is dirty and needs repaint
    pub dirty: bool,
    /// Node is first child of parent
    pub first_child: bool,
    /// Node is last child of parent
    pub last_child: bool,
    /// Node is only child of parent
    pub only_child: bool,
    /// Node index among siblings (0-based)
    pub child_index: usize,
    /// Total sibling count
    pub sibling_count: usize,
}

impl NodeState {
    /// Create a new default state
    pub fn new() -> Self {
        Self::default()
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set hovered state
    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set checked state
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Mark the node as dirty
    pub fn dirty(mut self, dirty: bool) -> Self {
        self.dirty = dirty;
        self
    }

    /// Update positional states based on sibling info
    pub fn update_position(&mut self, index: usize, total: usize) {
        self.child_index = index;
        self.sibling_count = total;
        self.first_child = index == 0;
        self.last_child = index == total.saturating_sub(1);
        self.only_child = total == 1;
    }
}

/// DOM node identifier (for parent/child references)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DomId(pub NodeId);

impl DomId {
    /// Create a new DOM ID
    pub fn new(id: NodeId) -> Self {
        Self(id)
    }

    /// Get the inner ID value
    pub fn inner(&self) -> NodeId {
        self.0
    }
}

/// A node in the DOM tree
#[derive(Debug, Clone)]
pub struct DomNode {
    /// Unique node identifier
    pub id: DomId,
    /// Widget metadata
    pub meta: WidgetMeta,
    /// Current state
    pub state: NodeState,
    /// Parent node ID (None for root)
    pub parent: Option<DomId>,
    /// Child node IDs
    pub children: Vec<DomId>,
    /// Computed style (after cascade)
    pub computed_style: Style,
    /// Inline style (highest priority)
    pub inline_style: Option<Style>,
}

impl DomNode {
    /// Create a new DOM node
    pub fn new(id: DomId, meta: WidgetMeta) -> Self {
        Self {
            id,
            meta,
            state: NodeState::default(),
            parent: None,
            children: Vec::new(),
            computed_style: Style::default(),
            inline_style: None,
        }
    }

    /// Get widget type
    pub fn widget_type(&self) -> &str {
        &self.meta.widget_type
    }

    /// Get element ID
    pub fn element_id(&self) -> Option<&str> {
        self.meta.id.as_deref()
    }

    /// Check if node has a class
    pub fn has_class(&self, class: &str) -> bool {
        self.meta.has_class(class)
    }

    /// Get all classes
    pub fn classes(&self) -> impl Iterator<Item = &str> {
        self.meta.classes.iter().map(|s| s.as_str())
    }

    /// Check if this node matches a pseudo-class
    pub fn matches_pseudo(&self, pseudo: &super::PseudoClass) -> bool {
        use super::PseudoClass::*;
        match pseudo {
            Focus => self.state.focused,
            Hover => self.state.hovered,
            Active => self.state.active,
            Disabled => self.state.disabled,
            Enabled => !self.state.disabled,
            Checked => self.state.checked,
            Selected => self.state.selected,
            Empty => self.state.empty,
            FirstChild => self.state.first_child,
            LastChild => self.state.last_child,
            OnlyChild => self.state.only_child,
            NthChild(n) => self.state.child_index + 1 == *n,
            NthLastChild(n) => {
                let from_end = self.state.sibling_count - self.state.child_index;
                from_end == *n
            }
            Not(inner) => !self.matches_pseudo(inner),
        }
    }

    /// Set inline style
    pub fn set_inline_style(&mut self, style: Style) {
        self.inline_style = Some(style);
    }

    /// Add a child
    pub fn add_child(&mut self, child_id: DomId) {
        self.children.push(child_id);
    }

    /// Remove a child
    pub fn remove_child(&mut self, child_id: DomId) {
        self.children.retain(|&id| id != child_id);
    }

    /// Check if has children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Get child count
    pub fn child_count(&self) -> usize {
        self.children.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_meta() {
        let meta = WidgetMeta::new("Button")
            .id("submit")
            .class("primary")
            .class("large");

        assert_eq!(meta.widget_type, "Button");
        assert_eq!(meta.id, Some("submit".to_string()));
        assert!(meta.has_class("primary"));
        assert!(meta.has_class("large"));
        assert!(!meta.has_class("small"));
    }

    #[test]
    fn test_node_state() {
        let mut state = NodeState::new().focused(true).disabled(false);
        state.update_position(0, 3);

        assert!(state.focused);
        assert!(!state.disabled);
        assert!(state.first_child);
        assert!(!state.last_child);
        assert!(!state.only_child);
    }

    #[test]
    fn test_dom_node() {
        let meta = WidgetMeta::new("Button").class("primary");
        let node = DomNode::new(DomId::new(1), meta);

        assert_eq!(node.widget_type(), "Button");
        assert!(node.has_class("primary"));
    }
}
