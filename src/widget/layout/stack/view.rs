//! The `View` impl: render, measure and what the stack fills

use super::layout::{is_hidden, row_places};
use super::{ChildSize, Direction, Stack};
use crate::layout::Rect;
use crate::widget::traits::render_context::StyledSubtree;
use crate::widget::traits::{Fill, RenderContext, View};
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

        // CSS wins when it specified one, `gap: 0` included; a stylesheet that
        // says nothing leaves the builder's gap.
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

        let pass = Pass {
            area,
            gap,
            total_gap,
            subtrees: &subtrees,
            hidden: &hidden,
            overflow_hidden,
            parent_clip,
        };
        match self.direction {
            Direction::Row => self.render_row(ctx, &pass, wrap),
            Direction::Column => self.render_column(ctx, &pass),
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

    /// What its children ask for while filling: their sum along the axis, the
    /// largest across it, so a minimum survives a nested stack. Children with
    /// a builder size do not count: the builder decides theirs.
    fn min_size(&self) -> (u16, u16) {
        let column = self.direction == Direction::Column;
        let (mut along, mut across) = (0u16, 0u16);
        for (child, _) in self
            .children
            .iter()
            .zip(&self.child_sizes)
            .filter(|(_, size)| matches!(size, ChildSize::Auto))
        {
            let (w, h) = child.min_size();
            let (main, cross) = if column { (h, w) } else { (w, h) };
            along = along.saturating_add(main);
            across = across.max(cross);
        }
        if column {
            (across, along)
        } else {
            (along, across)
        }
    }

    fn children(&self) -> &[Box<dyn View>] {
        &self.children
    }

    crate::impl_view_meta!("Stack");
}

impl Stack {
    /// A row: widths shared along it, or lines when it wraps
    fn render_row(&self, ctx: &mut RenderContext, pass: &Pass<'_, '_>, wrap: bool) {
        let Pass {
            area,
            gap,
            total_gap,
            subtrees,
            hidden,
            overflow_hidden,
            parent_clip,
        } = *pass;
        let n = self.children.len();
        let available_width = area.width.saturating_sub(total_gap);
        // Each item of a wrapping row may take a whole line.
        let measure_width = if wrap { area.width } else { available_width };
        let mut sizes = self.effective_sizes(measure_width, area.height, subtrees);
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
        let places = row_places(&widths, hidden, wrap_width, gap);
        let lines = if wrap {
            self.wrapped_lines(area, &widths, hidden, &places, subtrees, gap)
        } else {
            vec![(0, area.height)]
        };

        for (i, child) in self.children.iter().enumerate() {
            let (line, x) = places[i];
            // A line that starts below the stack is not drawn, nor is
            // anything after it.
            let Some(&(y, row_height)) = lines.get(line).filter(|l| l.0 < area.height) else {
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

    /// A column: heights shared down it
    fn render_column(&self, ctx: &mut RenderContext, pass: &Pass<'_, '_>) {
        let Pass {
            area,
            gap,
            total_gap,
            subtrees,
            hidden,
            overflow_hidden,
            parent_clip,
        } = *pass;
        let n = self.children.len();
        let available_height = area.height.saturating_sub(total_gap);
        let sizes = self.effective_sizes(available_height, area.width, subtrees);
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

/// What one render pass hands the row or column layout: the area after the
/// stack's constraints, the gap (and all gaps), each child's subtree and
/// whether it is hidden, and the overflow clip
#[derive(Clone, Copy)]
struct Pass<'a, 's> {
    area: Rect,
    gap: u16,
    total_gap: u16,
    subtrees: &'a [Option<StyledSubtree<'s>>],
    hidden: &'a [bool],
    overflow_hidden: bool,
    parent_clip: Option<Rect>,
}
