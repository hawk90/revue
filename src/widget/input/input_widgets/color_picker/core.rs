//! Color picker widget core implementation

use super::types::{ColorPalette, ColorPickerMode};
use crate::style::Color;
use crate::utils::color::{hsl_to_rgb, rgb_to_hsl};
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Color Picker widget
#[derive(Clone)]
pub struct ColorPicker {
    /// Current mode
    pub mode: ColorPickerMode,
    /// Selected color
    pub color: Color,
    /// RGB components
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
    /// HSL components (0-360, 0-100, 0-100)
    pub h: u16,
    /// Saturation component
    pub s: u8,
    /// Lightness component
    pub l: u8,
    /// Hex input
    pub hex_input: String,
    /// Current palette
    pub palette: ColorPalette,
    /// Selected index in palette
    pub palette_index: usize,
    /// Active RGB/HSL slider (0=R/H, 1=G/S, 2=B/L)
    pub active_slider: usize,
    /// Show preview
    pub show_preview: bool,
    /// Show hex value
    pub show_hex: bool,
    /// Border color
    pub border_color: Option<Color>,
    /// Width
    pub width: u16,
    /// Height
    pub height: u16,
    /// Widget properties
    pub props: WidgetProps,
}

impl ColorPicker {
    /// Create a new color picker
    pub fn new() -> Self {
        Self {
            mode: ColorPickerMode::Palette,
            color: Color::WHITE,
            r: 255,
            g: 255,
            b: 255,
            h: 0,
            s: 0,
            l: 100,
            hex_input: String::new(),
            palette: ColorPalette::Basic,
            palette_index: 0,
            active_slider: 0,
            show_preview: true,
            show_hex: true,
            border_color: None,
            width: 40,
            height: 12,
            props: WidgetProps::new(),
        }
    }

    /// Set initial color
    pub fn color(mut self, color: Color) -> Self {
        self.set_color(color);
        self
    }

    /// Set mode
    pub fn mode(mut self, mode: ColorPickerMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set palette
    pub fn palette(mut self, palette: ColorPalette) -> Self {
        self.palette = palette;
        self
    }

    /// Show/hide preview
    pub fn preview(mut self, show: bool) -> Self {
        self.show_preview = show;
        self
    }

    /// Show/hide hex value
    pub fn hex(mut self, show: bool) -> Self {
        self.show_hex = show;
        self
    }

    /// Set border
    pub fn border(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Set size
    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.width = width.max(20);
        self.height = height.max(6);
        self
    }

    /// Get current color
    pub fn get_color(&self) -> Color {
        self.color
    }

    /// Set color and update all representations
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        // Extract RGB from Color struct
        self.r = color.r;
        self.g = color.g;
        self.b = color.b;
        // Calculate HSL
        let (h, s, l) = rgb_to_hsl(color);
        self.h = h;
        self.s = s;
        self.l = l;
    }

    /// Update color from RGB
    pub(crate) fn update_from_rgb(&mut self) {
        self.color = Color::rgb(self.r, self.g, self.b);
        let (h, s, l) = rgb_to_hsl(self.color);
        self.h = h;
        self.s = s;
        self.l = l;
    }

    /// Update color from HSL
    pub(crate) fn update_from_hsl(&mut self) {
        let rgb = hsl_to_rgb(self.h, self.s, self.l);
        self.r = rgb.r;
        self.g = rgb.g;
        self.b = rgb.b;
        self.color = rgb;
    }

    /// Get hex string
    pub fn hex_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Set color from hex string
    pub fn set_hex(&mut self, hex: &str) -> bool {
        let hex = hex.trim_start_matches('#');
        // Hex digits only: the slices below are by byte (#863)
        if hex.len() == 6 && hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                self.r = r;
                self.g = g;
                self.b = b;
                self.update_from_rgb();
                return true;
            }
        }
        false
    }

    /// Cycle to next mode
    pub fn next_mode(&mut self) {
        self.mode = match self.mode {
            ColorPickerMode::Palette => ColorPickerMode::Rgb,
            ColorPickerMode::Rgb => ColorPickerMode::Hsl,
            ColorPickerMode::Hsl => ColorPickerMode::Hex,
            ColorPickerMode::Hex => ColorPickerMode::Palette,
        };
    }
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(ColorPicker);
impl_props_builders!(ColorPicker);
