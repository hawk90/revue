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

        // Check if overflow: hidden is set via CSS
        let overflow_hidden = ctx.css_overflow_hidden();
        let parent_clip = ctx.clip();

        let n = self.children.len();
        let total_gap = self.gap * (n.saturating_sub(1) as u16);

        match self.direction {
            Direction::Row => {
                let available_width = area.width.saturating_sub(total_gap);
                let widths = self.calculate_sizes(available_width, n);

                let mut x: u16 = 0;
                for (i, child) in self.children.iter().enumerate() {
                    let w = widths[i];
                    let child_area = ctx.sub_area(x, 0, w, area.height);
                    let mut child_ctx = RenderContext::child_ctx_with_overflow(
                        ctx.buffer,
                        child_area,
                        overflow_hidden,
                        parent_clip,
                    );
                    child.render(&mut child_ctx);
                    x = x.saturating_add(w).saturating_add(self.gap);
                }
            }
            Direction::Column => {
                let available_height = area.height.saturating_sub(total_gap);
                let heights = self.calculate_sizes(available_height, n);

                let mut y: u16 = 0;
                for (i, child) in self.children.iter().enumerate() {
                    let h = heights[i];
                    let child_area = ctx.sub_area(0, y, area.width, h);
                    let mut child_ctx = RenderContext::child_ctx_with_overflow(
                        ctx.buffer,
                        child_area,
                        overflow_hidden,
                        parent_clip,
                    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::widget::Text;

    #[test]
    fn test_stack_new_is_empty() {
        let s = Stack::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_stack_add_children() {
        let s = Stack::new().child(Text::new("A")).child(Text::new("B"));
        assert_eq!(s.len(), 2);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_stack_direction() {
        let row = Stack::new().direction(Direction::Row);
        assert_eq!(row.direction, Direction::Row);

        let col = Stack::new().direction(Direction::Column);
        assert_eq!(col.direction, Direction::Column);
    }

    #[test]
    fn test_vstack_hstack_constructors() {
        let v = vstack();
        assert_eq!(v.direction, Direction::Column);

        let h = hstack();
        assert_eq!(h.direction, Direction::Row);
    }

    #[test]
    fn test_stack_render_empty_no_panic() {
        let mut buf = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buf, area);
        let s = Stack::new();
        s.render(&mut ctx); // Should not panic
    }

    #[test]
    fn test_stack_render_zero_area_no_panic() {
        let mut buf = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 0, 0);
        let mut ctx = RenderContext::new(&mut buf, area);
        let s = Stack::new().child(Text::new("A"));
        s.render(&mut ctx); // Should not panic
    }

    #[test]
    fn test_stack_calculate_sizes_auto() {
        let s = Stack::new()
            .child(Text::new("A"))
            .child(Text::new("B"))
            .child(Text::new("C"));
        let sizes = s.calculate_sizes(30, 3);
        assert_eq!(sizes.len(), 3);
        assert_eq!(sizes.iter().sum::<u16>(), 30);
    }

    #[test]
    fn test_stack_calculate_sizes_fixed() {
        let s = Stack::new()
            .child_sized(Text::new("A"), 10)
            .child_sized(Text::new("B"), 20);
        let sizes = s.calculate_sizes(50, 2);
        assert_eq!(sizes, vec![10, 20]);
    }

    #[test]
    fn test_stack_calculate_sizes_flex() {
        let s = Stack::new()
            .child_flex(Text::new("A"), 1.0)
            .child_flex(Text::new("B"), 2.0);
        let sizes = s.calculate_sizes(30, 2);
        assert_eq!(sizes[0], 10);
        assert_eq!(sizes[1], 20);
    }

    #[test]
    fn test_stack_calculate_sizes_mixed() {
        let s = Stack::new()
            .child_sized(Text::new("Fixed"), 10)
            .child_flex(Text::new("Flex"), 1.0);
        let sizes = s.calculate_sizes(30, 2);
        assert_eq!(sizes[0], 10);
        assert_eq!(sizes[1], 20); // 30 - 10 = 20 (no auto children)
    }

    #[test]
    fn test_stack_calculate_sizes_empty() {
        let s = Stack::new();
        let sizes = s.calculate_sizes(100, 0);
        assert!(sizes.is_empty());
    }

    #[test]
    fn test_stack_constraints() {
        let s = Stack::new()
            .min_width(20)
            .max_width(60)
            .min_height(5)
            .max_height(30);
        let area = Rect::new(0, 0, 100, 100);
        let constrained = s.apply_constraints(area);
        assert_eq!(constrained.width, 60);
        assert_eq!(constrained.height, 30);
    }

    #[test]
    fn test_stack_constraints_below_min() {
        let s = Stack::new().min_width(20).min_height(10);
        let area = Rect::new(0, 0, 5, 3);
        let constrained = s.apply_constraints(area);
        assert_eq!(constrained.width, 20);
        assert_eq!(constrained.height, 10);
    }

    #[test]
    fn test_stack_default() {
        let s = Stack::default();
        assert!(s.is_empty());
        assert_eq!(s.direction, Direction::Row);
    }

    #[test]
    fn test_stack_gap() {
        let s = Stack::new().gap(2);
        assert_eq!(s.gap, 2);
    }

    #[test]
    fn test_stack_render_row_children() {
        let mut buf = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buf, area);
        let s = hstack().child(Text::new("AB")).child(Text::new("CD"));
        s.render(&mut ctx);
        assert_eq!(buf.get(0, 0).unwrap().symbol, 'A');
        assert_eq!(buf.get(1, 0).unwrap().symbol, 'B');
    }
}
