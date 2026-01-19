//! Cell tests (from src/render/cell.rs)

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::*;
use revue::style::Color;

#[test]
fn test_cell_new() {
    let cell = Cell::new('A');
    assert_eq!(cell.symbol, 'A');
    assert!(cell.fg.is_none());
    assert!(cell.bg.is_none());
    assert!(cell.modifier.is_empty());
}

#[test]
fn test_cell_builder() {
    let cell = Cell::new('X')
        .fg(Color::RED)
        .bg(Color::BLACK)
        .bold()
        .underline();

    assert_eq!(cell.symbol, 'X');
    assert_eq!(cell.fg, Some(Color::RED));
    assert_eq!(cell.bg, Some(Color::BLACK));
    assert!(cell.modifier.contains(Modifier::BOLD));
    assert!(cell.modifier.contains(Modifier::UNDERLINE));
    assert!(!cell.modifier.contains(Modifier::ITALIC));
}

#[test]
fn test_cell_empty() {
    let cell = Cell::empty();
    assert_eq!(cell.symbol, ' ');
}

#[test]
fn test_cell_continuation() {
    let cell = Cell::continuation();
    assert!(cell.is_continuation());
    assert_eq!(cell.symbol, '\0');
}

#[test]
fn test_cell_reset() {
    let mut cell = Cell::new('A').fg(Color::RED).bold();
    cell.reset();
    assert_eq!(cell.symbol, ' ');
    assert!(cell.fg.is_none());
    assert!(cell.modifier.is_empty());
}

#[test]
fn test_modifier_merge() {
    let m1 = Modifier::BOLD;
    let m2 = Modifier::ITALIC;
    let merged = m1.merge(&m2);

    assert!(merged.contains(Modifier::BOLD));
    assert!(merged.contains(Modifier::ITALIC));
    assert!(!merged.contains(Modifier::UNDERLINE));
}

#[test]
fn test_cell_equality() {
    let c1 = Cell::new('A').fg(Color::RED);
    let c2 = Cell::new('A').fg(Color::RED);
    let c3 = Cell::new('A').fg(Color::BLUE);

    assert_eq!(c1, c2);
    assert_ne!(c1, c3);
}

#[test]
fn test_cell_is_copy() {
    let c1 = Cell::new('A');
    let c2 = c1; // Copy, not move
    assert_eq!(c1.symbol, c2.symbol); // Both still valid
}

#[test]
fn test_modifier_size() {
    // Modifier should be 1 byte with bitflags
    assert_eq!(std::mem::size_of::<Modifier>(), 1);
}

#[test]
fn test_cell_sequence() {
    let cell = Cell::new('X').sequence(42);
    assert_eq!(cell.sequence_id, Some(42));
}

#[test]
fn test_cell_reset_clears_sequence() {
    let mut cell = Cell::new('A').sequence(1);
    assert!(cell.sequence_id.is_some());
    cell.reset();
    assert!(cell.sequence_id.is_none());
}

#[test]
fn test_cell_reverse() {
    let cell = Cell::new('X').reverse();
    assert!(cell.modifier.contains(Modifier::REVERSE));
}

#[test]
fn test_modifier_reverse_combined() {
    let cell = Cell::new('X').bold().reverse();
    assert!(cell.modifier.contains(Modifier::BOLD));
    assert!(cell.modifier.contains(Modifier::REVERSE));
}
