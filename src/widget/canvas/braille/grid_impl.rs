//! Braille grid implementation

use super::super::layer::Layer;
use crate::layout::Rect;
use crate::render::{Buffer, Cell};
use crate::style::Color;

use super::constants::{BRAILLE_BASE, BRAILLE_DOTS};

/// A high-resolution grid using braille patterns
///
/// Each terminal cell represents a 2x4 dot matrix, giving 8x the resolution
/// in the vertical direction and 2x in the horizontal direction.
pub struct BrailleGrid {
    /// Width in braille dots (2x terminal width)
    width: usize,
    /// Height in braille dots (4x terminal height)
    height: usize,
    /// Dot patterns for each cell
    cells: Vec<u8>,
    /// Colors for each cell
    colors: Vec<Option<Color>>,
    /// Terminal width
    term_width: usize,
    /// Terminal height
    term_height: usize,
}

impl BrailleGrid {
    /// Create a new braille grid for the given terminal dimensions
    pub fn new(term_width: u16, term_height: u16) -> Self {
        let tw = term_width as usize;
        let th = term_height as usize;
        let cell_count = tw * th;

        Self {
            width: tw * 2,
            height: th * 4,
            cells: vec![0; cell_count],
            colors: vec![None; cell_count],
            term_width: tw,
            term_height: th,
        }
    }

    /// Get the width in braille dots
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height in braille dots
    pub fn height(&self) -> usize {
        self.height
    }

    /// Set a dot at the given braille coordinates
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        let cell_x = x / 2;
        let cell_y = y / 4;
        let dot_x = x % 2;
        let dot_y = y % 4;

        let cell_idx = cell_y * self.term_width + cell_x;
        if cell_idx < self.cells.len() {
            self.cells[cell_idx] |= BRAILLE_DOTS[dot_x][dot_y];
            self.colors[cell_idx] = Some(color);
        }
    }

    /// Clear the grid
    pub fn clear(&mut self) {
        self.cells.fill(0);
        self.colors.fill(None);
    }

    /// Get the braille character for a cell
    pub fn get_char(&self, cell_x: usize, cell_y: usize) -> char {
        let idx = cell_y * self.term_width + cell_x;
        if idx < self.cells.len() {
            char::from_u32(BRAILLE_BASE + self.cells[idx] as u32).unwrap_or('⠀')
        } else {
            '⠀'
        }
    }

    /// Get the color for a cell
    fn get_color(&self, cell_x: usize, cell_y: usize) -> Option<Color> {
        let idx = cell_y * self.term_width + cell_x;
        if idx < self.colors.len() {
            self.colors[idx]
        } else {
            None
        }
    }

    /// Render the grid to the buffer
    pub fn render(&self, buffer: &mut Buffer, area: Rect) {
        for cy in 0..self.term_height.min(area.height as usize) {
            for cx in 0..self.term_width.min(area.width as usize) {
                let ch = self.get_char(cx, cy);
                if ch != '⠀' {
                    let mut cell = Cell::new(ch);
                    cell.fg = self.get_color(cx, cy);
                    buffer.set(area.x + cx as u16, area.y + cy as u16, cell);
                }
            }
        }
    }

    /// Composite a layer onto this grid
    ///
    /// The layer's dots are OR'd with existing dots, and colors are overwritten.
    pub fn composite_layer(&mut self, layer: &Layer) {
        if !layer.is_visible() || layer.opacity() <= 0.0 {
            return;
        }

        let layer_grid = layer.grid();
        let max_cells = self.cells.len().min(layer_grid.cells.len());

        for idx in 0..max_cells {
            let pattern = layer_grid.cells[idx];
            if pattern != 0 {
                self.cells[idx] |= pattern;
                if let Some(color) = layer_grid.colors[idx] {
                    self.colors[idx] = Some(color);
                }
            }
        }
    }

    /// Get cells for testing
    #[cfg(test)]
    pub fn cells(&self) -> &[u8] {
        &self.cells
    }

    /// Get colors for testing
    #[cfg(test)]
    pub fn colors(&self) -> &[Option<Color>] {
        &self.colors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // BrailleGrid::new tests
    // =========================================================================

    #[test]
    fn test_braille_grid_new() {
        let grid = BrailleGrid::new(40, 20);
        assert_eq!(grid.width(), 80); // 40 * 2
        assert_eq!(grid.height(), 80); // 20 * 4
        assert_eq!(grid.cells().len(), 40 * 20);
        assert_eq!(grid.colors().len(), 40 * 20);
    }

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

    #[test]
    fn test_braille_grid_width() {
        let grid = BrailleGrid::new(60, 30);
        assert_eq!(grid.width(), 120); // 60 * 2
    }

    #[test]
    fn test_braille_grid_height() {
        let grid = BrailleGrid::new(60, 30);
        assert_eq!(grid.height(), 120); // 30 * 4
    }

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
        // Cell (0,0) should have dots OR'd together
        assert_ne!(grid.cells()[0], 0);
        assert_eq!(grid.colors()[0], Some(Color::GREEN)); // Last color wins
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
                                     // Should not panic
        assert_ne!(grid.cells()[0], 0);
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
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        grid.render(&mut buffer, area);
        // Should not panic
    }

    #[test]
    fn test_braille_grid_render_empty() {
        let grid = BrailleGrid::new(10, 10);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        grid.render(&mut buffer, area);
        // Should not panic
    }

    #[test]
    fn test_braille_grid_render_partial_area() {
        let mut grid = BrailleGrid::new(20, 20);
        grid.set(0, 0, Color::RED);
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(5, 5, 10, 10);
        grid.render(&mut buffer, area);
        // Should not panic
    }

    #[test]
    fn test_braille_grid_render_with_color() {
        let mut grid = BrailleGrid::new(10, 10);
        grid.set(0, 0, Color::RED);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        grid.render(&mut buffer, area);
        // Should not panic
    }

    // =========================================================================
    // composite_layer tests
    // =========================================================================

    #[test]
    fn test_braille_grid_composite_layer() {
        let mut grid = BrailleGrid::new(10, 10);
        let mut layer = Layer::new(10, 10);
        layer.set(0, 0, Color::RED);
        grid.composite_layer(&layer);
        // Grid should have the layer's dots
        assert_ne!(grid.cells()[0], 0);
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
        // Grid should have the layer's dots (opacity doesn't affect pattern)
        assert_ne!(grid.cells()[0], 0);
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
        let mut grid = BrailleGrid::new(20, 20);
        let mut layer = Layer::new(10, 10);
        layer.set(0, 0, Color::RED);
        grid.composite_layer(&layer);
        // Should only composite up to the layer size
        assert_ne!(grid.cells()[0], 0);
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
        for x in 0..10 {
            for y in 0..10 {
                grid.set(x, y, Color::WHITE);
            }
        }
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 10, 10);
        grid.render(&mut buffer, area);
        // Should not panic
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
}
