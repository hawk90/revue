//! Layer container for overlapping widgets
//!
//! Layers allow multiple widgets to be rendered in the same area,
//! with later children appearing on top of earlier ones.

use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Wrapper holding a child view with its z-index for render ordering
struct LayerChild {
    child: Box<dyn View>,
    z_index: i16,
}

/// A container that renders children in overlapping layers
///
/// Children are rendered in order, with later children appearing
/// on top of earlier ones. This is useful for:
/// - Modal dialogs
/// - Tooltips
/// - Gantt chart overlays
/// - Any UI where elements need to overlap
///
/// Z-index can be set per child to control stacking order.
/// Children with higher z-index render on top. Children with
/// equal z-index maintain insertion order.
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let layered = Layers::new()
///     .child(background_grid())       // z=0, bottom
///     .child_z(gantt_bars(), 1)        // z=1, middle
///     .child_z(tooltip(), 10);         // z=10, top
/// ```
pub struct Layers {
    children: Vec<LayerChild>,
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

    /// Add a child to the layer stack with default z-index (0)
    ///
    /// Children added later will render on top of earlier children
    /// when z-index values are equal.
    pub fn child<V: View + 'static>(mut self, child: V) -> Self {
        self.children.push(LayerChild {
            child: Box::new(child),
            z_index: 0,
        });
        self
    }

    /// Add a child with an explicit z-index
    ///
    /// Higher z-index values render on top. Children with equal
    /// z-index maintain insertion order (stable sort).
    pub fn child_z<V: View + 'static>(mut self, child: V, z_index: i16) -> Self {
        self.children.push(LayerChild {
            child: Box::new(child),
            z_index,
        });
        self
    }

    /// Add multiple children at once (all with default z-index 0)
    pub fn children<I, V>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: View + 'static,
    {
        for child in children {
            self.children.push(LayerChild {
                child: Box::new(child),
                z_index: 0,
            });
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
        if self.children.is_empty() {
            return;
        }

        // Build render order: sort by z-index (stable to preserve insertion order for ties)
        let mut order: Vec<usize> = (0..self.children.len()).collect();
        order.sort_by_key(|&i| self.children[i].z_index);

        // Render each child in z-index order
        for &idx in &order {
            let mut child_ctx = RenderContext::new(ctx.buffer, ctx.area);
            self.children[idx].child.render(&mut child_ctx);
        }
    }
}

impl_styled_view!(Layers);
impl_props_builders!(Layers);

/// Create a layer container
pub fn layers() -> Layers {
    Layers::new()
}
