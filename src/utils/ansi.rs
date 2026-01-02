//! ANSI escape sequence parsing
//!
//! Provides utilities for parsing ANSI escape sequences in terminal output.
//! Useful for RichLog and Terminal widgets.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::ansi::{parse_ansi, AnsiSpan};
//!
//! let text = "\x1b[31mRed\x1b[0m Normal";
//! let spans = parse_ansi(text);
//!
//! for span in spans {
//!     println!("Text: '{}', FG: {:?}", span.text, span.fg);
//! }
//! ```

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

/// Current ANSI parser state
#[derive(Clone, Debug, Default)]
struct AnsiState {
    fg: Option<Color>,
    bg: Option<Color>,
    modifiers: Modifier,
}

impl AnsiState {
    fn reset(&mut self) {
        self.fg = None;
        self.bg = None;
        self.modifiers = Modifier::empty();
    }

    fn to_span(&self, text: String) -> AnsiSpan {
        AnsiSpan {
            text,
            fg: self.fg,
            bg: self.bg,
            modifiers: self.modifiers,
        }
    }
}

/// Parse ANSI escape sequences from text
///
/// Returns a list of styled text spans.
///
/// # Supported sequences
///
/// - SGR (Select Graphic Rendition) codes:
///   - Reset (0)
///   - Bold (1), Dim (2), Italic (3), Underline (4)
///   - Blink (5), Reverse (7), Hidden (8), Strikethrough (9)
///   - Standard colors (30-37, 40-47)
///   - Bright colors (90-97, 100-107)
///   - 256-color (38;5;N, 48;5;N)
///   - True color (38;2;R;G;B, 48;2;R;G;B)
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::ansi::parse_ansi;
///
/// let spans = parse_ansi("\x1b[1;31mBold Red\x1b[0m");
/// assert_eq!(spans[0].text, "Bold Red");
/// assert!(spans[0].modifiers.contains(Modifier::BOLD));
/// ```
pub fn parse_ansi(input: &str) -> Vec<AnsiSpan> {
    let mut spans = Vec::new();
    let mut state = AnsiState::default();
    let mut current_text = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Start of escape sequence
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['

                // Parse CSI sequence
                let mut params = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == ';' {
                        params.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Get the final character
                if let Some(&final_char) = chars.peek() {
                    chars.next();

                    if final_char == 'm' {
                        // SGR sequence
                        if !current_text.is_empty() {
                            spans.push(state.to_span(current_text.clone()));
                            current_text.clear();
                        }

                        apply_sgr(&mut state, &params);
                    }
                    // Ignore other CSI sequences
                }
            } else if chars.peek() == Some(&']') {
                // OSC sequence - skip until ST or BEL
                chars.next(); // consume ']'
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c == '\x07' || (c == '\x1b' && chars.peek() == Some(&'\\')) {
                        if c == '\x1b' {
                            chars.next(); // consume '\'
                        }
                        break;
                    }
                }
            }
            // Skip other escape sequences
        } else {
            current_text.push(ch);
        }
    }

    // Final span
    if !current_text.is_empty() {
        spans.push(state.to_span(current_text));
    }

    spans
}

