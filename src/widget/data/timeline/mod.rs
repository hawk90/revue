//! Timeline widget for activity feeds and event logs
//!
//! Displays chronological events with timestamps and icons.

mod horizontal;
mod selection;
mod types;
mod vertical;
mod view;

pub use types::{EventType, TimelineEvent, TimelineOrientation, TimelineStyle};

use crate::style::Color;
use crate::utils::ScrollState;
use crate::widget::theme::{DARK_GRAY, LIGHT_GRAY, MUTED_TEXT};
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Timeline widget
#[derive(Clone)]
pub struct Timeline {
    /// Events
    events: Vec<TimelineEvent>,
    /// Orientation
    orientation: TimelineOrientation,
    /// Style
    style: TimelineStyle,
    /// Selected event index
    selected: Option<usize>,
    /// Where the view is scrolled to: in rows when vertical, in events
    /// when horizontal
    scroll: ScrollState,
    /// Show timestamps
    show_timestamps: bool,
    /// Show descriptions
    show_descriptions: bool,
    /// Line color
    line_color: Color,
    /// Timestamp color
    timestamp_color: Color,
    /// Title color
    title_color: Color,
    /// Description color
    desc_color: Color,
    /// Widget props for CSS integration
    props: WidgetProps,
}

impl Timeline {
    /// Create a new timeline
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            orientation: TimelineOrientation::Vertical,
            style: TimelineStyle::Line,
            selected: None,
            scroll: ScrollState::new(),
            show_timestamps: true,
            show_descriptions: true,
            line_color: DARK_GRAY,
            timestamp_color: LIGHT_GRAY,
            title_color: Color::WHITE,
            desc_color: MUTED_TEXT,
            props: WidgetProps::new(),
        }
    }

    /// Add an event
    pub fn event(mut self, event: TimelineEvent) -> Self {
        self.events.push(event);
        self
    }

    /// Add events
    pub fn events(mut self, events: Vec<TimelineEvent>) -> Self {
        self.events.extend(events);
        self
    }

    /// Set orientation
    pub fn orientation(mut self, orientation: TimelineOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set vertical orientation
    pub fn vertical(mut self) -> Self {
        self.orientation = TimelineOrientation::Vertical;
        self
    }

    /// Set horizontal orientation
    pub fn horizontal(mut self) -> Self {
        self.orientation = TimelineOrientation::Horizontal;
        self
    }

    /// Set style
    pub fn style(mut self, style: TimelineStyle) -> Self {
        self.style = style;
        self
    }

    /// Show/hide timestamps
    pub fn timestamps(mut self, show: bool) -> Self {
        self.show_timestamps = show;
        self
    }

    /// Show/hide descriptions
    pub fn descriptions(mut self, show: bool) -> Self {
        self.show_descriptions = show;
        self
    }

    /// Set line color
    pub fn line_color(mut self, color: Color) -> Self {
        self.line_color = color;
        self
    }

    /// Clear events
    pub fn clear(&mut self) {
        self.events.clear();
        self.selected = None;
        self.scroll.scroll_to_top();
    }

    /// Add event dynamically
    pub fn push(&mut self, event: TimelineEvent) {
        self.events.push(event);
    }

    /// Get event count
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}
impl_styled_view!(Timeline);
impl_props_builders!(Timeline);

// Helper functions

/// Create a new timeline widget
pub fn timeline() -> Timeline {
    Timeline::new()
}

/// Create a new timeline event with title
pub fn timeline_event(title: impl Into<String>) -> TimelineEvent {
    TimelineEvent::new(title)
}
