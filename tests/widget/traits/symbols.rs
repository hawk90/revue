//! Tests for Symbols
//!
//! Extracted from src/widget/traits/symbols.rs

use revue::widget::traits::Symbols;

#[test]
fn test_arrow_symbols() {
    assert_eq!(Symbols::ARROW_LEFT, '←');
    assert_eq!(Symbols::ARROW_RIGHT, '→');
    assert_eq!(Symbols::ARROW_UP, '↑');
    assert_eq!(Symbols::ARROW_DOWN, '↓');
}

#[test]
fn test_triangle_symbols() {
    assert_eq!(Symbols::TRIANGLE_RIGHT, '▶');
    assert_eq!(Symbols::TRIANGLE_DOWN, '▼');
    assert_eq!(Symbols::TRIANGLE_SMALL_RIGHT, '▸');
    assert_eq!(Symbols::TRIANGLE_SMALL_DOWN, '▾');
}

#[test]
fn test_checkbox_symbols() {
    assert_eq!(Symbols::CHECKBOX_EMPTY, '☐');
    assert_eq!(Symbols::CHECKBOX_CHECKED, '☑');
    assert_eq!(Symbols::CHECKBOX_CROSSED, '☒');
}

#[test]
fn test_radio_symbols() {
    assert_eq!(Symbols::RADIO_EMPTY, '○');
    assert_eq!(Symbols::RADIO_SELECTED, '●');
}

#[test]
fn test_star_symbols() {
    assert_eq!(Symbols::STAR_EMPTY, '☆');
    assert_eq!(Symbols::STAR_FILLED, '★');
}

#[test]
fn test_block_symbols() {
    assert_eq!(Symbols::BLOCK_FULL, '█');
    assert_eq!(Symbols::BLOCK_3_4, '▓');
    assert_eq!(Symbols::BLOCK_HALF, '▒');
    assert_eq!(Symbols::BLOCK_1_4, '░');
    assert_eq!(Symbols::BLOCK_EMPTY, '░');
    assert_eq!(Symbols::BLOCK_LIGHT, '░');
    assert_eq!(Symbols::BLOCK_MEDIUM, '▒');
    assert_eq!(Symbols::BLOCK_DARK, '▓');
}

#[test]
fn test_separator_symbols() {
    assert_eq!(Symbols::SEP_VERT, '│');
    assert_eq!(Symbols::BULLET, '•');
    assert_eq!(Symbols::CHEVRON_RIGHT, '»');
    assert_eq!(Symbols::CHEVRON_LEFT, '«');
}

#[test]
fn test_symbols_are_single_chars() {
    // Verify all symbols are single Unicode characters
    assert_eq!(Symbols::ARROW_LEFT.len_utf8(), 3); // Most are 3 bytes
    assert_eq!(Symbols::CHECKBOX_EMPTY.len_utf8(), 3);
    assert_eq!(Symbols::BLOCK_FULL.len_utf8(), 3);
}

// =========================================================================
// Additional symbol tests
// =========================================================================

#[test]
fn test_arrow_symbols_are_distinct() {
    assert_ne!(Symbols::ARROW_LEFT, Symbols::ARROW_RIGHT);
    assert_ne!(Symbols::ARROW_UP, Symbols::ARROW_DOWN);
    assert_ne!(Symbols::ARROW_LEFT, Symbols::ARROW_UP);
}

#[test]
fn test_triangle_symbols_are_distinct() {
    assert_ne!(Symbols::TRIANGLE_RIGHT, Symbols::TRIANGLE_DOWN);
    assert_ne!(Symbols::TRIANGLE_SMALL_RIGHT, Symbols::TRIANGLE_SMALL_DOWN);
}

#[test]
fn test_checkbox_states() {
    assert_ne!(Symbols::CHECKBOX_EMPTY, Symbols::CHECKBOX_CHECKED);
    assert_ne!(Symbols::CHECKBOX_CHECKED, Symbols::CHECKBOX_CROSSED);
    assert_ne!(Symbols::CHECKBOX_EMPTY, Symbols::CHECKBOX_CROSSED);
}

#[test]
fn test_radio_states() {
    assert_ne!(Symbols::RADIO_EMPTY, Symbols::RADIO_SELECTED);
}

#[test]
fn test_star_states() {
    assert_ne!(Symbols::STAR_EMPTY, Symbols::STAR_FILLED);
}

#[test]
fn test_block_levels() {
    // BLOCK_FULL, BLOCK_DARK, BLOCK_HALF/MEDIUM, BLOCK_1_4/EMPTY/LIGHT
    assert_ne!(Symbols::BLOCK_FULL, Symbols::BLOCK_DARK);
    assert_ne!(Symbols::BLOCK_FULL, Symbols::BLOCK_HALF);
    assert_ne!(Symbols::BLOCK_FULL, Symbols::BLOCK_1_4);
    // Note: BLOCK_HALF and BLOCK_MEDIUM are the same (by design)
    assert_eq!(Symbols::BLOCK_HALF, Symbols::BLOCK_MEDIUM);
    // Note: BLOCK_1_4, BLOCK_EMPTY, and BLOCK_LIGHT are the same (by design)
    assert_eq!(Symbols::BLOCK_1_4, Symbols::BLOCK_EMPTY);
    assert_eq!(Symbols::BLOCK_1_4, Symbols::BLOCK_LIGHT);
}

#[test]
fn test_block_shade_aliases() {
    // Verify aliases match their counterparts
    assert_eq!(Symbols::BLOCK_EMPTY, Symbols::BLOCK_LIGHT);
    assert_eq!(Symbols::BLOCK_MEDIUM, Symbols::BLOCK_HALF);
}

#[test]
fn test_chevron_directions() {
    assert_ne!(Symbols::CHEVRON_LEFT, Symbols::CHEVRON_RIGHT);
}

#[test]
fn test_all_symbols_printable() {
    // All symbols should be printable (not control chars)
    assert!(!Symbols::ARROW_LEFT.is_control());
    assert!(!Symbols::CHECKBOX_CHECKED.is_control());
    assert!(!Symbols::BLOCK_FULL.is_control());
    assert!(!Symbols::BULLET.is_control());
}

#[test]
fn test_symbol_consistency() {
    // BLOCK_1_4 and BLOCK_EMPTY should be the same
    assert_eq!(Symbols::BLOCK_1_4, Symbols::BLOCK_EMPTY);
}
