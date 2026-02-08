//! Profiler types

use crate::style::Color;
use std::time::{Duration, Instant};

/// Profiler view mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProfilerView {
    /// Flamegraph visualization
    #[default]
    Flamegraph,
    /// Timeline view
    Timeline,
    /// Ranked list by render time
    Ranked,
    /// Component render counts
    Counts,
}

impl ProfilerView {
    /// Get view label
    pub fn label(&self) -> &'static str {
        match self {
            Self::Flamegraph => "Flamegraph",
            Self::Timeline => "Timeline",
            Self::Ranked => "Ranked",
            Self::Counts => "Counts",
        }
    }

    /// Get all views
    pub fn all() -> &'static [ProfilerView] {
        &[
            ProfilerView::Flamegraph,
            ProfilerView::Timeline,
            ProfilerView::Ranked,
            ProfilerView::Counts,
        ]
    }

    /// Next view
    pub fn next(&self) -> Self {
        match self {
            Self::Flamegraph => Self::Timeline,
            Self::Timeline => Self::Ranked,
            Self::Ranked => Self::Counts,
            Self::Counts => Self::Flamegraph,
        }
    }
}

/// A single render event
#[derive(Debug, Clone)]
pub struct RenderEvent {
    /// Component name
    pub component: String,
    /// Parent component (for hierarchy)
    pub parent: Option<String>,
    /// Render duration
    pub duration: Duration,
    /// Timestamp when render started
    pub timestamp: Instant,
    /// Render reason
    pub reason: RenderReason,
    /// Depth in component tree
    pub depth: usize,
}

impl RenderEvent {
    /// Create a new render event
    pub fn new(component: impl Into<String>, duration: Duration) -> Self {
        Self {
            component: component.into(),
            parent: None,
            duration,
            timestamp: Instant::now(),
            reason: RenderReason::Initial,
            depth: 0,
        }
    }

    /// Set parent component
    pub fn parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }

    /// Set render reason
    pub fn reason(mut self, reason: RenderReason) -> Self {
        self.reason = reason;
        self
    }

    /// Set depth
    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }
}

/// Reason for render
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderReason {
    /// Initial render
    #[default]
    Initial,
    /// State change
    StateChange,
    /// Props change
    PropsChange,
    /// Context change
    ContextChange,
    /// Parent re-render
    ParentRender,
    /// Force update
    ForceUpdate,
}

impl RenderReason {
    /// Get display label
    pub fn label(&self) -> &'static str {
        match self {
            Self::Initial => "Initial",
            Self::StateChange => "State",
            Self::PropsChange => "Props",
            Self::ContextChange => "Context",
            Self::ParentRender => "Parent",
            Self::ForceUpdate => "Force",
        }
    }

    /// Get color for visualization
    pub fn color(&self) -> Color {
        match self {
            Self::Initial => Color::rgb(100, 180, 100),       // Green
            Self::StateChange => Color::rgb(100, 150, 220),   // Blue
            Self::PropsChange => Color::rgb(220, 180, 100),   // Yellow
            Self::ContextChange => Color::rgb(180, 100, 220), // Purple
            Self::ParentRender => Color::rgb(180, 180, 180),  // Gray
            Self::ForceUpdate => Color::rgb(220, 100, 100),   // Red
        }
    }
}

/// A frame in the timeline
#[derive(Debug, Clone)]
pub struct Frame {
    /// Frame number
    pub number: u64,
    /// Total frame duration
    pub duration: Duration,
    /// Render events in this frame
    pub events: Vec<RenderEvent>,
    /// Frame start time
    pub start_time: Instant,
}

impl Frame {
    /// Create a new frame
    pub fn new(number: u64) -> Self {
        Self {
            number,
            duration: Duration::ZERO,
            events: Vec::new(),
            start_time: Instant::now(),
        }
    }

    /// Add a render event
    pub fn add_event(&mut self, event: RenderEvent) {
        self.events.push(event);
    }

    /// End the frame and calculate duration
    pub fn end(&mut self) {
        self.duration = self.start_time.elapsed();
    }

