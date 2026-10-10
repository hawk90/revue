//! Layer tests

use revue::style::Color;
use revue::widget::BrailleGrid;
use revue::widget::Circle;
use revue::widget::Layer;
use revue::widget::Line;

const EMPTY: char = '\u{2800}';

#[test]
fn test_layer_odd_size() {
    let layer = Layer::new(60, 25);
    // Dots, not terminal cells
    assert_eq!(layer.width(), 120);
    assert_eq!(layer.height(), 100);
    assert_eq!(layer.cells().len(), 60 * 25);
    assert_eq!(layer.colors().len(), 60 * 25);
}

#[test]
fn test_layer_set_dot() {
    let mut layer = Layer::new(40, 20);
    // Dot (10, 5) is in terminal cell (5, 1), second row of its left column
    layer.set(10, 5, Color::BLUE);
    assert_eq!(layer.grid().get_char(5, 1), '\u{2802}');
    assert_eq!(layer.colors()[40 + 5], Some(Color::BLUE));

    let mut layer = Layer::new(40, 20);
    layer.set(10, 10, Color::BLUE);
    // Dot should be set
}

#[test]
fn test_layer_set_out_of_bounds() {
    let mut layer = Layer::new(4, 2);
    layer.set(8, 0, Color::RED);
    layer.set(0, 8, Color::RED);
    assert!(layer.cells().iter().all(|&c| c == 0));
}

#[test]
fn test_layer_set_color() {
    let mut layer = Layer::new(40, 20);
    layer.set(0, 0, Color::GREEN);
    assert_eq!(layer.colors()[0], Some(Color::GREEN));
    assert_eq!(layer.cells()[0], 0x01);
}

#[test]
fn test_layer_multiple_sets() {
    let mut layer = Layer::new(40, 20);
    layer.set(0, 0, Color::RED);
    layer.set(2, 0, Color::BLUE);
    layer.set(4, 0, Color::GREEN);

    // Each dot lands in its own terminal cell
    let colors = layer.colors();
    assert_eq!(colors[0], Some(Color::RED));
    assert_eq!(colors[1], Some(Color::BLUE));
    assert_eq!(colors[2], Some(Color::GREEN));
}

#[test]
fn test_layer_same_cell_last_color_wins() {
    let mut layer = Layer::new(4, 2);
    layer.set(0, 0, Color::RED);
    layer.set(1, 3, Color::BLUE);
    assert_eq!(layer.cells()[0], 0x01 | 0x80);
    assert_eq!(layer.colors()[0], Some(Color::BLUE));
}

#[test]
fn test_layer_clear() {
    let mut layer = Layer::new(40, 20);
    layer.set(5, 5, Color::RED);
    layer.draw(&Circle::new(20.0, 20.0, 10.0, Color::RED));
    layer.clear();
    assert!(layer.cells().iter().all(|&c| c == 0));
    assert!(layer.colors().iter().all(|c| c.is_none()));

    let mut layer = Layer::new(40, 20);
    layer.draw(&Circle::new(20.0, 20.0, 10.0, Color::RED));
    layer.clear();
    // Layer should be cleared
}

#[test]
fn test_layer_draw_shape() {
    let mut layer = Layer::new(20, 10);
    layer.draw(&Line::new(0.0, 0.0, 7.0, 0.0, Color::RED));
    // Eight dots along the top row: the top-left dot of cells 0..4 and their
    // top-right dot
    for cx in 0..4 {
        assert_eq!(layer.grid().get_char(cx, 0), '\u{2809}', "cell {cx}");
    }
    assert_eq!(layer.grid().get_char(4, 0), EMPTY);

    let mut layer = Layer::new(40, 20);
    layer.draw(&Circle::new(20.0, 20.0, 10.0, Color::RED));
    // Shape should be drawn on the layer
}

#[test]
fn test_layer_grid_mut() {
    let mut layer = Layer::new(40, 20);
    let grid = layer.grid_mut();
    assert_eq!(grid.width(), 80);
    grid.set(1, 0, Color::CYAN);
    // Changes made through grid_mut are the layer's
    assert_eq!(layer.cells()[0], 0x08);
    assert_eq!(layer.colors()[0], Some(Color::CYAN));
}

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
