//! Reading the written text: the shared ANSI scanner and SGR codes turn it
//! into cells

use super::types::TermCell;
use crate::style::Color;
use crate::utils::ansi::{Csi, Scanner, SgrState, Token};

/// What one written character comes to
#[derive(Clone, Debug)]
pub(super) enum Read {
    /// A cell to show
    Cell(TermCell),
    /// A control the screen acts on: newline, carriage return, tab or
    /// backspace
    Control(char),
    /// A line-oriented control sequence (#1423)
    Edit(Edit),
}

/// The control sequences acted on: the ones command output uses on its own
/// line - erase in line, cursor moves within the screen, and a column. A
/// count of 0 or none is 1. Whole-screen ones (erase display, absolute
/// position, scroll regions) are not: the lines are a scrollback, not a
/// screen grid (#1461).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Edit {
    /// `ESC [ K` / `0K`: from the cursor to the end of the line
    EraseToEnd,
    /// `ESC [ 1K`: from the start of the line to the cursor, inclusive
    EraseToCursor,
    /// `ESC [ 2K`: the whole line
    EraseLine,
    /// `ESC [ n A`
    Up(usize),
    /// `ESC [ n B`
    Down(usize),
    /// `ESC [ n C`
    Right(usize),
    /// `ESC [ n D`
    Left(usize),
    /// `ESC [ n G`: column `n`, counted from 1
    Column(usize),
}

impl Edit {
    /// The edit `csi` asks for, if it is one of them: plain parameters
    /// only, no private marker or intermediate byte
    fn from_csi(csi: &Csi) -> Option<Self> {
        if !csi.intermediates.is_empty() || !csi.params.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        // A number too long to keep is as far as the screen goes
        let n: usize = if csi.params.is_empty() {
            0
        } else {
            csi.params.parse().unwrap_or(usize::MAX)
        };
        let count = n.max(1);
        Some(match csi.final_byte {
            'K' => match n {
                0 => Edit::EraseToEnd,
                1 => Edit::EraseToCursor,
                2 => Edit::EraseLine,
                _ => return None,
            },
            'A' => Edit::Up(count),
            'B' => Edit::Down(count),
            'C' => Edit::Right(count),
            'D' => Edit::Left(count),
            'G' => Edit::Column(count),
            _ => return None,
        })
    }
}

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

    /// Read a character: the cell to show for it, a control or edit to act
    /// on, or `None` when it is part of an escape sequence or does nothing.
    /// A default color is `Color::default()` in the cell, drawn in the
    /// terminal's default.
    pub(super) fn parse(&mut self, ch: char) -> Option<Read> {
        match self.scanner.feed(ch)? {
            Token::Char(ch @ ('\n' | '\r' | '\t' | '\x08')) => Some(Read::Control(ch)),
            Token::Char(ch) if ch.is_control() => None,
            Token::Char(ch) => Some(Read::Cell(self.cell(ch))),
            // SGR sets the pen; the line-oriented sequences edit; modes and
            // the rest are dropped
            Token::Csi(csi) => {
                if csi.is_sgr() {
                    self.sgr.apply(&csi.params);
                    None
                } else {
                    Edit::from_csi(&csi).map(Read::Edit)
                }
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
