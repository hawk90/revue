//! Core types for CSS stylesheet representation

use std::collections::HashMap;

/// A parsed stylesheet
#[derive(Debug, Default, Clone)]
pub struct StyleSheet {
    /// CSS rules
    pub rules: Vec<Rule>,
    /// CSS variables
    pub variables: HashMap<String, String>,
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
