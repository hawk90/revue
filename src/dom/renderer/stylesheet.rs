//! Stylesheet management for DomRenderer

use crate::dom::renderer::types::DomRenderer;
use crate::style::StyleSheet;

impl DomRenderer {
    /// Create with a stylesheet
    pub fn with_stylesheet(stylesheet: StyleSheet) -> Self {
        Self {
            tree: crate::dom::DomTree::new(),
            stylesheet,
            styles: std::collections::HashMap::new(),
            cached_selectors: None,
            focused: None,
            hovered: None,
        }
    }

    /// Set the stylesheet
    pub fn set_stylesheet(&mut self, stylesheet: StyleSheet) {
        self.stylesheet = stylesheet;
        // Invalidate cached selectors when stylesheet changes
        self.cached_selectors = None;
        // Invalidate style cache
        self.styles.clear();
    }

    /// Get mutable access to the stylesheet (for hot reload)
    pub fn stylesheet_mut(&mut self) -> &mut StyleSheet {
        // Invalidate cached selectors when stylesheet might be modified
        self.cached_selectors = None;
        // Invalidate style cache
        self.styles.clear();
        &mut self.stylesheet
    }

    /// Invalidate stylesheet and style caches
    pub fn invalidate_styles(&mut self) {
        self.cached_selectors = None;
        self.styles.clear();
    }
}
