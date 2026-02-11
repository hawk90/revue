//! Tests for WidgetState and WidgetProps
//!
//! Extracted from src/widget/traits/widget_state.rs

use revue::style::Color;
use revue::widget::traits::widget_state::{DISABLED_BG, DISABLED_FG, WidgetProps, WidgetState};

// =========================================================================
// WidgetProps tests
// =========================================================================

#[test]
fn test_widget_props_new() {
    let props = WidgetProps::new();
    assert!(props.id.is_none());
    assert!(props.classes.is_empty());
    assert!(props.inline_style.is_none());
}

#[test]
fn test_widget_props_default() {
    let props = WidgetProps::default();
    assert!(props.id.is_none());
    assert!(props.classes.is_empty());
}

#[test]
fn test_widget_props_id() {
    let props = WidgetProps::new().id("my-id");
    assert_eq!(props.id, Some("my-id".to_string()));
}

#[test]
fn test_widget_props_class() {
    let props = WidgetProps::new().class("primary").class("active");
    assert_eq!(props.classes.len(), 2);
    assert!(props.classes.contains(&"primary".to_string()));
    assert!(props.classes.contains(&"active".to_string()));
}

#[test]
fn test_widget_props_style() {
    let style = revue::style::Style::default();
    let props = WidgetProps::new().style(style.clone());
    assert!(props.inline_style.is_some());
}

#[test]
fn test_widget_props_classes_slice() {
    let props = WidgetProps::new().class("a").class("b");
    let slice = props.classes_slice();
    assert_eq!(slice.len(), 2);
}

#[test]
fn test_widget_props_classes_vec() {
    let props = WidgetProps::new().class("a").class("b");
    let vec = props.classes_vec();
    assert_eq!(vec.len(), 2);
}

// =========================================================================
// WidgetState tests
// =========================================================================

#[test]
fn test_widget_state_new() {
    let state = WidgetState::new();
    assert!(!state.focused);
    assert!(!state.disabled);
    assert!(!state.pressed);
    assert!(!state.hovered);
    assert!(state.fg.is_none());
    assert!(state.bg.is_none());
}

#[test]
fn test_widget_state_default() {
    let state = WidgetState::default();
    assert!(!state.is_focused());
    assert!(!state.is_disabled());
}

#[test]
fn test_widget_state_focused_builder() {
    let state = WidgetState::new().focused(true);
    assert!(state.is_focused());
}

#[test]
fn test_widget_state_disabled_builder() {
    let state = WidgetState::new().disabled(true);
    assert!(state.is_disabled());
}

#[test]
fn test_widget_state_pressed_builder() {
    let state = WidgetState::new().pressed(true);
    assert!(state.is_pressed());
}

#[test]
fn test_widget_state_hovered_builder() {
    let state = WidgetState::new().hovered(true);
    assert!(state.is_hovered());
}

#[test]
fn test_widget_state_fg_builder() {
    let color = Color::rgb(255, 0, 0);
    let state = WidgetState::new().fg(color);
    assert_eq!(state.fg, Some(color));
}

#[test]
fn test_widget_state_bg_builder() {
    let color = Color::rgb(0, 0, 255);
    let state = WidgetState::new().bg(color);
    assert_eq!(state.bg, Some(color));
}

#[test]
fn test_widget_state_is_interactive() {
    // Not interactive when not focused/hovered/pressed
    let state = WidgetState::new();
    assert!(!state.is_interactive());

    // Interactive when focused
    let state = WidgetState::new().focused(true);
    assert!(state.is_interactive());

    // Interactive when hovered
    let state = WidgetState::new().hovered(true);
    assert!(state.is_interactive());

    // Not interactive when disabled (even if focused)
    let state = WidgetState::new().focused(true).disabled(true);
    assert!(!state.is_interactive());
}

#[test]
fn test_widget_state_effective_fg() {
    let default = Color::rgb(255, 255, 255);
    let custom = Color::rgb(255, 0, 0);

    // Uses default when no custom color
    let state = WidgetState::new();
    assert_eq!(state.effective_fg(default), default);

    // Uses custom color when set
    let state = WidgetState::new().fg(custom);
    assert_eq!(state.effective_fg(default), custom);

    // Uses DISABLED_FG when disabled
    let state = WidgetState::new().disabled(true);
    assert_eq!(state.effective_fg(default), DISABLED_FG);
}

#[test]
fn test_widget_state_effective_bg() {
    let default = Color::rgb(0, 0, 0);
    let custom = Color::rgb(0, 0, 255);

    // Uses default when no custom color
    let state = WidgetState::new();
    assert_eq!(state.effective_bg(default), default);

    // Uses custom color when set
    let state = WidgetState::new().bg(custom);
    assert_eq!(state.effective_bg(default), custom);

    // Uses DISABLED_BG when disabled
    let state = WidgetState::new().disabled(true);
    assert_eq!(state.effective_bg(default), DISABLED_BG);
}

#[test]
fn test_widget_state_state_colors() {
    let base_fg = Color::rgb(255, 255, 255);
    let base_bg = Color::rgb(0, 0, 0);
    let hover_bg = Color::rgb(50, 50, 50);

    // Normal state
    let state = WidgetState::new();
    let (fg, bg) = state.state_colors(base_fg, base_bg, hover_bg);
    assert_eq!(fg, base_fg);
    assert_eq!(bg, base_bg);

    // Hovered state
    let state = WidgetState::new().hovered(true);
    let (fg, bg) = state.state_colors(base_fg, base_bg, hover_bg);
    assert_eq!(fg, base_fg);
    assert_eq!(bg, hover_bg);

    // Disabled state
    let state = WidgetState::new().disabled(true);
    let (fg, bg) = state.state_colors(base_fg, base_bg, hover_bg);
    assert_eq!(fg, DISABLED_FG);
    assert_eq!(bg, DISABLED_BG);
}

