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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_stylesheet() {
        let stylesheet = StyleSheet::new();
        let renderer = DomRenderer::with_stylesheet(stylesheet);
        assert!(renderer.tree().is_empty());
        assert!(renderer.styles.is_empty());
        assert!(renderer.cached_selectors.is_none());
    }

    #[test]
    fn test_set_stylesheet() {
        let mut renderer = DomRenderer::with_stylesheet(StyleSheet::new());
        let new_stylesheet = StyleSheet::new();

        renderer.set_stylesheet(new_stylesheet);

        // Caches should be invalidated
        assert!(renderer.styles.is_empty());
        assert!(renderer.cached_selectors.is_none());
    }

    #[test]
    fn test_stylesheet_mut() {
        let mut renderer = DomRenderer::with_stylesheet(StyleSheet::new());

        {
            let _sheet = renderer.stylesheet_mut();
            // Caches should be invalidated after getting mutable access
            assert!(renderer.styles.is_empty());
            assert!(renderer.cached_selectors.is_none());
        }
    }

    #[test]
    fn test_invalidate_styles() {
        let mut renderer = DomRenderer::with_stylesheet(StyleSheet::new());
        use crate::dom::DomId;
        let dom_id = DomId::new(1);
        renderer
            .styles
            .insert(dom_id, crate::style::Style::default());
        renderer.cached_selectors = Some(Vec::new());

        renderer.invalidate_styles();

        assert!(renderer.styles.is_empty());
        assert!(renderer.cached_selectors.is_none());
    }

    #[test]
    fn test_stylesheet_mut_invalidates_on_access() {
        let mut renderer = DomRenderer::with_stylesheet(StyleSheet::new());
        renderer.cached_selectors = Some(vec![]);

        // Accessing stylesheet_mut should invalidate cached selectors
        let _sheet = renderer.stylesheet_mut();
        assert!(renderer.cached_selectors.is_none());
    }
}
