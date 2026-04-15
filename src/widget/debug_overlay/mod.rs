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
mod widget_info;

pub use event_log::{DebugEvent, EventLog};
pub use metrics::PerfMetrics;
pub use widget_info::WidgetInfo;

use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::utils::draw_text_overlay;
use crate::widget::theme::{EDITOR_BG, SECONDARY_TEXT};
use crate::widget::{RenderContext, View};

// =============================================================================
// Debug Panel Configuration
// =============================================================================

/// Position for debug panel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DebugPosition {
    /// Top-left corner
    TopLeft,
    /// Top-right corner
    #[default]
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-right corner
    BottomRight,
}

/// Debug panel configuration
#[derive(Debug, Clone)]
pub struct DebugConfig {
    /// Show performance metrics
    pub show_metrics: bool,
    /// Show widget tree
    pub show_tree: bool,
    /// Show event log
    pub show_events: bool,
    /// Show style inspector
    pub show_styles: bool,
    /// Panel position
    pub position: DebugPosition,
    /// Panel width
    pub width: u16,
    /// Maximum height
    pub max_height: u16,
    /// Panel opacity (0-255)
    pub opacity: u8,
    /// Background color
    pub bg_color: Color,
    /// Text color
    pub fg_color: Color,
    /// Accent color
    pub accent_color: Color,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            show_metrics: true,
            show_tree: false,
            show_events: false,
            show_styles: false,
            position: DebugPosition::TopRight,
            width: 40,
            max_height: 20,
            opacity: 220,
            bg_color: EDITOR_BG,
            fg_color: SECONDARY_TEXT,
            accent_color: Color::rgb(100, 200, 255),
        }
    }
}

// =============================================================================
// Debug Overlay
// =============================================================================

/// Debug overlay widget
///
/// Wraps another view and displays debugging information.
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

    /// Calculate panel rectangle
    fn panel_rect(&self, area: Rect) -> Rect {
        let width = self.config.width.min(area.width);
        let height = self.config.max_height.min(area.height);

        let (x, y) = match self.config.position {
            DebugPosition::TopLeft => (area.x, area.y),
            DebugPosition::TopRight => (area.x + area.width - width, area.y),
            DebugPosition::BottomLeft => (area.x, area.y + area.height - height),
            DebugPosition::BottomRight => {
                (area.x + area.width - width, area.y + area.height - height)
            }
        };

        Rect::new(x, y, width, height)
    }

    /// Render debug panel
    fn render_panel(&self, buffer: &mut Buffer, rect: Rect) {
        // Fill background
        for y in rect.y..rect.y + rect.height {
            for x in rect.x..rect.x + rect.width {
                if let Some(cell) = buffer.get_mut(x, y) {
                    cell.symbol = ' ';
                    cell.bg = Some(self.config.bg_color);
                    cell.fg = Some(self.config.fg_color);
                }
            }
        }

        let mut y = rect.y;
        let x = rect.x + 1;
        let max_y = rect.y + rect.height;

        // Title bar
        let title = " Debug ";
        self.draw_text(buffer, x, y, title, self.config.accent_color);
        y += 1;

        // Separator
        self.draw_text(
            buffer,
            x,
            y,
            &"-".repeat((rect.width - 2) as usize),
            self.config.fg_color,
        );
        y += 1;

        // Performance metrics
        if self.config.show_metrics && y < max_y {
            let fps_color = if self.metrics.fps() >= 30.0 {
                Color::rgb(100, 255, 100) // Green
            } else if self.metrics.fps() >= 15.0 {
                Color::rgb(255, 255, 100) // Yellow
            } else {
                Color::rgb(255, 100, 100) // Red
            };

            self.draw_text(
                buffer,
                x,
                y,
                &format!("FPS: {:.1}", self.metrics.fps()),
                fps_color,
            );
            y += 1;

            if y < max_y {
                self.draw_text(
                    buffer,
                    x,
                    y,
                    &format!("Frame: {:.2}ms", self.metrics.avg_frame_time_ms()),
                    self.config.fg_color,
                );
                y += 1;
            }

            if y < max_y {
                self.draw_text(
                    buffer,
                    x,
                    y,
                    &format!("Layout: {:.2}ms", self.metrics.avg_layout_time_ms()),
                    self.config.fg_color,
                );
                y += 1;
            }

            if y < max_y {
                self.draw_text(
                    buffer,
                    x,
                    y,
                    &format!("Render: {:.2}ms", self.metrics.avg_render_time_ms()),
                    self.config.fg_color,
                );
                y += 1;
            }

            y += 1; // Spacing
        }

        // Widget tree
        if self.config.show_tree && y < max_y {
            self.draw_text(buffer, x, y, "Widgets:", self.config.accent_color);
            y += 1;

            for widget in &self.widgets {
                if y >= max_y {
                    break;
                }
                let line = widget.tree_line();
                let truncated = if line.len() > (rect.width - 2) as usize {
                    format!("{}...", &line[..(rect.width - 5) as usize])
                } else {
                    line
                };
                let color = if widget.focused {
                    self.config.accent_color
                } else {
                    self.config.fg_color
                };
                self.draw_text(buffer, x, y, &truncated, color);
                y += 1;
            }

            y += 1; // Spacing
        }

        // Event log
        if self.config.show_events && y < max_y {
            self.draw_text(buffer, x, y, "Events:", self.config.accent_color);
            y += 1;

            for (_, event) in self.events.recent(5) {
                if y >= max_y {
                    break;
                }
                let text = match event {
                    DebugEvent::KeyPress(k) => format!("Key: {}", k),
                    DebugEvent::Mouse(m) => format!("Mouse: {}", m),
                    DebugEvent::StateChange(s) => format!("State: {}", s),
                    DebugEvent::Custom(c) => c.clone(),
                };
                let truncated = if text.len() > (rect.width - 2) as usize {
                    format!("{}...", &text[..(rect.width - 5) as usize])
                } else {
                    text
                };
                self.draw_text(buffer, x, y, &truncated, self.config.fg_color);
                y += 1;
            }
        }

        // Border
        self.draw_border(buffer, rect);
    }

    /// Draw text at position
    fn draw_text(&self, buffer: &mut Buffer, x: u16, y: u16, text: &str, color: Color) {
        draw_text_overlay(buffer, x, y, text, color);
    }

    /// Draw border around rect
    fn draw_border(&self, buffer: &mut Buffer, rect: Rect) {
        let border_color = self.config.accent_color;

        // Top and bottom
        for x in rect.x..rect.x + rect.width {
            if let Some(cell) = buffer.get_mut(x, rect.y) {
                cell.symbol = if x == rect.x {
                    '┌'
                } else if x == rect.x + rect.width - 1 {
                    '┐'
                } else {
                    '─'
                };
                cell.fg = Some(border_color);
            }
            if let Some(cell) = buffer.get_mut(x, rect.y + rect.height - 1) {
                cell.symbol = if x == rect.x {
                    '└'
                } else if x == rect.x + rect.width - 1 {
                    '┘'
                } else {
                    '─'
                };
                cell.fg = Some(border_color);
            }
        }

        // Left and right
        for y in rect.y + 1..rect.y + rect.height - 1 {
            if let Some(cell) = buffer.get_mut(rect.x, y) {
                cell.symbol = '│';
                cell.fg = Some(border_color);
            }
            if let Some(cell) = buffer.get_mut(rect.x + rect.width - 1, y) {
                cell.symbol = '│';
                cell.fg = Some(border_color);
            }
        }
    }
}

