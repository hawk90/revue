//! Braille shape tests (Line, Circle, Arc, Polygon, Rectangle, Points)

use revue::style::Color;
use revue::widget::Arc;
use revue::widget::BrailleGrid;
use revue::widget::Circle;
use revue::widget::FilledCircle;
use revue::widget::FilledPolygon;
use revue::widget::FilledRectangle;
use revue::widget::Line;
use revue::widget::Points;
use revue::widget::Polygon;
use revue::widget::Rectangle;
use revue::widget::Shape;
use std::collections::BTreeSet;

type Dots = BTreeSet<(usize, usize)>;

/// Bit of each dot within a cell, indexed [x][y].
const BITS: [[u8; 4]; 2] = [[0x01, 0x02, 0x04, 0x40], [0x08, 0x10, 0x20, 0x80]];

/// The raised dots of `grid`, in dot coordinates.
fn dots(grid: &BrailleGrid) -> Dots {
    let term_width = grid.width() / 2;
    let mut set = Dots::new();
    for (idx, &cell) in grid.cells().iter().enumerate() {
        let (cx, cy) = (idx % term_width, idx / term_width);
        for (dx, column) in BITS.iter().enumerate() {
            for (dy, &bit) in column.iter().enumerate() {
                if cell & bit != 0 {
                    set.insert((cx * 2 + dx, cy * 4 + dy));
                }
            }
        }
    }
    set
}

fn draw(shape: &impl Shape) -> Dots {
    let mut grid = BrailleGrid::new(20, 5); // 40 x 20 dots
    grid.draw(shape);
    dots(&grid)
}

fn rect_dots(xs: std::ops::RangeInclusive<usize>, ys: std::ops::RangeInclusive<usize>) -> Dots {
    ys.flat_map(|y| xs.clone().map(move |x| (x, y))).collect()
}

fn dist(p: (usize, usize), c: (f64, f64)) -> f64 {
    ((p.0 as f64 - c.0).powi(2) + (p.1 as f64 - c.1).powi(2)).sqrt()
}

// =========================================================================
// Line
// =========================================================================

#[test]
fn test_line_horizontal() {
    assert_eq!(
        draw(&Line::new(2.0, 3.0, 7.0, 3.0, Color::RED)),
        rect_dots(2..=7, 3..=3)
    );
}

#[test]
fn test_line_vertical() {
    assert_eq!(
        draw(&Line::new(5.0, 1.0, 5.0, 6.0, Color::RED)),
        rect_dots(5..=5, 1..=6)
    );
}

#[test]
fn test_line_diagonal() {
    let expected: Dots = (0..=5).map(|i| (i, i)).collect();
    assert_eq!(draw(&Line::new(0.0, 0.0, 5.0, 5.0, Color::RED)), expected);
}

#[test]
fn test_line_shallow_one_dot_per_column() {
    let dots = draw(&Line::new(0.0, 0.0, 9.0, 3.0, Color::RED));
    assert!(dots.contains(&(0, 0)));
    assert!(dots.contains(&(9, 3)));
    for x in 0..=9 {
        assert_eq!(dots.iter().filter(|d| d.0 == x).count(), 1, "column {x}");
    }
}

#[test]
fn test_line_reversed_joins_same_ends() {
    // Bresenham may break ties differently in each direction, but both
    // directions join the same two dots with one dot per column
    for dots in [
        draw(&Line::new(1.0, 2.0, 9.0, 6.0, Color::RED)),
        draw(&Line::new(9.0, 6.0, 1.0, 2.0, Color::RED)),
    ] {
        assert!(dots.contains(&(1, 2)) && dots.contains(&(9, 6)), "{dots:?}");
        for x in 1..=9 {
            assert_eq!(dots.iter().filter(|d| d.0 == x).count(), 1, "column {x}");
        }
    }
}

