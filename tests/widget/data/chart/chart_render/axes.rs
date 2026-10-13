//! Shared chart axis tests: grid lines

use super::super::rows;
use super::draw;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::chart::chart_render::render_grid;
use revue::widget::data::chart::ChartGrid;
use revue::widget::traits::RenderContext;

#[test]
fn test_render_grid() {
    let buffer = draw(20, 10, |ctx, area| {
        render_grid(ctx, area, &ChartGrid::both(), 5, 5);
    });
    let rows = rows(&buffer);
    // Horizontal lines every 2 rows (drawn over the vertical ones),
    // vertical lines every 4 columns ending in '┴' on the last row
    assert_eq!(rows[0], format!("├{}", "─".repeat(19)));
    assert_eq!(rows[2], rows[0]);
    assert_eq!(rows[1], "│   │   │   │   │   ");
    assert_eq!(rows[9], "┴   ┴   ┴   ┴   ┴   ");
}

#[test]
fn test_render_grid_none() {
    let buffer = draw(20, 10, |ctx, area| {
        render_grid(ctx, area, &ChartGrid::new(), 5, 5);
    });
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

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