#[test]
fn test_widget_state_reset_transient() {
    let mut state = WidgetState::new().focused(true).pressed(true).hovered(true);

    state.reset_transient();

    assert!(state.is_focused()); // Persistent, not reset
    assert!(!state.is_pressed()); // Transient, reset
    assert!(!state.is_hovered()); // Transient, reset
}

#[test]
fn test_widget_state_set_focused() {
    let mut state = WidgetState::new();
    state.set_focused(true);
    assert!(state.is_focused());
    state.set_focused(false);
    assert!(!state.is_focused());
}

#[test]
fn test_widget_state_set_disabled() {
    let mut state = WidgetState::new();
    state.set_disabled(true);
    assert!(state.is_disabled());
}

#[test]
fn test_widget_state_set_hovered() {
    let mut state = WidgetState::new();
    state.set_hovered(true);
    assert!(state.is_hovered());
}

#[test]
fn test_widget_state_set_pressed() {
    let mut state = WidgetState::new();
    state.set_pressed(true);
    assert!(state.is_pressed());
}

#[test]
fn test_widget_state_set_fg() {
    let mut state = WidgetState::new();
    let color = Color::rgb(255, 0, 0);
    state.set_fg(Some(color));
    assert_eq!(state.fg, Some(color));
    state.set_fg(None);
    assert_eq!(state.fg, None);
}

#[test]
fn test_widget_state_set_bg() {
    let mut state = WidgetState::new();
    let color = Color::rgb(0, 0, 255);
    state.set_bg(Some(color));
    assert_eq!(state.bg, Some(color));
}

#[test]
fn test_widget_state_effective_fg_opt() {
    // None when not disabled and no custom color
    let state = WidgetState::new();
    assert!(state.effective_fg_opt().is_none());

    // Custom color when set
    let color = Color::rgb(255, 0, 0);
    let state = WidgetState::new().fg(color);
    assert_eq!(state.effective_fg_opt(), Some(color));

    // DISABLED_FG when disabled
    let state = WidgetState::new().disabled(true);
    assert_eq!(state.effective_fg_opt(), Some(DISABLED_FG));
}

#[test]
fn test_widget_state_effective_bg_opt() {
    // None when not disabled and no custom color
    let state = WidgetState::new();
    assert!(state.effective_bg_opt().is_none());

    // Custom color when set
    let color = Color::rgb(0, 0, 255);
    let state = WidgetState::new().bg(color);
    assert_eq!(state.effective_bg_opt(), Some(color));

    // DISABLED_BG when disabled
    let state = WidgetState::new().disabled(true);
    assert_eq!(state.effective_bg_opt(), Some(DISABLED_BG));
}

#[test]
fn test_widget_state_resolve_fg() {
    let default = Color::rgb(255, 255, 255);
    let custom = Color::rgb(255, 0, 0);

    // Uses default with no style
    let state = WidgetState::new();
    assert_eq!(state.resolve_fg(None, default), default);

    // Uses custom override
    let state = WidgetState::new().fg(custom);
    assert_eq!(state.resolve_fg(None, default), custom);

    // Disabled overrides everything
    let state = WidgetState::new().fg(custom).disabled(true);
    assert_eq!(state.resolve_fg(None, default), DISABLED_FG);
}

#[test]
fn test_widget_state_resolve_bg() {
    let default = Color::rgb(0, 0, 0);

    let state = WidgetState::new();
    assert_eq!(state.resolve_bg(None, default), default);
}

#[test]
fn test_widget_state_resolve_colors() {
    let default_fg = Color::rgb(255, 255, 255);
    let default_bg = Color::rgb(0, 0, 0);

    let state = WidgetState::new();
    let (fg, bg) = state.resolve_colors(None, default_fg, default_bg);
    assert_eq!(fg, default_fg);
    assert_eq!(bg, default_bg);
}

#[test]
fn test_widget_state_resolve_colors_interactive_disabled() {
    let default_fg = Color::rgb(255, 255, 255);
    let default_bg = Color::rgb(0, 0, 0);

    let state = WidgetState::new().disabled(true);
    let (fg, bg) = state.resolve_colors_interactive(None, default_fg, default_bg);
    assert_eq!(fg, DISABLED_FG);
    assert_eq!(bg, DISABLED_BG);
}

#[test]
fn test_widget_state_visual_changed() {
    let state1 = WidgetState::new();
    let state2 = WidgetState::new();
    assert!(!state1.visual_changed(&state2));

    let state1 = WidgetState::new().focused(true);
    let state2 = WidgetState::new();
    assert!(state1.visual_changed(&state2));

    let state1 = WidgetState::new().disabled(true);
    let state2 = WidgetState::new();
    assert!(state1.visual_changed(&state2));

    let state1 = WidgetState::new().pressed(true);
    let state2 = WidgetState::new();
    assert!(state1.visual_changed(&state2));

    let state1 = WidgetState::new().hovered(true);
    let state2 = WidgetState::new();
    assert!(state1.visual_changed(&state2));
}

#[test]
fn test_disabled_colors_are_defined() {
    // Just verify the constants exist and have expected values
    assert_eq!(DISABLED_FG.r, 100);
    assert_eq!(DISABLED_FG.g, 100);
    assert_eq!(DISABLED_FG.b, 100);

    assert_eq!(DISABLED_BG.r, 50);
    assert_eq!(DISABLED_BG.g, 50);
    assert_eq!(DISABLED_BG.b, 50);
}