#[test]
fn test_line_fractional_endpoints() {
    // Endpoints that are not a whole number of dots apart: the line joins
    // the dots they fall in (and the draw terminates)
    let dots = draw(&Line::new(0.2, 0.7, 5.6, 3.4, Color::RED));
    assert!(dots.contains(&(0, 0)));
    assert!(dots.contains(&(5, 3)));
    assert_eq!(dots.len(), 6);
    for x in 0..=5 {
        assert_eq!(dots.iter().filter(|d| d.0 == x).count(), 1, "column {x}");
    }
}

#[test]
fn test_line_single_point() {
    assert_eq!(
        draw(&Line::new(3.0, 3.0, 3.0, 3.0, Color::RED)),
        rect_dots(3..=3, 3..=3)
    );
}

#[test]
fn test_line_color() {
    let mut grid = BrailleGrid::new(4, 1);
    grid.draw(&Line::new(0.0, 0.0, 7.0, 0.0, Color::CYAN));
    assert!(grid.colors().iter().all(|c| *c == Some(Color::CYAN)));
}

#[test]
fn test_line_clipped_at_grid_edge() {
    // The part past the right edge is dropped, not wrapped
    let dots = draw(&Line::new(35.0, 0.0, 45.0, 0.0, Color::RED));
    assert_eq!(dots, rect_dots(35..=39, 0..=0));
}

#[test]
fn test_line_off_grid_draws_nothing() {
    // Entirely left of / above the grid: nothing is drawn, rather than the
    // line being squashed onto column or row 0
    assert!(draw(&Line::new(-10.0, 2.0, -2.0, 6.0, Color::RED)).is_empty());
    assert!(draw(&Line::new(2.0, -8.0, 6.0, -1.0, Color::RED)).is_empty());
}

#[test]
fn test_line_partly_off_grid() {
    // Only the on-grid part of a line crossing the left edge is drawn
    let dots = draw(&Line::new(-4.0, 3.0, 4.0, 3.0, Color::RED));
    assert_eq!(dots, rect_dots(0..=4, 3..=3));
    let dots = draw(&Line::new(-4.0, 0.0, 4.0, 8.0, Color::RED));
    let expected: Dots = (0..=4).map(|i| (i, i + 4)).collect();
    assert_eq!(dots, expected);
}

// =========================================================================
// Circle
// =========================================================================

#[test]
fn test_circle_draw() {
    let dots = draw(&Circle::new(10.0, 10.0, 5.0, Color::BLUE));
    for p in [(15, 10), (5, 10), (10, 15), (10, 5)] {
        assert!(dots.contains(&p), "missing {p:?}");
    }
    // An outline: every dot is about one radius from the centre
    for &p in &dots {
        let d = dist(p, (10.0, 10.0));
        assert!((4.0..=6.0).contains(&d), "{p:?} at {d}");
    }
    // Symmetric under reflection through the centre
    for &(x, y) in &dots {
        assert!(dots.contains(&(20 - x, 20 - y)), "no mirror for ({x}, {y})");
    }

    let mut grid = BrailleGrid::new(20, 10);
    let circle = Circle::new(20.0, 20.0, 10.0, Color::CYAN);
    circle.draw(&mut grid);
}

#[test]
fn test_circle_zero_radius() {
    assert_eq!(
        draw(&Circle::new(10.0, 10.0, 0.0, Color::GREEN)),
        rect_dots(10..=10, 10..=10)
    );
}

#[test]
fn test_circle_near_origin_is_clipped() {
    // The part left of / above the grid is dropped, not folded onto it
    let dots = draw(&Circle::new(1.0, 1.0, 3.0, Color::GREEN));
    assert!(dots.contains(&(4, 1)));
    assert!(dots.contains(&(1, 4)));
    for &p in &dots {
        let d = dist(p, (1.0, 1.0));
        assert!((2.0..=4.0).contains(&d), "{p:?} at {d}");
    }
}

#[test]
fn test_filled_circle_draw() {
    let dots = draw(&FilledCircle::new(10.0, 10.0, 3.0, Color::RED));
    let expected: Dots = rect_dots(7..=13, 7..=13)
        .into_iter()
        .filter(|&p| dist(p, (10.0, 10.0)) <= 3.0)
        .collect();
    assert_eq!(dots, expected);

    let mut grid = BrailleGrid::new(20, 10);
    let circle = FilledCircle::new(20.0, 20.0, 5.0, Color::YELLOW);
    circle.draw(&mut grid);
}

