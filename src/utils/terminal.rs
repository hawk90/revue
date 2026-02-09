//! Terminal detection utilities
//!
//! Provides centralized detection for terminal capabilities and types.
//! Prevents duplication of terminal detection logic across the codebase.

use std::sync::OnceLock;

/// Terminal type detection result
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TerminalType {
    /// Kitty terminal - supports advanced graphics protocols
    Kitty,
    /// iTerm2 terminal - supports inline images
    Iterm2,
    /// Unknown or standard terminal
    #[default]
    Unknown,
}

/// Cached terminal type detection
static TERMINAL_TYPE: OnceLock<TerminalType> = OnceLock::new();

/// Get the detected terminal type
///
/// Uses cached detection for performance. Detection checks for:
/// - KITTY_WINDOW_ID or KITTY_PID environment variables (Kitty terminal)
/// - TERM_PROGRAM environment variable (iTerm2, Kitty, WezTerm, etc.)
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::terminal::terminal_type;
///
/// match terminal_type() {
///     TerminalType::Kitty => println!("Running in Kitty"),
///     TerminalType::Iterm2 => println!("Running in iTerm2"),
///     TerminalType::Unknown => println!("Unknown terminal"),
/// }
/// ```
pub fn terminal_type() -> TerminalType {
    *TERMINAL_TYPE.get_or_init(|| {
        // Check for Kitty terminal first
        if is_kitty_terminal() {
            return TerminalType::Kitty;
        }

        // Check for iTerm2
        if is_iterm2_terminal() {
            return TerminalType::Iterm2;
        }

        TerminalType::Unknown
    })
}

/// Check if running in Kitty terminal
///
/// Checks for:
/// - KITTY_WINDOW_ID environment variable (primary)
/// - KITTY_PID environment variable (backup)
/// - TERM_PROGRAM environment variable set to "kitty"
fn is_kitty_terminal() -> bool {
    std::env::var("KITTY_WINDOW_ID").is_ok()
        || std::env::var("KITTY_PID").is_ok()
        || std::env::var("TERM_PROGRAM")
            .map(|v| v.to_lowercase() == "kitty")
            .unwrap_or(false)
}

/// Check if running in iTerm2 terminal
///
/// Checks for:
/// - TERM_PROGRAM environment variable set to "iTerm.app"
/// - LC_TERMINAL environment variable set to "iTerm2"
fn is_iterm2_terminal() -> bool {
    std::env::var("TERM_PROGRAM")
        .map(|v| v.to_lowercase() == "iterm.app")
        .unwrap_or(false)
        || std::env::var("LC_TERMINAL")
            .map(|v| v.to_lowercase() == "iterm2")
            .unwrap_or(false)
}

/// Check if terminal supports Sixel graphics
///
/// Checks TERM environment variable for known Sixel-capable terminals.
pub fn is_sixel_capable() -> bool {
    is_sixel_capable_auto()
}

/// Auto-detect Sixel capability without checking user override
fn is_sixel_capable_auto() -> bool {
    // Check for known Sixel-capable terminals
    if let Ok(term) = std::env::var("TERM") {
        let term_lower = term.to_lowercase();
        // mlterm, xterm with Sixel, foot, contour, etc.
        term_lower.contains("mlterm")
            || term_lower.contains("yaft")
            || term_lower.contains("foot")
            || term_lower.contains("contour")
            || term_lower.contains("sixel")
            || term_lower.contains("vt340")
            || term_lower.contains("vt384")
    } else {
        false
    }
}
