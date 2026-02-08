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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_ratios() {
        assert_eq!(TextSizing::size_ratio(1), (7, 7));
        assert_eq!(TextSizing::size_ratio(2), (5, 6));
        assert_eq!(TextSizing::size_ratio(3), (3, 4));
        assert_eq!(TextSizing::size_ratio(4), (2, 3));
        assert_eq!(TextSizing::size_ratio(5), (3, 5));
        assert_eq!(TextSizing::size_ratio(6), (1, 3));
        // Beyond H6 should use H6 ratio
        assert_eq!(TextSizing::size_ratio(7), (1, 3));
    }

    #[test]
    fn test_scaled_width() {
        // H1: width / 2 * 7 / 7 = width / 2
        assert_eq!(TextSizing::scaled_width(80, 1), 40);

        // H2: width / 2 * 6 / 5 = 48
        assert_eq!(TextSizing::scaled_width(80, 2), 48);

        // H3: width / 2 * 4 / 3 = 53
        assert_eq!(TextSizing::scaled_width(80, 3), 53);
    }

    #[test]
    fn test_escape_sequence_structure() {
        let seq = TextSizing::escape_sequence("Hello", 1, 80);

        // Should contain OSC 66 marker
        assert!(seq.contains("\x1b]66;"));
        // Should contain string terminator
        assert!(seq.contains("\x1b\\"));
        // Should contain the text
        assert!(seq.contains("Hello"));
        // Should contain size parameters for H1
        assert!(seq.contains("n=7"));
        assert!(seq.contains("d=7"));
    }

    #[test]
    fn test_escape_sequence_chunks() {
        // With d=7 (H1), text of 10 chars should be split into 2 chunks
        let seq = TextSizing::escape_sequence("1234567890", 1, 80);

        // Count OSC 66 sequences (one per chunk)
        let osc_count = seq.matches("\x1b]66;").count();
        assert_eq!(osc_count, 2);
    }

    #[test]
    fn test_height() {
        assert_eq!(TextSizing::height(), 2);
    }

    #[test]
    fn test_is_supported_caching() {
        // Multiple calls should return the same value (cached)
        let first = is_supported();
        let second = is_supported();
        assert_eq!(first, second);
    }

    // =========================================================================
    // Additional text sizing tests
    // =========================================================================

    #[test]
    fn test_size_ratio_all_tiers() {
        // Test all heading tiers return valid ratios
        for tier in 1..=6 {
            let (n, d) = TextSizing::size_ratio(tier);
            assert!(n > 0, "Numerator should be positive");
            assert!(d > 0, "Denominator should be positive");
            assert!(n <= d, "Numerator should not exceed denominator");
        }
    }

    #[test]
    fn test_size_ratio_tier_zero() {
        // Tier 0 should use smallest ratio (H6+)
        let (n, d) = TextSizing::size_ratio(0);
        assert_eq!((n, d), (1, 3));
    }

    #[test]
    fn test_size_ratio_large_tier() {
        // Tier 100 should use smallest ratio
        let (n, d) = TextSizing::size_ratio(100);
        assert_eq!((n, d), (1, 3));
    }

    #[test]
    fn test_scaled_width_all_tiers() {
        // Test scaled width for all tiers
        let width = 100;
        for tier in 1..=6 {
            let scaled = TextSizing::scaled_width(width, tier);
            assert!(scaled > 0, "Scaled width should be positive");
        }
    }

    #[test]
    fn test_scaled_width_zero_width() {
        // Zero width should return zero
        assert_eq!(TextSizing::scaled_width(0, 1), 0);
        assert_eq!(TextSizing::scaled_width(0, 3), 0);
    }

    #[test]
    fn test_scaled_width_one() {
        // Width of 1 should produce reasonable results
        let w1 = TextSizing::scaled_width(1, 1);
        assert!(w1 <= 1);
    }

    #[test]
    fn test_scaled_width_large_width() {
        // Large width should be handled without overflow
        let large = u16::MAX;
        let scaled = TextSizing::scaled_width(large, 1);
        assert!(scaled > 0);
    }

    #[test]
    fn test_escape_sequence_empty_text() {
        let seq = TextSizing::escape_sequence("", 1, 80);
        // Empty text produces only the prefix/suffix code, no OSC 66 sequences
        // Should still have the erase character dance
        assert!(seq.contains("\x1b["));
        assert!(seq.contains("X"));
    }

    #[test]
    fn test_escape_sequence_unicode() {
        let seq = TextSizing::escape_sequence("你好世界", 1, 80);
        assert!(seq.contains("你好世界"));
    }

    #[test]
    fn test_escape_sequence_special_chars() {
        let seq = TextSizing::escape_sequence("Test\n\t\r", 1, 80);
        assert!(seq.contains("Test"));
    }

    #[test]
    fn test_escape_sequence_different_tiers() {
        // Test that different tiers produce different escape sequences
        let seq1 = TextSizing::escape_sequence("Test", 1, 80);
        let seq2 = TextSizing::escape_sequence("Test", 2, 80);
        let seq3 = TextSizing::escape_sequence("Test", 3, 80);

        // Each tier should have different size parameters
        assert!(seq1.contains("n=7")); // H1: (7, 7)
        assert!(seq2.contains("n=5")); // H2: (5, 6)
        assert!(seq3.contains("n=3")); // H3: (3, 4)
    }

    #[test]
    fn test_escape_sequence_narrow_width() {
        let seq = TextSizing::escape_sequence("A", 1, 10);
        // Should handle narrow width
        assert!(seq.contains("\x1b]66;"));
    }

    #[test]
    fn test_escape_sequence_wide_width() {
        let seq = TextSizing::escape_sequence("A", 1, 500);
        // Should handle wide width
        assert!(seq.contains("\x1b]66;"));
    }

    #[test]
    fn test_escape_sequence_long_text() {
        let long_text = "A".repeat(1000);
        let seq = TextSizing::escape_sequence(&long_text, 2, 500);
        assert!(seq.contains("\x1b]66;"));
    }

    #[test]
    fn test_escape_sequence_contains_width_param() {
        let seq = TextSizing::escape_sequence("Test", 1, 100);
        // Should contain the width parameter using numerator from size_ratio
        // For H1, size_ratio(1) = (7, 7), so w=7 (not w=100)
        assert!(seq.contains("w=7"));
    }

    #[test]
    fn test_height_constant() {
        // Height should always be 2 regardless of content
        assert_eq!(TextSizing::height(), 2);
    }

    #[test]
    fn test_is_supported_returns_bool() {
        let result = is_supported();
        // Should return a boolean
        match result {
            true | false => {}
        }
    }

    #[test]
    fn test_size_ratio_decreases_with_tier() {
        // Higher tiers should have smaller ratios
        let h1 = TextSizing::size_ratio(1);
        let h2 = TextSizing::size_ratio(2);
        let h3 = TextSizing::size_ratio(3);
        let h4 = TextSizing::size_ratio(4);
        let h5 = TextSizing::size_ratio(5);
        let h6 = TextSizing::size_ratio(6);

        // Check the trend (ratios get smaller as tier increases)
        let h1_ratio = h1.0 as f64 / h1.1 as f64;
        let h2_ratio = h2.0 as f64 / h2.1 as f64;
        let h3_ratio = h3.0 as f64 / h3.1 as f64;

        assert!(h1_ratio >= h2_ratio);
        assert!(h2_ratio >= h3_ratio);
    }

    #[test]
    fn test_scaled_width_values() {
        // Test specific scaled width values to verify the formula
        // width=80: H1=40, H2=48, H3=53, H4=60, H5=66, H6=120
        assert_eq!(TextSizing::scaled_width(80, 1), 40);
        assert_eq!(TextSizing::scaled_width(80, 2), 48);
        assert_eq!(TextSizing::scaled_width(80, 3), 53);
        assert_eq!(TextSizing::scaled_width(80, 4), 60);
        assert_eq!(TextSizing::scaled_width(80, 5), 66);
        assert_eq!(TextSizing::scaled_width(80, 6), 120);
    }

    #[test]
    fn test_escape_sequence_contains_clear_commands() {
        let seq = TextSizing::escape_sequence("Test", 1, 80);
        // Should contain erase character commands
        assert!(seq.contains("\x1b["));
        assert!(seq.contains("X")); // ECH uses 'X'
    }
}
