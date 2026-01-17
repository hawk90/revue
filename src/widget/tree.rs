//! Tree widget for displaying hierarchical data

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{fuzzy_match, FuzzyMatch, Selection};
use crate::{impl_props_builders, impl_styled_view};

/// A tree node
#[derive(Clone)]
pub struct TreeNode {
    /// Node label
    pub label: String,
    /// Child nodes
    pub children: Vec<TreeNode>,
    /// Whether node is expanded
    pub expanded: bool,
}

impl TreeNode {
    /// Create a new tree node
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            children: Vec::new(),
            expanded: false,
        }
    }

    /// Add a child node
    pub fn child(mut self, node: TreeNode) -> Self {
        self.children.push(node);
        self
    }

    /// Add multiple children
    pub fn children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    /// Set expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Check if node has children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Create a leaf node (no children)
    pub fn leaf(label: impl Into<String>) -> Self {
        Self::new(label)
    }
}

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

    // --- Fuzzy search methods ---

    /// Get current search query
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Set search query and find matches
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.update_matches();
    }

    /// Clear search query
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.matches.clear();
        self.current_match = 0;
    }

    /// Check if searchable mode is enabled
    pub fn is_searchable(&self) -> bool {
        self.searchable
    }

    /// Get number of matches
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Get current match index (1-based for display)
    pub fn current_match_index(&self) -> usize {
        if self.matches.is_empty() {
            0
        } else {
            self.current_match + 1
        }
    }

    /// Update matches based on current query
    fn update_matches(&mut self) {
        self.matches.clear();
        self.current_match = 0;

        if self.query.is_empty() {
            return;
        }

        // Find all visible nodes that match
        fn find_matches(
            nodes: &[TreeNode],
            query: &str,
            visible_index: &mut usize,
            matches: &mut Vec<usize>,
        ) {
            for node in nodes {
                if fuzzy_match(query, &node.label).is_some() {
                    matches.push(*visible_index);
                }
                *visible_index += 1;
                if node.expanded && !node.children.is_empty() {
                    find_matches(&node.children, query, visible_index, matches);
                }
            }
        }

        let mut visible_index = 0;
        find_matches(
            &self.root,
            &self.query,
            &mut visible_index,
            &mut self.matches,
        );

        // Jump to first match
        if let Some(&first) = self.matches.first() {
            self.selection.set(first);
        }
    }

    /// Jump to next match
    pub fn next_match(&mut self) -> bool {
        if self.matches.is_empty() {
            return false;
        }
        self.current_match = (self.current_match + 1) % self.matches.len();
        self.selection.set(self.matches[self.current_match]);
        true
    }

    /// Jump to previous match
    pub fn prev_match(&mut self) -> bool {
        if self.matches.is_empty() {
            return false;
        }
        self.current_match = self
            .current_match
            .checked_sub(1)
            .unwrap_or(self.matches.len() - 1);
        self.selection.set(self.matches[self.current_match]);
        true
    }

    /// Get fuzzy match for a label
    pub fn get_match(&self, label: &str) -> Option<FuzzyMatch> {
        if self.query.is_empty() {
            None
        } else {
            fuzzy_match(&self.query, label)
        }
    }

    /// Check if a visible index is a match
    pub fn is_match(&self, visible_index: usize) -> bool {
        self.matches.contains(&visible_index)
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
                let old_count = self.selection.len;
                self.collapse();
                old_count != self.selection.len
            }
            Key::Left if self.searchable => {
                let old_count = self.selection.len;
                self.collapse();
                old_count != self.selection.len
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

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 || self.root.is_empty() {
            return;
        }

        let mut y = area.y;
        let mut visible_index = 0;

        fn render_nodes(
            tree: &Tree,
            nodes: &[TreeNode],
            ctx: &mut RenderContext,
            y: &mut u16,
            visible_index: &mut usize,
            depth: usize,
            is_last_stack: &[bool],
        ) {
            let area = ctx.area;

            for (i, node) in nodes.iter().enumerate() {
                if *y >= area.y + area.height {
                    return;
                }

                let is_selected = *visible_index == tree.selection.index;
                let is_last = i == nodes.len() - 1;

                let (fg, bg) = if is_selected {
                    (tree.selected_fg, tree.selected_bg)
                } else {
                    (tree.fg, tree.bg)
                };

                // Draw background if selected
                if is_selected {
                    for x in area.x..area.x + area.width {
                        let mut cell = Cell::new(' ');
                        cell.bg = bg;
                        ctx.buffer.set(x, *y, cell);
                    }
                }

                let mut x = area.x;

                // Draw tree lines for depth
                for (d, &parent_is_last) in is_last_stack.iter().enumerate() {
                    if d < depth {
                        let ch = if parent_is_last { ' ' } else { '│' };
                        let mut cell = Cell::new(ch);
                        cell.fg = fg;
                        cell.bg = bg;
                        ctx.buffer.set(x, *y, cell);
                        x += 1;
                        // Add spacing
                        for _ in 1..tree.indent {
                            let mut cell = Cell::new(' ');
                            cell.bg = bg;
                            ctx.buffer.set(x, *y, cell);
                            x += 1;
                        }
                    }
                }

                // Draw current node connector
                if depth > 0 {
                    let connector = if is_last { '└' } else { '├' };
                    let mut cell = Cell::new(connector);
                    cell.fg = fg;
                    cell.bg = bg;
                    ctx.buffer.set(x, *y, cell);
                    x += 1;

                    // Draw horizontal line
                    let mut cell = Cell::new('─');
                    cell.fg = fg;
                    cell.bg = bg;
                    ctx.buffer.set(x, *y, cell);
                    x += 1;
                }

                // Draw expand/collapse indicator
                let indicator = if node.has_children() {
                    if node.expanded {
                        '▼'
                    } else {
                        '▶'
                    }
                } else {
                    ' '
                };
                let mut cell = Cell::new(indicator);
                cell.fg = fg;
                cell.bg = bg;
                ctx.buffer.set(x, *y, cell);
                x += 1;

                // Draw label with optional highlighting
                let available_width = (area.x + area.width).saturating_sub(x) as usize;
                let truncated: String = node.label.chars().take(available_width).collect();

                // Get match indices for highlighting
                let match_indices: Vec<usize> = tree
                    .get_match(&node.label)
                    .map(|m| m.indices)
                    .unwrap_or_default();

                for (idx, ch) in truncated.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.bg = bg;

                    // Highlight matched characters
                    if match_indices.contains(&idx) {
                        cell.fg = tree.highlight_fg;
                    } else {
                        cell.fg = fg;
                    }

                    ctx.buffer.set(x, *y, cell);
                    x += 1;
                }

                *y += 1;
                *visible_index += 1;

                // Render children if expanded
                if node.expanded && !node.children.is_empty() {
                    let mut new_stack = is_last_stack.to_vec();
                    new_stack.push(is_last);
                    render_nodes(
                        tree,
                        &node.children,
                        ctx,
                        y,
                        visible_index,
                        depth + 1,
                        &new_stack,
                    );
                }
            }
        }

        render_nodes(self, &self.root, ctx, &mut y, &mut visible_index, 0, &[]);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_tree_new() {
        let t = Tree::new();
        assert!(t.is_empty());
        assert_eq!(t.selected_index(), 0);
    }

    #[test]
    fn test_tree_node() {
        let node = TreeNode::new("Root")
            .child(TreeNode::new("Child 1"))
            .child(TreeNode::new("Child 2"));

        assert_eq!(node.label, "Root");
        assert_eq!(node.children.len(), 2);
        assert!(node.has_children());
    }

    #[test]
    fn test_tree_builder() {
        let t = Tree::new()
            .node(TreeNode::new("A"))
            .node(TreeNode::new("B"));

        assert_eq!(t.len(), 2);
        assert_eq!(t.visible_count(), 2);
    }

    #[test]
    fn test_tree_navigation() {
        let t = Tree::new().nodes(vec![
            TreeNode::new("One"),
            TreeNode::new("Two"),
            TreeNode::new("Three"),
        ]);

        let mut t = t;
        assert_eq!(t.selected_index(), 0);

        t.select_next();
        assert_eq!(t.selected_index(), 1);

        t.select_next();
        assert_eq!(t.selected_index(), 2);

        t.select_next(); // Should stay at last
        assert_eq!(t.selected_index(), 2);

        t.select_prev();
        assert_eq!(t.selected_index(), 1);

        t.select_first();
        assert_eq!(t.selected_index(), 0);

        t.select_last();
        assert_eq!(t.selected_index(), 2);
    }

    #[test]
    fn test_tree_expand_collapse() {
        let mut t = Tree::new().node(
            TreeNode::new("Parent")
                .child(TreeNode::new("Child 1"))
                .child(TreeNode::new("Child 2")),
        );

        // Initially collapsed
        assert_eq!(t.visible_count(), 1);

        // Expand
        t.toggle_expand();
        assert_eq!(t.visible_count(), 3);

        // Collapse
        t.toggle_expand();
        assert_eq!(t.visible_count(), 1);
    }

    #[test]
    fn test_tree_handle_key() {
        use crate::event::Key;

        let mut t = Tree::new().node(
            TreeNode::new("Root")
                .child(TreeNode::new("A"))
                .child(TreeNode::new("B")),
        );

        // Expand
        t.handle_key(&Key::Enter);
        assert_eq!(t.visible_count(), 3);

        // Navigate down
        t.handle_key(&Key::Down);
        assert_eq!(t.selected_index(), 1);

        // Navigate up
        t.handle_key(&Key::Up);
        assert_eq!(t.selected_index(), 0);
    }

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
    fn test_tree_selected_label() {
        let t = Tree::new().nodes(vec![TreeNode::new("First"), TreeNode::new("Second")]);

        assert_eq!(t.selected_label(), Some("First"));
    }

    #[test]
    fn test_tree_helper() {
        let t = tree().node(tree_node("Test"));

        assert_eq!(t.len(), 1);
    }

    #[test]
    fn test_tree_node_leaf() {
        let leaf = TreeNode::leaf("Leaf");
        assert!(!leaf.has_children());
    }

    #[test]
    fn test_tree_searchable() {
        let mut t = Tree::new()
            .nodes(vec![
                TreeNode::new("apple.rs"),
                TreeNode::new("banana.rs"),
                TreeNode::new("cherry.rs"),
            ])
            .searchable(true);

        assert!(t.is_searchable());
        assert_eq!(t.query(), "");

        // Set query
        t.set_query("ap");
        assert_eq!(t.query(), "ap");
        assert_eq!(t.match_count(), 1); // only apple.rs
        assert_eq!(t.selected_index(), 0); // jumped to first match

        // Search for 'rs' - matches all
        t.set_query("rs");
        assert_eq!(t.match_count(), 3);

        // Clear query
        t.clear_query();
        assert_eq!(t.query(), "");
        assert_eq!(t.match_count(), 0);
    }

    #[test]
    fn test_tree_search_navigation() {
        let mut t = Tree::new()
            .nodes(vec![
                TreeNode::new("file1.txt"),
                TreeNode::new("file2.txt"),
                TreeNode::new("other.rs"),
                TreeNode::new("file3.txt"),
            ])
            .searchable(true);

        // Search for "file"
        t.set_query("file");
        assert_eq!(t.match_count(), 3);
        assert_eq!(t.current_match_index(), 1); // 1-based
        assert_eq!(t.selected_index(), 0); // file1.txt

        // Next match
        t.next_match();
        assert_eq!(t.current_match_index(), 2);
        assert_eq!(t.selected_index(), 1); // file2.txt

        // Next match
        t.next_match();
        assert_eq!(t.current_match_index(), 3);
        assert_eq!(t.selected_index(), 3); // file3.txt

        // Wrap around
        t.next_match();
        assert_eq!(t.current_match_index(), 1);
        assert_eq!(t.selected_index(), 0); // file1.txt

        // Previous match
        t.prev_match();
        assert_eq!(t.current_match_index(), 3);
        assert_eq!(t.selected_index(), 3); // file3.txt
    }

    #[test]
    fn test_tree_search_expanded() {
        let mut t = Tree::new()
            .node(
                TreeNode::new("src")
                    .expanded(true)
                    .child(TreeNode::new("main.rs"))
                    .child(TreeNode::new("lib.rs")),
            )
            .node(TreeNode::new("Cargo.toml"))
            .searchable(true);

        // Search for ".rs"
        t.set_query(".rs");
        assert_eq!(t.match_count(), 2); // main.rs, lib.rs
    }

    #[test]
    fn test_tree_get_match() {
        let mut t = Tree::new()
            .nodes(vec![TreeNode::new("Hello World")])
            .searchable(true);

        // No match when no query
        assert!(t.get_match("Hello World").is_none());

        // Set query
        t.set_query("hw");

        // Should have match with indices
        let m = t.get_match("Hello World");
        assert!(m.is_some());
        let m = m.unwrap();
        assert!(m.indices.contains(&0)); // H
        assert!(m.indices.contains(&6)); // W
    }

    #[test]
    fn test_tree_searchable_keys() {
        use crate::event::Key;

        let mut t = Tree::new()
            .nodes(vec![
                TreeNode::new("apple"),
                TreeNode::new("apricot"),
                TreeNode::new("banana"),
            ])
            .searchable(true);

        // Type 'a' - matches all 3 (apple, apricot, banana all contain 'a')
        t.handle_key(&Key::Char('a'));
        assert_eq!(t.query(), "a");
        assert_eq!(t.match_count(), 3);

        // Type 'p' -> "ap" only matches apple, apricot
        t.handle_key(&Key::Char('p'));
        assert_eq!(t.query(), "ap");
        assert_eq!(t.match_count(), 2);

        // Backspace
        t.handle_key(&Key::Backspace);
        assert_eq!(t.query(), "a");

        // n for next match
        t.handle_key(&Key::Char('n'));
        assert_eq!(t.current_match_index(), 2);

        // N for prev match
        t.handle_key(&Key::Char('N'));
        assert_eq!(t.current_match_index(), 1);

        // Escape clears query
        t.handle_key(&Key::Escape);
        assert_eq!(t.query(), "");
    }

    // =========================================================================
    // Additional coverage tests
    // =========================================================================

    #[test]
    fn test_tree_node_children_builder() {
        let children = vec![TreeNode::new("Child 1"), TreeNode::new("Child 2")];
        let node = TreeNode::new("Parent").children(children);
        assert_eq!(node.children.len(), 2);
    }

    #[test]
    fn test_tree_node_expanded_builder() {
        let node = TreeNode::new("Test").expanded(true);
        assert!(node.expanded);
    }

    #[test]
    fn test_tree_fg_bg_colors() {
        let t = Tree::new().fg(Color::RED).bg(Color::BLUE);
        assert_eq!(t.fg, Some(Color::RED));
        assert_eq!(t.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_tree_indent() {
        let t = Tree::new().indent(4);
        assert_eq!(t.indent, 4);
    }

    #[test]
    fn test_tree_expand_collapse_methods() {
        let mut t = Tree::new().node(
            TreeNode::new("Parent")
                .child(TreeNode::new("Child 1"))
                .child(TreeNode::new("Child 2")),
        );

        // Initially collapsed
        assert_eq!(t.visible_count(), 1);

        // Expand
        t.expand();
        assert_eq!(t.visible_count(), 3);

        // Collapse
        t.collapse();
        assert_eq!(t.visible_count(), 1);
    }

    #[test]
    fn test_tree_expand_already_expanded() {
        let mut t = Tree::new().node(
            TreeNode::new("Parent")
                .expanded(true)
                .child(TreeNode::new("Child")),
        );

        let visible_before = t.visible_count();
        t.expand();
        assert_eq!(t.visible_count(), visible_before);
    }

    #[test]
    fn test_tree_collapse_already_collapsed() {
        let mut t = Tree::new().node(TreeNode::new("Parent").child(TreeNode::new("Child")));

        // Already collapsed
        t.collapse();
        assert_eq!(t.visible_count(), 1);
    }

    #[test]
    fn test_tree_expand_leaf_node() {
        let mut t = Tree::new().node(TreeNode::new("Leaf"));

        // Expanding a leaf should do nothing
        t.expand();
        assert_eq!(t.visible_count(), 1);
    }

    #[test]
    fn test_tree_is_match() {
        let mut t = Tree::new()
            .nodes(vec![TreeNode::new("file1.txt"), TreeNode::new("file2.txt")])
            .searchable(true);

        t.set_query("file1");
        assert!(t.is_match(0));
        assert!(!t.is_match(1));
    }

    #[test]
    fn test_tree_current_match_index_empty() {
        let t = Tree::new()
            .nodes(vec![TreeNode::new("test")])
            .searchable(true);

        // No query means no matches
        assert_eq!(t.current_match_index(), 0);
    }

    #[test]
    fn test_tree_next_prev_match_empty() {
        let mut t = Tree::new()
            .nodes(vec![TreeNode::new("test")])
            .searchable(true);

        // No matches, should return false
        assert!(!t.next_match());
        assert!(!t.prev_match());
    }

    #[test]
    fn test_tree_handle_key_non_searchable_vim_keys() {
        use crate::event::Key;

        let mut t = Tree::new().nodes(vec![
            TreeNode::new("One"),
            TreeNode::new("Two"),
            TreeNode::new("Three"),
        ]);

        // j for down
        assert!(t.handle_key(&Key::Char('j')));
        assert_eq!(t.selected_index(), 1);

        // k for up
        assert!(t.handle_key(&Key::Char('k')));
        assert_eq!(t.selected_index(), 0);
    }

    #[test]
    fn test_tree_handle_key_expand_with_space() {
        use crate::event::Key;

        let mut t = Tree::new().node(TreeNode::new("Parent").child(TreeNode::new("Child")));

        assert!(t.handle_key(&Key::Char(' ')));
        assert_eq!(t.visible_count(), 2); // Expanded
    }

    #[test]
    fn test_tree_handle_key_expand_collapse_with_vim() {
        use crate::event::Key;

        let mut t = Tree::new().node(TreeNode::new("Parent").child(TreeNode::new("Child")));

        // l for expand
        assert!(t.handle_key(&Key::Char('l')));
        assert_eq!(t.visible_count(), 2);

        // h for collapse
        assert!(t.handle_key(&Key::Char('h')));
        assert_eq!(t.visible_count(), 1);
    }

    #[test]
    fn test_tree_handle_key_searchable_up_down() {
        use crate::event::Key;

        let mut t = Tree::new()
            .nodes(vec![TreeNode::new("One"), TreeNode::new("Two")])
            .searchable(true);

        assert!(t.handle_key(&Key::Down));
        assert_eq!(t.selected_index(), 1);

        assert!(t.handle_key(&Key::Up));
        assert_eq!(t.selected_index(), 0);
    }

    #[test]
    fn test_tree_handle_key_searchable_enter() {
        use crate::event::Key;

        let mut t = Tree::new()
            .node(TreeNode::new("Parent").child(TreeNode::new("Child")))
            .searchable(true);

        assert!(t.handle_key(&Key::Enter));
        assert_eq!(t.visible_count(), 2);
    }

    #[test]
    fn test_tree_handle_key_searchable_left_right() {
        use crate::event::Key;

        let mut t = Tree::new()
            .node(TreeNode::new("Parent").child(TreeNode::new("Child")))
            .searchable(true);

        assert!(t.handle_key(&Key::Right));
        assert_eq!(t.visible_count(), 2);

        assert!(t.handle_key(&Key::Left));
        assert_eq!(t.visible_count(), 1);
    }

    #[test]
    fn test_tree_handle_key_special_search_chars() {
        use crate::event::Key;

        let mut t = Tree::new()
            .nodes(vec![
                TreeNode::new("file-name"),
                TreeNode::new("file_name"),
                TreeNode::new("file.name"),
                TreeNode::new("path/to/file"),
            ])
            .searchable(true);

        // Test special chars that are allowed in search
        t.handle_key(&Key::Char('-'));
        assert_eq!(t.query(), "-");

        t.clear_query();
        t.handle_key(&Key::Char('_'));
        assert_eq!(t.query(), "_");

        t.clear_query();
        t.handle_key(&Key::Char('.'));
        assert_eq!(t.query(), ".");

        t.clear_query();
        t.handle_key(&Key::Char('/'));
        assert_eq!(t.query(), "/");
    }

    #[test]
    fn test_tree_handle_key_unhandled() {
        use crate::event::Key;

        let mut t = Tree::new().node(TreeNode::new("Test"));

        // Tab key is not handled
        assert!(!t.handle_key(&Key::Tab));
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

    #[test]
    fn test_tree_selected_label_none() {
        let t = Tree::new();
        assert!(t.selected_label().is_none());
    }

    #[test]
    fn test_tree_default() {
        let t = Tree::default();
        assert!(t.is_empty());
    }
}
