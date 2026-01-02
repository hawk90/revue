//! Color Picker widget for selecting colors
//!
//! Provides a visual color selection interface with palette,
//! RGB sliders, and hex input.

use super::traits::{RenderContext, View, WidgetProps};
use crate::event::Key;
use crate::layout::Rect;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::border::render_border;
use crate::utils::color::{hsl_to_rgb, rgb_to_hsl};
use crate::{impl_props_builders, impl_styled_view};

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

impl ColorPalette {
    /// Get colors for this palette
    fn colors(&self) -> Vec<Color> {
        match self {
            ColorPalette::Basic => vec![
                Color::BLACK,
                Color::rgb(128, 0, 0),
                Color::rgb(0, 128, 0),
                Color::rgb(128, 128, 0),
                Color::rgb(0, 0, 128),
                Color::rgb(128, 0, 128),
                Color::rgb(0, 128, 128),
                Color::rgb(192, 192, 192),
                Color::rgb(128, 128, 128),
                Color::RED,
                Color::GREEN,
                Color::YELLOW,
                Color::BLUE,
                Color::MAGENTA,
                Color::CYAN,
                Color::WHITE,
            ],
            ColorPalette::Extended => {
                let mut colors = Vec::with_capacity(256);
                // Standard 16 colors
                colors.extend(ColorPalette::Basic.colors());
                // 216 color cube (6x6x6)
                for r in 0..6 {
                    for g in 0..6 {
                        for b in 0..6 {
                            let r = if r > 0 { 55 + r * 40 } else { 0 };
                            let g = if g > 0 { 55 + g * 40 } else { 0 };
                            let b = if b > 0 { 55 + b * 40 } else { 0 };
                            colors.push(Color::rgb(r, g, b));
                        }
                    }
                }
                // 24 grayscale
                for i in 0..24 {
                    let v = 8 + i * 10;
                    colors.push(Color::rgb(v, v, v));
                }
                colors
            }
            ColorPalette::WebSafe => {
                let mut colors = Vec::with_capacity(216);
                for r in (0..=255).step_by(51) {
                    for g in (0..=255).step_by(51) {
                        for b in (0..=255).step_by(51) {
                            colors.push(Color::rgb(r as u8, g as u8, b as u8));
                        }
                    }
                }
                colors
            }
            ColorPalette::Material => vec![
                // Red
                Color::rgb(244, 67, 54),
                Color::rgb(229, 115, 115),
                Color::rgb(183, 28, 28),
                // Pink
                Color::rgb(233, 30, 99),
                Color::rgb(240, 98, 146),
                Color::rgb(136, 14, 79),
                // Purple
                Color::rgb(156, 39, 176),
                Color::rgb(186, 104, 200),
                Color::rgb(74, 20, 140),
                // Blue
                Color::rgb(33, 150, 243),
                Color::rgb(100, 181, 246),
                Color::rgb(13, 71, 161),
                // Cyan
                Color::rgb(0, 188, 212),
                Color::rgb(77, 208, 225),
                Color::rgb(0, 96, 100),
                // Green
                Color::rgb(76, 175, 80),
                Color::rgb(129, 199, 132),
                Color::rgb(27, 94, 32),
                // Yellow
                Color::rgb(255, 235, 59),
                Color::rgb(255, 241, 118),
                Color::rgb(245, 127, 23),
                // Orange
                Color::rgb(255, 152, 0),
                Color::rgb(255, 183, 77),
                Color::rgb(230, 81, 0),
                // Brown
                Color::rgb(121, 85, 72),
                Color::rgb(161, 136, 127),
                Color::rgb(62, 39, 35),
                // Grey
                Color::rgb(158, 158, 158),
                Color::rgb(189, 189, 189),
                Color::rgb(66, 66, 66),
            ],
            ColorPalette::Pastel => vec![
                Color::rgb(255, 179, 186),
                Color::rgb(255, 223, 186),
                Color::rgb(255, 255, 186),
                Color::rgb(186, 255, 201),
                Color::rgb(186, 225, 255),
                Color::rgb(219, 186, 255),
                Color::rgb(255, 186, 255),
                Color::rgb(255, 218, 233),
                Color::rgb(255, 240, 219),
                Color::rgb(240, 255, 219),
                Color::rgb(219, 255, 240),
                Color::rgb(219, 240, 255),
                Color::rgb(240, 219, 255),
                Color::rgb(255, 219, 240),
                Color::rgb(224, 224, 224),
                Color::rgb(245, 245, 245),
            ],
        }
    }

