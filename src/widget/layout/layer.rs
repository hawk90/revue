//! Layer container for overlapping widgets
//!
//! Layers allow multiple widgets to be rendered in the same area,
//! with later children appearing on top of earlier ones.

use crate::layout::Rect;
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
    /// Minimum width constraint (0 = no constraint)
    min_width: u16,
    /// Minimum height constraint (0 = no constraint)
    min_height: u16,
    /// Maximum width constraint (0 = no constraint)
    max_width: u16,
    /// Maximum height constraint (0 = no constraint)
    max_height: u16,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Layers {
    /// Create a new empty layer container
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0,
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

    /// Set minimum width constraint
    pub fn min_width(mut self, width: u16) -> Self {
        self.min_width = width;
        self
    }

    /// Set minimum height constraint
    pub fn min_height(mut self, height: u16) -> Self {
        self.min_height = height;
        self
    }

    /// Set maximum width constraint (0 = no limit)
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = width;
        self
    }

    /// Set maximum height constraint (0 = no limit)
    pub fn max_height(mut self, height: u16) -> Self {
        self.max_height = height;
        self
    }

    /// Set both min width and height
    pub fn min_size(self, width: u16, height: u16) -> Self {
        self.min_width(width).min_height(height)
    }

    /// Set both max width and height (0 = no limit)
    pub fn max_size(self, width: u16, height: u16) -> Self {
        self.max_width(width).max_height(height)
    }

    /// Set all size constraints at once
    pub fn constrain(self, min_w: u16, min_h: u16, max_w: u16, max_h: u16) -> Self {
        self.min_width(min_w)
            .min_height(min_h)
            .max_width(max_w)
            .max_height(max_h)
    }

    /// Apply size constraints to the available area
    fn apply_constraints(&self, area: Rect) -> Rect {
        let width = if self.min_width > 0 && area.width < self.min_width {
            self.min_width
        } else if self.max_width > 0 && area.width > self.max_width {
            self.max_width
        } else {
            area.width
        };

        let height = if self.min_height > 0 && area.height < self.min_height {
            self.min_height
        } else if self.max_height > 0 && area.height > self.max_height {
            self.max_height
        } else {
            area.height
        };

        Rect::new(area.x, area.y, width, height)
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

        let _area = self.apply_constraints(ctx.area);

        // Build render order: sort by z-index (stable to preserve insertion order for ties)
        let mut order: Vec<usize> = (0..self.children.len()).collect();
        order.sort_by_key(|&i| self.children[i].z_index);

        // Render each child in z-index order, preserving overlay queue
        for &idx in &order {
            self.children[idx].child.render(ctx);
        }
    }
}

impl_styled_view!(Layers);
impl_props_builders!(Layers);

/// Create a layer container
pub fn layers() -> Layers {
    Layers::new()
}
