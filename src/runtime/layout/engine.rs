//! Layout engine using custom TUI-optimized algorithm
//!
//! Provides a simplified flexbox/grid layout implementation optimized for
//! terminal user interfaces using integer cell coordinates.

use super::compute::compute_layout;
use super::node::{
    ComputedLayout, Edges, FlexProps, GridProps, Inset, LayoutNode, LayoutSpacing, SizeConstraints,
};
use super::tree::LayoutTree;
use super::Rect;
use crate::dom::DomId;
use crate::style::Style;
use std::collections::HashMap;

/// Errors that can occur during layout operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum LayoutError {
    /// Node was not found in the layout tree
    #[error("Layout node not found for DOM ID {0}")]
    NodeNotFound(u64),

    /// Failed to create a new node
    #[error("Failed to create layout node: {0}")]
    NodeCreationFailed(String),

    /// Failed to update node style
    #[error("Failed to update layout style: {0}")]
    StyleUpdateFailed(String),

    /// Failed to add child to parent
    #[error("Failed to add child node: {0}")]
    AddChildFailed(String),

    /// Failed to remove node
    #[error("Failed to remove layout node: {0}")]
    RemoveFailed(String),

    /// Failed to compute layout
    #[error("Failed to compute layout: {0}")]
    ComputeFailed(String),

    /// Failed to get layout result
    #[error("Failed to get layout result: {0}")]
    LayoutRetrievalFailed(String),
}

/// Result type for layout operations
pub type LayoutResult<T> = Result<T, LayoutError>;

/// Layout engine using custom TUI-optimized algorithm
pub struct LayoutEngine {
    tree: LayoutTree,
    /// Maps DomId to internal node ID (same value, but tracked for API compatibility)
    nodes: HashMap<DomId, u64>,
}

impl LayoutEngine {
    /// Create a new layout engine
    pub fn new() -> Self {
        Self {
            tree: LayoutTree::new(),
            nodes: HashMap::new(),
        }
    }

    /// Create a new node with the given style
    ///
    /// # Errors
    ///
    /// Returns `Err(LayoutError::NodeNotFound)` if the node cannot be created
    /// or if a node with the same ID already exists.
    pub fn create_node(&mut self, dom_id: DomId, style: &Style) -> LayoutResult<()> {
        let node_id = dom_id.inner();
        let node = style_to_layout_node(node_id, style);
        self.tree.insert(node);
        self.nodes.insert(dom_id, node_id);
        Ok(())
    }

    /// Create a node with children
    ///
    /// # Errors
    ///
    /// Returns `Err(LayoutError::NodeNotFound)` if any child node is not found
    /// in the layout tree.
    pub fn create_node_with_children(
        &mut self,
        dom_id: DomId,
        style: &Style,
        children: &[DomId],
    ) -> LayoutResult<()> {
        let node_id = dom_id.inner();
        let mut node = style_to_layout_node(node_id, style);

        // Set children
        let child_ids: Vec<u64> = children
            .iter()
            .filter_map(|id| self.nodes.get(id).copied())
            .collect();
        node.children = child_ids.clone();

        self.tree.insert(node);
        self.nodes.insert(dom_id, node_id);

        // Update parent references for children
        for child_id in child_ids {
            if let Some(child) = self.tree.get_mut(child_id) {
                child.parent = Some(node_id);
            }
        }

        Ok(())
    }

    /// Update a node's style
    ///
    /// # Errors
    ///
    /// Returns `Err(LayoutError::NodeNotFound)` if the node with the given ID
    /// does not exist in the layout tree.
    pub fn update_style(&mut self, dom_id: DomId, style: &Style) -> LayoutResult<()> {
        let node_id = dom_id.inner();
        if let Some(node) = self.tree.get_mut(node_id) {
            update_node_from_style(node, style);
            Ok(())
        } else {
            Err(LayoutError::NodeNotFound(dom_id.inner()))
        }
    }

    /// Add a child to a node
    ///
    /// # Errors
    ///
    /// Returns `Err(LayoutError::NodeNotFound)` if:
    /// - The parent node with the given ID does not exist
    /// - The child node with the given ID does not exist
    pub fn add_child(&mut self, parent_dom_id: DomId, child_dom_id: DomId) -> LayoutResult<()> {
        let parent_id = parent_dom_id.inner();
        let child_id = child_dom_id.inner();

        if !self.nodes.contains_key(&parent_dom_id) {
            return Err(LayoutError::NodeNotFound(parent_dom_id.inner()));
        }
        if !self.nodes.contains_key(&child_dom_id) {
            return Err(LayoutError::NodeNotFound(child_dom_id.inner()));
        }

        self.tree.add_child(parent_id, child_id);
        Ok(())
    }

