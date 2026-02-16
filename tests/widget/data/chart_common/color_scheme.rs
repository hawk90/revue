//! Tests for chart common color scheme public APIs

use revue::style::Color;
use revue::widget::data::chart::chart_common::ColorScheme;

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_color_scheme_clone() {
        let scheme1 = ColorScheme::categorical();
        let scheme2 = scheme1.clone();
        assert_eq!(scheme1.len(), scheme2.len());
        assert_eq!(scheme1.get(0), scheme2.get(0));
    }
}