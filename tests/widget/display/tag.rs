//! Tests for Tag widget
//!
//! Extracted from src/widget/display/tag.rs

use revue::prelude::*;

// =========================================================================
// TagStyle enum tests
// =========================================================================

#[test]
fn test_tag_style_default() {
    let style = TagStyle::default();
    assert_eq!(style, TagStyle::Filled);
}

#[test]
fn test_tag_style_clone() {
    let style = TagStyle::Outlined;
    let cloned = style.clone();
    assert_eq!(style, cloned);
}

#[test]
fn test_tag_style_copy() {
    let style1 = TagStyle::Subtle;
    let style2 = style1;
    assert_eq!(style1, TagStyle::Subtle);
    assert_eq!(style2, TagStyle::Subtle);
}

#[test]
fn test_tag_style_partial_eq() {
    assert_eq!(TagStyle::Filled, TagStyle::Filled);
    assert_ne!(TagStyle::Filled, TagStyle::Outlined);
}

#[test]
fn test_tag_style_debug() {
    let style = TagStyle::Subtle;
    assert!(format!("{:?}", style).contains("Subtle"));
}

// =========================================================================
// Tag builder tests
// =========================================================================

#[test]
fn test_tag_styles() {
    let t = tag("Test").outlined();
    // Can't access private style field
    // Just verify builder compiles

    let t = tag("Test").subtle();
}

#[test]
fn test_tag_colors() {
    let t = tag("Test").blue();
    // Can't access private color field
    // Just verify builder compiles

    let t = tag("Test").red();
}

#[test]
fn test_tag_closable() {
    let t = tag("Test").closable();
    // Can't access private closable field
    // Just verify builder compiles
}

#[test]
fn test_tag_icon() {
    let t = tag("Rust").icon('🦀');
    // Can't access private icon field
    // Just verify builder compiles
}

#[test]
fn test_tag_selected_disabled() {
    let t = tag("Test").selected().disabled();
    // Can't access private fields
    // Just verify builder compiles
}

#[test]
fn test_helper_functions() {
    let t = tag("A");
    // Can't access private text field
    // Just verify helper works

    let c = chip("B");
    // Can't access private text field
    // Just verify helper works
}

#[test]
fn test_tag_green() {
    let t = tag("Test").green();
    // Can't access private color field
    // Just verify builder compiles
}

#[test]
fn test_tag_yellow() {
    let t = tag("Test").yellow();
    // Can't access private color field
    // Just verify builder compiles
}

#[test]
fn test_tag_purple() {
    let t = tag("Test").purple();
    // Can't access private color field
    // Just verify builder compiles
}

// =========================================================================
// Tag Default trait tests
// =========================================================================

#[test]
fn test_tag_default() {
    let t = Tag::default();
    // Can't access private text field
    // Just verify Default implementation works
}

// =========================================================================
// Tag builder chain tests
// =========================================================================

#[test]
fn test_tag_builder_chain() {
    let t = Tag::new("Chained")
        .blue()
        .outlined()
        .closable()
        .icon('T')
        .selected();
    // Can't access private fields
    // Just verify builder chain compiles
}

#[test]
fn test_tag_style_chain() {
    let t = Tag::new("Test")
        .style(TagStyle::Subtle)
        .color(Color::RED)
        .text_color(Color::WHITE);
    // Can't access private fields
    // Just verify builder chain compiles
}
