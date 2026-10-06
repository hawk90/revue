//! Stack container widget

use crate::layout::Rect;
use crate::style::{Display, Size, Style};
use crate::widget::traits::render_context::box_model;
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
            content_sized: false,
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
    /// **Off by default** in 2.x; 3.0 turns it on. With it off, every unsized
    /// child gets an equal share of what the sized ones left, so
    /// `vstack().child(Text::new("a")).child(Text::new("b"))` puts `b` halfway
    /// down the screen.
    ///
    /// With it on, a child whose [`View::measure`] answers gets that size
    /// along the stack's axis, and only the children that fill (`measure`
    /// returns `None`) share what is left - equally, as before. `child_sized`
    /// and `child_flex` still win. The cross axis is unchanged: every child
    /// gets the stack's full width (in a column) or height (in a row).
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
        // CSS wins when it specified one; `gap: 0` is the initial value and so
        // reads as "not specified".
        let gap = ctx.gap_or(self.gap);

        let wrap = ctx.css_flex_wrap();
        let styles = self.child_styles(ctx, wrap);
        let hidden: Vec<bool> = styles
            .iter()
            .map(|s| s.is_some_and(|s| s.layout.display == Display::None))
            .collect();
        // A hidden child takes no gap either.
        let shown = hidden.iter().filter(|h| !**h).count();
        let total_gap = gap.saturating_mul(shown.saturating_sub(1) as u16);

        match self.direction {
            Direction::Row => {
                let available_width = area.width.saturating_sub(total_gap);
                let sizes = self.effective_sizes(available_width, area.height, &styles);
                let widths = Self::calculate_sizes(&sizes, available_width, n);

                let mut x: u16 = 0;
                let mut y: u16 = 0;
                let row_height = if wrap { area.height / 2 } else { area.height };

                for (i, child) in self.children.iter().enumerate() {
                    let w = widths[i];

                    // Wrap to next row if needed
                    if wrap && x > 0 && x.saturating_add(w) > area.width {
                        x = 0;
                        y = y.saturating_add(row_height).saturating_add(gap);
                        if y >= area.height {
                            break;
                        }
                    }

                    if child.needs_render() {
                        let child_area = ctx.sub_area(x, y, w, row_height);
                        ctx.render_child_with_overflow(
                            child.as_ref(),
                            child_area,
                            overflow_hidden,
                            parent_clip,
                        );
                    }
                    if !hidden[i] {
                        x = x.saturating_add(w).saturating_add(gap);
                    }
                }
            }
            Direction::Column => {
                let available_height = area.height.saturating_sub(total_gap);
                let sizes = self.effective_sizes(available_height, area.width, &styles);
                let heights = Self::calculate_sizes(&sizes, available_height, n);

                let mut y: u16 = 0;
                for (i, child) in self.children.iter().enumerate() {
                    let h = heights[i];
                    if child.needs_render() {
                        let child_area = ctx.sub_area(0, y, area.width, h);
                        ctx.render_child_with_overflow(
                            child.as_ref(),
                            child_area,
                            overflow_hidden,
                            parent_clip,
                        );
                    }
                    if !hidden[i] {
                        y = y.saturating_add(h).saturating_add(gap);
                    }
                }
            }
        }
    }

    /// Children laid end to end along the axis, plus the gaps; as wide (in a
    /// column) or tall (in a row) as the largest. A child that fills, or one
    /// added with `child_flex`, makes the whole stack fill.
    ///
    /// This is the stack's content whether or not it is
    /// [`content_sized`](Self::content_sized) itself - the flag decides how it
    /// lays out its children, not how large its children are.
    ///
    /// It does not include the children's CSS box: `measure` has no render
    /// context, so the children's computed styles are out of reach. A
    /// content-sized stack nested in another sizes itself from its children's
    /// bare content, and only its own CSS box is folded in by its parent.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let gaps = self
            .gap
            .saturating_mul(self.children.len().saturating_sub(1) as u16);
        let (mut main, mut cross) = (gaps, 0u16);
        for (child, size) in self.children.iter().zip(&self.child_sizes) {
            let (w, h) = match (size, self.direction) {
                (ChildSize::Flex(_), _) => return None,
                (ChildSize::Fixed(n), Direction::Column) => {
                    (child.measure(max_width, *n).map_or(max_width, |m| m.0), *n)
                }
                (ChildSize::Fixed(n), Direction::Row) => (
                    *n,
                    child.measure(*n, max_height).map_or(max_height, |m| m.1),
                ),
                (ChildSize::Auto, _) => child.measure(max_width, max_height)?,
            };
            let (along, across) = match self.direction {
                Direction::Column => (h, w),
                Direction::Row => (w, h),
            };
            main = main.saturating_add(along);
            cross = cross.max(across);
        }
        let (w, h) = match self.direction {
            Direction::Column => (cross, main),
            Direction::Row => (main, cross),
        };
        let w = w.max(self.min_width);
        let h = h.max(self.min_height);
        let w = if self.max_width > 0 {
            w.min(self.max_width)
        } else {
            w
        };
        let h = if self.max_height > 0 {
            h.min(self.max_height)
        } else {
            h
        };
        Some((w.min(max_width), h.min(max_height)))
    }

    fn children(&self) -> &[Box<dyn View>] {
        &self.children
    }

    crate::impl_view_meta!("Stack");
}