#[test]
fn test_filled_circle_contains_outline() {
    let outline = draw(&Circle::new(10.0, 10.0, 4.0, Color::RED));
    let filled = draw(&FilledCircle::new(10.0, 10.0, 4.0, Color::RED));
    assert!(filled.len() > outline.len());
    assert!(filled.contains(&(10, 10)));
}

#[test]
fn test_filled_circle_near_origin_is_clipped() {
    let dots = draw(&FilledCircle::new(0.0, 0.0, 2.0, Color::RED));
    let expected: Dots = rect_dots(0..=2, 0..=2)
        .into_iter()
        .filter(|&p| dist(p, (0.0, 0.0)) <= 2.0)
        .collect();
    assert_eq!(dots, expected);
}

// =========================================================================
// Arc
// =========================================================================

#[test]
fn test_arc_draw_quarter() {
    // Angles grow clockwise on screen (y points down): 0..90 degrees is the
    // lower-right quarter
    let dots = draw(&Arc::from_degrees(10.0, 10.0, 5.0, 0.0, 90.0, Color::GREEN));
    assert!(dots.contains(&(15, 10)));
    assert!(dots.contains(&(10, 15)));
    for &(x, y) in &dots {
        assert!(x >= 10 && y >= 10, "({x}, {y}) outside the quarter");
    }
}

#[test]
fn test_arc_radians_match_degrees() {
    assert_eq!(
        draw(&Arc::new(
            10.0,
            10.0,
            5.0,
            0.0,
            std::f64::consts::PI,
            Color::GREEN
        )),
        draw(&Arc::from_degrees(
            10.0,
            10.0,
            5.0,
            0.0,
            180.0,
            Color::GREEN
        ))
    );
}

#[test]
fn test_arc_full_circle() {
    let dots = draw(&Arc::from_degrees(
        10.0,
        10.0,
        5.0,
        0.0,
        360.0,
        Color::YELLOW,
    ));
    // Reaches all four quadrants
    assert!(dots.iter().any(|&(x, y)| x > 10 && y > 10));
    assert!(dots.iter().any(|&(x, y)| x < 10 && y > 10));
    assert!(dots.iter().any(|&(x, y)| x < 10 && y < 10));
    assert!(dots.iter().any(|&(x, y)| x > 10 && y < 10));

    let mut grid = BrailleGrid::new(20, 10);
    let arc = Arc::new(20.0, 20.0, 10.0, 0.0, std::f64::consts::TAU, Color::GREEN);
    arc.draw(&mut grid);
}

#[test]
fn test_arc_end_before_start_wraps() {
    // 270 -> 90 degrees goes the long way round through 0: the right half
    let dots = draw(&Arc::from_degrees(
        10.0,
        10.0,
        5.0,
        270.0,
        90.0,
        Color::YELLOW,
    ));
    // Spans top to bottom through the rightmost point (0 degrees)
    assert!(
        dots.iter().any(|&(x, y)| x >= 14 && (9..=10).contains(&y)),
        "{dots:?}"
    );
    assert!(dots.iter().any(|&(_, y)| y <= 5), "{dots:?}");
    assert!(dots.iter().any(|&(_, y)| y >= 14), "{dots:?}");
    assert!(dots.iter().all(|&(x, _)| x >= 9), "{dots:?}");
}

// =========================================================================
// Polygon
// =========================================================================

#[test]
fn test_polygon_draw() {
    let square = Polygon::new(
        vec![(2.0, 2.0), (8.0, 2.0), (8.0, 8.0), (2.0, 8.0)],
        Color::CYAN,
    );
    let expected: Dots = rect_dots(2..=8, 2..=8)
        .into_iter()
        .filter(|&(x, y)| x == 2 || x == 8 || y == 2 || y == 8)
        .collect();
    assert_eq!(draw(&square), expected);

    let mut grid = BrailleGrid::new(20, 10);
    let polygon = Polygon::new(
        vec![(10.0, 10.0), (30.0, 10.0), (30.0, 30.0), (10.0, 30.0)],
        Color::CYAN,
    );
    polygon.draw(&mut grid);
}

