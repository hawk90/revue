//! Braille shape tests

use revue::style::Color;
use revue::widget::canvas::BrailleGrid;
use revue::widget::canvas::{Arc, Circle, FilledCircle, FilledPolygon, FilledRectangle, Line, Points, Polygon, Rectangle};

fn make_grid() -> BrailleGrid {
    BrailleGrid::new(40, 20)
}

// =========================================================================
// Line tests
// =========================================================================

#[test]
fn test_line_new() {
    let line = Line::new(0.0, 0.0, 10.0, 10.0, Color::RED);
    assert_eq!(line.x0, 0.0);
    assert_eq!(line.y0, 0.0);
    assert_eq!(line.x1, 10.0);
    assert_eq!(line.y1, 10.0);
}

#[test]
fn test_line_draw() {
    let mut grid = make_grid();
    let line = Line::new(0.0, 0.0, 5.0, 5.0, Color::BLUE);
    line.draw(&mut grid);
    // Should not panic
}

#[test]
fn test_line_horizontal() {
    let mut grid = make_grid();
    let line = Line::new(0.0, 5.0, 20.0, 5.0, Color::GREEN);
    line.draw(&mut grid);
}

#[test]
fn test_line_vertical() {
    let mut grid = make_grid();
    let line = Line::new(5.0, 0.0, 5.0, 20.0, Color::YELLOW);
    line.draw(&mut grid);
}

#[test]
fn test_line_clone() {
    let line1 = Line::new(0.0, 0.0, 10.0, 10.0, Color::RED);
    let line2 = line1.clone();
    assert_eq!(line1.x0, line2.x0);
    assert_eq!(line1.y1, line2.y1);
}

// =========================================================================
// Circle tests
// =========================================================================

#[test]
fn test_circle_new() {
    let circle = Circle::new(10.0, 10.0, 5.0, Color::RED);
    assert_eq!(circle.x, 10.0);
    assert_eq!(circle.y, 10.0);
    assert_eq!(circle.radius, 5.0);
}

#[test]
fn test_circle_draw() {
    let mut grid = make_grid();
    let circle = Circle::new(20.0, 20.0, 10.0, Color::BLUE);
    circle.draw(&mut grid);
}

#[test]
fn test_circle_zero_radius() {
    let mut grid = make_grid();
    let circle = Circle::new(10.0, 10.0, 0.0, Color::GREEN);
    circle.draw(&mut grid);
}

#[test]
fn test_circle_clone() {
    let circle1 = Circle::new(10.0, 10.0, 5.0, Color::BLUE);
    let circle2 = circle1.clone();
    assert_eq!(circle1.radius, circle2.radius);
}

// =========================================================================
// FilledCircle tests
// =========================================================================

#[test]
fn test_filled_circle_new() {
    let circle = FilledCircle::new(10.0, 10.0, 5.0, Color::RED);
    assert_eq!(circle.x, 10.0);
    assert_eq!(circle.radius, 5.0);
}

#[test]
fn test_filled_circle_draw() {
    let mut grid = make_grid();
    let circle = FilledCircle::new(20.0, 20.0, 5.0, Color::CYAN);
    circle.draw(&mut grid);
}

#[test]
fn test_filled_circle_clone() {
    let circle1 = FilledCircle::new(10.0, 10.0, 5.0, Color::RED);
    let circle2 = circle1.clone();
    assert_eq!(circle1.radius, circle2.radius);
}

// =========================================================================
// Arc tests
// =========================================================================

#[test]
fn test_arc_new() {
    let arc = Arc::new(10.0, 10.0, 5.0, 0.0, std::f64::consts::PI, Color::RED);
    assert_eq!(arc.x, 10.0);
    assert_eq!(arc.radius, 5.0);
}

#[test]
fn test_arc_from_degrees() {
    let arc = Arc::from_degrees(10.0, 10.0, 5.0, 0.0, 180.0, Color::BLUE);
    assert_eq!(arc.x, 10.0);
    assert!((arc.start_angle - 0.0).abs() < 1e-10);
    assert!((arc.end_angle - std::f64::consts::PI).abs() < 1e-10);
}

