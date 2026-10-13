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
    use crate::widget::data::chart::chart_common::{ChartGrid, LegendPosition};
    use crate::widget::traits::RenderContext;

    // =========================================================================
    // render_title edge cases
    // =========================================================================

    #[test]
    fn test_render_title_centered() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let offset = render_title(&mut ctx, area, Some("ABC"), Color::CYAN);
        assert_eq!(offset, 1);

        // Check that title was centered (starts around x=8 or 9 for 3-char title)
        let mut found_a = false;
        let mut found_b = false;
        let mut found_c = false;
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, 0) {
                match cell.symbol {
                    'A' => found_a = true,
                    'B' => found_b = true,
                    'C' => found_c = true,
                    _ => {}
                }
            }
        }
        assert!(found_a && found_b && found_c);
    }

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

    #[test]
    fn test_render_grid_none() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let grid = ChartGrid::new();
        render_grid(&mut ctx, area, &grid, 5, 5);

        // Should not have any grid characters
        let mut found_grid = false;
        for y in 0..10 {
            for x in 0..20 {
                if let Some(cell) = buffer.get(x, y) {
                    if matches!(cell.symbol, '│' | '─' | '├' | '└' | '┴') {
                        found_grid = true;
                        break;
                    }
                }
            }
        }
        assert!(!found_grid);
    }

    // =========================================================================
    // calculate_legend_position for all positions
    // =========================================================================

    #[test]
    fn test_calculate_legend_position_top_right() {
        let area = Rect::new(0, 0, 100, 50);
        let pos = calculate_legend_position(LegendPosition::TopRight, area, 20, 5);
        assert_eq!(pos, Some((79, 1)));
    }

    #[test]
    fn test_calculate_legend_position_top_center() {
        let area = Rect::new(0, 0, 100, 50);
        let pos = calculate_legend_position(LegendPosition::TopCenter, area, 20, 5);
        assert_eq!(pos, Some((40, 1)));
    }

    #[test]
    fn test_calculate_legend_position_bottom_left() {
        let area = Rect::new(0, 0, 100, 50);
        let pos = calculate_legend_position(LegendPosition::BottomLeft, area, 20, 5);
        assert_eq!(pos, Some((1, 44)));
    }

    #[test]
    fn test_calculate_legend_position_bottom_center() {
        let area = Rect::new(0, 0, 100, 50);
        let pos = calculate_legend_position(LegendPosition::BottomCenter, area, 20, 5);
        assert!(pos.is_some());
        let (x, y) = pos.unwrap();
        assert!(x > 0 && x < 100);
        assert!(y > 0);
    }

    #[test]
    fn test_calculate_legend_position_left() {
        let area = Rect::new(0, 0, 100, 50);
        let pos = calculate_legend_position(LegendPosition::Left, area, 20, 5);
        assert_eq!(pos, Some((1, 22)));
    }

    #[test]
    fn test_calculate_legend_position_right() {
        let area = Rect::new(0, 0, 100, 50);
        let pos = calculate_legend_position(LegendPosition::Right, area, 20, 5);
        assert_eq!(pos, Some((79, 22)));
    }

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

    #[test]
    fn test_fill_background_full_area() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        fill_background(&mut ctx, area, Color::GREEN);

        // Check all cells have green background
        for y in 0..5 {
            for x in 0..10 {
                if let Some(cell) = buffer.get(x, y) {
                    assert_eq!(cell.bg, Some(Color::GREEN));
                }
            }
        }
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

    #[test]
    fn test_calculate_chart_area_small_area() {
        let area = Rect::new(0, 0, 10, 5);
        let chart_area = calculate_chart_area(area, true, 8, 2);
        // Should handle small areas without negative dimensions
        assert!(chart_area.width < 10);
        assert!(chart_area.height < 5);
    }
}
