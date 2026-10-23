//! ANSI escape sequence parsing
//!
//! Provides utilities for parsing ANSI escape sequences in terminal output.
//! Useful for RichLog and Terminal widgets.
//!
//! # Example
//!
//! ```
//! use revue::utils::ansi::{parse_ansi, AnsiSpan};
//!
//! let text = "\x1b[31mRed\x1b[0m Normal";
//! let spans = parse_ansi(text);
//!
//! for span in spans {
//!     println!("Text: '{}', FG: {:?}", span.text, span.fg);
//! }
//! ```

mod scan;
mod sgr;

pub(crate) use scan::{Scanner, Token};
pub(crate) use sgr::SgrState;

use crate::render::Modifier;
use crate::style::Color;

/// An ANSI-styled text span
#[derive(Clone, Debug, PartialEq)]
pub struct AnsiSpan {
    /// The text content
    pub text: String,
    /// Foreground color
    pub fg: Option<Color>,
    /// Background color
    pub bg: Option<Color>,
    /// Text modifiers (bold, italic, etc.)
    pub modifiers: Modifier,
}

impl Default for AnsiSpan {
    fn default() -> Self {
        Self {
            text: String::new(),
            fg: None,
            bg: None,
            modifiers: Modifier::empty(),
        }
    }
}

impl AnsiSpan {
    /// Create a new ANSI span
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    /// Check if span has default styling
    pub fn is_default(&self) -> bool {
        self.fg.is_none() && self.bg.is_none() && self.modifiers.is_empty()
    }
}

/// Parse ANSI escape sequences from text
///
/// Returns a list of styled text spans.
///
/// # Supported sequences
///
/// - SGR (Select Graphic Rendition) codes:
///   - Reset (0), and default colors (39, 49): `None`
///   - Bold (1), Dim (2), Italic (3), Underline (4, 21), Reverse (7),
///     Strikethrough (9), and their resets (22, 23, 24, 27, 29)
///   - Standard colors (30-37, 40-47)
///   - Bright colors (90-97, 100-107)
///   - 256-color (38;5;N, 48;5;N)
///   - True color (38;2;R;G;B, 48;2;R;G;B), and the colon forms
///     (38:5:N, 38:2:R:G:B, 38:2::R:G:B)
///
///   An index or a component past 255 sets nothing; blink and hidden are
///   ignored.
/// - Every other escape sequence is removed whole: other control sequences
///   (`ESC[?25l`, `ESC[2 q`), `ESC(B`, and OSC strings ended by BEL or
///   `ESC\`.
///
/// # Example
///
/// ```
/// use revue::render::Modifier;
/// use revue::utils::ansi::parse_ansi;
///
/// let spans = parse_ansi("\x1b[1;31mBold Red\x1b[0m");
/// assert_eq!(spans[0].text, "Bold Red");
/// assert!(spans[0].modifiers.contains(Modifier::BOLD));
/// ```
pub fn parse_ansi(input: &str) -> Vec<AnsiSpan> {
    let mut spans = Vec::new();
    let mut state = SgrState::default();
    let mut text = String::new();
    let span = |text: String, state: &SgrState| AnsiSpan {
        text,
        fg: state.fg,
        bg: state.bg,
        modifiers: state.modifiers,
    };

    for token in scan::tokens(input) {
        match token {
            Token::Char(ch) => text.push(ch),
            // Other control sequences (cursor moves, modes) are dropped
            Token::Csi(csi) if csi.is_sgr() => {
                if !text.is_empty() {
                    spans.push(span(std::mem::take(&mut text), &state));
                }
                state.apply(&csi.params);
            }
            Token::Csi(_) => {}
        }
    }
    if !text.is_empty() {
        spans.push(span(text, &state));
    }
    spans
}

/// Strip ANSI escape sequences from text
///
/// Returns plain text without any formatting.
pub fn strip_ansi(input: &str) -> String {
    scan::tokens(input)
        .filter_map(|token| match token {
            Token::Char(ch) => Some(ch),
            Token::Csi(_) => None,
        })
        .collect()
}

/// Calculate the display length of ANSI text (excluding escape sequences)
pub fn ansi_len(input: &str) -> usize {
    strip_ansi(input).len()
}

/// ANSI color codes for generating styled text
pub mod codes {
    /// Reset all attributes
    pub const RESET: &str = "\x1b[0m";

    /// Bold text
    pub const BOLD: &str = "\x1b[1m";
    /// Dim text
    pub const DIM: &str = "\x1b[2m";
    /// Italic text
    pub const ITALIC: &str = "\x1b[3m";
    /// Underlined text
    pub const UNDERLINE: &str = "\x1b[4m";

    // Foreground colors
    /// Black foreground
    pub const FG_BLACK: &str = "\x1b[30m";
    /// Red foreground
    pub const FG_RED: &str = "\x1b[31m";
    /// Green foreground
    pub const FG_GREEN: &str = "\x1b[32m";
    /// Yellow foreground
    pub const FG_YELLOW: &str = "\x1b[33m";
    /// Blue foreground
    pub const FG_BLUE: &str = "\x1b[34m";
    /// Magenta foreground
    pub const FG_MAGENTA: &str = "\x1b[35m";
    /// Cyan foreground
    pub const FG_CYAN: &str = "\x1b[36m";
    /// White foreground
    pub const FG_WHITE: &str = "\x1b[37m";

    // Background colors
    /// Black background
    pub const BG_BLACK: &str = "\x1b[40m";
    /// Red background
    pub const BG_RED: &str = "\x1b[41m";
    /// Green background
    pub const BG_GREEN: &str = "\x1b[42m";
    /// Yellow background
    pub const BG_YELLOW: &str = "\x1b[43m";
    /// Blue background
    pub const BG_BLUE: &str = "\x1b[44m";
    /// Magenta background
    pub const BG_MAGENTA: &str = "\x1b[45m";
    /// Cyan background
    pub const BG_CYAN: &str = "\x1b[46m";
    /// White background
    pub const BG_WHITE: &str = "\x1b[47m";

    /// Generate 256-color foreground code
    pub fn fg_256(code: u8) -> String {
        format!("\x1b[38;5;{}m", code)
    }

    /// Generate 256-color background code
    pub fn bg_256(code: u8) -> String {
        format!("\x1b[48;5;{}m", code)
    }

    /// Generate true color (24-bit) foreground code
    pub fn fg_rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    }

    /// Generate true color (24-bit) background code
    pub fn bg_rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[48;2;{};{};{}m", r, g, b)
    }
}
