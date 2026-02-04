//! Pseudo-class tests

use crate::dom::cascade::resolver::StyleResolver;
use crate::dom::node::{DomNode, WidgetMeta};
use crate::dom::DomId;
use crate::style::{Declaration, Rule, StyleSheet};

#[test]
fn test_match_pseudo_hover() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "Button:hover".to_string(),
            declarations: vec![Declaration {
                property: "background".to_string(),
                value: "red".to_string(),
            }],
        }],
        variables: std::collections::HashMap::new(),
    };
    let mut resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Button"));

    let get_node = |_: DomId| -> Option<&DomNode> { None };

    // Not hovered - should not match
    let matches = resolver.match_node(&node, &get_node);
    assert_eq!(matches.len(), 0);

    // Hovered - should match
    node.state.hovered = true;
    let matches = resolver.match_node(&node, &get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_match_pseudo_focus() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "Input:focus".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let mut resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Input"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    // Not focused
    let matches = resolver.match_node(&node, &get_node);
    assert_eq!(matches.len(), 0);

    // Focused
    node.state.focused = true;
    let matches = resolver.match_node(&node, &get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_match_pseudo_disabled() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "Button:disabled".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let mut resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Button"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    node.state.disabled = true;
    let matches = resolver.match_node(&node, &get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_match_pseudo_first_child() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "Item:first-child".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let mut resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Item"));
    node.state.update_position(0, 3); // First of 3
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, &get_node);
    assert_eq!(matches.len(), 1);

    // Not first child
    node.state.update_position(1, 3); // Second of 3
    let matches = resolver.match_node(&node, &get_node);
    assert_eq!(matches.len(), 0);
}

#[test]
fn test_match_pseudo_last_child() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "Item:last-child".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let mut resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Item"));
    node.state.update_position(2, 3); // Last of 3
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, &get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_match_pseudo_checked() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "Checkbox:checked".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let mut resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Checkbox"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    // Not checked
    let matches = resolver.match_node(&node, &get_node);
    assert_eq!(matches.len(), 0);

    // Checked
    node.state.checked = true;
    let matches = resolver.match_node(&node, &get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_match_pseudo_selected() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "Option:selected".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let mut resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Option"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    node.state.selected = true;
    let matches = resolver.match_node(&node, &get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_match_pseudo_only_child() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "Item:only-child".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let mut resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Item"));
    node.state.update_position(0, 1); // Only child
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, &get_node);
    assert_eq!(matches.len(), 1);
}
