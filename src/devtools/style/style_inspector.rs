//! StyleInspector method implementations

use super::core::StyleInspector;
use super::types::{ComputedProperty, PropertySource, StyleCategory};
use std::collections::HashMap;

impl StyleInspector {
    /// Create new style inspector
    pub fn new() -> Self {
        let mut expanded = HashMap::new();
        for cat in StyleCategory::all() {
            expanded.insert(*cat, true);
        }

        Self {
            show_inherited: true,
            show_overridden: false,
            expanded_categories: expanded,
            ..Default::default()
        }
    }

    /// Clear current selection
    pub fn clear(&mut self) {
        self.properties.clear();
        self.classes.clear();
        self.widget_id = None;
        self.widget_type.clear();
        self.selected = None;
    }

    /// Set widget info
    pub fn set_widget(&mut self, type_name: impl Into<String>, id: Option<String>) {
        self.widget_type = type_name.into();
        self.widget_id = id;
    }

    /// Add a class
    pub fn add_class(&mut self, class: impl Into<String>) {
        self.classes.push(class.into());
    }

    /// Add a property
    pub fn add_property(&mut self, prop: ComputedProperty) {
        self.properties.push(prop);
    }

    /// Set properties
    pub fn set_properties(&mut self, props: Vec<ComputedProperty>) {
        self.properties = props;
    }

    /// Toggle show inherited
    pub fn toggle_inherited(&mut self) {
        self.show_inherited = !self.show_inherited;
    }

    /// Toggle show overridden
    pub fn toggle_overridden(&mut self) {
        self.show_overridden = !self.show_overridden;
    }

    /// Set category filter
    pub fn set_category_filter(&mut self, category: Option<StyleCategory>) {
        self.category_filter = category;
    }

    /// Toggle category expansion
    pub fn toggle_category(&mut self, category: StyleCategory) {
        let expanded = self.expanded_categories.entry(category).or_insert(true);
        *expanded = !*expanded;
    }

    /// Get filtered properties
    pub fn filtered(&self) -> Vec<&ComputedProperty> {
        self.properties
            .iter()
            .filter(|p| {
                if !self.show_inherited && p.source == PropertySource::Inherited {
                    return false;
                }
                if !self.show_overridden && p.overridden {
                    return false;
                }
                if let Some(cat) = self.category_filter {
                    if StyleCategory::from_property(&p.name) != cat {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Select next property
    pub fn select_next(&mut self) {
        let count = self.filtered().len();
        if count == 0 {
            return;
        }

        self.selected = Some(match self.selected {
            Some(i) => (i + 1).min(count - 1),
            None => 0,
        });
    }

    /// Select previous property
    pub fn select_prev(&mut self) {
        let count = self.filtered().len();
        if count == 0 {
            return;
        }

        self.selected = Some(match self.selected {
            Some(i) => i.saturating_sub(1),
            None => 0,
        });
    }
}
