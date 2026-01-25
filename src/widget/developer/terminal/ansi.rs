//! ANSI escape sequence parser

use super::types::TermCell;
use crate::render::Modifier;
use crate::style::Color;

/// ANSI parser state
#[derive(Clone, Debug, Default)]
pub(super) struct AnsiParser {
    /// Current state
    state: ParserState,
    /// CSI parameters
    params: Vec<u16>,
    /// Current parameter being built
    current_param: Option<u16>,
    /// Current foreground
    fg: Color,
    /// Current background
    bg: Color,
    /// Current modifiers
    modifiers: Modifier,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum ParserState {
    #[default]
    Normal,
    Escape,
    Csi,
    OscStart,
    Osc,
}

impl AnsiParser {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn reset_attrs(&mut self) {
        self.fg = Color::WHITE;
        self.bg = Color::BLACK;
        self.modifiers = Modifier::empty();
    }

    pub(super) fn reset_fg(&mut self, color: Color) {
        self.fg = color;
    }

    pub(super) fn reset_bg(&mut self, color: Color) {
        self.bg = color;
    }

    /// Parse a character and return cell if printable
    pub(super) fn parse(&mut self, ch: char) -> Option<TermCell> {
        match self.state {
            ParserState::Normal => {
                if ch == '\x1b' {
                    self.state = ParserState::Escape;
                    None
                } else if ch.is_control() && ch != '\n' && ch != '\r' && ch != '\t' {
                    None
                } else {
                    Some(TermCell {
                        ch,
                        fg: self.fg,
                        bg: self.bg,
                        modifiers: self.modifiers,
                    })
                }
            }
            ParserState::Escape => match ch {
                '[' => {
                    self.state = ParserState::Csi;
                    self.params.clear();
                    self.current_param = None;
                    None
                }
                ']' => {
                    self.state = ParserState::OscStart;
                    None
                }
                _ => {
                    self.state = ParserState::Normal;
                    None
                }
            },
            ParserState::Csi => {
                if ch.is_ascii_digit() {
                    let digit = ch.to_digit(10).unwrap() as u16;
                    self.current_param = Some(
                        self.current_param
                            .unwrap_or(0)
                            .saturating_mul(10)
                            .saturating_add(digit),
                    );
                    None
                } else if ch == ';' {
                    self.params.push(self.current_param.unwrap_or(0));
                    self.current_param = None;
                    None
                } else {
                    // End of CSI sequence
                    if let Some(p) = self.current_param {
                        self.params.push(p);
                    }
                    self.handle_csi(ch);
                    self.state = ParserState::Normal;
                    None
                }
            }
            ParserState::OscStart => {
                self.state = ParserState::Osc;
                None
            }
            ParserState::Osc => {
                // OSC sequences end with BEL or ST
                if ch == '\x07' || ch == '\\' {
                    self.state = ParserState::Normal;
                }
                None
            }
        }
    }

    fn handle_csi(&mut self, cmd: char) {
        if cmd == 'm' {
            self.handle_sgr();
        }
        // Other CSI commands can be added here
    }

    fn handle_sgr(&mut self) {
        if self.params.is_empty() {
            self.reset_attrs();
            return;
        }

        let mut i = 0;
        while i < self.params.len() {
            match self.params[i] {
                0 => self.reset_attrs(),
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
                // Standard foreground colors
                30 => self.fg = Color::BLACK,
                31 => self.fg = Color::RED,
                32 => self.fg = Color::GREEN,
                33 => self.fg = Color::YELLOW,
                34 => self.fg = Color::BLUE,
                35 => self.fg = Color::MAGENTA,
                36 => self.fg = Color::CYAN,
                37 => self.fg = Color::WHITE,
                38 => {
                    // Extended foreground color
                    if i + 2 < self.params.len() && self.params[i + 1] == 5 {
                        // 256 color
                        self.fg = color_256(self.params[i + 2]);
                        i += 2;
                    } else if i + 4 < self.params.len() && self.params[i + 1] == 2 {
                        // RGB color
                        self.fg = Color::rgb(
                            self.params[i + 2] as u8,
                            self.params[i + 3] as u8,
                            self.params[i + 4] as u8,
                        );
                        i += 4;
                    }
                }
                39 => self.fg = Color::WHITE,
                // Standard background colors
                40 => self.bg = Color::BLACK,
                41 => self.bg = Color::RED,
                42 => self.bg = Color::GREEN,
                43 => self.bg = Color::YELLOW,
                44 => self.bg = Color::BLUE,
                45 => self.bg = Color::MAGENTA,
                46 => self.bg = Color::CYAN,
                47 => self.bg = Color::WHITE,
                48 => {
                    // Extended background color
                    if i + 2 < self.params.len() && self.params[i + 1] == 5 {
                        // 256 color
                        self.bg = color_256(self.params[i + 2]);
                        i += 2;
                    } else if i + 4 < self.params.len() && self.params[i + 1] == 2 {
                        // RGB color
                        self.bg = Color::rgb(
                            self.params[i + 2] as u8,
                            self.params[i + 3] as u8,
                            self.params[i + 4] as u8,
                        );
                        i += 4;
                    }
                }
                49 => self.bg = Color::BLACK,
                // Bright foreground colors
                90 => self.fg = Color::rgb(128, 128, 128),
                91 => self.fg = Color::rgb(255, 85, 85),
                92 => self.fg = Color::rgb(85, 255, 85),
                93 => self.fg = Color::rgb(255, 255, 85),
                94 => self.fg = Color::rgb(85, 85, 255),
                95 => self.fg = Color::rgb(255, 85, 255),
                96 => self.fg = Color::rgb(85, 255, 255),
                97 => self.fg = Color::WHITE,
                // Bright background colors
                100 => self.bg = Color::rgb(128, 128, 128),
                101 => self.bg = Color::rgb(255, 85, 85),
                102 => self.bg = Color::rgb(85, 255, 85),
                103 => self.bg = Color::rgb(255, 255, 85),
                104 => self.bg = Color::rgb(85, 85, 255),
                105 => self.bg = Color::rgb(255, 85, 255),
                106 => self.bg = Color::rgb(85, 255, 255),
                107 => self.bg = Color::WHITE,
                _ => {}
            }
            i += 1;
        }
    }
}

/// Convert 256-color code to RGB
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
