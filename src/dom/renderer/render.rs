//! Rendering logic for DomRenderer

use crate::dom::renderer::types::DomRenderer;
use crate::dom::Query;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::{RenderContext, View};

impl DomRenderer {
    /// Render with DOM context (with CSS inheritance)
    pub fn render<V: View>(&mut self, root: &V, buffer: &mut Buffer, area: Rect) {
        // Compute styles with inheritance
        self.compute_styles_with_inheritance();

        // Get root style and state
        let root_id = self.tree.root_id();
        let (style, state) = if let Some(id) = root_id {
            let style = self.styles.get(&id);
            let state = self.tree.get(id).map(|n| &n.state);
            (style, state)
        } else {
            (None, None)
        };

        // Create context with style and state
        let mut ctx = if let (Some(style), Some(state)) = (style, state) {
            RenderContext::full(buffer, area, style, state)
        } else if let Some(style) = style {
            RenderContext::with_style(buffer, area, style)
        } else {
            RenderContext::new(buffer, area)
        };

        root.render(&mut ctx);
    }

    /// Query nodes by selector
    pub fn query(&self, selector: &str) -> Vec<&crate::dom::DomNode> {
        self.tree.query_all(selector).all().to_vec()
    }

    /// Query one node by selector
    pub fn query_one(&self, selector: &str) -> Option<&crate::dom::DomNode> {
        self.tree.query_one(selector)
    }

    /// Get node by element ID
    pub fn get_by_id(&self, id: &str) -> Option<&crate::dom::DomNode> {
        self.tree.get_by_id(id)
    }
}