impl<V: View> View for DebugOverlay<V> {
    fn render(&self, ctx: &mut RenderContext) {
        // Render inner view
        self.inner.render(ctx);

        // Render debug overlay
        if self.visible {
            let panel_rect = self.panel_rect(ctx.area);
            self.render_panel(ctx.buffer, panel_rect);
        }
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
    fn test_debug_config_default() {
        let config = DebugConfig::default();
        assert!(config.show_metrics);
        assert!(!config.show_tree);
        assert!(!config.show_events);
        assert_eq!(config.width, 40);
    }

    #[test]
    fn test_global_debug_state() {
        disable_debug();
        assert!(!is_debug_enabled());

        enable_debug();
        assert!(is_debug_enabled());

        toggle_debug();
        assert!(!is_debug_enabled());
    }

    #[test]
    fn test_panel_rect_positions() {
        let text = Text::new("test");
        let overlay = DebugOverlay::wrap(text)
            .width(20)
            .position(DebugPosition::TopLeft);

        let area = Rect::new(0, 0, 80, 24);
        let panel = overlay.panel_rect(area);

        assert_eq!(panel.x, 0);
        assert_eq!(panel.y, 0);
        assert_eq!(panel.width, 20);
    }

    // =========================================================================
    // DebugPosition enum tests
    // =========================================================================

    #[test]
    fn test_debug_position_default() {
        let pos = DebugPosition::default();
        assert_eq!(pos, DebugPosition::TopRight);
    }

    #[test]
    fn test_debug_position_clone() {
        let pos = DebugPosition::BottomLeft;
        let cloned = pos.clone();
        assert_eq!(pos, cloned);
    }

    #[test]
    fn test_debug_position_copy() {
        let pos1 = DebugPosition::TopLeft;
        let pos2 = pos1;
        assert_eq!(pos1, DebugPosition::TopLeft);
        assert_eq!(pos2, DebugPosition::TopLeft);
    }

    #[test]
    fn test_debug_position_partial_eq() {
        assert_eq!(DebugPosition::TopLeft, DebugPosition::TopLeft);
        assert_ne!(DebugPosition::TopLeft, DebugPosition::BottomLeft);
    }

    #[test]
    fn test_debug_position_debug() {
        let pos = DebugPosition::BottomRight;
        assert!(format!("{:?}", pos).contains("BottomRight"));
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
    // panel_rect tests for all positions
    // =========================================================================

    #[test]
    fn test_panel_rect_top_right() {
        let text = Text::new("test");
        let overlay = DebugOverlay::wrap(text)
            .width(20)
            .position(DebugPosition::TopRight);

        let area = Rect::new(0, 0, 100, 50);
        let panel = overlay.panel_rect(area);

        assert_eq!(panel.x, 80);
        assert_eq!(panel.y, 0);
    }

    #[test]
    fn test_panel_rect_bottom_left() {
        let text = Text::new("test");
        let mut overlay = DebugOverlay::wrap(text).width(30);
        overlay.config.max_height = 15;
        overlay.config.position = DebugPosition::BottomLeft;

        let area = Rect::new(0, 0, 100, 50);
        let panel = overlay.panel_rect(area);

        assert_eq!(panel.x, 0);
        assert_eq!(panel.y, 35);
    }

    #[test]
    fn test_panel_rect_bottom_right() {
        let text = Text::new("test");
        let mut overlay = DebugOverlay::wrap(text).width(25);
        overlay.config.max_height = 10;
        overlay.config.position = DebugPosition::BottomRight;

        let area = Rect::new(0, 0, 100, 50);
        let panel = overlay.panel_rect(area);

        assert_eq!(panel.x, 75);
        assert_eq!(panel.y, 40);
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
