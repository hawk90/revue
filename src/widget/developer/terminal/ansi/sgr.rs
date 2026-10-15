//! SGR (Select Graphic Rendition, `ESC[...m`): the codes that set the
//! colors and modifiers of the cells written after them

use super::AnsiParser;
use crate::render::Modifier;
use crate::style::Color;

impl AnsiParser {
    pub(super) fn handle_sgr(&mut self) {
        if self.params.is_empty() {
            self.reset_attrs();
            return;
        }

        let mut i = 0;
        while i < self.params.len() {
            match self.params[i] {
                0 => self.reset_attrs(),
                // Standard foreground colors
                code @ 30..=37 => self.fg = color_256(code - 30),
                38 => {
                    // Extended foreground color
                    if let Some((color, used)) = self.extended_color(i) {
                        self.fg = color;
                        i += used;
                    }
                }
                39 => self.fg = Color::WHITE,
                // Standard background colors
                code @ 40..=47 => self.bg = color_256(code - 40),
                48 => {
                    // Extended background color
                    if let Some((color, used)) = self.extended_color(i) {
                        self.bg = color;
                        i += used;
                    }
                }
                49 => self.bg = Color::BLACK,
                // Bright foreground colors
                code @ 90..=97 => self.fg = color_256(code - 90 + 8),
                // Bright background colors
                code @ 100..=107 => self.bg = color_256(code - 100 + 8),
                code => self.apply_modifier(code),
            }
            i += 1;
        }
    }

    /// The color of a `38`/`48` code at `i` and how many parameters after it
    /// the color took: `5;n` (256 colors) or `2;r;g;b` (RGB). `None` when the
    /// parameters after it are missing or name another form.
    fn extended_color(&self, i: usize) -> Option<(Color, usize)> {
        if i + 2 < self.params.len() && self.params[i + 1] == 5 {
            // 256 color
            Some((color_256(self.params[i + 2]), 2))
        } else if i + 4 < self.params.len() && self.params[i + 1] == 2 {
            // RGB color
            Some((
                Color::rgb(
                    self.params[i + 2] as u8,
                    self.params[i + 3] as u8,
                    self.params[i + 4] as u8,
                ),
                4,
            ))
        } else {
            None
        }
    }

    /// The modifier codes; any other code is ignored
    fn apply_modifier(&mut self, code: u16) {
        match code {
            1 => self.modifiers |= Modifier::BOLD,
            2 => self.modifiers |= Modifier::DIM,
            3 => self.modifiers |= Modifier::ITALIC,
            4 => self.modifiers |= Modifier::UNDERLINE,
            5 | 6 => {} // Blink not supported
            7 => {}     // Reverse not supported
            8 => {}     // Hidden not supported
            9 => self.modifiers |= Modifier::CROSSED_OUT,
            22 => self.modifiers &= !(Modifier::BOLD | Modifier::DIM),
            23 => self.modifiers &= !Modifier::ITALIC,
            24 => self.modifiers &= !Modifier::UNDERLINE,
            25 => {} // Blink not supported
            27 => {} // Reverse not supported
            28 => {} // Hidden not supported
            29 => self.modifiers &= !Modifier::CROSSED_OUT,
            _ => {}
        }
    }
}

/// Convert 256-color code to RGB. Codes 0-15 are also the colors of SGR
/// 30-37/40-47 (0-7) and 90-97/100-107 (8-15).
fn color_256(code: u16) -> Color {
    match code {
        // Standard colors (0-15)
        0 => Color::BLACK,
        1 => Color::RED,
        2 => Color::GREEN,
        3 => Color::YELLOW,
        4 => Color::BLUE,
        5 => Color::MAGENTA,
        6 => Color::CYAN,
        7 => Color::WHITE,
        8 => Color::rgb(128, 128, 128),
        9 => Color::rgb(255, 85, 85),
        10 => Color::rgb(85, 255, 85),
        11 => Color::rgb(255, 255, 85),
        12 => Color::rgb(85, 85, 255),
        13 => Color::rgb(255, 85, 255),
        14 => Color::rgb(85, 255, 255),
        15 => Color::rgb(255, 255, 255),
        // 216 colors (16-231)
        16..=231 => {
            let n = code - 16;
            let r = (n / 36) as u8;
            let g = ((n % 36) / 6) as u8;
            let b = (n % 6) as u8;
            let to_val = |v: u8| if v == 0 { 0 } else { 55 + 40 * v };
            Color::rgb(to_val(r), to_val(g), to_val(b))
        }
        // Grayscale (232-255)
        232..=255 => {
            let gray = ((code - 232) * 10 + 8) as u8;
            Color::rgb(gray, gray, gray)
        }
        _ => Color::WHITE,
    }
}
