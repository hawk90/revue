//! Tests for Symbols
//!
//! Every constant is pinned to its exact character, which also covers
//! which ones are distinct and which are deliberate aliases.

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
