//! Chart axes: grid lines, tick labels and axis titles

use super::labels::{label_ticks, put_label};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::unicode::display_width_u16;
use crate::utils::{char_width, truncate_to_width};
use crate::widget::data::chart::chart_common::{Axis, ChartGrid};
use crate::widget::traits::RenderContext;

// ============================================================================
// Grid Rendering
// ============================================================================

/// Render grid lines in the chart area, at most one per row or column
/// however many ticks there are
///
/// The `chart_area` parameter uses **relative** coordinates.
pub fn render_grid(
    ctx: &mut RenderContext,
    chart_area: Rect,
    grid: &ChartGrid,
    x_ticks: usize,
    y_ticks: usize,
) {
    let grid_color = grid.effective_color();

    if grid.x {
        // Vertical grid lines
        for (_, offset) in label_ticks(x_ticks, chart_area.width, 1) {
            let x = chart_area.x + offset;
            for y in chart_area.y..chart_area.y + chart_area.height {
                if x < chart_area.x + chart_area.width {
                    let ch = if y == chart_area.y + chart_area.height - 1 {
                        '┴'
                    } else {
                        '│'
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(grid_color);
                    ctx.set(x, y, cell);
                }
            }
        }
    }

    if grid.y {
        // Horizontal grid lines
        for (_, offset) in label_ticks(y_ticks, chart_area.height, 1) {
            let y = chart_area.y + offset;
            for x in chart_area.x..chart_area.x + chart_area.width {
                if y < chart_area.y + chart_area.height {
                    let ch = if x == chart_area.x { '├' } else { '─' };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(grid_color);
                    ctx.set(x, y, cell);
                }
            }
        }
    }
}

// ============================================================================
// Axis Rendering
// ============================================================================

/// The Y labels [`render_y_axis_labels`] draws in `area`, with their rows:
/// at most one per row, from `y_max` at the top to `y_min` at the bottom
fn y_axis_labels(area: Rect, axis: &Axis, y_min: f64, y_max: f64) -> Vec<(u16, String)> {
    let ticks = axis.ticks.max(1);
    label_ticks(ticks, area.height.saturating_sub(2), 1)
        .into_iter()
        .map(|(i, offset)| {
            let value = y_min + (y_max - y_min) * (1.0 - i as f64 / ticks as f64);
            (area.y + 1 + offset, axis.format_value(value))
        })
        .collect()
}

/// The width of the widest Y label [`render_y_axis_labels`] draws in
/// `area`, in columns, for [`y_label_column`](super::y_label_column)
pub(crate) fn y_axis_label_columns(area: Rect, axis: &Axis, y_min: f64, y_max: f64) -> u16 {
    y_axis_labels(area, axis, y_min, y_max)
        .iter()
        .map(|(_, label)| display_width_u16(label))
        .max()
        .unwrap_or(0)
}

/// Render Y axis labels on the left side, left-aligned in `label_width`
/// columns; a wider label is cut with `…`. When there are more ticks than
/// rows, a row shows one label and both ends are kept.
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
    let style = Cell {
        fg: Some(axis.color),
        ..Cell::default()
    };
    for (y, label) in y_axis_labels(area, axis, y_min, y_max) {
        if y < area.y + area.height {
            put_label(ctx, &label, (area.x, y), label_width, false, style);
        }
    }
}

/// Render X axis labels at the bottom, at most one per two columns
/// however many ticks there are, with both ends
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
    let label_y = area.y + area.height - 1;
    let ticks = axis.ticks.max(1);
    for (i, offset) in label_ticks(ticks, area.width.saturating_sub(x_offset), 2) {
        let value = x_min + (x_max - x_min) * i as f64 / ticks as f64;
        let label = axis.format_value(value);
        let x = area.x + x_offset + offset;

        let label_truncated = truncate_to_width(&label, 6);
        let mut dx: u16 = 0;
        for ch in label_truncated.chars() {
            let label_x = x + dx;
            if label_x < area.x + area.width && label_y >= y_offset {
                let mut cell = Cell::new(ch);
                cell.fg = Some(axis.color);
                ctx.set(label_x, label_y, cell);
            }
            dx += char_width(ch) as u16;
        }
    }
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
