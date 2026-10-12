//! Tree widget for displaying hierarchical data

use crate::style::Color;
use crate::utils::Selection;
use crate::widget::traits::{View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

mod keys;
mod navigation;
mod rows;
mod search;
mod types;
mod view;

pub use types::TreeNode;

/// Callback type for node selection events
///
/// Shared, so a cloned tree calls the same callback.
type SelectCallback = std::sync::Arc<dyn Fn(&TreeNode)>;

/// A tree widget for displaying hierarchical data
///
/// A clone has its own nodes and selection and shares the `on_select`
/// callback.
#[derive(Clone)]
pub struct Tree {
    root: Vec<TreeNode>,
    selection: Selection,
    fg: Option<Color>,
    bg: Option<Color>,
    selected_fg: Option<Color>,
    selected_bg: Option<Color>,
    highlight_fg: Option<Color>,
    indent: u16,
    /// Search query for filtering/highlighting
    query: String,
    /// Enable search mode
    searchable: bool,
    /// Matched node indices (visible indices)
    matches: Vec<usize>,
    /// Current match index in matches vec
    current_match: usize,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
    /// Enable multi-select mode
    multi_select: bool,
    /// Indices of selected nodes in multi-select mode
    selected_indices: Vec<usize>,
    /// Callback invoked when a node is selected
    on_select: Option<SelectCallback>,
}

impl Tree {
    /// Create a new tree widget
    pub fn new() -> Self {
        Self {
            root: Vec::new(),
            selection: Selection::new(0),
            fg: None,
            bg: None,
            selected_fg: Some(Color::WHITE),
            selected_bg: Some(Color::BLUE),
            highlight_fg: Some(Color::YELLOW),
            indent: 2,
            query: String::new(),
            searchable: false,
            matches: Vec::new(),
            current_match: 0,
            props: WidgetProps::new(),
            multi_select: false,
            selected_indices: Vec::new(),
            on_select: None,
        }
    }

    /// Enable search mode for fuzzy filtering
    pub fn searchable(mut self, enable: bool) -> Self {
        self.searchable = enable;
        self
    }

    /// Set highlight color for matched characters
    pub fn highlight_fg(mut self, color: Color) -> Self {
        self.highlight_fg = Some(color);
        self
    }

    /// Set root nodes
    pub fn nodes(mut self, nodes: Vec<TreeNode>) -> Self {
        self.root = nodes;
        self.selection.set_len(self.count_visible());
        self.settle_selection();
        self
    }

    /// Add a root node
    pub fn node(mut self, node: TreeNode) -> Self {
        self.root.push(node);
        self.selection.set_len(self.count_visible());
        self.settle_selection();
        self
    }

    /// Set selected index
    pub fn selected(mut self, index: usize) -> Self {
        self.selection.set(index);
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set selected colors
    pub fn selected_style(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = Some(fg);
        self.selected_bg = Some(bg);
        self
    }

    /// Set indent width
    pub fn indent(mut self, indent: u16) -> Self {
        self.indent = indent;
        self
    }

    /// Enable multi-select mode
    pub fn multi_select(mut self, enable: bool) -> Self {
        self.multi_select = enable;
        self
    }

    /// Set selection callback invoked when a node is selected
    pub fn on_select(mut self, callback: impl Fn(&TreeNode) + 'static) -> Self {
        self.on_select = Some(std::sync::Arc::new(callback));
        self
    }

    /// Get selected index
    pub fn selected_index(&self) -> usize {
        self.selection.index
    }

    /// Get number of root nodes
    pub fn len(&self) -> usize {
        self.root.len()
    }

    /// Check if tree is empty
    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    /// Get number of visible nodes
    pub fn visible_count(&self) -> usize {
        self.selection.len
    }

    /// Get selected node label
    pub fn selected_label(&self) -> Option<&str> {
        self.get_node_at(self.selection.index)
            .map(|(n, _)| n.label.as_str())
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Tree {
    crate::impl_view_meta!("Tree");

    fn render(&self, ctx: &mut crate::widget::traits::RenderContext) {
        self.render_internal(ctx);
    }
}

impl_styled_view!(Tree);
impl_props_builders!(Tree);

/// Helper function to create a tree widget
pub fn tree() -> Tree {
    Tree::new()
}

/// Helper function to create a tree node
pub fn tree_node(label: impl Into<String>) -> TreeNode {
    TreeNode::new(label)
}

// KEEP HERE: Tests that access private fields (fg, bg, indent)

#[cfg(test)]
mod tests {
    //! Tests for Tree widget that access private fields

    use super::*;

    // =========================================================================
    // Private field access tests
    // =========================================================================

    #[test]
    fn test_tree_fg_bg_colors() {
        // KEEP HERE: accesses private fields fg and bg
        let t = Tree::new().fg(Color::RED).bg(Color::BLUE);
        assert_eq!(t.fg, Some(Color::RED));
        assert_eq!(t.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_tree_indent() {
        // KEEP HERE: accesses private field indent
        let t = Tree::new().indent(4);
        assert_eq!(t.indent, 4);
    }
}
