//! Each child's size rule (builder, content, CSS box), the styled subtrees,
//! and the lines of a wrapping row

use super::axis_box::AxisBox;
use super::measure::measure_child;
use super::{ChildSize, Direction, Stack};
use crate::layout::Rect;
use crate::style::{Display, FlexWrap, Style};
use crate::widget::traits::render_context::{box_model, StyledSubtree};
use crate::widget::traits::{RenderContext, View};
impl Stack {
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
    pub(super) fn effective_sizes(
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
    ///   bounds then apply to its share, as in an equal-share stack, and the
    ///   share keeps room for its minimum, see [`share_of`](Self::share_of)
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
    pub(super) fn content_size(
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
            None if self.fills_main_axis(child) => return self.share_of(child, &along),
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
                    None => return self.share_of(child, &along),
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

    /// The share a filling child takes: an equal one, with its minimum if it
    /// asks for one.
    ///
    /// What the view asks for ([`View::min_size`]) goes through the CSS box
    /// like a measured size, so `max-*` caps it and `min-*` raises it, and the
    /// margins are added, as they sit in the share too. Only `max-*` and the
    /// margins of a view that asks for nothing stay inside an ordinary share.
    ///
    /// With a CSS minimum the box model puts the size back whatever the share
    /// is, so it may not give way; a minimum the view asks for may.
    pub(super) fn share_of(&self, child: &dyn View, along: &AxisBox) -> ChildSize {
        let (width, height) = child.min_size();
        let least = self.oriented(width, height).0;
        match (along.clamp(least), along.min) {
            (0, _) => ChildSize::Auto,
            (size, Some(_)) => ChildSize::AutoAtLeast(along.slot(size)),
            (size, None) => ChildSize::AutoMin(along.slot(size)),
        }
    }

    /// Whether `child` takes whatever it is offered along this stack's axis
    /// ([`View::fills`]): the width in a row, the height in a column.
    pub(super) fn fills_main_axis(&self, child: &dyn View) -> bool {
        let fills = child.fills();
        match self.direction {
            Direction::Column => fills.height,
            Direction::Row => fills.width,
        }
    }

    /// Swap `(main, cross)` into `(width, height)` for this stack - or back:
    /// the swap is its own inverse.
    pub(super) fn oriented(&self, a: u16, b: u16) -> (u16, u16) {
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
    pub(super) fn child_subtrees<'s>(
        &self,
        ctx: &RenderContext<'s>,
    ) -> Vec<Option<StyledSubtree<'s>>> {
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
    pub(super) fn align_subtrees<'s>(
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
    pub(super) fn wrapped_lines(
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
    pub(super) fn wrapped_item_height(
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
}

/// Does this computed style say `display: none`?
pub(super) fn is_hidden(style: Option<&Style>) -> bool {
    style.is_some_and(|s| s.layout.display == Display::None)
}

/// Does this computed style make a row wrap? The same test as
/// [`RenderContext::css_flex_wrap`].
pub(super) fn wraps(style: &Style) -> bool {
    style.layout.flex_wrap != FlexWrap::NoWrap
}

/// Where each child of a row goes: `(line, x)`.
///
/// Children are laid end to end with `gap` between them; one that would
/// cross `width` starts a new line, unless it is the first on its line. A
/// hidden child takes no space and no gap. `width` = `u16::MAX` never wraps.
pub(super) fn row_places(
    widths: &[u16],
    hidden: &[bool],
    width: u16,
    gap: u16,
) -> Vec<(usize, u16)> {
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
