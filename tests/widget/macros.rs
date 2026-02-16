//! Tests for declarative UI macros
//!
//! These tests verify the functionality of the declarative UI macros
//! including vstack, hstack, text, bordered, and ui! macros.

use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::{RenderContext, Text, View};

#[test]
fn test_vstack_macro() {
    let stack = vstack![Text::new("Line 1"), Text::new("Line 2"),];
    assert_eq!(stack.len(), 2);
}

#[test]
fn test_vstack_macro_with_gap() {
    let stack = vstack![gap: 2;
        Text::new("A"),
        Text::new("B"),
    ];
    assert_eq!(stack.len(), 2);
}

#[test]
fn test_hstack_macro() {
    let stack = hstack![Text::new("Left"), Text::new("Right"),];
    assert_eq!(stack.len(), 2);
}

#[test]
fn test_text_macro() {
    let t = text!("Hello");
    assert_eq!(t.content(), "Hello");

    let t = text!("Error", red);
    assert_eq!(t.content(), "Error");
}

#[test]
fn test_bordered_macro() {
    let b = bordered![Text::new("Content")];
    // Just verify it compiles and creates a border
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
}

#[test]
fn test_bordered_macro_with_title() {
    let b = bordered!["Title"; Text::new("Content")];
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
}

#[test]
fn test_nested_layout() {
    let layout = vstack![
        Text::heading("Title"),
        hstack![Text::new("Left"), Text::new("Right"),],
        Text::muted("Footer"),
    ];
    assert_eq!(layout.len(), 3);
}

// =========================================================================
// hstack! macro tests
// =========================================================================

#[test]
fn test_hstack_macro_with_gap() {
    let stack = hstack![gap: 2;
        Text::new("A"),
        Text::new("B"),
    ];
    assert_eq!(stack.len(), 2);
}

// =========================================================================
// text! macro color variants
// =========================================================================

#[test]
fn test_text_macro_green() {
    let t = text!("Success", green);
    assert_eq!(t.content(), "Success");
}

#[test]
fn test_text_macro_yellow() {
    let t = text!("Warning", yellow);
    assert_eq!(t.content(), "Warning");
}

#[test]
fn test_text_macro_cyan() {
    let t = text!("Info", cyan);
    assert_eq!(t.content(), "Info");
}

// =========================================================================
// text! macro with modifiers
// =========================================================================

#[test]
fn test_text_macro_bold_modifier() {
    let t = text!("Bold", WHITE, bold);
    assert_eq!(t.content(), "Bold");
}

#[test]
fn test_text_macro_italic_modifier() {
    let t = text!("Italic", CYAN, italic);
    assert_eq!(t.content(), "Italic");
}

// =========================================================================
// bordered! macro edge cases
// =========================================================================

#[test]
fn test_bordered_macro_border_type_and_title() {
    let b = bordered![double, "Card"; Text::new("Content")];
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
}

#[test]
fn test_bordered_macro_rounded() {
    let b = bordered![rounded, "Card"; Text::new("Content")];
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
}

// =========================================================================
// Macro edge cases
// =========================================================================

#[test]
fn test_vstack_macro_trailing_comma() {
    let stack = vstack![Text::new("A"), Text::new("B"),];
    assert_eq!(stack.len(), 2);
}

#[test]
fn test_hstack_macro_trailing_comma() {
    let stack = hstack![Text::new("A"), Text::new("B"),];
    assert_eq!(stack.len(), 2);
}

#[test]
fn test_vstack_macro_single_child() {
    let stack = vstack![Text::new("Only")];
    assert_eq!(stack.len(), 1);
}

#[test]
fn test_hstack_macro_single_child() {
    let stack = hstack![Text::new("Only")];
    assert_eq!(stack.len(), 1);
}

// =========================================================================
// vstack! macro with no children edge case
// =========================================================================

#[test]
fn test_vstack_macro_empty() {
    let stack = vstack![];
    assert_eq!(stack.len(), 0);
}

#[test]
fn test_hstack_macro_empty() {
    let stack = hstack![];
    assert_eq!(stack.len(), 0);
}