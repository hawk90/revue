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

use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::utils::draw_text_overlay;
use crate::widget::{RenderContext, View};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

// =============================================================================
// Performance Metrics
// =============================================================================

/// Performance metrics tracker
#[derive(Debug, Clone, Default)]
pub struct PerfMetrics {
    /// Frame times (last N frames)
    frame_times: VecDeque<Duration>,
    /// Last frame start time
    last_frame_start: Option<Instant>,
    /// Layout times
    layout_times: VecDeque<Duration>,
    /// Render times
    render_times: VecDeque<Duration>,
    /// Maximum samples to keep
    max_samples: usize,
}

impl PerfMetrics {
    /// Create new metrics tracker
    pub fn new() -> Self {
        Self {
            frame_times: VecDeque::new(),
            last_frame_start: None,
            layout_times: VecDeque::new(),
            render_times: VecDeque::new(),
            max_samples: 60,
        }
    }

    /// Start a new frame
    pub fn start_frame(&mut self) {
        let now = Instant::now();
        if let Some(last) = self.last_frame_start {
            let elapsed = now.duration_since(last);
            self.frame_times.push_back(elapsed);
            if self.frame_times.len() > self.max_samples {
                self.frame_times.pop_front();
            }
        }
        self.last_frame_start = Some(now);
    }

    /// Record layout time
    pub fn record_layout(&mut self, duration: Duration) {
        self.layout_times.push_back(duration);
        if self.layout_times.len() > self.max_samples {
            self.layout_times.pop_front();
        }
    }

    /// Record render time
    pub fn record_render(&mut self, duration: Duration) {
        self.render_times.push_back(duration);
        if self.render_times.len() > self.max_samples {
            self.render_times.pop_front();
        }
    }

    /// Get average FPS
    pub fn fps(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let total: Duration = self.frame_times.iter().sum();
        let avg = total.as_secs_f64() / self.frame_times.len() as f64;
        if avg > 0.0 {
            1.0 / avg
        } else {
            0.0
        }
    }

    /// Get average frame time in ms
    pub fn avg_frame_time_ms(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let total: Duration = self.frame_times.iter().sum();
        total.as_secs_f64() * 1000.0 / self.frame_times.len() as f64
    }

    /// Get average layout time in ms
    pub fn avg_layout_time_ms(&self) -> f64 {
        if self.layout_times.is_empty() {
            return 0.0;
        }
        let total: Duration = self.layout_times.iter().sum();
        total.as_secs_f64() * 1000.0 / self.layout_times.len() as f64
    }

    /// Get average render time in ms
    pub fn avg_render_time_ms(&self) -> f64 {
        if self.render_times.is_empty() {
            return 0.0;
        }
        let total: Duration = self.render_times.iter().sum();
        total.as_secs_f64() * 1000.0 / self.render_times.len() as f64
    }

    /// Reset all metrics
    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.layout_times.clear();
        self.render_times.clear();
        self.last_frame_start = None;
    }
}

// =============================================================================
// Event Log
// =============================================================================

/// Debug event types
#[derive(Debug, Clone)]
pub enum DebugEvent {
    /// Key press
    KeyPress(String),
    /// Mouse event
    Mouse(String),
    /// State change
    StateChange(String),
    /// Custom event
    Custom(String),
}

/// Event log for debugging
#[derive(Debug, Clone, Default)]
pub struct EventLog {
    /// Logged events
    events: VecDeque<(Instant, DebugEvent)>,
    /// Max events to keep
    max_events: usize,
}

impl EventLog {
    /// Create new event log
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            max_events: 50,
        }
    }

    /// Log an event
    pub fn log(&mut self, event: DebugEvent) {
        self.events.push_back((Instant::now(), event));
        if self.events.len() > self.max_events {
            self.events.pop_front();
        }
    }

    /// Get recent events
    pub fn recent(&self, count: usize) -> impl Iterator<Item = &(Instant, DebugEvent)> {
        self.events.iter().rev().take(count)
    }

    /// Clear all events
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

// =============================================================================
// Widget Info
// =============================================================================

/// Information about a widget for debugging
#[derive(Debug, Clone)]
pub struct WidgetInfo {
    /// Widget type name
    pub type_name: String,
    /// Widget ID (if any)
    pub id: Option<String>,
    /// CSS classes
    pub classes: Vec<String>,
    /// Bounding rect
    pub rect: Rect,
    /// Depth in tree
    pub depth: usize,
    /// Is focused
    pub focused: bool,
    /// Is hovered
    pub hovered: bool,
}

impl WidgetInfo {
    /// Create new widget info
    pub fn new(type_name: impl Into<String>) -> Self {
        Self {
            type_name: type_name.into(),
            id: None,
            classes: Vec::new(),
            rect: Rect::default(),
            depth: 0,
            focused: false,
            hovered: false,
        }
    }

    /// Set widget ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add CSS class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Set bounding rect
    pub fn rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    /// Set depth
    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    /// Format as tree line
    pub fn tree_line(&self) -> String {
        let indent = "  ".repeat(self.depth);
        let mut line = format!("{}{}", indent, self.type_name);

        if let Some(ref id) = self.id {
            line.push_str(&format!(" #{}", id));
        }

        for class in &self.classes {
            line.push_str(&format!(" .{}", class));
        }

        if self.focused {
            line.push_str(" [focused]");
        }

        if self.hovered {
            line.push_str(" [hover]");
        }

        line
    }
}

// =============================================================================
// Debug Panel
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
            bg_color: Color::rgb(30, 30, 30),
            fg_color: Color::rgb(200, 200, 200),
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
#[cfg(test)]
mod tests;
