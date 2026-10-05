//! BrailleGrid tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::{BrailleGrid, Layer};

fn sym(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).map(|c| c.symbol).unwrap_or('\0')
}

// =========================================================================
// BrailleGrid::new tests
// =========================================================================

#[test]
fn test_braille_grid_new_small() {
    let grid = BrailleGrid::new(5, 5);
    assert_eq!(grid.width(), 10); // 5 * 2
    assert_eq!(grid.height(), 20); // 5 * 4
}

#[test]
fn test_braille_grid_new_empty() {
    let grid = BrailleGrid::new(0, 0);
    assert_eq!(grid.width(), 0);
    assert_eq!(grid.height(), 0);
    assert_eq!(grid.cells().len(), 0);
}

#[test]
fn test_braille_grid_new_single_cell() {
    let grid = BrailleGrid::new(1, 1);
    assert_eq!(grid.width(), 2);
    assert_eq!(grid.height(), 4);
    assert_eq!(grid.cells().len(), 1);
}

// =========================================================================
// width and height tests
// =========================================================================

// =========================================================================
// set tests
// =========================================================================

#[test]
fn test_braille_grid_set_single_dot() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(0, 0, Color::RED);
    // Cell (0,0) should have the dot pattern set
    assert_eq!(grid.cells()[0], 0x01); // First dot
    assert_eq!(grid.colors()[0], Some(Color::RED));
}

#[test]
fn test_braille_grid_set_multiple_dots_same_cell() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(0, 0, Color::RED);
    grid.set(1, 0, Color::BLUE);
    grid.set(0, 1, Color::GREEN);
    // Cell (0,0) has the three dots OR'd together
    assert_eq!(grid.cells()[0], 0x01 | 0x08 | 0x02);
    assert_eq!(grid.colors()[0], Some(Color::GREEN)); // Last color wins
}

#[test]
fn test_braille_grid_dot_bit_positions() {
    // Unicode braille numbering: dots 1-3 and 7 down the left column,
    // dots 4-6 and 8 down the right column
    let expected = [
        ((0, 0), 0x01),
        ((0, 1), 0x02),
        ((0, 2), 0x04),
        ((0, 3), 0x40),
        ((1, 0), 0x08),
        ((1, 1), 0x10),
        ((1, 2), 0x20),
        ((1, 3), 0x80),
    ];
    for ((x, y), bit) in expected {
        let mut grid = BrailleGrid::new(1, 1);
        grid.set(x, y, Color::WHITE);
        assert_eq!(grid.cells()[0], bit, "dot ({x}, {y})");
        assert_eq!(
            grid.get_char(0, 0),
            char::from_u32(0x2800 + bit as u32).unwrap(),
            "dot ({x}, {y})"
        );
    }
}

#[test]
fn test_braille_grid_set_all_dots_in_cell() {
    let mut grid = BrailleGrid::new(10, 10);
    // Set all 8 dots in the first cell (0,0 to 1,3)
    for x in 0..2 {
        for y in 0..4 {
            grid.set(x, y, Color::WHITE);
        }
    }
    // Should be 0xFF (all dots set)
    assert_eq!(grid.cells()[0], 0xFF);
}

#[test]
fn test_braille_grid_set_out_of_bounds() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(100, 100, Color::RED);
    grid.set(20, 0, Color::RED); // width is 20
    grid.set(0, 40, Color::RED); // height is 40
                                 // Should not panic, dots should be ignored
                                 // All cells should remain 0
    assert!(grid.cells().iter().all(|&c| c == 0));
}

#[test]
fn test_braille_grid_set_at_boundary() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(0, 0, Color::RED);
    grid.set(19, 0, Color::RED); // max width - 1
    grid.set(0, 39, Color::RED); // max height - 1
    assert_eq!(grid.cells()[0], 0x01);
    assert_eq!(grid.cells()[9], 0x08); // top-right dot of the last column
    assert_eq!(grid.cells()[90], 0x40); // bottom-left dot of the last row
}

#[test]
fn test_braille_grid_set_different_cells() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(0, 0, Color::RED); // cell (0,0)
    grid.set(2, 0, Color::BLUE); // cell (1,0)
    grid.set(0, 4, Color::GREEN); // cell (0,1)
                                  // Different cells should be set
    assert_ne!(grid.cells()[0], 0);
    assert_ne!(grid.cells()[1], 0);
    assert_ne!(grid.cells()[10], 0); // cell (0,1) is at index 10
}

// =========================================================================
// clear tests
// =========================================================================

#[test]
fn test_braille_grid_clear() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(0, 0, Color::RED);
    grid.set(1, 1, Color::BLUE);
    grid.clear();
    // All cells should be cleared
    assert!(grid.cells().iter().all(|&c| c == 0));
    assert!(grid.colors().iter().all(|c| c.is_none()));
}

#[test]
fn test_braille_grid_clear_empty() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.clear();
    // Should not panic
    assert!(grid.cells().iter().all(|&c| c == 0));
}

