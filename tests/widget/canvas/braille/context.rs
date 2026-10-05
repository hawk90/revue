//! BrailleContext tests
//!
//! Each drawing method is a shorthand for drawing the matching shape; the
//! shapes themselves are checked dot by dot in shapes.rs.

use revue::style::Color;
use revue::widget::{
    Arc, BrailleContext, BrailleGrid, Circle, FilledCircle, FilledPolygon, FilledRectangle, Line,
    Points, Polygon, Rectangle, Shape,
};

/// Raised dots and colors after `f` draws through a context.
fn via_context(f: impl FnOnce(&mut BrailleContext)) -> (Vec<u8>, Vec<Option<Color>>) {
    let mut grid = BrailleGrid::new(20, 10);
    let mut ctx = BrailleContext::new(&mut grid);
    f(&mut ctx);
    (grid.cells().to_vec(), grid.colors().to_vec())
}

/// Raised dots and colors after drawing `shape` straight onto a grid.
fn via_shape(shape: &impl Shape) -> (Vec<u8>, Vec<Option<Color>>) {
    let mut grid = BrailleGrid::new(20, 10);
    grid.draw(shape);
    (grid.cells().to_vec(), grid.colors().to_vec())
}

fn assert_nonempty(drawn: &(Vec<u8>, Vec<Option<Color>>)) {
    assert!(drawn.0.iter().any(|&c| c != 0), "nothing was drawn");
}

#[test]
fn test_context_dimensions() {
    let mut grid = BrailleGrid::new(30, 15);
    let ctx = BrailleContext::new(&mut grid);
    assert_eq!(ctx.width(), 60); // 30 * 2
    assert_eq!(ctx.height(), 60); // 15 * 4
}

#[test]
fn test_context_set() {
    let (cells, colors) = via_context(|ctx| ctx.set(5, 5, Color::BLUE));
    // Dot (5, 5) is cell (2, 1), right column, second row
    assert_eq!(cells[20 + 2], 0x10);
    assert_eq!(colors[20 + 2], Some(Color::BLUE));
    assert_eq!(cells.iter().filter(|&&c| c != 0).count(), 1);
}

#[test]
fn test_context_clear() {
    let mut grid = BrailleGrid::new(20, 10);
    {
        let mut ctx = BrailleContext::new(&mut grid);
        ctx.set(10, 10, Color::RED);
        ctx.circle(20.0, 20.0, 5.0, Color::RED);
        ctx.clear();
    }
    assert!(grid.cells().iter().all(|&c| c == 0));
    assert!(grid.colors().iter().all(|c| c.is_none()));
}

#[test]
fn test_context_draw_shape() {
    let shape = Circle::new(20.0, 20.0, 7.0, Color::GREEN);
    let drawn = via_context(|ctx| ctx.draw(&shape));
    assert_nonempty(&drawn);
    assert_eq!(drawn, via_shape(&shape));
}

#[test]
fn test_context_line() {
    let drawn = via_context(|ctx| ctx.line(0.0, 0.0, 30.0, 20.0, Color::RED));
    assert_nonempty(&drawn);
    assert_eq!(
        drawn,
        via_shape(&Line::new(0.0, 0.0, 30.0, 20.0, Color::RED))
    );
}

#[test]
fn test_context_circle() {
    let drawn = via_context(|ctx| ctx.circle(20.0, 20.0, 10.0, Color::GREEN));
    assert_nonempty(&drawn);
    assert_eq!(
        drawn,
        via_shape(&Circle::new(20.0, 20.0, 10.0, Color::GREEN))
    );
}

#[test]
fn test_context_filled_circle() {
    let drawn = via_context(|ctx| ctx.filled_circle(20.0, 20.0, 5.0, Color::BLUE));
    assert_nonempty(&drawn);
    assert_eq!(
        drawn,
        via_shape(&FilledCircle::new(20.0, 20.0, 5.0, Color::BLUE))
    );
}

#[test]
fn test_context_rect() {
    let drawn = via_context(|ctx| ctx.rect(5.0, 5.0, 20.0, 10.0, Color::YELLOW));
    assert_nonempty(&drawn);
    assert_eq!(
        drawn,
        via_shape(&Rectangle::new(5.0, 5.0, 20.0, 10.0, Color::YELLOW))
    );
}

