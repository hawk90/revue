//! Braille pattern constants

/// Braille dot pattern offsets
/// Each braille character is a 2x4 grid of dots:
/// ```text
/// (0,0) (1,0)    0x01  0x08
/// (0,1) (1,1)    0x02  0x10
/// (0,2) (1,2)    0x04  0x20
/// (0,3) (1,3)    0x40  0x80
/// ```
pub const BRAILLE_DOTS: [[u8; 4]; 2] = [
    [0x01, 0x02, 0x04, 0x40], // Left column (x=0)
    [0x08, 0x10, 0x20, 0x80], // Right column (x=1)
];

/// Base braille character (empty pattern)
pub const BRAILLE_BASE: u32 = 0x2800;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braille_dots_structure() {
        // Verify the BRAILLE_DOTS array has the correct structure
        assert_eq!(BRAILLE_DOTS.len(), 2);
        assert_eq!(BRAILLE_DOTS[0].len(), 4);
        assert_eq!(BRAILLE_DOTS[1].len(), 4);
    }

    #[test]
    fn test_braille_dots_unique_values() {
        // All dot values should be unique
        let all_dots: Vec<u8> = BRAILLE_DOTS
            .iter()
            .flat_map(|col| col.iter())
            .copied()
            .collect();

        let unique_dots: std::collections::HashSet<u8> = all_dots.iter().copied().collect();

        assert_eq!(all_dots.len(), unique_dots.len());
    }

    #[test]
    fn test_braille_dots_bit_positions() {
        // Verify each dot has a unique bit position
        assert_eq!(BRAILLE_DOTS[0][0], 0x01); // bit 0
        assert_eq!(BRAILLE_DOTS[0][1], 0x02); // bit 1
        assert_eq!(BRAILLE_DOTS[0][2], 0x04); // bit 2
        assert_eq!(BRAILLE_DOTS[0][3], 0x40); // bit 6
        assert_eq!(BRAILLE_DOTS[1][0], 0x08); // bit 3
        assert_eq!(BRAILLE_DOTS[1][1], 0x10); // bit 4
        assert_eq!(BRAILLE_DOTS[1][2], 0x20); // bit 5
        assert_eq!(BRAILLE_DOTS[1][3], 0x80); // bit 7
    }

    #[test]
    fn test_braille_base_value() {
        // Verify the base Unicode code point for braille patterns
        assert_eq!(BRAILLE_BASE, 0x2800);
    }

    #[test]
    fn test_braille_dots_all_combinations() {
        // Verify we can combine all dots
        let all_dots: u8 = BRAILLE_DOTS
            .iter()
            .flat_map(|col| col.iter())
            .fold(0u8, |acc, &dot| acc | dot);

        assert_eq!(all_dots, 0xFF);
    }

    #[test]
    fn test_braille_dots_column_order() {
        // Left column should be first, right column second
        assert_eq!(BRAILLE_DOTS[0][0], 0x01); // left column
        assert_eq!(BRAILLE_DOTS[1][0], 0x08); // right column
    }

    #[test]
    fn test_braille_dots_row_order() {
        // Rows should be ordered top to bottom
        assert_eq!(BRAILLE_DOTS[0][0], 0x01); // row 0
        assert_eq!(BRAILLE_DOTS[0][1], 0x02); // row 1
        assert_eq!(BRAILLE_DOTS[0][2], 0x04); // row 2
        assert_eq!(BRAILLE_DOTS[0][3], 0x40); // row 3
    }
}
