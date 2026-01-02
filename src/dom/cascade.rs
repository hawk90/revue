//! CSS style cascade and specificity
//!
//! Implements CSS cascade algorithm:
//! 1. Find all matching rules
//! 2. Sort by specificity
//! 3. Apply in order (lowest to highest specificity)
//! 4. Apply inline styles last

use std::cmp::Ordering;

use super::node::DomNode;
use super::selector::{Selector, SelectorPart, Combinator};
use super::DomId;
use crate::style::{Style, StyleSheet, Rule, apply_declaration};

/// CSS specificity (a, b, c)
/// - a: ID selectors count
/// - b: class, attribute, pseudo-class count
/// - c: type, pseudo-element count
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Specificity {
    /// Inline style flag (highest priority)
    pub inline: bool,
    /// !important flag
    pub important: bool,
    /// ID count
    pub ids: usize,
    /// Class/attribute/pseudo-class count
    pub classes: usize,
    /// Type/pseudo-element count
    pub types: usize,
    /// Source order (later = higher priority for equal specificity)
    pub order: usize,
}

impl Specificity {
    /// Create from specificity tuple
    pub fn new(ids: usize, classes: usize, types: usize, order: usize) -> Self {
        Self {
            inline: false,
            important: false,
            ids,
            classes,
            types,
            order,
        }
    }

    /// Create inline style specificity
    pub fn inline() -> Self {
        Self {
            inline: true,
            ..Default::default()
        }
    }

    /// Mark as !important
    pub fn important(mut self) -> Self {
        self.important = true;
        self
    }
}

impl Ord for Specificity {
    fn cmp(&self, other: &Self) -> Ordering {
        // 1. !important wins
        match (self.important, other.important) {
            (true, false) => return Ordering::Greater,
            (false, true) => return Ordering::Less,
            _ => {}
        }

        // 2. Inline styles win
        match (self.inline, other.inline) {
            (true, false) => return Ordering::Greater,
            (false, true) => return Ordering::Less,
            _ => {}
        }

        // 3. Compare (a, b, c)
        match self.ids.cmp(&other.ids) {
            Ordering::Equal => {}
            ord => return ord,
        }

        match self.classes.cmp(&other.classes) {
            Ordering::Equal => {}
            ord => return ord,
        }

        match self.types.cmp(&other.types) {
            Ordering::Equal => {}
            ord => return ord,
        }

        // 4. Source order (later wins)
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for Specificity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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
    selectors: Vec<(Selector, usize)>,
}

impl<'a> StyleResolver<'a> {
    /// Create a new style resolver
    pub fn new(stylesheet: &'a StyleSheet) -> Self {
        // Pre-parse all selectors
        let mut selectors = Vec::new();
        for (idx, rule) in stylesheet.rules.iter().enumerate() {
            if let Ok(selector) = super::parse_selector(&rule.selector) {
                selectors.push((selector, idx));
            }
        }

        Self {
            stylesheet,
            selectors,
        }
    }

