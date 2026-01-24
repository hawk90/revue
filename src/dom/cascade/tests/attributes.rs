//! Attribute selector tests

use crate::dom::cascade::resolver::StyleResolver;
use crate::dom::node::{DomNode, WidgetMeta};
use crate::dom::DomId;
use crate::style::{Rule, StyleSheet};

#[test]
fn test_attribute_class_exists() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[class]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let node = DomNode::new(dom_id, WidgetMeta::new("Button").class("primary"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);

    // Without class
    let node2 = DomNode::new(DomId::new(2), WidgetMeta::new("Button"));
    let matches2 = resolver.match_node(&node2, get_node);
    assert_eq!(matches2.len(), 0);
}

#[test]
fn test_attribute_class_contains_word() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[class~=primary]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let node = DomNode::new(
        dom_id,
        WidgetMeta::new("Button").class("primary").class("large"),
    );
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_id_exists() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[id]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let node = DomNode::new(dom_id, WidgetMeta::new("Button").id("submit"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_id_equals() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[id=submit]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let node = DomNode::new(dom_id, WidgetMeta::new("Button").id("submit"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_id_starts_with() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[id^=btn]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let node = DomNode::new(dom_id, WidgetMeta::new("Button").id("btn-submit"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_id_ends_with() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[id$=submit]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let node = DomNode::new(dom_id, WidgetMeta::new("Button").id("btn-submit"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_id_contains() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[id*=sub]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let node = DomNode::new(dom_id, WidgetMeta::new("Button").id("btn-submit-form"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_type_exists() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[type]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let node = DomNode::new(dom_id, WidgetMeta::new("Button"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_type_equals() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[type=Button]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let node = DomNode::new(dom_id, WidgetMeta::new("Button"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_type_contains() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[type*=utt]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let node = DomNode::new(dom_id, WidgetMeta::new("Button"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_disabled_exists() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[disabled]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Button"));
    node.state.disabled = true;
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_disabled_equals() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[disabled=true]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Button"));
    node.state.disabled = true;
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_checked_exists() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[checked]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Checkbox"));
    node.state.checked = true;
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_selected_exists() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[selected]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Option"));
    node.state.selected = true;
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_focused_exists() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[focused]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Input"));
    node.state.focused = true;
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_hovered_exists() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[hovered]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let mut node = DomNode::new(dom_id, WidgetMeta::new("Button"));
    node.state.hovered = true;
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_attribute_unknown() {
    let stylesheet = StyleSheet {
        rules: vec![Rule {
            selector: "[unknown-attr]".to_string(),
            declarations: vec![],
        }],
        variables: std::collections::HashMap::new(),
    };
    let resolver = StyleResolver::new(&stylesheet);

    let dom_id = DomId::new(1);
    let node = DomNode::new(dom_id, WidgetMeta::new("Button"));
    let get_node = |_: DomId| -> Option<&DomNode> { None };

    let matches = resolver.match_node(&node, get_node);
    assert_eq!(matches.len(), 0);
}
