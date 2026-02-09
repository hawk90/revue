//! Layer container for overlapping widgets
//!
//! Layers allow multiple widgets to be rendered in the same area,
//! with later children appearing on top of earlier ones.

use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// A container that renders children in overlapping layers
///
/// Children are rendered in order, with later children appearing
/// on top of earlier ones. This is useful for:
/// - Modal dialogs
/// - Tooltips
/// - Gantt chart overlays
/// - Any UI where elements need to overlap
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let layered = Layers::new()
///     .child(background_grid())  // Bottom layer
///     .child(gantt_bars())       // Middle layer
///     .child(tooltip());         // Top layer
/// ```
pub struct Layers {
    children: Vec<Box<dyn View>>,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Layers {
    /// Create a new empty layer container
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            props: WidgetProps::new(),
        }
    }

    /// Add a child to the layer stack
    ///
    /// Children added later will render on top of earlier children.
    pub fn child<V: View + 'static>(mut self, child: V) -> Self {
        self.children.push(Box::new(child));
        self
    }

    /// Add multiple children at once
    pub fn children<I, V>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: View + 'static,
    {
        for child in children {
            self.children.push(Box::new(child));
        }
        self
    }

    /// Get the number of layers
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

impl Default for Layers {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Layers {
    crate::impl_view_meta!("Layers");

    fn render(&self, ctx: &mut RenderContext) {
        // Render each child in the same area
        // Later children will overwrite earlier ones where they overlap
        for child in &self.children {
            // Create a new context with the same area
            let mut child_ctx = RenderContext::new(ctx.buffer, ctx.area);
            child.render(&mut child_ctx);
        }
    }

    fn children(&self) -> &[Box<dyn View>] {
        &self.children
    }
}

impl_styled_view!(Layers);
impl_props_builders!(Layers);

/// Create a layer container
pub fn layers() -> Layers {
    Layers::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::{Border, Text};

    // =========================================================================
    // Layers::new tests
    // =========================================================================

    #[test]
    fn test_layers_new() {
        let l = Layers::new();
        assert!(l.is_empty());
        assert_eq!(l.len(), 0);
    }

    // =========================================================================
    // Layers::child tests
    // =========================================================================

    #[test]
    fn test_layers_child() {
        let l = Layers::new()
            .child(Text::new("Layer 1"))
            .child(Text::new("Layer 2"));

        assert_eq!(l.len(), 2);
    }

    #[test]
    fn test_layers_child_single() {
        let l = Layers::new().child(Text::new("Single"));
        assert_eq!(l.len(), 1);
    }

    #[test]
    fn test_layers_child_multiple() {
        let l = Layers::new()
            .child(Text::new("A"))
            .child(Text::new("B"))
            .child(Text::new("C"))
            .child(Text::new("D"))
            .child(Text::new("E"));

        assert_eq!(l.len(), 5);
    }

    // =========================================================================
    // Layers::children tests
    // =========================================================================

    #[test]
    fn test_layers_children_vec() {
        let texts: Vec<Text> = vec![Text::new("A"), Text::new("B"), Text::new("C")];

        let l = Layers::new().children(texts);
        assert_eq!(l.len(), 3);
    }

    #[test]
    fn test_layers_children_array() {
        let texts = [Text::new("X"), Text::new("Y"), Text::new("Z")];

        let l = Layers::new().children(texts);
        assert_eq!(l.len(), 3);
    }

    #[test]
    fn test_layers_children_empty() {
        let texts: Vec<Text> = vec![];

        let l = Layers::new().children(texts);
        assert_eq!(l.len(), 0);
        assert!(l.is_empty());
    }

    #[test]
    fn test_layers_children_iterator() {
        let texts = (0..5).map(|i| Text::new(format!("Item {}", i)));

        let l = Layers::new().children(texts);
        assert_eq!(l.len(), 5);
    }

    #[test]
    fn test_layers_child_and_children_mixed() {
        let l = Layers::new()
            .child(Text::new("First"))
            .children(vec![Text::new("A"), Text::new("B")])
            .child(Text::new("Last"));

        assert_eq!(l.len(), 4);
    }

    // =========================================================================
    // Layers::len tests
    // =========================================================================

    #[test]
    fn test_layers_len_empty() {
        let l = Layers::new();
        assert_eq!(l.len(), 0);
    }

    #[test]
    fn test_layers_len_after_adding() {
        let mut l = Layers::new();
        assert_eq!(l.len(), 0);

        l = l.child(Text::new("A"));
        assert_eq!(l.len(), 1);

        l = l.child(Text::new("B"));
        assert_eq!(l.len(), 2);
    }

    // =========================================================================
    // Layers::is_empty tests
    // =========================================================================

    #[test]
    fn test_layers_is_empty_true() {
        let l = Layers::new();
        assert!(l.is_empty());
    }

    #[test]
    fn test_layers_is_empty_false() {
        let l = Layers::new().child(Text::new("Not Empty"));
        assert!(!l.is_empty());
    }

    #[test]
    fn test_layers_is_empty_after_children() {
        let l = Layers::new().children(vec![Text::new("A"), Text::new("B")]);
        assert!(!l.is_empty());
    }

    // =========================================================================
    // Layers render tests
    // =========================================================================

    #[test]
    fn test_layers_render() {
        let l = Layers::new()
            .child(Text::new("Bottom"))
            .child(Text::new("Top"));

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        l.render(&mut ctx);

        // "Top" should be visible because it renders last
        // and overwrites "Bottom" where they overlap
    }

    #[test]
    fn test_layers_render_empty() {
        let l = Layers::new();

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        l.render(&mut ctx); // Should not panic
    }

    #[test]
    fn test_layers_with_border() {
        let l = Layers::new()
            .child(Border::single().child(Text::new("Background")))
            .child(Text::new("Overlay"));

        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        l.render(&mut ctx);
    }

    #[test]
    fn test_layers_render_order() {
        let l = Layers::new()
            .child(Text::new("First"))
            .child(Text::new("Second"))
            .child(Text::new("Third"));

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        l.render(&mut ctx); // Third should be on top
    }

    // =========================================================================
    // Layers Default tests
    // =========================================================================

    #[test]
    fn test_layers_default() {
        let l = Layers::default();
        assert!(l.is_empty());
        assert_eq!(l.len(), 0);
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_layers_helper() {
        let l = layers().child(Text::new("Test"));
        assert_eq!(l.len(), 1);
    }

    #[test]
    fn test_layers_helper_empty() {
        let l = layers();
        assert!(l.is_empty());
    }

    #[test]
    fn test_layers_helper_chain() {
        let l = layers()
            .child(Text::new("A"))
            .child(Text::new("B"))
            .children(vec![Text::new("C"), Text::new("D")]);

        assert_eq!(l.len(), 4);
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_layers_full_builder_chain() {
        let l = Layers::new()
            .child(Text::new("Layer 1"))
            .child(Text::new("Layer 2"))
            .children(vec![
                Text::new("Layer 3"),
                Text::new("Layer 4"),
                Text::new("Layer 5"),
            ])
            .child(Text::new("Layer 6"));

        assert_eq!(l.len(), 6);
    }
}
