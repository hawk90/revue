//! Tests for Progress widget
//!
//! Extracted from src/widget/display/progress.rs

use revue::prelude::*;

// =========================================================================
// Progress::new tests
// =========================================================================

#[test]
fn test_progress_new_zero() {
    let p = Progress::new(0.0);
    assert_eq!(p.value(), 0.0);
}

#[test]
fn test_progress_new_half() {
    let p = Progress::new(0.5);
    assert_eq!(p.value(), 0.5);
}

#[test]
fn test_progress_new_full() {
    let p = Progress::new(1.0);
    assert_eq!(p.value(), 1.0);
}

#[test]
fn test_progress_new_clamps_above() {
    let p = Progress::new(1.5);
    assert_eq!(p.value(), 1.0);
}

#[test]
fn test_progress_new_clamps_below() {
    let p = Progress::new(-0.5);
    assert_eq!(p.value(), 0.0);
}

// =========================================================================
// Progress::progress tests
// =========================================================================

#[test]
fn test_progress_builder() {
    let p = Progress::new(0.0).progress(0.75);
    assert_eq!(p.value(), 0.75);
}

#[test]
fn test_progress_builder_clamps() {
    let p = Progress::new(0.0).progress(2.0);
    assert_eq!(p.value(), 1.0);
}

#[test]
fn test_progress_builder_clamps_negative() {
    let p = Progress::new(0.0).progress(-1.0);
    assert_eq!(p.value(), 0.0);
}

// =========================================================================
// Progress::style tests
// =========================================================================

#[test]
fn test_style_block() {
    let p = Progress::new(0.5).style(ProgressStyle::Block);
    // Can't access private style field
    // Just verify builder compiles
}

#[test]
fn test_style_line() {
    let p = Progress::new(0.5).style(ProgressStyle::Line);
}

#[test]
fn test_style_ascii() {
    let p = Progress::new(0.5).style(ProgressStyle::Ascii);
}

#[test]
fn test_style_braille() {
    let p = Progress::new(0.5).style(ProgressStyle::Braille);
}

// =========================================================================
// Progress::value tests
// =========================================================================

#[test]
fn test_value_zero() {
    let p = Progress::new(0.0);
    assert_eq!(p.value(), 0.0);
}

#[test]
fn test_value_quarter() {
    let p = Progress::new(0.25);
    assert_eq!(p.value(), 0.25);
}

#[test]
fn test_value_half() {
    let p = Progress::new(0.5);
    assert_eq!(p.value(), 0.5);
}

#[test]
fn test_value_three_quarters() {
    let p = Progress::new(0.75);
    assert_eq!(p.value(), 0.75);
}

#[test]
fn test_value_full() {
    let p = Progress::new(1.0);
    assert_eq!(p.value(), 1.0);
}

// =========================================================================
// Progress::set_progress tests
// =========================================================================

#[test]
fn test_set_progress() {
    let mut p = Progress::new(0.0);
    p.set_progress(0.5);
    assert_eq!(p.value(), 0.5);
}

#[test]
fn test_set_progress_clamps_above() {
    let mut p = Progress::new(0.0);
    p.set_progress(1.5);
    assert_eq!(p.value(), 1.0);
}

#[test]
fn test_set_progress_clamps_below() {
    let mut p = Progress::new(0.0);
    p.set_progress(-0.5);
    assert_eq!(p.value(), 0.0);
}

#[test]
fn test_set_progress_multiple_times() {
    let mut p = Progress::new(0.0);
    p.set_progress(0.3);
    assert_eq!(p.value(), 0.3);
    p.set_progress(0.6);
    assert_eq!(p.value(), 0.6);
    p.set_progress(0.9);
    assert_eq!(p.value(), 0.9);
}

// =========================================================================
// Default trait tests
// =========================================================================

#[test]
fn test_progress_default() {
    let p = Progress::default();
    assert_eq!(p.value(), 0.0);
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_progress_helper() {
    let p = progress(0.5);
    assert_eq!(p.value(), 0.5);
}

#[test]
fn test_progress_helper_zero() {
    let p = progress(0.0);
    assert_eq!(p.value(), 0.0);
}

#[test]
fn test_progress_helper_full() {
    let p = progress(1.0);
    assert_eq!(p.value(), 1.0);
}

// =========================================================================
// Builder chain tests
// =========================================================================

#[test]
fn test_builder_chain_full() {
    let p = Progress::new(0.5)
        .style(ProgressStyle::Line)
        .filled_color(Color::BLUE)
        .empty_color(Color::rgb(128, 128, 128))
        .show_percentage(true);
    // Can't access private fields
    // Just verify builder chain compiles
}

#[test]
fn test_builder_chain_with_progress_update() {
    let p = Progress::new(0.25)
        .progress(0.75)
        .style(ProgressStyle::Braille);
    assert_eq!(p.value(), 0.75);
}

// =========================================================================
// ProgressStyle enum trait tests
// =========================================================================

#[test]
fn test_progress_style_default() {
    assert_eq!(ProgressStyle::default(), ProgressStyle::Block);
}

#[test]
fn test_progress_style_clone() {
    let style = ProgressStyle::Line;
    let cloned = style;
    assert_eq!(style, cloned);
}

#[test]
fn test_progress_style_copy() {
    let s1 = ProgressStyle::Ascii;
    let s2 = s1;
    assert_eq!(s1, ProgressStyle::Ascii);
    assert_eq!(s2, ProgressStyle::Ascii);
}

#[test]
fn test_progress_style_partial_eq() {
    assert_eq!(ProgressStyle::Block, ProgressStyle::Block);
    assert_ne!(ProgressStyle::Block, ProgressStyle::Line);
    assert_ne!(ProgressStyle::Line, ProgressStyle::Ascii);
    assert_ne!(ProgressStyle::Ascii, ProgressStyle::Braille);
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_very_small_progress() {
    let p = Progress::new(0.001);
    assert_eq!(p.value(), 0.001);
}

#[test]
fn test_very_large_progress_clamped() {
    let p = Progress::new(999.0);
    assert_eq!(p.value(), 1.0);
}

#[test]
fn test_negative_progress_clamped() {
    let p = Progress::new(-999.0);
    assert_eq!(p.value(), 0.0);
}

#[test]
fn test_exact_boundary_zero() {
    let p = Progress::new(0.0);
    assert_eq!(p.value(), 0.0);
}

#[test]
fn test_exact_boundary_one() {
    let p = Progress::new(1.0);
    assert_eq!(p.value(), 1.0);
}

#[test]
fn test_set_then_builder() {
    let mut p = Progress::new(0.0);
    p.set_progress(0.6);
    assert_eq!(p.value(), 0.6);

    let p2 = p.progress(0.8);
    assert_eq!(p2.value(), 0.8);
}

// =========================================================================
// Style variation tests
// =========================================================================

#[test]
fn test_all_styles_distinct() {
    let _block = Progress::new(0.5).style(ProgressStyle::Block);
    let _line = Progress::new(0.5).style(ProgressStyle::Line);
    let _ascii = Progress::new(0.5).style(ProgressStyle::Ascii);
    let _braille = Progress::new(0.5).style(ProgressStyle::Braille);
    // Can't access private style field
    // Just verify all styles compile
}