#[test]
fn test_polygon_empty() {
    assert!(draw(&Polygon::new(vec![], Color::CYAN)).is_empty());
}

#[test]
fn test_polygon_single_vertex() {
    assert!(draw(&Polygon::new(vec![(5.0, 5.0)], Color::CYAN)).is_empty());
}

#[test]
fn test_polygon_two_vertices_is_a_line() {
    assert_eq!(
        draw(&Polygon::new(vec![(1.0, 1.0), (6.0, 1.0)], Color::CYAN)),
        rect_dots(1..=6, 1..=1)
    );
}

#[test]
fn test_polygon_regular_vertices() {
    let square = Polygon::regular(10.0, 10.0, 5.0, 4, Color::CYAN);
    assert_eq!(square.vertices.len(), 4);
    // The first vertex points straight up; the rest go clockwise
    let expected = [(10.0, 5.0), (15.0, 10.0), (10.0, 15.0), (5.0, 10.0)];
    for (v, e) in square.vertices.iter().zip(expected) {
        assert!(
            (v.0 - e.0).abs() < 1e-9 && (v.1 - e.1).abs() < 1e-9,
            "{v:?}"
        );
    }
}

#[test]
fn test_polygon_regular_draw() {
    let dots = draw(&Polygon::regular(10.0, 10.0, 5.0, 6, Color::CYAN));
    // Every edge of a hexagon of radius 5 lies between its inradius (4.33)
    // and radius, give or take the up-to-one-dot snap of each coordinate
    for &p in &dots {
        let d = dist(p, (10.0, 10.0));
        assert!((3.0..=6.5).contains(&d), "{p:?} at {d}");
    }
    assert!(dots.contains(&(10, 5)));
}

#[test]
fn test_filled_polygon_draw() {
    // Ray casting treats the top/left edges as inside and bottom/right as
    // outside, so a 2..8 square fills [2, 8) on both axes
    let square = FilledPolygon::new(
        vec![(2.0, 2.0), (8.0, 2.0), (8.0, 8.0), (2.0, 8.0)],
        Color::RED,
    );
    assert_eq!(draw(&square), rect_dots(2..=7, 2..=7));

    let mut grid = BrailleGrid::new(20, 10);
    let polygon = FilledPolygon::new(
        vec![(10.0, 10.0), (30.0, 10.0), (20.0, 30.0)],
        Color::MAGENTA,
    );
    polygon.draw(&mut grid);
}

#[test]
fn test_filled_polygon_triangle() {
    let dots = draw(&FilledPolygon::new(
        vec![(0.0, 0.0), (10.0, 0.0), (0.0, 10.0)],
        Color::RED,
    ));
    assert!(dots.contains(&(1, 1)));
    // Nothing beyond the hypotenuse x + y = 10
    assert!(dots.iter().all(|&(x, y)| x + y < 10), "{dots:?}");
}

#[test]
fn test_filled_polygon_empty() {
    assert!(draw(&FilledPolygon::new(vec![], Color::RED)).is_empty());
}

#[test]
fn test_filled_polygon_two_vertices() {
    assert!(draw(&FilledPolygon::new(
        vec![(0.0, 0.0), (5.0, 5.0)],
        Color::RED
    ))
    .is_empty());
}

// =========================================================================
// Rectangle
// =========================================================================

#[test]
fn test_rectangle_draw() {
    let expected: Dots = rect_dots(2..=7, 3..=7)
        .into_iter()
        .filter(|&(x, y)| x == 2 || x == 7 || y == 3 || y == 7)
        .collect();
    assert_eq!(
        draw(&Rectangle::new(2.0, 3.0, 5.0, 4.0, Color::RED)),
        expected
    );

    let mut grid = BrailleGrid::new(20, 10);
    let rect = Rectangle::new(5.0, 5.0, 20.0, 15.0, Color::WHITE);
    rect.draw(&mut grid);
}

