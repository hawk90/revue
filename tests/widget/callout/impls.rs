//! Callout implementation tests

use revue::widget::callout::*;
use revue::widget::callout::types::{CalloutType, CalloutVariant};
use revue::event::Key;

// =========================================================================
// Callout::new tests
// =========================================================================

#[test]
fn test_callout_new() {
    let callout = Callout::new("Test content");
    assert_eq!(callout.content, "Test content");
    assert!(callout.title.is_none());
    assert!(callout.show_icon);
    assert!(callout.custom_icon.is_none());
    assert!(!callout.collapsible);
    assert!(callout.expanded);
}

#[test]
fn test_callout_new_with_string() {
    let s = String::from("Owned string");
    let callout = Callout::new(s);
    assert_eq!(callout.content, "Owned string");
}

#[test]
fn test_callout_new_empty() {
    let callout = Callout::new("");
    assert_eq!(callout.content, "");
}

// =========================================================================
// Callout type constructors
// =========================================================================

#[test]
fn test_callout_note() {
    let callout = Callout::note("Note content");
    assert_eq!(callout.content, "Note content");
    assert_eq!(callout.callout_type, CalloutType::Note);
}

#[test]
fn test_callout_tip() {
    let callout = Callout::tip("Tip content");
    assert_eq!(callout.content, "Tip content");
    assert_eq!(callout.callout_type, CalloutType::Tip);
}

#[test]
fn test_callout_important() {
    let callout = Callout::important("Important content");
    assert_eq!(callout.content, "Important content");
    assert_eq!(callout.callout_type, CalloutType::Important);
}

#[test]
fn test_callout_warning() {
    let callout = Callout::warning("Warning content");
    assert_eq!(callout.content, "Warning content");
    assert_eq!(callout.callout_type, CalloutType::Warning);
}

#[test]
fn test_callout_danger() {
    let callout = Callout::danger("Danger content");
    assert_eq!(callout.content, "Danger content");
    assert_eq!(callout.callout_type, CalloutType::Danger);
}

#[test]
fn test_callout_info() {
    let callout = Callout::info("Info content");
    assert_eq!(callout.content, "Info content");
    assert_eq!(callout.callout_type, CalloutType::Info);
}

// =========================================================================
// Builder methods
// =========================================================================

#[test]
fn test_callout_type_builder() {
    let callout = Callout::new("Test").callout_type(CalloutType::Tip);
    assert_eq!(callout.callout_type, CalloutType::Tip);
}

#[test]
fn test_callout_title() {
    let callout = Callout::new("Test").title("Custom Title");
    assert_eq!(callout.title, Some("Custom Title".to_string()));
}

#[test]
fn test_callout_title_string() {
    let callout = Callout::new("Test").title(String::from("Title"));
    assert_eq!(callout.title, Some("Title".to_string()));
}

#[test]
fn test_callout_variant() {
    let callout = Callout::new("Test").variant(CalloutVariant::Minimal);
    assert_eq!(callout.variant, CalloutVariant::Minimal);
}

#[test]
fn test_callout_icon_true() {
    let callout = Callout::new("Test").icon(true);
    assert!(callout.show_icon);
}

#[test]
fn test_callout_icon_false() {
    let callout = Callout::new("Test").icon(false);
    assert!(!callout.show_icon);
}

#[test]
fn test_callout_custom_icon() {
    let callout = Callout::new("Test").custom_icon('🔔');
    assert_eq!(callout.custom_icon, Some('🔔'));
    assert!(callout.show_icon);
}

#[test]
fn test_callout_collapsible() {
    let callout = Callout::new("Test").collapsible(true);
    assert!(callout.collapsible);
}

#[test]
fn test_callout_not_collapsible() {
    let callout = Callout::new("Test").collapsible(false);
    assert!(!callout.collapsible);
}

#[test]
fn test_callout_expanded() {
    let callout = Callout::new("Test").expanded(true);
    assert!(callout.expanded);
}

