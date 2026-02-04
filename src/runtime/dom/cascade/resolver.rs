//! CSS style resolver
//!
//! Matches CSS selectors to DOM nodes and computes the final styles.

use super::super::node::DomNode;
use super::super::selector::{Combinator, PseudoClass, Selector};
use super::super::DomId;
use super::specificity::Specificity;
use crate::style::{apply_declaration, Rule, Style, StyleSheet};
use std::collections::{HashMap, HashSet, VecDeque};

// Import StyleMerge trait for merge() method
use super::merge::StyleMerge;

/// Check if a selector contains state-dependent or position-dependent pseudo-classes
/// These pseudo-classes depend on runtime state or DOM position and shouldn't be cached
fn has_dynamic_pseudo_class(selector: &Selector) -> bool {
    for (part, _) in &selector.parts {
        for pseudo in &part.pseudo_classes {
            match pseudo {
                // Dynamic pseudo-classes that depend on node state
                PseudoClass::Focus
                | PseudoClass::Hover
                | PseudoClass::Active
                | PseudoClass::Disabled
                | PseudoClass::Enabled
                | PseudoClass::Checked
                | PseudoClass::Selected => return true,
                // Position-dependent pseudo-classes (structural but can change without DomId change)
                PseudoClass::FirstChild
                | PseudoClass::LastChild
                | PseudoClass::OnlyChild
                | PseudoClass::NthChild(_)
                | PseudoClass::NthLastChild(_) => return true,
                // Empty and Not are relatively stable (can be cached)
                PseudoClass::Empty | PseudoClass::Not(_) => continue,
            }
        }
    }
    false
}

/// Cache entry for matched rules
/// Stores selector indices (positions in the selectors Vec) for O(1) lookup
#[derive(Debug, Clone)]
struct CachedMatchedRules {
    /// Selector indices that matched this node (O(1) lookup into self.selectors)
    selector_indices: Vec<usize>,
}

/// LRU cache for matched rules
/// Bounded size to prevent unbounded memory growth
struct MatchCache {
    /// Map from DomId to cached rules
    cache: HashMap<DomId, CachedMatchedRules>,
    /// LRU ordering (front = least recently used, back = most recently used)
    lru_order: VecDeque<DomId>,
    /// Maximum cache size
    max_size: usize,
}

impl MatchCache {
    /// Create a new cache with the given max size
    fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            lru_order: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    /// Get cached rules for a node
    fn get(&mut self, id: DomId) -> Option<&CachedMatchedRules> {
        if self.cache.contains_key(&id) {
            // Move to end (most recently used)
            if let Some(pos) = self.lru_order.iter().position(|&x| x == id) {
                self.lru_order.remove(pos);
                self.lru_order.push_back(id);
            }
            self.cache.get(&id)
        } else {
            None
        }
    }

    /// Insert rules into the cache
    fn insert(&mut self, id: DomId, rules: CachedMatchedRules) {
        // Evict if at capacity
        if self.lru_order.len() >= self.max_size {
            if let Some(evict_id) = self.lru_order.pop_front() {
                self.cache.remove(&evict_id);
            }
        }

        // Add new entry
        self.lru_order.push_back(id);
        self.cache.insert(id, rules);
    }

