//! CSS style resolver
//!
//! Matches CSS selectors to DOM nodes and computes the final styles.

use super::super::node::DomNode;
use super::super::selector::{Combinator, Selector};
use super::super::DomId;
use super::specificity::Specificity;
use crate::style::{apply_declaration, Rule, Style, StyleSheet};

// Import StyleMerge trait for merge() method
use super::merge::StyleMerge;

/// A matched CSS rule with its specificity
#[derive(Debug, Clone)]
pub struct MatchedRule<'a> {
    /// The selector that matched
    pub selector: &'a Selector,
    /// The rule containing declarations
    pub rule: &'a Rule,
    /// Computed specificity
    pub specificity: Specificity,
}

/// Style resolver - matches selectors and computes styles
pub struct StyleResolver<'a> {
    /// Parsed stylesheet
    stylesheet: &'a StyleSheet,
    /// Parsed selectors for each rule
    pub selectors: Vec<(Selector, usize)>,
}

impl<'a> StyleResolver<'a> {
    /// Create a new style resolver
    pub fn new(stylesheet: &'a StyleSheet) -> Self {
        // Pre-parse all selectors
        let mut selectors = Vec::new();
        for (idx, rule) in stylesheet.rules.iter().enumerate() {
            if let Ok(selector) = super::super::parse_selector(&rule.selector) {
                selectors.push((selector, idx));
            }
        }

        Self {
            stylesheet,
            selectors,
        }
    }

    /// Create a style resolver with pre-parsed selectors (avoids reparsing)
    pub fn with_cached_selectors(
        stylesheet: &'a StyleSheet,
        selectors: Vec<(Selector, usize)>,
    ) -> Self {
        Self {
            stylesheet,
            selectors,
        }
    }

    /// Find all rules matching a node
    pub fn match_node<F>(&self, node: &DomNode, get_node: F) -> Vec<MatchedRule<'_>>
    where
        F: Fn(DomId) -> Option<&'a DomNode>,
    {
        // Pre-allocate with reasonable capacity (most nodes match < 8 rules)
        let mut matched = Vec::with_capacity(4);

        for (selector, rule_idx) in &self.selectors {
            if self.matches(selector, node, &get_node) {
                let (a, b, c) = selector.specificity();
                let specificity = Specificity::new(a, b, c, *rule_idx);
                matched.push(MatchedRule {
                    selector,
                    rule: &self.stylesheet.rules[*rule_idx],
                    specificity,
                });
            }
        }

        // Sort by specificity (ascending)
        matched.sort_by(|a, b| a.specificity.cmp(&b.specificity));

        matched
    }

    /// Compute final style for a node (without inheritance)
    pub fn compute_style<F>(&self, node: &DomNode, get_node: F) -> Style
    where
        F: Fn(DomId) -> Option<&'a DomNode>,
    {
        let matched = self.match_node(node, &get_node);
        let mut style = Style::default();

        // Apply matched rules in specificity order
        for rule in &matched {
            self.apply_rule(&mut style, rule.rule);
        }

        // Apply inline style last (highest priority)
        if let Some(ref inline) = node.inline_style {
            style = style.merge(inline);
        }

        style
    }

    /// Compute final style for a node with inheritance from parent
    ///
    /// CSS inheritance works as follows:
    /// 1. Start with inherited values from parent (color, opacity, visible)
    /// 2. Apply matching CSS rules in specificity order
    /// 3. Apply inline styles last
    ///
    /// Non-inherited properties (display, padding, margin, etc.) reset to defaults.
    pub fn compute_style_with_parent<F>(
        &self,
        node: &DomNode,
        parent_style: Option<&Style>,
        get_node: F,
    ) -> Style
    where
        F: Fn(DomId) -> Option<&'a DomNode>,
    {
        // Start with base style (inherit from parent if available)
        let base_style = match parent_style {
            Some(parent) => Style::inherit(parent),
            None => Style::default(),
        };

        let matched = self.match_node(node, &get_node);
        let mut style = base_style;

        // Apply matched rules in specificity order
        for rule in &matched {
            self.apply_rule(&mut style, rule.rule);
        }

        // Apply inline style last (highest priority)
        if let Some(ref inline) = node.inline_style {
            style = style.merge(inline);
        }

        style
    }

    /// Apply a rule's declarations to a style
    fn apply_rule(&self, style: &mut Style, rule: &Rule) {
        for decl in &rule.declarations {
            apply_declaration(
                style,
                &decl.property,
                &decl.value,
                &self.stylesheet.variables,
            );
        }
    }

