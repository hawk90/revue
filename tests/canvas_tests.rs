//! Tests for Canvas widget

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{
    braille_canvas, canvas, Arc, BrailleGrid, Circle, ClipRegion, FilledCircle, FilledPolygon,
    FilledRectangle, Layer, Line, Points, Polygon, Rectangle, Shape, Transform,
};

// =============================================================================
// Shape Tests
// =============================================================================

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
fn test_circle_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let circle = Circle::new(20.0, 20.0, 10.0, Color::CYAN);
    circle.draw(&mut grid);
}

#[test]
fn test_filled_circle_creation() {
    let circle = FilledCircle::new(15.0, 15.0, 8.0, Color::GREEN);
    assert_eq!(circle.x, 15.0);
    assert_eq!(circle.y, 15.0);
    assert_eq!(circle.radius, 8.0);
}

#[test]
fn test_filled_circle_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let circle = FilledCircle::new(20.0, 20.0, 5.0, Color::YELLOW);
    circle.draw(&mut grid);
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
fn test_rectangle_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let rect = Rectangle::new(5.0, 5.0, 20.0, 15.0, Color::WHITE);
    rect.draw(&mut grid);
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
fn test_filled_rectangle_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let rect = FilledRectangle::new(5.0, 5.0, 10.0, 8.0, Color::BLUE);
    rect.draw(&mut grid);
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

// =============================================================================
// Arc Tests
// =============================================================================

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
fn test_arc_full_circle() {
    let mut grid = BrailleGrid::new(20, 10);
    let arc = Arc::new(20.0, 20.0, 10.0, 0.0, std::f64::consts::TAU, Color::GREEN);
    arc.draw(&mut grid);
}

#[test]
fn test_arc_reverse_direction() {
    let mut grid = BrailleGrid::new(20, 10);
    // End angle less than start angle should still work
    let arc = Arc::new(20.0, 20.0, 10.0, std::f64::consts::PI, 0.0, Color::YELLOW);
    arc.draw(&mut grid);
}

// =============================================================================
// Polygon Tests
// =============================================================================

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
fn test_polygon_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let polygon = Polygon::new(
        vec![(10.0, 10.0), (30.0, 10.0), (30.0, 30.0), (10.0, 30.0)],
        Color::CYAN,
    );
    polygon.draw(&mut grid);
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
fn test_filled_polygon_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let polygon = FilledPolygon::new(
        vec![(10.0, 10.0), (30.0, 10.0), (20.0, 30.0)],
        Color::MAGENTA,
    );
    polygon.draw(&mut grid);
}

// =============================================================================
// Transform Tests
// =============================================================================

#[test]
fn test_transform_identity() {
    let t = Transform::identity();
    let (x, y) = t.apply(5.0, 10.0);
    assert!((x - 5.0).abs() < 0.001);
    assert!((y - 10.0).abs() < 0.001);
}

#[test]
fn test_transform_translate() {
    let t = Transform::translate(10.0, 20.0);
    let (x, y) = t.apply(5.0, 5.0);
    assert!((x - 15.0).abs() < 0.001);
    assert!((y - 25.0).abs() < 0.001);
}

#[test]
fn test_transform_scale() {
    let t = Transform::scale(2.0, 3.0);
    let (x, y) = t.apply(5.0, 10.0);
    assert!((x - 10.0).abs() < 0.001);
    assert!((y - 30.0).abs() < 0.001);
}

#[test]
fn test_transform_scale_uniform() {
    let t = Transform::scale_uniform(2.0);
    let (x, y) = t.apply(5.0, 10.0);
    assert!((x - 10.0).abs() < 0.001);
    assert!((y - 20.0).abs() < 0.001);
}

#[test]
fn test_transform_rotate() {
    let t = Transform::rotate(std::f64::consts::FRAC_PI_2);
    let (x, y) = t.apply(1.0, 0.0);
    assert!(x.abs() < 0.001);
    assert!((y - 1.0).abs() < 0.001);
}

