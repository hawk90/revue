//! CSS parser types

use std::collections::HashMap;

/// A parsed @keyframes definition
#[derive(Debug, Clone)]
pub struct KeyframesDefinition {
    /// Name of the keyframes animation
    pub name: String,
    /// Keyframe blocks (percent + declarations)
    pub keyframes: Vec<KeyframeBlock>,
}

/// A single keyframe block within @keyframes
#[derive(Debug, Clone)]
pub struct KeyframeBlock {
    /// Percentage (0-100)
    pub percent: u8,
    /// CSS declarations at this keyframe
    pub declarations: Vec<Declaration>,
}

/// A parsed stylesheet
#[derive(Debug, Default, Clone)]
pub struct StyleSheet {
    /// CSS rules
    pub rules: Vec<Rule>,
    /// CSS variables
    pub variables: HashMap<String, String>,
    /// @keyframes definitions
    pub keyframes: HashMap<String, KeyframesDefinition>,
}

/// A CSS rule (selector + declarations)
#[derive(Debug, Clone)]
pub struct Rule {
    /// Selector string (e.g., ".class", "#id", "widget")
    pub selector: String,
    /// Declarations in this rule
    pub declarations: Vec<Declaration>,
}

/// A CSS declaration (property: value)
#[derive(Debug, Clone)]
pub struct Declaration {
    /// Property name
    pub property: String,
    /// Property value
    pub value: String,
}

impl StyleSheet {
    /// Create a new empty stylesheet
    pub fn new() -> Self {
        Self::default()
    }

    /// Merge another stylesheet into this one
    pub fn merge(&mut self, other: StyleSheet) {
        self.rules.extend(other.rules);
        self.variables.extend(other.variables);
        self.keyframes.extend(other.keyframes);
    }

    /// Get a CSS variable value
    pub fn variable(&self, name: &str) -> Option<&str> {
        self.variables.get(name).map(|s| s.as_str())
    }

    /// Get rules matching a selector
    pub fn rules(&self, selector: &str) -> Vec<&Rule> {
        self.rules
            .iter()
            .filter(|r| r.selector == selector)
            .collect()
    }

    /// Apply stylesheet to a base style for a given selector
    pub fn apply(&self, selector: &str, base: &crate::style::Style) -> crate::style::Style {
        let mut style = base.clone();

        for rule in self.rules(selector) {
            for decl in &rule.declarations {
                super::apply_declaration(&mut style, &decl.property, &decl.value, &self.variables);
            }
        }

        style
    }

    /// Get a @keyframes definition by name
    pub fn keyframes_definition(&self, name: &str) -> Option<&KeyframesDefinition> {
        self.keyframes.get(name)
    }

    /// Resolve animation for a selector into a KeyframeAnimation
    ///
    /// Looks for `animation` or `animation-*` properties in the selector's rules,
    /// then resolves the referenced @keyframes definition into a `KeyframeAnimation`.
    pub fn animation(&self, selector: &str) -> Option<crate::style::animation::KeyframeAnimation> {
        super::apply::resolve_animation(self, selector)
    }
}
