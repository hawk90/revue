//! Layout engine wrapper around taffy

use std::collections::HashMap;
use taffy::prelude::*;
use super::Rect;
use super::convert::to_taffy_style;
use crate::style::Style;
use crate::dom::DomId;

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

/// A node identifier in the layout tree
pub type NodeId = taffy::NodeId;

/// Layout engine using taffy for Flexbox computation
pub struct LayoutEngine {
    taffy: TaffyTree,
    nodes: HashMap<DomId, NodeId>,
}

impl LayoutEngine {
    /// Create a new layout engine
    pub fn new() -> Self {
        Self {
            taffy: TaffyTree::new(),
            nodes: HashMap::new(),
        }
    }

    /// Create a new node with the given style
    ///
    /// Returns an error if the taffy tree fails to create the node.
    pub fn create_node(&mut self, dom_id: DomId, style: &Style) -> LayoutResult<()> {
        let taffy_style = to_taffy_style(style);
        let node_id = self.taffy.new_leaf(taffy_style)
            .map_err(|e| LayoutError::NodeCreationFailed(format!("{:?}", e)))?;
        self.nodes.insert(dom_id, node_id);
        Ok(())
    }

    /// Create a node with children
    ///
    /// Returns an error if the taffy tree fails to create the node.
    pub fn create_node_with_children(&mut self, dom_id: DomId, style: &Style, children: &[DomId]) -> LayoutResult<()> {
        let taffy_style = to_taffy_style(style);
        let child_nodes: Vec<NodeId> = children
            .iter()
            .filter_map(|id| self.nodes.get(id).copied())
            .collect();

        let node_id = self.taffy.new_with_children(taffy_style, &child_nodes)
            .map_err(|e| LayoutError::NodeCreationFailed(format!("{:?}", e)))?;
        self.nodes.insert(dom_id, node_id);
        Ok(())
    }

    /// Update a node's style
    ///
    /// Returns an error if the node is not found or style update fails.
    pub fn update_style(&mut self, dom_id: DomId, style: &Style) -> LayoutResult<()> {
        let &node_id = self.nodes.get(&dom_id)
            .ok_or_else(|| LayoutError::NodeNotFound(dom_id.inner()))?;
        let taffy_style = to_taffy_style(style);
        self.taffy.set_style(node_id, taffy_style)
            .map_err(|e| LayoutError::StyleUpdateFailed(format!("{:?}", e)))
    }

    /// Add a child to a node
    ///
    /// Returns an error if either node is not found or the operation fails.
    pub fn add_child(&mut self, parent_dom_id: DomId, child_dom_id: DomId) -> LayoutResult<()> {
        let &parent = self.nodes.get(&parent_dom_id)
            .ok_or_else(|| LayoutError::NodeNotFound(parent_dom_id.inner()))?;
        let &child = self.nodes.get(&child_dom_id)
            .ok_or_else(|| LayoutError::NodeNotFound(child_dom_id.inner()))?;
        self.taffy.add_child(parent, child)
            .map_err(|e| LayoutError::AddChildFailed(format!("{:?}", e)))
    }

    /// Remove a node
    ///
    /// Returns an error if the node removal fails. Returns Ok if node doesn't exist.
    pub fn remove_node(&mut self, dom_id: DomId) -> LayoutResult<()> {
        if let Some(node_id) = self.nodes.remove(&dom_id) {
            self.taffy.remove(node_id)
                .map_err(|e| LayoutError::RemoveFailed(format!("{:?}", e)))?;
        }
        Ok(())
    }

    /// Compute layout for a root node
    ///
    /// Returns an error if the root node is not found or computation fails.
    pub fn compute(&mut self, root_dom_id: DomId, available_width: u16, available_height: u16) -> LayoutResult<()> {
        let &node_id = self.nodes.get(&root_dom_id)
            .ok_or_else(|| LayoutError::NodeNotFound(root_dom_id.inner()))?;
        let available_space = taffy::Size {
            width: AvailableSpace::Definite(available_width as f32),
            height: AvailableSpace::Definite(available_height as f32),
        };
        self.taffy.compute_layout(node_id, available_space)
            .map_err(|e| LayoutError::ComputeFailed(format!("{:?}", e)))
    }

    /// Get the computed layout for a node
    ///
    /// Returns an error if the node is not found or layout retrieval fails.
    pub fn layout(&self, dom_id: DomId) -> LayoutResult<Rect> {
        let &node_id = self.nodes.get(&dom_id)
            .ok_or_else(|| LayoutError::NodeNotFound(dom_id.inner()))?;

        let layout = self.taffy.layout(node_id)
            .map_err(|e| LayoutError::LayoutRetrievalFailed(format!("{:?}", e)))?;

        // Safe f32 → u16 conversion with bounds checking
        let x = layout.location.x.max(0.0).min(u16::MAX as f32) as u16;
        let y = layout.location.y.max(0.0).min(u16::MAX as f32) as u16;
        let width = layout.size.width.max(0.0).min(u16::MAX as f32) as u16;
        let height = layout.size.height.max(0.0).min(u16::MAX as f32) as u16;

        debug_assert!(
            layout.location.x >= 0.0 && layout.location.x <= u16::MAX as f32,
            "Layout x overflow: {}",
            layout.location.x
        );
        debug_assert!(
            layout.location.y >= 0.0 && layout.location.y <= u16::MAX as f32,
            "Layout y overflow: {}",
            layout.location.y
        );

        Ok(Rect { x, y, width, height })
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
        self.taffy.clear();
        self.nodes.clear();
    }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
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
        assert_eq!(layout.width, 100);
        assert_eq!(layout.height, 50);
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
        engine.create_node_with_children(parent, &parent_style, &[child1, child2]).unwrap();

        engine.compute(parent, 300, 300).unwrap();

        let parent_layout = engine.layout(parent).unwrap();
        assert_eq!(parent_layout.width, 200);
        assert_eq!(parent_layout.height, 100);

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
