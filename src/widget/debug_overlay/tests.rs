//! Tests for the overlay's builders and the global debug flag

use super::*;
use crate::widget::Text;
use serial_test::serial;

#[test]
fn test_debug_overlay() {
    let text = Text::new("Hello");
    let overlay = DebugOverlay::wrap(text)
        .show_metrics(true)
        .show_tree(true)
        .position(DebugPosition::TopRight)
        .width(30);

    assert!(overlay.visible);
    assert!(overlay.config.show_metrics);
    assert!(overlay.config.show_tree);
}

#[test]
#[serial]
#[allow(deprecated)]
fn test_global_debug_state() {
    disable_debug();
    assert!(!is_debug_enabled());

    enable_debug();
    assert!(is_debug_enabled());

    toggle_debug();
    assert!(!is_debug_enabled());
}

// =========================================================================
// DebugOverlay builder tests
// =========================================================================

#[test]
fn test_debug_overlay_visible() {
    let text = Text::new("test");
    let overlay = DebugOverlay::wrap(text).visible(false);
    assert!(!overlay.visible);
}

#[test]
fn test_debug_overlay_toggle() {
    let text = Text::new("test");
    let mut overlay = DebugOverlay::wrap(text);
    let was_visible = overlay.visible;
    overlay.toggle();
    assert_eq!(overlay.visible, !was_visible);
}

#[test]
fn test_debug_overlay_show_events() {
    let text = Text::new("test");
    let overlay = DebugOverlay::wrap(text).show_events(true);
    assert!(overlay.config.show_events);
}

#[test]
fn test_debug_overlay_position_top_right() {
    let text = Text::new("test");
    let overlay = DebugOverlay::wrap(text).position(DebugPosition::TopRight);
    assert_eq!(overlay.config.position, DebugPosition::TopRight);
}

#[test]
fn test_debug_overlay_position_bottom_left() {
    let text = Text::new("test");
    let overlay = DebugOverlay::wrap(text).position(DebugPosition::BottomLeft);
    assert_eq!(overlay.config.position, DebugPosition::BottomLeft);
}

#[test]
fn test_debug_overlay_position_bottom_right() {
    let text = Text::new("test");
    let overlay = DebugOverlay::wrap(text).position(DebugPosition::BottomRight);
    assert_eq!(overlay.config.position, DebugPosition::BottomRight);
}

// =========================================================================
// DebugOverlay method tests
// =========================================================================

#[test]
fn test_debug_overlay_metrics_mut() {
    let text = Text::new("test");
    let mut overlay = DebugOverlay::wrap(text);
    // The overlay's own metrics: what is recorded through it is what it
    // averages
    overlay
        .metrics_mut()
        .record_render(std::time::Duration::from_millis(4));
    assert_eq!(overlay.metrics.avg_render_time_ms(), 4.0);
}

#[test]
fn test_debug_overlay_events_mut() {
    let text = Text::new("test");
    let mut overlay = DebugOverlay::wrap(text);
    overlay
        .events_mut()
        .log(DebugEvent::KeyPress("x".to_string()));
    assert_eq!(overlay.events.recent(usize::MAX).count(), 1);
}

#[test]
fn test_debug_overlay_log_event() {
    let text = Text::new("test");
    let mut overlay = DebugOverlay::wrap(text);
    overlay.log_event(DebugEvent::Custom("test".to_string()));
    assert_eq!(overlay.events.recent(usize::MAX).count(), 1);
}

#[test]
fn test_debug_overlay_record_widget() {
    let text = Text::new("test");
    let mut overlay = DebugOverlay::wrap(text);
    let info = WidgetInfo::new("Button");
    overlay.record_widget(info);
    assert_eq!(overlay.widgets.len(), 1);
}

#[test]
fn test_debug_overlay_clear_widgets() {
    let text = Text::new("test");
    let mut overlay = DebugOverlay::wrap(text);
    overlay.record_widget(WidgetInfo::new("A"));
    overlay.record_widget(WidgetInfo::new("B"));
    overlay.clear_widgets();
    assert!(overlay.widgets.is_empty());
}

// =========================================================================
// Builder chain tests
// =========================================================================

#[test]
fn test_debug_overlay_builder_chain() {
    let text = Text::new("test");
    let overlay = DebugOverlay::wrap(text)
        .visible(true)
        .show_metrics(true)
        .show_tree(true)
        .show_events(true)
        .position(DebugPosition::TopLeft)
        .width(50);

    assert!(overlay.visible);
    assert!(overlay.config.show_metrics);
    assert!(overlay.config.show_tree);
    assert!(overlay.config.show_events);
    assert_eq!(overlay.config.position, DebugPosition::TopLeft);
    assert_eq!(overlay.config.width, 50);
}

// =========================================================================
// Global debug state tests
// =========================================================================

#[test]
#[serial]
#[allow(deprecated)]
fn test_enable_debug() {
    disable_debug();
    enable_debug();
    assert!(is_debug_enabled());
}

#[test]
#[serial]
#[allow(deprecated)]
fn test_disable_debug() {
    enable_debug();
    disable_debug();
    assert!(!is_debug_enabled());
}

#[test]
#[serial]
#[allow(deprecated)]
fn test_toggle_debug_returns_new_state() {
    disable_debug();
    let enabled = toggle_debug();
    assert!(enabled);
    assert!(is_debug_enabled());

    let disabled = toggle_debug();
    assert!(!disabled);
    assert!(!is_debug_enabled());
}
