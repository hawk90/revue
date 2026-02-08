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

    /// Get number of children
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
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

        let area = ctx.area;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::widget::Text;

    #[test]
    fn test_stack_new() {
        let s = Stack::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_stack_builder() {
        let s = Stack::new().direction(Direction::Column).gap(2);

        assert_eq!(s.direction, Direction::Column);
        assert_eq!(s.gap, 2);
    }

    #[test]
    fn test_stack_children() {
        let s = Stack::new()
            .child(Text::new("Hello"))
            .child(Text::new("World"));

        assert_eq!(s.len(), 2);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_vstack_hstack_helpers() {
        let v = vstack();
        assert_eq!(v.direction, Direction::Column);

        let h = hstack();
        assert_eq!(h.direction, Direction::Row);
    }

    #[test]
    fn test_stack_render_row() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = hstack().child(Text::new("AB")).child(Text::new("CD"));

        s.render(&mut ctx);

        // First child gets 10 chars, second gets 10
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'B');
        assert_eq!(buffer.get(10, 0).unwrap().symbol, 'C');
        assert_eq!(buffer.get(11, 0).unwrap().symbol, 'D');
    }

    #[test]
    fn test_stack_render_column() {
        let mut buffer = Buffer::new(20, 4);
        let area = Rect::new(0, 0, 20, 4);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = vstack().child(Text::new("Top")).child(Text::new("Bottom"));

        s.render(&mut ctx);

        // First child on row 0-1, second on row 2-3
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
        assert_eq!(buffer.get(0, 2).unwrap().symbol, 'B');
    }

    #[test]
    fn test_stack_render_with_gap() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = hstack().gap(2).child(Text::new("A")).child(Text::new("B"));

        s.render(&mut ctx);

        // With gap=2, width=20: available=18, each child gets 9
        // A at 0, B at 9+2=11
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
        assert_eq!(buffer.get(11, 0).unwrap().symbol, 'B');
    }

    // =========================================================================
    // Direction enum tests
    // =========================================================================

    #[test]
    fn test_direction_default() {
        let dir = Direction::default();
        assert_eq!(dir, Direction::Row);
    }

    #[test]
    fn test_direction_clone() {
        let dir = Direction::Column;
        let cloned = dir.clone();
        assert_eq!(dir, cloned);
    }

    #[test]
    fn test_direction_copy() {
        let dir1 = Direction::Row;
        let dir2 = dir1;
        assert_eq!(dir1, Direction::Row);
        assert_eq!(dir2, Direction::Row);
    }

    #[test]
    fn test_direction_partial_eq() {
        assert_eq!(Direction::Row, Direction::Row);
        assert_ne!(Direction::Row, Direction::Column);
    }

    #[test]
    fn test_direction_debug() {
        let dir = Direction::Column;
        assert!(format!("{:?}", dir).contains("Column"));
    }

    // =========================================================================
    // Stack builder tests
    // =========================================================================

    #[test]
    fn test_stack_new_default_values() {
        let s = Stack::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        assert_eq!(s.direction, Direction::Row);
        assert_eq!(s.gap, 0);
        assert!(s.sizes.is_empty());
    }

    #[test]
    fn test_stack_direction_row() {
        let s = Stack::new().direction(Direction::Row);
        assert_eq!(s.direction, Direction::Row);
    }

    #[test]
    fn test_stack_gap() {
        let s = Stack::new().gap(5);
        assert_eq!(s.gap, 5);
    }

    #[test]
    fn test_stack_child_sized() {
        let s = Stack::new().child_sized(Text::new("Test"), 10);
        assert_eq!(s.len(), 1);
        assert_eq!(s.sizes.len(), 1);
        assert_eq!(s.sizes[0], Some(10));
    }

    #[test]
    fn test_stack_child_auto_size() {
        let s = Stack::new().child(Text::new("Test"));
        assert_eq!(s.sizes[0], None);
    }

    // =========================================================================
    // Stack Default trait tests
    // =========================================================================

    #[test]
    fn test_stack_default() {
        let s = Stack::default();
        assert!(s.is_empty());
        assert_eq!(s.direction, Direction::Row);
    }

    // =========================================================================
    // Stack len and is_empty tests
    // =========================================================================

    #[test]
    fn test_stack_len_empty() {
        let s = Stack::new();
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_stack_len_multiple() {
        let s = Stack::new()
            .child(Text::new("A"))
            .child(Text::new("B"))
            .child(Text::new("C"));
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn test_stack_is_empty_true() {
        let s = Stack::new();
        assert!(s.is_empty());
    }

    #[test]
    fn test_stack_is_empty_false() {
        let s = Stack::new().child(Text::new("X"));
        assert!(!s.is_empty());
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_vstack_creates_column() {
        let s = vstack();
        assert_eq!(s.direction, Direction::Column);
    }

    #[test]
    fn test_hstack_creates_row() {
        let s = hstack();
        assert_eq!(s.direction, Direction::Row);
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_stack_builder_chain() {
        let s = Stack::new()
            .direction(Direction::Column)
            .gap(3)
            .child(Text::new("A"))
            .child_sized(Text::new("B"), 10);
        assert_eq!(s.direction, Direction::Column);
        assert_eq!(s.gap, 3);
        assert_eq!(s.len(), 2);
        assert_eq!(s.sizes[0], None);
        assert_eq!(s.sizes[1], Some(10));
    }

    // =========================================================================
    // Render edge case tests
    // =========================================================================

    #[test]
    fn test_stack_render_empty() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = Stack::new();
        s.render(&mut ctx);
        // Should not crash
    }

    #[test]
    fn test_stack_render_zero_width() {
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = hstack().child(Text::new("Test"));
        s.render(&mut ctx);
        // Should not crash
    }

    #[test]
    fn test_stack_render_zero_height() {
        let mut buffer = Buffer::new(10, 0);
        let area = Rect::new(0, 0, 10, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = vstack().child(Text::new("Test"));
        s.render(&mut ctx);
        // Should not crash
    }

    // =========================================================================
    // child_sized render tests
    // =========================================================================

    #[test]
    fn test_stack_child_sized_row() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = hstack()
            .child_sized(Text::new("A"), 5)
            .child_sized(Text::new("B"), 10);

        s.render(&mut ctx);

        // A at 0-4, B at 5-14
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
        assert_eq!(buffer.get(5, 0).unwrap().symbol, 'B');
    }

    #[test]
    fn test_stack_child_sized_column() {
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = vstack()
            .child_sized(Text::new("Top"), 3)
            .child_sized(Text::new("Bottom"), 5);

        s.render(&mut ctx);

        // Top at 0-2, Bottom at 3-7
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
        assert_eq!(buffer.get(0, 3).unwrap().symbol, 'B');
    }

    #[test]
    fn test_stack_mixed_sizes_row() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Fixed 5, auto (gets remaining 15), fixed 3
        let s = hstack()
            .gap(1)
            .child_sized(Text::new("A"), 5)
            .child(Text::new("B"))
            .child_sized(Text::new("C"), 3);

        s.render(&mut ctx);

        // A: 0-4, gap: 5, B: 6-20 (auto gets 15), gap: 21 (clamped), C: 22+
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
        assert_eq!(buffer.get(6, 0).unwrap().symbol, 'B');
    }

    // =========================================================================
    // calculate_sizes edge cases
    // =========================================================================

    #[test]
    fn test_calculate_sizes_empty() {
        let s = Stack::new();
        let sizes = s.calculate_sizes(100, 0);
        assert!(sizes.is_empty());
    }

    #[test]
    fn test_calculate_sizes_all_auto() {
        let s = Stack::new()
            .child(Text::new("A"))
            .child(Text::new("B"))
            .child(Text::new("C"));
        let sizes = s.calculate_sizes(30, 3);
        // Each gets 30/3 = 10
        assert_eq!(sizes, vec![10, 10, 10]);
    }

    #[test]
    fn test_calculate_sizes_all_fixed() {
        let s = Stack::new()
            .child_sized(Text::new("A"), 5)
            .child_sized(Text::new("B"), 10)
            .child_sized(Text::new("C"), 15);
        let sizes = s.calculate_sizes(100, 3);
        assert_eq!(sizes, vec![5, 10, 15]);
    }

    #[test]
    fn test_calculate_sizes_insufficient_space() {
        let s = Stack::new()
            .child_sized(Text::new("A"), 50)
            .child(Text::new("B"));
        let sizes = s.calculate_sizes(30, 2);
        // Fixed: 50, but available is only 30
        // Auto gets minimal 1
        assert_eq!(sizes[0], 50); // Fixed size preserved
        assert_eq!(sizes[1], 1); // Auto gets minimal
    }
}
