//! Chart axes: grid lines, tick labels and axis titles

use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, display_width, truncate_to_width};
use crate::widget::data::chart::chart_common::{Axis, ChartGrid};
use crate::widget::traits::RenderContext;

// ============================================================================
// Grid Rendering
// ============================================================================

/// Render grid lines in the chart area
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
        for i in 0..=x_ticks {
            let x = chart_area.x + tick_offset(i, chart_area.width, x_ticks);
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
        for i in 0..=y_ticks {
            let y = chart_area.y + tick_offset(i, chart_area.height, y_ticks);
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

/// Render Y axis labels on the left side
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
    let ticks = axis.ticks.max(1);
    for i in 0..=ticks {
        let value = y_min + (y_max - y_min) * (1.0 - i as f64 / ticks as f64);
        let label = axis.format_value(value);
        let y = area.y + 1 + tick_offset(i, area.height.saturating_sub(2), ticks);

        let label_truncated = truncate_to_width(&label, label_width as usize);
        let mut dx: u16 = 0;
        for ch in label_truncated.chars() {
            let x = area.x + dx;
            if x < area.x + label_width && y < area.y + area.height {
                let mut cell = Cell::new(ch);
                cell.fg = Some(axis.color);
                ctx.set(x, y, cell);
            }
            dx += char_width(ch) as u16;
        }
    }
}

/// Render X axis labels at the bottom
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
    for i in 0..=ticks {
        let value = x_min + (x_max - x_min) * i as f64 / ticks as f64;
        let label = axis.format_value(value);
        let x = area.x + x_offset + tick_offset(i, area.width.saturating_sub(x_offset), ticks);

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
        let title_x = area.x + (area.width.saturating_sub(display_width(title) as u16)) / 2;
        let title_y = area.y + area.height - 1;
        let mut dx: u16 = 0;
        for ch in title.chars() {
            let x = title_x + dx;
            if x < area.x + area.width {
                let mut cell = Cell::new(ch);
                cell.fg = Some(color);
                ctx.set(x, title_y, cell);
            }
            dx += char_width(ch) as u16;
        }
    }
    // Y axis title rendering would go here (rotated text, not commonly needed)
}

/// The offset of tick `i` of `ticks` (at least one) across `span` cells:
/// at most `span`, computed wide so many ticks do not overflow
fn tick_offset(i: usize, span: u16, ticks: usize) -> u16 {
    (i as u128 * u128::from(span) / ticks.max(1) as u128) as u16
}