#[test]
fn test_rectangle_zero_size() {
    assert_eq!(
        draw(&Rectangle::new(2.0, 3.0, 0.0, 0.0, Color::RED)),
        rect_dots(2..=2, 3..=3)
    );
}

#[test]
fn test_filled_rectangle_draw() {
    assert_eq!(
        draw(&FilledRectangle::new(2.0, 3.0, 5.0, 4.0, Color::RED)),
        rect_dots(2..=7, 3..=7)
    );

    let mut grid = BrailleGrid::new(20, 10);
    let rect = FilledRectangle::new(5.0, 5.0, 10.0, 8.0, Color::BLUE);
    rect.draw(&mut grid);
}

#[test]
fn test_filled_rectangle_contains_outline() {
    let outline = draw(&Rectangle::new(2.0, 3.0, 5.0, 4.0, Color::RED));
    let filled = draw(&FilledRectangle::new(2.0, 3.0, 5.0, 4.0, Color::RED));
    assert!(outline.is_subset(&filled));
}

#[test]
fn test_filled_rectangle_negative_coords() {
    // Clipped to the grid
    assert_eq!(
        draw(&FilledRectangle::new(-2.0, -2.0, 4.0, 4.0, Color::RED)),
        rect_dots(0..=2, 0..=2)
    );
}

// =========================================================================
// Points
// =========================================================================

#[test]
fn test_points_draw_polyline() {
    // Consecutive points are joined by lines
    let points = Points::new(vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0)], Color::MAGENTA);
    let expected: Dots = rect_dots(0..=4, 0..=0)
        .union(&rect_dots(4..=4, 0..=3))
        .copied()
        .collect();
    assert_eq!(draw(&points), expected);
}

#[test]
fn test_points_empty() {
    assert!(draw(&Points::new(vec![], Color::MAGENTA)).is_empty());
}

#[test]
fn test_points_single() {
    // A single point has no segment to draw
    assert!(draw(&Points::new(vec![(5.0, 5.0)], Color::MAGENTA)).is_empty());
}

#[test]
fn test_points_from_slices_pairs_up() {
    let points = Points::from_slices(&[0.0, 4.0, 4.0], &[0.0, 0.0, 3.0], Color::WHITE);
    assert_eq!(points.coords, vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0)]);
    assert_eq!(
        draw(&points),
        draw(&Points::new(points.coords.clone(), Color::WHITE))
    );
}

#[test]
fn test_points_from_slices_uneven() {
    // Extra values in the longer slice are ignored
    let points = Points::from_slices(&[1.0, 2.0, 3.0], &[4.0], Color::WHITE);
    assert_eq!(points.coords, vec![(1.0, 4.0)]);
}

#[test]
fn test_points_from_slices_empty() {
    let points = Points::from_slices(&[], &[], Color::MAGENTA);
    assert!(points.coords.is_empty());
}

#[test]
fn test_arc_creation() {
    let arc = Arc::new(20.0, 20.0, 10.0, 0.0, std::f64::consts::PI, Color::RED);
    assert_eq!(arc.x, 20.0);
    assert_eq!(arc.y, 20.0);
    assert_eq!(arc.radius, 10.0);
    assert_eq!(arc.start_angle, 0.0);
    assert_eq!(arc.end_angle, std::f64::consts::PI);
}

#[test]
fn test_arc_from_degrees() {
    let arc = Arc::from_degrees(20.0, 20.0, 10.0, 0.0, 180.0, Color::BLUE);
    assert_eq!(arc.x, 20.0);
    assert_eq!(arc.y, 20.0);
    assert!((arc.end_angle - std::f64::consts::PI).abs() < 0.001);
}

#[test]
fn test_arc_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let arc = Arc::new(
        20.0,
        20.0,
        10.0,
        0.0,
        std::f64::consts::FRAC_PI_2,
        Color::CYAN,
    );
    arc.draw(&mut grid);
}

#[test]
fn test_arc_reverse_direction() {
    let mut grid = BrailleGrid::new(20, 10);
    // End angle less than start angle should still work
    let arc = Arc::new(20.0, 20.0, 10.0, std::f64::consts::PI, 0.0, Color::YELLOW);
    arc.draw(&mut grid);
}

