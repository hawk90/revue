//! DOM node representation

use super::NodeId;
use crate::style::Style;
use std::collections::HashSet;

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

    // =========================================================================
    // WidgetMeta tests
    // =========================================================================

    #[test]
    fn test_widget_meta_default() {
        let meta = WidgetMeta::default();
        assert!(meta.widget_type.is_empty());
        assert!(meta.id.is_none());
        assert!(meta.classes.is_empty());
    }

    #[test]
    fn test_widget_meta_new() {
        let meta = WidgetMeta::new("Input");
        assert_eq!(meta.widget_type, "Input");
        assert!(meta.id.is_none());
        assert!(meta.classes.is_empty());
    }

    #[test]
    fn test_widget_meta_id() {
        let meta = WidgetMeta::new("Button").id("submit-btn");
        assert_eq!(meta.id, Some("submit-btn".to_string()));
    }

    #[test]
    fn test_widget_meta_class() {
        let meta = WidgetMeta::new("Button").class("primary");
        assert!(meta.has_class("primary"));
        assert!(!meta.has_class("secondary"));
    }

    #[test]
    fn test_widget_meta_classes_iterator() {
        let meta = WidgetMeta::new("Button").classes(vec!["primary", "large", "rounded"]);

        assert!(meta.has_class("primary"));
        assert!(meta.has_class("large"));
        assert!(meta.has_class("rounded"));
        assert_eq!(meta.classes.len(), 3);
    }

    #[test]
    fn test_widget_meta_duplicate_classes() {
        let meta = WidgetMeta::new("Button").class("primary").class("primary"); // Duplicate

        // HashSet deduplicates
        assert_eq!(meta.classes.len(), 1);
    }

    #[test]
    fn test_widget_meta_clone() {
        let meta = WidgetMeta::new("Button").id("btn").class("primary");
        let cloned = meta.clone();

        assert_eq!(cloned.widget_type, "Button");
        assert_eq!(cloned.id, Some("btn".to_string()));
        assert!(cloned.has_class("primary"));
    }

    // =========================================================================
    // NodeState tests
    // =========================================================================

    #[test]
    fn test_node_state_default() {
        let state = NodeState::default();
        assert!(!state.focused);
        assert!(!state.hovered);
        assert!(!state.disabled);
        assert!(!state.selected);
        assert!(!state.checked);
        assert!(!state.active);
        assert!(!state.empty);
        assert!(!state.dirty);
        assert!(!state.first_child);
        assert!(!state.last_child);
        assert!(!state.only_child);
        assert_eq!(state.child_index, 0);
        assert_eq!(state.sibling_count, 0);
    }

    #[test]
    fn test_node_state_focused() {
        let state = NodeState::new().focused(true);
        assert!(state.focused);

        let state = state.focused(false);
        assert!(!state.focused);
    }

    #[test]
    fn test_node_state_hovered() {
        let state = NodeState::new().hovered(true);
        assert!(state.hovered);
    }

    #[test]
    fn test_node_state_disabled() {
        let state = NodeState::new().disabled(true);
        assert!(state.disabled);
    }

    #[test]
    fn test_node_state_selected() {
        let state = NodeState::new().selected(true);
        assert!(state.selected);
    }

    #[test]
    fn test_node_state_checked() {
        let state = NodeState::new().checked(true);
        assert!(state.checked);
    }

    #[test]
    fn test_node_state_dirty() {
        let state = NodeState::new().dirty(true);
        assert!(state.dirty);
    }

    #[test]
    fn test_node_state_update_position_first() {
        let mut state = NodeState::new();
        state.update_position(0, 5);

        assert_eq!(state.child_index, 0);
        assert_eq!(state.sibling_count, 5);
        assert!(state.first_child);
        assert!(!state.last_child);
        assert!(!state.only_child);
    }

    #[test]
    fn test_node_state_update_position_last() {
        let mut state = NodeState::new();
        state.update_position(4, 5);

        assert_eq!(state.child_index, 4);
        assert!(state.last_child);
        assert!(!state.first_child);
        assert!(!state.only_child);
    }

    #[test]
    fn test_node_state_update_position_only_child() {
        let mut state = NodeState::new();
        state.update_position(0, 1);

        assert!(state.first_child);
        assert!(state.last_child);
        assert!(state.only_child);
    }

    #[test]
    fn test_node_state_update_position_middle() {
        let mut state = NodeState::new();
        state.update_position(2, 5);

        assert!(!state.first_child);
        assert!(!state.last_child);
        assert!(!state.only_child);
    }

    #[test]
    fn test_node_state_clone() {
        let state = NodeState::new().focused(true).disabled(true);
        let cloned = state.clone();

        assert!(cloned.focused);
        assert!(cloned.disabled);
    }

    // =========================================================================
    // DomId tests
    // =========================================================================

    #[test]
    fn test_dom_id_new() {
        let id = DomId::new(42);
        assert_eq!(id.inner(), 42);
    }

    #[test]
    fn test_dom_id_inner() {
        let id = DomId(100);
        assert_eq!(id.inner(), 100);
    }

    #[test]
    fn test_dom_id_equality() {
        let id1 = DomId::new(1);
        let id2 = DomId::new(1);
        let id3 = DomId::new(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_dom_id_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(DomId::new(1));
        set.insert(DomId::new(2));
        set.insert(DomId::new(1)); // Duplicate

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_dom_id_copy() {
        let id1 = DomId::new(42);
        let id2 = id1; // Copy
        assert_eq!(id1, id2);
    }

    // =========================================================================
    // DomNode tests
    // =========================================================================

    #[test]
    fn test_dom_node_new() {
        let meta = WidgetMeta::new("Text");
        let node = DomNode::new(DomId::new(1), meta);

        assert_eq!(node.id.inner(), 1);
        assert_eq!(node.widget_type(), "Text");
        assert!(node.parent.is_none());
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_dom_node_element_id() {
        let meta = WidgetMeta::new("Button").id("submit");
        let node = DomNode::new(DomId::new(1), meta);

        assert_eq!(node.element_id(), Some("submit"));
    }

    #[test]
    fn test_dom_node_element_id_none() {
        let meta = WidgetMeta::new("Button");
        let node = DomNode::new(DomId::new(1), meta);

        assert_eq!(node.element_id(), None);
    }

    #[test]
    fn test_dom_node_has_class() {
        let meta = WidgetMeta::new("Button").class("primary").class("large");
        let node = DomNode::new(DomId::new(1), meta);

        assert!(node.has_class("primary"));
        assert!(node.has_class("large"));
        assert!(!node.has_class("small"));
    }

    #[test]
    fn test_dom_node_classes_iterator() {
        let meta = WidgetMeta::new("Button").class("primary").class("large");
        let node = DomNode::new(DomId::new(1), meta);

        let classes: Vec<&str> = node.classes().collect();
        assert_eq!(classes.len(), 2);
        assert!(classes.contains(&"primary"));
        assert!(classes.contains(&"large"));
    }

    #[test]
    fn test_dom_node_set_inline_style() {
        let meta = WidgetMeta::new("Button");
        let mut node = DomNode::new(DomId::new(1), meta);

        assert!(node.inline_style.is_none());
        node.set_inline_style(Style::default());
        assert!(node.inline_style.is_some());
    }

    #[test]
    fn test_dom_node_add_child() {
        let meta = WidgetMeta::new("Container");
        let mut node = DomNode::new(DomId::new(1), meta);

        assert!(!node.has_children());
        assert_eq!(node.child_count(), 0);

        node.add_child(DomId::new(2));
        node.add_child(DomId::new(3));

        assert!(node.has_children());
        assert_eq!(node.child_count(), 2);
        assert_eq!(node.children, vec![DomId::new(2), DomId::new(3)]);
    }

    #[test]
    fn test_dom_node_remove_child() {
        let meta = WidgetMeta::new("Container");
        let mut node = DomNode::new(DomId::new(1), meta);

        node.add_child(DomId::new(2));
        node.add_child(DomId::new(3));
        node.add_child(DomId::new(4));

        node.remove_child(DomId::new(3));

        assert_eq!(node.child_count(), 2);
        assert_eq!(node.children, vec![DomId::new(2), DomId::new(4)]);
    }

    #[test]
    fn test_dom_node_remove_nonexistent_child() {
        let meta = WidgetMeta::new("Container");
        let mut node = DomNode::new(DomId::new(1), meta);

        node.add_child(DomId::new(2));
        node.remove_child(DomId::new(99)); // Non-existent

        assert_eq!(node.child_count(), 1);
    }

    #[test]
    fn test_dom_node_matches_pseudo_focus() {
        use crate::dom::PseudoClass;

        let meta = WidgetMeta::new("Input");
        let mut node = DomNode::new(DomId::new(1), meta);
        node.state = NodeState::new().focused(true);

        assert!(node.matches_pseudo(&PseudoClass::Focus));
    }

    #[test]
    fn test_dom_node_matches_pseudo_hover() {
        use crate::dom::PseudoClass;

        let meta = WidgetMeta::new("Button");
        let mut node = DomNode::new(DomId::new(1), meta);
        node.state = NodeState::new().hovered(true);

        assert!(node.matches_pseudo(&PseudoClass::Hover));
    }

    #[test]
    fn test_dom_node_matches_pseudo_disabled() {
        use crate::dom::PseudoClass;

        let meta = WidgetMeta::new("Button");
        let mut node = DomNode::new(DomId::new(1), meta);
        node.state = NodeState::new().disabled(true);

        assert!(node.matches_pseudo(&PseudoClass::Disabled));
        assert!(!node.matches_pseudo(&PseudoClass::Enabled));
    }

    #[test]
    fn test_dom_node_matches_pseudo_enabled() {
        use crate::dom::PseudoClass;

        let meta = WidgetMeta::new("Button");
        let node = DomNode::new(DomId::new(1), meta);

        assert!(node.matches_pseudo(&PseudoClass::Enabled));
        assert!(!node.matches_pseudo(&PseudoClass::Disabled));
    }

    #[test]
    fn test_dom_node_matches_pseudo_checked() {
        use crate::dom::PseudoClass;

        let meta = WidgetMeta::new("Checkbox");
        let mut node = DomNode::new(DomId::new(1), meta);
        node.state = NodeState::new().checked(true);

        assert!(node.matches_pseudo(&PseudoClass::Checked));
    }

    #[test]
    fn test_dom_node_matches_pseudo_selected() {
        use crate::dom::PseudoClass;

        let meta = WidgetMeta::new("ListItem");
        let mut node = DomNode::new(DomId::new(1), meta);
        node.state = NodeState::new().selected(true);

        assert!(node.matches_pseudo(&PseudoClass::Selected));
    }

    #[test]
    fn test_dom_node_matches_pseudo_first_child() {
        use crate::dom::PseudoClass;

        let meta = WidgetMeta::new("ListItem");
        let mut node = DomNode::new(DomId::new(1), meta);
        node.state.update_position(0, 5);

        assert!(node.matches_pseudo(&PseudoClass::FirstChild));
        assert!(!node.matches_pseudo(&PseudoClass::LastChild));
    }

    #[test]
    fn test_dom_node_matches_pseudo_last_child() {
        use crate::dom::PseudoClass;

        let meta = WidgetMeta::new("ListItem");
        let mut node = DomNode::new(DomId::new(1), meta);
        node.state.update_position(4, 5);

        assert!(node.matches_pseudo(&PseudoClass::LastChild));
        assert!(!node.matches_pseudo(&PseudoClass::FirstChild));
    }

    #[test]
    fn test_dom_node_matches_pseudo_only_child() {
        use crate::dom::PseudoClass;

        let meta = WidgetMeta::new("ListItem");
        let mut node = DomNode::new(DomId::new(1), meta);
        node.state.update_position(0, 1);

        assert!(node.matches_pseudo(&PseudoClass::OnlyChild));
        assert!(node.matches_pseudo(&PseudoClass::FirstChild));
        assert!(node.matches_pseudo(&PseudoClass::LastChild));
    }

    #[test]
    fn test_dom_node_matches_pseudo_nth_child() {
        use crate::dom::PseudoClass;

        let meta = WidgetMeta::new("ListItem");
        let mut node = DomNode::new(DomId::new(1), meta);
        node.state.update_position(2, 5); // 3rd child (0-indexed: 2)

        assert!(node.matches_pseudo(&PseudoClass::NthChild(3))); // 1-indexed
        assert!(!node.matches_pseudo(&PseudoClass::NthChild(2)));
    }

    #[test]
    fn test_dom_node_matches_pseudo_nth_last_child() {
        use crate::dom::PseudoClass;

        let meta = WidgetMeta::new("ListItem");
        let mut node = DomNode::new(DomId::new(1), meta);
        node.state.update_position(3, 5); // 4th child, 2nd from last

        assert!(node.matches_pseudo(&PseudoClass::NthLastChild(2)));
    }

    #[test]
    fn test_dom_node_matches_pseudo_not() {
        use crate::dom::PseudoClass;

        let meta = WidgetMeta::new("Button");
        let node = DomNode::new(DomId::new(1), meta);

        // Not disabled (node is enabled by default)
        assert!(node.matches_pseudo(&PseudoClass::Not(Box::new(PseudoClass::Disabled))));
    }

    #[test]
    fn test_dom_node_matches_pseudo_empty() {
        use crate::dom::PseudoClass;

        let meta = WidgetMeta::new("Container");
        let mut node = DomNode::new(DomId::new(1), meta);
        node.state.empty = true;

        assert!(node.matches_pseudo(&PseudoClass::Empty));
    }

    #[test]
    fn test_dom_node_matches_pseudo_active() {
        use crate::dom::PseudoClass;

        let meta = WidgetMeta::new("Button");
        let mut node = DomNode::new(DomId::new(1), meta);
        node.state.active = true;

        assert!(node.matches_pseudo(&PseudoClass::Active));
    }

    #[test]
    fn test_dom_node_clone() {
        let meta = WidgetMeta::new("Button").id("btn").class("primary");
        let mut node = DomNode::new(DomId::new(1), meta);
        node.add_child(DomId::new(2));

        let cloned = node.clone();

        assert_eq!(cloned.id, node.id);
        assert_eq!(cloned.widget_type(), "Button");
        assert_eq!(cloned.element_id(), Some("btn"));
        assert!(cloned.has_class("primary"));
        assert_eq!(cloned.child_count(), 1);
    }
}