    /// Clear the cache (call when DOM changes)
    fn clear(&mut self) {
        self.cache.clear();
        self.lru_order.clear();
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

/// Index for fast selector lookup by key (element, class, id, or universal)
///
/// This reduces the number of selector comparisons from O(n*m) to O(n*k)
/// where n = nodes, m = total selectors, k = relevant selectors per node (usually << m).
struct SelectorIndex {
    /// Map from element name to selector indices
    by_element: HashMap<String, Vec<usize>>,
    /// Map from class name to selector indices
    by_class: HashMap<String, Vec<usize>>,
    /// Map from id to selector indices
    by_id: HashMap<String, Vec<usize>>,
    /// Universal selectors that match everything
    universal: Vec<usize>,
}

impl SelectorIndex {
    /// Build an index from selectors
    fn build(selectors: &[(Selector, usize)]) -> Self {
        let mut by_element: HashMap<String, Vec<usize>> = HashMap::new();
        let mut by_class: HashMap<String, Vec<usize>> = HashMap::new();
        let mut by_id: HashMap<String, Vec<usize>> = HashMap::new();
        let mut universal: Vec<usize> = Vec::new();

        for (idx, (selector, _rule_idx)) in selectors.iter().enumerate() {
            // Get the target (rightmost) part of the selector
            let target = match selector.target() {
                Some(t) => t,
                None => continue,
            };

            // Determine the primary key for this selector
            // Priority: id > element > class > universal
            if let Some(ref id) = target.id {
                by_id.entry(id.clone()).or_default().push(idx);
            } else if let Some(ref element) = target.element {
                by_element.entry(element.clone()).or_default().push(idx);
            } else if !target.classes.is_empty() {
                // Index by first class for simplicity
                if let Some(first_class) = target.classes.first() {
                    by_class.entry(first_class.clone()).or_default().push(idx);
                }
            } else {
                // Universal selector or pseudo-class only
                universal.push(idx);
            }
        }

        Self {
            by_element,
            by_class,
            by_id,
            universal,
        }
    }

    /// Get candidate selector indices for a node
    fn get_candidates(&self, node: &DomNode) -> Vec<usize> {
        let mut candidates = Vec::new();
        let mut seen = HashSet::new();

        // Always include universal selectors
        for &idx in &self.universal {
            if seen.insert(idx) {
                candidates.push(idx);
            }
        }

        // Check by id (highest priority)
        if let Some(id) = node.element_id() {
            if let Some(indices) = self.by_id.get(id) {
                for &idx in indices {
                    if seen.insert(idx) {
                        candidates.push(idx);
                    }
                }
            }
        }

        // Check by element
        let element = node.widget_type();
        if let Some(indices) = self.by_element.get(element) {
            for &idx in indices {
                if seen.insert(idx) {
                    candidates.push(idx);
                }
            }
        }

        // Check by class (add all matching classes)
        for class in &node.meta.classes {
            if let Some(indices) = self.by_class.get(class) {
                for &idx in indices {
                    if seen.insert(idx) {
                        candidates.push(idx);
                    }
                }
            }
        }

        candidates
    }
}

/// Style resolver - matches selectors and computes styles
pub struct StyleResolver<'a> {
    /// Parsed stylesheet
    stylesheet: &'a StyleSheet,
    /// Parsed selectors for each rule
    pub selectors: Vec<(Selector, usize)>,
    /// Index for fast selector lookup
    index: SelectorIndex,
    /// Cache for matched rules (reduces selector matching overhead)
    match_cache: MatchCache,
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

        // Build the index for fast lookup
        let index = SelectorIndex::build(&selectors);

        // Create match cache with reasonable size (tunable based on typical DOM size)
        let match_cache = MatchCache::new(512);

        Self {
            stylesheet,
            selectors,
            index,
            match_cache,
        }
    }

    /// Create a style resolver with pre-parsed selectors (avoids reparsing)
    ///
    /// Takes a slice reference to avoid unnecessary cloning of the selector cache.
    /// The selectors are copied into the resolver's internal Vec for storage.
    pub fn with_cached_selectors(
        stylesheet: &'a StyleSheet,
        selectors: &[(Selector, usize)],
    ) -> Self {
        let selectors = selectors.to_vec();
        let index = SelectorIndex::build(&selectors);
        let match_cache = MatchCache::new(512);
        Self {
            stylesheet,
            selectors,
            index,
            match_cache,
        }
    }

    /// Clear the match cache
    ///
    /// Call this when the DOM structure changes (nodes added/removed) or when
    /// classes/attributes are modified that might affect selector matching.
    pub fn clear_cache(&mut self) {
        self.match_cache.clear();
    }

