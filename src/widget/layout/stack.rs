//! Stack container widget

use crate::layout::Rect;
use crate::style::{Display, FlexWrap, Size, Style};
use crate::widget::traits::render_context::{box_model, StyledSubtree};
use crate::widget::traits::{Fill, RenderContext, View, WidgetProps};
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
        let subtrees = self.child_subtrees(ctx);
        let hidden: Vec<bool> = subtrees
            .iter()
            .map(|s| is_hidden(s.and_then(|s| s.style())))
            .collect();
        // A hidden child takes no gap either.
        let shown = hidden.iter().filter(|h| !**h).count();
        let total_gap = gap.saturating_mul(shown.saturating_sub(1) as u16);

        match self.direction {
            Direction::Row => {
                let available_width = area.width.saturating_sub(total_gap);
                // Each item of a wrapping row may take a whole line.
                let measure_width = if wrap { area.width } else { available_width };
                let mut sizes = self.effective_sizes(measure_width, area.height, &subtrees);
                if wrap {
                    // A wrapping row moves what does not fit to the next row
                    // instead of shrinking it.
                    for size in &mut sizes {
                        if let ChildSize::Content(n) = *size {
                            *size = ChildSize::Fixed(n);
                        }
                    }
                }
                let widths = Self::calculate_sizes(&sizes, available_width, n);
                let wrap_width = if wrap { area.width } else { u16::MAX };
                let places = row_places(&widths, &hidden, wrap_width, gap);
                let lines = if wrap {
                    self.wrapped_lines(area, &widths, &hidden, &places, &subtrees, gap)
                } else {
                    vec![(0, area.height)]
                };

                for (i, child) in self.children.iter().enumerate() {
                    let (line, x) = places[i];
                    // A line that starts below the stack is not drawn, nor is
                    // anything after it.
                    let Some(&(y, row_height)) = lines.get(line).filter(|l| l.0 < area.height)
                    else {
                        break;
                    };

                    if child.needs_render() {
                        let child_area = ctx.sub_area(x, y, widths[i], row_height);
                        ctx.render_child_with_overflow(
                            child.as_ref(),
                            child_area,
                            overflow_hidden,
                            parent_clip,
                        );
                    }
                }
            }
            Direction::Column => {
                let available_height = area.height.saturating_sub(total_gap);
                let sizes = self.effective_sizes(available_height, area.width, &subtrees);
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
    /// This is the children's bare content: `measure` has no render context,
    /// so their computed styles are out of reach. A content-sized parent
    /// stack calls [`measure_styled`](View::measure_styled) instead, which
    /// adds the spacing the stylesheet gives this stack and its descendants.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        self.measure_with(max_width, max_height, None)
    }

    /// [`measure`](View::measure), plus what the stylesheet adds inside this
    /// stack: its own CSS `gap`, and each child's CSS box - the same rule a
    /// content-sized stack lays its children out by (see
    /// [`content_sized`](Self::content_sized)): an explicit size replaces the
    /// measured one, `min-*`/`max-*` clamp it and the margins are added
    /// around it, on both axes. A child that is itself a stack is measured
    /// the same way, so this recurses through every level of nesting. A child
    /// whose stylesheet says `display: none` takes no space and no gap.
    ///
    /// A percentage size on a child has no basis here, so the stack fills
    /// (`None`) rather than guess.
    fn measure_styled(
        &self,
        max_width: u16,
        max_height: u16,
        subtree: StyledSubtree<'_>,
    ) -> Option<(u16, u16)> {
        self.measure_with(max_width, max_height, Some(subtree))
    }

    /// The axes any child added with [`child`](Self::child) fills.
    ///
    /// A row holding a text field fills the width, so an outer row shares
    /// its leftover with it rather than handing it the width the field
    /// measured. Children added with `child_sized` / `child_flex` do not
    /// count: their size along the axis is the builder's, and a flex child
    /// already makes the stack's `measure` answer `None`.
    fn fills(&self) -> Fill {
        self.children
            .iter()
            .zip(&self.child_sizes)
            .filter(|(_, size)| matches!(size, ChildSize::Auto))
            .fold(Fill::NONE, |acc, (child, _)| acc.or(child.fills()))
    }

    fn children(&self) -> &[Box<dyn View>] {
        &self.children
    }

    crate::impl_view_meta!("Stack");
}

