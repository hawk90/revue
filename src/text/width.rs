//! Character width calculation

use std::collections::HashMap;
use std::env;

/// Known terminal types with their emoji/unicode rendering characteristics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalType {
    /// iTerm2 - good unicode support, emoji width 2
    ITerm2,
    /// Kitty - excellent unicode support
    Kitty,
    /// Alacritty - good unicode support
    Alacritty,
    /// WezTerm - excellent unicode support
    WezTerm,
    /// Windows Terminal - good unicode support
    WindowsTerminal,
    /// VSCode integrated terminal
    VSCode,
    /// Apple Terminal.app
    AppleTerminal,
    /// Generic xterm or unknown
    Unknown,
}

impl TerminalType {
    /// Detect terminal type from environment variables
    pub fn detect() -> Self {
        // Check specific terminal identifiers
        if env::var("TERM_PROGRAM").as_deref() == Ok("iTerm.app") {
            return Self::ITerm2;
        }
        if env::var("TERM_PROGRAM").as_deref() == Ok("Apple_Terminal") {
            return Self::AppleTerminal;
        }
        if env::var("TERM_PROGRAM").as_deref() == Ok("vscode") {
            return Self::VSCode;
        }
        if env::var("KITTY_WINDOW_ID").is_ok() {
            return Self::Kitty;
        }
        if env::var("ALACRITTY_SOCKET").is_ok() || env::var("ALACRITTY_LOG").is_ok() {
            return Self::Alacritty;
        }
        if env::var("WEZTERM_PANE").is_ok() {
            return Self::WezTerm;
        }
        if env::var("WT_SESSION").is_ok() {
            return Self::WindowsTerminal;
        }

        Self::Unknown
    }

    /// Get default emoji width for this terminal
    pub fn emoji_width(&self) -> u8 {
        match self {
            // Modern terminals typically render emoji as 2 cells
            Self::ITerm2
            | Self::Kitty
            | Self::Alacritty
            | Self::WezTerm
            | Self::WindowsTerminal
            | Self::VSCode => 2,
            // Apple Terminal may have issues with some emoji
            Self::AppleTerminal => 2,
            // Unknown - assume 2 (most common)
            Self::Unknown => 2,
        }
    }

    /// Get default CJK width for this terminal
    pub fn cjk_width(&self) -> u8 {
        // Most terminals handle CJK characters consistently as width 2
        2
    }

    /// Get default nerd font width for this terminal
    pub fn nerd_font_width(&self) -> u8 {
        match self {
            // Terminals with good nerd font support
            Self::Kitty | Self::WezTerm | Self::Alacritty | Self::ITerm2 => 1,
            // Others may render as 2
            _ => 1,
        }
    }
}

/// Character width table with caching
pub struct CharWidthTable {
    /// Width for CJK characters
    pub cjk: u8,
    /// Width for emoji
    pub emoji: u8,
    /// Width for nerd font icons
    pub nerd_font: u8,
    /// Detected terminal type
    pub terminal: TerminalType,
    overrides: HashMap<char, u8>,
}

impl CharWidthTable {
    /// Create a new width table with defaults
    pub fn new() -> Self {
        Self {
            cjk: 2,
            emoji: 2,
            nerd_font: 1,
            terminal: TerminalType::Unknown,
            overrides: HashMap::new(),
        }
    }

    /// Detect character widths from terminal environment
    ///
    /// This detects the terminal type from environment variables and
    /// sets appropriate default widths for emoji, CJK, and nerd fonts.
    ///
    /// Note: This is a heuristic based on known terminal behaviors.
    /// For precise width detection, some terminals support querying
    /// character widths via escape sequences, but this is not universally
    /// supported and may cause issues in some environments.
    pub fn detect() -> Self {
        let terminal = TerminalType::detect();
        Self {
            cjk: terminal.cjk_width(),
            emoji: terminal.emoji_width(),
            nerd_font: terminal.nerd_font_width(),
            terminal,
            overrides: HashMap::new(),
        }
    }