#[test]
fn test_transform_rotate_degrees() {
    let t = Transform::rotate_degrees(90.0);
    let (x, y) = t.apply(1.0, 0.0);
    assert!(x.abs() < 0.001);
    assert!((y - 1.0).abs() < 0.001);
}

#[test]
fn test_transform_chain() {
    let t = Transform::translate(10.0, 0.0).then(&Transform::scale(2.0, 2.0));
    let (x, y) = t.apply(5.0, 5.0);
    // First scale: (10, 10), then translate: (20, 10)
    assert!((x - 20.0).abs() < 0.001);
    assert!((y - 10.0).abs() < 0.001);
}

#[test]
fn test_transform_with_translate() {
    let t = Transform::identity().with_translate(10.0, 20.0);
    let (x, y) = t.apply(0.0, 0.0);
    assert!((x - 10.0).abs() < 0.001);
    assert!((y - 20.0).abs() < 0.001);
}

#[test]
fn test_transform_with_scale() {
    let t = Transform::identity().with_scale(2.0, 3.0);
    let (x, y) = t.apply(5.0, 5.0);
    assert!((x - 10.0).abs() < 0.001);
    assert!((y - 15.0).abs() < 0.001);
}

#[test]
fn test_transform_with_rotate() {
    let t = Transform::identity().with_rotate(std::f64::consts::PI);
    let (x, y) = t.apply(1.0, 0.0);
    assert!((x + 1.0).abs() < 0.001);
    assert!(y.abs() < 0.001);
}

#[test]
fn test_transform_default() {
    let t = Transform::default();
    let (x, y) = t.apply(5.0, 10.0);
    assert!((x - 5.0).abs() < 0.001);
    assert!((y - 10.0).abs() < 0.001);
}

// =============================================================================
// ClipRegion Tests
// =============================================================================

#[test]
fn test_clip_region_creation() {
    let clip = ClipRegion::new(10.0, 20.0, 30.0, 40.0);
    assert_eq!(clip.x_min, 10.0);
    assert_eq!(clip.y_min, 20.0);
    assert_eq!(clip.x_max, 40.0);
    assert_eq!(clip.y_max, 60.0);
}

#[test]
fn test_clip_region_from_bounds() {
    let clip = ClipRegion::from_bounds(0.0, 0.0, 100.0, 100.0);
    assert_eq!(clip.x_min, 0.0);
    assert_eq!(clip.y_min, 0.0);
    assert_eq!(clip.x_max, 100.0);
    assert_eq!(clip.y_max, 100.0);
}

#[test]
fn test_clip_region_contains() {
    let clip = ClipRegion::new(10.0, 10.0, 20.0, 20.0);

    assert!(clip.contains(15.0, 15.0)); // Inside
    assert!(clip.contains(10.0, 10.0)); // On min edge
    assert!(clip.contains(30.0, 30.0)); // On max edge
    assert!(!clip.contains(5.0, 15.0)); // Left of region
    assert!(!clip.contains(35.0, 15.0)); // Right of region
    assert!(!clip.contains(15.0, 5.0)); // Above region
    assert!(!clip.contains(15.0, 35.0)); // Below region
}

#[test]
fn test_clip_region_intersect() {
    let clip1 = ClipRegion::new(0.0, 0.0, 20.0, 20.0);
    let clip2 = ClipRegion::new(10.0, 10.0, 20.0, 20.0);

    let intersection = clip1.intersect(&clip2).unwrap();
    assert_eq!(intersection.x_min, 10.0);
    assert_eq!(intersection.y_min, 10.0);
    assert_eq!(intersection.x_max, 20.0);
    assert_eq!(intersection.y_max, 20.0);
}

#[test]
fn test_clip_region_no_intersect() {
    let clip1 = ClipRegion::new(0.0, 0.0, 10.0, 10.0);
    let clip2 = ClipRegion::new(20.0, 20.0, 10.0, 10.0);

    assert!(clip1.intersect(&clip2).is_none());
}

// =============================================================================
// Layer Tests
// =============================================================================