#[test]
fn test_polygon_creation() {
    let vertices = vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)];
    let polygon = Polygon::new(vertices.clone(), Color::RED);
    assert_eq!(polygon.vertices, vertices);
}

#[test]
fn test_polygon_regular() {
    let hex = Polygon::regular(20.0, 20.0, 10.0, 6, Color::BLUE);
    assert_eq!(hex.vertices.len(), 6);
}

#[test]
fn test_polygon_regular_triangle() {
    let triangle = Polygon::regular(20.0, 20.0, 10.0, 3, Color::GREEN);
    assert_eq!(triangle.vertices.len(), 3);
}

#[test]
fn test_polygon_draw_empty() {
    let mut grid = BrailleGrid::new(20, 10);
    let polygon = Polygon::new(vec![], Color::RED);
    polygon.draw(&mut grid);
    // Should not crash with empty vertices
}

#[test]
fn test_polygon_draw_single_point() {
    let mut grid = BrailleGrid::new(20, 10);
    let polygon = Polygon::new(vec![(10.0, 10.0)], Color::RED);
    polygon.draw(&mut grid);
    // Should not crash with single point
}

#[test]
fn test_filled_polygon_creation() {
    let vertices = vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)];
    let polygon = FilledPolygon::new(vertices.clone(), Color::YELLOW);
    assert_eq!(polygon.vertices, vertices);
}

#[test]
fn test_line_creation() {
    let line = Line::new(0.0, 0.0, 10.0, 10.0, Color::RED);
    assert_eq!(line.x0, 0.0);
    assert_eq!(line.y0, 0.0);
    assert_eq!(line.x1, 10.0);
    assert_eq!(line.y1, 10.0);
}

#[test]
fn test_line_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let line = Line::new(0.0, 0.0, 10.0, 10.0, Color::WHITE);
    line.draw(&mut grid);
    // Line should be drawn (visual verification would require more setup)
}

#[test]
fn test_circle_creation() {
    let circle = Circle::new(20.0, 20.0, 10.0, Color::BLUE);
    assert_eq!(circle.x, 20.0);
    assert_eq!(circle.y, 20.0);
    assert_eq!(circle.radius, 10.0);
}

#[test]
fn test_filled_circle_creation() {
    let circle = FilledCircle::new(15.0, 15.0, 8.0, Color::GREEN);
    assert_eq!(circle.x, 15.0);
    assert_eq!(circle.y, 15.0);
    assert_eq!(circle.radius, 8.0);
}

#[test]
fn test_rectangle_creation() {
    let rect = Rectangle::new(5.0, 5.0, 20.0, 10.0, Color::MAGENTA);
    assert_eq!(rect.x, 5.0);
    assert_eq!(rect.y, 5.0);
    assert_eq!(rect.width, 20.0);
    assert_eq!(rect.height, 10.0);
}

#[test]
fn test_filled_rectangle_creation() {
    let rect = FilledRectangle::new(0.0, 0.0, 10.0, 5.0, Color::RED);
    assert_eq!(rect.x, 0.0);
    assert_eq!(rect.y, 0.0);
    assert_eq!(rect.width, 10.0);
    assert_eq!(rect.height, 5.0);
}

#[test]
fn test_points_creation() {
    let coords = vec![(0.0, 0.0), (5.0, 5.0), (10.0, 0.0)];
    let points = Points::new(coords.clone(), Color::CYAN);
    assert_eq!(points.coords, coords);
}

#[test]
fn test_points_from_slices() {
    let xs = [0.0, 5.0, 10.0, 15.0];
    let ys = [0.0, 5.0, 0.0, 5.0];
    let points = Points::from_slices(&xs, &ys, Color::WHITE);
    assert_eq!(points.coords.len(), 4);
}

#[test]
fn test_points_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let points = Points::new(vec![(0.0, 0.0), (20.0, 40.0), (40.0, 0.0)], Color::MAGENTA);
    points.draw(&mut grid);
}
