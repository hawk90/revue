//! ANSI escape sequence parser

mod sgr;

use super::types::TermCell;
use crate::render::Modifier;
use crate::style::Color;

/// ANSI parser state
#[derive(Clone, Debug, Default)]
#[doc(hidden)]
pub struct AnsiParser {
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
#[doc(hidden)]
pub enum ParserState {
    #[default]
    Normal,
    Escape,
    Csi,
    OscStart,
    Osc,
}

#[allow(dead_code)]
impl AnsiParser {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self::default()
    }

    #[doc(hidden)]
    pub fn reset_attrs(&mut self) {
        self.fg = Color::WHITE;
        self.bg = Color::BLACK;
        self.modifiers = Modifier::empty();
    }

    #[doc(hidden)]
    pub fn reset_fg(&mut self, color: Color) {
        self.fg = color;
    }

    #[doc(hidden)]
    pub fn reset_bg(&mut self, color: Color) {
        self.bg = color;
    }

    /// Get the current parser state
    #[doc(hidden)]
    pub fn state(&self) -> ParserState {
        self.state.clone()
    }

    /// Get the current foreground color
    #[doc(hidden)]
    pub fn fg(&self) -> Color {
        self.fg
    }

    /// Get the current background color
    #[doc(hidden)]
    pub fn bg(&self) -> Color {
        self.bg
    }

    /// Parse a character and return cell if printable
    #[doc(hidden)]
    pub fn parse(&mut self, ch: char) -> Option<TermCell> {
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
                    let digit = ch.to_digit(10).unwrap_or(0) as u16;
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
}
