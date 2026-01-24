//! Style inspector for viewing computed styles

use super::types::{ComputedProperty, StyleCategory};
use std::collections::HashMap;

/// Style inspector for viewing computed styles
#[derive(Debug, Default)]
pub struct StyleInspector {
    /// Properties for current selection
    pub properties: Vec<ComputedProperty>,
    /// Applied classes
    pub classes: Vec<String>,
    /// Widget ID
    pub widget_id: Option<String>,
    /// Widget type
    pub widget_type: String,
    /// Selected property index
    pub selected: Option<usize>,
    /// Scroll offset (for future UI)
    pub _scroll: usize,
    /// Show inherited properties
    pub show_inherited: bool,
    /// Show overridden properties
    pub show_overridden: bool,
    /// Filter by category
    pub category_filter: Option<StyleCategory>,
    /// Expanded categories
    pub expanded_categories: HashMap<StyleCategory, bool>,
}
