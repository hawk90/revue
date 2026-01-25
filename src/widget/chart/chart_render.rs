//! Common rendering functions for chart widgets
//!
//! Shared rendering utilities for title, legend, grid, and axis labels.

// Allow dead code for utility functions that are not yet used but available for future use
#![allow(dead_code)]

use super::chart_common::{Axis, ChartGrid, Legend, LegendPosition};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::RenderContext;

// ============================================================================
// Title Rendering
// ============================================================================

/// Render a centered title at the top of the area
///
/// Returns the number of rows used (0 if no title, 1 if title rendered)
pub fn render_title(ctx: &mut RenderContext, area: Rect, title: Option<&str>, color: Color) -> u16 {
    let Some(title) = title else {
        return 0;
    };

    let title_x = area.x + (area.width.saturating_sub(title.len() as u16)) / 2;
    for (i, ch) in title.chars().enumerate() {
        let x = title_x + i as u16;
        if x < area.x + area.width {
            let mut cell = Cell::new(ch);
            cell.fg = Some(color);
            ctx.buffer.set(x, area.y, cell);
        }
    }
    1
}

// ============================================================================
// Grid Rendering
// ============================================================================

/// Render grid lines in the chart area
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
            let x = chart_area.x + (i as u16 * chart_area.width / x_ticks as u16);
            for y in chart_area.y..chart_area.y + chart_area.height {
                if x < chart_area.x + chart_area.width {
                    let ch = if y == chart_area.y + chart_area.height - 1 {
                        '┴'
                    } else {
                        '│'
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(grid_color);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }
    }

    if grid.y {
        // Horizontal grid lines
        for i in 0..=y_ticks {
            let y = chart_area.y + (i as u16 * chart_area.height / y_ticks as u16);
            for x in chart_area.x..chart_area.x + chart_area.width {
                if y < chart_area.y + chart_area.height {
                    let ch = if x == chart_area.x { '├' } else { '─' };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(grid_color);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }
    }
}

// ============================================================================
// Axis Rendering
// ============================================================================

/// Render Y axis labels on the left side
pub fn render_y_axis_labels(
    ctx: &mut RenderContext,
    area: Rect,
    axis: &Axis,
    y_min: f64,
    y_max: f64,
    label_width: u16,
) {
    for i in 0..=axis.ticks {
        let value = y_min + (y_max - y_min) * (1.0 - i as f64 / axis.ticks as f64);
        let label = axis.format_value(value);
        let y = area.y + 1 + (i as u16 * (area.height - 2) / axis.ticks as u16);

        for (j, ch) in label.chars().take(label_width as usize).enumerate() {
            let x = area.x + j as u16;
            if x < area.x + label_width && y < area.y + area.height {
                let mut cell = Cell::new(ch);
                cell.fg = Some(axis.color);
                ctx.buffer.set(x, y, cell);
            }
        }
    }
}

/// Render X axis labels at the bottom
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
    for i in 0..=axis.ticks {
        let value = x_min + (x_max - x_min) * i as f64 / axis.ticks as f64;
        let label = axis.format_value(value);
        let x = area.x + x_offset + (i as u16 * (area.width - x_offset) / axis.ticks as u16);

        for (j, ch) in label.chars().take(6).enumerate() {
            let label_x = x + j as u16;
            if label_x < area.x + area.width && label_y >= y_offset {
                let mut cell = Cell::new(ch);
                cell.fg = Some(axis.color);
                ctx.buffer.set(label_x, label_y, cell);
            }
        }
    }
}

/// Render axis title
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
        let title_x = area.x + (area.width - title.len() as u16) / 2;
        let title_y = area.y + area.height - 1;
        for (i, ch) in title.chars().enumerate() {
            let x = title_x + i as u16;
            if x < area.x + area.width {
                let mut cell = Cell::new(ch);
                cell.fg = Some(color);
                ctx.buffer.set(x, title_y, cell);
            }
        }
    }
    // Y axis title rendering would go here (rotated text, not commonly needed)
}

// ============================================================================
// Legend Rendering
// ============================================================================

/// A legend item with label and color
pub struct LegendItem<'a> {
    pub label: &'a str,
    pub color: Color,
}