/// Apply SGR (Select Graphic Rendition) parameters
fn apply_sgr(state: &mut AnsiState, params: &str) {
    if params.is_empty() {
        state.reset();
        return;
    }

    let codes: Vec<u8> = params.split(';').filter_map(|s| s.parse().ok()).collect();

    let mut i = 0;
    while i < codes.len() {
        match codes[i] {
            0 => state.reset(),
            1 => state.modifiers |= Modifier::BOLD,
            2 => state.modifiers |= Modifier::DIM,
            3 => state.modifiers |= Modifier::ITALIC,
            4 => state.modifiers |= Modifier::UNDERLINE,
            5 | 6 => {} // Blink (not supported)
            7 => {}     // Reverse (not supported)
            8 => {}     // Hidden (not supported)
            9 => state.modifiers |= Modifier::CROSSED_OUT,

            22 => state.modifiers.remove(Modifier::BOLD | Modifier::DIM),
            23 => state.modifiers.remove(Modifier::ITALIC),
            24 => state.modifiers.remove(Modifier::UNDERLINE),
            25 => {} // Blink off (not supported)
            27 => {} // Reverse off (not supported)
            28 => {} // Hidden off (not supported)
            29 => state.modifiers.remove(Modifier::CROSSED_OUT),

            // Standard foreground colors
            30 => state.fg = Some(Color::BLACK),
            31 => state.fg = Some(Color::RED),
            32 => state.fg = Some(Color::GREEN),
            33 => state.fg = Some(Color::YELLOW),
            34 => state.fg = Some(Color::BLUE),
            35 => state.fg = Some(Color::MAGENTA),
            36 => state.fg = Some(Color::CYAN),
            37 => state.fg = Some(Color::WHITE),
            39 => state.fg = None, // Default foreground

            // Standard background colors
            40 => state.bg = Some(Color::BLACK),
            41 => state.bg = Some(Color::RED),
            42 => state.bg = Some(Color::GREEN),
            43 => state.bg = Some(Color::YELLOW),
            44 => state.bg = Some(Color::BLUE),
            45 => state.bg = Some(Color::MAGENTA),
            46 => state.bg = Some(Color::CYAN),
            47 => state.bg = Some(Color::WHITE),
            49 => state.bg = None, // Default background

            // Bright foreground colors
            90 => state.fg = Some(Color::rgb(128, 128, 128)), // Bright black (gray)
            91 => state.fg = Some(Color::rgb(255, 85, 85)),   // Bright red
            92 => state.fg = Some(Color::rgb(85, 255, 85)),   // Bright green
            93 => state.fg = Some(Color::rgb(255, 255, 85)),  // Bright yellow
            94 => state.fg = Some(Color::rgb(85, 85, 255)),   // Bright blue
            95 => state.fg = Some(Color::rgb(255, 85, 255)),  // Bright magenta
            96 => state.fg = Some(Color::rgb(85, 255, 255)),  // Bright cyan
            97 => state.fg = Some(Color::rgb(255, 255, 255)), // Bright white

            // Bright background colors
            100 => state.bg = Some(Color::rgb(128, 128, 128)),
            101 => state.bg = Some(Color::rgb(255, 85, 85)),
            102 => state.bg = Some(Color::rgb(85, 255, 85)),
            103 => state.bg = Some(Color::rgb(255, 255, 85)),
            104 => state.bg = Some(Color::rgb(85, 85, 255)),
            105 => state.bg = Some(Color::rgb(255, 85, 255)),
            106 => state.bg = Some(Color::rgb(85, 255, 255)),
            107 => state.bg = Some(Color::rgb(255, 255, 255)),

            // 256-color mode
            38 if i + 2 < codes.len() && codes[i + 1] == 5 => {
                state.fg = Some(color_256(codes[i + 2]));
                i += 2;
            }
            48 if i + 2 < codes.len() && codes[i + 1] == 5 => {
                state.bg = Some(color_256(codes[i + 2]));
                i += 2;
            }

            // True color mode
            38 if i + 4 < codes.len() && codes[i + 1] == 2 => {
                state.fg = Some(Color::rgb(codes[i + 2], codes[i + 3], codes[i + 4]));
                i += 4;
            }
            48 if i + 4 < codes.len() && codes[i + 1] == 2 => {
                state.bg = Some(Color::rgb(codes[i + 2], codes[i + 3], codes[i + 4]));
                i += 4;
            }

            _ => {}
        }
        i += 1;
    }
}

