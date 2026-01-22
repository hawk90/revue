//! CSS cascade tests

#[cfg(test)]
mod tests {
    use crate::dom::cascade::merge::StyleMerge;
    use crate::dom::cascade::resolver::{MatchedRule, StyleResolver};
    use crate::dom::cascade::specificity::Specificity;
    use crate::dom::node::{DomNode, WidgetMeta};
    use crate::dom::DomId;
    use crate::style::{Declaration, Rule, Style, StyleSheet};
    // ─────────────────────────────────────────────────────────────────────────────
    // Specificity Tests
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_specificity_ordering() {
        // Type < Class < ID
        let type_spec = Specificity::new(0, 0, 1, 0);
        let class_spec = Specificity::new(0, 1, 0, 0);
        let id_spec = Specificity::new(1, 0, 0, 0);

        assert!(type_spec < class_spec);
        assert!(class_spec < id_spec);
    }

    #[test]
    fn test_specificity_same_level() {
        // More of same type wins
        let one_class = Specificity::new(0, 1, 0, 0);
        let two_classes = Specificity::new(0, 2, 0, 0);

        assert!(one_class < two_classes);
    }

    #[test]
    fn test_specificity_order_tiebreak() {
        // Later declaration wins
        let first = Specificity::new(0, 1, 0, 0);
        let second = Specificity::new(0, 1, 0, 1);

        assert!(first < second);
    }

    #[test]
    fn test_specificity_inline() {
        let inline = Specificity::inline();
        let id = Specificity::new(1, 0, 0, 0);

        assert!(id < inline);
    }

    #[test]
    fn test_specificity_important() {
        let normal_id = Specificity::new(1, 0, 0, 0);
        let important_class = Specificity::new(0, 1, 0, 0).important();

        assert!(normal_id < important_class);
    }

    #[test]
    fn test_specificity_default() {
        let spec = Specificity::default();
        assert_eq!(spec.ids, 0);
        assert_eq!(spec.classes, 0);
        assert_eq!(spec.types, 0);
        assert!(!spec.inline);
        assert!(!spec.important);
    }

    #[test]
    fn test_specificity_partial_ord() {
        let a = Specificity::new(1, 0, 0, 0);
        let b = Specificity::new(0, 1, 0, 0);
        assert!(a.partial_cmp(&b) == Some(std::cmp::Ordering::Greater));
    }

    #[test]
    fn test_specificity_debug() {
        let spec = Specificity::new(1, 2, 3, 4);
        let debug = format!("{:?}", spec);
        assert!(debug.contains("Specificity"));
    }

    #[test]
    fn test_specificity_clone() {
        let spec = Specificity::new(1, 2, 3, 4);
        let cloned = spec;
        assert_eq!(spec.ids, cloned.ids);
        assert_eq!(spec.classes, cloned.classes);
        assert_eq!(spec.types, cloned.types);
    }

    #[test]
    fn test_specificity_eq() {
        let a = Specificity::new(1, 2, 3, 0);
        let b = Specificity::new(1, 2, 3, 0);
        assert_eq!(a, b);
    }

    #[test]
    fn test_specificity_important_over_inline() {
        let inline = Specificity::inline();
        let important = Specificity::new(0, 0, 0, 0).important();
        assert!(inline < important);
    }

    #[test]
    fn test_specificity_both_inline() {
        let inline1 = Specificity {
            inline: true,
            important: false,
            ids: 0,
            classes: 0,
            types: 0,
            order: 0,
        };
        let inline2 = Specificity {
            inline: true,
            important: false,
            ids: 0,
            classes: 0,
            types: 0,
            order: 1,
        };
        assert!(inline1 < inline2); // Order matters
    }

    #[test]
    fn test_specificity_both_important() {
        let important1 = Specificity::new(0, 1, 0, 0).important();
        let important2 = Specificity::new(1, 0, 0, 0).important();
        assert!(important1 < important2); // IDs still count
    }

