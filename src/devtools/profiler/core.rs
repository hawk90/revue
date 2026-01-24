//! Profiler core implementation

use super::super::helpers::draw_text_overlay;
use super::super::DevToolsConfig;
use super::types::{ComponentStats, Frame, ProfilerView, RenderEvent};
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance profiler for tracking render performance
pub struct Profiler {
    /// Is recording
    recording: bool,
    /// Recorded frames
    frames: Vec<Frame>,
    /// Current frame (while recording)
    current_frame: Option<Frame>,
    /// Component statistics
    stats: HashMap<String, ComponentStats>,
    /// Frame counter
    frame_counter: u64,
    /// Recording start time
    recording_start: Option<Instant>,
    /// Current view mode
    view: ProfilerView,
    /// Selected frame index (for timeline)
    selected_frame: Option<usize>,
    /// Scroll offset
    scroll_offset: usize,
}

impl Profiler {
    /// Create a new profiler
    pub fn new() -> Self {
        Self {
            recording: false,
            frames: Vec::new(),
            current_frame: None,
            stats: HashMap::new(),
            frame_counter: 0,
            recording_start: None,
            view: ProfilerView::default(),
            selected_frame: None,
            scroll_offset: 0,
        }
    }

    /// Start recording
    pub fn start_recording(&mut self) {
        self.recording = true;
        self.recording_start = Some(Instant::now());
        self.frames.clear();
        self.stats.clear();
        self.frame_counter = 0;
        self.start_frame();
    }

    /// Stop recording
    pub fn stop_recording(&mut self) {
        self.end_frame();
        self.recording = false;
        self.recording_start = None;
    }

    /// Check if recording
    pub fn is_recording(&self) -> bool {
        self.recording
    }

    /// Toggle recording
    pub fn toggle_recording(&mut self) {
        if self.recording {
            self.stop_recording();
        } else {
            self.start_recording();
        }
    }

    /// Start a new frame
    pub fn start_frame(&mut self) {
        if self.recording {
            self.frame_counter += 1;
            self.current_frame = Some(Frame::new(self.frame_counter));
        }
    }

    /// End the current frame
    pub fn end_frame(&mut self) {
        if let Some(mut frame) = self.current_frame.take() {
            frame.end();
            self.frames.push(frame);
        }
    }

    /// Record a render event
    pub fn record_render(&mut self, event: RenderEvent) {
        // Update stats
        let stats = self
            .stats
            .entry(event.component.clone())
            .or_insert_with(|| ComponentStats::new(&event.component));
        stats.record(event.duration, event.reason);

        // Add to current frame
        if let Some(frame) = &mut self.current_frame {
            frame.add_event(event);
        }
    }

    /// Get frame count
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Get total recording duration
    pub fn recording_duration(&self) -> Duration {
        self.recording_start
            .map(|start| start.elapsed())
            .unwrap_or(Duration::ZERO)
    }

    /// Get average frame time
    pub fn avg_frame_time(&self) -> Duration {
        if self.frames.is_empty() {
            return Duration::ZERO;
        }
        let total: Duration = self.frames.iter().map(|f| f.duration).sum();
        total / self.frames.len() as u32
    }

    /// Get stats sorted by total time
    pub fn stats_by_time(&self) -> Vec<&ComponentStats> {
        let mut stats: Vec<_> = self.stats.values().collect();
        stats.sort_by(|a, b| b.total_time.cmp(&a.total_time));
        stats
    }

    /// Get stats sorted by render count
    pub fn stats_by_count(&self) -> Vec<&ComponentStats> {
        let mut stats: Vec<_> = self.stats.values().collect();
        stats.sort_by(|a, b| b.render_count.cmp(&a.render_count));
        stats
    }

    /// Get current view
    pub fn view(&self) -> ProfilerView {
        self.view
    }

    /// Set view
    pub fn set_view(&mut self, view: ProfilerView) {
        self.view = view;
        self.scroll_offset = 0;
    }

    /// Next view
    pub fn next_view(&mut self) {
        self.view = self.view.next();
        self.scroll_offset = 0;
    }

    /// Select a frame
    pub fn select_frame(&mut self, index: Option<usize>) {
        self.selected_frame = index;
    }

    /// Scroll up
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    /// Scroll down
    pub fn scroll_down(&mut self) {
        self.scroll_offset += 1;
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.frames.clear();
        self.stats.clear();
        self.current_frame = None;
        self.frame_counter = 0;
        self.selected_frame = None;
        self.scroll_offset = 0;
    }