/// Convert 256-color code to RGB
fn color_256(code: u8) -> Color {
    match code {
        // Standard colors (0-7)
        0 => Color::BLACK,
        1 => Color::RED,
        2 => Color::GREEN,
        3 => Color::YELLOW,
        4 => Color::BLUE,
        5 => Color::MAGENTA,
        6 => Color::CYAN,
        7 => Color::WHITE,

        // Bright colors (8-15)
        8 => Color::rgb(128, 128, 128),
        9 => Color::rgb(255, 85, 85),
        10 => Color::rgb(85, 255, 85),
        11 => Color::rgb(255, 255, 85),
        12 => Color::rgb(85, 85, 255),
        13 => Color::rgb(255, 85, 255),
        14 => Color::rgb(85, 255, 255),
        15 => Color::rgb(255, 255, 255),

        // 216 color cube (16-231)
        16..=231 => {
            let n = code - 16;
            let r = (n / 36) % 6;
            let g = (n / 6) % 6;
            let b = n % 6;
            Color::rgb(
                if r > 0 { r * 40 + 55 } else { 0 },
                if g > 0 { g * 40 + 55 } else { 0 },
                if b > 0 { b * 40 + 55 } else { 0 },
            )
        }

        // Grayscale (232-255)
        232..=255 => {
            let gray = (code - 232) * 10 + 8;
            Color::rgb(gray, gray, gray)
        }
    }
}

/// Strip ANSI escape sequences from text
///
/// Returns plain text without any formatting.
pub fn strip_ansi(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Skip escape sequence
            if chars.peek() == Some(&'[') {
                chars.next();
                // Skip until final character
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            } else if chars.peek() == Some(&']') {
                chars.next();
                // OSC sequence - skip until ST or BEL
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c == '\x07' || (c == '\x1b' && chars.peek() == Some(&'\\')) {
                        if c == '\x1b' {
                            chars.next();
                        }
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let spans = parse_ansi("Hello World");
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].text, "Hello World");
        assert!(spans[0].is_default());
    }

    #[test]
    fn test_parse_color() {
        let spans = parse_ansi("\x1b[31mRed\x1b[0m Normal");
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].text, "Red");
        assert_eq!(spans[0].fg, Some(Color::RED));
        assert_eq!(spans[1].text, " Normal");
        assert!(spans[1].is_default());
    }

    #[test]
    fn test_parse_bold() {
        let spans = parse_ansi("\x1b[1mBold\x1b[0m");
        assert_eq!(spans.len(), 1);
        assert!(spans[0].modifiers.contains(Modifier::BOLD));
    }

    #[test]
    fn test_parse_combined() {
        let spans = parse_ansi("\x1b[1;31mBold Red\x1b[0m");
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].fg, Some(Color::RED));
        assert!(spans[0].modifiers.contains(Modifier::BOLD));
    }

    #[test]
    fn test_strip_ansi() {
        let plain = strip_ansi("\x1b[1;31mBold Red\x1b[0m Normal");
        assert_eq!(plain, "Bold Red Normal");
    }

    #[test]
    fn test_ansi_len() {
        let len = ansi_len("\x1b[31mRed\x1b[0m");
        assert_eq!(len, 3);
    }

    #[test]
    fn test_256_color() {
        let spans = parse_ansi("\x1b[38;5;196mRed256\x1b[0m");
        assert_eq!(spans.len(), 1);
        assert!(spans[0].fg.is_some());
    }

    #[test]
    fn test_true_color() {
        let spans = parse_ansi("\x1b[38;2;255;128;64mOrange\x1b[0m");
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].fg, Some(Color::rgb(255, 128, 64)));
    }

    #[test]
    fn test_background() {
        let spans = parse_ansi("\x1b[44mBlue BG\x1b[0m");
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].bg, Some(Color::BLUE));
    }

    #[test]
    fn test_multiple_styles() {
        let spans = parse_ansi("\x1b[31mRed\x1b[32mGreen\x1b[34mBlue\x1b[0m");
        assert_eq!(spans.len(), 3);
        assert_eq!(spans[0].fg, Some(Color::RED));
        assert_eq!(spans[1].fg, Some(Color::GREEN));
        assert_eq!(spans[2].fg, Some(Color::BLUE));
    }

    #[test]
    fn test_codes() {
        use codes::*;
        let text = format!("{}Bold{}", BOLD, RESET);
        assert!(text.contains("\x1b[1m"));
    }
}