    /// Find all rules matching a node
    ///
    /// Uses the selector index to only check relevant selectors, reducing
    /// comparisons from O(all selectors) to O(relevant selectors).
    ///
    /// Results are cached by DomId for subsequent calls, reducing selector
    /// matching overhead for frequently-styled nodes.
    ///
    /// Note: Matches involving state-dependent pseudo-classes (:hover, :focus, etc.)
    /// are not cached since their results can change without the DomId changing.
    pub fn match_node<F>(&mut self, node: &DomNode, get_node: F) -> Vec<MatchedRule<'_>>
    where
        F: Fn(DomId) -> Option<&'a DomNode>,
    {
        let node_id = node.id;

        // Check cache first (fast path)
        // Only use cache if we have an entry for this node
        if let Some(cached) = self.match_cache.get(node_id) {
            // Reconstruct MatchedRule structs from cached selector indices
            // O(k) where k = number of matched rules (usually small)
            return cached
                .selector_indices
                .iter()
                .map(|&idx| {
                    let (selector, rule_idx) = &self.selectors[idx];
                    let (a, b, c) = selector.specificity();
                    MatchedRule {
                        selector,
                        rule: &self.stylesheet.rules[*rule_idx],
                        specificity: Specificity::new(a, b, c, *rule_idx),
                    }
                })
                .collect();
        }

        // Cache miss - compute matched rules
        // Get candidate selectors using the index (fast path)
        let candidates = self.index.get_candidates(node);

        // Pre-allocate with reasonable capacity (most nodes match < 8 rules)
        let mut matched = Vec::with_capacity(4);
        let mut has_dynamic_pseudo = false;

        // Only check candidate selectors (not all selectors)
        for &idx in &candidates {
            let (selector, rule_idx) = &self.selectors[idx];
            if self.matches(selector, node, &get_node) {
                // Check if this selector uses dynamic pseudo-classes
                if has_dynamic_pseudo_class(selector) {
                    has_dynamic_pseudo = true;
                }
                matched.push((idx, *rule_idx)); // Store (selector_index, rule_index)
            }
        }

        // Sort by specificity (ascending)
        // Inline insertion sort for small arrays - faster than quicksort for n < ~32
        // This is safe: we always have valid selector indices from candidates
        for i in 1..matched.len() {
            let mut j = i;
            while j > 0 {
                // Get specificities for comparison
                let (sel_a, idx_a) = &self.selectors[matched[j - 1].0];
                let (sel_b, idx_b) = &self.selectors[matched[j].0];
                let spec_a = Specificity::new(
                    sel_a.specificity().0,
                    sel_a.specificity().1,
                    sel_a.specificity().2,
                    *idx_a,
                );
                let spec_b = Specificity::new(
                    sel_b.specificity().0,
                    sel_b.specificity().1,
                    sel_b.specificity().2,
                    *idx_b,
                );

                // Only swap if specificity of j-1 > specificity of j
                if spec_a > spec_b {
                    matched.swap(j - 1, j);
                    j -= 1;
                } else {
                    break;
                }
            }
        }

        // Only cache if there are no dynamic pseudo-classes involved
        // Dynamic pseudo-classes depend on node state (hover, focus, etc.)
        // which can change without the DomId changing, making cache entries invalid
        if !has_dynamic_pseudo && !matched.is_empty() {
            let selector_indices: Vec<usize> = matched.iter().map(|(idx, _)| *idx).collect();
            self.match_cache
                .insert(node_id, CachedMatchedRules { selector_indices });
        }

        // Reconstruct MatchedRule structs for return
        matched
            .into_iter()
            .map(|(idx, rule_idx)| {
                let (selector, _) = &self.selectors[idx];
                let (a, b, c) = selector.specificity();
                MatchedRule {
                    selector,
                    rule: &self.stylesheet.rules[rule_idx],
                    specificity: Specificity::new(a, b, c, rule_idx),
                }
            })
            .collect()
    }

    /// Compute final style for a node (without inheritance)
    pub fn compute_style<F>(&mut self, node: &DomNode, get_node: F) -> Style
    where
        F: Fn(DomId) -> Option<&'a DomNode>,
    {
        let matched = self.match_node(node, &get_node);
        let mut style = Style::default();

        // Collect all declarations first to avoid holding borrow on self
        let declarations: Vec<_> = matched
            .iter()
            .flat_map(|r| r.rule.declarations.iter().cloned())
            .collect();

        // Apply declarations (no longer holding borrow on self)
        for decl in &declarations {
            apply_declaration(
                &mut style,
                &decl.property,
                &decl.value,
                &self.stylesheet.variables,
            );
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
        &mut self,
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

        // Collect all declarations first to avoid holding borrow on self
        let declarations: Vec<_> = matched
            .iter()
            .flat_map(|r| r.rule.declarations.iter().cloned())
            .collect();

        // Apply declarations (no longer holding borrow on self)
        for decl in &declarations {
            apply_declaration(
                &mut style,
                &decl.property,
                &decl.value,
                &self.stylesheet.variables,
            );
        }

        // Apply inline style last (highest priority)
        if let Some(ref inline) = node.inline_style {
            style = style.merge(inline);
        }

        style
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
                                    c.to_lowercase().contains(val.to_lowercase().as_str())
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
                                node_id.to_lowercase().contains(val.to_lowercase().as_str())
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
                                widget_type
                                    .to_lowercase()
                                    .contains(val.to_lowercase().as_str())
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
                        let is_true =
                            val.as_str() == "true" || val.as_str() == "1" || val.is_empty();
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
                        let is_true =
                            val.as_str() == "true" || val.as_str() == "1" || val.is_empty();
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
                        let is_true =
                            val.as_str() == "true" || val.as_str() == "1" || val.is_empty();
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
