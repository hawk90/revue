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
