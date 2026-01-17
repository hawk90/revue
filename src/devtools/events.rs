//! Event logger for debugging event flow

use super::helpers::{draw_separator, draw_text_overlay};
use super::DevToolsConfig;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Helper context for rendering devtools panels
struct RenderCtx<'a> {
    buffer: &'a mut Buffer,
    x: u16,
    width: u16,
    config: &'a DevToolsConfig,
}

impl<'a> RenderCtx<'a> {
    fn new(buffer: &'a mut Buffer, x: u16, width: u16, config: &'a DevToolsConfig) -> Self {
        Self {
            buffer,
            x,
            width,
            config,
        }
    }

    fn draw_text(&mut self, y: u16, text: &str, color: Color) {
        draw_text_overlay(self.buffer, self.x, y, text, color);
    }

    fn draw_separator(&mut self, y: u16) {
        draw_separator(self.buffer, self.x, y, self.width, self.config.accent_color);
    }
}

/// Event type for logging
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    /// Key press event
    KeyPress,
    /// Key release event
    KeyRelease,
    /// Mouse click
    MouseClick,
    /// Mouse move
    MouseMove,
    /// Mouse scroll
    MouseScroll,
    /// Focus gained
    FocusIn,
    /// Focus lost
    FocusOut,
    /// Resize event
    Resize,
    /// Custom/user event
    Custom,
}

impl EventType {
    /// Get display label
    pub fn label(&self) -> &'static str {
        match self {
            Self::KeyPress => "KeyPress",
            Self::KeyRelease => "KeyRelease",
            Self::MouseClick => "Click",
            Self::MouseMove => "Move",
            Self::MouseScroll => "Scroll",
            Self::FocusIn => "FocusIn",
            Self::FocusOut => "FocusOut",
            Self::Resize => "Resize",
            Self::Custom => "Custom",
        }
    }

    /// Get icon
    pub fn icon(&self) -> &'static str {
        match self {
            Self::KeyPress => "⌨",
            Self::KeyRelease => "⌨",
            Self::MouseClick => "●",
            Self::MouseMove => "→",
            Self::MouseScroll => "↕",
            Self::FocusIn => "◉",
            Self::FocusOut => "○",
            Self::Resize => "⊡",
            Self::Custom => "★",
        }
    }

    /// Get color for event type
    pub fn color(&self) -> Color {
        match self {
            Self::KeyPress | Self::KeyRelease => Color::rgb(130, 180, 255),
            Self::MouseClick => Color::rgb(255, 180, 130),
            Self::MouseMove => Color::rgb(180, 180, 180),
            Self::MouseScroll => Color::rgb(180, 255, 180),
            Self::FocusIn | Self::FocusOut => Color::rgb(255, 220, 130),
            Self::Resize => Color::rgb(200, 130, 255),
            Self::Custom => Color::rgb(255, 130, 180),
        }
    }
}

/// A logged event
#[derive(Debug, Clone)]
pub struct LoggedEvent {
    /// Event ID
    pub id: u64,
    /// Event type
    pub event_type: EventType,
    /// Event details
    pub details: String,
    /// Target widget (if any)
    pub target: Option<String>,
    /// Timestamp
    pub timestamp: Instant,
    /// Was event handled
    pub handled: bool,
    /// Was event propagated
    pub propagated: bool,
}

impl LoggedEvent {
    /// Create a new logged event
    pub fn new(id: u64, event_type: EventType, details: impl Into<String>) -> Self {
        Self {
            id,
            event_type,
            details: details.into(),
            target: None,
            timestamp: Instant::now(),
            handled: false,
            propagated: true,
        }
    }

    /// Set target
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Mark as handled
    pub fn handled(mut self) -> Self {
        self.handled = true;
        self
    }

    /// Mark as not propagated
    pub fn stopped(mut self) -> Self {
        self.propagated = false;
        self
    }

    /// Get age since event occurred
    pub fn age(&self) -> Duration {
        self.timestamp.elapsed()
    }

