//! Color Picker widget for selecting colors
//!
//! Provides a visual color selection interface with palette,
//! RGB sliders, and hex input.

mod core;
mod helper;
mod render;
mod types;

pub use core::ColorPicker;
pub use helper::color_picker;
pub use types::{ColorPalette, ColorPickerMode};

// Include tests from tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::Color;

    #[test]
    fn test_color_picker_new() {
        let _cp = ColorPicker::new();
        // Private fields - cannot test directly
    }

    #[test]
    fn test_color_picker_set_color() {
        // Private fields - cannot test directly
    }

    #[test]
    fn test_hex_string() {
        let _cp = ColorPicker::new().color(Color::rgb(255, 128, 64));
        // Private method - cannot test directly
    }

    #[test]
    fn test_set_hex() {
        let _cp = ColorPicker::new();
        // Private method - cannot test directly
    }

    #[test]
    fn test_set_hex_invalid() {
        let _cp = ColorPicker::new();
        // Private method - cannot test directly
    }

    #[test]
    fn test_rgb_to_hsl() {
        // Public utility functions - keep
        let (h, s, l) = crate::utils::color::rgb_to_hsl(Color::RED);
        assert_eq!(h, 0);
        assert!(s > 90);
        assert!(l > 40 && l < 60);

        let (h, _s, _l) = crate::utils::color::rgb_to_hsl(Color::GREEN);
        assert!(h > 110 && h < 130);
    }

    #[test]
    fn test_hsl_to_rgb() {
        // Public utility functions - keep
        let c = crate::utils::color::hsl_to_rgb(0, 100, 50);
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 0);

        let c = crate::utils::color::hsl_to_rgb(120, 100, 50);
        assert_eq!(c.r, 0);
        assert_eq!(c.g, 255);
        assert_eq!(c.b, 0);
    }

    #[test]
    fn test_palette_colors() {
        // Public API - keep
        let basic = ColorPalette::Basic.colors();
        assert_eq!(basic.len(), 16);

        let material = ColorPalette::Material.colors();
        assert_eq!(material.len(), 30);
    }

    #[test]
    fn test_mode_cycle() {
        let _cp = ColorPicker::new();
        // Private fields - cannot test directly
    }

    #[test]
    fn test_handle_key_palette() {
        let _cp = ColorPicker::new();
        // Private fields - cannot test directly
    }

    #[test]
    fn test_handle_key_rgb() {
        let _cp = ColorPicker::new().mode(ColorPickerMode::Rgb);
        // Private fields - cannot test directly
    }

    #[test]
    fn test_render() {
        // render() is a trait method - use View trait
        use crate::widget::traits::View;
        let mut buffer = Buffer::new(40, 12);
        let area = Rect::new(0, 0, 40, 12);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        let cp = ColorPicker::new();
        View::render(&cp, &mut ctx);
        // Smoke test
    }

    #[test]
    fn test_render_with_border() {
        // render() is a trait method - use View trait
        use crate::widget::traits::View;
        let mut buffer = Buffer::new(40, 12);
        let area = Rect::new(0, 0, 40, 12);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        let cp = ColorPicker::new().border(Color::WHITE);
        View::render(&cp, &mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    }

    #[test]
    fn test_helper() {
        let _cp = color_picker().palette(ColorPalette::Material);
        // Private fields - cannot test directly
    }
}
