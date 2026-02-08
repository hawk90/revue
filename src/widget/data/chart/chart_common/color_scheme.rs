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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ColorScheme::new tests
    // =========================================================================

    #[test]
    fn test_color_scheme_new() {
        let colors = vec![Color::RED, Color::GREEN, Color::BLUE];
        let scheme = ColorScheme::new(colors);
        assert_eq!(scheme.len(), 3);
        assert!(!scheme.is_empty());
    }

    #[test]
    fn test_color_scheme_new_empty() {
        let scheme = ColorScheme::new(vec![]);
        assert!(scheme.is_empty());
        assert_eq!(scheme.len(), 0);
    }

    // =========================================================================
    // ColorScheme constructors
    // =========================================================================

    #[test]
    fn test_color_scheme_default_palette() {
        let scheme = ColorScheme::default_palette();
        assert_eq!(scheme.len(), 10);
        assert!(!scheme.is_empty());
    }

    #[test]
    fn test_color_scheme_default_trait() {
        let scheme = ColorScheme::default();
        assert_eq!(scheme.len(), 10);
    }

    #[test]
    fn test_color_scheme_monochrome() {
        let scheme = ColorScheme::monochrome(Color::rgb(100, 150, 200));
        assert_eq!(scheme.len(), 5);
        assert!(!scheme.is_empty());
    }

    #[test]
    fn test_color_scheme_monochrome_red() {
        let scheme = ColorScheme::monochrome(Color::RED);
        // All colors should be red based
        assert_eq!(scheme.len(), 5);
    }

    #[test]
    fn test_color_scheme_categorical() {
        let scheme = ColorScheme::categorical();
        assert_eq!(scheme.len(), 10);
        assert!(!scheme.is_empty());
    }

    // =========================================================================
    // get method tests
    // =========================================================================

    #[test]
    fn test_color_scheme_get_within_bounds() {
        let scheme = ColorScheme::new(vec![Color::RED, Color::GREEN, Color::BLUE]);
        assert_eq!(scheme.get(0), Color::RED);
        assert_eq!(scheme.get(1), Color::GREEN);
        assert_eq!(scheme.get(2), Color::BLUE);
    }

    #[test]
    fn test_color_scheme_get_cycles() {
        let scheme = ColorScheme::new(vec![Color::RED, Color::GREEN, Color::BLUE]);
        // Should cycle back to RED
        assert_eq!(scheme.get(3), Color::RED);
        assert_eq!(scheme.get(4), Color::GREEN);
        assert_eq!(scheme.get(5), Color::BLUE);
        assert_eq!(scheme.get(6), Color::RED);
    }

    #[test]
    fn test_color_scheme_get_empty() {
        let scheme = ColorScheme::new(vec![]);
        assert_eq!(scheme.get(0), Color::WHITE);
        assert_eq!(scheme.get(100), Color::WHITE);
    }

    // =========================================================================
    // len and is_empty tests
    // =========================================================================

    #[test]
    fn test_color_scheme_len() {
        let scheme = ColorScheme::new(vec![Color::RED; 7]);
        assert_eq!(scheme.len(), 7);
    }

    #[test]
    fn test_color_scheme_is_empty_true() {
        let scheme = ColorScheme::new(vec![]);
        assert!(scheme.is_empty());
    }

    #[test]
    fn test_color_scheme_is_empty_false() {
        let scheme = ColorScheme::new(vec![Color::RED]);
        assert!(!scheme.is_empty());
    }

    // =========================================================================
    // Clone tests
    // =========================================================================

    #[test]
    fn test_color_scheme_clone() {
        let scheme1 = ColorScheme::categorical();
        let scheme2 = scheme1.clone();
        assert_eq!(scheme1.len(), scheme2.len());
        assert_eq!(scheme1.get(0), scheme2.get(0));
    }

    // =========================================================================
    // Palette verification tests
    // =========================================================================

    #[test]
    fn test_default_palette_colors() {
        let scheme = ColorScheme::default_palette();
        // First color should be blue-ish
        let c = scheme.get(0);
        assert!(c.r >= 90 && c.r <= 105); // ~97
        assert!(c.g >= 170 && c.g <= 180); // ~175
        assert!(c.b >= 235 && c.b <= 245); // ~239
    }

    #[test]
    fn test_categorical_palette_high_contrast() {
        let scheme = ColorScheme::categorical();
        // Check adjacent colors are different
        let c1 = scheme.get(0);
        let c2 = scheme.get(1);
        // Colors should be different
        assert_ne!(c1.r, c2.r);
    }

    #[test]
    fn test_monochrome_shades_progressive() {
        let scheme = ColorScheme::monochrome(Color::rgb(100, 100, 100));
        // Each color should be progressively lighter
        let c1 = scheme.get(0);
        let c2 = scheme.get(1);
        // c2 should be lighter than c1
        assert!(c2.r >= c1.r);
    }
}