    /// Render profiler content
    pub fn render_content(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        match self.view {
            ProfilerView::Flamegraph => self.render_flamegraph(buffer, area, config),
            ProfilerView::Timeline => self.render_timeline(buffer, area, config),
            ProfilerView::Ranked => self.render_ranked(buffer, area, config),
            ProfilerView::Counts => self.render_counts(buffer, area, config),
        }
    }

    fn render_flamegraph(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        if self.frames.is_empty() {
            self.render_empty(buffer, area, config, "No data. Press R to start recording.");
            return;
        }

        // Header
        let header = format!(
            "Flamegraph - {} frames, avg {:.2}ms/frame",
            self.frames.len(),
            self.avg_frame_time().as_secs_f64() * 1000.0
        );
        self.render_text(buffer, area.x, area.y, &header, config.fg_color);

        // Get the selected frame or last frame
        let frame = self
            .selected_frame
            .and_then(|i| self.frames.get(i))
            .or_else(|| self.frames.last());

        if let Some(frame) = frame {
            let content_y = area.y + 2;
            let content_height = area.height.saturating_sub(3);

            // Group events by depth for flamegraph rows
            let max_depth = frame.events.iter().map(|e| e.depth).max().unwrap_or(0);
            let row_height = if max_depth > 0 {
                (content_height as usize / (max_depth + 1)).max(1)
            } else {
                content_height as usize
            };

            for event in &frame.events {
                let y = content_y + (event.depth * row_height) as u16;
                if y >= area.y + area.height {
                    continue;
                }

                // Calculate bar width based on duration
                let total_time = frame.total_render_time().as_nanos() as f64;
                let event_time = event.duration.as_nanos() as f64;
                let width = if total_time > 0.0 {
                    ((event_time / total_time) * area.width as f64) as u16
                } else {
                    1
                };
                let width = width.max(1).min(area.width);

                // Draw bar
                let color = event.reason.color();
                for x in area.x..area.x + width {
                    if let Some(cell) = buffer.get_mut(x, y) {
                        cell.bg = Some(color);
                    }
                }

                // Draw label
                let label = format!(
                    "{} ({:.2}ms)",
                    event.component,
                    event.duration.as_secs_f64() * 1000.0
                );
                self.render_text(buffer, area.x, y, &label, config.bg_color);
            }
        }
    }

    fn render_timeline(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        if self.frames.is_empty() {
            self.render_empty(buffer, area, config, "No data. Press R to start recording.");
            return;
        }

        // Header
        let header = format!("Timeline - {} frames", self.frames.len());
        self.render_text(buffer, area.x, area.y, &header, config.fg_color);

        let content_y = area.y + 2;
        let content_height = area.height.saturating_sub(3) as usize;

        // Find max frame time for scaling
        let max_time = self
            .frames
            .iter()
            .map(|f| f.duration)
            .max()
            .unwrap_or(Duration::from_millis(16));

        // Draw timeline bars
        let visible_frames = area.width as usize;
        let start_frame = self
            .scroll_offset
            .min(self.frames.len().saturating_sub(visible_frames));

        for (i, frame) in self
            .frames
            .iter()
            .skip(start_frame)
            .take(visible_frames)
            .enumerate()
        {
            let x = area.x + i as u16;
            let height = ((frame.duration.as_nanos() as f64 / max_time.as_nanos() as f64)
                * content_height as f64) as u16;
            let height = height.max(1);

            let bar_y = content_y + (content_height as u16).saturating_sub(height);

            // Color based on frame time (green = fast, red = slow)
            let color = if frame.duration < Duration::from_millis(8) {
                Color::rgb(100, 200, 100) // Green - very fast
            } else if frame.duration < Duration::from_millis(16) {
                Color::rgb(200, 200, 100) // Yellow - 60fps
            } else if frame.duration < Duration::from_millis(33) {
                Color::rgb(220, 150, 100) // Orange - 30fps
            } else {
                Color::rgb(220, 100, 100) // Red - slow
            };

            // Draw bar
            for y in bar_y..content_y + content_height as u16 {
                if let Some(cell) = buffer.get_mut(x, y) {
                    cell.bg = Some(color);
                    cell.symbol = ' ';
                }
            }

            // Highlight selected frame
            if self.selected_frame == Some(start_frame + i) {
                if let Some(cell) = buffer.get_mut(x, bar_y) {
                    cell.symbol = '▼';
                    cell.fg = Some(config.accent_color);
                }
            }
        }

        // Show frame info at bottom
        if let Some(idx) = self.selected_frame {
            if let Some(frame) = self.frames.get(idx) {
                let info = format!(
                    "Frame {}: {:.2}ms, {} renders",
                    frame.number,
                    frame.duration.as_secs_f64() * 1000.0,
                    frame.event_count()
                );
                self.render_text(
                    buffer,
                    area.x,
                    area.y + area.height - 1,
                    &info,
                    config.accent_color,
                );
            }
        }
    }