    #[test]
    fn test_specificity_types_tiebreak() {
        let a = Specificity::new(0, 0, 1, 0);
        let b = Specificity::new(0, 0, 2, 0);
        assert!(a < b);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // StyleResolver Tests
    // ─────────────────────────────────────────────────────────────────────────────

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
        let resolver = StyleResolver::new(&stylesheet);

        let node = create_button_node(1);
        let get_node = |_: DomId| -> Option<&DomNode> { None };

        let matches = resolver.match_node(&node, get_node);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].rule.selector, "Button");
    }

    #[test]
    fn test_resolver_match_by_class() {
        let stylesheet = create_test_stylesheet();
        let resolver = StyleResolver::new(&stylesheet);

        let node = create_node_with_class(1, "Text", "primary");
        let get_node = |_: DomId| -> Option<&DomNode> { None };

        let matches = resolver.match_node(&node, get_node);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].rule.selector, ".primary");
    }

    #[test]
    fn test_resolver_match_by_id() {
        let stylesheet = create_test_stylesheet();
        let resolver = StyleResolver::new(&stylesheet);

        let node = create_node_with_id(1, "Button", "submit");
        let get_node = |_: DomId| -> Option<&DomNode> { None };

        let matches = resolver.match_node(&node, get_node);
        // Should match Button and #submit
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_resolver_match_multiple() {
        let stylesheet = create_test_stylesheet();
        let resolver = StyleResolver::new(&stylesheet);

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
        let resolver = StyleResolver::new(&stylesheet);

        let dom_id = DomId::new(1);
        let node = DomNode::new(dom_id, WidgetMeta::new("Text")); // No matching rules
        let get_node = |_: DomId| -> Option<&DomNode> { None };

        let matches = resolver.match_node(&node, get_node);
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_compute_style_basic() {
        let stylesheet = create_test_stylesheet();
        let resolver = StyleResolver::new(&stylesheet);

        let node = create_button_node(1);
        let get_node = |_: DomId| -> Option<&DomNode> { None };

        let style = resolver.compute_style(&node, get_node);
        // Verify style is computed (at minimum it's a default style)
        let _ = style; // Style was computed successfully
    }

    #[test]
    fn test_with_cached_selectors() {
        let stylesheet = create_test_stylesheet();
        let resolver1 = StyleResolver::new(&stylesheet);

        // Get cached selectors
        let cached = resolver1.selectors.clone();

        // Create new resolver with cached selectors
        let resolver2 = StyleResolver::with_cached_selectors(&stylesheet, cached);

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
        let resolver = StyleResolver::new(&stylesheet);

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
        let resolver = StyleResolver::new(&stylesheet);

        let dom_id = DomId::new(1);
        let mut node = DomNode::new(dom_id, WidgetMeta::new("Button"));
        // Set inline style
        let mut inline = Style::default();
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
        let resolver = StyleResolver::new(&stylesheet);

        let dom_id = DomId::new(1);
        let node = DomNode::new(dom_id, WidgetMeta::new("Text"));
        let get_node = |_: DomId| -> Option<&DomNode> { None };

        // Parent with color
        let mut parent_style = Style::default();
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
        let resolver = StyleResolver::new(&stylesheet);

        let dom_id = DomId::new(1);
        let node = DomNode::new(dom_id, WidgetMeta::new("Text"));
        let get_node = |_: DomId| -> Option<&DomNode> { None };

        let style = resolver.compute_style_with_parent(&node, None, get_node);
        // Should be default style
        assert!(style.visual.visible);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Pseudo-class Tests
    // ─────────────────────────────────────────────────────────────────────────────

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
        let resolver = StyleResolver::new(&stylesheet);

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
        let resolver = StyleResolver::new(&stylesheet);

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
        let resolver = StyleResolver::new(&stylesheet);

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
        let resolver = StyleResolver::new(&stylesheet);

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
        let resolver = StyleResolver::new(&stylesheet);

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
        let resolver = StyleResolver::new(&stylesheet);

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
        let resolver = StyleResolver::new(&stylesheet);

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
        let resolver = StyleResolver::new(&stylesheet);

        let dom_id = DomId::new(1);
        let mut node = DomNode::new(dom_id, WidgetMeta::new("Item"));
        node.state.update_position(0, 1); // Only child
        let get_node = |_: DomId| -> Option<&DomNode> { None };

        let matches = resolver.match_node(&node, &get_node);
        assert_eq!(matches.len(), 1);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Combinator Tests
    // ─────────────────────────────────────────────────────────────────────────────

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
        let node = create_button_node(1);
        let get_node = |_: DomId| -> Option<&DomNode> { None };

        let matches = resolver.match_node(&node, get_node);
        assert_eq!(matches.len(), 0);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Attribute Selector Tests
    // ─────────────────────────────────────────────────────────────────────────────

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

    // ─────────────────────────────────────────────────────────────────────────────
    // MatchedRule Tests
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_matched_rule_debug() {
        let rule = Rule {
            selector: "Button".to_string(),
            declarations: vec![],
        };
        let selector = crate::dom::parse_selector("Button").unwrap();
        let matched = MatchedRule {
            selector: &selector,
            rule: &rule,
            specificity: Specificity::new(0, 0, 1, 0),
        };

        // Should be debuggable
        let debug_str = format!("{:?}", matched);
        assert!(debug_str.contains("MatchedRule"));
    }

    #[test]
    fn test_matched_rule_clone() {
        let rule = Rule {
            selector: "Button".to_string(),
            declarations: vec![],
        };
        let selector = crate::dom::parse_selector("Button").unwrap();
        let matched = MatchedRule {
            selector: &selector,
            rule: &rule,
            specificity: Specificity::new(0, 0, 1, 0),
        };

        let cloned = matched.clone();
        assert_eq!(cloned.specificity.types, 1);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // StyleMerge Tests
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_style_merge_default() {
        let style1 = Style::default();
        let style2 = Style::default();
        let merged = style1.merge(&style2);
        assert!(merged.visual.visible);
    }

    #[test]
    fn test_style_merge_color() {
        use crate::style::Color;

        let mut style1 = Style::default();
        let mut style2 = Style::default();
        style2.visual.color = Color::hex(0xff0000);

        let merged = style1.merge(&style2);
        assert_eq!(merged.visual.color, Color::hex(0xff0000));

        // First style color is preserved if second is default
        style1.visual.color = Color::hex(0x00ff00);
        let style3 = Style::default();
        let merged2 = style1.merge(&style3);
        assert_eq!(merged2.visual.color, Color::hex(0x00ff00));
    }

    #[test]
    fn test_style_merge_background() {
        use crate::style::Color;

        let style1 = Style::default();
        let mut style2 = Style::default();
        style2.visual.background = Color::hex(0x0000ff);

        let merged = style1.merge(&style2);
        assert_eq!(merged.visual.background, Color::hex(0x0000ff));
    }

    #[test]
    fn test_style_merge_opacity() {
        let style1 = Style::default();
        let mut style2 = Style::default();
        style2.visual.opacity = 0.5;

        let merged = style1.merge(&style2);
        assert!((merged.visual.opacity - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_style_merge_visible() {
        let style1 = Style::default();
        let mut style2 = Style::default();
        style2.visual.visible = false;

        let merged = style1.merge(&style2);
        assert!(!merged.visual.visible);
    }

    #[test]
    fn test_style_merge_display() {
        use crate::style::Display;

        let style1 = Style::default();
        let mut style2 = Style::default();
        style2.layout.display = Display::Flex;

        let merged = style1.merge(&style2);
        assert_eq!(merged.layout.display, Display::Flex);
    }

    #[test]
    fn test_style_merge_gap() {
        let style1 = Style::default();
        let mut style2 = Style::default();
        style2.layout.gap = 10;

        let merged = style1.merge(&style2);
        assert_eq!(merged.layout.gap, 10);
    }

    #[test]
    fn test_style_merge_sizing() {
        use crate::style::Size;

        let style1 = Style::default();
        let mut style2 = Style::default();
        style2.sizing.width = Size::Fixed(100);
        style2.sizing.height = Size::Percent(50.0);

        let merged = style1.merge(&style2);
        assert_eq!(merged.sizing.width, Size::Fixed(100));
        assert_eq!(merged.sizing.height, Size::Percent(50.0));
    }

    #[test]
    fn test_style_merge_min_max_sizing() {
        use crate::style::Size;

        let style1 = Style::default();
        let mut style2 = Style::default();
        style2.sizing.min_width = Size::Fixed(50);
        style2.sizing.max_width = Size::Fixed(200);
        style2.sizing.min_height = Size::Fixed(30);
        style2.sizing.max_height = Size::Fixed(100);

        let merged = style1.merge(&style2);
        assert_eq!(merged.sizing.min_width, Size::Fixed(50));
        assert_eq!(merged.sizing.max_width, Size::Fixed(200));
        assert_eq!(merged.sizing.min_height, Size::Fixed(30));
        assert_eq!(merged.sizing.max_height, Size::Fixed(100));
    }

    #[test]
    fn test_style_merge_flex() {
        use crate::style::{AlignItems, FlexDirection, JustifyContent};

        let style1 = Style::default();
        let mut style2 = Style::default();
        style2.layout.flex_direction = FlexDirection::Column;
        style2.layout.justify_content = JustifyContent::Center;
        style2.layout.align_items = AlignItems::End;

        let merged = style1.merge(&style2);
        assert_eq!(merged.layout.flex_direction, FlexDirection::Column);
        assert_eq!(merged.layout.justify_content, JustifyContent::Center);
        assert_eq!(merged.layout.align_items, AlignItems::End);
    }

    #[test]
    fn test_style_merge_spacing() {
        use crate::style::Spacing;

        let style1 = Style::default();
        let mut style2 = Style::default();
        style2.spacing.margin = Spacing::all(10);
        style2.spacing.padding = Spacing::all(5);

        let merged = style1.merge(&style2);
        assert_eq!(merged.spacing.margin, Spacing::all(10));
        assert_eq!(merged.spacing.padding, Spacing::all(5));
    }

    #[test]
    fn test_style_merge_border() {
        use crate::style::{BorderStyle, Color};

        let style1 = Style::default();
        let mut style2 = Style::default();
        style2.visual.border_style = BorderStyle::Solid;
        style2.visual.border_color = Color::hex(0xffffff);

        let merged = style1.merge(&style2);
        assert_eq!(merged.visual.border_style, BorderStyle::Solid);
        assert_eq!(merged.visual.border_color, Color::hex(0xffffff));
    }
}
