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
mod sections;
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
    #[deprecated(
        since = "3.7.0",
        note = "the overlay has no style panel and never reads this; inspect styles with `revue::devtools::StyleInspector`"
    )]
    #[allow(deprecated)] // sets the deprecated `DebugConfig::show_styles`
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
#[deprecated(
    since = "3.5.0",
    note = "nothing reads the global debug flag; show or hide a `DebugOverlay` with its `visible` builder"
)]
pub fn enable_debug() {
    DEBUG_ENABLED.store(true, Ordering::Relaxed);
}

/// Disable global debug mode
#[deprecated(
    since = "3.5.0",
    note = "nothing reads the global debug flag; show or hide a `DebugOverlay` with its `visible` builder"
)]
pub fn disable_debug() {
    DEBUG_ENABLED.store(false, Ordering::Relaxed);
}

/// Check if debug mode is enabled
#[deprecated(
    since = "3.5.0",
    note = "nothing reads the global debug flag; show or hide a `DebugOverlay` with its `visible` builder"
)]
pub fn is_debug_enabled() -> bool {
    DEBUG_ENABLED.load(Ordering::Relaxed)
}

/// Toggle debug mode
#[deprecated(
    since = "3.5.0",
    note = "nothing reads the global debug flag; show or hide a `DebugOverlay` with its `visible` builder"
)]
pub fn toggle_debug() -> bool {
    let was_enabled = DEBUG_ENABLED.fetch_xor(true, Ordering::Relaxed);
    !was_enabled
}

#[cfg(test)]
mod tests;
