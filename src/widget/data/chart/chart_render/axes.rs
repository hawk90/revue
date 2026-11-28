//! Chart axes: grid lines, tick labels and axis titles

use super::labels::{label_ticks, put_label, put_x_labels, x_label_slot};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::char_width;
use crate::utils::unicode::display_width_u16;
use crate::widget::data::chart::chart_common::{Axis, ChartGrid};
use crate::widget::traits::RenderContext;

// ============================================================================
// Scale
// ============================================================================

/// The cell of `value` along an axis from `min` (cell 0) to `max` (cell
/// `span`, the cells less one, so both ends are on the chart): the one
/// mapping the grid, the tick labels and the marks share (#1622). Tick `i`
/// of `n`, at `min + (max - min) * i / n`, lands on cell `i * span / n`,
/// the cell [`label_ticks`] gives it; a value past an end goes on that
/// end, and an empty or non-finite range puts every value on cell 0.
pub(crate) fn axis_cell(value: f64, min: f64, max: f64, span: u16) -> u16 {
    let range = max - min;
    if !(range > 0.0 && range.is_finite()) {
        return 0;
    }
    let ratio = ((value - min) / range).clamp(0.0, 1.0);
    if ratio.is_nan() {
        return 0;
    }
    // A tick value can come back a hair under its cell after rounding
    let cell = (ratio * f64::from(span) + 1e-9).floor();
    (cell as u16).min(span)
}

// ============================================================================
// Grid Rendering
// ============================================================================

/// Render grid lines in the chart area, at most one per row or column
/// however many ticks there are: tick `i` of `ticks` on the column and
/// row its tick label and [`axis_cell`] use, the first on the left and
/// top edges and the last on the right and bottom ones (#1622)
///
/// The `chart_area` parameter uses **relative** coordinates.
pub fn render_grid(
    ctx: &mut RenderContext,
    chart_area: Rect,
    grid: &ChartGrid,
    x_ticks: usize,
    y_ticks: usize,
) {
    if chart_area.width == 0 || chart_area.height == 0 {
        return;
    }
    let grid_color = grid.effective_color();
    let bottom = chart_area.y + chart_area.height - 1;

    if grid.x {
        // Vertical grid lines
        for (_, offset) in label_ticks(x_ticks, chart_area.width - 1, 1) {
            let x = chart_area.x + offset;
            for y in chart_area.y..=bottom {
                let ch = if y == bottom { '┴' } else { '│' };
                let mut cell = Cell::new(ch);
                cell.fg = Some(grid_color);
                ctx.set(x, y, cell);
            }
        }
    }

    if grid.y {
        // Horizontal grid lines
        for (_, offset) in label_ticks(y_ticks, chart_area.height - 1, 1) {
            let y = chart_area.y + offset;
            for x in chart_area.x..chart_area.x + chart_area.width {
                let ch = if x == chart_area.x { '├' } else { '─' };
                let mut cell = Cell::new(ch);
                cell.fg = Some(grid_color);
                ctx.set(x, y, cell);
            }
        }
    }
}

// ============================================================================
// Axis Rendering
// ============================================================================

/// The Y labels drawn beside `chart_area`, with their rows: tick `i` of
/// the axis's ticks, counted from `y_max` at the top row to `y_min` at
/// the bottom one, on the row the grid and [`axis_cell`] give it, at most
/// one per row (#1615, #1622)
fn y_axis_labels(chart_area: Rect, axis: &Axis, y_min: f64, y_max: f64) -> Vec<(u16, String)> {
    let ticks = axis.ticks.max(1);
    label_ticks(ticks, chart_area.height.saturating_sub(1), 1)
        .into_iter()
        .map(|(i, offset)| {
            let value = y_min + (y_max - y_min) * (1.0 - i as f64 / ticks as f64);
            (chart_area.y + offset, axis.format_value(value))
        })
        .collect()
}

/// The width of the widest Y label drawn beside a chart area of `rows`
/// rows, in columns, for [`y_label_column`](super::y_label_column)
pub(crate) fn y_axis_label_columns(rows: u16, axis: &Axis, y_min: f64, y_max: f64) -> u16 {
    y_axis_labels(Rect::new(0, 0, 1, rows), axis, y_min, y_max)
        .iter()
        .map(|(_, label)| display_width_u16(label))
        .max()
        .unwrap_or(0)
}

