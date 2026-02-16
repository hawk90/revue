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