#[test]
fn test_layer_creation() {
    let layer = Layer::new(40, 20);
    assert_eq!(layer.width(), 80); // 40 * 2
    assert_eq!(layer.height(), 80); // 20 * 4
    assert!(layer.is_visible());
    assert!((layer.opacity() - 1.0).abs() < 0.001);
}

#[test]
fn test_layer_visibility() {
    let mut layer = Layer::new(40, 20);
    assert!(layer.is_visible());

    layer.set_visible(false);
    assert!(!layer.is_visible());

    layer.set_visible(true);
    assert!(layer.is_visible());
}

#[test]
fn test_layer_opacity() {
    let mut layer = Layer::new(40, 20);
    assert!((layer.opacity() - 1.0).abs() < 0.001);

    layer.set_opacity(0.5);
    assert!((layer.opacity() - 0.5).abs() < 0.001);

    layer.set_opacity(0.0);
    assert!((layer.opacity() - 0.0).abs() < 0.001);

    // Test clamping
    layer.set_opacity(2.0);
    assert!((layer.opacity() - 1.0).abs() < 0.001);

    layer.set_opacity(-1.0);
    assert!((layer.opacity() - 0.0).abs() < 0.001);
}

#[test]
fn test_layer_draw_shape() {
    let mut layer = Layer::new(40, 20);
    layer.draw(&Circle::new(20.0, 20.0, 10.0, Color::RED));
    // Shape should be drawn on the layer
}

#[test]
fn test_layer_clear() {
    let mut layer = Layer::new(40, 20);
    layer.draw(&Circle::new(20.0, 20.0, 10.0, Color::RED));
    layer.clear();
    // Layer should be cleared
}

#[test]
fn test_layer_set_dot() {
    let mut layer = Layer::new(40, 20);
    layer.set(10, 10, Color::BLUE);
    // Dot should be set
}

#[test]
fn test_layer_composite() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut layer = Layer::new(40, 20);

    layer.draw(&Circle::new(20.0, 20.0, 10.0, Color::RED));
    grid.composite_layer(&layer);
    // Layer should be composited onto grid
}

#[test]
fn test_layer_composite_invisible() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut layer = Layer::new(40, 20);

    layer.draw(&Circle::new(20.0, 20.0, 10.0, Color::RED));
    layer.set_visible(false);

    // Pre-draw something on the grid
    grid.draw(&Line::new(0.0, 0.0, 10.0, 10.0, Color::WHITE));

    grid.composite_layer(&layer);
    // Invisible layer should not affect grid
}

#[test]
fn test_layer_composite_zero_opacity() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut layer = Layer::new(40, 20);

    layer.draw(&Circle::new(20.0, 20.0, 10.0, Color::RED));
    layer.set_opacity(0.0);

    grid.composite_layer(&layer);
    // Zero opacity layer should not affect grid
}

// =============================================================================
// BrailleGrid Tests
// =============================================================================

#[test]
fn test_braille_grid_creation() {
    let grid = BrailleGrid::new(40, 20);
    assert_eq!(grid.width(), 80); // 40 * 2
    assert_eq!(grid.height(), 80); // 20 * 4
}

#[test]
fn test_braille_grid_set() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(5, 5, Color::RED);
    // Dot should be set
}

#[test]
fn test_braille_grid_set_bounds() {
    let mut grid = BrailleGrid::new(10, 10);
    // Should not crash when setting out of bounds
    grid.set(1000, 1000, Color::RED);
}

#[test]
fn test_braille_grid_clear() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(5, 5, Color::RED);
    grid.clear();
    // Grid should be cleared
}

#[test]
fn test_braille_grid_draw_shape() {
    let mut grid = BrailleGrid::new(20, 10);
    grid.draw(&Line::new(0.0, 0.0, 20.0, 20.0, Color::WHITE));
}

#[test]
fn test_braille_grid_render() {
    let mut grid = BrailleGrid::new(20, 10);
    grid.draw(&Circle::new(10.0, 10.0, 5.0, Color::CYAN));

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    grid.render(&mut buffer, area);
}