/// Draw the Y labels of the chart in `chart_area` from column `x`,
/// left-aligned in `label_width` columns, each on its tick's row; a wider
/// label is cut with `…`
pub(crate) fn render_chart_y_labels(
    ctx: &mut RenderContext,
    chart_area: Rect,
    x: u16,
    axis: &Axis,
    (y_min, y_max): (f64, f64),
    label_width: u16,
) {
    let style = Cell {
        fg: Some(axis.color),
        ..Cell::default()
    };
    for (y, label) in y_axis_labels(chart_area, axis, y_min, y_max) {
        put_label(ctx, &label, (x, y), label_width, false, style);
    }
}

/// Render Y axis labels on the left side, left-aligned in `label_width`
/// columns, on rows `area.y + 1` (`y_max`) to the last row of `area`
/// (`y_min`); a wider label is cut with `…`. When there are more ticks
/// than rows, a row shows one label and both ends are kept.
///
/// The `area` parameter uses **relative** coordinates.
pub fn render_y_axis_labels(
    ctx: &mut RenderContext,
    area: Rect,
    axis: &Axis,
    y_min: f64,
    y_max: f64,
    label_width: u16,
) {
    let rows = Rect {
        y: area.y.saturating_add(1),
        height: area.height.saturating_sub(1),
        ..area
    };
    render_chart_y_labels(ctx, rows, area.x, axis, (y_min, y_max), label_width);
}

/// Draw the X labels of the chart in `chart_area` on row `y`: tick `i`
/// of the axis's ticks on the column the grid and [`axis_cell`] give it,
/// `x_min` on the first column and `x_max` on the last, at most one label
/// per two columns, each centered on its tick and cut with `…` to the
/// columns between labeled ticks less one, inside the columns
/// `bounds` (#1606, #1615, #1622, #1623)
pub(crate) fn render_chart_x_labels(
    ctx: &mut RenderContext,
    chart_area: Rect,
    y: u16,
    bounds: (u16, u16),
    axis: &Axis,
    (x_min, x_max): (f64, f64),
) {
    let ticks = axis.ticks.max(1);
    let span = chart_area.width.saturating_sub(1);
    let labeled = label_ticks(ticks, span, 2);
    let slot = x_label_slot(&labeled, span);
    let labels = labeled.into_iter().map(|(i, offset)| {
        let value = x_min + (x_max - x_min) * i as f64 / ticks as f64;
        (chart_area.x + offset, axis.format_value(value))
    });
    let style = Cell {
        fg: Some(axis.color),
        ..Cell::default()
    };
    put_x_labels(ctx, labels, y, bounds, slot, style);
}

/// Render X axis labels on the last row of `area`, for the chart area
/// [`calculate_chart_area`](super::calculate_chart_area) leaves after an
/// `x_offset`-column Y label column: `x_min` on its first column and
/// `x_max` on its last, at most one label per two columns, each centered
/// on its tick and cut with `…` so neighbors never overwrite each other.
/// Nothing is drawn when that row is above `y_offset`.
///
/// The `area` parameter uses **relative** coordinates.
pub fn render_x_axis_labels(
    ctx: &mut RenderContext,
    area: Rect,
    axis: &Axis,
    x_min: f64,
    x_max: f64,
    y_offset: u16,
    x_offset: u16,
) {
    if area.height == 0 {
        return;
    }
    let label_y = area.y + area.height - 1;
    if label_y < y_offset {
        return;
    }
    let chart_area = Rect {
        x: area.x.saturating_add(x_offset),
        width: area.width.saturating_sub(x_offset.saturating_add(1)),
        ..area
    };
    let right = area.x.saturating_add(area.width);
    render_chart_x_labels(
        ctx,
        chart_area,
        label_y,
        (area.x, right),
        axis,
        (x_min, x_max),
    );
}

/// Render axis title
///
/// The `area` parameter uses **relative** coordinates.
pub fn render_axis_title(
    ctx: &mut RenderContext,
    area: Rect,
    title: Option<&str>,
    color: Color,
    is_x_axis: bool,
) {
    let Some(title) = title else {
        return;
    };

    if is_x_axis {
        let title_x = area.x + (area.width.saturating_sub(display_width_u16(title))) / 2;
        let title_y = area.y + area.height - 1;
        let right_edge = area.x.saturating_add(area.width);
        let mut dx: u16 = 0;
        for ch in title.chars() {
            let x = title_x.saturating_add(dx);
            if x >= right_edge {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(color);
            ctx.set(x, title_y, cell);
            dx = dx.saturating_add(char_width(ch) as u16);
        }
    }
    // Y axis title rendering would go here (rotated text, not commonly needed)
}
