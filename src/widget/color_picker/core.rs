//! Color picker widget core implementation

use super::types::{ColorPalette, ColorPickerMode};
use crate::event::Key;
use crate::style::Color;
use crate::utils::color::{hsl_to_rgb, rgb_to_hsl};
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Color Picker widget
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
        if hex.len() == 6 {
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

    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match self.mode {
            ColorPickerMode::Palette => self.handle_palette_key(key),
            ColorPickerMode::Rgb => self.handle_slider_key(key, false),
            ColorPickerMode::Hsl => self.handle_slider_key(key, true),
            ColorPickerMode::Hex => self.handle_hex_key(key),
        }
    }

    fn handle_palette_key(&mut self, key: &Key) -> bool {
        let colors = self.palette.colors();
        let (cols, _rows) = self.palette.grid_size();

        match key {
            Key::Left | Key::Char('h') => {
                if self.palette_index > 0 {
                    self.palette_index -= 1;
                    self.set_color(colors[self.palette_index]);
                }
                true
            }
            Key::Right | Key::Char('l') => {
                if self.palette_index < colors.len() - 1 {
                    self.palette_index += 1;
                    self.set_color(colors[self.palette_index]);
                }
                true
            }
            Key::Up | Key::Char('k') => {
                if self.palette_index >= cols {
                    self.palette_index -= cols;
                    self.set_color(colors[self.palette_index]);
                }
                true
            }
            Key::Down | Key::Char('j') => {
                if self.palette_index + cols < colors.len() {
                    self.palette_index += cols;
                    self.set_color(colors[self.palette_index]);
                }
                true
            }
            Key::Tab => {
                self.next_mode();
                true
            }
            _ => false,
        }
    }

    fn handle_slider_key(&mut self, key: &Key, is_hsl: bool) -> bool {
        match key {
            Key::Up | Key::Char('k') => {
                self.active_slider = self.active_slider.saturating_sub(1);
                true
            }
            Key::Down | Key::Char('j') => {
                if self.active_slider < 2 {
                    self.active_slider += 1;
                }
                true
            }
            Key::Left | Key::Char('h') => {
                if is_hsl {
                    match self.active_slider {
                        0 => self.h = self.h.saturating_sub(5),
                        1 => self.s = self.s.saturating_sub(5),
                        2 => self.l = self.l.saturating_sub(5),
                        _ => {}
                    }
                    self.update_from_hsl();
                } else {
                    match self.active_slider {
                        0 => self.r = self.r.saturating_sub(5),
                        1 => self.g = self.g.saturating_sub(5),
                        2 => self.b = self.b.saturating_sub(5),
                        _ => {}
                    }
                    self.update_from_rgb();
                }
                true
            }
            Key::Right | Key::Char('l') => {
                if is_hsl {
                    match self.active_slider {
                        0 => self.h = (self.h + 5).min(360),
                        1 => self.s = (self.s + 5).min(100),
                        2 => self.l = (self.l + 5).min(100),
                        _ => {}
                    }
                    self.update_from_hsl();
                } else {
                    match self.active_slider {
                        0 => self.r = self.r.saturating_add(5),
                        1 => self.g = self.g.saturating_add(5),
                        2 => self.b = self.b.saturating_add(5),
                        _ => {}
                    }
                    self.update_from_rgb();
                }
                true
            }
            Key::Tab => {
                self.next_mode();
                true
            }
            _ => false,
        }
    }

    fn handle_hex_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char(c) if c.is_ascii_hexdigit() => {
                if self.hex_input.len() < 6 {
                    self.hex_input.push(c.to_ascii_uppercase());
                    if self.hex_input.len() == 6 {
                        let hex = self.hex_input.clone();
                        self.set_hex(&hex);
                    }
                }
                true
            }
            Key::Backspace => {
                self.hex_input.pop();
                true
            }
            Key::Enter => {
                let hex = self.hex_input.clone();
                self.set_hex(&hex);
                true
            }
            Key::Tab => {
                self.next_mode();
                true
            }
            _ => false,
        }
    }
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(ColorPicker);
impl_props_builders!(ColorPicker);
