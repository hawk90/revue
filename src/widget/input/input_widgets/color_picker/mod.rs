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

    use crate::event::Key;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::Color;
    use crate::widget::traits::View;

    // =========================================================================
    // Constructor tests
    // =========================================================================

    #[test]
    fn test_color_picker_new() {
        let cp = ColorPicker::new();
        assert_eq!(cp.get_color(), Color::WHITE);
    }

    #[test]
    fn test_color_picker_default() {
        let cp = ColorPicker::default();
        assert_eq!(cp.get_color(), Color::WHITE);
    }

    // =========================================================================
    // Builder method tests
    // =========================================================================

    #[test]
    fn test_color_picker_color() {
        let cp = ColorPicker::new().color(Color::RED);
        assert_eq!(cp.get_color(), Color::RED);
    }

    #[test]
    fn test_color_picker_mode() {
        let cp = ColorPicker::new().mode(ColorPickerMode::Rgb);
        assert_eq!(cp.mode, ColorPickerMode::Rgb);
    }

    #[test]
    fn test_color_picker_mode_all_variants() {
        let cp1 = ColorPicker::new().mode(ColorPickerMode::Palette);
        assert_eq!(cp1.mode, ColorPickerMode::Palette);

        let cp2 = ColorPicker::new().mode(ColorPickerMode::Rgb);
        assert_eq!(cp2.mode, ColorPickerMode::Rgb);

        let cp3 = ColorPicker::new().mode(ColorPickerMode::Hsl);
        assert_eq!(cp3.mode, ColorPickerMode::Hsl);

        let cp4 = ColorPicker::new().mode(ColorPickerMode::Hex);
        assert_eq!(cp4.mode, ColorPickerMode::Hex);
    }

    #[test]
    fn test_color_picker_palette() {
        let cp = ColorPicker::new().palette(ColorPalette::Material);
        assert_eq!(cp.palette, ColorPalette::Material);
    }

    #[test]
    fn test_color_picker_palette_all_variants() {
        let cp1 = ColorPicker::new().palette(ColorPalette::Basic);
        assert_eq!(cp1.palette, ColorPalette::Basic);

        let cp2 = ColorPicker::new().palette(ColorPalette::Extended);
        assert_eq!(cp2.palette, ColorPalette::Extended);

        let cp3 = ColorPicker::new().palette(ColorPalette::WebSafe);
        assert_eq!(cp3.palette, ColorPalette::WebSafe);

        let cp4 = ColorPicker::new().palette(ColorPalette::Material);
        assert_eq!(cp4.palette, ColorPalette::Material);

        let cp5 = ColorPicker::new().palette(ColorPalette::Pastel);
        assert_eq!(cp5.palette, ColorPalette::Pastel);
    }

    #[test]
    fn test_color_picker_preview() {
        let cp = ColorPicker::new().preview(false);
        assert!(!cp.show_preview);

        let cp2 = ColorPicker::new().preview(true);
        assert!(cp2.show_preview);
    }

    #[test]
    fn test_color_picker_hex() {
        let cp = ColorPicker::new().hex(false);
        assert!(!cp.show_hex);

        let cp2 = ColorPicker::new().hex(true);
        assert!(cp2.show_hex);
    }

    #[test]
    fn test_color_picker_border() {
        let cp = ColorPicker::new().border(Color::BLUE);
        assert_eq!(cp.border_color, Some(Color::BLUE));
    }

    #[test]
    fn test_color_picker_size() {
        let cp = ColorPicker::new().size(50, 20);
        assert_eq!(cp.width, 50);
        assert_eq!(cp.height, 20);
    }

    #[test]
    fn test_color_picker_size_minimum() {
        let cp = ColorPicker::new().size(10, 2);
        assert_eq!(cp.width, 20); // min is 20
        assert_eq!(cp.height, 6); // min is 6
    }

    // =========================================================================
    // Getter method tests
    // =========================================================================

    #[test]
    fn test_color_picker_get_color() {
        let cp = ColorPicker::new().color(Color::GREEN);
        assert_eq!(cp.get_color(), Color::GREEN);
    }

    #[test]
    fn test_color_picker_get_color_default() {
        let cp = ColorPicker::new();
        assert_eq!(cp.get_color(), Color::WHITE);
    }

    #[test]
    fn test_color_picker_hex_string_white() {
        let cp = ColorPicker::new();
        assert_eq!(cp.hex_string(), "#FFFFFF");
    }

    #[test]
    fn test_color_picker_hex_string_red() {
        let cp = ColorPicker::new().color(Color::RED);
        assert_eq!(cp.hex_string(), "#FF0000");
    }

    #[test]
    fn test_color_picker_hex_string_custom() {
        let cp = ColorPicker::new().color(Color::rgb(255, 128, 64));
        assert_eq!(cp.hex_string(), "#FF8040");
    }

    // =========================================================================
    // Setter method tests
    // =========================================================================

    #[test]
    fn test_color_picker_set_color_red() {
        let mut cp = ColorPicker::new();
        cp.set_color(Color::RED);
        assert_eq!(cp.get_color(), Color::RED);
        assert_eq!(cp.r, 255);
        assert_eq!(cp.g, 0);
        assert_eq!(cp.b, 0);
    }

    #[test]
    fn test_color_picker_set_color_blue() {
        let mut cp = ColorPicker::new();
        cp.set_color(Color::BLUE);
        assert_eq!(cp.get_color(), Color::BLUE);
        assert_eq!(cp.r, 0);
        assert_eq!(cp.g, 0);
        assert_eq!(cp.b, 255);
    }

    #[test]
    fn test_color_picker_set_hex_valid() {
        let mut cp = ColorPicker::new();
        let result = cp.set_hex("FF8000");
        assert!(result);
        assert_eq!(cp.hex_string(), "#FF8000");
    }

    #[test]
    fn test_color_picker_set_hex_with_hash() {
        let mut cp = ColorPicker::new();
        let result = cp.set_hex("#FF8000");
        assert!(result);
        assert_eq!(cp.hex_string(), "#FF8000");
    }

    #[test]
    fn test_color_picker_set_hex_invalid_length() {
        let mut cp = ColorPicker::new();
        let result = cp.set_hex("FFF");
        assert!(!result);
    }

    #[test]
    fn test_color_picker_set_hex_invalid_chars() {
        let mut cp = ColorPicker::new();
        let result = cp.set_hex("GGGGGG");
        assert!(!result);
    }

    #[test]
    fn test_color_picker_set_hex_empty() {
        let mut cp = ColorPicker::new();
        let result = cp.set_hex("");
        assert!(!result);
    }

    // =========================================================================
    // State-changing method tests
    // =========================================================================

    #[test]
    fn test_color_picker_next_mode() {
        let mut cp = ColorPicker::new();
        assert_eq!(cp.mode, ColorPickerMode::Palette);

        cp.next_mode();
        assert_eq!(cp.mode, ColorPickerMode::Rgb);

        cp.next_mode();
        assert_eq!(cp.mode, ColorPickerMode::Hsl);

        cp.next_mode();
        assert_eq!(cp.mode, ColorPickerMode::Hex);

        cp.next_mode();
        assert_eq!(cp.mode, ColorPickerMode::Palette);
    }

    #[test]
    fn test_color_picker_next_mode_from_rgb() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Rgb);
        cp.next_mode();
        assert_eq!(cp.mode, ColorPickerMode::Hsl);
    }

    #[test]
    fn test_color_picker_next_mode_from_hsl() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Hsl);
        cp.next_mode();
        assert_eq!(cp.mode, ColorPickerMode::Hex);
    }

    #[test]
    fn test_color_picker_next_mode_from_hex() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Hex);
        cp.next_mode();
        assert_eq!(cp.mode, ColorPickerMode::Palette);
    }

    // =========================================================================
    // Key handling tests
    // =========================================================================

    #[test]
    fn test_color_picker_handle_key_palette_left() {
        let mut cp = ColorPicker::new().palette(ColorPalette::Basic);
        cp.palette_index = 5;
        let handled = cp.handle_key(&Key::Left);
        assert!(handled);
        assert_eq!(cp.palette_index, 4);
    }

    #[test]
    fn test_color_picker_handle_key_palette_left_at_start() {
        let mut cp = ColorPicker::new().palette(ColorPalette::Basic);
        cp.palette_index = 0;
        let handled = cp.handle_key(&Key::Left);
        assert!(handled);
        assert_eq!(cp.palette_index, 0); // Stays at 0
    }

    #[test]
    fn test_color_picker_handle_key_palette_right() {
        let mut cp = ColorPicker::new().palette(ColorPalette::Basic);
        cp.palette_index = 5;
        let handled = cp.handle_key(&Key::Right);
        assert!(handled);
        assert_eq!(cp.palette_index, 6);
    }

    #[test]
    fn test_color_picker_handle_key_palette_right_at_end() {
        let mut cp = ColorPicker::new().palette(ColorPalette::Basic);
        cp.palette_index = 15;
        let handled = cp.handle_key(&Key::Right);
        assert!(handled);
        assert_eq!(cp.palette_index, 15); // Stays at end
    }

    #[test]
    fn test_color_picker_handle_key_palette_up() {
        let mut cp = ColorPicker::new().palette(ColorPalette::Basic);
        cp.palette_index = 10;
        let handled = cp.handle_key(&Key::Up);
        assert!(handled);
        assert_eq!(cp.palette_index, 2); // 10 - 8 = 2
    }

    #[test]
    fn test_color_picker_handle_key_palette_down() {
        let mut cp = ColorPicker::new().palette(ColorPalette::Basic);
        cp.palette_index = 2;
        let handled = cp.handle_key(&Key::Down);
        assert!(handled);
        assert_eq!(cp.palette_index, 10); // 2 + 8 = 10
    }

    #[test]
    fn test_color_picker_handle_key_palette_tab() {
        let mut cp = ColorPicker::new();
        let handled = cp.handle_key(&Key::Tab);
        assert!(handled);
        assert_eq!(cp.mode, ColorPickerMode::Rgb);
    }

    #[test]
    fn test_color_picker_handle_key_palette_vim_h() {
        let mut cp = ColorPicker::new().palette(ColorPalette::Basic);
        cp.palette_index = 5;
        let handled = cp.handle_key(&Key::Char('h'));
        assert!(handled);
        assert_eq!(cp.palette_index, 4);
    }

    #[test]
    fn test_color_picker_handle_key_palette_vim_l() {
        let mut cp = ColorPicker::new().palette(ColorPalette::Basic);
        cp.palette_index = 5;
        let handled = cp.handle_key(&Key::Char('l'));
        assert!(handled);
        assert_eq!(cp.palette_index, 6);
    }

    #[test]
    fn test_color_picker_handle_key_palette_vim_k() {
        let mut cp = ColorPicker::new().palette(ColorPalette::Basic);
        cp.palette_index = 10;
        let handled = cp.handle_key(&Key::Char('k'));
        assert!(handled);
        assert_eq!(cp.palette_index, 2);
    }

    #[test]
    fn test_color_picker_handle_key_palette_vim_j() {
        let mut cp = ColorPicker::new().palette(ColorPalette::Basic);
        cp.palette_index = 2;
        let handled = cp.handle_key(&Key::Char('j'));
        assert!(handled);
        assert_eq!(cp.palette_index, 10);
    }

    #[test]
    fn test_color_picker_handle_key_rgb_up() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Rgb);
        cp.active_slider = 2;
        let handled = cp.handle_key(&Key::Up);
        assert!(handled);
        assert_eq!(cp.active_slider, 1);
    }

    #[test]
    fn test_color_picker_handle_key_rgb_up_at_top() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Rgb);
        cp.active_slider = 0;
        let handled = cp.handle_key(&Key::Up);
        assert!(handled);
        assert_eq!(cp.active_slider, 0); // Stays at 0
    }

    #[test]
    fn test_color_picker_handle_key_rgb_down() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Rgb);
        cp.active_slider = 0;
        let handled = cp.handle_key(&Key::Down);
        assert!(handled);
        assert_eq!(cp.active_slider, 1);
    }

    #[test]
    fn test_color_picker_handle_key_rgb_down_at_bottom() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Rgb);
        cp.active_slider = 2;
        let handled = cp.handle_key(&Key::Down);
        assert!(handled);
        assert_eq!(cp.active_slider, 2); // Stays at 2
    }

    #[test]
    fn test_color_picker_handle_key_rgb_left() {
        let mut cp = ColorPicker::new()
            .mode(ColorPickerMode::Rgb)
            .color(Color::rgb(100, 100, 100));
        cp.active_slider = 0;
        let handled = cp.handle_key(&Key::Left);
        assert!(handled);
        assert!(cp.r < 100); // Decreased
    }

    #[test]
    fn test_color_picker_handle_key_rgb_right() {
        let mut cp = ColorPicker::new()
            .mode(ColorPickerMode::Rgb)
            .color(Color::rgb(100, 100, 100));
        cp.active_slider = 1;
        let handled = cp.handle_key(&Key::Right);
        assert!(handled);
        assert!(cp.g > 100); // Increased
    }

    #[test]
    fn test_color_picker_handle_key_rgb_tab() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Rgb);
        let handled = cp.handle_key(&Key::Tab);
        assert!(handled);
        assert_eq!(cp.mode, ColorPickerMode::Hsl);
    }

    #[test]
    fn test_color_picker_handle_key_hex_char() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Hex);
        let handled = cp.handle_key(&Key::Char('F'));
        assert!(handled);
        assert_eq!(cp.hex_input, "F");
    }

    #[test]
    fn test_color_picker_handle_key_hex_char_lowercase() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Hex);
        let handled = cp.handle_key(&Key::Char('a'));
        assert!(handled);
        assert_eq!(cp.hex_input, "A"); // Converted to uppercase
    }

    #[test]
    fn test_color_picker_handle_key_hex_backspace() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Hex);
        cp.hex_input = String::from("ABC");
        let handled = cp.handle_key(&Key::Backspace);
        assert!(handled);
        assert_eq!(cp.hex_input, "AB");
    }

    #[test]
    fn test_color_picker_handle_key_hex_enter_incomplete() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Hex);
        cp.hex_input = String::from("ABC");
        let handled = cp.handle_key(&Key::Enter);
        assert!(handled);
        // Should not change color
    }

    #[test]
    fn test_color_picker_handle_key_hex_enter_complete() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Hex);
        cp.hex_input = String::from("FF0000");
        let handled = cp.handle_key(&Key::Enter);
        assert!(handled);
        assert_eq!(cp.get_color(), Color::RED);
    }

    #[test]
    fn test_color_picker_handle_key_hex_tab() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Hex);
        let handled = cp.handle_key(&Key::Tab);
        assert!(handled);
        assert_eq!(cp.mode, ColorPickerMode::Palette);
    }

    #[test]
    fn test_color_picker_handle_key_unhandled() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Palette);
        let handled = cp.handle_key(&Key::Escape);
        assert!(!handled);
    }

    #[test]
    fn test_color_picker_handle_key_hex_auto_complete() {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Hex);
        cp.handle_key(&Key::Char('F'));
        cp.handle_key(&Key::Char('F'));
        cp.handle_key(&Key::Char('0'));
        cp.handle_key(&Key::Char('0'));
        cp.handle_key(&Key::Char('0'));
        cp.handle_key(&Key::Char('0'));
        assert_eq!(cp.get_color(), Color::RED);
    }

    // =========================================================================
    // CSS integration tests
    // =========================================================================

    #[test]
    fn test_color_picker_css_id() {
        let cp = ColorPicker::new().element_id("my-picker");
        assert_eq!(View::id(&cp), Some("my-picker"));

        let meta = cp.meta();
        assert_eq!(meta.id, Some("my-picker".to_string()));
    }

    #[test]
    fn test_color_picker_css_classes() {
        let cp = ColorPicker::new()
            .class("form-control")
            .class("color-input");

        assert!(cp.props.classes.contains(&"form-control".to_string()));
        assert!(cp.props.classes.contains(&"color-input".to_string()));
        assert!(!cp.props.classes.contains(&"optional".to_string()));

        let meta = cp.meta();
        assert!(meta.classes.contains(&"form-control".to_string()));
        assert!(meta.classes.contains(&"color-input".to_string()));
    }

    #[test]
    fn test_color_picker_styled_view() {
        let cp = ColorPicker::new()
            .element_id("test-picker")
            .class("focused");

        assert_eq!(View::id(&cp), Some("test-picker"));
        assert!(cp.props.classes.contains(&"focused".to_string()));

        let cp2 = cp.class("error");
        assert!(cp2.props.classes.contains(&"error".to_string()));

        let meta = cp2.meta();
        assert!(meta.classes.contains(&"error".to_string()));
    }

    // =========================================================================
    // Render tests
    // =========================================================================

    #[test]
    fn test_render() {
        let mut buffer = Buffer::new(40, 12);
        let area = Rect::new(0, 0, 40, 12);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        let cp = ColorPicker::new();
        View::render(&cp, &mut ctx);
        // Smoke test - should not panic
    }

    #[test]
    fn test_render_with_border() {
        let mut buffer = Buffer::new(40, 12);
        let area = Rect::new(0, 0, 40, 12);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        let cp = ColorPicker::new().border(Color::WHITE);
        View::render(&cp, &mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    }

    // =========================================================================
    // Utility function tests (keep existing)
    // =========================================================================

    #[test]
    fn test_rgb_to_hsl() {
        let (h, s, l) = crate::utils::color::rgb_to_hsl(Color::RED);
        assert_eq!(h, 0);
        assert!(s > 90);
        assert!(l > 40 && l < 60);

        let (h, _s, _l) = crate::utils::color::rgb_to_hsl(Color::GREEN);
        assert!(h > 110 && h < 130);
    }

    #[test]
    fn test_hsl_to_rgb() {
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
        let basic = ColorPalette::Basic.colors();
        assert_eq!(basic.len(), 16);

        let material = ColorPalette::Material.colors();
        assert_eq!(material.len(), 30);
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_helper() {
        let cp = color_picker().palette(ColorPalette::Material);
        assert_eq!(cp.palette, ColorPalette::Material);
    }

    #[test]
    fn test_helper_chain() {
        let cp = color_picker()
            .palette(ColorPalette::Pastel)
            .color(Color::BLUE)
            .size(60, 15);
        assert_eq!(cp.palette, ColorPalette::Pastel);
        assert_eq!(cp.get_color(), Color::BLUE);
        assert_eq!(cp.width, 60);
        assert_eq!(cp.height, 15);
    }
}