#[test]
fn test_arc_draw() {
    let mut grid = make_grid();
    let arc = Arc::new(20.0, 20.0, 10.0, 0.0, std::f64::consts::PI, Color::GREEN);
    arc.draw(&mut grid);
}

#[test]
fn test_arc_full_circle() {
    let mut grid = make_grid();
    let arc = Arc::from_degrees(20.0, 20.0, 10.0, 0.0, 360.0, Color::YELLOW);
    arc.draw(&mut grid);
}

#[test]
fn test_arc_clone() {
    let arc1 = Arc::new(10.0, 10.0, 5.0, 0.0, 1.0, Color::GREEN);
    let arc2 = arc1.clone();
    assert_eq!(arc1.radius, arc2.radius);
}

// =========================================================================
// Polygon tests
// =========================================================================

#[test]
fn test_polygon_new() {
    let vertices = vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)];
    let poly = Polygon::new(vertices.clone(), Color::RED);
    assert_eq!(poly.vertices.len(), 3);
}

#[test]
fn test_polygon_draw() {
    let mut grid = make_grid();
    let vertices = vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)];
    let poly = Polygon::new(vertices, Color::BLUE);
    poly.draw(&mut grid);
}

#[test]
fn test_polygon_empty() {
    let mut grid = make_grid();
    let poly = Polygon::new(vec![], Color::GREEN);
    poly.draw(&mut grid); // Should not panic
}

#[test]
fn test_polygon_single_vertex() {
    let mut grid = make_grid();
    let poly = Polygon::new(vec![(5.0, 5.0)], Color::YELLOW);
    poly.draw(&mut grid); // Should not panic (len < 2 returns early)
}

#[test]
fn test_polygon_regular_triangle() {
    let poly = Polygon::regular(10.0, 10.0, 5.0, 3, Color::CYAN);
    assert_eq!(poly.vertices.len(), 3);
}

#[test]
fn test_polygon_regular_hexagon() {
    let poly = Polygon::regular(10.0, 10.0, 5.0, 6, Color::MAGENTA);
    assert_eq!(poly.vertices.len(), 6);
}

#[test]
fn test_polygon_clone() {
    let vertices = vec![(0.0, 0.0), (1.0, 1.0)];
    let poly1 = Polygon::new(vertices.clone(), Color::YELLOW);
    let poly2 = poly1.clone();
    assert_eq!(poly1.vertices.len(), poly2.vertices.len());
}

// =========================================================================
// FilledPolygon tests
// =========================================================================

#[test]
fn test_filled_polygon_new() {
    let vertices = vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)];
    let poly = FilledPolygon::new(vertices.clone(), Color::RED);
    assert_eq!(poly.vertices.len(), 3);
}

#[test]
fn test_filled_polygon_draw() {
    let mut grid = make_grid();
    let vertices = vec![(10.0, 10.0), (30.0, 10.0), (20.0, 30.0)];
    let poly = FilledPolygon::new(vertices, Color::BLUE);
    poly.draw(&mut grid);
}

#[test]
fn test_filled_polygon_empty() {
    let mut grid = make_grid();
    let poly = FilledPolygon::new(vec![], Color::GREEN);
    poly.draw(&mut grid); // Should not panic (len < 3 returns early)
}

#[test]
fn test_filled_polygon_two_vertices() {
    let mut grid = make_grid();
    let poly = FilledPolygon::new(vec![(0.0, 0.0), (10.0, 10.0)], Color::YELLOW);
    poly.draw(&mut grid); // Should not panic (len < 3 returns early)
}

