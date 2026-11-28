//! Axis labels shared by the charts: how wide the Y label column is, which
//! ticks get a label, and drawing one label in its cells

use crate::render::Cell;
use crate::utils::char_width;
use crate::utils::unicode::{display_width_u16, truncate_with_ellipsis};
use crate::widget::traits::RenderContext;

/// The Y label column and the gap after it, in columns: as wide as the
/// widest label (`widest` columns), at most a third of `area_width` as for
/// the horizontal BarChart's labels, and one column apart from the plot.
/// A label wider than the column is cut with `…` (#1606, #1614).
pub(crate) fn y_label_column(widest: u16, area_width: u16) -> u16 {
    widest.min(area_width / 3).saturating_add(1)
}

/// The ticks `0..=ticks` to label along an axis whose ends are `span` cells
/// apart, tick `i` at cell `i * span / ticks` (`ticks` is at least one);
/// each item is a tick and its cell, from the first end to the last.
///
/// Labels closer than `gap` cells would overwrite each other, so a tick is
/// kept only `gap` cells or more past the one kept before it, and the last
/// tick always is, in place of a kept one closer than `gap` to it: at most
/// one label per `gap` cells, with both ends (#1615). The loop runs once per
/// kept tick, so a huge tick count costs no more than the cells (#1616).
/// When `span` is under `gap` there is room for one label, the first.
pub(crate) fn label_ticks(ticks: usize, span: u16, gap: u16) -> Vec<(usize, u16)> {
    let ticks = ticks.max(1);
    let gap = gap.max(1);
    if span < gap {
        return vec![(0, 0)];
    }
    let mut kept = vec![(0usize, 0u16)];
    loop {
        let (_, last) = kept[kept.len() - 1];
        // The first tick at or past `last + gap`: tick `i` is at or past
        // cell `c` when `i * span / ticks >= c`, that is `i >= c * ticks / span`
        let target = u128::from(last) + u128::from(gap);
        let i = (target * ticks as u128).div_ceil(u128::from(span));
        if i >= ticks as u128 {
            break;
        }
        let i = i as usize;
        kept.push((i, tick_cell(i, span, ticks)));
    }
    while kept.len() > 1 && span - kept[kept.len() - 1].1 < gap {
        kept.pop();
    }
    kept.push((ticks, span));
    kept
}

/// The cell of tick `i` of `ticks` (at least one) along `span` cells,
/// computed wide so many ticks do not overflow
fn tick_cell(i: usize, span: u16, ticks: usize) -> u16 {
    (i as u128 * u128::from(span) / ticks.max(1) as u128) as u16
}

/// Draw `label` on row `y` in the `columns` cells from column `x`, cut with
/// `…` when it is wider, against the right end of those cells when
/// `right_align` and from `x` otherwise. Each cell is `style` with the
/// label's glyph.
pub(crate) fn put_label(
    ctx: &mut RenderContext,
    label: &str,
    (x, y): (u16, u16),
    columns: u16,
    right_align: bool,
    style: Cell,
) {
    let label = truncate_with_ellipsis(label, usize::from(columns));
    let start = if right_align {
        x.saturating_add(columns.saturating_sub(display_width_u16(&label)))
    } else {
        x
    };
    let mut dx: u16 = 0;
    for ch in label.chars() {
        let mut cell = style;
        cell.symbol = ch;
        ctx.set(start.saturating_add(dx), y, cell);
        dx = dx.saturating_add(char_width(ch) as u16);
    }
}

/// The columns each X label among `labeled` (from [`label_ticks`] along
/// `span` cells) may take: the fewest columns between two labeled ticks
/// less one, so neighbors never touch, or the whole axis for one label.
/// At least one (#1606).
pub(crate) fn x_label_slot(labeled: &[(usize, u16)], span: u16) -> u16 {
    labeled
        .windows(2)
        .map(|pair| pair[1].1 - pair[0].1)
        .min()
        .unwrap_or(span.saturating_add(1))
        .saturating_sub(1)
        .max(1)
}

/// Draw X labels on row `y`, each `(tick, label)` centered on column
/// `tick` and cut with `…` to `slot` columns ([`x_label_slot`]), from left
/// to right. A label is kept inside the columns `left..right`: one past
/// an end is moved in, and one moved onto the label before it starts one
/// column after that label, cut with `…` to the columns left, so labels
/// never overwrite each other and the last tick keeps its label while
/// there is room for one (#1622, #1623). Each cell is `style`.
pub(crate) fn put_x_labels(
    ctx: &mut RenderContext,
    labels: impl IntoIterator<Item = (u16, String)>,
    y: u16,
    (left, right): (u16, u16),
    slot: u16,
    style: Cell,
) {
    let mut next_free = left;
    for (tick, label) in labels {
        let label = truncate_with_ellipsis(&label, usize::from(slot));
        let width = display_width_u16(&label);
        let start = tick
            .saturating_sub(width / 2)
            .min(right.saturating_sub(width))
            .max(next_free);
        let columns = width.min(right.saturating_sub(start));
        if columns == 0 {
            continue;
        }
        put_label(ctx, &label, (start, y), columns, false, style);
        next_free = start.saturating_add(columns).saturating_add(1);
    }
}
