//! Layer tests

use revue::style::Color;
use revue::widget::canvas::Layer;

#[test]
fn test_layer_new() {
    let layer = Layer::new(40, 20);
    assert!(layer.is_visible());
    assert_eq!(layer.opacity(), 1.0);
    // Layer width/height returns braille dots, not terminal cells
    assert_eq!(layer.width(), 80); // 40 * 2
    assert_eq!(layer.height(), 80); // 20 * 4
}

#[test]
fn test_layer_set_visible() {
    let mut layer = Layer::new(40, 20);
    assert!(layer.is_visible());

    layer.set_visible(false);
    assert!(!layer.is_visible());

    layer.set_visible(true);
    assert!(layer.is_visible());
}

#[test]
fn test_layer_set_opacity() {
    let mut layer = Layer::new(40, 20);

    layer.set_opacity(0.5);
    assert_eq!(layer.opacity(), 0.5);

    layer.set_opacity(1.5);
    // Should clamp to 1.0
    assert_eq!(layer.opacity(), 1.0);

    layer.set_opacity(-0.5);
    // Should clamp to 0.0
    assert_eq!(layer.opacity(), 0.0);
}

#[test]
fn test_layer_opacity_clamp_max() {
    let mut layer = Layer::new(40, 20);
    layer.set_opacity(2.0);
    assert_eq!(layer.opacity(), 1.0);
}

#[test]
fn test_layer_opacity_clamp_min() {
    let mut layer = Layer::new(40, 20);
    layer.set_opacity(-1.0);
    assert_eq!(layer.opacity(), 0.0);
}

#[test]
fn test_layer_set_and_get() {
    let mut layer = Layer::new(40, 20);

    layer.set_opacity(0.75);
    assert_eq!(layer.opacity(), 0.75);
}

#[test]
fn test_layer_width() {
    let layer = Layer::new(60, 25);
    // BrailleGrid width is term_width * 2, height is term_height * 4
    assert_eq!(layer.width(), 120); // 60 * 2
    assert_eq!(layer.height(), 100); // 25 * 4
}

#[test]
fn test_layer_clear() {
    let mut layer = Layer::new(40, 20);
    layer.set(5, 5, Color::RED);

    layer.clear();
    // Layer should be cleared
    let cells = layer.cells();
    // All cells should be 0 (empty)
    assert!(cells.iter().all(|&c| c == 0));
}

#[test]
fn test_layer_set_dot() {
    let mut layer = Layer::new(40, 20);
    // Set at braille coordinates (10, 5)
    layer.set(10, 5, Color::BLUE);

    let colors = layer.colors();
    // Calculate the terminal cell position
    // cell_x = 10 / 2 = 5, cell_y = 5 / 4 = 1
    // cell_idx = cell_y * term_width + cell_x = 1 * 40 + 5 = 45
    let expected_cell = 1 * 40 + 5; // cell_y * term_width + cell_x
    assert_eq!(colors[expected_cell], Some(Color::BLUE));
}

#[test]
fn test_layer_grid_borrow() {
    let layer = Layer::new(40, 20);
    let _grid = layer.grid();
    // Just verify we can borrow the grid
}

#[test]
fn test_layer_grid_mut() {
    let mut layer = Layer::new(40, 20);
    let grid = layer.grid_mut();
    // BrailleGrid width is term_width * 2
    assert_eq!(grid.width(), 80); // 40 * 2
}

#[test]
fn test_layer_get_cells() {
    let layer = Layer::new(40, 20);
    let cells = layer.cells();
    assert_eq!(cells.len(), 40 * 20);
}

#[test]
fn test_layer_get_colors() {
    let layer = Layer::new(40, 20);
    let colors = layer.colors();
    assert_eq!(colors.len(), 40 * 20);
}

#[test]
fn test_layer_default_visible() {
    let layer = Layer::new(30, 15);
    assert!(layer.is_visible());
}

#[test]
fn test_layer_default_opacity() {
    let layer = Layer::new(30, 15);
    assert_eq!(layer.opacity(), 1.0);
}

#[test]
fn test_layer_opacity_boundary_values() {
    let mut layer = Layer::new(40, 20);

    layer.set_opacity(0.0);
    assert_eq!(layer.opacity(), 0.0);

    layer.set_opacity(1.0);
    assert_eq!(layer.opacity(), 1.0);
}

#[test]
fn test_layer_set_color() {
    let mut layer = Layer::new(40, 20);
    layer.set(0, 0, Color::GREEN);

    let colors = layer.colors();
    assert_eq!(colors[0], Some(Color::GREEN));
}

#[test]
fn test_layer_multiple_sets() {
    let mut layer = Layer::new(40, 20);
    layer.set(0, 0, Color::RED);
    layer.set(2, 0, Color::BLUE);
    layer.set(4, 0, Color::GREEN);

    let colors = layer.colors();
    // Calculate terminal cell positions
    // For (0,0): cell_x=0, cell_y=0, idx=0*40+0=0
    // For (2,0): cell_x=1, cell_y=0, idx=0*40+1=1
    // For (4,0): cell_x=2, cell_y=0, idx=0*40+2=2
    assert_eq!(colors[0], Some(Color::RED));
    assert_eq!(colors[1], Some(Color::BLUE));
    assert_eq!(colors[2], Some(Color::GREEN));
}

#[test]
fn test_layer_clear_resets_all() {
    let mut layer = Layer::new(40, 20);
    layer.set(0, 0, Color::RED);
    layer.set(1, 1, Color::BLUE);
    layer.set(2, 2, Color::GREEN);

    layer.clear();

    let colors = layer.colors();
    // All colors should be None (cleared)
    assert!(colors.iter().all(|c| c.is_none()));
}
