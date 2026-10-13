//! Chart legend: where it goes and the boxed or one-line list of series

use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::char_width;
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
            area.x + area.width.saturating_sub(legend_width + 1),
            area.y + 1,
        )),
        LegendPosition::BottomLeft => Some((
            area.x + 1,
            area.y + area.height.saturating_sub(legend_height + 1),
        )),
        LegendPosition::BottomCenter => Some((
            area.x + area.width.saturating_sub(legend_width) / 2,
            area.y + area.height.saturating_sub(legend_height + 1),
        )),
        LegendPosition::BottomRight => Some((
            area.x + area.width.saturating_sub(legend_width + 1),
            area.y + area.height.saturating_sub(legend_height + 1),
        )),
        LegendPosition::Left => Some((
            area.x + 1,
            area.y + area.height.saturating_sub(legend_height) / 2,
        )),
        LegendPosition::Right => Some((
            area.x + area.width.saturating_sub(legend_width + 1),
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
                cell.fg = Some(DISABLED_FG);
                ctx.set(x, y, cell);
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
            ctx.set(marker_x, y, cell);
        }

        // Label
        let mut dx: u16 = 0;
        for ch in item.label.chars() {
            let x = legend_x + 3 + dx;
            if x < area.x + area.width - 1 && x < legend_x + legend_width - 1 {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                ctx.set(x, y, cell);
            }
            dx += char_width(ch) as u16;
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
        ctx.set(x, legend_y, cell);
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
            ctx.set(x, legend_y, cell);
            x += 1;
        }

        // Gap between items
        x += 2;
    }
}
