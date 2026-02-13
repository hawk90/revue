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

    // =========================================================================
    // TimelineEvent::warning tests
    // =========================================================================

    #[test]
    fn test_timeline_event_warning() {
        let event = TimelineEvent::new("Warning").warning();
        assert_eq!(event.event_type, EventType::Warning);
    }

    #[test]
    fn test_timeline_event_error() {
        let event = TimelineEvent::new("Error").error();
        assert_eq!(event.event_type, EventType::Error);
    }

    // =========================================================================
    // TimelineEvent::color tests
    // =========================================================================

    #[test]
    fn test_timeline_event_color() {
        let event = TimelineEvent::new("Test").color(Color::MAGENTA);
        assert_eq!(event.color, Some(Color::MAGENTA));
    }

    #[test]
    fn test_display_color_override() {
        let event = TimelineEvent::new("Test").success().color(Color::MAGENTA);
        assert_eq!(event.display_color(), Color::MAGENTA);
    }

    #[test]
    fn test_display_color_default() {
        let event = TimelineEvent::new("Test").error();
        assert_eq!(event.display_color(), Color::RED);
    }

    // =========================================================================
    // TimelineEvent::meta tests
    // =========================================================================

    #[test]
    fn test_timeline_event_meta() {
        let event = TimelineEvent::new("Test")
            .meta("key1", "value1")
            .meta("key2", "value2");

        assert_eq!(event.metadata.len(), 2);
        assert_eq!(event.metadata[0].0, "key1");
        assert_eq!(event.metadata[0].1, "value1");
    }

    #[test]
    fn test_timeline_event_meta_single() {
        let event = TimelineEvent::new("Test").meta("status", "pending");
        assert_eq!(event.metadata.len(), 1);
    }

    // =========================================================================
    // Timeline::events tests
    // =========================================================================

    #[test]
    fn test_timeline_events_multiple() {
        let events = vec![
            TimelineEvent::new("A"),
            TimelineEvent::new("B"),
            TimelineEvent::new("C"),
        ];
        let tl = Timeline::new().events(events);
        assert_eq!(tl.len(), 3);
    }

    #[test]
    fn test_timeline_events_empty() {
        let tl = Timeline::new().events(vec![]);
        assert!(tl.is_empty());
    }

    // =========================================================================
    // Timeline::orientation tests
    // =========================================================================

    #[test]
    fn test_timeline_orientation_horizontal() {
        let tl = Timeline::new().orientation(TimelineOrientation::Horizontal);
        assert_eq!(tl.orientation, TimelineOrientation::Horizontal);
    }

    #[test]
    fn test_timeline_vertical() {
        let tl = Timeline::new().vertical();
        assert_eq!(tl.orientation, TimelineOrientation::Vertical);
    }

    #[test]
    fn test_timeline_horizontal() {
        let tl = Timeline::new().horizontal();
        assert_eq!(tl.orientation, TimelineOrientation::Horizontal);
    }

    // =========================================================================
    // Timeline::style tests
    // =========================================================================

    #[test]
    fn test_timeline_style() {
        let tl = Timeline::new().style(TimelineStyle::Boxed);
        assert_eq!(tl.style, TimelineStyle::Boxed);
    }

    #[test]
    fn test_timeline_style_minimal() {
        let tl = Timeline::new().style(TimelineStyle::Minimal);
        assert_eq!(tl.style, TimelineStyle::Minimal);
    }

    #[test]
    fn test_timeline_style_alternating() {
        let tl = Timeline::new().style(TimelineStyle::Alternating);
        assert_eq!(tl.style, TimelineStyle::Alternating);
    }

    // =========================================================================
    // Timeline::timestamps tests
    // =========================================================================

    #[test]
    fn test_timeline_hide_timestamps() {
        let tl = Timeline::new().timestamps(false);
        assert!(!tl.show_timestamps);
    }

    #[test]
    fn test_timeline_show_timestamps() {
        let tl = Timeline::new().timestamps(true);
        assert!(tl.show_timestamps);
    }

    // =========================================================================
    // Timeline::descriptions tests
    // =========================================================================

    #[test]
    fn test_timeline_hide_descriptions() {
        let tl = Timeline::new().descriptions(false);
        assert!(!tl.show_descriptions);
    }

    #[test]
    fn test_timeline_show_descriptions() {
        let tl = Timeline::new().descriptions(true);
        assert!(tl.show_descriptions);
    }

    // =========================================================================
    // Timeline::line_color tests
    // =========================================================================

    #[test]
    fn test_timeline_line_color() {
        let tl = Timeline::new().line_color(Color::MAGENTA);
        assert_eq!(tl.line_color, Color::MAGENTA);
    }

    // =========================================================================
    // Timeline::select tests
    // =========================================================================

    #[test]
    fn test_select_specific() {
        let mut tl = Timeline::new()
            .event(TimelineEvent::new("A"))
            .event(TimelineEvent::new("B"));

        tl.select(Some(1));
        assert_eq!(tl.selected, Some(1));
    }

    #[test]
    fn test_select_none() {
        let mut tl = Timeline::new().event(TimelineEvent::new("A"));
        tl.select(Some(0));
        tl.select(None);
        assert_eq!(tl.selected, None);
    }

    #[test]
    fn test_select_out_of_bounds() {
        let mut tl = Timeline::new().event(TimelineEvent::new("A"));
        tl.select(Some(10));
        // Should still set the value
        assert_eq!(tl.selected, Some(10));
    }

    // =========================================================================
    // Timeline::selected_event tests
    // =========================================================================

    #[test]
    fn test_selected_event() {
        let mut tl = Timeline::new()
            .event(TimelineEvent::new("First"))
            .event(TimelineEvent::new("Second"));

        tl.select(Some(1));
        let event = tl.selected_event();
        assert!(event.is_some());
        assert_eq!(event.unwrap().title, "Second");
    }

    #[test]
    fn test_selected_event_none() {
        let tl = Timeline::new().event(TimelineEvent::new("A"));
        let event = tl.selected_event();
        assert!(event.is_none());
    }

    #[test]
    fn test_selected_event_empty() {
        let tl = Timeline::new();
        let event = tl.selected_event();
        assert!(event.is_none());
    }

    // =========================================================================
    // Timeline::clear tests
    // =========================================================================

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

    // =========================================================================
    // Timeline::push tests
    // =========================================================================

    #[test]
    fn test_push() {
        let mut tl = Timeline::new();
        assert!(tl.is_empty());

        tl.push(TimelineEvent::new("Dynamic"));
        assert_eq!(tl.len(), 1);
    }

    #[test]
    fn test_push_multiple() {
        let mut tl = Timeline::new();
        tl.push(TimelineEvent::new("1"));
        tl.push(TimelineEvent::new("2"));
        tl.push(TimelineEvent::new("3"));
        assert_eq!(tl.len(), 3);
    }

    // =========================================================================
    // EventType::icon tests
    // =========================================================================

    #[test]
    fn test_event_type_icon_info() {
        assert_eq!(EventType::Info.icon(), '●');
    }

    #[test]
    fn test_event_type_icon_success() {
        assert_eq!(EventType::Success.icon(), '✓');
    }

    #[test]
    fn test_event_type_icon_warning() {
        assert_eq!(EventType::Warning.icon(), '⚠');
    }

    #[test]
    fn test_event_type_icon_error() {
        assert_eq!(EventType::Error.icon(), '✗');
    }

    #[test]
    fn test_event_type_icon_custom() {
        assert_eq!(EventType::Custom('★').icon(), '★');
    }

    // =========================================================================
    // EventType::color tests
    // =========================================================================

    #[test]
    fn test_event_type_color_info() {
        assert_eq!(EventType::Info.color(), Color::CYAN);
    }

    #[test]
    fn test_event_type_color_custom() {
        assert_eq!(EventType::Custom('X').color(), Color::WHITE);
    }

    // =========================================================================
    // TimelineOrientation enum tests
    // =========================================================================

    #[test]
    fn test_timeline_orientation_default() {
        assert_eq!(
            TimelineOrientation::default(),
            TimelineOrientation::Vertical
        );
    }

    #[test]
    fn test_timeline_orientation_clone() {
        let orient = TimelineOrientation::Horizontal;
        let cloned = orient;
        assert_eq!(orient, cloned);
    }

    #[test]
    fn test_timeline_orientation_copy() {
        let o1 = TimelineOrientation::Horizontal;
        let o2 = o1;
        assert_eq!(o1, TimelineOrientation::Horizontal);
        assert_eq!(o2, TimelineOrientation::Horizontal);
    }

    // =========================================================================
    // TimelineStyle enum tests
    // =========================================================================

    #[test]
    fn test_timeline_style_default() {
        assert_eq!(TimelineStyle::default(), TimelineStyle::Line);
    }

    #[test]
    fn test_timeline_style_clone() {
        let style = TimelineStyle::Boxed;
        let cloned = style;
        assert_eq!(style, cloned);
    }

    #[test]
    fn test_timeline_style_copy() {
        let s1 = TimelineStyle::Minimal;
        let s2 = s1;
        assert_eq!(s1, TimelineStyle::Minimal);
        assert_eq!(s2, TimelineStyle::Minimal);
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_timeline_helper() {
        let tl = timeline();
        assert!(tl.is_empty());
    }

    #[test]
    fn test_timeline_event_helper() {
        let event = timeline_event("Test Event");
        assert_eq!(event.title, "Test Event");
    }

    #[test]
    fn test_timeline_event_helper_with_string() {
        let event = timeline_event("Event".to_string());
        assert_eq!(event.title, "Event");
    }

    // =========================================================================
    // Timeline Default tests
    // =========================================================================

    #[test]
    fn test_timeline_default() {
        let tl = Timeline::default();
        assert!(tl.is_empty());
    }

    // =========================================================================
    // TimelineEvent Default trait not implemented
    // =========================================================================

    #[test]
    fn test_timeline_event_no_default() {
        // TimelineEvent doesn't implement Default
        // Just verify we can create one with new()
        let event = TimelineEvent::new("Test");
        assert_eq!(event.title, "Test");
    }

    // =========================================================================
    // Timeline::select_next edge cases
    // =========================================================================

    #[test]
    fn test_select_next_empty() {
        let mut tl = Timeline::new();
        tl.select_next(); // Should do nothing
        assert_eq!(tl.selected, None);
    }

    #[test]
    fn test_select_next_at_end() {
        let mut tl = Timeline::new()
            .event(TimelineEvent::new("A"))
            .event(TimelineEvent::new("B"));
        tl.select(Some(1));
        tl.select_next(); // Should stay at end
        assert_eq!(tl.selected, Some(1));
    }

    // =========================================================================
    // Timeline::select_prev edge cases
    // =========================================================================

    #[test]
    fn test_select_prev_from_start() {
        let mut tl = Timeline::new()
            .event(TimelineEvent::new("A"))
            .event(TimelineEvent::new("B"));
        tl.select(Some(0));
        tl.select_prev(); // Should stay at start
        assert_eq!(tl.selected, Some(0));
    }

    #[test]
    fn test_select_prev_none() {
        let mut tl = Timeline::new().event(TimelineEvent::new("A"));
        tl.select_prev(); // Should do nothing
        assert_eq!(tl.selected, None);
    }

    // =========================================================================
    // Render tests
    // =========================================================================

    #[test]
    fn test_render_horizontal() {
        let mut buffer = Buffer::new(60, 10);
        let area = Rect::new(0, 0, 60, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tl = Timeline::new()
            .horizontal()
            .event(TimelineEvent::new("Event 1").timestamp("10:00"))
            .event(TimelineEvent::new("Event 2").timestamp("11:00"));

        tl.render(&mut ctx); // Should not panic
    }

    #[test]
    fn test_render_empty() {
        let mut buffer = Buffer::new(60, 10);
        let area = Rect::new(0, 0, 60, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tl = Timeline::new();
        tl.render(&mut ctx); // Should return early without panicking
    }

    #[test]
    fn test_render_with_descriptions() {
        let mut buffer = Buffer::new(60, 10);
        let area = Rect::new(0, 0, 60, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tl = Timeline::new()
            .descriptions(true)
            .event(TimelineEvent::new("Event").description("Details here"));

        tl.render(&mut ctx);
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_builder_chain_full() {
        let tl = Timeline::new()
            .vertical()
            .style(TimelineStyle::Alternating)
            .timestamps(true)
            .descriptions(false)
            .line_color(Color::CYAN);

        assert_eq!(tl.orientation, TimelineOrientation::Vertical);
        assert_eq!(tl.style, TimelineStyle::Alternating);
        assert!(tl.show_timestamps);
        assert!(!tl.show_descriptions);
    }

    // =========================================================================
    // TimelineEvent builder chain tests
    // =========================================================================

    #[test]
    fn test_event_builder_chain() {
        let event = TimelineEvent::new("Title")
            .description("Description")
            .timestamp("10:30 AM")
            .warning()
            .color(Color::YELLOW);

        assert_eq!(event.title, "Title");
        assert_eq!(event.description, Some("Description".to_string()));
        assert_eq!(event.timestamp, Some("10:30 AM".to_string()));
        assert_eq!(event.event_type, EventType::Warning);
        assert_eq!(event.color, Some(Color::YELLOW));
    }

    // =========================================================================
    // Clone tests
    // =========================================================================

    #[test]
    fn test_event_type_clone() {
        let et = EventType::Warning;
        let cloned = et;
        assert_eq!(et, cloned);
    }

    #[test]
    fn test_timeline_event_clone() {
        let event = TimelineEvent::new("Test")
            .description("Desc")
            .timestamp("Now");

        let cloned = event.clone();
        assert_eq!(cloned.title, "Test");
        assert_eq!(cloned.description, event.description);
    }

    // =========================================================================
    // EventType Debug tests
    // =========================================================================

    #[test]
    fn test_event_type_debug() {
        let debug_str = format!("{:?}", EventType::Success);
        assert!(debug_str.contains("Success"));
    }

    #[test]
    fn test_event_type_custom_debug() {
        let debug_str = format!("{:?}", EventType::Custom('★'));
        assert!(debug_str.contains("Custom"));
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