    /// Remove a node from the layout tree
    ///
    /// Returns Ok even if the node doesn't exist (idempotent).
    pub fn remove_node(&mut self, dom_id: DomId) -> LayoutResult<()> {
        let node_id = dom_id.inner();
        self.tree.remove(node_id);
        self.nodes.remove(&dom_id);
        Ok(())
    }

    /// Compute layout for a root node and all its descendants
    ///
    /// # Errors
    ///
    /// Returns `Err(LayoutError::NodeNotFound)` if the root node with the given ID
    /// does not exist in the layout tree.
    pub fn compute(
        &mut self,
        root_dom_id: DomId,
        available_width: u16,
        available_height: u16,
    ) -> LayoutResult<()> {
        let node_id = root_dom_id.inner();
        if !self.tree.contains(node_id) {
            return Err(LayoutError::NodeNotFound(root_dom_id.inner()));
        }

        compute_layout(&mut self.tree, node_id, available_width, available_height);
        Ok(())
    }

    /// Get the computed layout for a node
    ///
    /// # Errors
    ///
    /// Returns `Err(LayoutError::NodeNotFound)` if the node with the given ID
    /// does not exist in the layout tree.
    pub fn layout(&self, dom_id: DomId) -> LayoutResult<Rect> {
        let node_id = dom_id.inner();
        self.tree
            .get(node_id)
            .map(|node| Rect {
                x: node.computed.x,
                y: node.computed.y,
                width: node.computed.width,
                height: node.computed.height,
            })
            .ok_or_else(|| LayoutError::NodeNotFound(dom_id.inner()))
    }

    /// Get the computed layout for a node, returning None if not found
    ///
    /// This is a convenience method that converts errors to None.
    /// Use `layout()` if you need detailed error information.
    pub fn try_layout(&self, dom_id: DomId) -> Option<Rect> {
        self.layout(dom_id).ok()
    }

