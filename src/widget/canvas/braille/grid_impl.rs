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
