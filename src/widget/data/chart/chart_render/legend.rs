//! Chart legend: where it goes and the boxed or one-line list of series

use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, display_width};
use crate::widget::data::chart::chart_common::{Legend, LegendPosition};
use crate::widget::theme::DISABLED_FG;
use crate::widget::traits::RenderContext;

// ============================================================================
// Legend Rendering
// ============================================================================

/// A legend item with label and color
pub struct LegendItem<'a> {
    /// The text label for this legend item
    pub label: &'a str,
    /// The color to display for this legend item
    pub color: Color,
}

/// Calculate legend position based on LegendPosition
///
/// The `area` parameter and returned coordinates use **relative** coordinates.
pub fn calculate_legend_position(
    position: LegendPosition,
    area: Rect,
    legend_width: u16,
    legend_height: u16,
) -> Option<(u16, u16)> {
    match position {
        LegendPosition::TopLeft => Some((area.x + 1, area.y + 1)),
        LegendPosition::TopCenter => Some((
            area.x + area.width.saturating_sub(legend_width) / 2,
            area.y + 1,
        )),
        LegendPosition::TopRight => Some((
            area.x + area.width.saturating_sub(legend_width.saturating_add(1)),
            area.y + 1,
        )),
        LegendPosition::BottomLeft => Some((
            area.x + 1,
            area.y + area.height.saturating_sub(legend_height.saturating_add(1)),
        )),
        LegendPosition::BottomCenter => Some((
            area.x + area.width.saturating_sub(legend_width) / 2,
            area.y + area.height.saturating_sub(legend_height.saturating_add(1)),
        )),
        LegendPosition::BottomRight => Some((
            area.x + area.width.saturating_sub(legend_width.saturating_add(1)),
            area.y + area.height.saturating_sub(legend_height.saturating_add(1)),
        )),
        LegendPosition::Left => Some((
            area.x + 1,
            area.y + area.height.saturating_sub(legend_height) / 2,
        )),
        LegendPosition::Right => Some((
            area.x + area.width.saturating_sub(legend_width.saturating_add(1)),
            area.y + area.height.saturating_sub(legend_height) / 2,
        )),
        LegendPosition::None => None,
    }
}

/// Render a boxed legend with items
pub fn render_legend(
    ctx: &mut RenderContext,
    area: Rect,
    legend: &Legend,
    items: &[LegendItem<'_>],
) {
    if !legend.is_visible() || items.is_empty() {
        return;
    }

    // Sized in usize and clamped to u16: a count cast to u16 and then
    // added to overflows near 65535 (#1552)
    let clamp = |n: usize| u16::try_from(n).unwrap_or(u16::MAX);
    let legend_width = clamp(
        items
            .iter()
            .map(|item| item.label.len().saturating_add(4))
            .max()
            .unwrap_or(10),
    );
    let legend_height = clamp(items.len().saturating_add(2));

    let Some((legend_x, legend_y)) =
        calculate_legend_position(legend.position, area, legend_width, legend_height)
    else {
        return;
    };
    let right = area.x.saturating_add(area.width);
    let bottom = area.y.saturating_add(area.height);

    // Draw legend box, as much of it as is in the area
    for dy in 0..legend_height.min(bottom.saturating_sub(legend_y)) {
        for dx in 0..legend_width.min(right.saturating_sub(legend_x)) {
            let x = legend_x + dx;
            let y = legend_y + dy;
            let ch = if dy == 0 && dx == 0 {
                '┌'
            } else if dy == 0 && dx == legend_width - 1 {
                '┐'
            } else if dy == legend_height - 1 && dx == 0 {
                '└'
            } else if dy == legend_height - 1 && dx == legend_width - 1 {
                '┘'
            } else if dy == 0 || dy == legend_height - 1 {
                '─'
            } else if dx == 0 || dx == legend_width - 1 {
                '│'
            } else {
                ' '
            };
            let mut cell = Cell::new(ch);
            cell.fg = Some(DISABLED_FG);
            ctx.set(x, y, cell);
        }
    }

    // Draw legend items, one per row down to the area's bottom
    let label_end = right
        .saturating_sub(1)
        .min((legend_x.saturating_add(legend_width)).saturating_sub(1));
    for (y, item) in (legend_y.saturating_add(1)..bottom).zip(items) {
        // Color marker
        let marker_x = legend_x.saturating_add(1);
        if marker_x < right {
            let mut cell = Cell::new('■');
            cell.fg = Some(item.color);
            ctx.set(marker_x, y, cell);
        }

        // Label
        let mut x = legend_x.saturating_add(3);
        for ch in item.label.chars() {
            if x >= label_end {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            ctx.set(x, y, cell);
            x = x.saturating_add(char_width(ch) as u16);
        }
    }
}

/// Render a simple horizontal legend (for pie charts)
pub fn render_horizontal_legend(
    ctx: &mut RenderContext,
    area: Rect,
    legend: &Legend,
    items: &[LegendItem<'_>],
) {
    if !legend.is_visible() || items.is_empty() {
        return;
    }

    // Calculate total width needed: each marker, space and label, and a
    // 2-column gap between items, in usize and clamped to u16 (#1552)
    let total_width = items
        .iter()
        .map(|item| display_width(item.label).saturating_add(3))
        .fold(0usize, usize::saturating_add)
        .saturating_add((items.len() - 1).saturating_mul(2));
    let total_width = u16::try_from(total_width).unwrap_or(u16::MAX);

    let legend_height = 1;
    let Some((legend_x, legend_y)) =
        calculate_legend_position(legend.position, area, total_width, legend_height)
    else {
        return;
    };

    let right = area.x.saturating_add(area.width);
    let mut x = legend_x;
    for item in items {
        if x >= right {
            break;
        }

        // Color marker
        let mut cell = Cell::new('●');
        cell.fg = Some(item.color);
        ctx.set(x, legend_y, cell);

        // A space, the label in terminal columns, and the gap
        x = x.saturating_add(2);
        let used = ctx.put_str_with(x, legend_y, item.label, right, |ch| {
            Cell::new(ch).fg(Color::WHITE)
        });
        x = x.saturating_add(used).saturating_add(2);
    }
}
