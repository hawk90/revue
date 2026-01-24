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
