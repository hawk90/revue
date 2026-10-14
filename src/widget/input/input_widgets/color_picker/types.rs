//! Color picker widget types

/// Color picker mode
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ColorPickerMode {
    /// Palette grid selection
    #[default]
    Palette,
    /// RGB sliders
    Rgb,
    /// HSL sliders
    Hsl,
    /// Hex input
    Hex,
}

/// Predefined color palette
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ColorPalette {
    /// Basic 16 colors
    #[default]
    Basic,
    /// Extended 256 colors
    Extended,
    /// Web-safe colors
    WebSafe,
    /// Material Design colors
    Material,
    /// Pastel colors
    Pastel,
}
