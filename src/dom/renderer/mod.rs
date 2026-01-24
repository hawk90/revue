//! DOM-aware rendering pipeline
//!
//! Integrates the DOM tree with style resolution and rendering.

mod build;
mod focus;
mod helpers;
mod incremental;
mod render;
mod style;
mod stylesheet;
mod types;

#[cfg(test)]
mod tests {
//! Unit tests for DOM renderer

#![allow(unused_imports)]

use crate::dom::renderer::helpers::styled_context;
use crate::dom::renderer::types::DomRenderer;
use crate::dom::WidgetMeta;
use crate::style::{parse_css, Color};
use crate::widget::{Stack, Text};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dom_renderer_new() {
        let renderer = DomRenderer::new_internal();
        assert!(renderer.tree.is_empty());
    }

    #[test]
    fn test_build_tree() {
        let mut renderer = DomRenderer::new_internal();

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
        let mut renderer = DomRenderer::new_internal();

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
        let mut renderer = DomRenderer::new_internal();

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
        let mut renderer = DomRenderer::new_internal();

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
        let mut renderer = DomRenderer::new_internal();

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
        let mut renderer = DomRenderer::new_internal();

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
        let mut renderer = DomRenderer::new_internal();

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
        let mut renderer = DomRenderer::new_internal();

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

    #[test]
    fn test_stylesheet_mut_invalidates_cache() {
        let css = r#"
            App { color: #FF0000; }
        "#;
        let stylesheet = parse_css(css).unwrap();
        let mut renderer = DomRenderer::with_stylesheet(stylesheet);

        // Build tree and compute styles to populate caches
        let root_id = renderer.tree.create_root(WidgetMeta::new("App").id("app"));
        renderer.compute_styles_with_inheritance();

        // Verify style was computed
        assert!(renderer.styles.contains_key(&root_id));

        // Get mutable stylesheet - should invalidate caches
        let sheet = renderer.stylesheet_mut();
        // Modify the stylesheet
        let new_css = parse_css("App { color: #00FF00; }").unwrap();
        sheet.merge(new_css);

        // Caches should be invalidated
        assert!(renderer.styles.is_empty());
        assert!(renderer.cached_selectors.is_none());
    }

    #[test]
    fn test_stylesheet_mut_returns_mutable_ref() {
        let mut renderer = DomRenderer::new_internal();
        let original_rules_count = renderer.stylesheet_mut().rules.len();
        assert_eq!(original_rules_count, 0);

        // Add rules through mutable reference
        let css = parse_css("Button { color: red; }").unwrap();
        renderer.stylesheet_mut().merge(css);

        // Verify rules were added
        assert_eq!(renderer.stylesheet_mut().rules.len(), 1);
    }
}

}

// Re-export the main type and helpers
pub use helpers::styled_context;
pub use types::DomRenderer;

// Implement Default trait
impl Default for DomRenderer {
    fn default() -> Self {
        Self::new_internal()
    }
}

// Public constructor
impl DomRenderer {
    /// Create a new DOM renderer with an empty stylesheet
    pub fn new() -> Self {
        Self::new_internal()
    }
}
