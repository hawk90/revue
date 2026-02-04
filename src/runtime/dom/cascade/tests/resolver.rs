//! StyleResolver tests

use crate::dom::cascade::resolver::StyleResolver;
use crate::dom::node::{DomNode, WidgetMeta};
use crate::dom::DomId;
use crate::style::{Declaration, Rule, StyleSheet};

fn create_test_stylesheet() -> StyleSheet {
    StyleSheet {
        rules: vec![
            Rule {
                selector: "Button".to_string(),
                declarations: vec![Declaration {
                    property: "padding".to_string(),
                    value: "1".to_string(),
                }],
            },
            Rule {
                selector: ".primary".to_string(),
                declarations: vec![Declaration {
                    property: "background".to_string(),
                    value: "blue".to_string(),
                }],
            },
            Rule {
                selector: "#submit".to_string(),
                declarations: vec![Declaration {
                    property: "width".to_string(),
                    value: "100".to_string(),
                }],
            },
        ],
        variables: std::collections::HashMap::new(),
    }
}

fn create_button_node(id: u64) -> DomNode {
    let dom_id = DomId::new(id);
    DomNode::new(dom_id, WidgetMeta::new("Button"))
}

fn create_node_with_class(id: u64, widget_type: &str, class: &str) -> DomNode {
    let dom_id = DomId::new(id);
    DomNode::new(dom_id, WidgetMeta::new(widget_type).class(class))
}

fn create_node_with_id(id: u64, widget_type: &str, element_id: &str) -> DomNode {
    let dom_id = DomId::new(id);
    DomNode::new(dom_id, WidgetMeta::new(widget_type).id(element_id))
}

#[test]
fn test_resolver_new() {
    let stylesheet = create_test_stylesheet();
    let resolver = StyleResolver::new(&stylesheet);

    // Should have parsed 3 selectors
    assert_eq!(resolver.selectors.len(), 3);
}

#[test]
fn test_resolver_match_by_type() {
    let stylesheet = create_test_stylesheet();
    let mut resolver = StyleResolver::new(&stylesheet);

    let node = create_button_node(1);
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].rule.selector, "Button");
}

#[test]
fn test_resolver_match_by_class() {
    let stylesheet = create_test_stylesheet();
    let mut resolver = StyleResolver::new(&stylesheet);

    let node = create_node_with_class(1, "Text", "primary");
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].rule.selector, ".primary");
}

#[test]
fn test_resolver_match_by_id() {
    let stylesheet = create_test_stylesheet();
    let mut resolver = StyleResolver::new(&stylesheet);

    let node = create_node_with_id(1, "Button", "submit");
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    // Should match Button and #submit
    assert_eq!(matches.len(), 2);
}

#[test]
fn test_resolver_match_multiple() {
    let stylesheet = create_test_stylesheet();
    let mut resolver = StyleResolver::new(&stylesheet);

    // Node that matches type, class, and id
    let dom_id = DomId::new(1);
    let node = DomNode::new(
        dom_id,
        WidgetMeta::new("Button").class("primary").id("submit"),
    );
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    // Should match all 3 rules
    assert_eq!(matches.len(), 3);

    // Should be sorted by specificity (type < class < id)
    assert_eq!(matches[0].rule.selector, "Button");
    assert_eq!(matches[1].rule.selector, ".primary");
    assert_eq!(matches[2].rule.selector, "#submit");
}

#[test]
fn test_resolver_no_match() {
    let stylesheet = create_test_stylesheet();
    let mut resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let node = DomNode::new(dom_id, WidgetMeta::new("Text")); // No matching rules
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 0);
}

#[test]
fn test_compute_style_basic() {
    let stylesheet = create_test_stylesheet();
    let mut resolver = StyleResolver::new(&stylesheet);

    let node = create_button_node(1);
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let style = resolver.compute_style(&node, get_node);
    // Verify style is computed (at minimum it's a default style)
    let _ = style; // Style was computed successfully
}

#[test]
fn test_with_cached_selectors() {
    let stylesheet = create_test_stylesheet();
    let mut resolver1 = StyleResolver::new(&stylesheet);

    // Get cached selectors
    let cached = resolver1.selectors.clone();

    // Create new resolver with cached selectors (pass reference)
    let mut resolver2 = StyleResolver::with_cached_selectors(&stylesheet, &cached);

    let node = create_button_node(1);
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    // Both should produce same results
    let matches1 = resolver1.match_node(&node, &get_node);
    let matches2 = resolver2.match_node(&node, &get_node);
    assert_eq!(matches1.len(), matches2.len());
}

#[test]
fn test_resolver_invalid_selector() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[invalid".to_string(), // Invalid selector syntax
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    // Invalid selectors should be skipped
    assert_eq!(resolver.selectors.len(), 0);
}

#[test]
fn test_universal_selector() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "*".to_string(),
            declarations: vec![Declaration {
                property: "color".to_string(),
                value: "white".to_string(),
            }],
        }],
        variables: std::collections::HashMap::new(),
    };
    let mut resolver = StyleResolver::new(&stylesheet);

    let node = create_button_node(1);
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_compute_style_with_inline() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "Button".to_string(),
            declarations: vec![Declaration {
                property: "opacity".to_string(),
                value: "0.5".to_string(),
            }],
        }],
        variables: std::collections::HashMap::new(),
    };
    let mut resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Button"));
    // Set inline style
    let mut inline = crate::style::Style::default();
    inline.visual.opacity = 0.8;
    node.inline_style = Some(inline);

    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let style = resolver.compute_style(&node, get_node);
    // Inline style should override rule
    assert!((style.visual.opacity - 0.8).abs() < 0.001);
}

#[test]
fn test_compute_style_with_parent() {
    use crate::style::Color;

    let stylesheet = StyleSheet {
        rules: vec![],
        variables: std::collections::HashMap::new(),
    };
    let mut resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let node = DomNode::new(dom_id, WidgetMeta::new("Text"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    // Parent with color
    let mut parent_style = crate::style::Style::default();
    parent_style.visual.color = Color::hex(0xff0000);

    let style = resolver.compute_style_with_parent(&node, Some(&parent_style), get_node);
    // Color should be inherited
    assert_eq!(style.visual.color, Color::hex(0xff0000));
}

#[test]
fn test_compute_style_with_parent_none() {
    let stylesheet = StyleSheet {
        rules: vec![],
        variables: std::collections::HashMap::new(),
    };
    let mut resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let node = DomNode::new(dom_id, WidgetMeta::new("Text"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let style = resolver.compute_style_with_parent(&node, None, get_node);
    // Should be default style
    assert!(style.visual.visible);
}