impl_styled_view!(Stack);
impl_props_builders!(Stack);

impl Stack {
    /// The size rule each child is laid out by.
    ///
    /// Without [`content_sized`](Self::content_sized) that is what the builder
    /// recorded. With it, an unsized child that can measure itself is laid out
    /// as if it had been added with `child_sized` at its measured size - its
    /// CSS box folded in, see [`content_size`](Self::content_size).
    ///
    /// A child whose computed style (see [`child_styles`](Self::child_styles))
    /// is `display: none` takes no space at all.
    fn effective_sizes(&self, main: u16, cross: u16, styles: &[Option<&Style>]) -> Vec<ChildSize> {
        if !self.content_sized {
            return self.child_sizes.clone();
        }
        self.children
            .iter()
            .zip(&self.child_sizes)
            .zip(styles)
            .map(|((child, size), style)| match size {
                _ if style.is_some_and(|s| s.layout.display == Display::None) => {
                    ChildSize::Fixed(0)
                }
                ChildSize::Auto => self.content_size(child.as_ref(), *style, main, cross),
                other => *other,
            })
            .collect()
    }

    /// The slot an unsized child gets in a content-sized stack.
    ///
    /// The paint pass applies the child's CSS box to whatever area the stack
    /// hands it (`box_model::apply`: margins inset, then `height`/`width`
    /// replaces, then `min-*`/`max-*` clamp). Handing it its bare content size
    /// would let that run on a box already the size of the content - a
    /// `margin-top` insets one row to nothing, a `height: 3` grows over the
    /// next sibling.
    ///
    /// So the box is folded in here, along the stack's axis, and the slot is
    /// chosen so that the box model, run afterwards, lands exactly on it:
    ///
    /// 1. `size` = the explicit `height` (column) / `width` (row) if there is
    ///    one, else the measured content - measured within the cross extent
    ///    the box model will give the child, minus the main-axis margins
    /// 2. clamp `size` by `max-*`, then `min-*` - the box model's order
    /// 3. slot = margin before + `size` + margin after
    ///
    /// The box model then insets the slot by the same margins and gets
    /// `size` back; replacing with the same explicit size and clamping by the
    /// same bounds changes nothing. Nothing is applied twice - the stack
    /// *reserves*, the box model *places*. The cross axis is left entirely to
    /// the box model, as before.
    ///
    /// A child keeps filling (`ChildSize::Auto`, an equal share of what is
    /// left) when:
    ///
    /// - it does not measure itself and has no explicit size - its margins and
    ///   bounds then apply to its share, as in an equal-share stack
    /// - any main-axis size is a percentage. A percentage needs a basis and the
    ///   box model resolves it against the slot, so no slot computed from it
    ///   would survive the box model unchanged; the share is the basis an
    ///   equal-share stack would have used
    ///
    /// Builder sizes (`child_sized`, `child_flex`) never reach here: the
    /// builder outranks the stylesheet, so their slot is what the builder said
    /// and the box model adjusts inside it, as before.
    fn content_size(
        &self,
        child: &dyn View,
        style: Option<&Style>,
        main: u16,
        cross: u16,
    ) -> ChildSize {
        let style = style.filter(|s| box_model::specifies_anything(s));
        let Some(style) = style else {
            let (w, h) = self.oriented(main, cross);
            return match child.measure(w, h) {
                Some((w, h)) => ChildSize::Fixed(self.oriented(w, h).0),
                None => ChildSize::Auto,
            };
        };

        let (sizing, margin) = (&style.sizing, &style.spacing.margin);
        let (size, min, max, before, after) = match self.direction {
            Direction::Column => (
                sizing.height,
                sizing.min_height,
                sizing.max_height,
                margin.top,
                margin.bottom,
            ),
            Direction::Row => (
                sizing.width,
                sizing.min_width,
                sizing.max_width,
                margin.left,
                margin.right,
            ),
        };
        if [size, min, max]
            .iter()
            .any(|s| matches!(s, Size::Percent(_)))
        {
            return ChildSize::Auto;
        }
        let fixed = |s: Size| match s {
            Size::Fixed(v) => Some(v),
            _ => None,
        };
        let margins = before.saturating_add(after);

        let size = match fixed(size) {
            Some(size) => size,
            None => {
                // The cross extent is what the box model will leave the child;
                // a narrower box can wrap to more rows.
                let (w, h) = self.oriented(main, cross);
                let boxed = box_model::apply(style, Rect::new(0, 0, w, h));
                let cross = self.oriented(boxed.width, boxed.height).1;
                let (w, h) = self.oriented(main.saturating_sub(margins), cross);
                match child.measure(w, h) {
                    Some((w, h)) => self.oriented(w, h).0,
                    None => return ChildSize::Auto,
                }
            }
        };
        let size = fixed(max).map_or(size, |m| size.min(m));
        let size = fixed(min).map_or(size, |m| size.max(m));
        ChildSize::Fixed(size.saturating_add(margins))
    }

