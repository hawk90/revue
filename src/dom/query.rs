//! DOM query support
//!
//! Enables jQuery/CSS-like queries on the DOM tree:
//!
//! ```ignore
//! // Single element
//! let btn = dom.query_one("#submit");
//! let input = dom.query_one("Input.email");
//!
//! // Multiple elements
//! let buttons = dom.query_all("Button");
//! let cards = dom.query_all(".card");
//! ```

use super::selector::{parse_selector, Combinator, Selector, SelectorPart};
use super::{DomId, DomNode};
use std::collections::HashMap;

/// Query result - found nodes
pub struct QueryResult<'a> {
    nodes: Vec<&'a DomNode>,
}

impl<'a> QueryResult<'a> {
    /// Create empty result
    pub fn empty() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Create from nodes
    pub fn from_nodes(nodes: Vec<&'a DomNode>) -> Self {
        Self { nodes }
    }

    /// Get first result
    pub fn first(&self) -> Option<&'a DomNode> {
        self.nodes.first().copied()
    }

    /// Get all results
    pub fn all(&self) -> &[&'a DomNode] {
        &self.nodes
    }

    /// Get result count
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Iterate over results
    pub fn iter(&self) -> impl Iterator<Item = &'a DomNode> + '_ {
        self.nodes.iter().copied()
    }
}

impl<'a> IntoIterator for QueryResult<'a> {
    type Item = &'a DomNode;
    type IntoIter = std::vec::IntoIter<&'a DomNode>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

/// DOM query interface
pub trait Query {
    /// Query for a single element
    fn query_one(&self, selector: &str) -> Option<&DomNode>;

    /// Query for all matching elements
    fn query_all(&self, selector: &str) -> QueryResult<'_>;

    /// Get element by ID
    fn get_by_id(&self, id: &str) -> Option<&DomNode>;

    /// Get elements by class
    fn get_by_class(&self, class: &str) -> QueryResult<'_>;

    /// Get elements by type
    fn get_by_type(&self, widget_type: &str) -> QueryResult<'_>;
}

/// DOM Tree - manages the widget hierarchy
#[derive(Debug, Default)]
pub struct DomTree {
    /// All nodes by ID
    nodes: HashMap<DomId, DomNode>,
    /// Root node ID
    root: Option<DomId>,
    /// ID to DomId mapping
    id_map: HashMap<String, DomId>,
}

impl DomTree {
    /// Create a new empty DOM tree
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a root node
    pub fn create_root(&mut self, meta: super::node::WidgetMeta) -> DomId {
        let id = DomId::new(super::generate_node_id());
        let mut node = DomNode::new(id, meta);

        // Mark new nodes as dirty so they get rendered
        node.state.dirty = true;

        if let Some(ref element_id) = node.meta.id {
            self.id_map.insert(element_id.clone(), id);
        }

        self.nodes.insert(id, node);
        self.root = Some(id);
        id
    }

    /// Add a child node
    pub fn add_child(&mut self, parent_id: DomId, meta: super::node::WidgetMeta) -> DomId {
        let id = DomId::new(super::generate_node_id());
        let mut node = DomNode::new(id, meta);
        node.parent = Some(parent_id);

        // Mark new nodes as dirty so they get rendered
        node.state.dirty = true;

        if let Some(ref element_id) = node.meta.id {
            self.id_map.insert(element_id.clone(), id);
        }

        // Add to parent's children and collect IDs for position update
        let children_to_update: Vec<(DomId, usize, usize)> =
            if let Some(parent) = self.nodes.get_mut(&parent_id) {
                parent.children.push(id);
                let child_count = parent.children.len();
                parent
                    .children
                    .iter()
                    .enumerate()
                    .map(|(idx, &child_id)| (child_id, idx, child_count))
                    .collect()
            } else {
                Vec::new()
            };

        self.nodes.insert(id, node);

        // Update sibling positions
        for (child_id, idx, child_count) in children_to_update {
            if let Some(child) = self.nodes.get_mut(&child_id) {
                child.state.update_position(idx, child_count);
            }
        }

        id
    }

    /// Remove a node and its children
    pub fn remove(&mut self, id: DomId) {
        // Collect info we need before modifying
        let (parent_id, element_id, children) = if let Some(node) = self.nodes.get(&id) {
            (node.parent, node.meta.id.clone(), node.children.clone())
        } else {
            return;
        };

        // Remove from parent
        if let Some(parent_id) = parent_id {
            if let Some(parent) = self.nodes.get_mut(&parent_id) {
                parent.remove_child(id);
            }
        }

        // Remove from ID map
        if let Some(element_id) = element_id {
            self.id_map.remove(&element_id);
        }

        // Remove children recursively
        for child_id in children {
            self.remove(child_id);
        }

        self.nodes.remove(&id);
    }

    /// Get a node by DomId
    pub fn get(&self, id: DomId) -> Option<&DomNode> {
        self.nodes.get(&id)
    }

