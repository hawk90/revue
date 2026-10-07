//! Debug overlay widget for development
//!
//! Provides a visual debugging overlay that displays:
//! - Widget tree hierarchy
//! - Current styles and computed values
//! - Performance metrics
//! - Event log
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::prelude::*;
//! use revue::widget::DebugOverlay;
//!
//! // Wrap your app with debug overlay
//! let debug = DebugOverlay::wrap(my_view)
//!     .show_tree(true)
//!     .show_metrics(true);
//! ```

mod event_log;
mod metrics;
mod render;
mod types;
mod widget_info;

pub use event_log::{DebugEvent, EventLog};
pub use metrics::PerfMetrics;
pub use types::{DebugConfig, DebugPosition};
pub use widget_info::WidgetInfo;

use crate::widget::View;

// =============================================================================
// Debug Overlay
// =============================================================================

/// Debug overlay widget
///
/// Wraps another view and displays debugging information.
#[derive(Clone)]
pub struct DebugOverlay<V: View> {
    /// Inner view
    inner: V,
    /// Configuration
    config: DebugConfig,
    /// Performance metrics
    metrics: PerfMetrics,
    /// Event log
    events: EventLog,
    /// Widget tree
    widgets: Vec<WidgetInfo>,
    /// Is visible
    visible: bool,
}

impl<V: View> DebugOverlay<V> {
    /// Wrap a view with debug overlay
    pub fn wrap(view: V) -> Self {
        Self {
            inner: view,
            config: DebugConfig::default(),
            metrics: PerfMetrics::new(),
            events: EventLog::new(),
            widgets: Vec::new(),
            visible: true,
        }
    }

    /// Set visibility
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Show/hide metrics
    pub fn show_metrics(mut self, show: bool) -> Self {
        self.config.show_metrics = show;
        self
    }

    /// Show/hide widget tree
    pub fn show_tree(mut self, show: bool) -> Self {
        self.config.show_tree = show;
        self
    }

    /// Show/hide event log
    pub fn show_events(mut self, show: bool) -> Self {
        self.config.show_events = show;
        self
    }

    /// Show/hide style inspector
    pub fn show_styles(mut self, show: bool) -> Self {
        self.config.show_styles = show;
        self
    }

    /// Set panel position
    pub fn position(mut self, position: DebugPosition) -> Self {
        self.config.position = position;
        self
    }

    /// Set panel width
    pub fn width(mut self, width: u16) -> Self {
        self.config.width = width;
        self
    }

    /// Get mutable access to metrics
    pub fn metrics_mut(&mut self) -> &mut PerfMetrics {
        &mut self.metrics
    }

    /// Get mutable access to event log
    pub fn events_mut(&mut self) -> &mut EventLog {
        &mut self.events
    }

    /// Log an event
    pub fn log_event(&mut self, event: DebugEvent) {
        self.events.log(event);
    }

    /// Record widget info
    pub fn record_widget(&mut self, info: WidgetInfo) {
        self.widgets.push(info);
    }

    /// Clear widget info
    pub fn clear_widgets(&mut self) {
        self.widgets.clear();
    }
}

// =============================================================================
// Global Debug State
// =============================================================================

use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

/// Enable global debug mode
pub fn enable_debug() {
    DEBUG_ENABLED.store(true, Ordering::Relaxed);
}

/// Disable global debug mode
pub fn disable_debug() {
    DEBUG_ENABLED.store(false, Ordering::Relaxed);
}

/// Check if debug mode is enabled
pub fn is_debug_enabled() -> bool {
    DEBUG_ENABLED.load(Ordering::Relaxed)
}

/// Toggle debug mode
pub fn toggle_debug() -> bool {
    let was_enabled = DEBUG_ENABLED.fetch_xor(true, Ordering::Relaxed);
    !was_enabled
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
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
    fn test_debug_overlay_show_styles() {
        let text = Text::new("test");
        let overlay = DebugOverlay::wrap(text).show_styles(true);
        assert!(overlay.config.show_styles);
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
        overlay.metrics_mut().start_frame();
        assert!(overlay.metrics.last_frame_start.is_some());
    }

    #[test]
    fn test_debug_overlay_events_mut() {
        let text = Text::new("test");
        let mut overlay = DebugOverlay::wrap(text);
        overlay
            .events_mut()
            .log(DebugEvent::KeyPress("x".to_string()));
        assert_eq!(overlay.events.events.len(), 1);
    }

    #[test]
    fn test_debug_overlay_log_event() {
        let text = Text::new("test");
        let mut overlay = DebugOverlay::wrap(text);
        overlay.log_event(DebugEvent::Custom("test".to_string()));
        assert_eq!(overlay.events.events.len(), 1);
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
            .show_styles(true)
            .position(DebugPosition::TopLeft)
            .width(50);

        assert!(overlay.visible);
        assert!(overlay.config.show_metrics);
        assert!(overlay.config.show_tree);
        assert!(overlay.config.show_events);
        assert!(overlay.config.show_styles);
        assert_eq!(overlay.config.position, DebugPosition::TopLeft);
        assert_eq!(overlay.config.width, 50);
    }

    // =========================================================================
    // Global debug state tests
    // =========================================================================

    #[test]
    #[serial]
    fn test_enable_debug() {
        disable_debug();
        enable_debug();
        assert!(is_debug_enabled());
    }

    #[test]
    #[serial]
    fn test_disable_debug() {
        enable_debug();
        disable_debug();
        assert!(!is_debug_enabled());
    }

    #[test]
    #[serial]
    fn test_toggle_debug_returns_new_state() {
        disable_debug();
        let enabled = toggle_debug();
        assert!(enabled);
        assert!(is_debug_enabled());

        let disabled = toggle_debug();
        assert!(!disabled);
        assert!(!is_debug_enabled());
    }
}
