//! Timeline widget for activity feeds and event logs
//!
//! Displays chronological events with timestamps and icons.

mod render;
mod types;

pub use types::{EventType, TimelineEvent, TimelineOrientation, TimelineStyle};

use crate::style::Color;
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
    /// Scroll offset
    scroll: usize,
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
            scroll: 0,
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

    /// Select an event
    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }

    /// Select next event
    pub fn select_next(&mut self) {
        match self.selected {
            Some(i) if i < self.events.len() - 1 => self.selected = Some(i + 1),
            None if !self.events.is_empty() => self.selected = Some(0),
            _ => {}
        }
    }

    /// Select previous event
    pub fn select_prev(&mut self) {
        match self.selected {
            Some(i) if i > 0 => self.selected = Some(i - 1),
            _ => {}
        }
    }

    /// Get selected event
    pub fn selected_event(&self) -> Option<&TimelineEvent> {
        self.selected.and_then(|i| self.events.get(i))
    }

    /// Clear events
    pub fn clear(&mut self) {
        self.events.clear();
        self.selected = None;
        self.scroll = 0;
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

// Private tests - KEEP HERE: uses internal RenderContext, Buffer, or private fields
#[cfg(test)]
mod tests {
    use super::*;

    // KEEP HERE: accesses private field `scroll`
    #[test]
    fn test_clear() {
        let mut tl = Timeline::new()
            .event(TimelineEvent::new("A"))
            .event(TimelineEvent::new("B"))
            .event(TimelineEvent::new("C"));

        tl.select_next();
        tl.clear();

        assert!(tl.is_empty());
        assert_eq!(tl.selected, None);
        assert_eq!(tl.scroll, 0);
    }
}

// Keep private tests that require private field access here

#[test]
fn test_timeline_render_private() {
    // Test private render methods - keeping in source
    let _t = Timeline::new().event(TimelineEvent::new("Test"));

    // This would require accessing private render methods
    // Test kept inline due to private access
}