    /// Create a style resolver with pre-parsed selectors (avoids reparsing)
    pub fn with_cached_selectors(stylesheet: &'a StyleSheet, selectors: Vec<(Selector, usize)>) -> Self {
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
        let mut matched = Vec::new();

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
                    if let Some((_, Some(Combinator::Descendant))) = selector.parts.get(part_idx + 1) {
                        // Try parent
                        if let Some(parent_id) = node.parent {
                            if let Some(parent) = get_node(parent_id) {
                                current_node = Some(parent);
                                part_idx += 1; // Retry this part with parent
                                continue;
                            }
                        }
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
    fn matches_part(&self, part: &SelectorPart, node: &DomNode) -> bool {
        // Universal selector matches everything
        if part.universal && part.id.is_none() && part.classes.is_empty()
            && part.pseudo_classes.is_empty() && part.element.is_none()
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
    fn matches_attribute(&self, attr: &super::selector::AttributeSelector, node: &DomNode) -> bool {
        use super::selector::AttributeOp;

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
                            node.meta.classes.iter().any(|c| compare(c, val, attr.case_insensitive))
                        } else {
                            false
                        }
                    }
                    AttributeOp::Equals => {
                        // Exact match: classes joined with space equals value
                        if let Some(ref val) = attr.value {
                            let classes: Vec<_> = node.meta.classes.iter().collect();
                            let joined = classes.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
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
            "disabled" => {
                match &attr.op {
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
                }
            }
            "checked" => {
                match &attr.op {
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
                }
            }
            "selected" => {
                match &attr.op {
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
                }
            }
            "focused" | "focus" => {
                match &attr.op {
                    AttributeOp::Exists => node.state.focused,
                    _ => false,
                }
            }
            "hovered" | "hover" => {
                match &attr.op {
                    AttributeOp::Exists => node.state.hovered,
                    _ => false,
                }
            }
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

/// Trait for styles that can be merged
pub trait StyleMerge {
    /// Merge another style into this one (other values override)
    fn merge(&self, other: &Self) -> Self;
}

impl StyleMerge for Style {
    fn merge(&self, other: &Self) -> Self {
        use crate::style::{
            Display, FlexDirection, JustifyContent, AlignItems,
            Size, Spacing, BorderStyle, Color,
        };

        let mut result = self.clone();

        // Merge layout (display)
        if other.layout.display != Display::default() {
            result.layout.display = other.layout.display;
        }

        // Merge flex properties
        if other.layout.flex_direction != FlexDirection::default() {
            result.layout.flex_direction = other.layout.flex_direction;
        }
        if other.layout.justify_content != JustifyContent::default() {
            result.layout.justify_content = other.layout.justify_content;
        }
        if other.layout.align_items != AlignItems::default() {
            result.layout.align_items = other.layout.align_items;
        }
        if other.layout.gap != 0 {
            result.layout.gap = other.layout.gap;
        }

        // Merge size
        if other.sizing.width != Size::default() {
            result.sizing.width = other.sizing.width;
        }
        if other.sizing.height != Size::default() {
            result.sizing.height = other.sizing.height;
        }
        if other.sizing.min_width != Size::default() {
            result.sizing.min_width = other.sizing.min_width;
        }
        if other.sizing.min_height != Size::default() {
            result.sizing.min_height = other.sizing.min_height;
        }
        if other.sizing.max_width != Size::default() {
            result.sizing.max_width = other.sizing.max_width;
        }
        if other.sizing.max_height != Size::default() {
            result.sizing.max_height = other.sizing.max_height;
        }

        // Merge spacing
        if other.spacing.margin != Spacing::default() {
            result.spacing.margin = other.spacing.margin;
        }
        if other.spacing.padding != Spacing::default() {
            result.spacing.padding = other.spacing.padding;
        }

        // Merge colors (non-black means it was set)
        if other.visual.color != Color::default() {
            result.visual.color = other.visual.color;
        }
        if other.visual.background != Color::default() {
            result.visual.background = other.visual.background;
        }

        // Merge border
        if other.visual.border_style != BorderStyle::default() {
            result.visual.border_style = other.visual.border_style;
        }
        if other.visual.border_color != Color::default() {
            result.visual.border_color = other.visual.border_color;
        }

        // Merge visual
        if other.visual.opacity != 1.0 {
            result.visual.opacity = other.visual.opacity;
        }
        if !other.visual.visible {
            result.visual.visible = other.visual.visible;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::node::{DomNode, WidgetMeta};
    use crate::style::{StyleSheet, Rule, Declaration};

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

    // ─────────────────────────────────────────────────────────────────────────────
    // StyleResolver Tests
    // ─────────────────────────────────────────────────────────────────────────────

    fn create_test_stylesheet() -> StyleSheet {
        StyleSheet {
            rules: vec![
                Rule {
                    selector: "Button".to_string(),
                    declarations: vec![
                        Declaration {
                            property: "padding".to_string(),
                            value: "1".to_string(),
                        },
                    ],
                },
                Rule {
                    selector: ".primary".to_string(),
                    declarations: vec![
                        Declaration {
                            property: "background".to_string(),
                            value: "blue".to_string(),
                        },
                    ],
                },
                Rule {
                    selector: "#submit".to_string(),
                    declarations: vec![
                        Declaration {
                            property: "width".to_string(),
                            value: "100".to_string(),
                        },
                    ],
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
            WidgetMeta::new("Button")
                .class("primary")
                .id("submit")
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

    // ─────────────────────────────────────────────────────────────────────────────
    // Pseudo-class Tests
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_match_pseudo_hover() {
        let stylesheet = StyleSheet {
            rules: vec![
                Rule {
                    selector: "Button:hover".to_string(),
                    declarations: vec![
                        Declaration {
                            property: "background".to_string(),
                            value: "red".to_string(),
                        },
                    ],
                },
            ],
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
            rules: vec![
                Rule {
                    selector: "Input:focus".to_string(),
                    declarations: vec![],
                },
            ],
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
            rules: vec![
                Rule {
                    selector: "Button:disabled".to_string(),
                    declarations: vec![],
                },
            ],
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
            rules: vec![
                Rule {
                    selector: "Item:first-child".to_string(),
                    declarations: vec![],
                },
            ],
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
            rules: vec![
                Rule {
                    selector: "Item:last-child".to_string(),
                    declarations: vec![],
                },
            ],
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

    // ─────────────────────────────────────────────────────────────────────────────
    // Combinator Tests
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_match_descendant_combinator() {
        let stylesheet = StyleSheet {
            rules: vec![
                Rule {
                    selector: "Container Button".to_string(),
                    declarations: vec![],
                },
            ],
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
            rules: vec![
                Rule {
                    selector: "Container > Button".to_string(),
                    declarations: vec![],
                },
            ],
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

    // ─────────────────────────────────────────────────────────────────────────────
    // MatchedRule Tests
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_matched_rule_debug() {
        let rule = Rule {
            selector: "Button".to_string(),
            declarations: vec![],
        };
        let selector = super::super::parse_selector("Button").unwrap();
        let matched = MatchedRule {
            selector: &selector,
            rule: &rule,
            specificity: Specificity::new(0, 0, 1, 0),
        };

        // Should be debuggable
        let debug_str = format!("{:?}", matched);
        assert!(debug_str.contains("MatchedRule"));
    }
}