#[test]
fn test_braille_grid_clear_multiple_times() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(5, 5, Color::RED);
    grid.clear();
    grid.clear();
    // Should not panic
    assert!(grid.cells().iter().all(|&c| c == 0));
}

// =========================================================================
// get_char tests
// =========================================================================

#[test]
fn test_braille_grid_get_char_empty() {
    let grid = BrailleGrid::new(10, 10);
    let ch = grid.get_char(0, 0);
    assert_eq!(ch, '⠀'); // Braille blank
}

#[test]
fn test_braille_grid_get_char_single_dot() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(0, 0, Color::RED);
    let ch = grid.get_char(0, 0);
    assert_eq!(ch, '⠁'); // First dot pattern
}

#[test]
fn test_braille_grid_get_char_all_dots() {
    let mut grid = BrailleGrid::new(10, 10);
    for x in 0..2 {
        for y in 0..4 {
            grid.set(x, y, Color::WHITE);
        }
    }
    let ch = grid.get_char(0, 0);
    assert_eq!(ch, '⣿'); // Full braille pattern
}

#[test]
fn test_braille_grid_get_char_out_of_bounds() {
    let grid = BrailleGrid::new(10, 10);
    let ch = grid.get_char(100, 100);
    assert_eq!(ch, '⠀'); // Should return blank for out of bounds
}

#[test]
fn test_braille_grid_get_char_different_cells() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(0, 0, Color::RED);
    grid.set(2, 0, Color::BLUE); // Different cell
    let ch1 = grid.get_char(0, 0);
    let ch2 = grid.get_char(1, 0);
    assert_eq!(ch1, '⠁');
    assert_eq!(ch2, '⠁');
}

// =========================================================================
// render tests
// =========================================================================

#[test]
fn test_braille_grid_render() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(0, 0, Color::RED);
    grid.set(3, 7, Color::BLUE);
    let mut buffer = Buffer::new(10, 10);
    grid.render(&mut buffer, Rect::new(0, 0, 10, 10));
    assert_eq!(sym(&buffer, 0, 0), '⠁');
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
    // Dot (3, 7) is the bottom-right dot of cell (1, 1)
    assert_eq!(sym(&buffer, 1, 1), '\u{2880}');
    assert_eq!(buffer.get(1, 1).unwrap().fg, Some(Color::BLUE));
}

#[test]
fn test_braille_grid_render_empty() {
    // Cells without dots are not written, so the buffer keeps its content
    let grid = BrailleGrid::new(10, 10);
    let mut buffer = Buffer::new(10, 10);
    buffer.set(3, 3, revue::render::Cell::new('#'));
    grid.render(&mut buffer, Rect::new(0, 0, 10, 10));
    assert_eq!(sym(&buffer, 3, 3), '#');
    assert_eq!(sym(&buffer, 0, 0), ' ');
}

#[test]
fn test_braille_grid_render_partial_area() {
    let mut grid = BrailleGrid::new(20, 20);
    grid.set(0, 0, Color::RED);
    let mut buffer = Buffer::new(20, 20);
    grid.render(&mut buffer, Rect::new(5, 5, 10, 10));
    // Drawn relative to the area's origin
    assert_eq!(sym(&buffer, 5, 5), '⠁');
    assert_eq!(sym(&buffer, 0, 0), ' ');
}

#[test]
fn test_braille_grid_render_with_color() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(0, 0, Color::RED);
    grid.set(1, 1, Color::GREEN); // same cell: last color wins
    let mut buffer = Buffer::new(10, 10);
    grid.render(&mut buffer, Rect::new(0, 0, 10, 10));
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, '\u{2811}');
    assert_eq!(cell.fg, Some(Color::GREEN));
}

// =========================================================================
// composite_layer tests
// =========================================================================

#[test]
fn test_braille_grid_composite_layer() {
    let mut grid = BrailleGrid::new(10, 10);
    let mut layer = Layer::new(10, 10);
    layer.set(0, 0, Color::RED);
    layer.set(7, 9, Color::GREEN);
    grid.composite_layer(&layer);
    // Grid has the layer's dots and colors
    assert_eq!(grid.cells(), layer.cells());
    assert_eq!(grid.colors()[0], Some(Color::RED));
    assert_eq!(grid.get_char(3, 2), '\u{2810}');
}

#[test]
fn test_braille_grid_composite_layer_invisible() {
    let mut grid = BrailleGrid::new(10, 10);
    let mut layer = Layer::new(10, 10);
    layer.set_visible(false);
    layer.set(0, 0, Color::RED);
    grid.composite_layer(&layer);
    // Grid should not have the layer's dots
    assert!(grid.cells().iter().all(|&c| c == 0));
}

#[test]
fn test_braille_grid_composite_layer_zero_opacity() {
    let mut grid = BrailleGrid::new(10, 10);
    let mut layer = Layer::new(10, 10);
    layer.set_opacity(0.0);
    layer.set(0, 0, Color::RED);
    grid.composite_layer(&layer);
    // Grid should not have the layer's dots
    assert!(grid.cells().iter().all(|&c| c == 0));
}

