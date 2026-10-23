//! Reading the written text: the shared ANSI scanner and SGR codes turn it
//! into cells

use super::types::TermCell;
use crate::style::Color;
use crate::utils::ansi::{Scanner, SgrState, Token};

/// The escape sequences read so far and the colors and modifiers they set
#[derive(Clone, Debug, Default)]
pub(super) struct AnsiParser {
    scanner: Scanner,
    sgr: SgrState,
}

impl AnsiParser {
    pub(super) fn new() -> Self {
        Self::default()
    }

    /// The colors and modifiers in effect: a color of `None` is the
    /// terminal's default one
    #[cfg(test)]
    pub(super) fn sgr(&self) -> SgrState {
        self.sgr
    }

    /// Read a character: the cell to show for it, or `None` when it is part
    /// of an escape sequence or a control character. A default color is
    /// `Color::default()` in the cell, drawn in the terminal's default.
    pub(super) fn parse(&mut self, ch: char) -> Option<TermCell> {
        match self.scanner.feed(ch)? {
            Token::Char(ch) if ch.is_control() && !matches!(ch, '\n' | '\r' | '\t') => None,
            Token::Char(ch) => Some(self.cell(ch)),
            // Only SGR is acted on; cursor moves and modes are dropped
            Token::Csi(csi) => {
                if csi.is_sgr() {
                    self.sgr.apply(&csi.params);
                }
                None
            }
        }
    }
}

impl AnsiParser {
    /// `ch` in the colors and modifiers in effect
    pub(super) fn cell(&self, ch: char) -> TermCell {
        TermCell {
            ch,
            fg: self.sgr.fg.unwrap_or_default(),
            bg: self.sgr.bg.unwrap_or_default(),
            modifiers: self.sgr.modifiers,
        }
    }
}

/// A cell no code has colored: a space in the default colors
pub(super) fn blank() -> TermCell {
    TermCell {
        ch: ' ',
        fg: Color::default(),
        bg: Color::default(),
        modifiers: crate::render::Modifier::empty(),
    }
}
