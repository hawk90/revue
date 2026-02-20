//! Tree widget for displaying hierarchical data

use crate::style::Color;
use crate::utils::Selection;
use crate::widget::traits::{View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

mod search;
mod types;
mod view;

pub use types::TreeNode;

/// Callback type for node selection events
type SelectCallback = Box<dyn Fn(&TreeNode)>;

/// A tree widget for displaying hierarchical data
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
        self
    }

    /// Add a root node
    pub fn node(mut self, node: TreeNode) -> Self {
        self.root.push(node);
        self.selection.set_len(self.count_visible());
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
        self.on_select = Some(Box::new(callback));
        self
    }

    /// Get selected index
    pub fn selected_index(&self) -> usize {
        self.selection.index
    }

    /// Count visible nodes (expanded recursively)
    fn count_visible(&self) -> usize {
        fn count_nodes(nodes: &[TreeNode]) -> usize {
            let mut count = 0;
            for node in nodes {
                count += 1;
                if node.expanded && !node.children.is_empty() {
                    count += count_nodes(&node.children);
                }
            }
            count
        }
        count_nodes(&self.root)
    }

    /// Get node at visible index
    fn get_node_at(&self, index: usize) -> Option<(&TreeNode, usize)> {
        fn find_node<'a>(
            nodes: &'a [TreeNode],
            target: usize,
            current: &mut usize,
            depth: usize,
        ) -> Option<(&'a TreeNode, usize)> {
            for node in nodes {
                if *current == target {
                    return Some((node, depth));
                }
                *current += 1;
                if node.expanded && !node.children.is_empty() {
                    if let Some(result) = find_node(&node.children, target, current, depth + 1) {
                        return Some(result);
                    }
                }
            }
            None
        }
        let mut current = 0;
        find_node(&self.root, index, &mut current, 0)
    }

    /// Get mutable node at visible index
    fn get_node_mut_at(&mut self, index: usize) -> Option<&mut TreeNode> {
        fn find_node_mut<'a>(
            nodes: &'a mut [TreeNode],
            target: usize,
            current: &mut usize,
        ) -> Option<&'a mut TreeNode> {
            for node in nodes {
                if *current == target {
                    return Some(node);
                }
                *current += 1;
                if node.expanded && !node.children.is_empty() {
                    if let Some(result) = find_node_mut(&mut node.children, target, current) {
                        return Some(result);
                    }
                }
            }
            None
        }
        let mut current = 0;
        find_node_mut(&mut self.root, index, &mut current)
    }

    /// Select next visible node
    pub fn select_next(&mut self) {
        self.selection.down();
    }

    /// Select previous visible node
    pub fn select_prev(&mut self) {
        self.selection.up();
    }

    /// Select first node
    pub fn select_first(&mut self) {
        self.selection.first();
    }

    /// Select last visible node
    pub fn select_last(&mut self) {
        self.selection.last();
    }

    /// Toggle expand/collapse of selected node
    pub fn toggle_expand(&mut self) {
        if let Some(node) = self.get_node_mut_at(self.selection.index) {
            if node.has_children() {
                node.expanded = !node.expanded;
                self.selection.set_len(self.count_visible());
            }
        }
    }

    /// Expand selected node
    pub fn expand(&mut self) {
        if let Some(node) = self.get_node_mut_at(self.selection.index) {
            if node.has_children() && !node.expanded {
                node.expanded = true;
                self.selection.set_len(self.count_visible());
            }
        }
    }

    /// Collapse selected node
    pub fn collapse(&mut self) {
        if let Some(node) = self.get_node_mut_at(self.selection.index) {
            if node.expanded {
                node.expanded = false;
                self.selection.set_len(self.count_visible());
            }
        }
    }

    /// Toggle selection of current node in multi-select mode
    pub fn toggle_select(&mut self) {
        let idx = self.selection.index;
        if let Some(pos) = self.selected_indices.iter().position(|&i| i == idx) {
            self.selected_indices.remove(pos);
        } else {
            self.selected_indices.push(idx);
        }
        if let Some(callback) = &self.on_select {
            if let Some((node, _)) = self.get_node_at(idx) {
                callback(node);
            }
        }
    }

    /// Get all selected nodes in multi-select mode
    pub fn selected_nodes(&self) -> Vec<&TreeNode> {
        self.selected_indices
            .iter()
            .filter_map(|&idx| self.get_node_at(idx).map(|(node, _)| node))
            .collect()
    }

    /// Get IDs of all selected nodes in multi-select mode
    pub fn selected_ids(&self) -> Vec<&str> {
        self.selected_nodes()
            .into_iter()
            .filter_map(|node| node.id.as_deref())
            .collect()
    }

    /// Check if a visible index is selected in multi-select mode
    pub fn is_multi_selected(&self, index: usize) -> bool {
        self.selected_indices.contains(&index)
    }

    /// Find the parent index of the node at the given visible index
    fn find_parent_index(&self, index: usize) -> Option<usize> {
        fn find_parent(
            nodes: &[TreeNode],
            target: usize,
            current: &mut usize,
            parent_index: Option<usize>,
        ) -> Option<usize> {
            for node in nodes {
                if *current == target {
                    return parent_index;
                }
                let my_index = *current;
                *current += 1;
                if node.expanded && !node.children.is_empty() {
                    if let Some(result) =
                        find_parent(&node.children, target, current, Some(my_index))
                    {
                        return Some(result);
                    }
                }
            }
            None
        }
        let mut current = 0;
        find_parent(&self.root, index, &mut current, None)
    }

    /// Handle key input, returns true if selection or expansion changed
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        // When searchable and has query, handle search navigation
        if self.searchable && !self.query.is_empty() {
            match key {
                Key::Char('n') => return self.next_match(),
                Key::Char('N') => return self.prev_match(),
                Key::Escape => {
                    self.clear_query();
                    return true;
                }
                Key::Backspace => {
                    self.query.pop();
                    self.update_matches();
                    return true;
                }
                _ => {}
            }
        }

        // When searchable, typing adds to query (except special keys)
        if self.searchable {
            if let Key::Char(c) = key {
                // Allow alphanumeric and common search chars
                if c.is_alphanumeric() || *c == '_' || *c == '-' || *c == '.' || *c == '/' {
                    self.query.push(*c);
                    self.update_matches();
                    return true;
                }
            }
        }

        match key {
            Key::Up | Key::Char('k') if !self.searchable => {
                let old = self.selection.index;
                self.select_prev();
                old != self.selection.index
            }
            Key::Up if self.searchable => {
                let old = self.selection.index;
                self.select_prev();
                old != self.selection.index
            }
            Key::Down | Key::Char('j') if !self.searchable => {
                let old = self.selection.index;
                self.select_next();
                old != self.selection.index
            }
            Key::Down if self.searchable => {
                let old = self.selection.index;
                self.select_next();
                old != self.selection.index
            }
            Key::Char(' ') if self.multi_select && !self.searchable => {
                self.toggle_select();
                true
            }
            Key::Enter | Key::Char(' ') if !self.searchable => {
                let old_count = self.selection.len;
                self.toggle_expand();
                old_count != self.selection.len
            }
            Key::Enter if self.searchable => {
                let old_count = self.selection.len;
                self.toggle_expand();
                old_count != self.selection.len
            }
            Key::Right | Key::Char('l') if !self.searchable => {
                let old_count = self.selection.len;
                self.expand();
                old_count != self.selection.len
            }
            Key::Right if self.searchable => {
                let old_count = self.selection.len;
                self.expand();
                old_count != self.selection.len
            }
            Key::Left | Key::Char('h') if !self.searchable => {
                // If node is expanded, collapse it; otherwise navigate to parent
                let is_expanded = self
                    .get_node_at(self.selection.index)
                    .map(|(n, _)| n.expanded && n.has_children())
                    .unwrap_or(false);
                if is_expanded {
                    let old_count = self.selection.len;
                    self.collapse();
                    old_count != self.selection.len
                } else if let Some(parent_idx) = self.find_parent_index(self.selection.index) {
                    let old = self.selection.index;
                    self.selection.set(parent_idx);
                    old != self.selection.index
                } else {
                    false
                }
            }
            Key::Left if self.searchable => {
                // If node is expanded, collapse it; otherwise navigate to parent
                let is_expanded = self
                    .get_node_at(self.selection.index)
                    .map(|(n, _)| n.expanded && n.has_children())
                    .unwrap_or(false);
                if is_expanded {
                    let old_count = self.selection.len;
                    self.collapse();
                    old_count != self.selection.len
                } else if let Some(parent_idx) = self.find_parent_index(self.selection.index) {
                    let old = self.selection.index;
                    self.selection.set(parent_idx);
                    old != self.selection.index
                } else {
                    false
                }
            }
            Key::Home => {
                let old = self.selection.index;
                self.select_first();
                old != self.selection.index
            }
            Key::End => {
                let old = self.selection.index;
                self.select_last();
                old != self.selection.index
            }
            _ => false,
        }
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