// =============================================================================
// Canvas Widget Tests
// =============================================================================

#[test]
fn test_canvas_creation() {
    let c = canvas(|_ctx| {});
    let _ = c;
}

#[test]
fn test_canvas_draw_basic() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.set(5, 5, 'X');
        ctx.text(0, 0, "Hello", Some(Color::WHITE));
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_canvas_draw_shapes() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.hline(0, 0, 10, '-', Some(Color::WHITE));
        ctx.vline(0, 0, 5, '|', Some(Color::WHITE));
        ctx.rect(2, 2, 8, 4, Some(Color::CYAN));
        ctx.bar(0, 5, 10, Color::GREEN, None);
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_canvas_partial_bar() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.partial_bar(0, 0, 5.5, Color::BLUE);
    });

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_canvas_line_drawing() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.line(0, 0, 15, 8, '*', Some(Color::YELLOW));
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_canvas_fill_rect() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.fill_rect(
            Rect::new(2, 2, 5, 3),
            '#',
            Some(Color::RED),
            Some(Color::BLACK),
        );
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_canvas_point() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.point(5, 5, Color::MAGENTA);
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_canvas_clear() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.set(5, 5, 'X');
        ctx.clear();
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

// =============================================================================
// BrailleCanvas Widget Tests
// =============================================================================

#[test]
fn test_braille_canvas_creation() {
    let bc = braille_canvas(|_ctx| {});
    let _ = bc;
}

#[test]
fn test_braille_canvas_draw_basic() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        ctx.set(10, 10, Color::WHITE);
        ctx.line(0.0, 0.0, 20.0, 40.0, Color::CYAN);
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_canvas_shapes() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        ctx.circle(20.0, 20.0, 10.0, Color::RED);
        ctx.filled_circle(40.0, 20.0, 8.0, Color::GREEN);
        ctx.rect(5.0, 5.0, 15.0, 10.0, Color::BLUE);
        ctx.filled_rect(50.0, 5.0, 15.0, 10.0, Color::YELLOW);
    });

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_canvas_points() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        ctx.points(
            vec![(0.0, 0.0), (10.0, 20.0), (20.0, 0.0), (30.0, 20.0)],
            Color::MAGENTA,
        );
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_canvas_arc() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        ctx.arc(20.0, 20.0, 10.0, 0.0, std::f64::consts::PI, Color::CYAN);
        ctx.arc_degrees(40.0, 20.0, 10.0, 0.0, 270.0, Color::YELLOW);
    });

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_canvas_polygon() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        ctx.polygon(vec![(10.0, 10.0), (30.0, 10.0), (20.0, 30.0)], Color::RED);
        ctx.regular_polygon(50.0, 20.0, 10.0, 6, Color::BLUE);
    });

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_canvas_filled_polygon() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        ctx.filled_polygon(vec![(10.0, 10.0), (30.0, 10.0), (20.0, 30.0)], Color::GREEN);
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_canvas_dimensions() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        let w = ctx.width();
        let h = ctx.height();
        // Draw border using dimensions
        ctx.rect(0.0, 0.0, w as f64 - 1.0, h as f64 - 1.0, Color::WHITE);
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_canvas_clear() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        ctx.circle(20.0, 20.0, 10.0, Color::RED);
        ctx.clear();
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

// =============================================================================
// DrawContext Tests
// =============================================================================

#[test]
fn test_draw_context_dimensions() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        // Just verify we can call these methods without panicking
        let w = ctx.width();
        let h = ctx.height();
        assert!(w > 0);
        assert!(h > 0);
    });

    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_draw_context_area() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        let area = ctx.area();
        assert!(area.width > 0);
        assert!(area.height > 0);
    });

    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(5, 5, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_draw_context_styled() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.set_styled(5, 5, 'X', Some(Color::RED), Some(Color::BLUE));
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_draw_context_text_bold() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.text_bold(0, 0, "Bold Text", Some(Color::WHITE));
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}