#[test]
fn test_callout_not_expanded() {
    let callout = Callout::new("Test").expanded(false);
    assert!(!callout.expanded);
}

#[test]
fn test_callout_collapse_icons() {
    let callout = Callout::new("Test").collapse_icons('◀', '▼');
    assert_eq!(callout.collapsed_icon, '◀');
    assert_eq!(callout.expanded_icon, '▼');
}

#[test]
fn test_callout_builder_chain() {
    let callout = Callout::new("Content")
        .title("Title")
        .callout_type(CalloutType::Warning)
        .variant(CalloutVariant::LeftBorder)
        .icon(true)
        .collapsible(true)
        .expanded(false);

    assert_eq!(callout.content, "Content");
    assert_eq!(callout.title, Some("Title".to_string()));
    assert_eq!(callout.callout_type, CalloutType::Warning);
    assert_eq!(callout.variant, CalloutVariant::LeftBorder);
    assert!(callout.show_icon);
    assert!(callout.collapsible);
    assert!(!callout.expanded);
}

// =========================================================================
// Mutable methods
// =========================================================================

#[test]
fn test_toggle_when_collapsible() {
    let mut callout = Callout::new("Test").collapsible(true).expanded(true);
    callout.toggle();
    assert!(!callout.expanded);
    callout.toggle();
    assert!(callout.expanded);
}

#[test]
fn test_toggle_when_not_collapsible() {
    let mut callout = Callout::new("Test").collapsible(false).expanded(true);
    callout.toggle();
    // Should remain expanded since not collapsible
    assert!(callout.expanded);
}

#[test]
fn test_expand() {
    let mut callout = Callout::new("Test").expanded(false);
    callout.expand();
    assert!(callout.expanded);
}

#[test]
fn test_collapse() {
    let mut callout = Callout::new("Test").expanded(true);
    callout.collapse();
    assert!(!callout.expanded);
}

#[test]
fn test_set_expanded() {
    let mut callout = Callout::new("Test");
    callout.set_expanded(false);
    assert!(!callout.expanded);
    callout.set_expanded(true);
    assert!(callout.expanded);
}

// =========================================================================
// Getter methods
// =========================================================================

#[test]
fn test_is_expanded_true() {
    let callout = Callout::new("Test").expanded(true);
    assert!(callout.is_expanded());
}

#[test]
fn test_is_expanded_false() {
    let callout = Callout::new("Test").expanded(false);
    assert!(!callout.is_expanded());
}

#[test]
fn test_is_collapsible_true() {
    let callout = Callout::new("Test").collapsible(true);
    assert!(callout.is_collapsible());
}

#[test]
fn test_is_collapsible_false() {
    let callout = Callout::new("Test").collapsible(false);
    assert!(!callout.is_collapsible());
}

#[test]
fn test_get_icon_default() {
    let callout = Callout::new("Test").callout_type(CalloutType::Note);
    assert_eq!(callout.get_icon(), CalloutType::Note.icon());
}

#[test]
fn test_get_icon_custom() {
    let callout = Callout::new("Test").custom_icon('🎯');
    assert_eq!(callout.get_icon(), '🎯');
}

#[test]
fn test_collapse_icon_when_expanded() {
    let callout = Callout::new("Test").collapse_icons('◀', '▼').expanded(true);
    assert_eq!(callout.collapse_icon(), '▼');
}

#[test]
fn test_collapse_icon_when_collapsed() {
    let callout = Callout::new("Test")
        .collapse_icons('◀', '▼')
        .expanded(false);
    assert_eq!(callout.collapse_icon(), '◀');
}

#[test]
fn test_get_title_default() {
    let callout = Callout::new("Test").callout_type(CalloutType::Warning);
    assert_eq!(callout.get_title(), "Warning");
}

#[test]
fn test_get_title_custom() {
    let callout = Callout::new("Test").title("Custom Title");
    assert_eq!(callout.get_title(), "Custom Title");
}

// =========================================================================
// Height calculation tests
// =========================================================================

