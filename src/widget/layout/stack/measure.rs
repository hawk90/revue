//! Measuring: the children laid end to end (or in wrapped lines), with the
//! gaps and their CSS boxes

use super::axis_box::AxisBox;
use super::layout::{is_hidden, row_places, wraps};
use super::{ChildSize, Direction, Stack};
use crate::widget::traits::render_context::StyledSubtree;
use crate::widget::traits::View;
impl Stack {
    /// [`measure`](View::measure) when `subtree` is `None`,
    /// [`measure_styled`](View::measure_styled) otherwise.
    ///
    /// Children laid end to end along the axis, plus the gaps; as wide (in a
    /// column) or tall (in a row) as the largest - each child's extent being
    /// its CSS box around its content when its style is known.
    pub(super) fn measure_with(
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
                ChildSize::Auto
                | ChildSize::AutoMin(_)
                | ChildSize::AutoAtLeast(_)
                | ChildSize::Content(_) => {
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

        let (w, h) = self.extent(&items, wrap, max_width, gap);
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

    /// The children's extent: a wrapping row's lines within `max_width`,
    /// stacked with the gap between them, or the shown children end to end
    /// with the gaps, as tall (wide) as the largest
    fn extent(
        &self,
        items: &[Option<(u16, u16)>],
        wrap: bool,
        max_width: u16,
        gap: u16,
    ) -> (u16, u16) {
        if wrap {
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
        }
    }
}

/// `child`'s size, with its styled subtree when that is known.
pub(super) fn measure_child(
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