    /// Get grid dimensions for this palette
    fn grid_size(&self) -> (usize, usize) {
        match self {
            ColorPalette::Basic => (8, 2),
            ColorPalette::Extended => (16, 16),
            ColorPalette::WebSafe => (18, 12),
            ColorPalette::Material => (6, 5),
            ColorPalette::Pastel => (4, 4),
        }
    }
}

/// Color Picker widget
pub struct ColorPicker {
    /// Current mode
    mode: ColorPickerMode,
    /// Selected color
    color: Color,
    /// RGB components
    r: u8,
    g: u8,
    b: u8,
    /// HSL components (0-360, 0-100, 0-100)
    h: u16,
    s: u8,
    l: u8,
    /// Hex input
    hex_input: String,
    /// Current palette
    palette: ColorPalette,
    /// Selected index in palette
    palette_index: usize,
    /// Active RGB/HSL slider (0=R/H, 1=G/S, 2=B/L)
    active_slider: usize,
    /// Show preview
    show_preview: bool,
    /// Show hex value
    show_hex: bool,
    /// Border color
    border_color: Option<Color>,
    /// Width
    width: u16,
    /// Height
    height: u16,
    /// Widget properties
    props: WidgetProps,
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
    fn update_from_rgb(&mut self) {
        self.color = Color::rgb(self.r, self.g, self.b);
        let (h, s, l) = rgb_to_hsl(self.color);
        self.h = h;
        self.s = s;
        self.l = l;
    }