    /// Get a mutable node by DomId
    pub fn get_mut(&mut self, id: DomId) -> Option<&mut DomNode> {
        self.nodes.get_mut(&id)
    }

    /// Get root node
    pub fn root(&self) -> Option<&DomNode> {
        self.root.and_then(|id| self.nodes.get(&id))
    }

    /// Get root node ID
    pub fn root_id(&self) -> Option<DomId> {
        self.root
    }

    /// Get all nodes
    pub fn nodes(&self) -> impl Iterator<Item = &DomNode> {
        self.nodes.values()
    }

    /// Get node count
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Collect all dirty node IDs
    pub fn get_dirty_nodes(&self) -> Vec<DomId> {
        self.nodes
            .values()
            .filter(|node| node.state.dirty)
            .map(|node| node.id)
            .collect()
    }

    /// Clear dirty flags for all nodes after rendering
    pub fn clear_dirty_flags(&mut self) {
        for node in self.nodes.values_mut() {
            node.state.dirty = false;
        }
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Update node state
    pub fn set_state(&mut self, id: DomId, state: super::node::NodeState) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.state = state;
        }
    }

    /// Set focused node
    pub fn set_focused(&mut self, id: Option<DomId>) {
        // Clear previous focus
        for node in self.nodes.values_mut() {
            node.state.focused = false;
        }

        // Set new focus
        if let Some(focus_id) = id {
            if let Some(node) = self.nodes.get_mut(&focus_id) {
                node.state.focused = true;
            }
        }
    }

    /// Set hovered node
    pub fn set_hovered(&mut self, id: Option<DomId>) {
        // Clear previous hover
        for node in self.nodes.values_mut() {
            node.state.hovered = false;
        }

        // Set new hover
        if let Some(hover_id) = id {
            if let Some(node) = self.nodes.get_mut(&hover_id) {
                node.state.hovered = true;
            }
        }
    }

    /// Internal matcher for selectors with full combinator support
    ///
    /// Matches selectors from right to left, following CSS combinator rules:
    /// - Descendant (` `): Matches any ancestor
    /// - Child (`>`): Matches direct parent only
    /// - AdjacentSibling (`+`): Matches immediately preceding sibling
    /// - GeneralSibling (`~`): Matches any preceding sibling
    fn matches_selector(&self, node: &DomNode, selector: &Selector) -> bool {
        if selector.parts.is_empty() {
            return false;
        }

        // Match from right to left (target first, then ancestors/siblings)
        self.matches_selector_from(node, selector, selector.parts.len() - 1)
    }

    /// Recursively match selector parts from right to left
    fn matches_selector_from(&self, node: &DomNode, selector: &Selector, part_idx: usize) -> bool {
        let (part, _) = &selector.parts[part_idx];

        // Check if current part matches node
        if !self.matches_part(part, node) {
            return false;
        }

        // If this is the first part (leftmost), we're done
        if part_idx == 0 {
            return true;
        }

        // Get the combinator from the previous part
        let prev_combinator = selector.parts[part_idx - 1].1;

        match prev_combinator {
            Some(Combinator::Descendant) => {
                // Any ancestor must match
                let mut current = node.parent;
                while let Some(parent_id) = current {
                    if let Some(parent) = self.nodes.get(&parent_id) {
                        if self.matches_selector_from(parent, selector, part_idx - 1) {
                            return true;
                        }
                        current = parent.parent;
                    } else {
                        break;
                    }
                }
                false
            }
            Some(Combinator::Child) => {
                // Direct parent must match
                if let Some(parent_id) = node.parent {
                    if let Some(parent) = self.nodes.get(&parent_id) {
                        return self.matches_selector_from(parent, selector, part_idx - 1);
                    }
                }
                false
            }
            Some(Combinator::AdjacentSibling) => {
                // Immediately preceding sibling must match
                if let Some(prev_sibling) = self.get_previous_sibling(node) {
                    return self.matches_selector_from(prev_sibling, selector, part_idx - 1);
                }
                false
            }
            Some(Combinator::GeneralSibling) => {
                // Any preceding sibling must match
                let mut current = self.get_previous_sibling(node);
                while let Some(sibling) = current {
                    if self.matches_selector_from(sibling, selector, part_idx - 1) {
                        return true;
                    }
                    current = self.get_previous_sibling(sibling);
                }
                false
            }
            None => {
                // No combinator means simple selector - already matched above
                true
            }
        }
    }

    /// Check if a selector part matches a node
    fn matches_part(&self, part: &SelectorPart, node: &DomNode) -> bool {
        // Universal selector matches everything
        if part.universal
            && part.id.is_none()
            && part.classes.is_empty()
            && part.pseudo_classes.is_empty()
            && part.element.is_none()
        {
            return true;
        }

        // Check element type
        if let Some(ref elem) = part.element {
            if node.widget_type() != elem {
                return false;
            }
        }

        // Check ID
        if let Some(ref id) = part.id {
            if node.element_id() != Some(id.as_str()) {
                return false;
            }
        }

        // Check classes
        for class in &part.classes {
            if !node.has_class(class) {
                return false;
            }
        }

        // Check pseudo-classes
        for pseudo in &part.pseudo_classes {
            if !node.matches_pseudo(pseudo) {
                return false;
            }
        }

        true
    }

