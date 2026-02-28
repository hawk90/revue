//! Stack container widget

use crate::layout::Rect;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// A stack container for layout
pub struct Stack {
    children: Vec<Box<dyn View>>,
    direction: Direction,
    gap: u16,
    /// Heights/widths for each child (None = auto/minimal)
    sizes: Vec<Option<u16>>,
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

/// Layout direction for Stack
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum Direction {
    /// Horizontal layout (left to right)
    #[default]
    Row,
    /// Vertical layout (top to bottom)
    Column,
}

impl Stack {
    /// Create a new empty Stack
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            direction: Direction::default(),
            gap: 0,
            sizes: Vec::new(),
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0,
            props: WidgetProps::new(),
        }
    }

    /// Set layout direction
    pub fn direction(mut self, dir: Direction) -> Self {
        self.direction = dir;
        self
    }

    /// Set gap between children
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Add a child view
    pub fn child(mut self, child: impl View + 'static) -> Self {
        self.children.push(Box::new(child));
        self.sizes.push(None); // Default: auto size (minimal)
        self
    }

    /// Add a child view with a fixed size (height for Column, width for Row)
    pub fn child_sized(mut self, child: impl View + 'static, size: u16) -> Self {
        self.children.push(Box::new(child));
        self.sizes.push(Some(size));
        self
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

    /// Get number of children
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
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

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Stack {
    fn render(&self, ctx: &mut RenderContext) {
        if self.children.is_empty() {
            return;
        }

        let area = self.apply_constraints(ctx.area);
        if area.width == 0 || area.height == 0 {
            return;
        }

        let n = self.children.len();
        let total_gap = self.gap * (n.saturating_sub(1) as u16);

        match self.direction {
            Direction::Row => {
                let available_width = area.width.saturating_sub(total_gap);

                // Calculate widths based on sizes
                let widths = self.calculate_sizes(available_width, n);

                let mut x = area.x;
                for (i, child) in self.children.iter().enumerate() {
                    let w = widths[i];

                    let child_area = Rect::new(x, area.y, w, area.height);
                    let mut child_ctx = RenderContext::new(ctx.buffer, child_area);
                    child.render(&mut child_ctx);

                    x = x.saturating_add(w).saturating_add(self.gap);
                }
            }
            Direction::Column => {
                let available_height = area.height.saturating_sub(total_gap);

                // Calculate heights based on sizes
                let heights = self.calculate_sizes(available_height, n);

                let mut y = area.y;
                for (i, child) in self.children.iter().enumerate() {
                    let h = heights[i];

                    let child_area = Rect::new(area.x, y, area.width, h);
                    let mut child_ctx = RenderContext::new(ctx.buffer, child_area);
                    child.render(&mut child_ctx);

                    y = y.saturating_add(h).saturating_add(self.gap);
                }
            }
        }
    }

    fn children(&self) -> &[Box<dyn View>] {
        &self.children
    }

    crate::impl_view_meta!("Stack");
}

impl_styled_view!(Stack);
impl_props_builders!(Stack);

impl Stack {
    /// Calculate sizes for children based on available space
    ///
    /// Strategy:
    /// - If all children have None (auto), give each minimal space (1 unit)
    /// - If some have fixed sizes, use those and divide remaining among auto children
    fn calculate_sizes(&self, available: u16, n: usize) -> Vec<u16> {
        if n == 0 {
            return Vec::new();
        }

        // Count auto-sized children and calculate fixed space used
        let mut auto_count = 0;
        let mut fixed_total = 0u16;

        for size_opt in &self.sizes {
            match size_opt {
                Some(size) => fixed_total = fixed_total.saturating_add(*size),
                None => auto_count += 1,
            }
        }

        // Calculate space for auto children
        let auto_space = if auto_count > 0 {
            let remaining = available.saturating_sub(fixed_total);
            if remaining > 0 {
                // If we have space, divide it among auto children
                remaining / (auto_count as u16)
            } else {
                // Not enough space, give minimal (1 unit) to each
                1
            }
        } else {
            0
        };

        // Allocate sizes
        let mut result = Vec::with_capacity(n);
        for size_opt in &self.sizes {
            match size_opt {
                Some(size) => result.push(*size),
                None => result.push(auto_space),
            }
        }

        result
    }
}

/// Create a vertical stack
pub fn vstack() -> Stack {
    Stack::new().direction(Direction::Column)
}

/// Create a horizontal stack
pub fn hstack() -> Stack {
    Stack::new().direction(Direction::Row)
}