    /// Get total render time for this frame
    pub fn total_render_time(&self) -> Duration {
        self.events.iter().map(|e| e.duration).sum()
    }

    /// Get event count
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}

/// Component render statistics
#[derive(Debug, Clone, Default)]
pub struct ComponentStats {
    /// Component name
    pub name: String,
    /// Total renders
    pub render_count: u64,
    /// Total render time
    pub total_time: Duration,
    /// Average render time
    pub avg_time: Duration,
    /// Min render time
    pub min_time: Duration,
    /// Max render time
    pub max_time: Duration,
    /// Last render reason
    pub last_reason: RenderReason,
}

impl ComponentStats {
    /// Create new stats for a component
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            render_count: 0,
            total_time: Duration::ZERO,
            avg_time: Duration::ZERO,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
            last_reason: RenderReason::Initial,
        }
    }

    /// Record a render
    pub fn record(&mut self, duration: Duration, reason: RenderReason) {
        self.render_count += 1;
        self.total_time += duration;
        self.avg_time = self.total_time / self.render_count as u32;
        self.min_time = self.min_time.min(duration);
        self.max_time = self.max_time.max(duration);
        self.last_reason = reason;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_view_default() {
        let view = ProfilerView::default();
        assert_eq!(view, ProfilerView::Flamegraph);
    }

    #[test]
    fn test_profiler_view_label() {
        assert_eq!(ProfilerView::Flamegraph.label(), "Flamegraph");
        assert_eq!(ProfilerView::Timeline.label(), "Timeline");
        assert_eq!(ProfilerView::Ranked.label(), "Ranked");
        assert_eq!(ProfilerView::Counts.label(), "Counts");
    }

    #[test]
    fn test_profiler_view_all() {
        let all = ProfilerView::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&ProfilerView::Flamegraph));
        assert!(all.contains(&ProfilerView::Timeline));
        assert!(all.contains(&ProfilerView::Ranked));
        assert!(all.contains(&ProfilerView::Counts));
    }

    #[test]
    fn test_profiler_view_next() {
        assert_eq!(ProfilerView::Flamegraph.next(), ProfilerView::Timeline);
        assert_eq!(ProfilerView::Timeline.next(), ProfilerView::Ranked);
        assert_eq!(ProfilerView::Ranked.next(), ProfilerView::Counts);
        assert_eq!(ProfilerView::Counts.next(), ProfilerView::Flamegraph);
    }

    #[test]
    fn test_render_event_new() {
        let event = RenderEvent::new("TestComponent", Duration::from_millis(10));
        assert_eq!(event.component, "TestComponent");
        assert_eq!(event.duration, Duration::from_millis(10));
        assert_eq!(event.parent, None);
        assert_eq!(event.reason, RenderReason::Initial);
        assert_eq!(event.depth, 0);
    }

    #[test]
    fn test_render_event_builder() {
        let event = RenderEvent::new("TestComponent", Duration::from_millis(10))
            .parent("ParentComponent")
            .reason(RenderReason::StateChange)
            .depth(2);

        assert_eq!(event.component, "TestComponent");
        assert_eq!(event.parent, Some("ParentComponent".to_string()));
        assert_eq!(event.reason, RenderReason::StateChange);
        assert_eq!(event.depth, 2);
    }

    #[test]
    fn test_render_event_public_fields() {
        let mut event = RenderEvent::new("Test", Duration::from_millis(5));
        event.component = "Modified".to_string();
        event.duration = Duration::from_millis(20);
        event.depth = 5;

        assert_eq!(event.component, "Modified");
        assert_eq!(event.duration, Duration::from_millis(20));
        assert_eq!(event.depth, 5);
    }

    #[test]
    fn test_render_reason_default() {
        let reason = RenderReason::default();
        assert_eq!(reason, RenderReason::Initial);
    }

    #[test]
    fn test_render_reason_label() {
        assert_eq!(RenderReason::Initial.label(), "Initial");
        assert_eq!(RenderReason::StateChange.label(), "State");
        assert_eq!(RenderReason::PropsChange.label(), "Props");
        assert_eq!(RenderReason::ContextChange.label(), "Context");
        assert_eq!(RenderReason::ParentRender.label(), "Parent");
        assert_eq!(RenderReason::ForceUpdate.label(), "Force");
    }

    #[test]
    fn test_render_reason_color() {
        let colors = [
            RenderReason::Initial.color(),
            RenderReason::StateChange.color(),
            RenderReason::PropsChange.color(),
            RenderReason::ContextChange.color(),
            RenderReason::ParentRender.color(),
            RenderReason::ForceUpdate.color(),
        ];
        // Just verify they return valid colors
        for color in colors {
            let _ = color;
        }
    }

    #[test]
    fn test_frame_new() {
        let frame = Frame::new(42);
        assert_eq!(frame.number, 42);
        assert_eq!(frame.duration, Duration::ZERO);
        assert!(frame.events.is_empty());
    }

    #[test]
    fn test_frame_add_event() {
        let mut frame = Frame::new(1);
        let event = RenderEvent::new("Test", Duration::from_millis(5));
        frame.add_event(event.clone());
        assert_eq!(frame.events.len(), 1);
    }

    #[test]
    fn test_frame_end() {
        let mut frame = Frame::new(1);
        std::thread::sleep(std::time::Duration::from_millis(10));
        frame.end();
        assert!(frame.duration.as_millis() >= 10);
    }

    #[test]
    fn test_frame_total_render_time() {
        let mut frame = Frame::new(1);
        frame.add_event(RenderEvent::new("A", Duration::from_millis(10)));
        frame.add_event(RenderEvent::new("B", Duration::from_millis(20)));
        assert_eq!(frame.total_render_time(), Duration::from_millis(30));
    }

    #[test]
    fn test_frame_event_count() {
        let mut frame = Frame::new(1);
        assert_eq!(frame.event_count(), 0);
        frame.add_event(RenderEvent::new("A", Duration::from_millis(10)));
        assert_eq!(frame.event_count(), 1);
        frame.add_event(RenderEvent::new("B", Duration::from_millis(10)));
        assert_eq!(frame.event_count(), 2);
    }

    #[test]
    fn test_component_stats_new() {
        let stats = ComponentStats::new("TestComponent");
        assert_eq!(stats.name, "TestComponent");
        assert_eq!(stats.render_count, 0);
        assert_eq!(stats.total_time, Duration::ZERO);
        assert_eq!(stats.min_time, Duration::MAX);
        assert_eq!(stats.max_time, Duration::ZERO);
    }

    #[test]
    fn test_component_stats_default() {
        let stats = ComponentStats::default();
        assert_eq!(stats.name, "");
        assert_eq!(stats.render_count, 0);
    }

    #[test]
    fn test_component_stats_record() {
        let mut stats = ComponentStats::new("Test");
        stats.record(Duration::from_millis(10), RenderReason::Initial);
        assert_eq!(stats.render_count, 1);
        assert_eq!(stats.total_time, Duration::from_millis(10));
        assert_eq!(stats.min_time, Duration::from_millis(10));
        assert_eq!(stats.max_time, Duration::from_millis(10));
        assert_eq!(stats.last_reason, RenderReason::Initial);

        stats.record(Duration::from_millis(20), RenderReason::StateChange);
        assert_eq!(stats.render_count, 2);
        assert_eq!(stats.total_time, Duration::from_millis(30));
        assert_eq!(stats.min_time, Duration::from_millis(10));
        assert_eq!(stats.max_time, Duration::from_millis(20));
        assert_eq!(stats.last_reason, RenderReason::StateChange);
    }

    #[test]
    fn test_component_stats_public_fields() {
        let mut stats = ComponentStats::new("Test");
        stats.name = "Modified".to_string();
        stats.render_count = 5;

        assert_eq!(stats.name, "Modified");
        assert_eq!(stats.render_count, 5);
    }
}
