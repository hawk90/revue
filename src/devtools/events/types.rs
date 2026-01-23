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
