//! Color picker widget tests

mod types;
mod view;

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{color_picker, ColorPalette, ColorPicker, ColorPickerMode, StyledView, View};

/// Each row of `cp` drawn in a `w` x `h` area
fn rows(cp: &ColorPicker, w: u16, h: u16) -> Vec<String> {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    cp.render(&mut ctx);
    (0..h)
        .map(|y| (0..w).map(|x| buffer.get(x, y).unwrap().symbol).collect())
        .collect()
}

/// The mode, as the first row shows it
fn shown_mode(cp: &ColorPicker) -> ColorPickerMode {
    let first = rows(cp, 40, 12).remove(0);
    if first.starts_with("Hex: #") {
        ColorPickerMode::Hex
    } else if first.starts_with("R ") {
        ColorPickerMode::Rgb
    } else if first.starts_with("H ") {
        ColorPickerMode::Hsl
    } else {
        ColorPickerMode::Palette
    }
}

pub const PALETTES: [ColorPalette; 5] = [
    ColorPalette::Basic,
    ColorPalette::Extended,
    ColorPalette::WebSafe,
    ColorPalette::Material,
    ColorPalette::Pastel,
];

// =========================================================================
// Constructor and builder tests
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

#[test]
fn test_color_picker_helper() {
    let cp = color_picker();
    assert_eq!(cp.get_color(), Color::WHITE);
}

#[test]
fn test_color_picker_color_builder() {
    let cp = ColorPicker::new().color(Color::RED);
    assert_eq!(cp.get_color(), Color::RED);

    let cp = ColorPicker::new().color(Color::rgb(128, 64, 192));
    assert_eq!(cp.get_color(), Color::rgb(128, 64, 192));
}

#[test]
fn test_color_picker_mode_builder() {
    for mode in [
        ColorPickerMode::Palette,
        ColorPickerMode::Rgb,
        ColorPickerMode::Hsl,
        ColorPickerMode::Hex,
    ] {
        assert_eq!(shown_mode(&ColorPicker::new().mode(mode)), mode);
    }
}

#[test]
fn test_color_picker_palette_builder() {
    // Right from the first swatch selects the palette's second color
    for palette in PALETTES {
        let mut cp = ColorPicker::new().palette(palette);
        cp.handle_key(&Key::Right);
        assert_eq!(cp.get_color(), palette.colors()[1], "{palette:?}");
    }
}

#[test]
fn test_color_picker_preview_builder() {
    // The preview: a 6-wide block in the color on the last two rows
    let cp = ColorPicker::new().color(Color::RED).preview(true);
    let shown = rows(&cp, 40, 12);
    assert!(shown[10].starts_with("██████ #FF0000"));
    assert!(shown[11].starts_with("██████ "));
    let hidden = rows(&ColorPicker::new().color(Color::RED).preview(false), 40, 12);
    assert!(hidden[10].trim().is_empty() && hidden[11].trim().is_empty());
}

#[test]
fn test_color_picker_hex_builder() {
    // The hex value beside the preview
    let cp = ColorPicker::new().color(Color::rgb(1, 2, 3));
    assert!(rows(&cp.clone().hex(true), 40, 12)[10].starts_with("██████ #010203"));
    assert_eq!(rows(&cp.hex(false), 40, 12)[10].trim_end(), "██████");
}

