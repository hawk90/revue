//! Timeline widget for activity feeds and event logs
//!
//! Displays chronological events with timestamps and icons.

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

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

/// Timeline widget
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
            line_color: Color::rgb(80, 80, 80),
            timestamp_color: Color::rgb(150, 150, 150),
            title_color: Color::WHITE,
            desc_color: Color::rgb(180, 180, 180),
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

impl View for Timeline {
    crate::impl_view_meta!("Timeline");

    fn render(&self, ctx: &mut RenderContext) {
        match self.orientation {
            TimelineOrientation::Vertical => self.render_vertical(ctx),
            TimelineOrientation::Horizontal => self.render_horizontal(ctx),
        }
    }
}

impl_styled_view!(Timeline);
impl_props_builders!(Timeline);

impl Timeline {
    fn render_vertical(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if self.events.is_empty() || area.height < 2 {
            return;
        }

        let timestamp_width = if self.show_timestamps { 12 } else { 0 };
        let icon_x = area.x + timestamp_width;
        let content_x = icon_x + 3;
        let content_width = area.width.saturating_sub(timestamp_width + 3);

        let mut y = area.y;

        for (i, event) in self.events.iter().enumerate().skip(self.scroll) {
            if y >= area.y + area.height {
                break;
            }

            let is_selected = self.selected == Some(i);
            let color = event.display_color();

            // Draw timestamp
            if self.show_timestamps {
                if let Some(ref ts) = event.timestamp {
                    for (j, ch) in ts.chars().take(timestamp_width as usize - 1).enumerate() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.timestamp_color);
                        ctx.buffer.set(area.x + j as u16, y, cell);
                    }
                }
            }

            // Draw icon
            let icon = event.event_type.icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(color);
            if is_selected {
                icon_cell.modifier |= Modifier::BOLD;
            }
            ctx.buffer.set(icon_x, y, icon_cell);

            // Draw line (except for last item)
            if i < self.events.len() - 1 && self.style != TimelineStyle::Minimal {
                let line_char = match self.style {
                    TimelineStyle::Line => '│',
                    TimelineStyle::Boxed => '│',
                    TimelineStyle::Alternating => '│',
                    TimelineStyle::Minimal => ' ',
                };
                let line_y = y + 1;
                if line_y < area.y + area.height {
                    let mut line_cell = Cell::new(line_char);
                    line_cell.fg = Some(self.line_color);
                    ctx.buffer.set(icon_x, line_y, line_cell);
                }
            }

            // Draw connector
            let connector = match self.style {
                TimelineStyle::Line | TimelineStyle::Alternating => '─',
                TimelineStyle::Boxed => '─',
                TimelineStyle::Minimal => ' ',
            };
            if self.style != TimelineStyle::Minimal {
                let mut conn_cell = Cell::new(connector);
                conn_cell.fg = Some(self.line_color);
                ctx.buffer.set(icon_x + 1, y, conn_cell);
            }

            // Draw title
            let title_fg = if is_selected { color } else { self.title_color };
            for (j, ch) in event.title.chars().take(content_width as usize).enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(title_fg);
                if is_selected {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.buffer.set(content_x + j as u16, y, cell);
            }

            y += 1;

            // Draw description
            if self.show_descriptions {
                if let Some(ref desc) = event.description {
                    if y < area.y + area.height {
                        // Draw line continuation
                        if i < self.events.len() - 1 && self.style != TimelineStyle::Minimal {
                            let mut line_cell = Cell::new('│');
                            line_cell.fg = Some(self.line_color);
                            ctx.buffer.set(icon_x, y, line_cell);
                        }

                        // Draw description text
                        for (j, ch) in desc.chars().take(content_width as usize).enumerate() {
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(self.desc_color);
                            ctx.buffer.set(content_x + j as u16, y, cell);
                        }

                        y += 1;
                    }
                }
            }

