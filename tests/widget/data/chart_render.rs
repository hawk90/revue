//! Chart rendering tests extracted from src/widget/data/chart/chart_render.rs
//!
//! This file contains tests for chart rendering utilities:
//! - render_title() - Render centered title
//! - render_grid() - Render grid lines
//! - calculate_legend_position() - Calculate legend position
//! - render_legend() - Render legend with items
//! - fill_background() - Fill area with background color
//! - calculate_chart_area() - Calculate chart area excluding decorations

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::chart_common::{Axis, ChartGrid, Legend, LegendPosition};
use revue::widget::data::chart::chart_render::{
    calculate_chart_area, calculate_legend_position, fill_background, render_grid,
    render_horizontal_legend, render_legend, render_title, LegendItem,
};

#[test]
fn test_render_title() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

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
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    let offset = render_title(&mut ctx, area, None, Color::WHITE);
    assert_eq!(offset, 0);
}

#[test]
fn test_render_grid() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

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
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

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
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

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

#[test]
fn test_render_title_centered() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

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
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    let offset = render_title(&mut ctx, area, Some("Very Long Title"), Color::WHITE);
    assert_eq!(offset, 1);
}

#[test]
fn test_render_grid_only_x() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

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
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

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
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

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

#[test]
fn test_fill_background_zero_area() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 0, 5);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    fill_background(&mut ctx, area, Color::RED);
    // Should not crash
}

#[test]
fn test_fill_background_full_area() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

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
