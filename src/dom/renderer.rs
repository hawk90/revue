//! DOM-aware rendering pipeline
//!
//! Integrates the DOM tree with style resolution and rendering.

use crate::dom::{DomId, DomNode, DomTree, NodeState, Query, StyleResolver, WidgetMeta};
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::{Style, StyleSheet};
use crate::widget::{RenderContext, View};

/// DOM-aware renderer
///
/// Manages the widget tree, computes styles via CSS cascade,
/// and renders widgets with proper styling context.
pub struct DomRenderer {
    /// The DOM tree
    tree: DomTree,
    /// The stylesheet, owned by the renderer
    stylesheet: StyleSheet,
    /// Computed styles cache
    styles: std::collections::HashMap<DomId, Style>,
    /// Cached parsed selectors (selector, rule_index)
    /// Cached to avoid reparsing selectors on every style computation
    cached_selectors: Option<Vec<(crate::dom::Selector, usize)>>,
    /// Focused node
    focused: Option<DomId>,
    /// Hovered node
    hovered: Option<DomId>,
}

impl DomRenderer {
    /// Create a new DOM renderer with an empty stylesheet
    pub fn new() -> Self {
        Self {
            tree: DomTree::new(),
            stylesheet: StyleSheet::new(),
            styles: std::collections::HashMap::new(),
            cached_selectors: None,
            focused: None,
            hovered: None,
        }
    }

    /// Create with a stylesheet
    pub fn with_stylesheet(stylesheet: StyleSheet) -> Self {
        Self {
            tree: DomTree::new(),
            stylesheet,
            styles: std::collections::HashMap::new(),
            cached_selectors: None,
            focused: None,
            hovered: None,
        }
    }

    /// Set the stylesheet
    pub fn set_stylesheet(&mut self, stylesheet: StyleSheet) {
        self.stylesheet = stylesheet;
        // Invalidate cached selectors when stylesheet changes
        self.cached_selectors = None;
        // Invalidate style cache
        self.styles.clear();
    }

    /// Build DOM from a View hierarchy
    ///
    /// This method now performs incremental updates when possible:
    /// - Reuses existing nodes that match by element ID or position
    /// - Only marks changed nodes as dirty
    /// - Preserves style cache for unchanged nodes
    pub fn build<V: View>(&mut self, root: &V) {
        if self.tree.is_empty() {
            // First build - create from scratch
            self.build_fresh(root);
        } else {
            // Incremental update
            self.build_incremental(root);
        }
    }

    /// Build DOM from scratch (first frame or full rebuild)
    fn build_fresh<V: View>(&mut self, root: &V) {
        self.tree = DomTree::new();
        self.styles.clear();

        // Create root node and recursively build children
        let meta = root.meta();
        let root_id = self.tree.create_root(meta);

        // Recursively build child nodes
        self.build_children(root_id, root.children());
    }

    /// Incremental DOM update - reuses existing nodes when possible
    fn build_incremental<V: View>(&mut self, root: &V) {
        let Some(root_id) = self.tree.root_id() else {
            // No root, do fresh build
            self.build_fresh(root);
            return;
        };

        // Update root node
        let new_meta = root.meta();
        if !self.update_node_meta(root_id, &new_meta) {
            // Root changed - full rebuild needed
            self.build_fresh(root);
            return;
        }

        // Recursively update children
        self.update_children(root_id, root.children());
    }

    /// Update node metadata if changed, returns true if node can be reused
    fn update_node_meta(&mut self, node_id: DomId, new_meta: &WidgetMeta) -> bool {
        let Some(node) = self.tree.get(node_id) else {
            return false;
        };

        // Check if widget type matches (required for reuse)
        if node.meta.widget_type != new_meta.widget_type {
            return false;
        }

        // Check if element ID matches (if present)
        if node.meta.id != new_meta.id {
            return false;
        }

        // Check if classes changed
        if node.meta.classes != new_meta.classes {
            // Classes changed - update and mark dirty
            if let Some(node) = self.tree.get_mut(node_id) {
                node.meta.classes = new_meta.classes.clone();
                node.state.dirty = true;
            }
            // Invalidate cached style
            self.styles.remove(&node_id);
        }

        true
    }