// Tests extracted to tests/widget/data/tree_mod.rs
//
// KEEP HERE: Tests that access private fields (fg, bg, indent)

#[cfg(test)]
mod tests {
    //! Tests for Tree widget that access private fields

    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

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

    // =========================================================================
    // Render tests (require internal buffer access)
    // =========================================================================

    #[test]
    fn test_tree_render() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tree::new().node(TreeNode::new("Files"));

        t.render(&mut ctx);

        // Check collapse indicator and label
        assert_eq!(buffer.get(0, 0).unwrap().symbol, ' '); // No children = space
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'F');
    }

    #[test]
    fn test_tree_render_with_expanded_children() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tree::new()
            .node(
                TreeNode::new("Parent")
                    .expanded(true)
                    .child(TreeNode::new("Child 1"))
                    .child(TreeNode::new("Child 2")),
            )
            .selected_style(Color::WHITE, Color::BLUE);

        t.render(&mut ctx);

        // Check expand indicator
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '▼');
    }

    #[test]
    fn test_tree_render_nested() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tree::new().node(
            TreeNode::new("Level 0").expanded(true).child(
                TreeNode::new("Level 1")
                    .expanded(true)
                    .child(TreeNode::new("Level 2")),
            ),
        );

        t.render(&mut ctx);
        // Smoke test - just verify no panic
    }

    #[test]
    fn test_tree_render_empty() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tree::new();
        t.render(&mut ctx);
        // Empty tree should not panic
    }

    #[test]
    fn test_tree_render_small_area() {
        let mut buffer = Buffer::new(2, 1);
        let area = Rect::new(0, 0, 2, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tree::new().node(TreeNode::new("Test"));
        t.render(&mut ctx);
        // Small area should not panic
    }

    #[test]
    fn test_tree_render_with_selection() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tree::new()
            .nodes(vec![TreeNode::new("First"), TreeNode::new("Second")])
            .selected(1);

        t.render(&mut ctx);
        // Verify selection rendering
    }

    #[test]
    fn test_tree_render_with_highlight() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut t = Tree::new()
            .nodes(vec![TreeNode::new("Hello World")])
            .searchable(true)
            .highlight_fg(Color::YELLOW);

        t.set_query("hw");
        t.render(&mut ctx);
        // Verify highlight rendering
    }
}
