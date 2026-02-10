//! Tests for Avatar widget
//!
//! Extracted from src/widget/display/avatar.rs

use revue::prelude::*;

#[test]
fn test_avatar_new() {
    let a = Avatar::new("John Doe");
    // Can't access private fields, but we can verify it was created
    assert_eq!(a.get_initials(), "JD");
}

#[test]
fn test_avatar_initials() {
    let a = Avatar::new("Alice Bob Charlie");
    assert_eq!(a.get_initials(), "AB"); // Only first 2

    let a = Avatar::new("SingleName");
    assert_eq!(a.get_initials(), "S");

    let a = Avatar::from_initials("XY");
    assert_eq!(a.get_initials(), "XY");
}

#[test]
fn test_avatar_icon() {
    let a = Avatar::from_icon('🤖');
    assert_eq!(a.get_initials(), "🤖");
}

#[test]
fn test_avatar_sizes() {
    let a = avatar("John").small();
    // Can't access private size field directly
    // Just verify the builder works
    let a = avatar("John").large();
}

#[test]
fn test_avatar_shapes() {
    let a = avatar("John").circle();
    // Can't access private shape field directly
    // Just verify the builder works
    let a = avatar("John").square();
}

#[test]
fn test_avatar_status() {
    let a = avatar("John").online();
    // Can't access private status field directly
    // Just verify the builder works
    let a = avatar("John").busy();
}

#[test]
fn test_avatar_color_generation() {
    let a1 = Avatar::new("Alice");
    let a2 = Avatar::new("Bob");

    // Different names should generate different colors
    let c1 = a1.get_bg_color();
    let c2 = a2.get_bg_color();
    // May or may not be different due to hash collisions, but should work
    let _ = (c1, c2);
}

#[test]
fn test_helper_functions() {
    let a = avatar("Test");
    // Can't access private name field
    // Just verify the helper works

    let a = avatar_icon('🎨');
    // Can't access private icon field
    // Just verify the helper works
}

// =========================================================================
// AvatarSize enum tests
// =========================================================================

#[test]
fn test_avatar_size_default() {
    let size = AvatarSize::default();
    assert_eq!(size, AvatarSize::Medium);
}

#[test]
fn test_avatar_size_clone() {
    let size = AvatarSize::Large;
    let cloned = size.clone();
    assert_eq!(size, cloned);
}

#[test]
fn test_avatar_size_copy() {
    let size1 = AvatarSize::Small;
    let size2 = size1;
    assert_eq!(size1, AvatarSize::Small);
    assert_eq!(size2, AvatarSize::Small);
}

#[test]
fn test_avatar_size_partial_eq() {
    assert_eq!(AvatarSize::Small, AvatarSize::Small);
    assert_ne!(AvatarSize::Small, AvatarSize::Medium);
}

#[test]
fn test_avatar_size_debug() {
    let size = AvatarSize::Large;
    assert!(format!("{:?}", size).contains("Large"));
}

// =========================================================================
// AvatarShape enum tests
// =========================================================================

#[test]
fn test_avatar_shape_default() {
    let shape = AvatarShape::default();
    assert_eq!(shape, AvatarShape::Circle);
}

#[test]
fn test_avatar_shape_clone() {
    let shape = AvatarShape::Square;
    let cloned = shape.clone();
    assert_eq!(shape, cloned);
}

#[test]
fn test_avatar_shape_copy() {
    let shape1 = AvatarShape::Rounded;
    let shape2 = shape1;
    assert_eq!(shape1, AvatarShape::Rounded);
    assert_eq!(shape2, AvatarShape::Rounded);
}

#[test]
fn test_avatar_shape_partial_eq() {
    assert_eq!(AvatarShape::Circle, AvatarShape::Circle);
    assert_ne!(AvatarShape::Circle, AvatarShape::Square);
}

#[test]
fn test_avatar_shape_debug() {
    let shape = AvatarShape::Rounded;
    assert!(format!("{:?}", shape).contains("Rounded"));
}

// =========================================================================
// Avatar builder chain tests
// =========================================================================

#[test]
fn test_avatar_builder_chain() {
    let a = Avatar::new("Chained")
        .small()
        .square()
        .colors(Color::BLUE, Color::WHITE)
        .online()
        .icon('C');
    // Can't access private fields, just verify the chain compiles
}

#[test]
fn test_avatar_status_chain() {
    let a = avatar("Test").offline().status(Color::YELLOW);
    // Can't verify status field, just verify the chain compiles
}

// =========================================================================
// get_initials edge cases
// =========================================================================

#[test]
fn test_get_initials_empty_name() {
    let a = Avatar::new("");
    assert_eq!(a.get_initials(), "");
}

#[test]
fn test_get_initials_single_word() {
    let a = Avatar::new("Hello");
    assert_eq!(a.get_initials(), "H");
}

#[test]
fn test_get_initials_multiple_words() {
    let a = Avatar::new("One Two Three Four");
    assert_eq!(a.get_initials(), "OT");
}

#[test]
fn test_get_initials_with_icon() {
    let a = Avatar::new("Test").icon('X');
    assert_eq!(a.get_initials(), "X");
}

#[test]
fn test_get_initials_uppercase() {
    let a = Avatar::new("hello world");
    assert_eq!(a.get_initials(), "HW");
}

// =========================================================================
// get_bg_color edge cases
// =========================================================================

#[test]
fn test_get_bg_color_custom() {
    let a = Avatar::new("Test").bg(Color::CYAN);
    assert_eq!(a.get_bg_color(), Color::CYAN);
}

#[test]
fn test_get_bg_color_same_name_same_color() {
    let a1 = Avatar::new("SameName");
    let a2 = Avatar::new("SameName");
    assert_eq!(a1.get_bg_color(), a2.get_bg_color());
}

// =========================================================================
// Avatar Default trait tests
// =========================================================================

#[test]
fn test_avatar_default() {
    let a = Avatar::default();
    assert_eq!(a.get_initials(), "");
}
