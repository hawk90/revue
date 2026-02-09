//! Core types for DOM renderer

use crate::dom::{DomId, DomTree};
use crate::style::{Style, StyleSheet};

/// DOM-aware renderer
///
/// Manages the widget tree, computes styles via CSS cascade,
/// and renders widgets with proper styling context.
pub struct DomRenderer {
    /// The DOM tree
    pub(crate) tree: DomTree,
    /// The stylesheet, owned by the renderer
    pub(crate) stylesheet: StyleSheet,
    /// Computed styles cache
    pub(crate) styles: std::collections::HashMap<DomId, Style>,
    /// Cached parsed selectors (selector, rule_index)
    /// Cached to avoid reparsing selectors on every style computation
    pub(crate) cached_selectors: Option<Vec<(crate::dom::Selector, usize)>>,
    /// Focused node
    pub(crate) focused: Option<DomId>,
    /// Hovered node
    pub(crate) hovered: Option<DomId>,
}

impl DomRenderer {
    /// Create a new DOM renderer with minimal fields
    pub(crate) fn new_internal() -> Self {
        Self {
            tree: DomTree::new(),
            stylesheet: StyleSheet::new(),
            styles: std::collections::HashMap::new(),
            cached_selectors: None,
            focused: None,
            hovered: None,
        }
    }

    /// Get the DOM tree
    pub fn tree(&self) -> &DomTree {
        &self.tree
    }

    /// Get mutable DOM tree
    pub fn tree_mut(&mut self) -> &mut DomTree {
        &mut self.tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dom_renderer_tree() {
        // Can't directly create DomRenderer since new_internal is pub(crate)
        // But we can test the public API through other constructors
        let renderer = DomRenderer::with_stylesheet(crate::style::StyleSheet::new());
        let tree = renderer.tree();
        assert!(tree.is_empty());
    }

    #[test]
    fn test_dom_renderer_tree_mut() {
        let mut renderer = DomRenderer::with_stylesheet(crate::style::StyleSheet::new());
        let tree = renderer.tree_mut();
        assert!(tree.is_empty());
    }

    #[test]
    fn test_dom_renderer_tree_and_tree_mut_different() {
        let mut renderer = DomRenderer::with_stylesheet(crate::style::StyleSheet::new());
        let tree_ref = renderer.tree() as *const DomTree;
        let tree_mut = renderer.tree_mut() as *const DomTree;
        // Both should point to the same tree
        assert_eq!(tree_ref, tree_mut);
    }
}
