//! Combinator tests

use crate::dom::cascade::resolver::StyleResolver;
use crate::dom::node::{DomNode, WidgetMeta};
use crate::dom::DomId;
use crate::style::{Rule, StyleSheet};

#[test]
fn test_match_descendant_combinator() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "Container Button".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    // Create parent node
    let parent_id = DomId::new(1);
    let parent = DomNode::new(parent_id, WidgetMeta::new("Container"));

    // Create child node with parent reference
    let child_id = DomId::new(2);
    let mut child = DomNode::new(child_id, WidgetMeta::new("Button"));
    child.parent = Some(parent_id);

    // Closure to get parent node
    let get_node = |id: DomId| -> Option<&DomNode> {
        if id == parent_id {
            Some(&parent)
        } else {
            None
        }
    };

    let matches = resolver.match_node(&child, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_match_child_combinator() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "Container > Button".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let parent_id = DomId::new(1);
    let parent = DomNode::new(parent_id, WidgetMeta::new("Container"));

    let child_id = DomId::new(2);
    let mut child = DomNode::new(child_id, WidgetMeta::new("Button"));
    child.parent = Some(parent_id);

    let get_node = |id: DomId| -> Option<&DomNode> {
        if id == parent_id {
            Some(&parent)
        } else {
            None
        }
    };

    let matches = resolver.match_node(&child, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_match_adjacent_sibling_combinator() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "Label + Input".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let parent_id = DomId::new(1);
    let sibling_id = DomId::new(2);
    let target_id = DomId::new(3);

    let mut parent = DomNode::new(parent_id, WidgetMeta::new("Form"));
    parent.children = vec![sibling_id, target_id];

    let mut sibling = DomNode::new(sibling_id, WidgetMeta::new("Label"));
    sibling.parent = Some(parent_id);

    let mut target = DomNode::new(target_id, WidgetMeta::new("Input"));
    target.parent = Some(parent_id);

    let get_node = |id: DomId| -> Option<&DomNode> {
        match id.inner() {
            1 => Some(&parent),
            2 => Some(&sibling),
            _ => None,
        }
    };

    let matches = resolver.match_node(&target, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_match_no_parent() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "Container Button".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    // Node without parent
    let dom_id = DomId::new(1);
    let node = DomNode::new(dom_id, WidgetMeta::new("Button"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 0);
}