#[test]
fn test_filled_polygon_clone() {
    let vertices = vec![(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let poly1 = FilledPolygon::new(vertices.clone(), Color::BLUE);
    let poly2 = poly1.clone();
    assert_eq!(poly1.vertices.len(), poly2.vertices.len());
}

// =========================================================================
// Rectangle tests
// =========================================================================

#[test]
fn test_rectangle_new() {
    let rect = Rectangle::new(5.0, 5.0, 20.0, 10.0, Color::RED);
    assert_eq!(rect.x, 5.0);
    assert_eq!(rect.y, 5.0);
    assert_eq!(rect.width, 20.0);
    assert_eq!(rect.height, 10.0);
}

#[test]
fn test_rectangle_draw() {
    let mut grid = make_grid();
    let rect = Rectangle::new(5.0, 5.0, 20.0, 10.0, Color::BLUE);
    rect.draw(&mut grid);
}

#[test]
fn test_rectangle_zero_size() {
    let mut grid = make_grid();
    let rect = Rectangle::new(10.0, 10.0, 0.0, 0.0, Color::GREEN);
    rect.draw(&mut grid);
}

#[test]
fn test_rectangle_clone() {
    let rect1 = Rectangle::new(0.0, 0.0, 10.0, 5.0, Color::CYAN);
    let rect2 = rect1.clone();
    assert_eq!(rect1.width, rect2.width);
}

// =========================================================================
// FilledRectangle tests
// =========================================================================

#[test]
fn test_filled_rectangle_new() {
    let rect = FilledRectangle::new(5.0, 5.0, 20.0, 10.0, Color::RED);
    assert_eq!(rect.x, 5.0);
    assert_eq!(rect.width, 20.0);
}

#[test]
fn test_filled_rectangle_draw() {
    let mut grid = make_grid();
    let rect = FilledRectangle::new(5.0, 5.0, 20.0, 10.0, Color::CYAN);
    rect.draw(&mut grid);
}

#[test]
fn test_filled_rectangle_negative_coords() {
    let mut grid = make_grid();
    let rect = FilledRectangle::new(-5.0, -5.0, 20.0, 10.0, Color::MAGENTA);
    rect.draw(&mut grid); // Should clamp to 0
}

#[test]
fn test_filled_rectangle_clone() {
    let rect1 = FilledRectangle::new(0.0, 0.0, 10.0, 5.0, Color::MAGENTA);
    let rect2 = rect1.clone();
    assert_eq!(rect1.width, rect2.width);
}

// =========================================================================
// Points tests
// =========================================================================

#[test]
fn test_points_new() {
    let coords = vec![(0.0, 0.0), (5.0, 5.0), (10.0, 0.0)];
    let points = Points::new(coords.clone(), Color::RED);
    assert_eq!(points.coords.len(), 3);
}

#[test]
fn test_points_draw() {
    let mut grid = make_grid();
    let coords = vec![(0.0, 0.0), (10.0, 10.0), (20.0, 5.0)];
    let points = Points::new(coords, Color::BLUE);
    points.draw(&mut grid);
}

#[test]
fn test_points_empty() {
    let mut grid = make_grid();
    let points = Points::new(vec![], Color::GREEN);
    points.draw(&mut grid); // Should not panic (windows(2) is empty)
}

#[test]
fn test_points_single() {
    let mut grid = make_grid();
    let points = Points::new(vec![(5.0, 5.0)], Color::YELLOW);
    points.draw(&mut grid); // Should not panic (windows(2) is empty)
}

#[test]
fn test_points_from_slices() {
    let xs = &[0.0, 5.0, 10.0];
    let ys = &[0.0, 5.0, 0.0];
    let points = Points::from_slices(xs, ys, Color::CYAN);
    assert_eq!(points.coords.len(), 3);
}

#[test]
fn test_points_from_slices_empty() {
    let xs: &[f64] = &[];
    let ys: &[f64] = &[];
    let points = Points::from_slices(xs, ys, Color::MAGENTA);
    assert_eq!(points.coords.len(), 0);
}

#[test]
fn test_points_clone() {
    let coords = vec![(0.0, 0.0), (1.0, 1.0)];
    let pts1 = Points::new(coords.clone(), Color::WHITE);
    let pts2 = pts1.clone();
    assert_eq!(pts1.coords.len(), pts2.coords.len());
}
