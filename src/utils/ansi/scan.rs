//! Splitting text into characters and control sequences the way a terminal
//! reads it (ECMA-48), so a sequence never shows up as text, whatever its
//! parameters
//!
//! - `ESC [` starts a control sequence (CSI): parameter bytes `0x30-0x3F`
//!   (digits, `;`, `:`, and private markers such as `?`), then intermediate
//!   bytes `0x20-0x2F`, ended by a final byte `0x40-0x7E`.
//! - `ESC ]` starts an operating system command (OSC), ended by BEL or by
//!   ST (`ESC \`); `ESC P`, `ESC X`, `ESC ^` and `ESC _` start strings ended
//!   by ST. Their payload is skipped.
//! - Any other `ESC` is followed by intermediate bytes and a final byte
//!   `0x30-0x7E`: `ESC ( B`, `ESC 7`.
//!
//! Inside a sequence a C0 control still acts (it is passed on as a
//! character), `ESC` starts a new sequence, and CAN or SUB cancels it. A
//! character that has no place in the sequence ends it and is shown.

use std::mem::take;

const BEL: char = '\x07';
const ESC: char = '\x1b';
const CAN: char = '\x18';
const SUB: char = '\x1a';

/// The parameter or intermediate bytes kept for one control sequence; a
/// longer one is read to its end and dropped, so a stream cannot grow the
/// scanner
const MAX_PARAMS: usize = 256;

/// A control sequence: `ESC [`, its parameter and intermediate bytes, and
/// the final byte that names it
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Csi {
    pub(crate) params: String,
    pub(crate) intermediates: String,
    pub(crate) final_byte: char,
}

impl Csi {
    /// Whether this sets colors and modifiers (SGR, `ESC [ ... m`): no
    /// private marker or intermediate byte
    pub(crate) fn is_sgr(&self) -> bool {
        self.final_byte == 'm'
            && self.intermediates.is_empty()
            && self
                .params
                .bytes()
                .all(|b| b.is_ascii_digit() || b == b';' || b == b':')
    }
}

/// What the scanner passes on
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Token {
    /// A character to show, or a C0 control to act on (never `ESC`)
    Char(char),
    /// A complete control sequence
    Csi(Csi),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum State {
    #[default]
    Ground,
    /// After `ESC`
    Escape,
    /// After `ESC` and an intermediate byte
    EscIntermediate,
    /// Inside `ESC [`
    Csi,
    /// Inside a control sequence that is read to its end and dropped
    CsiIgnore,
    /// Inside an OSC (`bel_ends`) or another string ended by ST
    Str { bel_ends: bool },
    /// After `ESC` inside a string: `\` ends it
    StrEsc,
}

/// Reads text one character at a time and passes on what is not part of a
/// sequence, and each complete control sequence. Keeps its place between
/// calls, so a sequence can be split across writes.
#[derive(Clone, Debug, Default)]
pub(crate) struct Scanner {
    state: State,
    csi: Csi,
}

impl Scanner {
    /// Read `ch`
    pub(crate) fn feed(&mut self, ch: char) -> Option<Token> {
        match self.state {
            State::Ground => {
                if ch == ESC {
                    self.state = State::Escape;
                    None
                } else {
                    Some(Token::Char(ch))
                }
            }
            State::Escape => match ch {
                '[' => {
                    self.csi = Csi::default();
                    self.state = State::Csi;
                    None
                }
                ']' => self.go(State::Str { bel_ends: true }),
                'P' | 'X' | '^' | '_' => self.go(State::Str { bel_ends: false }),
                '\x20'..='\x2f' => self.go(State::EscIntermediate),
                '\x30'..='\x7e' => self.go(State::Ground),
                _ => self.stray(ch),
            },
            State::EscIntermediate => match ch {
                '\x20'..='\x2f' => None,
                '\x30'..='\x7e' => self.go(State::Ground),
                _ => self.stray(ch),
            },
            State::Csi => match ch {
                '\x30'..='\x3f'
                    if self.csi.intermediates.is_empty() && self.csi.params.len() < MAX_PARAMS =>
                {
                    self.csi.params.push(ch);
                    None
                }
                // A parameter after an intermediate byte, or too many
                '\x30'..='\x3f' => self.go(State::CsiIgnore),
                '\x20'..='\x2f' if self.csi.intermediates.len() < MAX_PARAMS => {
                    self.csi.intermediates.push(ch);
                    None
                }
                '\x20'..='\x2f' => self.go(State::CsiIgnore),
                '\x40'..='\x7e' => {
                    self.state = State::Ground;
                    self.csi.final_byte = ch;
                    Some(Token::Csi(take(&mut self.csi)))
                }
                _ => self.stray(ch),
            },
            State::CsiIgnore => match ch {
                '\x20'..='\x3f' => None,
                '\x40'..='\x7e' => self.go(State::Ground),
                _ => self.stray(ch),
            },
            State::Str { bel_ends } => match ch {
                BEL if bel_ends => self.go(State::Ground),
                ESC => self.go(State::StrEsc),
                CAN | SUB => self.go(State::Ground),
                _ => None,
            },
            State::StrEsc => match ch {
                '\\' => self.go(State::Ground),
                // An `ESC` that is not ST starts a new sequence
                _ => {
                    self.state = State::Escape;
                    self.feed(ch)
                }
            },
        }
    }

    fn go(&mut self, state: State) -> Option<Token> {
        self.state = state;
        None
    }

    /// A character with no place in the sequence being read
    fn stray(&mut self, ch: char) -> Option<Token> {
        match ch {
            ESC => self.go(State::Escape),
            CAN | SUB => self.go(State::Ground),
            // A C0 control acts as it would outside the sequence
            c if c < '\x20' => Some(Token::Char(c)),
            '\x7f' => None,
            // Anything else ends the sequence and is shown
            c => {
                self.state = State::Ground;
                Some(Token::Char(c))
            }
        }
    }
}

/// The tokens of `input`, read from the start
pub(crate) fn tokens(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut scanner = Scanner::default();
    input.chars().filter_map(move |ch| scanner.feed(ch))
}