#[test]
fn test_braille_grid_composite_layer_partial_opacity() {
    let mut grid = BrailleGrid::new(10, 10);
    let mut layer = Layer::new(10, 10);
    layer.set_opacity(0.5);
    layer.set(0, 0, Color::RED);
    grid.composite_layer(&layer);
    // Opacity only gates compositing: any opacity above zero copies in full
    assert_eq!(grid.cells()[0], 0x01);
    assert_eq!(grid.colors()[0], Some(Color::RED));
}

#[test]
fn test_braille_grid_composite_layer_or_dots() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(0, 0, Color::BLUE);
    let mut layer = Layer::new(10, 10);
    layer.set(1, 0, Color::RED);
    grid.composite_layer(&layer);
    // Grid should have both dots OR'd together
    assert_eq!(grid.cells()[0], 0x01 | 0x08);
    assert_eq!(grid.colors()[0], Some(Color::RED)); // Layer color wins
}

#[test]
fn test_braille_grid_composite_layer_different_sizes() {
    // A smaller layer lands on the same terminal cells it covers, row by row
    let mut grid = BrailleGrid::new(4, 2);
    let mut layer = Layer::new(2, 2);
    layer.set(0, 0, Color::RED); // cell (0, 0)
    layer.set(3, 4, Color::GREEN); // cell (1, 1)
    grid.composite_layer(&layer);
    assert_eq!(grid.get_char(0, 0), '\u{2801}');
    assert_eq!(grid.get_char(1, 1), '\u{2808}');
    assert_eq!(grid.colors()[4 + 1], Some(Color::GREEN));
    // Nothing spills into the cells the layer does not cover
    for (cx, cy) in [(1, 0), (2, 0), (3, 0), (0, 1), (2, 1), (3, 1)] {
        assert_eq!(grid.get_char(cx, cy), '\u{2800}', "cell ({cx}, {cy})");
    }
}

#[test]
fn test_braille_grid_composite_larger_layer() {
    // Parts of a larger layer beyond the grid are dropped
    let mut grid = BrailleGrid::new(2, 1);
    let mut layer = Layer::new(4, 2);
    layer.set(2, 0, Color::RED); // cell (1, 0): inside the grid
    layer.set(6, 0, Color::RED); // cell (3, 0): beyond the grid
    layer.set(0, 4, Color::RED); // cell (0, 1): beyond the grid
    grid.composite_layer(&layer);
    assert_eq!(grid.cells(), &[0, 0x01]);
}

#[test]
fn test_braille_grid_composite_empty_layer() {
    let mut grid = BrailleGrid::new(10, 10);
    let layer = Layer::new(10, 10);
    grid.composite_layer(&layer);
    // Grid should remain unchanged
    assert!(grid.cells().iter().all(|&c| c == 0));
}

// =========================================================================
// cells and colors accessor tests
// =========================================================================

#[test]
fn test_braille_grid_cells() {
    let grid = BrailleGrid::new(10, 10);
    let cells = grid.cells();
    assert_eq!(cells.len(), 100);
}

#[test]
fn test_braille_grid_colors() {
    let grid = BrailleGrid::new(10, 10);
    let colors = grid.colors();
    assert_eq!(colors.len(), 100);
}

#[test]
fn test_braille_grid_colors_after_set() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(0, 0, Color::RED);
    assert_eq!(grid.colors()[0], Some(Color::RED));
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_braille_grid_set_same_dot_multiple_times() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(0, 0, Color::RED);
    grid.set(0, 0, Color::BLUE);
    grid.set(0, 0, Color::GREEN);
    // Dot should remain set (OR with itself)
    assert_eq!(grid.cells()[0], 0x01);
    assert_eq!(grid.colors()[0], Some(Color::GREEN));
}

#[test]
fn test_braille_grid_pattern_order() {
    let mut grid = BrailleGrid::new(10, 10);
    // Set dots in different positions to verify pattern
    grid.set(0, 0, Color::RED); // dot 0
    grid.set(1, 0, Color::RED); // dot 3
    grid.set(0, 1, Color::RED); // dot 1
    grid.set(1, 1, Color::RED); // dot 4
                                // Pattern should be 0x1B (dots 0, 1, 3, 4)
    assert_eq!(grid.cells()[0], 0x1B);
}

#[test]
fn test_braille_grid_render_smaller_area() {
    let mut grid = BrailleGrid::new(20, 20);
    for x in 0..40 {
        grid.set(x, 0, Color::WHITE);
    }
    let mut buffer = Buffer::new(20, 20);
    grid.render(&mut buffer, Rect::new(0, 0, 10, 10));
    // Cut off at the area's width
    assert_eq!(sym(&buffer, 9, 0), '\u{2809}');
    assert_eq!(sym(&buffer, 10, 0), ' ');
}

#[test]
fn test_braille_grid_composite_preserves_existing_dots() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(0, 0, Color::BLUE);
    let original_pattern = grid.cells()[0];

    let mut layer = Layer::new(10, 10);
    layer.set(5, 5, Color::RED); // Different cell
    grid.composite_layer(&layer);

    // Original pattern should be preserved
    assert_eq!(grid.cells()[0], original_pattern);
}
