use crate::style::Color;

/// Color scheme for chart series
#[derive(Clone, Debug)]
pub struct ColorScheme {
    /// Color palette
    pub palette: Vec<Color>,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::default_palette()
    }
}

impl ColorScheme {
    /// Create a color scheme with custom colors
    pub fn new(colors: Vec<Color>) -> Self {
        Self { palette: colors }
    }

    /// Default color palette (10 distinct colors)
    pub fn default_palette() -> Self {
        Self {
            palette: vec![
                Color::rgb(97, 175, 239),  // Blue
                Color::rgb(152, 195, 121), // Green
                Color::rgb(224, 108, 117), // Red
                Color::rgb(229, 192, 123), // Yellow
                Color::rgb(198, 120, 221), // Purple
                Color::rgb(86, 182, 194),  // Cyan
                Color::rgb(209, 154, 102), // Orange
                Color::rgb(190, 80, 70),   // Dark Red
                Color::rgb(152, 104, 1),   // Brown
                Color::rgb(171, 178, 191), // Gray
            ],
        }
    }

    /// Monochrome palette with shades of a base color
    pub fn monochrome(base: Color) -> Self {
        let (r, g, b) = (base.r, base.g, base.b);
        Self {
            palette: (1..=5)
                .map(|i| {
                    let factor = 0.5 + (i as f32 * 0.1);
                    Color::rgb(
                        (r as f32 * factor).min(255.0) as u8,
                        (g as f32 * factor).min(255.0) as u8,
                        (b as f32 * factor).min(255.0) as u8,
                    )
                })
                .collect(),
        }
    }

    /// Categorical palette (high contrast)
    pub fn categorical() -> Self {
        Self {
            palette: vec![
                Color::rgb(31, 119, 180),  // Blue
                Color::rgb(255, 127, 14),  // Orange
                Color::rgb(44, 160, 44),   // Green
                Color::rgb(214, 39, 40),   // Red
                Color::rgb(148, 103, 189), // Purple
                Color::rgb(140, 86, 75),   // Brown
                Color::rgb(227, 119, 194), // Pink
                Color::rgb(127, 127, 127), // Gray
                Color::rgb(188, 189, 34),  // Olive
                Color::rgb(23, 190, 207),  // Cyan
            ],
        }
    }

    /// Get color at index (cycles through palette)
    pub fn get(&self, index: usize) -> Color {
        if self.palette.is_empty() {
            Color::WHITE
        } else {
            self.palette[index % self.palette.len()]
        }
    }

    /// Number of colors in palette
    pub fn len(&self) -> usize {
        self.palette.len()
    }

    /// Check if palette is empty
    pub fn is_empty(&self) -> bool {
        self.palette.is_empty()
    }
}