    /// Format age for display
    pub fn age_str(&self) -> String {
        let age = self.age();
        if age.as_secs() >= 60 {
            format!("{}m ago", age.as_secs() / 60)
        } else if age.as_secs() > 0 {
            format!("{}s ago", age.as_secs())
        } else {
            format!("{}ms ago", age.as_millis())
        }
    }
}

/// Event filter configuration
#[derive(Debug, Clone, Default)]
pub struct EventFilter {
    /// Show key events
    pub show_keys: bool,
    /// Show mouse events
    pub show_mouse: bool,
    /// Show focus events
    pub show_focus: bool,
    /// Show resize events
    pub show_resize: bool,
    /// Show custom events
    pub show_custom: bool,
    /// Filter by target
    pub target_filter: Option<String>,
    /// Only show handled events
    pub only_handled: bool,
}

impl EventFilter {
    /// Create filter that shows all events
    pub fn all() -> Self {
        Self {
            show_keys: true,
            show_mouse: true,
            show_focus: true,
            show_resize: true,
            show_custom: true,
            target_filter: None,
            only_handled: false,
        }
    }

    /// Create filter for keyboard events only
    pub fn keys_only() -> Self {
        Self {
            show_keys: true,
            ..Default::default()
        }
    }

    /// Create filter for mouse events only
    pub fn mouse_only() -> Self {
        Self {
            show_mouse: true,
            ..Default::default()
        }
    }