    /// Check if a selector matches a node
    fn matches<F>(&self, selector: &Selector, node: &DomNode, get_node: &F) -> bool
    where
        F: Fn(DomId) -> Option<&'a DomNode>,
    {
        if selector.parts.is_empty() {
            return false;
        }

        // Match from right to left (target first)
        let mut current_node = Some(node);
        let mut part_idx = selector.parts.len();

        while part_idx > 0 {
            part_idx -= 1;
            let (part, _combinator) = &selector.parts[part_idx];

            let node = match current_node {
                Some(n) => n,
                None => return false,
            };

            // Check if this part matches current node
            if !self.matches_part(part, node) {
                // For descendant combinator, try ancestors
                if part_idx < selector.parts.len() - 1 {
                    match selector.parts.get(part_idx + 1) {
                        Some((_, Some(Combinator::Descendant))) => {
                            // Try parent
                            if let Some(parent_id) = node.parent {
                                if let Some(parent) = get_node(parent_id) {
                                    current_node = Some(parent);
                                    part_idx += 1; // Retry this part with parent
                                    continue;
                                }
                            }
                        }
                        Some((_, Some(Combinator::GeneralSibling))) => {
                            // Try previous sibling (general sibling matches any previous)
                            if let Some(sibling) = self.get_previous_sibling(node, get_node) {
                                current_node = Some(sibling);
                                part_idx += 1; // Retry this part with previous sibling
                                continue;
                            }
                        }
                        _ => {}
                    }
                }
                return false;
            }

            // Move to next node based on combinator
            if part_idx > 0 {
                let prev_combinator = selector.parts[part_idx - 1].1;
                current_node = match prev_combinator {
                    Some(Combinator::Descendant) => {
                        // Any ancestor - will be handled in next iteration
                        node.parent.and_then(get_node)
                    }
                    Some(Combinator::Child) => {
                        // Direct parent only
                        node.parent.and_then(get_node)
                    }
                    Some(Combinator::AdjacentSibling) => {
                        // Previous sibling
                        self.get_previous_sibling(node, get_node)
                    }
                    Some(Combinator::GeneralSibling) => {
                        // Any previous sibling
                        self.get_previous_sibling(node, get_node)
                    }
                    None => None,
                };
            }
        }

        true
    }

    /// Check if a selector part matches a node
    fn matches_part(&self, part: &crate::dom::selector::SelectorPart, node: &DomNode) -> bool {
        // Universal selector matches everything
        if part.universal
            && part.id.is_none()
            && part.classes.is_empty()
            && part.pseudo_classes.is_empty()
            && part.element.is_none()
        {
            return true;
        }

        // Check element type
        if let Some(ref elem) = part.element {
            if node.widget_type() != elem {
                return false;
            }
        }

        // Check ID
        if let Some(ref id) = part.id {
            match node.element_id() {
                Some(node_id) if node_id == id => {}
                _ => return false,
            }
        }

        // Check classes
        for class in &part.classes {
            if !node.has_class(class) {
                return false;
            }
        }

        // Check pseudo-classes
        for pseudo in &part.pseudo_classes {
            if !node.matches_pseudo(pseudo) {
                return false;
            }
        }

        // Check attributes
        for attr in &part.attributes {
            if !self.matches_attribute(attr, node) {
                return false;
            }
        }

        true
    }

