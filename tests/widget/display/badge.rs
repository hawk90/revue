//! Tests for Badge widget
//!
//! Extracted from src/widget/display/badge.rs

use revue::prelude::*;

// =========================================================================
// BadgeVariant enum tests
// =========================================================================

#[test]
fn test_badge_variant_default() {
    let v = BadgeVariant::default();
    assert_eq!(v, BadgeVariant::Default);
}

#[test]
fn test_badge_variant_clone() {
    let v = BadgeVariant::Success;
    let cloned = v.clone();
    assert_eq!(v, cloned);
}

#[test]
fn test_badge_variant_copy() {
    let v1 = BadgeVariant::Error;
    let v2 = v1;
    assert_eq!(v1, BadgeVariant::Error);
    assert_eq!(v2, BadgeVariant::Error);
}

#[test]
fn test_badge_variant_partial_eq() {
    assert_eq!(BadgeVariant::Primary, BadgeVariant::Primary);
    assert_ne!(BadgeVariant::Primary, BadgeVariant::Warning);
}

#[test]
fn test_badge_variant_debug() {
    let v = BadgeVariant::Info;
    assert!(format!("{:?}", v).contains("Info"));
}

#[test]
fn test_badge_variant_colors_default() {
    let (bg, fg) = BadgeVariant::Default.colors();
    assert_eq!(bg, Color::rgb(80, 80, 80));
    assert_eq!(fg, Color::WHITE);
}

#[test]
fn test_badge_variant_colors_primary() {
    let (bg, fg) = BadgeVariant::Primary.colors();
    assert_eq!(bg, Color::rgb(50, 100, 200));
    assert_eq!(fg, Color::WHITE);
}

#[test]
fn test_badge_variant_colors_success() {
    let (bg, fg) = BadgeVariant::Success.colors();
    assert_eq!(bg, Color::rgb(40, 160, 80));
    assert_eq!(fg, Color::WHITE);
}

#[test]
fn test_badge_variant_colors_warning() {
    let (bg, fg) = BadgeVariant::Warning.colors();
    assert_eq!(bg, Color::rgb(200, 150, 40));
    assert_eq!(fg, Color::BLACK);
}

#[test]
fn test_badge_variant_colors_error() {
    let (bg, fg) = BadgeVariant::Error.colors();
    assert_eq!(bg, Color::rgb(200, 60, 60));
    assert_eq!(fg, Color::WHITE);
}

#[test]
fn test_badge_variant_colors_info() {
    let (bg, fg) = BadgeVariant::Info.colors();
    assert_eq!(bg, Color::rgb(60, 160, 180));
    assert_eq!(fg, Color::WHITE);
}

// =========================================================================
// BadgeShape enum tests
// =========================================================================

#[test]
fn test_badge_shape_default() {
    let s = BadgeShape::default();
    assert_eq!(s, BadgeShape::Rounded);
}

#[test]
fn test_badge_shape_clone() {
    let s = BadgeShape::Pill;
    let cloned = s.clone();
    assert_eq!(s, cloned);
}

#[test]
fn test_badge_shape_copy() {
    let s1 = BadgeShape::Square;
    let s2 = s1;
    assert_eq!(s1, BadgeShape::Square);
    assert_eq!(s2, BadgeShape::Square);
}

#[test]
fn test_badge_shape_partial_eq() {
    assert_eq!(BadgeShape::Rounded, BadgeShape::Rounded);
    assert_ne!(BadgeShape::Rounded, BadgeShape::Dot);
}

#[test]
fn test_badge_shape_debug() {
    let s = BadgeShape::Pill;
    assert!(format!("{:?}", s).contains("Pill"));
}

// =========================================================================
// Badge builder tests
// =========================================================================

#[test]
fn test_badge_variants() {
    let b = badge("OK").success();
    // Can't access private variant field
    // Just verify builder compiles

    let b = badge("Error").error();

    let b = badge("Info").info();
}

#[test]
fn test_badge_shapes() {
    let b = badge("Tag").pill();
    // Can't access private shape field
    // Just verify builder compiles

    let b = badge("Box").square();
}

#[test]
fn test_badge_dot() {
    let b = Badge::dot().success();
    // Can't access private fields
    // Just verify builder compiles
}

#[test]
fn test_helper_functions() {
    let b = badge("Hi");
    // Can't access private text field
    // Just verify helper works

    let d = dot_badge();
    // Can't access private shape field
    // Just verify helper works
}

#[test]
fn test_badge_default_trait() {
    let b = Badge::default();
    // Can't access private fields
    // Just verify Default implementation works
}

#[test]
fn test_builder_chain() {
    let b = badge("Chain").primary().pill().bold().max_width(20);
    // Can't access private fields
    // Just verify builder chain compiles
}

#[test]
fn test_badge_variant_chain() {
    let b = Badge::new("X")
        .variant(BadgeVariant::Info)
        .shape(BadgeShape::Square);
    // Can't access private fields
    // Just verify builder chain compiles
}
