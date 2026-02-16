//! Public API tests for chart rendering functions

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::{chart_common, chart_render};
use revue::widget::traits::RenderContext;

#[test]
fn test_render_title() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let offset = chart_render::render_title(&mut ctx, area, Some("Test Title"), Color::WHITE);
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

    let offset = chart_render::render_title(&mut ctx, area, None, Color::WHITE);
    assert_eq!(offset, 0);
}

#[test]
fn test_render_grid() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = chart_common::ChartGrid::both();
    chart_render::render_grid(&mut ctx, area, &grid, 5, 5);

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

    let pos = chart_render::calculate_legend_position(
        chart_common::LegendPosition::TopLeft,
        area, 20, 5
    );
    assert_eq!(pos, Some((1, 1)));

    let pos = chart_render::calculate_legend_position(
        chart_common::LegendPosition::BottomRight,
        area, 20, 5
    );
    assert_eq!(pos, Some((79, 44)));

    let pos = chart_render::calculate_legend_position(
        chart_common::LegendPosition::None,
        area, 20, 5
    );
    assert_eq!(pos, None);
}

#[test]
fn test_render_legend() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let legend = chart_common::Legend::top_right();
    let items = vec![
        chart_render::LegendItem {
            label: "Series A",
            color: Color::RED,
        },
        chart_render::LegendItem {
            label: "Series B",
            color: Color::GREEN,
        },
    ];

    chart_render::render_legend(&mut ctx, area, &legend, &items);

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

    chart_render::fill_background(&mut ctx, area, Color::BLUE);

    // Check that background was set
    if let Some(cell) = buffer.get(5, 2) {
        assert_eq!(cell.bg, Some(Color::BLUE));
    }
}

#[test]
fn test_calculate_chart_area() {
    let area = Rect::new(0, 0, 100, 50);

    let chart_area = chart_render::calculate_chart_area(area, true, 8, 2);
    assert_eq!(chart_area.x, 8);
    assert_eq!(chart_area.y, 1);
    assert!(chart_area.width < 100);
    assert!(chart_area.height < 50);

    let chart_area = chart_render::calculate_chart_area(area, false, 8, 2);
    assert_eq!(chart_area.y, 0);
}