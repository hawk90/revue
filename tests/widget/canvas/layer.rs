//! Layer tests
//!
//! Construction, visibility and opacity clamping are covered in
//! tests/canvas/layer.rs, compositing in braille/grid_impl.rs; these cover
//! drawing on a layer.

use revue::style::Color;
use revue::widget::{Circle, Layer, Line};

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
