//! Color scheme public API tests
mod tests {
    use revue::widget::data::chart::ColorScheme;
    use revue::style::Color;

    #[test]
    fn test_default_palette_colors() {
        let scheme = ColorScheme::default_palette();
        assert_eq!(scheme.len(), 10);
        assert!(!scheme.is_empty());

        // Test cycling through palette
        let color0 = scheme.get(0);
        let color10 = scheme.get(10);
        assert_eq!(color0.r, color10.r);
        assert_eq!(color0.g, color10.g);
        assert_eq!(color0.b, color10.b);
    }

    #[test]
    fn test_categorical_palette_high_contrast() {
        let scheme = ColorScheme::categorical();
        assert_eq!(scheme.len(), 10);

        // Check adjacent colors are different
        let c1 = scheme.get(0);
        let c2 = scheme.get(1);
        // Colors should be different
        assert_ne!(c1.r, c2.r);
    }

    #[test]
    fn test_monochrome_shades_progressive() {
        let scheme = ColorScheme::monochrome(Color::rgb(100, 100, 100));
        assert_eq!(scheme.len(), 5);

        // Each color should be progressively lighter
        let c1 = scheme.get(0);
        let c2 = scheme.get(1);
        // c2 should be lighter than c1
        assert!(c2.r >= c1.r);
    }

    #[test]
    fn test_color_scheme_custom() {
        let scheme = ColorScheme::new(vec![Color::RED, Color::GREEN, Color::BLUE]);
        assert_eq!(scheme.len(), 3);
        assert_eq!(scheme.get(0).r, 255); // RED
        assert_eq!(scheme.get(1).g, 255); // GREEN
        assert_eq!(scheme.get(2).b, 255); // BLUE
    }

    #[test]
    fn test_color_scheme_empty() {
        let scheme = ColorScheme::new(vec![]);
        assert!(scheme.is_empty());
        assert_eq!(scheme.len(), 0);
        // Empty scheme returns white
        let color = scheme.get(0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);
    }

    #[test]
    fn test_color_scheme_index_beyond_length() {
        let scheme = ColorScheme::new(vec![
            Color::rgb(10, 20, 30),
            Color::rgb(40, 50, 60),
        ]);
        // Should cycle back to 0
        let color2 = scheme.get(2);
        assert_eq!(color2.r, 10);
        assert_eq!(color2.g, 20);
        assert_eq!(color2.b, 30);
    }
}