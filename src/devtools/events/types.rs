//! Event types

use crate::style::Color;
use std::time::{Duration, Instant};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_label() {
        assert_eq!(EventType::KeyPress.label(), "KeyPress");
        assert_eq!(EventType::KeyRelease.label(), "KeyRelease");
        assert_eq!(EventType::MouseClick.label(), "Click");
        assert_eq!(EventType::MouseMove.label(), "Move");
        assert_eq!(EventType::MouseScroll.label(), "Scroll");
        assert_eq!(EventType::FocusIn.label(), "FocusIn");
        assert_eq!(EventType::FocusOut.label(), "FocusOut");
        assert_eq!(EventType::Resize.label(), "Resize");
        assert_eq!(EventType::Custom.label(), "Custom");
    }

    #[test]
    fn test_event_type_icon() {
        assert!(!EventType::KeyPress.icon().is_empty());
        assert!(!EventType::MouseClick.icon().is_empty());
        assert!(!EventType::FocusIn.icon().is_empty());
    }

    #[test]
    fn test_event_type_color() {
        let _ = EventType::KeyPress.color();
        let _ = EventType::MouseClick.color();
        let _ = EventType::FocusIn.color();
        let _ = EventType::Resize.color();
        let _ = EventType::Custom.color();
    }

    #[test]
    fn test_logged_event_new() {
        let event = LoggedEvent::new(1, EventType::KeyPress, "Pressed 'a'");
        assert_eq!(event.id, 1);
        assert_eq!(event.event_type, EventType::KeyPress);
        assert_eq!(event.details, "Pressed 'a'");
        assert_eq!(event.target, None);
        assert!(!event.handled);
        assert!(event.propagated);
    }

    #[test]
    fn test_logged_event_builder() {
        let event = LoggedEvent::new(1, EventType::KeyPress, "Pressed 'a'")
            .target("Button")
            .handled()
            .stopped();

        assert_eq!(event.target, Some("Button".to_string()));
        assert!(event.handled);
        assert!(!event.propagated);
    }

    #[test]
    fn test_logged_event_public_fields() {
        let mut event = LoggedEvent::new(1, EventType::KeyPress, "test");
        event.id = 5;
        event.details = "modified".to_string();
        event.handled = true;

        assert_eq!(event.id, 5);
        assert_eq!(event.details, "modified");
        assert!(event.handled);
    }

    #[test]
    fn test_logged_event_age() {
        let event = LoggedEvent::new(1, EventType::KeyPress, "test");
        std::thread::sleep(std::time::Duration::from_millis(10));
        let age = event.age();
        assert!(age.as_millis() >= 10);
    }

    #[test]
    fn test_logged_event_age_str() {
        let event = LoggedEvent::new(1, EventType::KeyPress, "test");
        let age_str = event.age_str();
        // Should be in milliseconds since just created
        assert!(age_str.contains("ms ago"));
    }

    #[test]
    fn test_event_filter_default() {
        let filter = EventFilter::default();
        assert!(!filter.show_keys);
        assert!(!filter.show_mouse);
        assert!(!filter.show_focus);
        assert!(!filter.show_resize);
        assert!(!filter.show_custom);
        assert_eq!(filter.target_filter, None);
        assert!(!filter.only_handled);
    }

    #[test]
    fn test_event_filter_all() {
        let filter = EventFilter::all();
        assert!(filter.show_keys);
        assert!(filter.show_mouse);
        assert!(filter.show_focus);
        assert!(filter.show_resize);
        assert!(filter.show_custom);
        assert_eq!(filter.target_filter, None);
        assert!(!filter.only_handled);
    }

    #[test]
    fn test_event_filter_keys_only() {
        let filter = EventFilter::keys_only();
        assert!(filter.show_keys);
        assert!(!filter.show_mouse);
        assert!(!filter.show_focus);
        assert!(!filter.show_resize);
        assert!(!filter.show_custom);
    }

    #[test]
    fn test_event_filter_mouse_only() {
        let filter = EventFilter::mouse_only();
        assert!(!filter.show_keys);
        assert!(filter.show_mouse);
        assert!(!filter.show_focus);
        assert!(!filter.show_resize);
        assert!(!filter.show_custom);
    }

    #[test]
    fn test_event_filter_matches_key_event() {
        let filter = EventFilter::keys_only();
        let event = LoggedEvent::new(1, EventType::KeyPress, "test");
        assert!(filter.matches(&event));

        let mouse_event = LoggedEvent::new(2, EventType::MouseClick, "test");
        assert!(!filter.matches(&mouse_event));
    }

    #[test]
    fn test_event_filter_matches_mouse_event() {
        let filter = EventFilter::mouse_only();
        let event = LoggedEvent::new(1, EventType::MouseClick, "test");
        assert!(filter.matches(&event));

        let key_event = LoggedEvent::new(2, EventType::KeyPress, "test");
        assert!(!filter.matches(&key_event));
    }

    #[test]
    fn test_event_filter_only_handled() {
        let filter = EventFilter {
            show_keys: true,
            only_handled: true,
            ..Default::default()
        };

        let handled_event = LoggedEvent::new(1, EventType::KeyPress, "test").handled();
        assert!(filter.matches(&handled_event));

        let unhandled_event = LoggedEvent::new(2, EventType::KeyPress, "test");
        assert!(!filter.matches(&unhandled_event));
    }

    #[test]
    fn test_event_filter_target() {
        let filter = EventFilter {
            show_keys: true,
            target_filter: Some("Button".to_string()),
            ..Default::default()
        };

        let matching_event = LoggedEvent::new(1, EventType::KeyPress, "test").target("MyButton");
        assert!(filter.matches(&matching_event));

        let non_matching_event =
            LoggedEvent::new(2, EventType::KeyPress, "test").target("TextField");
        assert!(!filter.matches(&non_matching_event));

        let no_target_event = LoggedEvent::new(3, EventType::KeyPress, "test");
        assert!(!filter.matches(&no_target_event));
    }

    #[test]
    fn test_event_filter_target_case_insensitive() {
        let filter = EventFilter {
            show_keys: true,
            target_filter: Some("button".to_string()),
            ..Default::default()
        };

        let event = LoggedEvent::new(1, EventType::KeyPress, "test").target("MyButton");
        assert!(filter.matches(&event));
    }

    #[test]
    fn test_event_filter_public_fields() {
        let mut filter = EventFilter::default();
        filter.show_keys = true;
        filter.show_mouse = true;
        filter.only_handled = true;

        assert!(filter.show_keys);
        assert!(filter.show_mouse);
        assert!(filter.only_handled);
    }
}