    /// Update color from HSL
    fn update_from_hsl(&mut self) {
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

impl View for ColorPicker {
    crate::impl_view_meta!("ColorPicker");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 10 || area.height < 4 {
            return;
        }

        // Draw border if set
        if let Some(border_color) = self.border_color {
            render_border(ctx, area, border_color);
        }

        let content_area = if self.border_color.is_some() {
            Rect::new(
                area.x + 1,
                area.y + 1,
                area.width.saturating_sub(2),
                area.height.saturating_sub(2),
            )
        } else {
            area
        };

        match self.mode {
            ColorPickerMode::Palette => self.render_palette(ctx, content_area),
            ColorPickerMode::Rgb => self.render_rgb(ctx, content_area),
            ColorPickerMode::Hsl => self.render_hsl(ctx, content_area),
            ColorPickerMode::Hex => self.render_hex_mode(ctx, content_area),
        }

        // Render preview and hex if enabled
        if self.show_preview {
            self.render_preview(ctx, content_area);
        }
    }
}

impl ColorPicker {
    fn render_palette(&self, ctx: &mut RenderContext, area: Rect) {
        let colors = self.palette.colors();
        let (cols, _rows) = self.palette.grid_size();

        let mut x = area.x;
        let mut y = area.y;

        for (i, color) in colors.iter().enumerate() {
            if x + 2 > area.x + area.width {
                x = area.x;
                y += 1;
            }
            if y >= area.y + area.height.saturating_sub(2) {
                break;
            }

            let is_selected = i == self.palette_index;

            // Color block
            let ch = if is_selected { '█' } else { '▀' };
            let mut cell = Cell::new(ch);
            cell.fg = Some(*color);
            if is_selected {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.buffer.set(x, y, cell);

            let mut cell2 = Cell::new(if is_selected { '█' } else { '▄' });
            cell2.fg = Some(*color);
            ctx.buffer.set(x + 1, y, cell2);

            x += 2;
            if (i + 1) % cols == 0 {
                x = area.x;
                y += 1;
            }
        }
    }

    fn render_rgb(&self, ctx: &mut RenderContext, area: Rect) {
        let sliders = [
            ("R", self.r as f32 / 255.0, Color::RED),
            ("G", self.g as f32 / 255.0, Color::GREEN),
            ("B", self.b as f32 / 255.0, Color::BLUE),
        ];

        self.render_sliders(ctx, area, &sliders);
    }

    fn render_hsl(&self, ctx: &mut RenderContext, area: Rect) {
        let sliders = [
            ("H", self.h as f32 / 360.0, Color::MAGENTA),
            ("S", self.s as f32 / 100.0, Color::CYAN),
            ("L", self.l as f32 / 100.0, Color::YELLOW),
        ];

        self.render_sliders(ctx, area, &sliders);
    }

    fn render_sliders(&self, ctx: &mut RenderContext, area: Rect, sliders: &[(&str, f32, Color)]) {
        let slider_width = (area.width.saturating_sub(6)) as usize;

        for (i, (label, value, color)) in sliders.iter().enumerate() {
            let y = area.y + i as u16;
            if y >= area.y + area.height {
                break;
            }

            let is_active = i == self.active_slider;

            // Label
            let mut label_cell = Cell::new(label.chars().next().unwrap_or(' '));
            label_cell.fg = Some(if is_active { *color } else { Color::WHITE });
            if is_active {
                label_cell.modifier |= Modifier::BOLD;
            }
            ctx.buffer.set(area.x, y, label_cell);

            // Slider track
            let filled = (value * slider_width as f32) as usize;
            for j in 0..slider_width {
                let ch = if j < filled { '█' } else { '░' };
                let mut cell = Cell::new(ch);
                cell.fg = Some(if j < filled {
                    *color
                } else {
                    Color::rgb(60, 60, 60)
                });
                ctx.buffer.set(area.x + 2 + j as u16, y, cell);
            }

            // Value
            let val_str = match i {
                0 if self.mode == ColorPickerMode::Hsl => format!("{:3}", self.h),
                1 if self.mode == ColorPickerMode::Hsl => format!("{:3}", self.s),
                2 if self.mode == ColorPickerMode::Hsl => format!("{:3}", self.l),
                0 => format!("{:3}", self.r),
                1 => format!("{:3}", self.g),
                _ => format!("{:3}", self.b),
            };

            let val_x = area.x + 2 + slider_width as u16 + 1;
            for (j, ch) in val_str.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                ctx.buffer.set(val_x + j as u16, y, cell);
            }
        }
    }

    fn render_hex_mode(&self, ctx: &mut RenderContext, area: Rect) {
        // Label
        let label = "Hex: #";
        for (i, ch) in label.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            ctx.buffer.set(area.x + i as u16, area.y, cell);
        }

        // Input field
        let input_x = area.x + label.len() as u16;
        for i in 0..6 {
            let ch = self.hex_input.chars().nth(i).unwrap_or('_');
            let mut cell = Cell::new(ch);
            cell.fg = Some(if i < self.hex_input.len() {
                Color::CYAN
            } else {
                Color::rgb(60, 60, 60)
            });
            ctx.buffer.set(input_x + i as u16, area.y, cell);
        }