            // Add spacing between events
            if y < area.y + area.height && i < self.events.len() - 1 {
                if self.style != TimelineStyle::Minimal {
                    let mut line_cell = Cell::new('│');
                    line_cell.fg = Some(self.line_color);
                    ctx.buffer.set(icon_x, y, line_cell);
                }
                y += 1;
            }
        }
    }

    fn render_horizontal(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if self.events.is_empty() || area.width < 10 {
            return;
        }

        let event_width = 15u16;
        let line_y = area.y + 1;

        // Draw horizontal line
        for x in area.x..area.x + area.width {
            let mut cell = Cell::new('─');
            cell.fg = Some(self.line_color);
            ctx.buffer.set(x, line_y, cell);
        }

        // Draw events
        let mut x = area.x;
        for (i, event) in self.events.iter().enumerate() {
            if x >= area.x + area.width {
                break;
            }

            let is_selected = self.selected == Some(i);
            let color = event.display_color();

            // Draw icon
            let icon = event.event_type.icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(color);
            if is_selected {
                icon_cell.modifier |= Modifier::BOLD;
            }
            ctx.buffer.set(x + event_width / 2, line_y, icon_cell);

            // Draw title above
            let title: String = event.title.chars().take(event_width as usize - 1).collect();
            let title_x = x + (event_width - title.len() as u16) / 2;
            for (j, ch) in title.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(if is_selected { color } else { self.title_color });
                ctx.buffer.set(title_x + j as u16, area.y, cell);
            }

            // Draw timestamp below
            if self.show_timestamps {
                if let Some(ref ts) = event.timestamp {
                    let ts_str: String = ts.chars().take(event_width as usize - 1).collect();
                    let ts_x = x + (event_width - ts_str.len() as u16) / 2;
                    for (j, ch) in ts_str.chars().enumerate() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.timestamp_color);
                        ctx.buffer.set(ts_x + j as u16, line_y + 1, cell);
                    }
                }
            }

            x += event_width;
        }
    }
}

// Helper functions

/// Create a new timeline widget
pub fn timeline() -> Timeline {
    Timeline::new()
}

/// Create a new timeline event with title
pub fn timeline_event(title: impl Into<String>) -> TimelineEvent {
    TimelineEvent::new(title)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_timeline_event() {
        let event = TimelineEvent::new("Test Event")
            .description("Description")
            .timestamp("10:30 AM")
            .success();

        assert_eq!(event.title, "Test Event");
        assert_eq!(event.description, Some("Description".to_string()));
        assert_eq!(event.event_type, EventType::Success);
    }

    #[test]
    fn test_timeline() {
        let tl = Timeline::new()
            .event(TimelineEvent::new("First"))
            .event(TimelineEvent::new("Second"))
            .event(TimelineEvent::new("Third"));

        assert_eq!(tl.len(), 3);
        assert!(!tl.is_empty());
    }

    #[test]
    fn test_timeline_selection() {
        let mut tl = Timeline::new()
            .event(TimelineEvent::new("A"))
            .event(TimelineEvent::new("B"))
            .event(TimelineEvent::new("C"));

        assert_eq!(tl.selected, None);

        tl.select_next();
        assert_eq!(tl.selected, Some(0));

        tl.select_next();
        assert_eq!(tl.selected, Some(1));

        tl.select_prev();
        assert_eq!(tl.selected, Some(0));
    }

    #[test]
    fn test_event_colors() {
        assert_eq!(EventType::Success.color(), Color::GREEN);
        assert_eq!(EventType::Error.color(), Color::RED);
        assert_eq!(EventType::Warning.color(), Color::YELLOW);
    }

    #[test]
    fn test_timeline_render() {
        let mut buffer = Buffer::new(60, 20);
        let area = Rect::new(0, 0, 60, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tl = Timeline::new()
            .event(TimelineEvent::new("Event 1").timestamp("10:00"))
            .event(TimelineEvent::new("Event 2").timestamp("11:00"));

        tl.render(&mut ctx);
        // Smoke test
    }
}