/// Calculate legend position based on LegendPosition
pub fn calculate_legend_position(
    position: LegendPosition,
    area: Rect,
    legend_width: u16,
    legend_height: u16,
) -> Option<(u16, u16)> {
    match position {
        LegendPosition::TopLeft => Some((area.x + 1, area.y + 1)),
        LegendPosition::TopCenter => Some((area.x + (area.width - legend_width) / 2, area.y + 1)),
        LegendPosition::TopRight => Some((
            area.x + area.width.saturating_sub(legend_width + 1),
            area.y + 1,
        )),
        LegendPosition::BottomLeft => Some((
            area.x + 1,
            area.y + area.height.saturating_sub(legend_height + 1),
        )),
        LegendPosition::BottomCenter => Some((
            area.x + (area.width - legend_width) / 2,
            area.y + area.height.saturating_sub(legend_height + 1),
        )),
        LegendPosition::BottomRight => Some((
            area.x + area.width.saturating_sub(legend_width + 1),
            area.y + area.height.saturating_sub(legend_height + 1),
        )),
        LegendPosition::Left => Some((area.x + 1, area.y + (area.height - legend_height) / 2)),
        LegendPosition::Right => Some((
            area.x + area.width.saturating_sub(legend_width + 1),
            area.y + (area.height - legend_height) / 2,
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

    let legend_width = items
        .iter()
        .map(|item| item.label.len() + 4)
        .max()
        .unwrap_or(10) as u16;
    let legend_height = items.len() as u16 + 2;

    let Some((legend_x, legend_y)) =
        calculate_legend_position(legend.position, area, legend_width, legend_height)
    else {
        return;
    };

    // Draw legend box
    for dy in 0..legend_height {
        for dx in 0..legend_width {
            let x = legend_x + dx;
            let y = legend_y + dy;
            if x < area.x + area.width && y < area.y + area.height {
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
                cell.fg = Some(Color::rgb(100, 100, 100));
                ctx.buffer.set(x, y, cell);
            }
        }
    }

    // Draw legend items
    for (i, item) in items.iter().enumerate() {
        let y = legend_y + 1 + i as u16;
        if y >= area.y + area.height {
            break;
        }

        // Color marker
        let marker_x = legend_x + 1;
        if marker_x < area.x + area.width {
            let mut cell = Cell::new('■');
            cell.fg = Some(item.color);
            ctx.buffer.set(marker_x, y, cell);
        }

        // Label
        for (j, ch) in item.label.chars().enumerate() {
            let x = legend_x + 3 + j as u16;
            if x < area.x + area.width - 1 && x < legend_x + legend_width - 1 {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                ctx.buffer.set(x, y, cell);
            }
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

    // Calculate total width needed
    let total_width: u16 = items
        .iter()
        .map(|item| item.label.len() as u16 + 3)
        .sum::<u16>()
        + (items.len() as u16 - 1) * 2;

    let legend_height = 1;
    let Some((legend_x, legend_y)) =
        calculate_legend_position(legend.position, area, total_width, legend_height)
    else {
        return;
    };

    let mut x = legend_x;
    for item in items {
        if x >= area.x + area.width {
            break;
        }

        // Color marker
        let mut cell = Cell::new('●');
        cell.fg = Some(item.color);
        ctx.buffer.set(x, legend_y, cell);
        x += 1;

        // Space
        x += 1;

        // Label
        for ch in item.label.chars() {
            if x >= area.x + area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            ctx.buffer.set(x, legend_y, cell);
            x += 1;
        }

        // Gap between items
        x += 2;
    }
}

// ============================================================================
// Background Rendering
// ============================================================================

/// Fill area with background color
pub fn fill_background(ctx: &mut RenderContext, area: Rect, color: Color) {
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(color);
            ctx.buffer.set(x, y, cell);
        }
    }
}

// ============================================================================
// Chart Area Calculation
// ============================================================================

/// Calculate the chart area (excluding title, axes, legend)
pub fn calculate_chart_area(
    area: Rect,
    has_title: bool,
    y_label_width: u16,
    x_label_height: u16,
) -> Rect {
    let title_offset = if has_title { 1 } else { 0 };
    Rect {
        x: area.x + y_label_width,
        y: area.y + title_offset,
        width: area.width.saturating_sub(y_label_width + 1),
        height: area
            .height
            .saturating_sub(title_offset + x_label_height + 1),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;

    #[test]
    fn test_render_title() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let offset = render_title(&mut ctx, area, Some("Test Title"), Color::WHITE);
        assert_eq!(offset, 1);

        // Check that title was rendered
        let mut found = false;
        for x in 0..30 {
            if let Some(cell) = buffer.get(x, 0) {
                if cell.symbol == 'T' {
                    found = true;
                    break;
                }
            }
        }
        assert!(found);
    }

    #[test]
    fn test_render_title_none() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let offset = render_title(&mut ctx, area, None, Color::WHITE);
        assert_eq!(offset, 0);
    }

    #[test]
    fn test_render_grid() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let grid = ChartGrid::both();
        render_grid(&mut ctx, area, &grid, 5, 5);

        // Check for grid characters
        let mut found = false;
        for y in 0..10 {
            for x in 0..20 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '│' || cell.symbol == '─' {
                        found = true;
                        break;
                    }
                }
            }
        }
        assert!(found);
    }

    #[test]
    fn test_calculate_legend_position() {
        let area = Rect::new(0, 0, 100, 50);

        let pos = calculate_legend_position(LegendPosition::TopLeft, area, 20, 5);
        assert_eq!(pos, Some((1, 1)));

        let pos = calculate_legend_position(LegendPosition::BottomRight, area, 20, 5);
        assert_eq!(pos, Some((79, 44)));

        let pos = calculate_legend_position(LegendPosition::None, area, 20, 5);
        assert_eq!(pos, None);
    }

    #[test]
    fn test_render_legend() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let legend = Legend::top_right();
        let items = vec![
            LegendItem {
                label: "Series A",
                color: Color::RED,
            },
            LegendItem {
                label: "Series B",
                color: Color::GREEN,
            },
        ];

        render_legend(&mut ctx, area, &legend, &items);

        // Check for legend box
        let mut found = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '┌' || cell.symbol == '■' {
                        found = true;
                        break;
                    }
                }
            }
        }
        assert!(found);
    }

    #[test]
    fn test_fill_background() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        fill_background(&mut ctx, area, Color::BLUE);

        // Check that background was set
        if let Some(cell) = buffer.get(5, 2) {
            assert_eq!(cell.bg, Some(Color::BLUE));
        }
    }

    #[test]
    fn test_calculate_chart_area() {
        let area = Rect::new(0, 0, 100, 50);

        let chart_area = calculate_chart_area(area, true, 8, 2);
        assert_eq!(chart_area.x, 8);
        assert_eq!(chart_area.y, 1);
        assert!(chart_area.width < 100);
        assert!(chart_area.height < 50);

        let chart_area = calculate_chart_area(area, false, 8, 2);
        assert_eq!(chart_area.y, 0);
    }
}