#[test]
fn test_color_picker_border_builder() {
    let cp = ColorPicker::new().border(Color::RED);
    let _cp = ColorPicker::new().border(Color::rgb(128, 128, 128));

    let mut buffer = Buffer::new(40, 12);
    let area = Rect::new(0, 0, 40, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cp.render(&mut ctx);

    // Verify border is present
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_color_picker_size_builder() {
    // Stored but not read: the area decides the size (#799)
    let default = rows(&ColorPicker::new(), 20, 6);
    for (w, h) in [(30, 8), (50, 15), (5, 2)] {
        assert_eq!(rows(&ColorPicker::new().size(w, h), 20, 6), default);
    }
}

#[test]
fn test_color_picker_chained_builders() {
    let cp = ColorPicker::new()
        .color(Color::BLUE)
        .mode(ColorPickerMode::Rgb)
        .palette(ColorPalette::Material)
        .preview(false)
        .hex(false)
        .border(Color::GREEN)
        .size(50, 15);

    assert_eq!(cp.get_color(), Color::BLUE);

    let mut buffer = Buffer::new(50, 15);
    let area = Rect::new(0, 0, 50, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cp.render(&mut ctx);
}

// =========================================================================
// Color management tests
// =========================================================================

#[test]
fn test_color_picker_set_color() {
    let mut cp = ColorPicker::new();
    cp.set_color(Color::RED);
    assert_eq!(cp.get_color(), Color::RED);

    cp.set_color(Color::BLUE);
    assert_eq!(cp.get_color(), Color::BLUE);

    cp.set_color(Color::rgb(128, 128, 128));
    assert_eq!(cp.get_color(), Color::rgb(128, 128, 128));
}

// =========================================================================
// Hex string tests
// =========================================================================

#[test]
fn test_color_picker_hex_string() {
    let cp = ColorPicker::new().color(Color::RED);
    assert_eq!(cp.hex_string(), "#FF0000");

    let cp = ColorPicker::new().color(Color::rgb(255, 128, 64));
    assert_eq!(cp.hex_string(), "#FF8040");

    let cp = ColorPicker::new().color(Color::BLACK);
    assert_eq!(cp.hex_string(), "#000000");

    let cp = ColorPicker::new().color(Color::WHITE);
    assert_eq!(cp.hex_string(), "#FFFFFF");

    let cp = ColorPicker::new().color(Color::rgb(0, 15, 255));
    assert_eq!(cp.hex_string(), "#000FFF");
}

#[test]
fn test_color_picker_set_hex_valid() {
    let mut cp = ColorPicker::new();

    // Valid hex codes
    assert!(cp.set_hex("FF0000"));
    assert_eq!(cp.get_color(), Color::RED);

    assert!(cp.set_hex("#00FF00"));
    assert_eq!(cp.get_color(), Color::GREEN);

    assert!(cp.set_hex("0000FF"));
    assert_eq!(cp.get_color(), Color::BLUE);

    assert!(cp.set_hex("#808080"));
    assert_eq!(cp.get_color(), Color::rgb(128, 128, 128));

    assert!(cp.set_hex("ABCDEF"));
    assert_eq!(cp.get_color(), Color::rgb(171, 205, 239));

    assert!(cp.set_hex("#123456"));
    assert_eq!(cp.get_color(), Color::rgb(18, 52, 86));
}

#[test]
fn test_color_picker_set_hex_invalid() {
    let mut cp = ColorPicker::new();
    let initial_color = cp.get_color();

    // Invalid hex codes should return false and not change color
    assert!(!cp.set_hex("invalid"));
    assert_eq!(cp.get_color(), initial_color);

    assert!(!cp.set_hex("12345")); // Too short
    assert!(!cp.set_hex("1234567")); // Too long
    assert!(!cp.set_hex("XXXXXX")); // Invalid characters
    assert!(!cp.set_hex("#ZZZZZZ")); // Invalid characters with hash
    assert!(!cp.set_hex("")); // Empty string
    assert!(!cp.set_hex("#")); // Just hash
}

#[test]
fn test_color_picker_set_hex_case_insensitive() {
    let mut cp = ColorPicker::new();

    // Test uppercase
    assert!(cp.set_hex("FF0000"));
    assert_eq!(cp.get_color(), Color::RED);

    // Test lowercase
    assert!(cp.set_hex("00ff00"));
    assert_eq!(cp.get_color(), Color::GREEN);

    // Test mixed case
    assert!(cp.set_hex("#0000Ff"));
    assert_eq!(cp.get_color(), Color::BLUE);
}

// =========================================================================
// Mode cycling tests
// =========================================================================

#[test]
fn test_color_picker_next_mode() {
    let mut cp = ColorPicker::new();
    for mode in [
        ColorPickerMode::Rgb,
        ColorPickerMode::Hsl,
        ColorPickerMode::Hex,
        ColorPickerMode::Palette,
    ] {
        cp.next_mode();
        assert_eq!(shown_mode(&cp), mode);
    }
}

#[test]
fn test_color_picker_mode_cycle_complete() {
    // Four steps come back; the color is kept through the cycle
    let mut cp = ColorPicker::new()
        .color(Color::rgb(10, 20, 30))
        .mode(ColorPickerMode::Hsl);
    for _ in 0..3 {
        for _ in 0..4 {
            cp.next_mode();
        }
        assert_eq!(shown_mode(&cp), ColorPickerMode::Hsl);
    }
    assert_eq!(cp.hex_string(), "#0A141E");
}

// =========================================================================
// Palette tests
// =========================================================================

// Note: ColorPalette's colors() and grid_size() methods are private,
// so we can't directly test them. We test palettes indirectly through
// the ColorPicker's rendering and behavior.

// =========================================================================
// Keyboard handling - Palette mode
// =========================================================================

#[test]
fn test_handle_key_palette_right() {
    let mut cp = ColorPicker::new();
    let initial_color = cp.get_color();

    // Start with first color (WHITE for Basic palette)
    cp.handle_key(&Key::Right);
    // Should have changed color
    assert_ne!(cp.get_color(), initial_color);

    let second_color = cp.get_color();
    cp.handle_key(&Key::Right);
    // Should have changed again
    assert_ne!(cp.get_color(), second_color);
}

#[test]
fn test_handle_key_palette_left() {
    let mut cp = ColorPicker::new();

    // Move right twice
    cp.handle_key(&Key::Right);
    cp.handle_key(&Key::Right);

    let color_at_2 = cp.get_color();

    // Move back
    cp.handle_key(&Key::Left);
    let color_after_first_left = cp.get_color();
    assert_ne!(color_after_first_left, color_at_2);

    cp.handle_key(&Key::Left);
    let color_after_second_left = cp.get_color();
    assert_ne!(color_after_second_left, color_after_first_left);
}

#[test]
fn test_handle_key_palette_left_at_start() {
    let mut cp = ColorPicker::new();
    let initial_color = cp.get_color();

    // Should stay at first color
    let handled = cp.handle_key(&Key::Left);
    assert!(handled);
    assert_eq!(cp.get_color(), initial_color);
}

#[test]
fn test_handle_key_palette_up() {
    let mut cp = ColorPicker::new();

    // Move to second row (index 8 for Basic palette with 8 columns)
    for _ in 0..8 {
        cp.handle_key(&Key::Right);
    }

    let color_in_second_row = cp.get_color();

    cp.handle_key(&Key::Up);
    // Should move up by column count (8)
    assert_ne!(cp.get_color(), color_in_second_row);
}

#[test]
fn test_handle_key_palette_down() {
    let mut cp = ColorPicker::new();

    // Move to second column
    cp.handle_key(&Key::Right);
    let color_in_first_row = cp.get_color();

    cp.handle_key(&Key::Down);
    // Should move down by column count (8)
    assert_ne!(cp.get_color(), color_in_first_row);
}

#[test]
fn test_handle_key_palette_vim_keys() {
    // h, j, k, l move like the arrows
    let colors = ColorPalette::Basic.colors();
    let mut cp = ColorPicker::new();
    assert!(cp.handle_key(&Key::Char('l')));
    assert_eq!(cp.get_color(), colors[1]);
    assert!(cp.handle_key(&Key::Char('j')));
    assert_eq!(cp.get_color(), colors[9]);
    assert!(cp.handle_key(&Key::Char('h')));
    assert_eq!(cp.get_color(), colors[8]);
    assert!(cp.handle_key(&Key::Char('k')));
    assert_eq!(cp.get_color(), colors[0]);
}

#[test]
fn test_handle_key_palette_tab() {
    let mut cp = ColorPicker::new();

    // Tab should cycle mode
    let handled = cp.handle_key(&Key::Tab);
    assert!(handled);

    // After cycling, navigation behavior should change
    // (RGB mode adjusts RGB values instead of palette selection)
}

#[test]
fn test_handle_key_palette_unknown() {
    let mut cp = ColorPicker::new();
    let initial_color = cp.get_color();

    // Unknown keys should be ignored
    let handled = cp.handle_key(&Key::Char('x'));
    assert!(!handled);
    assert_eq!(cp.get_color(), initial_color);

    let handled = cp.handle_key(&Key::Enter);
    assert!(!handled);
    assert_eq!(cp.get_color(), initial_color);
}

// =========================================================================
// Keyboard handling - RGB mode
// =========================================================================

#[test]
fn test_handle_key_rgb_left_right_changes_color() {
    let mut cp = ColorPicker::new().color(Color::rgb(100, 100, 100));
    cp.next_mode(); // Switch to RGB mode

    let initial_color = cp.get_color();

    // Left/right should modify color in RGB mode
    let handled = cp.handle_key(&Key::Right);
    assert!(handled);

    // Color should have changed
    assert_ne!(cp.get_color(), initial_color);
}

#[test]
fn test_handle_key_rgb_up_down_changes_slider() {
    // Down picks the next channel; Right steps it by 5
    let mut cp = ColorPicker::new()
        .color(Color::BLACK)
        .mode(ColorPickerMode::Rgb);
    cp.handle_key(&Key::Right);
    assert_eq!(cp.hex_string(), "#050000");
    cp.handle_key(&Key::Down);
    cp.handle_key(&Key::Right);
    assert_eq!(cp.hex_string(), "#050500");
    cp.handle_key(&Key::Down);
    cp.handle_key(&Key::Right);
    assert_eq!(cp.hex_string(), "#050505");
    // Past the last channel Down stays; Up goes back to the first
    cp.handle_key(&Key::Down);
    cp.handle_key(&Key::Right);
    assert_eq!(cp.hex_string(), "#05050A");
    for _ in 0..5 {
        cp.handle_key(&Key::Up);
    }
    cp.handle_key(&Key::Right);
    assert_eq!(cp.hex_string(), "#0A050A");
}

#[test]
fn test_handle_key_rgb_boundaries() {
    let mut cp = ColorPicker::new().color(Color::WHITE);
    cp.next_mode(); // Switch to RGB mode

    let _maxed_color = cp.get_color();

    // Try to increase beyond max
    for _ in 0..20 {
        cp.handle_key(&Key::Right);
    }
    // Decrease multiple times - RGB mode modifies one component at a time
    for _ in 0..100 {
        cp.handle_key(&Key::Left);
    }
    // Only the active component saturates at 0, others remain
    assert_eq!(cp.get_color().r, 0);
    // G and B may stay at 255 depending on which component was active
}

#[test]
fn test_handle_key_rgb_vim_keys() {
    let mut cp = ColorPicker::new().color(Color::rgb(100, 100, 100));
    cp.next_mode(); // Switch to RGB mode

    let _initial_color = cp.get_color();

    // Test vim keys
    let handled = cp.handle_key(&Key::Char('l'));
    assert!(handled);
    let _initial = cp.get_color();

    cp.handle_key(&Key::Char('h'));
    // Should move back toward original
}

#[test]
fn test_handle_key_rgb_tab() {
    let mut cp = ColorPicker::new();
    cp.next_mode(); // Switch to RGB mode

    let handled = cp.handle_key(&Key::Tab);
    assert!(handled);

    // Tab should cycle to HSL mode
}

// =========================================================================
// Keyboard handling - HSL mode
// =========================================================================

#[test]
fn test_handle_key_hsl_left_right_changes_color() {
    let mut cp = ColorPicker::new().color(Color::rgb(100, 100, 100));
    cp.next_mode();
    cp.next_mode(); // Switch to HSL mode

    let initial_color = cp.get_color();

    // Left/right should modify color in HSL mode
    let handled = cp.handle_key(&Key::Right);
    assert!(handled);

    // Color should have changed
    assert_ne!(cp.get_color(), initial_color);
}

#[test]
fn test_handle_key_hsl_boundaries() {
    let mut cp = ColorPicker::new()
        .color(Color::WHITE)
        .mode(ColorPickerMode::Hsl);
    // H: up to 360 and down to 0, shown as three digits
    for _ in 0..100 {
        cp.handle_key(&Key::Right);
    }
    assert!(rows(&cp, 40, 12)[0].ends_with("360"));
    for _ in 0..100 {
        cp.handle_key(&Key::Left);
    }
    assert!(rows(&cp, 40, 12)[0].ends_with("  0"));
    // L: down to 0 is black, up to 100 is white
    cp.handle_key(&Key::Down);
    cp.handle_key(&Key::Down);
    for _ in 0..30 {
        cp.handle_key(&Key::Left);
    }
    assert_eq!(cp.get_color(), Color::rgb(0, 0, 0));
    for _ in 0..30 {
        cp.handle_key(&Key::Right);
    }
    assert_eq!(cp.get_color(), Color::rgb(255, 255, 255));
}

#[test]
fn test_handle_key_hsl_tab() {
    let mut cp = ColorPicker::new();
    cp.next_mode();
    cp.next_mode(); // Switch to HSL mode

    let handled = cp.handle_key(&Key::Tab);
    assert!(handled);

    // Tab should cycle to Hex mode
}

// =========================================================================
// Keyboard handling - Hex mode
// =========================================================================

#[test]
fn test_handle_key_hex_input() {
    let mut cp = ColorPicker::new();
    cp.next_mode();
    cp.next_mode();
    cp.next_mode(); // Switch to Hex mode

    let _initial_color = cp.get_color();

    // Input hex digits
    let handled = cp.handle_key(&Key::Char('F'));
    assert!(handled);

    let handled = cp.handle_key(&Key::Char('F'));
    assert!(handled);

    let handled = cp.handle_key(&Key::Char('0'));
    assert!(handled);

    let handled = cp.handle_key(&Key::Char('0'));
    assert!(handled);

    let handled = cp.handle_key(&Key::Char('0'));
    assert!(handled);

    let handled = cp.handle_key(&Key::Char('0'));
    assert!(handled);

    // After 6 chars, should parse and set color
    assert_eq!(cp.get_color(), Color::RED);
}

#[test]
fn test_handle_key_hex_lowercase() {
    let mut cp = ColorPicker::new();
    cp.next_mode();
    cp.next_mode();
    cp.next_mode(); // Switch to Hex mode

    // Test lowercase input
    for c in "ff0000".chars() {
        cp.handle_key(&Key::Char(c));
    }

    // Should still parse correctly
    assert_eq!(cp.get_color(), Color::RED);
}

#[test]
fn test_handle_key_hex_backspace() {
    let mut cp = ColorPicker::new();
    cp.next_mode();
    cp.next_mode();
    cp.next_mode(); // Switch to Hex mode

    // Input some characters
    for c in "FF000".chars() {
        cp.handle_key(&Key::Char(c));
    }

    let color_before = cp.get_color();

    // Backspace should remove last character
    let handled = cp.handle_key(&Key::Backspace);
    assert!(handled);

    // Color should not have changed (incomplete hex)
    assert_eq!(cp.get_color(), color_before);
}

#[test]
fn test_handle_key_hex_enter() {
    let mut cp = ColorPicker::new();
    cp.next_mode();
    cp.next_mode();
    cp.next_mode(); // Switch to Hex mode

    // Input incomplete hex
    for c in "FF00".chars() {
        cp.handle_key(&Key::Char(c));
    }

    // Enter should try to parse (and fail due to incomplete input)
    let handled = cp.handle_key(&Key::Enter);
    assert!(handled);
}

#[test]
fn test_handle_key_hex_max_length() {
    let mut cp = ColorPicker::new();
    cp.next_mode();
    cp.next_mode();
    cp.next_mode(); // Switch to Hex mode

    // Add 6 characters
    for c in "FF0000".chars() {
        cp.handle_key(&Key::Char(c));
    }

    let color_six = cp.get_color();

    // Try to add more - should be ignored
    let handled = cp.handle_key(&Key::Char('0'));
    assert!(handled);

    // Color should remain the same
    assert_eq!(cp.get_color(), color_six);
}

#[test]
fn test_handle_key_hex_invalid_chars() {
    let mut cp = ColorPicker::new();
    cp.next_mode();
    cp.next_mode();
    cp.next_mode(); // Switch to Hex mode

    let initial_color = cp.get_color();

    // Non-hex characters should be ignored
    let handled = cp.handle_key(&Key::Char('G'));
    assert!(!handled);

    assert_eq!(cp.get_color(), initial_color);

    let handled = cp.handle_key(&Key::Char('Z'));
    assert!(!handled);

    assert_eq!(cp.get_color(), initial_color);
}

#[test]
fn test_handle_key_hex_valid_hex_chars() {
    let mut cp = ColorPicker::new();
    cp.next_mode();
    cp.next_mode();
    cp.next_mode(); // Switch to Hex mode

    // Test all valid hex characters
    for c in "0123456789ABCDEFabcdef".chars() {
        let handled = cp.handle_key(&Key::Char(c));
        assert!(handled);
    }
    // Should process without panicking
}

#[test]
fn test_handle_key_hex_tab() {
    let mut cp = ColorPicker::new();
    cp.next_mode();
    cp.next_mode();
    cp.next_mode(); // Switch to Hex mode

    let handled = cp.handle_key(&Key::Tab);
    assert!(handled);

    // Tab should cycle back to Palette mode
}

// =========================================================================
// Rendering tests
// =========================================================================

#[test]
fn test_color_picker_render_palette_mode() {
    let shown = rows(&ColorPicker::new(), 40, 12);
    // Basic: 8 swatches of 2 columns per row, the first selected (solid)
    assert_eq!(shown[0], format!("██{}{}", "▀▄".repeat(7), " ".repeat(24)));
    assert_eq!(shown[1], format!("{}{}", "▀▄".repeat(8), " ".repeat(24)));
    assert!(shown[2].trim().is_empty());
}

#[test]
fn test_color_picker_render_rgb_mode() {
    let cp = ColorPicker::new()
        .mode(ColorPickerMode::Rgb)
        .color(Color::rgb(255, 0, 51));
    let shown = rows(&cp, 40, 12);
    // A 34-cell track per channel, filled by the value, then the value
    assert_eq!(shown[0], format!("R {} 255", "█".repeat(34)));
    assert_eq!(shown[1], format!("G {}   0", "░".repeat(34)));
    assert_eq!(
        shown[2],
        format!("B {}{}  51", "█".repeat(6), "░".repeat(28))
    );
}

#[test]
fn test_color_picker_render_hsl_mode() {
    let cp = ColorPicker::new()
        .mode(ColorPickerMode::Hsl)
        .color(Color::rgb(255, 0, 0));
    let shown = rows(&cp, 40, 12);
    assert!(shown[0].starts_with("H ░") && shown[0].ends_with("  0"));
    assert_eq!(shown[1], format!("S {} 100", "█".repeat(34)));
    assert_eq!(
        shown[2],
        format!("L {}{}  50", "█".repeat(17), "░".repeat(17))
    );
}

#[test]
fn test_color_picker_render_hex_mode() {
    let mut cp = ColorPicker::new().mode(ColorPickerMode::Hex);
    assert!(rows(&cp, 40, 12)[0].starts_with("Hex: #______"));
    for c in "1A".chars() {
        cp.handle_key(&Key::Char(c));
    }
    let shown = rows(&cp, 40, 12);
    assert!(shown[0].starts_with("Hex: #1A____"));
    assert!(shown[2].starts_with("Current: #FFFFFF"));
}

#[test]
fn test_color_picker_render_with_border() {
    let cp = ColorPicker::new().border(Color::WHITE);
    let mut buffer = Buffer::new(40, 12);
    let area = Rect::new(0, 0, 40, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cp.render(&mut ctx);

    // Check for border corners
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(39, 0).unwrap().symbol, '┐');
    assert_eq!(buffer.get(0, 11).unwrap().symbol, '└');
    assert_eq!(buffer.get(39, 11).unwrap().symbol, '┘');
}

#[test]
fn test_color_picker_render_with_custom_size() {
    // The 30x10 area, not the size, places the preview: rows 8 and 9
    let cp = ColorPicker::new().size(30, 10);
    let shown = rows(&cp, 30, 10);
    assert!(shown[8].starts_with("██████ #FFFFFF"));
    assert!(shown[9].starts_with("██████"));
}

#[test]
fn test_color_picker_render_too_small_width() {
    // Under 10 columns, nothing is drawn
    assert!(rows(&ColorPicker::new(), 9, 10)
        .iter()
        .all(|r| r.trim().is_empty()));
    assert!(!rows(&ColorPicker::new(), 10, 10)[0].trim().is_empty());
}

#[test]
fn test_color_picker_render_too_small_height() {
    // Under 4 rows, nothing is drawn
    assert!(rows(&ColorPicker::new(), 40, 3)
        .iter()
        .all(|r| r.trim().is_empty()));
    assert!(!rows(&ColorPicker::new(), 40, 4)[0].trim().is_empty());
}

#[test]
fn test_color_picker_render_zero_area() {
    let cp = ColorPicker::new();
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cp.render(&mut ctx);
    assert!((0..5).all(|y| (0..5).all(|x| buffer.get(x, y).unwrap().symbol == ' ')));
}

#[test]
fn test_color_picker_render_with_preview_enabled() {
    let cp = ColorPicker::new().preview(true).color(Color::RED);
    let mut buffer = Buffer::new(40, 12);
    let area = Rect::new(0, 0, 40, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cp.render(&mut ctx);
    // The block is drawn in the color itself
    for (x, y) in [(0, 10), (5, 11)] {
        assert_eq!(buffer.get(x, y).unwrap().fg, Some(Color::RED));
    }
    assert_eq!(buffer.get(6, 10).unwrap().symbol, ' ');
}

#[test]
fn test_color_picker_render_with_preview_disabled() {
    // Without the preview the palette may use the last rows (Extended's 16 x 16 grid)
    let shown = rows(
        &ColorPicker::new()
            .palette(ColorPalette::Extended)
            .preview(false),
        32,
        12,
    );
    assert!(shown.iter().all(|r| !r.contains('#')));
    assert!(shown[9].contains('▀'));
}

#[test]
fn test_color_picker_render_with_hex_enabled() {
    let cp = ColorPicker::new().hex(true).color(Color::RED);
    let mut buffer = Buffer::new(40, 12);
    let area = Rect::new(0, 0, 40, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cp.render(&mut ctx);
    let hex: String = (7..14).map(|x| buffer.get(x, 10).unwrap().symbol).collect();
    assert_eq!(hex, "#FF0000");
    assert_eq!(buffer.get(7, 10).unwrap().fg, Some(Color::WHITE));
}

#[test]
fn test_color_picker_render_with_hex_disabled() {
    // Only the hex text goes; the block stays
    let shown = rows(&ColorPicker::new().hex(false), 40, 12);
    assert_eq!(shown[10].trim_end(), "██████");
    assert!(!shown.iter().any(|r| r.contains('#')));
}

#[test]
fn test_color_picker_render_all_palettes() {
    // Each palette draws its own first row of swatches
    let firsts: Vec<Vec<Option<Color>>> = PALETTES
        .iter()
        .map(|&palette| {
            let cp = ColorPicker::new().palette(palette);
            let mut buffer = Buffer::new(40, 12);
            let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 12));
            cp.render(&mut ctx);
            (0..40).map(|x| buffer.get(x, 0).unwrap().fg).collect()
        })
        .collect();
    for (i, a) in firsts.iter().enumerate() {
        for b in &firsts[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

// =========================================================================
// CSS integration tests
// =========================================================================

#[test]
fn test_color_picker_css_id() {
    let cp = ColorPicker::new().element_id("my-color-picker");
    assert_eq!(View::id(&cp), Some("my-color-picker"));

    let meta = cp.meta();
    assert_eq!(meta.id, Some("my-color-picker".to_string()));
}

#[test]
fn test_color_picker_css_classes() {
    let cp = ColorPicker::new().class("picker").class("themed");

    assert!(cp.has_class("picker"));
    assert!(cp.has_class("themed"));
    assert!(!cp.has_class("other"));

    let meta = cp.meta();
    assert!(meta.classes.contains("picker"));
    assert!(meta.classes.contains("themed"));
}

#[test]
fn test_color_picker_styled_view() {
    let mut cp = ColorPicker::new();

    cp.set_id("test-id");
    assert_eq!(View::id(&cp), Some("test-id"));

    cp.add_class("active");
    assert!(cp.has_class("active"));

    cp.remove_class("active");
    assert!(!cp.has_class("active"));

    cp.toggle_class("selected");
    assert!(cp.has_class("selected"));

    cp.toggle_class("selected");
    assert!(!cp.has_class("selected"));
}

#[test]
fn test_color_picker_combined_with_classes() {
    let cp = ColorPicker::new()
        .element_id("my-picker")
        .class("custom")
        .class("picker")
        .color(Color::GREEN)
        .mode(ColorPickerMode::Rgb)
        .palette(ColorPalette::Material)
        .size(35, 10);

    assert_eq!(cp.get_color(), Color::GREEN);
    assert_eq!(View::id(&cp), Some("my-picker"));
    assert!(cp.has_class("custom"));
    assert!(cp.has_class("picker"));

    let mut buffer = Buffer::new(35, 10);
    let area = Rect::new(0, 0, 35, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cp.render(&mut ctx);
}

// =========================================================================
// Edge case tests
// =========================================================================

// Note: ColorPicker doesn't implement Clone, so we can't test that.
// The widget would need to derive Clone for this functionality.

#[test]
fn test_color_picker_multiple_mode_cycles() {
    // Ten steps from Palette: two cycles and two more, to Hsl
    let mut cp = ColorPicker::new();
    for _ in 0..10 {
        cp.next_mode();
    }
    assert_eq!(shown_mode(&cp), ColorPickerMode::Hsl);
}

#[test]
fn test_color_picker_palette_boundary_navigation() {
    let colors = ColorPalette::Basic.colors();
    let mut cp = ColorPicker::new();
    // Right stops at the last color, still taking the key
    for _ in 0..30 {
        assert!(cp.handle_key(&Key::Right));
    }
    assert_eq!(cp.get_color(), colors[colors.len() - 1]);
    // Left stops at the first
    for _ in 0..30 {
        cp.handle_key(&Key::Left);
    }
    assert_eq!(cp.get_color(), colors[0]);
}

#[test]
fn test_color_picker_rgb_saturating_arithmetic() {
    let mut cp = ColorPicker::new().color(Color::rgb(250, 0, 0));
    cp.next_mode(); // RGB mode

    // Test at boundary - should not overflow
    for _ in 0..20 {
        cp.handle_key(&Key::Right);
    }

    let _color = cp.get_color();

    // Test at minimum - should not underflow
    for _ in 0..100 {
        cp.handle_key(&Key::Left);
    }

    let color = cp.get_color();
    assert_eq!(color.r, 0);
}

#[test]
fn test_color_picker_all_modes_render() {
    // Each mode's content on row 0, and the preview on rows 10 and 11
    for (mode, first) in [
        (ColorPickerMode::Palette, "██▀▄"),
        (ColorPickerMode::Rgb, "R ███"),
        (ColorPickerMode::Hsl, "H ░░░"),
        (ColorPickerMode::Hex, "Hex: #______"),
    ] {
        let shown = rows(&ColorPicker::new().mode(mode), 40, 12);
        assert!(shown[0].starts_with(first), "{mode:?}: {:?}", shown[0]);
        assert!(shown[10].starts_with("██████ #FFFFFF"), "{mode:?}");
    }
}

#[test]
fn test_color_picker_all_palettes_render() {
    // The first swatch, selected, is drawn solid in the palette's first color
    for palette in PALETTES {
        let cp = ColorPicker::new().palette(palette);
        let mut buffer = Buffer::new(40, 12);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 12));
        cp.render(&mut ctx);
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(
            (cell.symbol, cell.fg),
            ('█', Some(palette.colors()[0])),
            "{palette:?}"
        );
    }
}

#[test]
fn test_color_picker_all_builder_combinations() {
    let palettes = [
        ColorPalette::Basic,
        ColorPalette::Extended,
        ColorPalette::WebSafe,
        ColorPalette::Material,
        ColorPalette::Pastel,
    ];

    let modes = [
        ColorPickerMode::Palette,
        ColorPickerMode::Rgb,
        ColorPickerMode::Hsl,
        ColorPickerMode::Hex,
    ];

    for mode in modes {
        for palette in palettes {
            let cp = ColorPicker::new()
                .mode(mode)
                .palette(palette)
                .color(Color::rgb(128, 64, 192));

            // Verify it was constructed successfully
            assert_eq!(cp.get_color(), Color::rgb(128, 64, 192));
        }
    }
}

// Note: ColorPicker doesn't implement Debug, so we can't test that.
// The widget would need to derive Debug for this functionality.

#[test]
fn test_color_picker_render_with_all_options() {
    let cp = ColorPicker::new()
        .color(Color::rgb(255, 128, 64))
        .mode(ColorPickerMode::Rgb)
        .palette(ColorPalette::Material)
        .preview(true)
        .hex(true)
        .border(Color::WHITE)
        .size(40, 12);

    let mut buffer = Buffer::new(40, 12);
    let area = Rect::new(0, 0, 40, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cp.render(&mut ctx);

    // Verify border is present
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_color_picker_navigation_with_different_palettes() {
    // Down moves by the palette's columns, Up comes back
    for palette in [
        ColorPalette::Basic,
        ColorPalette::Material,
        ColorPalette::Pastel,
    ] {
        let colors = palette.colors();
        let (cols, _) = palette.grid_size();
        let mut cp = ColorPicker::new().palette(palette);
        for _ in 0..5 {
            cp.handle_key(&Key::Right);
        }
        for _ in 0..3 {
            cp.handle_key(&Key::Left);
        }
        assert_eq!(cp.get_color(), colors[2], "{palette:?}");
        cp.handle_key(&Key::Down);
        assert_eq!(cp.get_color(), colors[2 + cols], "{palette:?}");
        cp.handle_key(&Key::Up);
        assert_eq!(cp.get_color(), colors[2], "{palette:?}");
    }
}

#[test]
fn test_color_picker_color_transitions() {
    let mut cp = ColorPicker::new();

    // Test various color transitions
    cp.set_color(Color::RED);
    assert_eq!(cp.get_color(), Color::RED);

    cp.set_color(Color::GREEN);
    assert_eq!(cp.get_color(), Color::GREEN);

    cp.set_color(Color::BLUE);
    assert_eq!(cp.get_color(), Color::BLUE);

    cp.set_color(Color::WHITE);
    assert_eq!(cp.get_color(), Color::WHITE);

    cp.set_color(Color::BLACK);
    assert_eq!(cp.get_color(), Color::BLACK);
}

#[test]
fn test_color_picker_hex_roundtrip() {
    let test_colors = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::WHITE,
        Color::BLACK,
        Color::rgb(128, 64, 192),
        Color::rgb(255, 128, 0),
    ];

    for original_color in test_colors {
        let mut cp = ColorPicker::new();
        cp.set_color(original_color);

        let hex = cp.hex_string();

        let mut cp2 = ColorPicker::new();
        assert!(cp2.set_hex(&hex));

        assert_eq!(cp2.get_color(), original_color);
    }
}

#[test]
fn test_color_picker_all_modes_with_key_input() {
    // The arrows and Tab are taken in every mode; Tab moves to the next
    let modes = [
        ColorPickerMode::Palette,
        ColorPickerMode::Rgb,
        ColorPickerMode::Hsl,
        ColorPickerMode::Hex,
    ];

    let keys = vec![Key::Up, Key::Down, Key::Left, Key::Right, Key::Tab];

    for (i, mode) in modes.iter().enumerate() {
        let mut cp = ColorPicker::new().mode(*mode);

        for key in &keys {
            let used = cp.handle_key(key);
            // Hex mode takes only hex digits, Backspace, Enter and Tab
            assert_eq!(
                used,
                *mode != ColorPickerMode::Hex || *key == Key::Tab,
                "{mode:?} {key:?}"
            );
        }
        assert_eq!(shown_mode(&cp), modes[(i + 1) % 4]);
    }
}
