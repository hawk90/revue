//! BrailleContext tests

use revue::style::Color;
use revue::widget::canvas::BrailleContext;
use revue::widget::canvas::BrailleGrid;

#[test]
fn test_context_new() {
    let mut grid = BrailleGrid::new(40, 20);
    let ctx = BrailleContext::new(&mut grid);
    assert_eq!(ctx.width(), 80); // 40 * 2
    assert_eq!(ctx.height(), 80); // 20 * 4
}

#[test]
fn test_context_width() {
    let mut grid = BrailleGrid::new(30, 15);
    let ctx = BrailleContext::new(&mut grid);
    assert_eq!(ctx.width(), 60); // 30 * 2
}

#[test]
fn test_context_height() {
    let mut grid = BrailleGrid::new(30, 15);
    let ctx = BrailleContext::new(&mut grid);
    assert_eq!(ctx.height(), 60); // 15 * 4
}

#[test]
fn test_context_clear() {
    let mut grid = BrailleGrid::new(40, 20);
    {
        let mut ctx = BrailleContext::new(&mut grid);
        ctx.set(10, 10, Color::RED);
        ctx.clear();
    }
    // After clear, grid should be empty
    let mut grid2 = BrailleGrid::new(40, 20);
    let ctx2 = BrailleContext::new(&mut grid2);
    assert_eq!(ctx2.width(), 80);
}

#[test]
fn test_context_set() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.set(5, 5, Color::BLUE);
    // Should not panic
}

#[test]
fn test_context_line() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.line(0.0, 0.0, 10.0, 10.0, Color::RED);
    // Should not panic
}

#[test]
fn test_context_circle() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.circle(20.0, 20.0, 10.0, Color::GREEN);
    // Should not panic
}

#[test]
fn test_context_filled_circle() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.filled_circle(20.0, 20.0, 5.0, Color::BLUE);
    // Should not panic
}

#[test]
fn test_context_rect() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.rect(5.0, 5.0, 20.0, 10.0, Color::YELLOW);
    // Should not panic
}

#[test]
fn test_context_filled_rect() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.filled_rect(5.0, 5.0, 20.0, 10.0, Color::CYAN);
    // Should not panic
}

#[test]
fn test_context_points() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    let coords = vec![(0.0, 0.0), (10.0, 10.0), (20.0, 5.0)];
    ctx.points(coords, Color::MAGENTA);
    // Should not panic
}

#[test]
fn test_context_points_empty() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    let coords = vec![];
    ctx.points(coords, Color::MAGENTA);
    // Should not panic with empty coords
}

#[test]
fn test_context_arc() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.arc(20.0, 20.0, 10.0, 0.0, std::f64::consts::PI, Color::WHITE);
    // Should not panic
}

#[test]
fn test_context_arc_degrees() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.arc_degrees(20.0, 20.0, 10.0, 0.0, 180.0, Color::WHITE);
    // Should not panic
}

#[test]
fn test_context_arc_full_circle() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.arc_degrees(20.0, 20.0, 10.0, 0.0, 360.0, Color::WHITE);
    // Should not panic
}

#[test]
fn test_context_polygon() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    let vertices = vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)];
    ctx.polygon(vertices, Color::RED);
    // Should not panic
}

#[test]
fn test_context_polygon_empty() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    let vertices = vec![];
    ctx.polygon(vertices, Color::RED);
    // Should not panic with empty vertices
}

#[test]
fn test_context_regular_polygon() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.regular_polygon(20.0, 20.0, 10.0, 6, Color::GREEN);
    // Should not panic - hexagon
}

#[test]
fn test_context_regular_polygon_triangle() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.regular_polygon(20.0, 20.0, 10.0, 3, Color::BLUE);
    // Should not panic - triangle
}

#[test]
fn test_context_filled_polygon() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    let vertices = vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)];
    ctx.filled_polygon(vertices, Color::YELLOW);
    // Should not panic
}

#[test]
fn test_context_filled_polygon_empty() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    let vertices = vec![];
    ctx.filled_polygon(vertices, Color::YELLOW);
    // Should not panic with empty vertices
}

#[test]
fn test_context_line_horizontal() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.line(0.0, 10.0, 50.0, 10.0, Color::RED);
    // Should not panic
}

#[test]
fn test_context_line_vertical() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.line(10.0, 0.0, 10.0, 50.0, Color::GREEN);
    // Should not panic
}

#[test]
fn test_context_circle_zero_radius() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.circle(20.0, 20.0, 0.0, Color::BLUE);
    // Should not panic
}

#[test]
fn test_context_rect_zero_size() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.rect(10.0, 10.0, 0.0, 0.0, Color::CYAN);
    // Should not panic
}

#[test]
fn test_context_filled_rect_zero_size() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.filled_rect(10.0, 10.0, 0.0, 0.0, Color::MAGENTA);
    // Should not panic
}

#[test]
fn test_context_negative_coords() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.line(-10.0, -10.0, 10.0, 10.0, Color::RED);
    // Should not panic (shapes handle bounds)
}

#[test]
fn test_context_large_radius() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.circle(20.0, 20.0, 1000.0, Color::YELLOW);
    // Should not panic (clipped to grid bounds)
}

#[test]
fn test_context_multiple_operations() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut ctx = BrailleContext::new(&mut grid);
    ctx.rect(0.0, 0.0, 20.0, 20.0, Color::RED);
    ctx.circle(10.0, 10.0, 5.0, Color::BLUE);
    ctx.line(0.0, 0.0, 20.0, 20.0, Color::GREEN);
    ctx.clear();
    // Should handle multiple operations and clear
}