impl_styled_view!(Stack);
impl_props_builders!(Stack);

impl Stack {
    /// [`measure`](View::measure) when `subtree` is `None`,
    /// [`measure_styled`](View::measure_styled) otherwise.
    ///
    /// Children laid end to end along the axis, plus the gaps; as wide (in a
    /// column) or tall (in a row) as the largest - each child's extent being
    /// its CSS box around its content when its style is known.
    fn measure_with(
        &self,
        max_width: u16,
        max_height: u16,
        subtree: Option<StyledSubtree<'_>>,
    ) -> Option<(u16, u16)> {
        let own = subtree.and_then(|s| s.style());
        let gap = own.and_then(|s| s.layout.gap).unwrap_or(self.gap);
        let subtrees = match subtree {
            Some(subtree) => self.align_subtrees(subtree.children()),
            None => vec![None; self.children.len()],
        };
        let column = self.direction == Direction::Column;
        let wrap = !column && own.is_some_and(wraps);
        let (max_main, max_cross) = self.oriented(max_width, max_height);

        // Each shown child's (main, cross) extent; `None` for a hidden one.
        let mut items = Vec::with_capacity(self.children.len());
        for ((child, size), sub) in self.children.iter().zip(&self.child_sizes).zip(&subtrees) {
            let style = sub.and_then(|s| s.style());
            if is_hidden(style) {
                items.push(None);
                continue;
            }
            let along = AxisBox::of(style, column);
            let across = AxisBox::of(style, !column);
            if along.percent || across.percent {
                return None;
            }
            // What the box model will leave the child's content on each axis.
            let inner_cross = across.clamp(
                across
                    .size
                    .unwrap_or(max_cross.saturating_sub(across.margins)),
            );
            let (child_main, child_cross) = match size {
                ChildSize::Flex(_) => return None,
                ChildSize::Fixed(n) => {
                    let (w, h) = self.oriented(*n, inner_cross);
                    let measured = measure_child(child.as_ref(), *sub, w, h);
                    let content = across.size.or(measured.map(|(w, h)| self.oriented(w, h).1));
                    (*n, content.map_or(max_cross, |c| across.slot(c)))
                }
                // `child_sizes` holds what the builder recorded; `Content` is
                // only ever produced while laying out.
                ChildSize::Auto | ChildSize::Content(_) => {
                    let inner_main =
                        along.clamp(along.size.unwrap_or(max_main.saturating_sub(along.margins)));
                    let (w, h) = self.oriented(inner_main, inner_cross);
                    let measured =
                        measure_child(child.as_ref(), *sub, w, h).map(|(w, h)| self.oriented(w, h));
                    let content_main = along.size.or(measured.map(|m| m.0))?;
                    let content_cross = across.size.or(measured.map(|m| m.1))?;
                    (along.slot(content_main), across.slot(content_cross))
                }
            };
            items.push(Some((child_main, child_cross)));
        }

        let (w, h) = if wrap {
            // A wrapping row: its lines within `max_width`, as `render` breaks
            // them, stacked with the gap between them.
            let widths: Vec<u16> = items.iter().map(|i| i.map_or(0, |i| i.0)).collect();
            let hidden: Vec<bool> = items.iter().map(Option::is_none).collect();
            let places = row_places(&widths, &hidden, max_width, gap);
            let lines = places.iter().map(|p| p.0 + 1).max().unwrap_or(0);
            let (mut heights, mut width) = (vec![0u16; lines], 0u16);
            for (item, &(line, x)) in items.iter().zip(&places) {
                if let Some((w, h)) = *item {
                    heights[line] = heights[line].max(h);
                    width = width.max(x.saturating_add(w));
                }
            }
            let gaps = gap.saturating_mul(lines.saturating_sub(1) as u16);
            let height = heights
                .iter()
                .fold(gaps, |total, &h| total.saturating_add(h));
            (width, height)
        } else {
            let shown = items.iter().flatten().count() as u16;
            let gaps = gap.saturating_mul(shown.saturating_sub(1));
            let (main, cross) = items
                .iter()
                .flatten()
                .fold((gaps, 0u16), |(main, cross), &(m, c)| {
                    (main.saturating_add(m), cross.max(c))
                });
            self.oriented(main, cross)
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

    /// The size rule each child is laid out by.
    ///
    /// Without [`content_sized`](Self::content_sized) that is what the builder
    /// recorded. With it, an unsized child that can measure itself is laid out
    /// at its measured size (`ChildSize::Content`: like `child_sized`, except
    /// that it may shrink when the stack overflows) - its CSS box folded in,
    /// see [`content_size`](Self::content_size).
    ///
    /// A child whose computed style (see [`child_subtrees`](Self::child_subtrees))
    /// is `display: none` takes no space at all.
    fn effective_sizes(
        &self,
        main: u16,
        cross: u16,
        subtrees: &[Option<StyledSubtree<'_>>],
    ) -> Vec<ChildSize> {
        if !self.content_sized {
            return self.child_sizes.clone();
        }
        self.children
            .iter()
            .zip(&self.child_sizes)
            .zip(subtrees)
            .map(|((child, size), subtree)| match size {
                _ if is_hidden(subtree.and_then(|s| s.style())) => ChildSize::Fixed(0),
                ChildSize::Auto => self.content_size(child.as_ref(), *subtree, main, cross),
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
    /// The content is measured with [`measure_styled`](View::measure_styled)
    /// when the child's subtree is known, so the spacing CSS adds *inside*
    /// the child - a nested stack's `gap`, its own children's margins - is in
    /// the size too.
    ///
    /// A child keeps filling (`ChildSize::Auto`, an equal share of what is
    /// left) when:
    ///
    /// - it does not measure itself and has no explicit size - its margins and
    ///   bounds then apply to its share, as in an equal-share stack
    /// - it fills this stack's axis ([`View::fills`]) and has no explicit
    ///   size - an explicit `height` (column) / `width` (row) still wins
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
        subtree: Option<StyledSubtree<'_>>,
        main: u16,
        cross: u16,
    ) -> ChildSize {
        let style = subtree
            .and_then(|s| s.style())
            .filter(|s| box_model::specifies_anything(s));
        let along = AxisBox::of(style, self.direction == Direction::Column);
        if along.percent {
            return ChildSize::Auto;
        }

        let size = match along.size {
            Some(size) => size,
            // No explicit size: a child that fills the axis shares what is
            // left, its margins and bounds applying to its share.
            None if self.fills_main_axis(child) => return ChildSize::Auto,
            None => {
                // The cross extent is what the box model will leave the child;
                // a narrower box can wrap to more rows.
                let cross = match style {
                    Some(style) => {
                        let (w, h) = self.oriented(main, cross);
                        let boxed = box_model::apply(style, Rect::new(0, 0, w, h));
                        self.oriented(boxed.width, boxed.height).1
                    }
                    None => cross,
                };
                let (w, h) = self.oriented(main.saturating_sub(along.margins), cross);
                match measure_child(child, subtree, w, h) {
                    Some((w, h)) => self.oriented(w, h).0,
                    None => return ChildSize::Auto,
                }
            }
        };
        let slot = along.slot(size);
        // An explicit size or minimum is put back by the box model whatever
        // slot the child gets, so shrinking that slot would only make the
        // next sibling start inside it. Only a measured size may shrink.
        if along.pinned() {
            ChildSize::Fixed(slot)
        } else {
            ChildSize::Content(slot)
        }
    }

    /// Whether `child` takes whatever it is offered along this stack's axis
    /// ([`View::fills`]): the width in a row, the height in a column.
    fn fills_main_axis(&self, child: &dyn View) -> bool {
        let fills = child.fills();
        match self.direction {
            Direction::Column => fills.height,
            Direction::Row => fills.width,
        }
    }

    /// Swap `(main, cross)` into `(width, height)` for this stack - or back:
    /// the swap is its own inverse.
    fn oriented(&self, a: u16, b: u16) -> (u16, u16) {
        match self.direction {
            Direction::Column => (b, a),
            Direction::Row => (a, b),
        }
    }

    /// Each child's subtree - its computed style and its descendants' - for a
    /// [`content_sized`](Self::content_sized) stack to lay out by; `None` for
    /// every child otherwise.
    ///
    /// Equal-share stacks do not look: there the CSS box is applied to the
    /// share afterwards, as it always was.
    ///
    /// A wrapping row looks too, although it may stop painting partway along
    /// when it runs out of lines. What the peek needs (see
    /// [`RenderContext::peek_child_subtrees`]) is that the *collect* pass
    /// rendered every child, and it did: `flex-wrap` is CSS, the collect pass
    /// has no computed styles, so there every row is a plain row that renders
    /// all of its children. Stopping early in the paint pass only leaves nodes
    /// unvisited, which the parent's resynchronization steps over.
    fn child_subtrees<'s>(&self, ctx: &RenderContext<'s>) -> Vec<Option<StyledSubtree<'s>>> {
        if !self.content_sized {
            return vec![None; self.children.len()];
        }
        let rendered = self.children.iter().filter(|c| c.needs_render()).count();
        self.align_subtrees(ctx.peek_child_subtrees(rendered).into_iter().flatten())
    }

    /// Match `subtrees`, in render order, to the children they belong to.
    ///
    /// A child that does not need rendering is never handed to
    /// `render_child`, so it has no node and is skipped. A child past the end
    /// of `subtrees` gets `None`.
    fn align_subtrees<'s>(
        &self,
        subtrees: impl IntoIterator<Item = StyledSubtree<'s>>,
    ) -> Vec<Option<StyledSubtree<'s>>> {
        let mut subtrees = subtrees.into_iter();
        self.children
            .iter()
            .map(|c| {
                if c.needs_render() {
                    subtrees.next()
                } else {
                    None
                }
            })
            .collect()
    }

    /// Each line of a wrapping row: `(y, height)`, top to bottom.
    ///
    /// A line is as tall as its tallest item - an item's measured height with
    /// its vertical CSS box folded in, as [`measure_styled`](View::measure_styled)
    /// reports it - and the lines are stacked with `gap` between them. An item
    /// that does not measure fills, so its line takes the rest of the height.
    /// A line past the bottom gets no height.
    ///
    /// An equal-share stack keeps the 2.x rule - every line half the height.
    fn wrapped_lines(
        &self,
        area: Rect,
        widths: &[u16],
        hidden: &[bool],
        places: &[(usize, u16)],
        subtrees: &[Option<StyledSubtree<'_>>],
        gap: u16,
    ) -> Vec<(u16, u16)> {
        let count = places.iter().map(|p| p.0 + 1).max().unwrap_or(0);
        if !self.content_sized {
            let h = area.height / 2;
            let step = h.saturating_add(gap);
            return (0..count)
                .map(|line| (step.saturating_mul(line.min(u16::MAX as usize) as u16), h))
                .collect();
        }
        let mut heights = vec![0u16; count];
        for (i, child) in self.children.iter().enumerate() {
            if hidden[i] {
                continue;
            }
            let h = self
                .wrapped_item_height(child.as_ref(), subtrees[i], widths[i], area.height)
                .unwrap_or(u16::MAX);
            let line = places[i].0;
            heights[line] = heights[line].max(h);
        }
        let mut y = 0u16;
        heights
            .into_iter()
            .map(|h| {
                let h = h.min(area.height.saturating_sub(y));
                let line = (y, h);
                y = y.saturating_add(h).saturating_add(gap);
                line
            })
            .collect()
    }

    /// How tall an item of a wrapping row is in a slot `width` wide: its
    /// measured height inside its CSS box, with the box's vertical margins and
    /// bounds - or `None` if it does not measure.
    fn wrapped_item_height(
        &self,
        child: &dyn View,
        subtree: Option<StyledSubtree<'_>>,
        width: u16,
        max_height: u16,
    ) -> Option<u16> {
        let style = subtree.and_then(|s| s.style());
        let vertical = AxisBox::of(style, true);
        if vertical.percent {
            return None;
        }
        let content = match vertical.size {
            Some(size) => size,
            None => {
                let inner_width = width.saturating_sub(AxisBox::of(style, false).margins);
                let inner_height = vertical.clamp(max_height.saturating_sub(vertical.margins));
                measure_child(child, subtree, inner_width, inner_height)?.1
            }
        };
        Some(vertical.slot(content))
    }

    /// Calculate sizes for children based on available space
    ///
    /// Strategy:
    /// - Fixed children get their exact size
    /// - Content (measured) children get their measured size; if together
    ///   with the fixed children they overflow, they shrink to fit what the
    ///   fixed children leave, each in proportion to its size - so a body
    ///   one row too tall loses that row instead of pushing the footer off
    ///   the screen
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
        let mut content_total = 0u32;

        for cs in child_sizes {
            match cs {
                ChildSize::Fixed(size) => fixed_total = fixed_total.saturating_add(*size),
                ChildSize::Content(size) => content_total += u32::from(*size),
                ChildSize::Flex(grow) => total_grow += grow,
                ChildSize::Auto => auto_count += 1,
            }
        }

        let mut result = vec![0u16; n];

        // Assign fixed sizes
        for (i, cs) in child_sizes.iter().enumerate() {
            if let ChildSize::Fixed(size) = cs {
                result[i] = *size;
            }
        }

        // Measured sizes, shrunk to fit if they overflow.
        let content_space = u32::from(available.saturating_sub(fixed_total));
        let content_used =
            Self::fit_content(child_sizes, content_total, content_space, &mut result);
        let remaining = available
            .saturating_sub(fixed_total)
            .saturating_sub(content_used);

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

impl Stack {
    /// Give each `Content` child its measured size, or - when they total more
    /// than `space` - a share of `space` in proportion to that size (CSS
    /// `flex-shrink: 1`). Rounding leftovers go to the earliest children.
    /// Returns the space used.
    fn fit_content(child_sizes: &[ChildSize], total: u32, space: u32, result: &mut [u16]) -> u16 {
        let content = || {
            child_sizes
                .iter()
                .enumerate()
                .filter_map(|(i, cs)| match cs {
                    ChildSize::Content(size) => Some((i, u32::from(*size))),
                    _ => None,
                })
        };
        if total <= space {
            for (i, size) in content() {
                result[i] = size as u16;
            }
            return total as u16;
        }
        let mut used = 0u32;
        for (i, size) in content() {
            let share = size * space / total;
            result[i] = share as u16;
            used += share;
        }
        // Integer division leaves at most one cell per child undistributed.
        for (i, size) in content() {
            if used >= space {
                break;
            }
            if u32::from(result[i]) < size {
                result[i] += 1;
                used += 1;
            }
        }
        used as u16
    }
}

/// Does this computed style say `display: none`?
fn is_hidden(style: Option<&Style>) -> bool {
    style.is_some_and(|s| s.layout.display == Display::None)
}

/// Does this computed style make a row wrap? The same test as
/// [`RenderContext::css_flex_wrap`].
fn wraps(style: &Style) -> bool {
    style.layout.flex_wrap != FlexWrap::NoWrap
}

/// Where each child of a row goes: `(line, x)`.
///
/// Children are laid end to end with `gap` between them; one that would
/// cross `width` starts a new line, unless it is the first on its line. A
/// hidden child takes no space and no gap. `width` = `u16::MAX` never wraps.
fn row_places(widths: &[u16], hidden: &[bool], width: u16, gap: u16) -> Vec<(usize, u16)> {
    let (mut line, mut x) = (0usize, 0u16);
    widths
        .iter()
        .zip(hidden)
        .map(|(&w, &hidden)| {
            if x > 0 && x.saturating_add(w) > width {
                line += 1;
                x = 0;
            }
            let place = (line, x);
            if !hidden {
                x = x.saturating_add(w).saturating_add(gap);
            }
            place
        })
        .collect()
}

/// `child`'s size, with its styled subtree when that is known.
fn measure_child(
    child: &dyn View,
    subtree: Option<StyledSubtree<'_>>,
    max_width: u16,
    max_height: u16,
) -> Option<(u16, u16)> {
    match subtree {
        Some(subtree) => child.measure_styled(max_width, max_height, subtree),
        None => child.measure(max_width, max_height),
    }
}

/// What a child's CSS box says along one axis - see
/// [`Stack::content_size`]. Empty for a child with no box properties.
#[derive(Default)]
struct AxisBox {
    /// Explicit `height` / `width`
    size: Option<u16>,
    /// `min-height` / `min-width`
    min: Option<u16>,
    /// `max-height` / `max-width`
    max: Option<u16>,
    /// Both margins on the axis
    margins: u16,
    /// Any of the three sizes is a percentage
    percent: bool,
}

impl AxisBox {
    /// The box along the vertical axis if `vertical`, else the horizontal one.
    fn of(style: Option<&Style>, vertical: bool) -> Self {
        let Some(style) = style.filter(|s| box_model::specifies_anything(s)) else {
            return Self::default();
        };
        let (sizing, margin) = (&style.sizing, &style.spacing.margin);
        let (size, min, max, before, after) = if vertical {
            (
                sizing.height,
                sizing.min_height,
                sizing.max_height,
                margin.top,
                margin.bottom,
            )
        } else {
            (
                sizing.width,
                sizing.min_width,
                sizing.max_width,
                margin.left,
                margin.right,
            )
        };
        let fixed = |s: Size| match s {
            Size::Fixed(v) => Some(v),
            _ => None,
        };
        Self {
            size: fixed(size),
            min: fixed(min),
            max: fixed(max),
            margins: before.saturating_add(after),
            percent: [size, min, max]
                .iter()
                .any(|s| matches!(s, Size::Percent(_))),
        }
    }

    /// `size` clamped by `max`, then `min` - the box model's order.
    fn clamp(&self, size: u16) -> u16 {
        let size = self.max.map_or(size, |m| size.min(m));
        self.min.map_or(size, |m| size.max(m))
    }

    /// The space the box takes for `content`: clamped, margins added.
    fn slot(&self, content: u16) -> u16 {
        self.clamp(content).saturating_add(self.margins)
    }

    /// Does the box model put the size back whatever slot it gets? An
    /// explicit size or minimum does.
    fn pinned(&self) -> bool {
        self.size.is_some() || self.min.is_some()
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
    fn test_stack_calculate_sizes_content_fits() {
        let sizes = [
            ChildSize::Content(3),
            ChildSize::Auto,
            ChildSize::Content(2),
        ];
        assert_eq!(Stack::calculate_sizes(&sizes, 10, 3), vec![3, 5, 2]);
    }

    /// Measured sizes that overflow shrink in proportion; fixed ones do not.
    #[test]
    fn test_stack_calculate_sizes_content_shrinks() {
        let sizes = [
            ChildSize::Fixed(2),
            ChildSize::Content(12),
            ChildSize::Content(4),
            ChildSize::Fixed(2),
        ];
        // 8 cells for 16 measured: 12 -> 6, 4 -> 2.
        assert_eq!(Stack::calculate_sizes(&sizes, 12, 4), vec![2, 6, 2, 2]);
        // 7 cells: 12*7/16 = 5, 4*7/16 = 1, the leftover cell to the first.
        assert_eq!(Stack::calculate_sizes(&sizes, 11, 4), vec![2, 6, 1, 2]);
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
