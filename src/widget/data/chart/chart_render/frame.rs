//! Chart frame: title, background and the plot area left inside them

use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, display_width};
use crate::widget::traits::RenderContext;

// ============================================================================
// Title Rendering
// ============================================================================

/// Render a centered title at the top of the area
///
/// The `area` parameter uses **relative** coordinates (0,0 = widget top-left).
/// Returns the number of rows used (0 if no title, 1 if title rendered)
pub fn render_title(ctx: &mut RenderContext, area: Rect, title: Option<&str>, color: Color) -> u16 {
    let Some(title) = title else {
        return 0;
    };

    let title_x = area.x + (area.width.saturating_sub(display_width(title) as u16)) / 2;
    let mut dx: u16 = 0;
    for ch in title.chars() {
        let x = title_x + dx;
        if x < area.x + area.width {
            let mut cell = Cell::new(ch);
            cell.fg = Some(color);
            ctx.set(x, area.y, cell);
        }
        dx += char_width(ch) as u16;
    }
    1
}

// ============================================================================
// Background Rendering
// ============================================================================

/// Fill area with background color
///
/// The `area` parameter uses **relative** coordinates.
pub fn fill_background(ctx: &mut RenderContext, area: Rect, color: Color) {
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(color);
            ctx.set(x, y, cell);
        }
    }
}

// ============================================================================
// Chart Area Calculation
// ============================================================================

/// Calculate the chart area (excluding title, axes, legend)
///
/// The `area` parameter and returned Rect use **relative** coordinates.
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