        // Current hex value
        let current = format!("Current: {}", self.hex_string());
        for (i, ch) in current.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(150, 150, 150));
            ctx.buffer.set(area.x + i as u16, area.y + 2, cell);
        }
    }

    fn render_preview(&self, ctx: &mut RenderContext, area: Rect) {
        let preview_y = area.y + area.height.saturating_sub(2);
        let preview_width = 6u16;

        // Preview block
        for dy in 0..2 {
            for dx in 0..preview_width {
                let mut cell = Cell::new('█');
                cell.fg = Some(self.color);
                ctx.buffer.set(area.x + dx, preview_y + dy, cell);
            }
        }

        // Hex value next to preview
        if self.show_hex {
            let hex = self.hex_string();
            for (i, ch) in hex.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                ctx.buffer
                    .set(area.x + preview_width + 1 + i as u16, preview_y, cell);
            }
        }
    }
}

impl_styled_view!(ColorPicker);
impl_props_builders!(ColorPicker);

/// Helper to create a color picker
pub fn color_picker() -> ColorPicker {
    ColorPicker::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;

    #[test]
    fn test_color_picker_new() {
        let cp = ColorPicker::new();
        assert_eq!(cp.mode, ColorPickerMode::Palette);
        assert_eq!(cp.get_color(), Color::WHITE);
    }

    #[test]
    fn test_color_picker_set_color() {
        let mut cp = ColorPicker::new();
        cp.set_color(Color::RED);
        assert_eq!(cp.r, 255);
        assert_eq!(cp.g, 0);
        assert_eq!(cp.b, 0);
    }

    #[test]
    fn test_hex_string() {
        let cp = ColorPicker::new().color(Color::rgb(255, 128, 64));
        assert_eq!(cp.hex_string(), "#FF8040");
    }

    #[test]
    fn test_set_hex() {
        let mut cp = ColorPicker::new();
        assert!(cp.set_hex("FF0000"));
        assert_eq!(cp.r, 255);
        assert_eq!(cp.g, 0);
        assert_eq!(cp.b, 0);

        assert!(cp.set_hex("#00FF00"));
        assert_eq!(cp.g, 255);
    }

    #[test]
    fn test_set_hex_invalid() {
        let mut cp = ColorPicker::new();
        assert!(!cp.set_hex("invalid"));
        assert!(!cp.set_hex("12345")); // Too short
    }

    #[test]
    fn test_rgb_to_hsl() {
        let (h, s, l) = rgb_to_hsl(Color::RED);
        assert_eq!(h, 0);
        assert!(s > 90);
        assert!(l > 40 && l < 60);

        let (h, _s, _l) = rgb_to_hsl(Color::GREEN);
        assert!(h > 110 && h < 130);
    }

    #[test]
    fn test_hsl_to_rgb() {
        let c = hsl_to_rgb(0, 100, 50);
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 0);

        let c = hsl_to_rgb(120, 100, 50);
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

    #[test]
    fn test_mode_cycle() {
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
    fn test_handle_key_palette() {
        use crate::event::Key;

        let mut cp = ColorPicker::new();
        assert_eq!(cp.palette_index, 0);

        cp.handle_key(&Key::Right);
        assert_eq!(cp.palette_index, 1);

        cp.handle_key(&Key::Left);
        assert_eq!(cp.palette_index, 0);
    }

    #[test]
    fn test_handle_key_rgb() {
        use crate::event::Key;

        let mut cp = ColorPicker::new().mode(ColorPickerMode::Rgb);
        cp.r = 100;

        cp.handle_key(&Key::Right);
        assert_eq!(cp.r, 105);

        cp.handle_key(&Key::Left);
        assert_eq!(cp.r, 100);
    }

    #[test]
    fn test_render() {
        let mut buffer = Buffer::new(40, 12);
        let area = Rect::new(0, 0, 40, 12);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let cp = ColorPicker::new();
        cp.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_render_with_border() {
        let mut buffer = Buffer::new(40, 12);
        let area = Rect::new(0, 0, 40, 12);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let cp = ColorPicker::new().border(Color::WHITE);
        cp.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    }

    #[test]
    fn test_helper() {
        let cp = color_picker().palette(ColorPalette::Material);
        assert_eq!(cp.palette, ColorPalette::Material);
    }
}