    /// Clear all nodes
    pub fn clear(&mut self) {
        self.tree.clear();
        self.nodes.clear();
    }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert Style to LayoutNode
fn style_to_layout_node(id: u64, style: &Style) -> LayoutNode {
    LayoutNode {
        id,
        display: style.layout.display,
        position: style.layout.position,
        flex: FlexProps {
            direction: style.layout.flex_direction,
            justify_content: style.layout.justify_content,
            align_items: style.layout.align_items,
            flex_grow: style.layout.flex_grow,
            gap: style.layout.gap,
            column_gap: style.layout.column_gap,
            row_gap: style.layout.row_gap,
        },
        grid: GridProps {
            template_columns: style.layout.grid_template_columns.tracks.clone(),
            template_rows: style.layout.grid_template_rows.tracks.clone(),
            column: style.layout.grid_column,
            row: style.layout.grid_row,
        },
        spacing: LayoutSpacing {
            padding: Edges::from(style.spacing.padding),
            margin: Edges::from(style.spacing.margin),
            inset: Inset {
                top: style.spacing.top,
                right: style.spacing.right,
                bottom: style.spacing.bottom,
                left: style.spacing.left,
            },
        },
        sizing: SizeConstraints {
            width: style.sizing.width,
            height: style.sizing.height,
            min_width: style.sizing.min_width,
            max_width: style.sizing.max_width,
            min_height: style.sizing.min_height,
            max_height: style.sizing.max_height,
        },
        children: Vec::new(),
        parent: None,
        computed: ComputedLayout::default(),
        dirty: true,
    }
}

/// Update an existing node from a style
fn update_node_from_style(node: &mut LayoutNode, style: &Style) {
    node.display = style.layout.display;
    node.position = style.layout.position;
    node.flex = FlexProps {
        direction: style.layout.flex_direction,
        justify_content: style.layout.justify_content,
        align_items: style.layout.align_items,
        flex_grow: style.layout.flex_grow,
        gap: style.layout.gap,
        column_gap: style.layout.column_gap,
        row_gap: style.layout.row_gap,
    };
    node.grid = GridProps {
        template_columns: style.layout.grid_template_columns.tracks.clone(),
        template_rows: style.layout.grid_template_rows.tracks.clone(),
        column: style.layout.grid_column,
        row: style.layout.grid_row,
    };
    node.spacing = LayoutSpacing {
        padding: Edges::from(style.spacing.padding),
        margin: Edges::from(style.spacing.margin),
        inset: Inset {
            top: style.spacing.top,
            right: style.spacing.right,
            bottom: style.spacing.bottom,
            left: style.spacing.left,
        },
    };
    node.sizing = SizeConstraints {
        width: style.sizing.width,
        height: style.sizing.height,
        min_width: style.sizing.min_width,
        max_width: style.sizing.max_width,
        min_height: style.sizing.min_height,
        max_height: style.sizing.max_height,
    };
    node.dirty = true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{Display, FlexDirection, Size};

    #[test]
    fn test_layout_engine_new() {
        let engine = LayoutEngine::new();
        assert!(engine.nodes.is_empty());
    }

    #[test]
    fn test_create_node() {
        let mut engine = LayoutEngine::new();
        let style = Style::default();
        let dom_id = DomId::new(1);

        engine.create_node(dom_id, &style).unwrap();
        assert!(engine.nodes.contains_key(&dom_id));
    }

    #[test]
    fn test_create_multiple_nodes() {
        let mut engine = LayoutEngine::new();
        let style = Style::default();

        let id1 = DomId::new(1);
        let id2 = DomId::new(2);
        let id3 = DomId::new(3);

        engine.create_node(id1, &style).unwrap();
        engine.create_node(id2, &style).unwrap();
        engine.create_node(id3, &style).unwrap();

        assert!(engine.nodes.contains_key(&id1));
        assert!(engine.nodes.contains_key(&id2));
        assert!(engine.nodes.contains_key(&id3));
    }

    #[test]
    fn test_compute_layout_single_node() {
        let mut engine = LayoutEngine::new();
        let mut style = Style::default();
        style.sizing.width = Size::Fixed(100);
        style.sizing.height = Size::Fixed(50);

        let id = DomId::new(1);
        engine.create_node(id, &style).unwrap();
        engine.compute(id, 200, 200).unwrap();

        let layout = engine.layout(id).unwrap();
        assert_eq!(layout.width, 200); // Root takes available space
        assert_eq!(layout.height, 200);
    }

    #[test]
    fn test_compute_layout_with_children() {
        let mut engine = LayoutEngine::new();

        // Create children
        let mut child_style = Style::default();
        child_style.sizing.width = Size::Fixed(50);
        child_style.sizing.height = Size::Fixed(30);
        let child1 = DomId::new(1);
        let child2 = DomId::new(2);
        engine.create_node(child1, &child_style).unwrap();
        engine.create_node(child2, &child_style).unwrap();

        // Create parent
        let mut parent_style = Style::default();
        parent_style.layout.display = Display::Flex;
        parent_style.layout.flex_direction = FlexDirection::Row;
        parent_style.sizing.width = Size::Fixed(200);
        parent_style.sizing.height = Size::Fixed(100);
        let parent = DomId::new(3);
        engine
            .create_node_with_children(parent, &parent_style, &[child1, child2])
            .unwrap();

        engine.compute(parent, 300, 300).unwrap();

        let parent_layout = engine.layout(parent).unwrap();
        assert_eq!(parent_layout.width, 300);
        assert_eq!(parent_layout.height, 300);

        let child1_layout = engine.layout(child1).unwrap();
        let child2_layout = engine.layout(child2).unwrap();

        // Children should be laid out in a row
        assert_eq!(child1_layout.x, 0);
        assert_eq!(child2_layout.x, 50); // After first child
    }

    #[test]
    fn test_remove_node() {
        let mut engine = LayoutEngine::new();
        let style = Style::default();

        let id = DomId::new(1);
        engine.create_node(id, &style).unwrap();
        assert!(engine.nodes.contains_key(&id));

        engine.remove_node(id).unwrap();
        assert!(!engine.nodes.contains_key(&id));
    }

    #[test]
    fn test_clear() {
        let mut engine = LayoutEngine::new();
        let style = Style::default();

        engine.create_node(DomId::new(1), &style).unwrap();
        engine.create_node(DomId::new(2), &style).unwrap();
        engine.create_node(DomId::new(3), &style).unwrap();

        assert_eq!(engine.nodes.len(), 3);

        engine.clear();
        assert!(engine.nodes.is_empty());
    }

    #[test]
    fn test_layout_error_node_not_found() {
        let engine = LayoutEngine::new();
        let result = engine.layout(DomId::new(999));
        assert!(matches!(result, Err(LayoutError::NodeNotFound(_))));
    }

    #[test]
    fn test_try_layout_returns_none_for_missing_node() {
        let engine = LayoutEngine::new();
        assert!(engine.try_layout(DomId::new(999)).is_none());
    }
}