    fn render_ranked(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        if self.stats.is_empty() {
            self.render_empty(buffer, area, config, "No data. Press R to start recording.");
            return;
        }

        // Header
        let header = "Ranked by Total Time";
        self.render_text(buffer, area.x, area.y, header, config.fg_color);

        let content_y = area.y + 2;
        let content_height = area.height.saturating_sub(3) as usize;

        let stats = self.stats_by_time();

        for (i, stat) in stats
            .iter()
            .skip(self.scroll_offset)
            .take(content_height)
            .enumerate()
        {
            let y = content_y + i as u16;

            // Component name
            let name = if stat.name.len() > 20 {
                format!("{}...", &stat.name[..17])
            } else {
                stat.name.clone()
            };

            // Stats
            let line = format!(
                "{:<20} {:>6.2}ms total  {:>6.2}ms avg  {:>4} renders",
                name,
                stat.total_time.as_secs_f64() * 1000.0,
                stat.avg_time.as_secs_f64() * 1000.0,
                stat.render_count
            );

            self.render_text(buffer, area.x, y, &line, config.fg_color);

            // Color indicator for render reason
            if let Some(cell) = buffer.get_mut(area.x + area.width - 2, y) {
                cell.bg = Some(stat.last_reason.color());
            }
        }
    }

    fn render_counts(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        if self.stats.is_empty() {
            self.render_empty(buffer, area, config, "No data. Press R to start recording.");
            return;
        }

        // Header
        let header = "Ranked by Render Count";
        self.render_text(buffer, area.x, area.y, header, config.fg_color);

        let content_y = area.y + 2;
        let content_height = area.height.saturating_sub(3) as usize;

        let stats = self.stats_by_count();
        let max_count = stats.first().map(|s| s.render_count).unwrap_or(1);

        for (i, stat) in stats
            .iter()
            .skip(self.scroll_offset)
            .take(content_height)
            .enumerate()
        {
            let y = content_y + i as u16;

            // Component name
            let name = if stat.name.len() > 20 {
                format!("{}...", &stat.name[..17])
            } else {
                stat.name.clone()
            };

            // Bar width based on count
            let bar_width =
                ((stat.render_count as f64 / max_count as f64) * (area.width as f64 / 2.0)) as u16;
            let bar_width = bar_width.max(1);

            // Draw name and count
            let count_str = format!("{:<20} {:>6}", name, stat.render_count);
            self.render_text(buffer, area.x, y, &count_str, config.fg_color);

            // Draw bar
            let bar_start = area.x + 28;
            for x in bar_start..bar_start + bar_width {
                if x < area.x + area.width {
                    if let Some(cell) = buffer.get_mut(x, y) {
                        cell.bg = Some(config.accent_color);
                        cell.symbol = ' ';
                    }
                }
            }
        }
    }

    fn render_empty(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig, msg: &str) {
        let x = area.x + (area.width.saturating_sub(msg.len() as u16)) / 2;
        let y = area.y + area.height / 2;
        self.render_text(buffer, x, y, msg, config.fg_color);
    }

