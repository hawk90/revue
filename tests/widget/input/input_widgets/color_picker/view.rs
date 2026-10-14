//! ColorPicker placement, and proptests of the color after any keys and of
//! hex input

use proptest::prelude::*;
use revue::event::Key;
use revue::style::Color;
use revue::widget::{ColorPalette, ColorPicker, ColorPickerMode};

use super::PALETTES;
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const MODES: [ColorPickerMode; 4] = [
    ColorPickerMode::Palette,
    ColorPickerMode::Rgb,
    ColorPickerMode::Hsl,
    ColorPickerMode::Hex,
];

#[test]
fn test_color_picker_zero_areas_draw_nothing() {
    for mode in MODES {
        assert_zero_areas_draw_nothing(&ColorPicker::new().mode(mode).border(Color::BLUE));
    }
}

#[test]
fn test_color_picker_same_when_offset() {
    for mode in MODES {
        let cp = ColorPicker::new()
            .mode(mode)
            .color(Color::rgb(10, 128, 250));
        assert_same_when_offset(cp.clone(), 24, 8);
        assert_same_when_offset(cp.border(Color::BLUE), 24, 8);
    }
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        Just(Key::Up),
        Just(Key::Down),
        Just(Key::Left),
        Just(Key::Right),
        Just(Key::Char('h')),
        Just(Key::Char('l')),
        Just(Key::Tab),
        Just(Key::Enter),
        Just(Key::Backspace),
        (0u32..16).prop_map(|d| Key::Char(char::from_digit(d, 16).unwrap())),
    ]
}

proptest! {
    /// After any keys in any mode and palette, the hex string is the color
    /// (the components and the color never disagree), and in the palette
    /// mode the color is one of the palette's.
    #[test]
    fn prop_color_picker_hex_matches_the_color(
        mode in 0usize..4,
        palette in 0usize..5,
        keys in prop::collection::vec(key(), 0..40),
    ) {
        let palette = PALETTES[palette];
        let mut cp = ColorPicker::new().palette(palette).mode(MODES[mode]);
        let mut in_palette_mode = mode == 0;
        for k in &keys {
            cp.handle_key(k);
            let c = cp.get_color();
            prop_assert_eq!(cp.hex_string(), format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b), "after {:?}", k);
            if *k == Key::Tab {
                in_palette_mode = false;
            }
            if in_palette_mode && c != Color::WHITE {
                prop_assert!(palette.colors().contains(&c));
            }
        }
    }

    /// Six hex digits typed in the hex mode set that color.
    #[test]
    fn prop_color_picker_hex_input_sets_the_color(r: u8, g: u8, b: u8, lower in any::<bool>()) {
        let mut cp = ColorPicker::new().mode(ColorPickerMode::Hex);
        let hex = format!("{:02X}{:02X}{:02X}", r, g, b);
        for c in hex.chars() {
            let typed = if lower { c.to_ascii_lowercase() } else { c };
            let used = cp.handle_key(&Key::Char(typed));
            prop_assert!(used);
        }
        prop_assert_eq!(cp.get_color(), Color::rgb(r, g, b));
        let with_hash = format!("#{}", hex);
        let ok = cp.set_hex(&with_hash);
        prop_assert!(ok);
        prop_assert_eq!(cp.hex_string(), with_hash);
    }
}

#[test]
fn test_color_picker_set_hex_rejects_non_hex() {
    let mut cp = ColorPicker::new().color(Color::rgb(1, 2, 3));
    for bad in ["", "12345", "1234567", "GGGGGG", "#éé0000", "12 456"] {
        assert!(!cp.set_hex(bad), "{bad:?}");
    }
    assert_eq!(cp.hex_string(), "#010203");
    // Pastel's first color is not white: the palette builder does not
    // change the color
    assert_eq!(
        ColorPicker::new().palette(ColorPalette::Pastel).get_color(),
        Color::WHITE
    );
}
