//! Popover widget for anchor-positioned interactive overlays
//!
//! Unlike Tooltip (hover-only), Popover supports click triggers, focus trapping,
//! and interactive content. Essential for DatePicker, Combobox, etc.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Popover, PopoverPosition, popover};
//!
//! // Basic popover
//! Popover::new("Click me for details")
//!     .anchor(10, 5)
//!     .position(PopoverPosition::Bottom);
//!
//! // Interactive popover with trigger
//! popover("Menu content")
//!     .trigger(PopoverTrigger::Click)
//!     .close_on_escape(true)
//!     .close_on_click_outside(true);
//! ```

mod core;
mod impls;
mod types;

#[cfg(test)]
mod tests {
use super::*;
use crate::event::Key;

#[test]
fn test_popover_new() {
    let p = Popover::new("Test content");
    assert!(!p.is_open());
}

#[test]
fn test_popover_builder() {
    let p = Popover::new("Content")
        .anchor(10, 5)
        .position(PopoverPosition::Top)
        .trigger(PopoverTrigger::Hover)
        .popover_style(PopoverStyle::Rounded)
        .arrow(PopoverArrow::Unicode)
        .title("Title")
        .max_width(30)
        .close_on_escape(false)
        .close_on_click_outside(false);

    assert_eq!(p.anchor, (10, 5));
}

#[test]
fn test_popover_visibility() {
    let mut p = Popover::new("Test");
    assert!(!p.is_open());

    p.show();
    assert!(p.is_open());

    p.hide();
    assert!(!p.is_open());

    p.toggle();
    assert!(p.is_open());

    p.toggle();
    assert!(!p.is_open());
}

#[test]
fn test_popover_handle_escape() {
    let mut p = Popover::new("Test").open(true);
    assert!(p.is_open());

    assert!(p.handle_key(&Key::Escape));
    assert!(!p.is_open());
}

#[test]
fn test_popover_handle_escape_disabled() {
    let mut p = Popover::new("Test").open(true).close_on_escape(false);
    assert!(!p.handle_key(&Key::Escape));
    assert!(p.is_open());
}

#[test]
fn test_popover_handle_key_closed() {
    let mut p = Popover::new("Test");
    assert!(!p.handle_key(&Key::Escape));
}

#[test]
fn test_popover_helper() {
    let p = popover("Quick popover");
}

#[test]
fn test_popover_default() {
    let p = Popover::default();
}

#[test]
fn test_popover_set_anchor() {
    let mut p = Popover::new("Test");
    p.set_anchor(15, 25);
    assert_eq!(p.anchor, (15, 25));
}

#[test]
fn test_popover_trigger_types() {
    let _ = Popover::new("Test").trigger(PopoverTrigger::Click);
    let _ = Popover::new("Test").trigger(PopoverTrigger::Hover);
    let _ = Popover::new("Test").trigger(PopoverTrigger::Focus);
    let _ = Popover::new("Test").trigger(PopoverTrigger::Manual);
}

#[test]
fn test_popover_custom_colors() {
    let p = Popover::new("Test")
        .fg(crate::style::Color::RED)
        .bg(crate::style::Color::BLUE)
        .border_color(crate::style::Color::GREEN);

    assert_eq!(p.state.fg, Some(crate::style::Color::RED));
    assert_eq!(p.state.bg, Some(crate::style::Color::BLUE));
    assert_eq!(p.border_color, Some(crate::style::Color::GREEN));
}

#[test]
fn test_popover_handle_click_inside() {
    let mut p = Popover::new("Test").anchor(20, 10).open(true);

    // Click inside the popover area
    let handled = p.handle_click(20, 12, 40, 20);
    assert!(handled);
    assert!(p.is_open()); // Should stay open
}

#[test]
fn test_popover_handle_click_outside() {
    let mut p = Popover::new("Test").anchor(20, 10).open(true);

    // Click outside the popover
    let handled = p.handle_click(0, 0, 40, 20);
    assert!(handled);
    assert!(!p.is_open()); // Should close
}

#[test]
fn test_popover_handle_click_outside_disabled() {
    let mut p = Popover::new("Test")
        .anchor(20, 10)
        .open(true)
        .close_on_click_outside(false);

    let handled = p.handle_click(0, 0, 40, 20);
    assert!(!handled);
    assert!(p.is_open()); // Should stay open
}

}

// Re-exports
pub use types::{PopoverArrow, PopoverPosition, PopoverStyle, PopoverTrigger};

pub use core::Popover;
pub use impls::popover;