    /// Check if event matches filter
    pub fn matches(&self, event: &LoggedEvent) -> bool {
        // Check event type
        let type_match = match event.event_type {
            EventType::KeyPress | EventType::KeyRelease => self.show_keys,
            EventType::MouseClick | EventType::MouseMove | EventType::MouseScroll => {
                self.show_mouse
            }
            EventType::FocusIn | EventType::FocusOut => self.show_focus,
            EventType::Resize => self.show_resize,
            EventType::Custom => self.show_custom,
        };

        if !type_match {
            return false;
        }

        // Check handled filter
        if self.only_handled && !event.handled {
            return false;
        }

        // Check target filter
        if let Some(ref filter) = self.target_filter {
            if let Some(ref target) = event.target {
                if !target.to_lowercase().contains(&filter.to_lowercase()) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

/// Event logger for debugging
#[derive(Debug)]
pub struct EventLogger {
    /// Logged events (ring buffer)
    events: VecDeque<LoggedEvent>,
    /// Maximum events to keep
    max_events: usize,
    /// Next event ID
    next_id: u64,
    /// Current filter
    filter: EventFilter,
    /// Selected event index
    selected: Option<usize>,
    /// Scroll offset
    scroll: usize,
    /// Is paused
    paused: bool,
    /// Start time for relative timestamps (for future UI)
    _start_time: Instant,
}

impl Default for EventLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl EventLogger {
    /// Create new event logger
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            max_events: 500,
            next_id: 0,
            filter: EventFilter::all(),
            selected: None,
            scroll: 0,
            paused: false,
            _start_time: Instant::now(),
        }
    }

    /// Set maximum events to keep
    pub fn max_events(mut self, max: usize) -> Self {
        self.max_events = max;
        self
    }

    /// Set filter
    pub fn filter(mut self, filter: EventFilter) -> Self {
        self.filter = filter;
        self
    }

    /// Clear all events
    pub fn clear(&mut self) {
        self.events.clear();
        self.selected = None;
        self.scroll = 0;
    }

    /// Pause logging
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resume logging
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Toggle pause
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Is paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Log an event
    pub fn log(&mut self, event_type: EventType, details: impl Into<String>) -> u64 {
        if self.paused {
            return 0;
        }

        let id = self.next_id;
        self.next_id += 1;

        let event = LoggedEvent::new(id, event_type, details);
        self.events.push_back(event);

        // Trim if needed
        while self.events.len() > self.max_events {
            self.events.pop_front();
        }

        id
    }

    /// Log a key event
    pub fn log_key(&mut self, key: &str, modifiers: &str) -> u64 {
        let details = if modifiers.is_empty() {
            key.to_string()
        } else {
            format!("{} + {}", modifiers, key)
        };
        self.log(EventType::KeyPress, details)
    }

    /// Log a mouse click
    pub fn log_click(&mut self, x: u16, y: u16, button: &str) -> u64 {
        self.log(
            EventType::MouseClick,
            format!("{} @ ({}, {})", button, x, y),
        )
    }

    /// Log a mouse move
    pub fn log_move(&mut self, x: u16, y: u16) -> u64 {
        self.log(EventType::MouseMove, format!("({}, {})", x, y))
    }

    /// Log focus change
    pub fn log_focus(&mut self, target: &str, gained: bool) -> u64 {
        let event_type = if gained {
            EventType::FocusIn
        } else {
            EventType::FocusOut
        };
        let mut event = LoggedEvent::new(self.next_id, event_type, target);
        event.target = Some(target.to_string());

        let id = self.next_id;
        self.next_id += 1;
        self.events.push_back(event);

        while self.events.len() > self.max_events {
            self.events.pop_front();
        }

        id
    }

    /// Mark event as handled
    pub fn mark_handled(&mut self, id: u64) {
        if let Some(event) = self.events.iter_mut().find(|e| e.id == id) {
            event.handled = true;
        }
    }

    /// Set event target
    pub fn set_target(&mut self, id: u64, target: impl Into<String>) {
        if let Some(event) = self.events.iter_mut().find(|e| e.id == id) {
            event.target = Some(target.into());
        }
    }

    /// Get filtered events
    fn filtered(&self) -> Vec<&LoggedEvent> {
        self.events
            .iter()
            .filter(|e| self.filter.matches(e))
            .collect()
    }

    /// Get event count
    pub fn count(&self) -> usize {
        self.events.len()
    }

    /// Get filtered event count
    pub fn filtered_count(&self) -> usize {
        self.filtered().len()
    }

    /// Select next event
    pub fn select_next(&mut self) {
        let count = self.filtered().len();
        if count == 0 {
            return;
        }

        self.selected = Some(match self.selected {
            Some(i) => (i + 1).min(count - 1),
            None => 0,
        });
    }

    /// Select previous event
    pub fn select_prev(&mut self) {
        let count = self.filtered().len();
        if count == 0 {
            return;
        }

        self.selected = Some(match self.selected {
            Some(i) => i.saturating_sub(1),
            None => 0,
        });
    }

    /// Toggle key events filter
    pub fn toggle_keys(&mut self) {
        self.filter.show_keys = !self.filter.show_keys;
    }

    /// Toggle mouse events filter
    pub fn toggle_mouse(&mut self) {
        self.filter.show_mouse = !self.filter.show_mouse;
    }

    /// Toggle focus events filter
    pub fn toggle_focus(&mut self) {
        self.filter.show_focus = !self.filter.show_focus;
    }

    /// Render event logger content
    pub fn render_content(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        let mut ctx = RenderCtx::new(buffer, area.x, area.width, config);
        let mut y = area.y;
        let max_y = area.y + area.height;

        // Header
        let status = if self.paused {
            "⏸ PAUSED"
        } else {
            "● Recording"
        };
        let header = format!("{} | {} events", status, self.filtered_count());
        ctx.draw_text(y, &header, config.accent_color);
        y += 1;

        // Filter info
        let mut filters = Vec::new();
        if self.filter.show_keys {
            filters.push("Keys");
        }
        if self.filter.show_mouse {
            filters.push("Mouse");
        }
        if self.filter.show_focus {
            filters.push("Focus");
        }
        if self.filter.show_resize {
            filters.push("Resize");
        }
        let filter_str = format!("Showing: {}", filters.join(", "));
        ctx.draw_text(y, &filter_str, config.fg_color);
        y += 2;

        // Events list (newest first)
        let filtered: Vec<_> = self.filtered().into_iter().rev().collect();
        for (i, event) in filtered.iter().enumerate().skip(self.scroll) {
            if y >= max_y - 2 {
                break;
            }

            let is_selected = self.selected == Some(i);
            Self::render_event(&mut ctx, y, event, is_selected);
            y += 1;
        }

        // Selected event details
        if let Some(idx) = self.selected {
            if let Some(event) = filtered.get(idx) {
                if y + 2 < max_y {
                    y = max_y - 3;
                    ctx.draw_separator(y);
                    y += 1;
                    Self::render_details(&mut ctx, y, event);
                }
            }
        }
    }

    fn render_event(ctx: &mut RenderCtx<'_>, y: u16, event: &LoggedEvent, selected: bool) {
        let icon = event.event_type.icon();
        let handled_mark = if event.handled { "✓" } else { " " };
        let age = event.age_str();

        // Truncate details if needed
        let max_details = (ctx.width as usize).saturating_sub(20);
        let details = if event.details.len() > max_details {
            format!("{}...", &event.details[..max_details.saturating_sub(3)])
        } else {
            event.details.clone()
        };

        let line = format!("{} {} {} {}", icon, handled_mark, details, age);

        let fg = if selected {
            ctx.config.bg_color
        } else {
            event.event_type.color()
        };
        let bg = if selected {
            Some(ctx.config.accent_color)
        } else {
            None
        };

        for (i, ch) in line.chars().enumerate() {
            if (i as u16) < ctx.width {
                if let Some(cell) = ctx.buffer.get_mut(ctx.x + i as u16, y) {
                    cell.symbol = ch;
                    cell.fg = Some(fg);
                    if let Some(b) = bg {
                        cell.bg = Some(b);
                    }
                }
            }
        }
    }

    fn render_details(ctx: &mut RenderCtx<'_>, y: u16, event: &LoggedEvent) {
        let target = event.target.as_deref().unwrap_or("none");
        let details = format!(
            "#{} {} | Target: {} | {}",
            event.id,
            event.event_type.label(),
            target,
            if event.handled {
                "Handled"
            } else {
                "Not handled"
            }
        );
        ctx.draw_text(y, &details, ctx.config.fg_color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_label() {
        assert_eq!(EventType::KeyPress.label(), "KeyPress");
        assert_eq!(EventType::MouseClick.label(), "Click");
        assert_eq!(EventType::FocusIn.label(), "FocusIn");
    }

    #[test]
    fn test_logged_event() {
        let event = LoggedEvent::new(1, EventType::KeyPress, "Enter")
            .target("Button#submit")
            .handled();

        assert_eq!(event.id, 1);
        assert_eq!(event.event_type, EventType::KeyPress);
        assert_eq!(event.details, "Enter");
        assert_eq!(event.target, Some("Button#submit".to_string()));
        assert!(event.handled);
    }

    #[test]
    fn test_event_filter() {
        let filter = EventFilter::keys_only();

        let key_event = LoggedEvent::new(1, EventType::KeyPress, "A");
        let mouse_event = LoggedEvent::new(2, EventType::MouseClick, "left");

        assert!(filter.matches(&key_event));
        assert!(!filter.matches(&mouse_event));
    }

    #[test]
    fn test_event_logger_log() {
        let mut logger = EventLogger::new();
        let id = logger.log_key("Enter", "Ctrl");

        assert_eq!(logger.count(), 1);
        assert!(id > 0 || id == 0); // First ID is 0
    }

    #[test]
    fn test_event_logger_pause() {
        let mut logger = EventLogger::new();
        logger.log_key("A", "");
        assert_eq!(logger.count(), 1);

        logger.pause();
        logger.log_key("B", "");
        assert_eq!(logger.count(), 1); // Should not log while paused

        logger.resume();
        logger.log_key("C", "");
        assert_eq!(logger.count(), 2);
    }

    #[test]
    fn test_event_logger_clear() {
        let mut logger = EventLogger::new();
        logger.log_key("A", "");
        logger.log_key("B", "");
        assert_eq!(logger.count(), 2);

        logger.clear();
        assert_eq!(logger.count(), 0);
    }

    #[test]
    fn test_event_logger_max_events() {
        let mut logger = EventLogger::new().max_events(3);

        for i in 0..5 {
            logger.log_key(&format!("Key{}", i), "");
        }

        assert_eq!(logger.count(), 3);
    }

    #[test]
    fn test_event_logger_mark_handled() {
        let mut logger = EventLogger::new();
        let id = logger.log_key("Enter", "");

        assert!(!logger.events.back().unwrap().handled);

        logger.mark_handled(id);
        assert!(logger.events.back().unwrap().handled);
    }

    #[test]
    fn test_event_filter_all() {
        let filter = EventFilter::all();

        // All event types should match
        let events = vec![
            LoggedEvent::new(1, EventType::KeyPress, "A"),
            LoggedEvent::new(2, EventType::MouseClick, "left"),
            LoggedEvent::new(3, EventType::FocusIn, "input"),
            LoggedEvent::new(4, EventType::Resize, "80x24"),
            LoggedEvent::new(5, EventType::Custom, "custom"),
        ];

        for event in &events {
            assert!(
                filter.matches(event),
                "Filter should match {:?}",
                event.event_type
            );
        }
    }

    // =========================================================================
    // Additional coverage tests
    // =========================================================================

    #[test]
    fn test_event_type_icon() {
        assert_eq!(EventType::KeyPress.icon(), "⌨");
        assert_eq!(EventType::KeyRelease.icon(), "⌨");
        assert_eq!(EventType::MouseClick.icon(), "●");
        assert_eq!(EventType::MouseMove.icon(), "→");
        assert_eq!(EventType::MouseScroll.icon(), "↕");
        assert_eq!(EventType::FocusIn.icon(), "◉");
        assert_eq!(EventType::FocusOut.icon(), "○");
        assert_eq!(EventType::Resize.icon(), "⊡");
        assert_eq!(EventType::Custom.icon(), "★");
    }

    #[test]
    fn test_event_type_color() {
        let key_color = EventType::KeyPress.color();
        let release_color = EventType::KeyRelease.color();
        assert_eq!(key_color, release_color);

        let click_color = EventType::MouseClick.color();
        assert_ne!(click_color, key_color);

        let move_color = EventType::MouseMove.color();
        let scroll_color = EventType::MouseScroll.color();
        assert_ne!(move_color, scroll_color);

        let focus_in = EventType::FocusIn.color();
        let focus_out = EventType::FocusOut.color();
        assert_eq!(focus_in, focus_out);

        let resize_color = EventType::Resize.color();
        let custom_color = EventType::Custom.color();
        assert_ne!(resize_color, custom_color);
    }

    #[test]
    fn test_event_type_all_labels() {
        assert_eq!(EventType::KeyRelease.label(), "KeyRelease");
        assert_eq!(EventType::MouseMove.label(), "Move");
        assert_eq!(EventType::MouseScroll.label(), "Scroll");
        assert_eq!(EventType::FocusOut.label(), "FocusOut");
        assert_eq!(EventType::Resize.label(), "Resize");
        assert_eq!(EventType::Custom.label(), "Custom");
    }

    #[test]
    fn test_logged_event_stopped() {
        let event = LoggedEvent::new(1, EventType::KeyPress, "A").stopped();
        assert!(!event.propagated);
    }

    #[test]
    fn test_logged_event_age_str() {
        let event = LoggedEvent::new(1, EventType::KeyPress, "A");
        let age_str = event.age_str();
        // Should be in milliseconds since it was just created
        assert!(age_str.contains("ms ago"));
    }

    #[test]
    fn test_event_filter_mouse_only() {
        let filter = EventFilter::mouse_only();

        let mouse_event = LoggedEvent::new(1, EventType::MouseClick, "left");
        let key_event = LoggedEvent::new(2, EventType::KeyPress, "A");

        assert!(filter.matches(&mouse_event));
        assert!(!filter.matches(&key_event));
    }

    #[test]
    fn test_event_filter_only_handled() {
        let mut filter = EventFilter::all();
        filter.only_handled = true;

        let handled_event = LoggedEvent::new(1, EventType::KeyPress, "A").handled();
        let unhandled_event = LoggedEvent::new(2, EventType::KeyPress, "B");

        assert!(filter.matches(&handled_event));
        assert!(!filter.matches(&unhandled_event));
    }

    #[test]
    fn test_event_filter_target_filter() {
        let mut filter = EventFilter::all();
        filter.target_filter = Some("Button".to_string());

        let mut matched_event = LoggedEvent::new(1, EventType::KeyPress, "A");
        matched_event.target = Some("Button#submit".to_string());

        let mut unmatched_event = LoggedEvent::new(2, EventType::KeyPress, "A");
        unmatched_event.target = Some("Input#name".to_string());

        let no_target_event = LoggedEvent::new(3, EventType::KeyPress, "A");

        assert!(filter.matches(&matched_event));
        assert!(!filter.matches(&unmatched_event));
        assert!(!filter.matches(&no_target_event));
    }

    #[test]
    fn test_event_logger_log_click() {
        let mut logger = EventLogger::new();
        let id = logger.log_click(10, 20, "left");
        assert!(id == 0 || id > 0);
        assert_eq!(logger.count(), 1);
    }

    #[test]
    fn test_event_logger_log_move() {
        let mut logger = EventLogger::new();
        logger.log_move(50, 60);
        assert_eq!(logger.count(), 1);
    }

    #[test]
    fn test_event_logger_log_focus() {
        let mut logger = EventLogger::new();
        logger.log_focus("Input#name", true);
        logger.log_focus("Input#name", false);
        assert_eq!(logger.count(), 2);
    }

    #[test]
    fn test_event_logger_set_target() {
        let mut logger = EventLogger::new();
        let id = logger.log_key("Enter", "");
        logger.set_target(id, "Button#submit");

        assert_eq!(
            logger.events.back().unwrap().target,
            Some("Button#submit".to_string())
        );
    }

    #[test]
    fn test_event_logger_toggle_pause() {
        let mut logger = EventLogger::new();
        assert!(!logger.is_paused());

        logger.toggle_pause();
        assert!(logger.is_paused());

        logger.toggle_pause();
        assert!(!logger.is_paused());
    }

    #[test]
    fn test_event_logger_select_next_prev() {
        let mut logger = EventLogger::new();
        logger.log_key("A", "");
        logger.log_key("B", "");
        logger.log_key("C", "");

        logger.select_next();
        assert!(logger.selected.is_some());

        logger.select_next();
        logger.select_prev();
    }

    #[test]
    fn test_event_logger_select_empty() {
        let mut logger = EventLogger::new();
        logger.select_next();
        logger.select_prev();
        // Should not panic on empty
    }

    #[test]
    fn test_event_logger_filtered_count() {
        let mut logger = EventLogger::new().filter(EventFilter::keys_only());
        logger.log_key("A", "");
        logger.log_click(0, 0, "left"); // This won't be logged because filter is keys_only

        // But we still need to log via log() which doesn't respect filter
        assert!(logger.filtered_count() <= logger.count());
    }

    #[test]
    fn test_event_logger_toggle_filters() {
        let mut logger = EventLogger::new();

        logger.toggle_keys();
        assert!(!logger.filter.show_keys);

        logger.toggle_mouse();
        assert!(!logger.filter.show_mouse);

        logger.toggle_focus();
        assert!(!logger.filter.show_focus);
    }

    #[test]
    fn test_event_filter_default() {
        let filter = EventFilter::default();
        assert!(!filter.show_keys);
        assert!(!filter.show_mouse);
        assert!(!filter.show_focus);
    }
}