    fn render_text(&self, buffer: &mut Buffer, x: u16, y: u16, text: &str, color: Color) {
        draw_text_overlay(buffer, x, y, text, color);
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devtools::profiler::types::RenderReason;

    #[test]
    fn test_profiler_creation() {
        let profiler = Profiler::new();
        assert!(!profiler.is_recording());
        assert_eq!(profiler.frame_count(), 0);
    }

    #[test]
    fn test_profiler_recording() {
        let mut profiler = Profiler::new();

        profiler.start_recording();
        assert!(profiler.is_recording());

        profiler.record_render(RenderEvent::new("Button", Duration::from_micros(100)));
        profiler.end_frame();
        profiler.start_frame();
        profiler.record_render(RenderEvent::new("Input", Duration::from_micros(200)));
        profiler.end_frame();

        profiler.stop_recording();
        assert!(!profiler.is_recording());
        assert_eq!(profiler.frame_count(), 2);
    }

    #[test]
    fn test_profiler_toggle() {
        let mut profiler = Profiler::new();

        profiler.toggle_recording();
        assert!(profiler.is_recording());

        profiler.toggle_recording();
        assert!(!profiler.is_recording());
    }

    #[test]
    fn test_render_event() {
        let event = RenderEvent::new("MyComponent", Duration::from_millis(5))
            .parent("ParentComponent")
            .reason(RenderReason::StateChange)
            .depth(2);

        assert_eq!(event.component, "MyComponent");
        assert_eq!(event.parent, Some("ParentComponent".to_string()));
        assert_eq!(event.reason, RenderReason::StateChange);
        assert_eq!(event.depth, 2);
    }

    #[test]
    fn test_frame() {
        let mut frame = Frame::new(1);
        frame.add_event(RenderEvent::new("A", Duration::from_micros(100)));
        frame.add_event(RenderEvent::new("B", Duration::from_micros(200)));
        frame.end();

        assert_eq!(frame.number, 1);
        assert_eq!(frame.event_count(), 2);
        assert_eq!(frame.total_render_time(), Duration::from_micros(300));
    }

    #[test]
    fn test_component_stats() {
        let mut stats = ComponentStats::new("Button");

        stats.record(Duration::from_micros(100), RenderReason::Initial);
        stats.record(Duration::from_micros(200), RenderReason::StateChange);
        stats.record(Duration::from_micros(150), RenderReason::PropsChange);

        assert_eq!(stats.render_count, 3);
        assert_eq!(stats.total_time, Duration::from_micros(450));
        assert_eq!(stats.min_time, Duration::from_micros(100));
        assert_eq!(stats.max_time, Duration::from_micros(200));
        assert_eq!(stats.last_reason, RenderReason::PropsChange);
    }

    #[test]
    fn test_stats_sorting() {
        let mut profiler = Profiler::new();
        profiler.start_recording();

        // Record with different times
        profiler.record_render(RenderEvent::new("Fast", Duration::from_micros(50)));
        profiler.record_render(RenderEvent::new("Slow", Duration::from_micros(500)));
        profiler.record_render(RenderEvent::new("Medium", Duration::from_micros(200)));

        let by_time = profiler.stats_by_time();
        assert_eq!(by_time[0].name, "Slow");
        assert_eq!(by_time[1].name, "Medium");
        assert_eq!(by_time[2].name, "Fast");
    }

    #[test]
    fn test_profiler_view_cycle() {
        let mut profiler = Profiler::new();
        assert_eq!(profiler.view(), ProfilerView::Flamegraph);

        profiler.next_view();
        assert_eq!(profiler.view(), ProfilerView::Timeline);

        profiler.next_view();
        assert_eq!(profiler.view(), ProfilerView::Ranked);

        profiler.next_view();
        assert_eq!(profiler.view(), ProfilerView::Counts);

        profiler.next_view();
        assert_eq!(profiler.view(), ProfilerView::Flamegraph);
    }

    #[test]
    fn test_profiler_clear() {
        let mut profiler = Profiler::new();
        profiler.start_recording();
        profiler.record_render(RenderEvent::new("Test", Duration::from_micros(100)));
        profiler.end_frame();
        profiler.stop_recording();

        assert!(!profiler.stats.is_empty());
        assert!(!profiler.frames.is_empty());

        profiler.clear();

        assert!(profiler.stats.is_empty());
        assert!(profiler.frames.is_empty());
    }

    #[test]
    fn test_render_reason_colors() {
        // Each reason should have a distinct color
        let reasons = [
            RenderReason::Initial,
            RenderReason::StateChange,
            RenderReason::PropsChange,
            RenderReason::ContextChange,
            RenderReason::ParentRender,
            RenderReason::ForceUpdate,
        ];

        for reason in &reasons {
            // Just ensure color() doesn't panic
            let _ = reason.color();
            let _ = reason.label();
        }
    }

    #[test]
    fn test_profiler_scroll() {
        let mut profiler = Profiler::new();

        profiler.scroll_down();
        assert_eq!(profiler.scroll_offset, 1);

        profiler.scroll_down();
        assert_eq!(profiler.scroll_offset, 2);

        profiler.scroll_up();
        assert_eq!(profiler.scroll_offset, 1);

        profiler.scroll_up();
        profiler.scroll_up(); // Should not go below 0
        assert_eq!(profiler.scroll_offset, 0);
    }

    #[test]
    fn test_profiler_select_frame() {
        let mut profiler = Profiler::new();

        profiler.select_frame(Some(5));
        assert_eq!(profiler.selected_frame, Some(5));

        profiler.select_frame(None);
        assert_eq!(profiler.selected_frame, None);
    }
}
