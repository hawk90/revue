//! Canvas widget tests

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::layout::Rect;
    use crate::prelude::RenderContext;
    use crate::render::Buffer;
    use crate::style::Color;
    use crate::widget::traits::View;

    // Tests that access private fields - KEEP HERE

    #[test]
    fn test_braille_grid_set() {
        let mut grid = BrailleGrid::new(10, 10);
        grid.set(0, 0, Color::RED);
        grid.set(1, 0, Color::RED);

        // Cell (0,0) should have dots at (0,0) and (1,0)
        assert_eq!(grid.cells()[0], 0x01 | 0x08);
    }

    #[test]
    fn test_braille_grid_get_char() {
        let mut grid = BrailleGrid::new(10, 10);

        // Set all 8 dots in the first cell
        for x in 0..2 {
            for y in 0..4 {
                grid.set(x, y, Color::WHITE);
            }
        }

        let ch = grid.get_char(0, 0);
        assert_eq!(ch, '⣿'); // Full braille character
    }
}