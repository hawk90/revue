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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // AnsiParser construction tests
    // =========================================================================

    #[test]
    fn test_ansi_parser_new() {
        let parser = AnsiParser::new();
        assert_eq!(parser.state, ParserState::Normal);
        assert!(parser.params.is_empty());
    }

    #[test]
    fn test_ansi_parser_default() {
        let parser = AnsiParser::default();
        assert_eq!(parser.state, ParserState::Normal);
    }

    #[test]
    fn test_ansi_parser_clone() {
        let parser = AnsiParser::new();
        let _ = parser.clone();
    }

    // =========================================================================
    // reset_attrs tests
    // =========================================================================

    #[test]
    fn test_ansi_parser_reset_attrs() {
        let mut parser = AnsiParser::new();
        parser.fg = Color::RED;
        parser.bg = Color::BLUE;
        parser.modifiers = Modifier::BOLD;
        parser.reset_attrs();
        assert_eq!(parser.fg, Color::WHITE);
        assert_eq!(parser.bg, Color::BLACK);
        assert_eq!(parser.modifiers, Modifier::empty());
    }

    // =========================================================================
    // reset_fg/reset_bg tests
    // =========================================================================

    #[test]
    fn test_ansi_parser_reset_fg() {
        let mut parser = AnsiParser::new();
        parser.reset_fg(Color::RED);
        assert_eq!(parser.fg, Color::RED);
    }

    #[test]
    fn test_ansi_parser_reset_bg() {
        let mut parser = AnsiParser::new();
        parser.reset_bg(Color::BLUE);
        assert_eq!(parser.bg, Color::BLUE);
    }

    // =========================================================================
    // parse normal character tests
    // =========================================================================

    #[test]
    fn test_ansi_parse_normal_char() {
        let mut parser = AnsiParser::new();
        let cell = parser.parse('A');
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().ch, 'A');
    }

    #[test]
    fn test_ansi_parse_normal_char_with_attrs() {
        let mut parser = AnsiParser::new();
        parser.fg = Color::RED;
        parser.modifiers = Modifier::BOLD;
        let cell = parser.parse('X');
        assert!(cell.is_some());
        let cell = cell.unwrap();
        assert_eq!(cell.ch, 'X');
        assert_eq!(cell.fg, Color::RED);
        assert_eq!(cell.modifiers, Modifier::BOLD);
    }

    #[test]
    fn test_ansi_parse_none_for_escape() {
        let mut parser = AnsiParser::new();
        let cell = parser.parse('\x1b');
        assert!(cell.is_none());
        assert_eq!(parser.state, ParserState::Escape);
    }

    // =========================================================================
    // CSI sequence tests
    // =========================================================================

    #[test]
    fn test_ansi_parse_csi_reset() {
        let mut parser = AnsiParser::new();
        parser.fg = Color::RED;
        parser.modifiers = Modifier::BOLD;
        // Send CSI reset sequence: ESC[m
        parser.parse('\x1b');
        parser.parse('[');
        parser.parse('m');
        assert_eq!(parser.fg, Color::WHITE);
        assert_eq!(parser.bg, Color::BLACK);
        assert_eq!(parser.modifiers, Modifier::empty());
    }

    #[test]
    fn test_ansi_parse_csi_bold() {
        let mut parser = AnsiParser::new();
        // CSI bold: ESC[1m
        parser.parse('\x1b');
        parser.parse('[');
        parser.parse('1');
        parser.parse('m');
        assert_eq!(parser.modifiers, Modifier::BOLD);
    }

    #[test]
    fn test_ansi_parse_csi_dim() {
        let mut parser = AnsiParser::new();
        // CSI dim: ESC[2m
        parser.parse('\x1b');
        parser.parse('[');
        parser.parse('2');
        parser.parse('m');
        assert_eq!(parser.modifiers, Modifier::DIM);
    }

    #[test]
    fn test_ansi_parse_csi_italic() {
        let mut parser = AnsiParser::new();
        // CSI italic: ESC[3m
        parser.parse('\x1b');
        parser.parse('[');
        parser.parse('3');
        parser.parse('m');
        assert_eq!(parser.modifiers, Modifier::ITALIC);
    }

    #[test]
    fn test_ansi_parse_csi_underline() {
        let mut parser = AnsiParser::new();
        // CSI underline: ESC[4m
        parser.parse('\x1b');
        parser.parse('[');
        parser.parse('4');
        parser.parse('m');
        assert_eq!(parser.modifiers, Modifier::UNDERLINE);
    }

    #[test]
    fn test_ansi_parse_csi_crossed_out() {
        let mut parser = AnsiParser::new();
        // CSI crossed out: ESC[9m
        parser.parse('\x1b');
        parser.parse('[');
        parser.parse('9');
        parser.parse('m');
        assert_eq!(parser.modifiers, Modifier::CROSSED_OUT);
    }

    #[test]
    fn test_ansi_parse_csi_red_fg() {
        let mut parser = AnsiParser::new();
        // CSI red fg: ESC[31m
        parser.parse('\x1b');
        parser.parse('[');
        parser.parse('3');
        parser.parse('1');
        parser.parse('m');
        assert_eq!(parser.fg, Color::RED);
    }

    #[test]
    fn test_ansi_parse_csi_green_fg() {
        let mut parser = AnsiParser::new();
        // CSI green fg: ESC[32m
        parser.parse('\x1b');
        parser.parse('[');
        parser.parse('3');
        parser.parse('2');
        parser.parse('m');
        assert_eq!(parser.fg, Color::GREEN);
    }

    #[test]
    fn test_ansi_parse_csi_blue_bg() {
        let mut parser = AnsiParser::new();
        // CSI blue bg: ESC[44m
        parser.parse('\x1b');
        parser.parse('[');
        parser.parse('4');
        parser.parse('4');
        parser.parse('m');
        assert_eq!(parser.bg, Color::BLUE);
    }

    #[test]
    fn test_ansi_parse_csi_rgb_fg() {
        let mut parser = AnsiParser::new();
        // CSI RGB fg: ESC[38;2;255;0;128m
        parser.parse('\x1b');
        parser.parse('[');
        parser.parse('3');
        parser.parse('8');
        parser.parse(';');
        parser.parse('2');
        parser.parse(';');
        for ch in "255;0;128".chars() {
            parser.parse(ch);
        }
        parser.parse('m');
        assert_eq!(parser.fg, Color::rgb(255, 0, 128));
    }

    #[test]
    fn test_ansi_parse_csi_rgb_bg() {
        let mut parser = AnsiParser::new();
        // CSI RGB bg: ESC[48;2;100;150;200m
        parser.parse('\x1b');
        parser.parse('[');
        parser.parse('4');
        parser.parse('8');
        parser.parse(';');
        parser.parse('2');
        parser.parse(';');
        for ch in "100;150;200".chars() {
            parser.parse(ch);
        }
        parser.parse('m');
        assert_eq!(parser.bg, Color::rgb(100, 150, 200));
    }

    #[test]
    fn test_ansi_parse_csi_256_fg() {
        let mut parser = AnsiParser::new();
        // CSI 256 fg: ESC[38;5;123m
        parser.parse('\x1b');
        parser.parse('[');
        parser.parse('3');
        parser.parse('8');
        parser.parse(';');
        parser.parse('5');
        parser.parse(';');
        parser.parse('1');
        parser.parse('2');
        parser.parse('3');
        parser.parse('m');
        // Just verify it doesn't panic
        let _ = parser.fg;
    }

    #[test]
    fn test_ansi_parse_csi_multiple_params() {
        let mut parser = AnsiParser::new();
        // CSI bold red: ESC[1;31m
        parser.parse('\x1b');
        parser.parse('[');
        parser.parse('1');
        parser.parse(';');
        parser.parse('3');
        parser.parse('1');
        parser.parse('m');
        assert_eq!(parser.modifiers, Modifier::BOLD);
        assert_eq!(parser.fg, Color::RED);
    }

    // =========================================================================
    // OSC sequence tests
    // =========================================================================

    #[test]
    fn test_ansi_parse_osc_start() {
        let mut parser = AnsiParser::new();
        parser.parse('\x1b');
        parser.parse(']');
        assert_eq!(parser.state, ParserState::OscStart);
    }

    #[test]
    fn test_ansi_parse_sc_with_bel() {
        let mut parser = AnsiParser::new();
        parser.parse('\x1b');
        parser.parse(']');
        parser.parse('0');
        parser.parse(';');
        for ch in "title".chars() {
            parser.parse(ch);
        }
        parser.parse('\x07'); // BEL
        assert_eq!(parser.state, ParserState::Normal);
    }

    #[test]
    fn test_ansi_parse_osc_with_st() {
        let mut parser = AnsiParser::new();
        parser.parse('\x1b');
        parser.parse(']');
        parser.parse('0');
        parser.parse(';');
        for ch in "title".chars() {
            parser.parse(ch);
        }
        parser.parse('\\'); // ST
        assert_eq!(parser.state, ParserState::Normal);
    }

    // =========================================================================
    // Control character tests
    // =========================================================================

    #[test]
    fn test_ansi_parse_newline() {
        let mut parser = AnsiParser::new();
        let cell = parser.parse('\n');
        // Newline is not a control character we filter
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().ch, '\n');
    }

    #[test]
    fn test_ansi_parse_carriage_return() {
        let mut parser = AnsiParser::new();
        let cell = parser.parse('\r');
        // CR is not a control character we filter
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().ch, '\r');
    }

    #[test]
    fn test_ansi_parse_tab() {
        let mut parser = AnsiParser::new();
        let cell = parser.parse('\t');
        // Tab is not a control character we filter
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().ch, '\t');
    }

    #[test]
    fn test_ansi_parse_null() {
        let mut parser = AnsiParser::new();
        let cell = parser.parse('\x00');
        // NULL is a control character
        assert!(cell.is_none());
    }

    // =========================================================================
    // color_256 function tests
    // =========================================================================

    #[test]
    fn test_color_256_black() {
        assert_eq!(color_256(0), Color::BLACK);
    }

    #[test]
    fn test_color_256_red() {
        assert_eq!(color_256(1), Color::RED);
    }

    #[test]
    fn test_color_256_green() {
        assert_eq!(color_256(2), Color::GREEN);
    }

    #[test]
    fn test_color_256_yellow() {
        assert_eq!(color_256(3), Color::YELLOW);
    }

    #[test]
    fn test_color_256_blue() {
        assert_eq!(color_256(4), Color::BLUE);
    }

    #[test]
    fn test_color_256_magenta() {
        assert_eq!(color_256(5), Color::MAGENTA);
    }

    #[test]
    fn test_color_256_cyan() {
        assert_eq!(color_256(6), Color::CYAN);
    }

    #[test]
    fn test_color_256_white() {
        assert_eq!(color_256(7), Color::WHITE);
    }

    #[test]
    fn test_color_256_range_216_low() {
        // Color 16 should be (0, 0, 0) in 216 color range
        let color = color_256(16);
        assert_eq!(color, Color::rgb(0, 0, 0));
    }

    #[test]
    fn test_color_256_range_216_middle() {
        // Color 123 should be some computed color
        let color = color_256(123);
        // Just verify the color exists (u8 values are always valid 0-255)
        let _ = color;
    }

    #[test]
    fn test_color_256_range_216_high() {
        // Color 231 should be (255, 255, 255) in 216 color range
        let color = color_256(231);
        assert_eq!(color, Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_color_256_grayscale_low() {
        // Color 232 is first grayscale
        let color = color_256(232);
        assert_eq!(color, Color::rgb(8, 8, 8));
    }

    #[test]
    fn test_color_256_grayscale_mid() {
        // Color 244 is middle grayscale
        let color = color_256(244);
        let gray = ((244 - 232) * 10 + 8) as u8;
        assert_eq!(color, Color::rgb(gray, gray, gray));
    }

    #[test]
    fn test_color_256_grayscale_high() {
        // Color 255 is last grayscale
        let color = color_256(255);
        assert_eq!(color, Color::rgb(238, 238, 238));
    }

    #[test]
    fn test_color_256_unknown() {
        // Out of range should return WHITE
        assert_eq!(color_256(999), Color::WHITE);
    }
}
