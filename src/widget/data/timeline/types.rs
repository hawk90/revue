//! Timeline event, event type, orientation and style types

use crate::style::Color;

/// Timeline event type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EventType {
    /// Informational event (default)
    #[default]
    Info,
    /// Success/completed event
    Success,
    /// Warning event
    Warning,
    /// Error/failed event
    Error,
    /// Custom event with icon
    Custom(char),
}

impl EventType {
    /// Get icon for event type
    pub fn icon(&self) -> char {
        match self {
            EventType::Info => '●',
            EventType::Success => '✓',
            EventType::Warning => '⚠',
            EventType::Error => '✗',
            EventType::Custom(c) => *c,
        }
    }

    /// Get color for event type
    pub fn color(&self) -> Color {
        match self {
            EventType::Info => Color::CYAN,
            EventType::Success => Color::GREEN,
            EventType::Warning => Color::YELLOW,
            EventType::Error => Color::RED,
            EventType::Custom(_) => Color::WHITE,
        }
    }
}

/// A timeline event
#[derive(Clone, Debug)]
pub struct TimelineEvent {
    /// Event title
    pub title: String,
    /// Event description
    pub description: Option<String>,
    /// Timestamp display
    pub timestamp: Option<String>,
    /// Event type
    pub event_type: EventType,
    /// Custom color override
    pub color: Option<Color>,
    /// Additional metadata
    pub metadata: Vec<(String, String)>,
}

impl TimelineEvent {
    /// Create a new event
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            timestamp: None,
            event_type: EventType::Info,
            color: None,
            metadata: Vec::new(),
        }
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set timestamp
    pub fn timestamp(mut self, ts: impl Into<String>) -> Self {
        self.timestamp = Some(ts.into());
        self
    }

    /// Set event type
    pub fn event_type(mut self, t: EventType) -> Self {
        self.event_type = t;
        self
    }

    /// Set as success event
    pub fn success(mut self) -> Self {
        self.event_type = EventType::Success;
        self
    }

    /// Set as warning event
    pub fn warning(mut self) -> Self {
        self.event_type = EventType::Warning;
        self
    }

    /// Set as error event
    pub fn error(mut self) -> Self {
        self.event_type = EventType::Error;
        self
    }

    /// Set custom color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Add metadata
    pub fn meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.push((key.into(), value.into()));
        self
    }

    /// Get display color
    pub fn display_color(&self) -> Color {
        self.color.unwrap_or_else(|| self.event_type.color())
    }
}

/// Timeline orientation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimelineOrientation {
    /// Vertical timeline (events stacked)
    #[default]
    Vertical,
    /// Horizontal timeline (events side by side)
    Horizontal,
}

/// Timeline style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimelineStyle {
    /// Simple line with dots
    #[default]
    Line,
    /// Connected boxes
    Boxed,
    /// Minimal (no line)
    Minimal,
    /// Alternating sides
    Alternating,
}