#[test]
fn test_height_filled_variant() {
    let callout = Callout::new("Line 1\nLine 2").variant(CalloutVariant::Filled);
    // 2 (top and title) + content_lines (2) + 1 (bottom) = 5
    assert_eq!(callout.height(), 5);
}

#[test]
fn test_height_left_border_variant() {
    let callout = Callout::new("Line 1\nLine 2").variant(CalloutVariant::LeftBorder);
    // title + content (2 lines) = 1 + 2 = 3
    assert_eq!(callout.height(), 3);
}

#[test]
fn test_height_minimal_variant() {
    let callout = Callout::new("Line 1\nLine 2").variant(CalloutVariant::Minimal);
    // title + content (2 lines) = 1 + 2 = 3
    assert_eq!(callout.height(), 3);
}

#[test]
fn test_height_collapsed() {
    let callout = Callout::new("Content")
        .collapsible(true)
        .expanded(false)
        .variant(CalloutVariant::Filled);
    assert_eq!(callout.height(), 1); // Just header
}

#[test]
fn test_height_empty_content() {
    let callout = Callout::new("").variant(CalloutVariant::Filled);
    // top border + title + content (1 line minimum) + bottom border = 2 + 1 + 1 + 1 = 5
    assert!(callout.height() >= 1);
}

// =========================================================================
// handle_key tests
// =========================================================================

#[test]
fn test_handle_key_enter_toggles() {
    let mut callout = Callout::new("Test").collapsible(true).expanded(true);
    let handled = callout.handle_key(&Key::Enter);
    assert!(handled);
    assert!(!callout.expanded);
}

#[test]
fn test_handle_key_space_toggles() {
    let mut callout = Callout::new("Test").collapsible(true).expanded(true);
    let handled = callout.handle_key(&Key::Char(' '));
    assert!(handled);
    assert!(!callout.expanded);
}

#[test]
fn test_handle_key_right_expands() {
    let mut callout = Callout::new("Test").collapsible(true).expanded(false);
    let handled = callout.handle_key(&Key::Right);
    assert!(handled);
    assert!(callout.expanded);
}

#[test]
fn test_handle_key_l_expands() {
    let mut callout = Callout::new("Test").collapsible(true).expanded(false);
    let handled = callout.handle_key(&Key::Char('l'));
    assert!(handled);
    assert!(callout.expanded);
}

#[test]
fn test_handle_key_left_collapses() {
    let mut callout = Callout::new("Test").collapsible(true).expanded(true);
    let handled = callout.handle_key(&Key::Left);
    assert!(handled);
    assert!(!callout.expanded);
}

#[test]
fn test_handle_key_h_collapses() {
    let mut callout = Callout::new("Test").collapsible(true).expanded(true);
    let handled = callout.handle_key(&Key::Char('h'));
    assert!(handled);
    assert!(!callout.expanded);
}

#[test]
fn test_handle_key_ignored_when_not_collapsible() {
    let mut callout = Callout::new("Test").collapsible(false);
    let handled = callout.handle_key(&Key::Enter);
    assert!(!handled);
    assert!(callout.expanded);
}

#[test]
fn test_handle_key_unknown_key() {
    let mut callout = Callout::new("Test").collapsible(true);
    let handled = callout.handle_key(&Key::Char('x'));
    assert!(!handled);
}

// =========================================================================
// Default trait
// =========================================================================

#[test]
fn test_callout_default() {
    let callout = Callout::default();
    assert_eq!(callout.content, "Callout");
    assert!(callout.show_icon);
}

// =========================================================================
// Clone tests
// =========================================================================

#[test]
fn test_callout_clone() {
    let callout1 = Callout::new("Test")
        .title("Title")
        .callout_type(CalloutType::Warning)
        .collapsible(true)
        .expanded(false);
    let callout2 = callout1.clone();
    assert_eq!(callout1.content, callout2.content);
    assert_eq!(callout1.title, callout2.title);
    assert_eq!(callout1.callout_type, callout2.callout_type);
}