    /// Swap `(main, cross)` into `(width, height)` for this stack - or back:
    /// the swap is its own inverse.
    fn oriented(&self, a: u16, b: u16) -> (u16, u16) {
        match self.direction {
            Direction::Column => (b, a),
            Direction::Row => (a, b),
        }
    }

    /// Each child's computed style, for a [`content_sized`](Self::content_sized)
    /// stack to lay out by - `None` for every child otherwise.
    ///
    /// Equal-share stacks do not look: there the CSS box is applied to the
    /// share afterwards, as it always was. Neither does a wrapping row, which
    /// may stop rendering partway along when it runs out of rows - the peek is
    /// only sound for children the stack is sure to render (see
    /// [`RenderContext::peek_child_styles`]).
    ///
    /// A child that does not need rendering is never handed to
    /// `render_child`, so it has no node, and is skipped in the walk.
    fn child_styles<'s>(&self, ctx: &RenderContext<'s>, wrap: bool) -> Vec<Option<&'s Style>> {
        let n = self.children.len();
        if !self.content_sized || (wrap && self.direction == Direction::Row) {
            return vec![None; n];
        }
        let rendered = self.children.iter().filter(|c| c.needs_render()).count();
        let mut peeked = ctx.peek_child_styles(rendered).into_iter();
        self.children
            .iter()
            .map(|c| {
                if c.needs_render() {
                    peeked.next().flatten()
                } else {
                    None
                }
            })
            .collect()
    }

    /// Calculate sizes for children based on available space
    ///
    /// Strategy:
    /// - Fixed children get their exact size
    /// - Flex children share remaining space proportionally by grow factor
    /// - Auto children share remaining space equally (after flex allocation)
    fn calculate_sizes(child_sizes: &[ChildSize], available: u16, n: usize) -> Vec<u16> {
        if n == 0 {
            return Vec::new();
        }

        // First pass: calculate fixed space and collect flex/auto info
        let mut auto_count: usize = 0;
        let mut total_grow: f32 = 0.0;
        let mut fixed_total = 0u16;

        for cs in child_sizes {
            match cs {
                ChildSize::Fixed(size) => fixed_total = fixed_total.saturating_add(*size),
                ChildSize::Flex(grow) => total_grow += grow,
                ChildSize::Auto => auto_count += 1,
            }
        }

        let remaining = available.saturating_sub(fixed_total);
        let mut result = vec![0u16; n];

        // Assign fixed sizes
        for (i, cs) in child_sizes.iter().enumerate() {
            if let ChildSize::Fixed(size) = cs {
                result[i] = *size;
            }
        }

        if total_grow > 0.0 {
            // Distribute remaining space to flex children proportionally
            let flex_indices: Vec<usize> = child_sizes
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
                let grow = match child_sizes[i] {
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
            for (i, cs) in child_sizes.iter().enumerate() {
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
            for (i, cs) in child_sizes.iter().enumerate() {
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
        let sizes = Stack::calculate_sizes(&s.child_sizes, 30, 3);
        assert_eq!(sizes.len(), 3);
        assert_eq!(sizes.iter().sum::<u16>(), 30);
    }

    #[test]
    fn test_stack_calculate_sizes_fixed() {
        let s = Stack::new()
            .child_sized(Text::new("A"), 10)
            .child_sized(Text::new("B"), 20);
        let sizes = Stack::calculate_sizes(&s.child_sizes, 50, 2);
        assert_eq!(sizes, vec![10, 20]);
    }

    #[test]
    fn test_stack_calculate_sizes_flex() {
        let s = Stack::new()
            .child_flex(Text::new("A"), 1.0)
            .child_flex(Text::new("B"), 2.0);
        let sizes = Stack::calculate_sizes(&s.child_sizes, 30, 2);
        assert_eq!(sizes[0], 10);
        assert_eq!(sizes[1], 20);
    }

    #[test]
    fn test_stack_calculate_sizes_mixed() {
        let s = Stack::new()
            .child_sized(Text::new("Fixed"), 10)
            .child_flex(Text::new("Flex"), 1.0);
        let sizes = Stack::calculate_sizes(&s.child_sizes, 30, 2);
        assert_eq!(sizes[0], 10);
        assert_eq!(sizes[1], 20); // 30 - 10 = 20 (no auto children)
    }

    #[test]
    fn test_stack_calculate_sizes_empty() {
        let s = Stack::new();
        let sizes = Stack::calculate_sizes(&s.child_sizes, 100, 0);
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

    #[test]
    fn test_stack_row_no_wrap_overflow() {
        // Without wrap, children extend beyond area (clipped by area bounds)
        let mut buf = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buf, area);
        let s = hstack()
            .child_sized(Text::new("AAAA"), 6)
            .child_sized(Text::new("BBBB"), 6);
        s.render(&mut ctx);
        // First child renders, second starts at x=6 but is clipped at width 10
        assert_eq!(buf.get(0, 0).unwrap().symbol, 'A');
    }

    #[test]
    fn test_stack_needs_render_skip() {
        // Verify Stack respects needs_render
        let mut buf = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let s = vstack()
            .child(Text::new("Visible"))
            .child(Text::new("Also visible"));
        let mut ctx = RenderContext::new(&mut buf, area);
        s.render(&mut ctx);
        assert_eq!(buf.get(0, 0).unwrap().symbol, 'V');
    }
}
