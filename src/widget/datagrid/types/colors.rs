//! Grid color schemes

use crate::style::Color;

/// Grid color scheme
#[derive(Clone, Debug)]
pub struct GridColors {
    /// Header background color
    pub header_bg: Color,
    /// Header foreground color
    pub header_fg: Color,
    /// Normal row background
    pub row_bg: Color,
    /// Alternate row background (zebra striping)
    pub alt_row_bg: Color,
    /// Selected row background
    pub selected_bg: Color,
    /// Selected row foreground
    pub selected_fg: Color,
    /// Border/separator color
    pub border_color: Color,
}

impl Default for GridColors {
    fn default() -> Self {
        Self {
            header_bg: Color::rgb(60, 60, 80),
            header_fg: Color::WHITE,
            row_bg: Color::rgb(30, 30, 30),
            alt_row_bg: Color::rgb(40, 40, 40),
            selected_bg: Color::rgb(60, 100, 180),
            selected_fg: Color::WHITE,
            border_color: Color::rgb(80, 80, 80),
        }
    }
}

impl GridColors {
    /// Create a new color scheme
    pub fn new() -> Self {
        Self::default()
    }

    /// Dark theme (default)
    pub fn dark() -> Self {
        Self::default()
    }

    /// Light theme
    pub fn light() -> Self {
        Self {
            header_bg: Color::rgb(220, 220, 230),
            header_fg: Color::BLACK,
            row_bg: Color::rgb(255, 255, 255),
            alt_row_bg: Color::rgb(245, 245, 250),
            selected_bg: Color::rgb(100, 150, 220),
            selected_fg: Color::WHITE,
            border_color: Color::rgb(180, 180, 190),
        }
    }
}
