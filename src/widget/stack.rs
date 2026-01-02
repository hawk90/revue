//! Stack container widget

use super::traits::{RenderContext, View, WidgetProps};
use crate::layout::Rect;
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

                    x += w + self.gap;
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

                    y += h + self.gap;
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
}
