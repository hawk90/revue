//! Kitty Text Sizing Protocol (OSC 66) support
//!
//! This module provides support for Kitty's Text Sizing Protocol which allows
//! rendering text at different sizes in the terminal. This is primarily used
//! for rendering markdown headers with actual size differentiation (H1 > H2 > H3...).
//!
//! # Protocol Reference
//!
//! The escape sequence format is:
//! ```text
//! \x1b]66;s=2:n={n}:d={d}:w={n};{text}\x1b\\
//! ```
//!
//! Where:
//! - `s=2` - Scaling mode
//! - `n` - Numerator (size factor)
//! - `d` - Denominator (text wrapping divisor)
//! - `w` - Width parameter
//!
//! # Example
//!
//! ```rust
//! use revue::utils::text_sizing::{TextSizing, is_supported};
//!
//! if is_supported() {
//!     let seq = TextSizing::escape_sequence("Hello World", 1, 80);
//!     // seq contains the OSC 66 escape sequence for H1-sized text
//! }
//! ```
//!
//! # Compatibility
//!
//! | Terminal | Support |
//! |----------|---------|
//! | Kitty ≥ 0.40 | Yes |
//! | Ghostty | Unknown |
//! | WezTerm | No |
//! | iTerm2 | No |
//! | Other | No |

use std::env;
use std::fmt::Write;
use std::sync::OnceLock;

use crate::utils::terminal::{terminal_type, TerminalType};

/// Cached result of text sizing support detection
static TEXT_SIZING_SUPPORTED: OnceLock<bool> = OnceLock::new();

/// Check if the current terminal supports the Text Sizing Protocol
///
/// This function caches its result for performance. The detection is based on:
/// 1. Centralized terminal detection for Kitty
/// 2. GHOSTTY_RESOURCES_DIR environment variable (Ghostty check)
///
/// Note: This is a heuristic. Kitty versions < 0.40 don't support OSC 66,
/// but there's no reliable way to query the version without side effects.
/// In practice, most Kitty users keep their terminal updated.
pub fn is_supported() -> bool {
    *TEXT_SIZING_SUPPORTED.get_or_init(|| {
        // Use centralized terminal detection for Kitty
        if terminal_type() == TerminalType::Kitty {
            return true;
        }

        // Check for Ghostty (may support it in the future)
        if env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
            // Ghostty might support text sizing in future versions
            // For now, return false until confirmed
            return false;
        }

        false
    })
}

/// Text Sizing Protocol utilities
pub struct TextSizing;

impl TextSizing {
    /// Get the size ratio for a heading tier
    ///
    /// Returns (numerator, denominator) tuple representing the size factor.
    /// These values are based on mdfried's implementation:
    ///
    /// | Tier | Ratio | Percentage |
    /// |------|-------|------------|
    /// | H1 | 7/7 | 100% |
    /// | H2 | 5/6 | ~83% |
    /// | H3 | 3/4 | 75% |
    /// | H4 | 2/3 | ~67% |
    /// | H5 | 3/5 | 60% |
    /// | H6 | 1/3 | ~33% |
    #[inline]
    pub fn size_ratio(tier: u8) -> (u8, u8) {
        match tier {
            1 => (7, 7), // H1: largest (100%)
            2 => (5, 6), // H2: 83%
            3 => (3, 4), // H3: 75%
            4 => (2, 3), // H4: 67%
            5 => (3, 5), // H5: 60%
            _ => (1, 3), // H6+: smallest (33%)
        }
    }

    /// Calculate the scaled width for text wrapping
    ///
    /// When rendering scaled text, the effective width needs to be adjusted
    /// based on the size ratio to properly wrap text.
    pub fn scaled_width(width: u16, tier: u8) -> u16 {
        let (n, d) = Self::size_ratio(tier);
        // Formula: width / 2 * d / n (from mdfried)
        // Use saturating arithmetic to prevent overflow for large width values
        // Width is first halved, then multiplied by d, then divided by n
        let half_width = width / 2;
        half_width.saturating_mul(u16::from(d)) / u16::from(n)
    }

    /// Generate the OSC 66 escape sequence for scaled text
    ///
    /// This generates the complete escape sequence needed to render text
    /// at the specified heading tier size.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to render
    /// * `tier` - Heading level (1-6, where 1 is largest)
    /// * `width` - Terminal width in columns
    ///
    /// # Returns
    ///
    /// A string containing the complete escape sequence including:
    /// - Line erase sequences to clear the rendering area
    /// - OSC 66 sequences for each text chunk
    pub fn escape_sequence(text: &str, tier: u8, width: u16) -> String {
        let (n, d) = Self::size_ratio(tier);

        let chars: Vec<char> = text.chars().collect();
        let chunk_count = chars.len().div_ceil(d as usize);

        // Pre-allocate with estimated capacity
        let width_digits = width.checked_ilog10().unwrap_or(0) as usize + 1;
        let capacity = 19 + 2 * width_digits + text.len() + chunk_count * 24;
        let mut result = String::with_capacity(capacity);

        // Erase-character dance (clears the rendering area)
        // ECH (Erase Character) + disable auto-wrap
        let _ = write!(result, "\x1b[{}X\x1b[?7l", width);
        // Move down 1 line
        let _ = write!(result, "\x1b[1B");
        // ECH + disable auto-wrap on second line
        let _ = write!(result, "\x1b[{}X\x1b[?7l", width);
        // Move back up 1 line
        let _ = write!(result, "\x1b[1A");

        // Generate OSC 66 sequences for each chunk
        for chunk in chars.chunks(d as usize) {
            // OSC 66 sequence: \x1b]66;s=2:n={n}:d={d}:w={n};{text}\x1b\\
            let _ = write!(result, "\x1b]66;s=2:n={n}:d={d}:w={n};");
            result.extend(chunk);
            // String Terminator
            let _ = write!(result, "\x1b\\");
        }

        result
    }

    /// Get the height in terminal rows for a text-sized heading
    ///
    /// All text-sized headings occupy exactly 2 rows in the terminal.
    #[inline]
    pub const fn height() -> u16 {
        2
    }
}