#[test]
fn test_context_filled_rect() {
    let drawn = via_context(|ctx| ctx.filled_rect(5.0, 5.0, 20.0, 10.0, Color::CYAN));
    assert_nonempty(&drawn);
    assert_eq!(
        drawn,
        via_shape(&FilledRectangle::new(5.0, 5.0, 20.0, 10.0, Color::CYAN))
    );
}

#[test]
fn test_context_points() {
    let coords = vec![(0.0, 0.0), (10.0, 10.0), (20.0, 5.0)];
    let drawn = via_context(|ctx| ctx.points(coords.clone(), Color::MAGENTA));
    assert_nonempty(&drawn);
    assert_eq!(drawn, via_shape(&Points::new(coords, Color::MAGENTA)));
}

#[test]
fn test_context_points_empty() {
    let (cells, _) = via_context(|ctx| ctx.points(vec![], Color::MAGENTA));
    assert!(cells.iter().all(|&c| c == 0));
}

#[test]
fn test_context_arc() {
    let drawn =
        via_context(|ctx| ctx.arc(20.0, 20.0, 10.0, 0.0, std::f64::consts::PI, Color::WHITE));
    assert_nonempty(&drawn);
    assert_eq!(
        drawn,
        via_shape(&Arc::new(
            20.0,
            20.0,
            10.0,
            0.0,
            std::f64::consts::PI,
            Color::WHITE
        ))
    );
}

#[test]
fn test_context_arc_degrees() {
    let drawn = via_context(|ctx| ctx.arc_degrees(20.0, 20.0, 10.0, 0.0, 270.0, Color::WHITE));
    assert_nonempty(&drawn);
    assert_eq!(
        drawn,
        via_shape(&Arc::from_degrees(
            20.0,
            20.0,
            10.0,
            0.0,
            270.0,
            Color::WHITE
        ))
    );
}

#[test]
fn test_context_polygon() {
    let vertices = vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)];
    let drawn = via_context(|ctx| ctx.polygon(vertices.clone(), Color::RED));
    assert_nonempty(&drawn);
    assert_eq!(drawn, via_shape(&Polygon::new(vertices, Color::RED)));
}

#[test]
fn test_context_polygon_empty() {
    let (cells, _) = via_context(|ctx| ctx.polygon(vec![], Color::RED));
    assert!(cells.iter().all(|&c| c == 0));
}

#[test]
fn test_context_regular_polygon() {
    for sides in [3, 6] {
        let drawn = via_context(|ctx| ctx.regular_polygon(20.0, 20.0, 10.0, sides, Color::GREEN));
        assert_nonempty(&drawn);
        assert_eq!(
            drawn,
            via_shape(&Polygon::regular(20.0, 20.0, 10.0, sides, Color::GREEN)),
            "{sides} sides"
        );
    }
}

#[test]
fn test_context_filled_polygon() {
    let vertices = vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)];
    let drawn = via_context(|ctx| ctx.filled_polygon(vertices.clone(), Color::YELLOW));
    assert_nonempty(&drawn);
    assert_eq!(
        drawn,
        via_shape(&FilledPolygon::new(vertices, Color::YELLOW))
    );
}

#[test]
fn test_context_filled_polygon_empty() {
    let (cells, _) = via_context(|ctx| ctx.filled_polygon(vec![], Color::YELLOW));
    assert!(cells.iter().all(|&c| c == 0));
}

#[test]
fn test_context_large_radius_is_clipped() {
    // A circle far bigger than the grid leaves the grid untouched
    let (cells, _) = via_context(|ctx| ctx.circle(20.0, 20.0, 1000.0, Color::YELLOW));
    assert!(cells.iter().all(|&c| c == 0));
}

#[test]
fn test_context_later_shape_color_wins() {
    let (_, colors) = via_context(|ctx| {
        ctx.filled_rect(0.0, 0.0, 3.0, 3.0, Color::RED);
        ctx.set(0, 0, Color::BLUE);
    });
    assert_eq!(colors[0], Some(Color::BLUE));
    assert_eq!(colors[1], Some(Color::RED));
}