    /// Check if an attribute selector matches
    ///
    /// Supports the following attribute selectors:
    /// - `[class]` - has any class
    /// - `[class~=value]` - contains word (class)
    /// - `[id]` - has id
    /// - `[id=value]` - exact id match
    /// - `[type]` / `[type=value]` - widget type matching
    /// - `[disabled]` - disabled state
    /// - `[checked]` - checked state
    /// - `[selected]` - selected state
    fn matches_attribute(
        &self,
        attr: &crate::dom::selector::AttributeSelector,
        node: &DomNode,
    ) -> bool {
        use crate::dom::selector::AttributeOp;

        // Helper for case-insensitive comparison
        let compare = |a: &str, b: &str, case_insensitive: bool| -> bool {
            if case_insensitive {
                a.eq_ignore_ascii_case(b)
            } else {
                a == b
            }
        };

        match attr.name.as_str() {
            "class" => {
                match &attr.op {
                    AttributeOp::Exists => !node.meta.classes.is_empty(),
                    AttributeOp::ContainsWord => {
                        if let Some(ref val) = attr.value {
                            node.meta
                                .classes
                                .iter()
                                .any(|c| compare(c, val, attr.case_insensitive))
                        } else {
                            false
                        }
                    }
                    AttributeOp::Equals => {
                        // Exact match: classes joined with space equals value
                        if let Some(ref val) = attr.value {
                            let classes: Vec<_> = node.meta.classes.iter().collect();
                            let joined = classes
                                .iter()
                                .map(|s| s.as_str())
                                .collect::<Vec<_>>()
                                .join(" ");
                            compare(&joined, val, attr.case_insensitive)
                        } else {
                            false
                        }
                    }
                    AttributeOp::Contains => {
                        if let Some(ref val) = attr.value {
                            node.meta.classes.iter().any(|c| {
                                if attr.case_insensitive {
                                    c.to_lowercase().contains(&val.to_lowercase())
                                } else {
                                    c.contains(val.as_str())
                                }
                            })
                        } else {
                            false
                        }
                    }
                    AttributeOp::StartsWith => {
                        if let Some(ref val) = attr.value {
                            node.meta.classes.iter().any(|c| {
                                if attr.case_insensitive {
                                    c.to_lowercase().starts_with(&val.to_lowercase())
                                } else {
                                    c.starts_with(val.as_str())
                                }
                            })
                        } else {
                            false
                        }
                    }
                    AttributeOp::EndsWith => {
                        if let Some(ref val) = attr.value {
                            node.meta.classes.iter().any(|c| {
                                if attr.case_insensitive {
                                    c.to_lowercase().ends_with(&val.to_lowercase())
                                } else {
                                    c.ends_with(val.as_str())
                                }
                            })
                        } else {
                            false
                        }
                    }
                    AttributeOp::StartsWithWord => {
                        // CSS |= operator: exact match or starts with value followed by hyphen
                        if let Some(ref val) = attr.value {
                            node.meta.classes.iter().any(|c| {
                                compare(c, val, attr.case_insensitive) || {
                                    let prefix = format!("{}-", val);
                                    if attr.case_insensitive {
                                        c.to_lowercase().starts_with(&prefix.to_lowercase())
                                    } else {
                                        c.starts_with(&prefix)
                                    }
                                }
                            })
                        } else {
                            false
                        }
                    }
                }
            }
            "id" => {
                let node_id = node.element_id().unwrap_or("");
                match &attr.op {
                    AttributeOp::Exists => node.element_id().is_some(),
                    AttributeOp::Equals => {
                        if let Some(ref val) = attr.value {
                            compare(node_id, val, attr.case_insensitive)
                        } else {
                            false
                        }
                    }
                    AttributeOp::StartsWith => {
                        if let Some(ref val) = attr.value {
                            if attr.case_insensitive {
                                node_id.to_lowercase().starts_with(&val.to_lowercase())
                            } else {
                                node_id.starts_with(val.as_str())
                            }
                        } else {
                            false
                        }
                    }
                    AttributeOp::EndsWith => {
                        if let Some(ref val) = attr.value {
                            if attr.case_insensitive {
                                node_id.to_lowercase().ends_with(&val.to_lowercase())
                            } else {
                                node_id.ends_with(val.as_str())
                            }
                        } else {
                            false
                        }
                    }
                    AttributeOp::Contains => {
                        if let Some(ref val) = attr.value {
                            if attr.case_insensitive {
                                node_id.to_lowercase().contains(&val.to_lowercase())
                            } else {
                                node_id.contains(val.as_str())
                            }
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            }
            "type" => {
                // Widget type matching
                let widget_type = node.widget_type();
                match &attr.op {
                    AttributeOp::Exists => !widget_type.is_empty(),
                    AttributeOp::Equals => {
                        if let Some(ref val) = attr.value {
                            compare(widget_type, val, attr.case_insensitive)
                        } else {
                            false
                        }
                    }
                    AttributeOp::Contains => {
                        if let Some(ref val) = attr.value {
                            if attr.case_insensitive {
                                widget_type.to_lowercase().contains(&val.to_lowercase())
                            } else {
                                widget_type.contains(val.as_str())
                            }
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            }
            "disabled" => match &attr.op {
                AttributeOp::Exists => node.state.disabled,
                AttributeOp::Equals => {
                    if let Some(ref val) = attr.value {
                        let is_true = val == "true" || val == "1" || val.is_empty();
                        node.state.disabled == is_true
                    } else {
                        node.state.disabled
                    }
                }
                _ => false,
            },
            "checked" => match &attr.op {
                AttributeOp::Exists => node.state.checked,
                AttributeOp::Equals => {
                    if let Some(ref val) = attr.value {
                        let is_true = val == "true" || val == "1" || val.is_empty();
                        node.state.checked == is_true
                    } else {
                        node.state.checked
                    }
                }
                _ => false,
            },
            "selected" => match &attr.op {
                AttributeOp::Exists => node.state.selected,
                AttributeOp::Equals => {
                    if let Some(ref val) = attr.value {
                        let is_true = val == "true" || val == "1" || val.is_empty();
                        node.state.selected == is_true
                    } else {
                        node.state.selected
                    }
                }
                _ => false,
            },
            "focused" | "focus" => match &attr.op {
                AttributeOp::Exists => node.state.focused,
                _ => false,
            },
            "hovered" | "hover" => match &attr.op {
                AttributeOp::Exists => node.state.hovered,
                _ => false,
            },
            _ => false,
        }
    }

    /// Get previous sibling of a node
    fn get_previous_sibling<F>(&self, node: &DomNode, get_node: F) -> Option<&'a DomNode>
    where
        F: Fn(DomId) -> Option<&'a DomNode>,
    {
        let parent_id = node.parent?;
        let parent = get_node(parent_id)?;

        let idx = parent.children.iter().position(|&id| id == node.id)?;
        if idx > 0 {
            get_node(parent.children[idx - 1])
        } else {
            None
        }
    }
}
