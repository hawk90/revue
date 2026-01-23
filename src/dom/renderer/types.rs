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
