//! Helper functions for BiDi text handling

use crate::text::bidi::types::{BidiClass, ResolvedDirection};

/// Detect the base direction of text
pub fn detect_direction(text: &str) -> ResolvedDirection {
    // UAX #9: The first strong character determines paragraph direction
    for c in text.chars() {
        let class = BidiClass::of(c);
        if class == BidiClass::L {
            return ResolvedDirection::Ltr;
        }
        if class.is_strong_rtl() {
            return ResolvedDirection::Rtl;
        }
    }
    // Default to LTR if no strong characters
    ResolvedDirection::Ltr
}

/// Check if a character is RTL
pub fn is_rtl_char(c: char) -> bool {
    BidiClass::of(c).is_strong_rtl()
}

/// Check if text contains RTL characters
pub fn contains_rtl(text: &str) -> bool {
    text.chars().any(is_rtl_char)
}

/// Get the mirrored version of a character for RTL display
pub fn mirror_char(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        '"' => '"',
        '\'' => '\'',
        '«' => '»',
        '»' => '«',
        '‹' => '›',
        '›' => '‹',
        '⟨' => '⟩',
        '⟩' => '⟨',
        '⟪' => '⟫',
        '⟫' => '⟪',
        '⁅' => '⁆',
        '⁆' => '⁅',
        _ => c,
    }
}

/// Reverse a string while preserving grapheme clusters
pub fn reverse_graphemes(text: &str) -> String {
    // Simple character-based reversal
    // For proper grapheme handling, use unicode-segmentation crate
    text.chars().rev().collect()
}
