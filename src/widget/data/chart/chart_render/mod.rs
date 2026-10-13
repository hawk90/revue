//! Common rendering functions for chart widgets
//!
//! Shared rendering utilities for title, legend, grid, and axis labels.

// Allow dead code for utility functions that are not yet used but available for future use
#![allow(dead_code)]

mod axes;
mod frame;
mod legend;

pub use axes::{render_axis_title, render_grid, render_x_axis_labels, render_y_axis_labels};
pub use frame::{calculate_chart_area, fill_background, render_title};
pub use legend::{calculate_legend_position, render_horizontal_legend, render_legend, LegendItem};

// ============================================================================
// Tests
// ============================================================================

// KEEP HERE - Private rendering tests (tests internal render_title, render_grid functions and buffer access)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::Color;
    use crate::widget::data::chart::chart_common::ChartGrid;
    use crate::widget::traits::RenderContext;

    // =========================================================================
    // render_title edge cases
    // =========================================================================

    #[test]
    fn test_render_title_long() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let offset = render_title(&mut ctx, area, Some("Very Long Title"), Color::WHITE);
        assert_eq!(offset, 1);
    }

    // =========================================================================
    // render_grid edge cases
    // =========================================================================

    #[test]
    fn test_render_grid_only_x() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let grid = ChartGrid::x_only();
        render_grid(&mut ctx, area, &grid, 5, 0);

        // Should only have vertical lines
        let mut found_vertical = false;
        for y in 0..10 {
            for x in 0..20 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '│' {
                        found_vertical = true;
                        break;
                    }
                }
            }
        }
        assert!(found_vertical);
    }

    #[test]
    fn test_render_grid_only_y() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let grid = ChartGrid::y_only();
        render_grid(&mut ctx, area, &grid, 0, 5);

        // Should only have horizontal lines
        let mut found_horizontal = false;
        for y in 0..10 {
            for x in 0..20 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '─' || cell.symbol == '├' {
                        found_horizontal = true;
                        break;
                    }
                }
            }
        }
        assert!(found_horizontal);
    }

    // =========================================================================
    // calculate_legend_position for all positions
    // =========================================================================

    // =========================================================================
    // fill_background edge cases
    // =========================================================================

    #[test]
    fn test_fill_background_zero_area() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 0, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        fill_background(&mut ctx, area, Color::RED);
        // Should not crash
    }

    // =========================================================================
    // calculate_chart_area edge cases
    // =========================================================================

    #[test]
    fn test_calculate_chart_area_no_labels() {
        let area = Rect::new(0, 0, 100, 50);
        let chart_area = calculate_chart_area(area, false, 0, 0);
        assert_eq!(chart_area.x, 0);
        assert_eq!(chart_area.y, 0);
        assert_eq!(chart_area.width, 99);
        assert_eq!(chart_area.height, 49);
    }
}
