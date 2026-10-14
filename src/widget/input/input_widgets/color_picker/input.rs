//! Keys: palette moves, slider moves and steps, hex entry, and Tab to the
//! next mode

use super::core::ColorPicker;
use super::types::ColorPickerMode;
use crate::event::Key;

impl ColorPicker {
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
        if colors.is_empty() {
            return false;
        }
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
                if self.palette_index + 1 < colors.len() {
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