    /// Get previous sibling of a node
    fn get_previous_sibling(&self, node: &DomNode) -> Option<&DomNode> {
        let parent_id = node.parent?;
        let parent = self.nodes.get(&parent_id)?;

        let idx = parent.children.iter().position(|&id| id == node.id)?;
        if idx > 0 {
            self.nodes.get(&parent.children[idx - 1])
        } else {
            None
        }
    }
}

impl Query for DomTree {
    fn query_one(&self, selector_str: &str) -> Option<&DomNode> {
        let selector = parse_selector(selector_str).ok()?;
        self.nodes
            .values()
            .find(|node| self.matches_selector(node, &selector))
    }

    fn query_all(&self, selector_str: &str) -> QueryResult<'_> {
        let selector = match parse_selector(selector_str) {
            Ok(s) => s,
            Err(_) => return QueryResult::empty(),
        };

        let nodes: Vec<_> = self
            .nodes
            .values()
            .filter(|node| self.matches_selector(node, &selector))
            .collect();

        QueryResult::from_nodes(nodes)
    }

    fn get_by_id(&self, id: &str) -> Option<&DomNode> {
        self.id_map
            .get(id)
            .and_then(|dom_id| self.nodes.get(dom_id))
    }

    fn get_by_class(&self, class: &str) -> QueryResult<'_> {
        let nodes: Vec<_> = self
            .nodes
            .values()
            .filter(|node| node.has_class(class))
            .collect();
        QueryResult::from_nodes(nodes)
    }

    fn get_by_type(&self, widget_type: &str) -> QueryResult<'_> {
        let nodes: Vec<_> = self
            .nodes
            .values()
            .filter(|node| node.widget_type() == widget_type)
            .collect();
        QueryResult::from_nodes(nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::super::node::WidgetMeta;
    use super::*;

    fn create_test_tree() -> DomTree {
        let mut tree = DomTree::new();

        // Create root
        let root = tree.create_root(WidgetMeta::new("App").id("app"));

        // Add sidebar
        let sidebar = tree.add_child(root, WidgetMeta::new("Container").class("sidebar"));

        // Add buttons to sidebar
        tree.add_child(
            sidebar,
            WidgetMeta::new("Button").class("primary").id("nav-home"),
        );
        tree.add_child(sidebar, WidgetMeta::new("Button").id("nav-settings"));

        // Add content area
        let content = tree.add_child(root, WidgetMeta::new("Container").class("content"));

        // Add cards
        tree.add_child(content, WidgetMeta::new("Container").class("card"));
        tree.add_child(
            content,
            WidgetMeta::new("Container").class("card").class("featured"),
        );

        tree
    }

    #[test]
    fn test_query_by_id() {
        let tree = create_test_tree();

        let node = tree.get_by_id("app");
        assert!(node.is_some());
        assert_eq!(node.unwrap().widget_type(), "App");

        let node = tree.get_by_id("nav-home");
        assert!(node.is_some());
        assert_eq!(node.unwrap().widget_type(), "Button");
    }

    #[test]
    fn test_query_by_type() {
        let tree = create_test_tree();

        let buttons = tree.get_by_type("Button");
        assert_eq!(buttons.len(), 2);
    }

    #[test]
    fn test_query_by_class() {
        let tree = create_test_tree();

        let cards = tree.get_by_class("card");
        assert_eq!(cards.len(), 2);

        let featured = tree.get_by_class("featured");
        assert_eq!(featured.len(), 1);
    }

    #[test]
    fn test_query_one() {
        let tree = create_test_tree();

        let node = tree.query_one("Button");
        assert!(node.is_some());

        let node = tree.query_one("#nav-home");
        assert!(node.is_some());

        let node = tree.query_one(".primary");
        assert!(node.is_some());

        let node = tree.query_one("#nonexistent");
        assert!(node.is_none());
    }

    #[test]
    fn test_query_all() {
        let tree = create_test_tree();

        let results = tree.query_all("Button");
        assert_eq!(results.len(), 2);

        let results = tree.query_all(".card");
        assert_eq!(results.len(), 2);

        let results = tree.query_all("Container");
        assert_eq!(results.len(), 4); // sidebar, content, 2 cards
    }

    #[test]
    fn test_query_combined() {
        let tree = create_test_tree();

        let results = tree.query_all("Button.primary");
        assert_eq!(results.len(), 1);

        let results = tree.query_all("Container.card.featured");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_sibling_positions() {
        let tree = create_test_tree();

        // Get sidebar buttons
        let home = tree.get_by_id("nav-home").unwrap();
        let settings = tree.get_by_id("nav-settings").unwrap();

        assert!(home.state.first_child);
        assert!(!home.state.last_child);
        assert!(!settings.state.first_child);
        assert!(settings.state.last_child);
    }
}
