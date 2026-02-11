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