    /// Recursively update children, reusing nodes when possible
    fn update_children(&mut self, parent_id: DomId, new_children: &[Box<dyn View>]) {
        // Get current children IDs
        let old_children: Vec<DomId> = self
            .tree
            .get(parent_id)
            .map(|n| n.children.clone())
            .unwrap_or_default();

        // Build ID lookup map for efficient matching
        let mut old_by_id: std::collections::HashMap<String, DomId> =
            std::collections::HashMap::new();

        for &child_id in &old_children {
            if let Some(node) = self.tree.get(child_id) {
                if let Some(ref id) = node.meta.id {
                    old_by_id.insert(id.clone(), child_id);
                }
            }
        }

        // Collect widget types for positional matching (need owned strings)
        let old_types: Vec<String> = old_children
            .iter()
            .filter_map(|&id| self.tree.get(id).map(|n| n.meta.widget_type.clone()))
            .collect();

        let mut matched_old: std::collections::HashSet<DomId> = std::collections::HashSet::new();
        let mut new_child_ids: Vec<DomId> = Vec::new();

        for (pos, child_view) in new_children.iter().enumerate() {
            let child_meta = child_view.meta();

            // Try to find matching existing node
            let matched_id = if let Some(ref id) = child_meta.id {
                // Match by element ID (highest priority)
                old_by_id.get(id).copied()
            } else {
                // Match by position and type
                old_children.get(pos).and_then(|&old_id| {
                    if !matched_old.contains(&old_id) {
                        let old_type = old_types.get(pos)?;
                        if old_type == &child_meta.widget_type {
                            Some(old_id)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            };

            let child_id = if let Some(existing_id) = matched_id {
                if !matched_old.contains(&existing_id) {
                    // Reuse existing node
                    matched_old.insert(existing_id);

                    // Update meta if needed
                    if !self.update_node_meta(existing_id, &child_meta) {
                        // Type mismatch - remove old and create new
                        self.remove_subtree(existing_id);
                        let new_id = self.tree.add_child(parent_id, child_meta);
                        self.build_children(new_id, child_view.children());
                        new_id
                    } else {
                        // Recursively update grandchildren
                        self.update_children(existing_id, child_view.children());
                        existing_id
                    }
                } else {
                    // Already matched - create new
                    let new_id = self.tree.add_child(parent_id, child_meta);
                    self.build_children(new_id, child_view.children());
                    new_id
                }
            } else {
                // No match - create new node
                let new_id = self.tree.add_child(parent_id, child_meta);
                self.build_children(new_id, child_view.children());
                new_id
            };

            new_child_ids.push(child_id);
        }

        // Remove unmatched old children
        for old_id in old_children {
            if !matched_old.contains(&old_id) && !new_child_ids.contains(&old_id) {
                self.remove_subtree(old_id);
            }
        }

        // Update parent's children list
        if let Some(parent) = self.tree.get_mut(parent_id) {
            parent.children = new_child_ids;
        }
    }

    /// Remove a node and its entire subtree
    fn remove_subtree(&mut self, node_id: DomId) {
        // Collect all descendant IDs first
        let descendants = self.collect_descendants(node_id);

        // Remove styles for all nodes
        self.styles.remove(&node_id);
        for &id in &descendants {
            self.styles.remove(&id);
        }

        // Remove from tree
        self.tree.remove(node_id);
    }

    /// Collect all descendant node IDs
    fn collect_descendants(&self, node_id: DomId) -> Vec<DomId> {
        let mut result = Vec::new();
        let mut stack = vec![node_id];

        while let Some(id) = stack.pop() {
            if let Some(node) = self.tree.get(id) {
                for &child_id in &node.children {
                    result.push(child_id);
                    stack.push(child_id);
                }
            }
        }

        result
    }

    /// Force a full DOM rebuild on next build() call
    pub fn invalidate(&mut self) {
        self.tree = DomTree::new();
        self.styles.clear();
    }

    /// Recursively build child nodes
    fn build_children(&mut self, parent_id: super::DomId, children: &[Box<dyn View>]) {
        for child in children {
            let child_meta = child.meta();
            let child_id = self.tree.add_child(parent_id, child_meta);

            // Recursively process this child's children
            self.build_children(child_id, child.children());
        }
    }

    /// Build DOM with children
    pub fn build_tree(&mut self, root_meta: WidgetMeta, children: Vec<WidgetMeta>) {
        self.tree = DomTree::new();
        self.styles.clear();

        let root_id = self.tree.create_root(root_meta);
        for child_meta in children {
            self.tree.add_child(root_id, child_meta);
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

    /// Set focused node by element ID
    pub fn set_focus(&mut self, element_id: Option<&str>) {
        let new_focus_id = element_id.and_then(|id| self.tree.get_by_id(id).map(|node| node.id));

        // Mark old focused node as dirty
        if let Some(old_id) = self.focused {
            if Some(old_id) != new_focus_id {
                if let Some(node) = self.tree.get_mut(old_id) {
                    node.state.dirty = true;
                }
            }
        }

        // Mark new focused node as dirty
        if let Some(new_id) = new_focus_id {
            if Some(new_id) != self.focused {
                if let Some(node) = self.tree.get_mut(new_id) {
                    node.state.dirty = true;
                }
            }
        }

        self.focused = new_focus_id;
        self.tree.set_focused(new_focus_id);

        // Invalidate affected styles
        if let Some(id) = new_focus_id {
            self.styles.remove(&id);
        }
    }

    /// Set hovered node by element ID
    pub fn set_hover(&mut self, element_id: Option<&str>) {
        let new_hover_id = element_id.and_then(|id| self.tree.get_by_id(id).map(|node| node.id));

        // Mark old hovered node as dirty
        if let Some(old_id) = self.hovered {
            if Some(old_id) != new_hover_id {
                if let Some(node) = self.tree.get_mut(old_id) {
                    node.state.dirty = true;
                }
            }
        }

        // Mark new hovered node as dirty
        if let Some(new_id) = new_hover_id {
            if Some(new_id) != self.hovered {
                if let Some(node) = self.tree.get_mut(new_id) {
                    node.state.dirty = true;
                }
            }
        }

        self.hovered = new_hover_id;
        self.tree.set_hovered(new_hover_id);
        // Invalidate affected styles
        if let Some(id) = new_hover_id {
            self.styles.remove(&id);
        }
    }

    /// Get or create cached parsed selectors
    fn get_cached_selectors(&mut self) -> &Vec<(crate::dom::Selector, usize)> {
        if self.cached_selectors.is_none() {
            // Parse all selectors and cache them
            let mut selectors = Vec::new();
            for (idx, rule) in self.stylesheet.rules.iter().enumerate() {
                if let Ok(selector) = crate::dom::parse_selector(&rule.selector) {
                    selectors.push((selector, idx));
                }
            }
            self.cached_selectors = Some(selectors);
        }
        self.cached_selectors.as_ref().unwrap()
    }

    /// Get computed style for a node (without inheritance)
    pub fn style_for(&mut self, node_id: DomId) -> Option<Style> {
        // Check cache
        if let Some(style) = self.styles.get(&node_id) {
            return Some(style.clone());
        }

        // Get cached selectors (parsed once, reused across all nodes)
        let cached_selectors = self.get_cached_selectors().clone();

        // Create resolver with cached selectors (avoids reparsing)
        let resolver = StyleResolver::with_cached_selectors(&self.stylesheet, cached_selectors);
        let node = self.tree.get(node_id)?;

        // Create closure for node lookup
        let get_node = |id: DomId| -> Option<&DomNode> { self.tree.get(id) };

        let style = resolver.compute_style(node, get_node);
        self.styles.insert(node_id, style.clone());
        Some(style)
    }

    /// Get computed style for a node with inheritance from parent
    pub fn style_for_with_inheritance(&mut self, node_id: DomId) -> Option<Style> {
        // Check cache
        if let Some(style) = self.styles.get(&node_id) {
            return Some(style.clone());
        }

        // Get parent info first (to avoid borrow conflicts)
        let parent_id = self.tree.get(node_id)?.parent;

        // Ensure parent is computed first (recursive call)
        if let Some(pid) = parent_id {
            if !self.styles.contains_key(&pid) {
                self.style_for_with_inheritance(pid);
            }
        }

        // Now get parent style from cache
        let parent_style = parent_id.and_then(|pid| self.styles.get(&pid).cloned());

        // Get cached selectors (parsed once, reused across all nodes)
        let cached_selectors = self.get_cached_selectors().clone();

        // Create resolver with cached selectors (avoids reparsing)
        let resolver = StyleResolver::with_cached_selectors(&self.stylesheet, cached_selectors);
        let node = self.tree.get(node_id)?;

        // Create closure for node lookup
        let get_node = |id: DomId| -> Option<&DomNode> { self.tree.get(id) };

        let style = resolver.compute_style_with_parent(node, parent_style.as_ref(), get_node);
        self.styles.insert(node_id, style.clone());
        Some(style)
    }

    /// Compute all styles (without inheritance)
    pub fn compute_styles(&mut self) {
        // Collect all node IDs first
        let node_ids: Vec<DomId> = self.tree.nodes().map(|n| n.id).collect();

        for node_id in node_ids {
            let _ = self.style_for(node_id);
        }
    }

    /// Compute all styles with CSS inheritance
    ///
    /// Walks the tree from root to leaves, ensuring parents are computed
    /// before children so inherited properties propagate correctly.
    pub fn compute_styles_with_inheritance(&mut self) {
        // With dirty tracking, we no longer need to clear the entire cache
        // Only dirty nodes will be recomputed (see compute_subtree_styles)

        // Start from root and walk tree in order
        if let Some(root_id) = self.tree.root_id() {
            self.compute_subtree_styles(root_id);
        }
    }

    /// Recursively compute styles for a subtree
    fn compute_subtree_styles(&mut self, node_id: DomId) {
        if let Some(node) = self.tree.get(node_id) {
            // Optimization: if node is not dirty and its style is already cached,
            // we can assume its children are also up-to-date.
            if !node.state.dirty && self.styles.contains_key(&node_id) {
                return;
            }
        }

        // Compute this node's style first
        let _ = self.style_for_with_inheritance(node_id);

        // Mark the node as clean after computing its style
        if let Some(node) = self.tree.get_mut(node_id) {
            node.state.dirty = false;
        }

        // Get children (need to collect to avoid borrow issues)
        let children: Vec<DomId> = self
            .tree
            .get(node_id)
            .map(|n| n.children.clone())
            .unwrap_or_default();

        // Recursively compute children
        for child_id in children {
            self.compute_subtree_styles(child_id);
        }
    }

    /// Render with DOM context (with CSS inheritance)
    pub fn render<V: View>(&mut self, root: &V, buffer: &mut Buffer, area: Rect) {
        // Compute styles with inheritance
        self.compute_styles_with_inheritance();

        // Get root style and state
        let root_id = self.tree.root_id();
        let (style, state) = if let Some(id) = root_id {
            let style = self.styles.get(&id);
            let state = self.tree.get(id).map(|n| &n.state);
            (style, state)
        } else {
            (None, None)
        };

        // Create context with style and state
        let mut ctx = if let (Some(style), Some(state)) = (style, state) {
            RenderContext::full(buffer, area, style, state)
        } else if let Some(style) = style {
            RenderContext::with_style(buffer, area, style)
        } else {
            RenderContext::new(buffer, area)
        };

        root.render(&mut ctx);
    }

    /// Query nodes by selector
    pub fn query(&self, selector: &str) -> Vec<&DomNode> {
        self.tree.query_all(selector).all().to_vec()
    }

    /// Query one node by selector
    pub fn query_one(&self, selector: &str) -> Option<&DomNode> {
        self.tree.query_one(selector)
    }

    /// Get node by element ID
    pub fn get_by_id(&self, id: &str) -> Option<&DomNode> {
        self.tree.get_by_id(id)
    }
}

impl Default for DomRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to create a styled render context from DOM node
pub fn styled_context<'a>(
    buffer: &'a mut Buffer,
    area: Rect,
    style: &'a Style,
    state: &'a NodeState,
) -> RenderContext<'a> {
    RenderContext::full(buffer, area, style, state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::WidgetMeta;
    use crate::style::{parse_css, Color};

    #[test]
    fn test_dom_renderer_new() {
        let renderer = DomRenderer::new();
        assert!(renderer.tree.is_empty());
    }

    #[test]
    fn test_build_tree() {
        let mut renderer = DomRenderer::new();

        let root = WidgetMeta::new("App").id("app");
        let children = vec![
            WidgetMeta::new("Button").id("btn1").class("primary"),
            WidgetMeta::new("Button").id("btn2"),
        ];

        renderer.build_tree(root, children);

        assert_eq!(renderer.tree.len(), 3);
        assert!(renderer.get_by_id("app").is_some());
        assert!(renderer.get_by_id("btn1").is_some());
        assert!(renderer.get_by_id("btn2").is_some());
    }

    #[test]
    fn test_query_nodes() {
        let mut renderer = DomRenderer::new();

        let root = WidgetMeta::new("App").id("app");
        let children = vec![
            WidgetMeta::new("Button").class("primary"),
            WidgetMeta::new("Button").class("secondary"),
            WidgetMeta::new("Input"),
        ];

        renderer.build_tree(root, children);

        let buttons = renderer.query("Button");
        assert_eq!(buttons.len(), 2);

        let primary = renderer.query(".primary");
        assert_eq!(primary.len(), 1);
    }

    #[test]
    fn test_focus_management() {
        let mut renderer = DomRenderer::new();

        let root = WidgetMeta::new("App").id("app");
        let children = vec![
            WidgetMeta::new("Button").id("btn1"),
            WidgetMeta::new("Button").id("btn2"),
        ];

        renderer.build_tree(root, children);

        renderer.set_focus(Some("btn1"));

        let btn1 = renderer.get_by_id("btn1").unwrap();
        assert!(btn1.state.focused);

        let btn2 = renderer.get_by_id("btn2").unwrap();
        assert!(!btn2.state.focused);
    }

    #[test]
    fn test_style_inheritance_color() {
        // CSS: App sets color, children inherit it
        let css = r#"
            App {
                color: #FF0000;
            }
        "#;
        let stylesheet = parse_css(css).unwrap();
        let mut renderer = DomRenderer::with_stylesheet(stylesheet);

        // Build tree: App -> Button
        let root_id = renderer.tree.create_root(WidgetMeta::new("App").id("app"));
        let child_id = renderer
            .tree
            .add_child(root_id, WidgetMeta::new("Button").id("btn"));

        // Compute styles with inheritance
        renderer.compute_styles_with_inheritance();

        // Parent has explicit color
        let app_style = renderer.styles.get(&root_id).unwrap();
        assert_eq!(app_style.visual.color, Color::hex(0xFF0000));

        // Child inherits color from parent
        let btn_style = renderer.styles.get(&child_id).unwrap();
        assert_eq!(btn_style.visual.color, Color::hex(0xFF0000));
    }

    #[test]
    fn test_style_inheritance_override() {
        // CSS: App sets color, Button overrides it
        let css = r#"
            App {
                color: #FF0000;
            }
            Button {
                color: #00FF00;
            }
        "#;
        let stylesheet = parse_css(css).unwrap();
        let mut renderer = DomRenderer::with_stylesheet(stylesheet);

        // Build tree: App -> Button
        let root_id = renderer.tree.create_root(WidgetMeta::new("App").id("app"));
        let child_id = renderer
            .tree
            .add_child(root_id, WidgetMeta::new("Button").id("btn"));

        // Compute styles with inheritance
        renderer.compute_styles_with_inheritance();

        // Parent has red color
        let app_style = renderer.styles.get(&root_id).unwrap();
        assert_eq!(app_style.visual.color, Color::hex(0xFF0000));

        // Child overrides with green
        let btn_style = renderer.styles.get(&child_id).unwrap();
        assert_eq!(btn_style.visual.color, Color::hex(0x00FF00));
    }

    #[test]
    fn test_style_non_inheritance_background() {
        // CSS: App sets background, children should NOT inherit it
        let css = r#"
            App {
                background: #0000FF;
            }
        "#;
        let stylesheet = parse_css(css).unwrap();
        let mut renderer = DomRenderer::with_stylesheet(stylesheet);

        // Build tree: App -> Button
        let root_id = renderer.tree.create_root(WidgetMeta::new("App").id("app"));
        let child_id = renderer
            .tree
            .add_child(root_id, WidgetMeta::new("Button").id("btn"));

        // Compute styles with inheritance
        renderer.compute_styles_with_inheritance();

        // Parent has blue background
        let app_style = renderer.styles.get(&root_id).unwrap();
        assert_eq!(app_style.visual.background, Color::hex(0x0000FF));

        // Child should NOT inherit background (non-inherited property)
        let btn_style = renderer.styles.get(&child_id).unwrap();
        assert_eq!(btn_style.visual.background, Color::default());
    }

    #[test]
    fn test_style_deep_inheritance() {
        // CSS: App sets color, deeply nested child inherits it
        let css = r#"
            App {
                color: #FF0000;
            }
        "#;
        let stylesheet = parse_css(css).unwrap();
        let mut renderer = DomRenderer::with_stylesheet(stylesheet);

        // Build tree: App -> Container -> Button
        let root_id = renderer.tree.create_root(WidgetMeta::new("App").id("app"));
        let container_id = renderer
            .tree
            .add_child(root_id, WidgetMeta::new("Container"));
        let btn_id = renderer
            .tree
            .add_child(container_id, WidgetMeta::new("Button"));

        // Compute styles with inheritance
        renderer.compute_styles_with_inheritance();

        // All nodes inherit the red color
        let app_style = renderer.styles.get(&root_id).unwrap();
        assert_eq!(app_style.visual.color, Color::hex(0xFF0000));

        let container_style = renderer.styles.get(&container_id).unwrap();
        assert_eq!(container_style.visual.color, Color::hex(0xFF0000));

        let btn_style = renderer.styles.get(&btn_id).unwrap();
        assert_eq!(btn_style.visual.color, Color::hex(0xFF0000));
    }

    #[test]
    fn test_build_recursive_tree() {
        use crate::widget::{Stack, Text};

        let mut renderer = DomRenderer::new();

        // Build a nested view tree: Stack -> [Text, Stack -> [Text, Text]]
        let root = Stack::new()
            .element_id("root")
            .child(Text::new("First").element_id("text1"))
            .child(
                Stack::new()
                    .element_id("nested")
                    .child(Text::new("Second").element_id("text2"))
                    .child(Text::new("Third").element_id("text3")),
            );

        // Build DOM from View hierarchy
        renderer.build(&root);

        // Verify root exists
        let root_node = renderer.get_by_id("root");
        assert!(root_node.is_some(), "Root node should exist");
        let root_node = root_node.unwrap();
        assert_eq!(root_node.meta.widget_type, "Stack");

        // Verify first child (Text)
        let text1 = renderer.get_by_id("text1");
        assert!(text1.is_some(), "First text node should exist");
        assert_eq!(text1.unwrap().meta.widget_type, "Text");

        // Verify nested Stack
        let nested = renderer.get_by_id("nested");
        assert!(nested.is_some(), "Nested stack should exist");
        assert_eq!(nested.unwrap().meta.widget_type, "Stack");

        // Verify nested Stack's children
        let text2 = renderer.get_by_id("text2");
        assert!(text2.is_some(), "Second text node should exist");
        assert_eq!(text2.unwrap().meta.widget_type, "Text");

        let text3 = renderer.get_by_id("text3");
        assert!(text3.is_some(), "Third text node should exist");
        assert_eq!(text3.unwrap().meta.widget_type, "Text");

        // Verify tree structure using children count
        assert_eq!(root_node.children.len(), 2, "Root should have 2 children");
        assert_eq!(
            nested.unwrap().children.len(),
            2,
            "Nested stack should have 2 children"
        );
    }

    #[test]
    fn test_incremental_build_reuses_nodes_by_id() {
        use crate::widget::{Stack, Text};

        let mut renderer = DomRenderer::new();

        // First build
        let view1 = Stack::new()
            .element_id("root")
            .child(Text::new("Hello").element_id("greeting"));

        renderer.build(&view1);

        let original_root_id = renderer.tree.root_id().unwrap();
        let original_greeting_id = renderer.get_by_id("greeting").unwrap().id;

        // Second build with same structure
        let view2 = Stack::new()
            .element_id("root")
            .child(Text::new("Hello").element_id("greeting"));

        renderer.build(&view2);

        // Nodes should be reused (same IDs)
        let new_root_id = renderer.tree.root_id().unwrap();
        let new_greeting_id = renderer.get_by_id("greeting").unwrap().id;

        assert_eq!(original_root_id, new_root_id, "Root node should be reused");
        assert_eq!(
            original_greeting_id, new_greeting_id,
            "Child node should be reused"
        );
    }

    #[test]
    fn test_incremental_build_detects_class_change() {
        use crate::widget::{Stack, Text};

        let css = r#"
            .highlight { color: #FF0000; }
        "#;
        let stylesheet = parse_css(css).unwrap();
        let mut renderer = DomRenderer::with_stylesheet(stylesheet);

        // First build
        let view1 = Stack::new()
            .element_id("root")
            .child(Text::new("Hello").element_id("text"));

        renderer.build(&view1);
        renderer.compute_styles_with_inheritance();

        let text_id = renderer.get_by_id("text").unwrap().id;

        // Mark as clean
        if let Some(node) = renderer.tree.get_mut(text_id) {
            node.state.dirty = false;
        }

        // Second build with class added
        let view2 = Stack::new()
            .element_id("root")
            .child(Text::new("Hello").element_id("text").class("highlight"));

        renderer.build(&view2);

        // Node should still exist and be marked dirty
        let text_node = renderer.get_by_id("text").unwrap();
        assert!(text_node.has_class("highlight"), "Class should be added");
        assert!(text_node.state.dirty, "Node should be marked dirty");
    }

    #[test]
    fn test_incremental_build_handles_child_addition() {
        use crate::widget::{Stack, Text};

        let mut renderer = DomRenderer::new();

        // First build with one child
        let view1 = Stack::new()
            .element_id("root")
            .child(Text::new("First").element_id("first"));

        renderer.build(&view1);
        assert_eq!(renderer.tree.len(), 2);

        // Second build with two children
        let view2 = Stack::new()
            .element_id("root")
            .child(Text::new("First").element_id("first"))
            .child(Text::new("Second").element_id("second"));

        renderer.build(&view2);

        assert_eq!(renderer.tree.len(), 3);
        assert!(renderer.get_by_id("first").is_some());
        assert!(renderer.get_by_id("second").is_some());
    }

    #[test]
    fn test_incremental_build_handles_child_removal() {
        use crate::widget::{Stack, Text};

        let mut renderer = DomRenderer::new();

        // First build with two children
        let view1 = Stack::new()
            .element_id("root")
            .child(Text::new("First").element_id("first"))
            .child(Text::new("Second").element_id("second"));

        renderer.build(&view1);
        assert_eq!(renderer.tree.len(), 3);

        // Second build with one child
        let view2 = Stack::new()
            .element_id("root")
            .child(Text::new("First").element_id("first"));

        renderer.build(&view2);

        assert_eq!(renderer.tree.len(), 2);
        assert!(renderer.get_by_id("first").is_some());
        assert!(renderer.get_by_id("second").is_none());
    }

    #[test]
    fn test_invalidate_forces_fresh_build() {
        use crate::widget::{Stack, Text};

        let mut renderer = DomRenderer::new();

        // First build
        let view = Stack::new().element_id("root").child(Text::new("Hello"));

        renderer.build(&view);
        let original_root_id = renderer.tree.root_id().unwrap();

        // Invalidate
        renderer.invalidate();
        assert!(renderer.tree.is_empty());

        // Rebuild
        renderer.build(&view);
        let new_root_id = renderer.tree.root_id().unwrap();

        // Should be a new node (different ID)
        assert_ne!(
            original_root_id, new_root_id,
            "Should create new node after invalidate"
        );
    }
}
