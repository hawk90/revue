//! Stack container widget

mod sizing;

use crate::layout::Rect;
use crate::widget::traits::{View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Size specification for a stack child
#[derive(Clone, Copy, Debug)]
enum ChildSize {
    /// Unsized: its content size when the stack is content-sized and the
    /// child measures without filling the axis, else an equal share of the
    /// remaining space
    Auto,
    /// Fixed pixel size
    Fixed(u16),
    /// The size a content-sized child measured (its CSS box folded in).
    /// Like `Fixed`, except that when the measured and fixed children
    /// together overflow the stack, the measured ones shrink to fit - see
    /// [`Stack::calculate_sizes`]
    Content(u16),
    /// Flex grow factor (proportional distribution of remaining space)
    Flex(f32),
}

/// A stack container for layout
///
/// Children are laid end to end along the stack's axis - top to bottom in a
/// [`vstack`], left to right in an [`hstack`]. A child added with
/// [`child`](Self::child) takes the size of its content when it can measure
/// itself (see [`content_sized`](Self::content_sized), on by default) and
/// otherwise shares the space left over with the other children that fill.
/// [`child_sized`](Self::child_sized) and [`child_flex`](Self::child_flex)
/// say exactly how much a child gets.
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
    /// Size unsized children to their content - see [`Stack::content_sized`].
    content_sized: bool,
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
            content_sized: true,
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

    /// Give each child added with [`child`](Self::child) the size of its
    /// content instead of an equal share of the space.
    ///
    /// **On by default** since 3.0. A child whose [`View::measure`] answers
    /// gets that size along the stack's axis, and only the children that fill
    /// share what is left - equally. A child fills when `measure` returns
    /// `None` or when [`View::fills`] covers the stack's axis: a text field in
    /// a row takes the width its siblings leave, while in a column it is still
    /// the one row it measures. `child_sized` and `child_flex` still win. The
    /// cross axis is unchanged: every child gets the stack's full width (in a
    /// column) or height (in a row).
    ///
    /// `content_sized(false)` restores the 2.x rule: every unsized child gets
    /// an equal share of what the sized ones left, so
    /// `vstack().content_sized(false).child(Text::new("a")).child(Text::new("b"))`
    /// puts `b` halfway down the screen.
    ///
    /// A layout whose body is itself content-sized (text, a bordered box of
    /// text) and should still take the rest of the screen adds the body with
    /// [`child_flex`](Self::child_flex):
    ///
    /// ```rust,ignore
    /// vstack()
    ///     .child(Text::new("header"))
    ///     .child_flex(Border::single().child(Text::new("body")), 1.0)
    ///     .child(Text::new("footer"))
    /// ```
    ///
    /// Under [`css_layout`](crate::core::app::AppBuilder::css_layout), each
    /// such child's CSS box counts along the stack's axis: an explicit
    /// `height` (column) or `width` (row) replaces the measured size,
    /// `min-*`/`max-*` clamp it, and the margins on that axis are added
    /// around it - so `margin-top: 2` on a one-row `Text` pushes it down two
    /// rows instead of insetting it out of sight. A child whose stylesheet
    /// says `display: none` takes no space and no gap. A percentage on the
    /// axis leaves the child filling, and builder sizes still win. (An
    /// equal-share stack is unchanged: the box is applied to the share.)
    pub fn content_sized(mut self, enabled: bool) -> Self {
        self.content_sized = enabled;
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

impl_styled_view!(Stack);
impl_props_builders!(Stack);

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
}