    /// Create for a specific terminal type
    pub fn for_terminal(terminal: TerminalType) -> Self {
        Self {
            cjk: terminal.cjk_width(),
            emoji: terminal.emoji_width(),
            nerd_font: terminal.nerd_font_width(),
            terminal,
            overrides: HashMap::new(),
        }
    }

    /// Get width of a character
    ///
    /// In order: a [`set_override`](Self::set_override) for `ch`; the
    /// [`emoji`](Self::emoji) width for a character shown as emoji by
    /// default; the [`nerd_font`](Self::nerd_font) width for a Private Use
    /// Area character, where Nerd Font icons live; the [`cjk`](Self::cjk)
    /// width for a wide (East Asian) character; otherwise the Unicode width.
    pub fn width(&self, ch: char) -> u8 {
        if let Some(&w) = self.overrides.get(&ch) {
            return w;
        }

        // Emoji shown as emoji by default. Characters like '1', '#' or '©'
        // have the Emoji property too but are drawn as text.
        if unic_emoji_char::is_emoji_presentation(ch) {
            return self.emoji;
        }

        if is_private_use(ch) {
            return self.nerd_font;
        }

        // Use unicode-width as base
        let base = unicode_width::UnicodeWidthChar::width(ch)
            .map(|w| w as u8)
            .unwrap_or(1);
        if base == 2 {
            self.cjk
        } else {
            base
        }
    }

    /// Override width for specific character
    pub fn set_override(&mut self, ch: char, width: u8) {
        self.overrides.insert(ch, width);
    }

    /// Set CJK width
    pub fn with_cjk(mut self, width: u8) -> Self {
        self.cjk = width;
        self
    }

    /// Set emoji width
    pub fn with_emoji(mut self, width: u8) -> Self {
        self.emoji = width;
        self
    }

    /// Set nerd font width
    pub fn with_nerd_font(mut self, width: u8) -> Self {
        self.nerd_font = width;
        self
    }
}

/// Whether `ch` is in a Private Use Area, where Nerd Fonts put their icons
fn is_private_use(ch: char) -> bool {
    matches!(
        ch as u32,
        0xE000..=0xF8FF | 0xF_0000..=0xF_FFFD | 0x10_0000..=0x10_FFFD
    )
}

impl Default for CharWidthTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Get character width using default table
///
/// This delegates to `utils::unicode::char_width` for consistency across the codebase.
/// The return type is `u8` for compatibility with terminal cell operations.
pub fn char_width(ch: char) -> u8 {
    crate::utils::unicode::char_width(ch) as u8
}

/// Get string width
#[cfg(test)] // Used in tests only
pub fn str_width(s: &str) -> usize {
    unicode_width::UnicodeWidthStr::width(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_width() {
        assert_eq!(char_width('a'), 1);
        assert_eq!(char_width('Z'), 1);
        assert_eq!(char_width(' '), 1);
        assert_eq!(char_width('!'), 1);
    }

    #[test]
    fn test_cjk_width() {
        assert_eq!(char_width('한'), 2);
        assert_eq!(char_width('글'), 2);
        assert_eq!(char_width('日'), 2);
        assert_eq!(char_width('本'), 2);
        assert_eq!(char_width('中'), 2);
    }

    #[test]
    fn test_str_width() {
        assert_eq!(str_width("hello"), 5);
        assert_eq!(str_width("한글"), 4);
        assert_eq!(str_width("hello한글"), 9);
    }

    #[test]
    fn test_width_table_override() {
        let mut table = CharWidthTable::new();
        table.set_override('X', 3);

        assert_eq!(table.width('X'), 3);
        assert_eq!(table.width('Y'), 1); // Default
    }

    #[test]
    fn test_emoji_width() {
        let table = CharWidthTable::new();

        // Basic emoji
        assert_eq!(table.width('😀'), 2);
        assert_eq!(table.width('🔥'), 2);
    }

    #[test]
    fn test_width_table_builder() {
        let table = CharWidthTable::new()
            .with_cjk(2)
            .with_emoji(2)
            .with_nerd_font(1);

        assert_eq!(table.cjk, 2);
        assert_eq!(table.emoji, 2);
        assert_eq!(table.nerd_font, 1);
    }
}
