//! Stack container widget

use crate::layout::Rect;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Size specification for a stack child
#[derive(Clone, Copy, Debug)]
enum ChildSize {
    /// Auto-sized (equal distribution of remaining space)
    Auto,
    /// Fixed pixel size
    Fixed(u16),
    /// Flex grow factor (proportional distribution of remaining space)
    Flex(f32),
}

/// A stack container for layout
pub struct Stack {
    children: Vec<Box<dyn View>>,
    direction: Direction,
    gap: u16,
    /// Size specification for each child
    child_sizes: Vec<ChildSize>,
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
            child_sizes: Vec::new(),
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
        self.child_sizes.push(ChildSize::Auto);
        self
    }

    /// Add a child view with a fixed size (height for Column, width for Row)
    pub fn child_sized(mut self, child: impl View + 'static, size: u16) -> Self {
        self.children.push(Box::new(child));
        self.child_sizes.push(ChildSize::Fixed(size));
        self
    }

    /// Add a child view with a flex grow factor
    ///
    /// Children with flex grow share remaining space proportionally.
    /// A child with `flex(2.0)` gets twice the space of one with `flex(1.0)`.
    pub fn child_flex(mut self, child: impl View + 'static, grow: f32) -> Self {
        self.children.push(Box::new(child));
        self.child_sizes.push(ChildSize::Flex(grow.max(0.0)));
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
        let eff_max_w = if self.max_width > 0 {
            self.max_width.max(self.min_width)
        } else {
            u16::MAX
        };
        let eff_max_h = if self.max_height > 0 {
            self.max_height.max(self.min_height)
        } else {
            u16::MAX
        };
        let width = area.width.clamp(self.min_width, eff_max_w);
        let height = area.height.clamp(self.min_height, eff_max_h);

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

                let mut x: u16 = 0;
                for (i, child) in self.children.iter().enumerate() {
                    let w = widths[i];

                    let child_area = ctx.sub_area(x, 0, w, area.height);
                    let mut child_ctx = RenderContext::new(ctx.buffer, child_area);
                    child.render(&mut child_ctx);

                    x = x.saturating_add(w).saturating_add(self.gap);
                }
            }
            Direction::Column => {
                let available_height = area.height.saturating_sub(total_gap);

                // Calculate heights based on sizes
                let heights = self.calculate_sizes(available_height, n);

                let mut y: u16 = 0;
                for (i, child) in self.children.iter().enumerate() {
                    let h = heights[i];

                    let child_area = ctx.sub_area(0, y, area.width, h);
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
    /// - Fixed children get their exact size
    /// - Flex children share remaining space proportionally by grow factor
    /// - Auto children share remaining space equally (after flex allocation)
    fn calculate_sizes(&self, available: u16, n: usize) -> Vec<u16> {
        if n == 0 {
            return Vec::new();
        }

        // First pass: calculate fixed space and collect flex/auto info
        let mut auto_count: usize = 0;
        let mut total_grow: f32 = 0.0;
        let mut fixed_total = 0u16;

        for cs in &self.child_sizes {
            match cs {
                ChildSize::Fixed(size) => fixed_total = fixed_total.saturating_add(*size),
                ChildSize::Flex(grow) => total_grow += grow,
                ChildSize::Auto => auto_count += 1,
            }
        }

        let remaining = available.saturating_sub(fixed_total);
        let mut result = vec![0u16; n];

        // Assign fixed sizes
        for (i, cs) in self.child_sizes.iter().enumerate() {
            if let ChildSize::Fixed(size) = cs {
                result[i] = *size;
            }
        }

        if total_grow > 0.0 {
            // Distribute remaining space to flex children proportionally
            let flex_indices: Vec<usize> = self
                .child_sizes
                .iter()
                .enumerate()
                .filter(|(_, cs)| matches!(cs, ChildSize::Flex(_)))
                .map(|(i, _)| i)
                .collect();

            // Space for flex children: remaining minus space for auto children
            let auto_min = auto_count as u16;
            let flex_space = remaining.saturating_sub(auto_min);
            let mut distributed: u16 = 0;

            for (fi, &i) in flex_indices.iter().enumerate() {
                let grow = match self.child_sizes[i] {
                    ChildSize::Flex(g) => g,
                    _ => 0.0,
                };
                let size = if fi == flex_indices.len() - 1 {
                    flex_space.saturating_sub(distributed)
                } else {
                    ((flex_space as f32) * grow / total_grow).round() as u16
                };
                result[i] = size;
                distributed = distributed.saturating_add(size);
            }

            // Auto children get 1 pixel each when flex is active
            for (i, cs) in self.child_sizes.iter().enumerate() {
                if matches!(cs, ChildSize::Auto) {
                    result[i] = 1;
                }
            }
        } else if auto_count > 0 {
            // No flex: distribute remaining space equally to auto children
            let (per_auto, extra) = if remaining > 0 {
                (
                    remaining / (auto_count as u16),
                    remaining % (auto_count as u16),
                )
            } else {
                (1, 0)
            };

            let mut extra_given = 0u16;
            for (i, cs) in self.child_sizes.iter().enumerate() {
                if matches!(cs, ChildSize::Auto) {
                    let mut size = per_auto;
                    if extra_given < extra {
                        size += 1;
                        extra_given += 1;
                    }
                    result[i] = size;
                }
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
