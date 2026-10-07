//! Stopwatch widget

use super::{format_ms, render_large_time, TimerFormat, TimerState};
use crate::style::Color;
use crate::widget::theme::{LIGHT_GRAY, MUTED_TEXT, PLACEHOLDER_FG};
use crate::widget::traits::WidgetProps;
use crate::widget::{RenderContext, View};
use crate::{impl_props_builders, impl_styled_view};
use std::time::Instant;

/// Stopwatch widget (counts up)
#[derive(Clone, Debug)]
pub struct Stopwatch {
    /// Elapsed time in milliseconds
    elapsed_ms: u64,
    /// State
    state: TimerState,
    /// Start instant
    started_at: Option<Instant>,
    /// Accumulated time before pause
    accumulated_ms: u64,
    /// Display format
    format: TimerFormat,
    /// Lap times
    laps: Vec<u64>,
    /// Show laps
    show_laps: bool,
    /// Max laps to display
    max_laps: usize,
    /// Colors
    fg: Option<Color>,
    /// Title
    title: Option<String>,
    /// Show large digits
    large_digits: bool,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Stopwatch {
    /// Create a new stopwatch
    pub fn new() -> Self {
        Self {
            elapsed_ms: 0,
            state: TimerState::Stopped,
            started_at: None,
            accumulated_ms: 0,
            format: TimerFormat::Short,
            laps: Vec::new(),
            show_laps: true,
            max_laps: 5,
            fg: None,
            title: None,
            large_digits: false,
            props: WidgetProps::new(),
        }
    }

    /// Set display format
    pub fn format(mut self, format: TimerFormat) -> Self {
        self.format = format;
        self
    }

    /// Show lap times
    pub fn show_laps(mut self, show: bool) -> Self {
        self.show_laps = show;
        self
    }

    /// Set max laps to display
    pub fn max_laps(mut self, max: usize) -> Self {
        self.max_laps = max;
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Show large digits
    pub fn large_digits(mut self, large: bool) -> Self {
        self.large_digits = large;
        self
    }

    /// Start the stopwatch
    pub fn start(&mut self) {
        if self.state != TimerState::Running {
            self.started_at = Some(Instant::now());
            self.state = TimerState::Running;
        }
    }

    /// Pause the stopwatch
    pub fn pause(&mut self) {
        if self.state == TimerState::Running {
            self.update();
            self.accumulated_ms = self.elapsed_ms;
            self.state = TimerState::Paused;
        }
    }

    /// Stop and reset
    pub fn stop(&mut self) {
        self.state = TimerState::Stopped;
        self.elapsed_ms = 0;
        self.accumulated_ms = 0;
        self.started_at = None;
        self.laps.clear();
    }

    /// Reset without stopping
    pub fn reset(&mut self) {
        self.elapsed_ms = 0;
        self.accumulated_ms = 0;
        self.laps.clear();
        if self.state == TimerState::Running {
            self.started_at = Some(Instant::now());
        }
    }

    /// Toggle between running and paused
    pub fn toggle(&mut self) {
        match self.state {
            TimerState::Running => self.pause(),
            _ => self.start(),
        }
    }

    /// Record a lap
    pub fn lap(&mut self) {
        if self.state == TimerState::Running {
            self.update();
            self.laps.push(self.elapsed_ms);
        }
    }

    /// Update elapsed time (call each frame)
    pub fn update(&mut self) {
        if self.state == TimerState::Running {
            if let Some(started) = self.started_at {
                self.elapsed_ms = self.accumulated_ms + started.elapsed().as_millis() as u64;
            }
        }
    }

    /// Get elapsed time in seconds
    pub fn elapsed_seconds(&self) -> f64 {
        self.elapsed_ms as f64 / 1000.0
    }

    /// Get elapsed milliseconds
    pub fn elapsed_millis(&self) -> u64 {
        self.elapsed_ms
    }

    /// Get lap times
    pub fn laps(&self) -> &[u64] {
        &self.laps
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        self.state == TimerState::Running
    }

    /// Format elapsed time
    pub fn format_elapsed(&self) -> String {
        format_ms(self.elapsed_ms, self.format)
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Stopwatch {
    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::vstack;
        use crate::widget::Text;

        // The elapsed time takes `color`, a builder color outranking it; the
        // state and lap lines keep theirs, as `Timer`'s do.
        let color = self.fg.unwrap_or_else(|| ctx.css_color(Color::WHITE));
        let mut content = vstack();

        // Title
        if let Some(title) = &self.title {
            content = content.child(Text::new(title).bold());
        }

        // Time display
        let time_str = self.format_elapsed();
        if self.large_digits {
            let digits = render_large_time(&time_str);
            for line in digits {
                content = content.child(Text::new(line).fg(color));
            }
        } else {
            content = content.child(Text::new(&time_str).fg(color).bold());
        }

        // State indicator
        let state_text = match self.state {
            TimerState::Stopped => "Stopped",
            TimerState::Running => "Running",
            TimerState::Paused => "Paused",
            TimerState::Completed => "Completed",
        };
        content = content.child(Text::new(state_text).fg(PLACEHOLDER_FG));

        // Lap times
        if self.show_laps && !self.laps.is_empty() {
            content = content.child(Text::new("Laps:").fg(MUTED_TEXT));

            let start = self.laps.len().saturating_sub(self.max_laps);
            for (i, &lap_ms) in self.laps.iter().skip(start).enumerate() {
                let lap_num = start + i + 1;
                let lap_str = format!("  #{}: {}", lap_num, format_ms(lap_ms, self.format));
                content = content.child(Text::new(lap_str).fg(LIGHT_GRAY));
            }
        }

        content.render(ctx);
    }

    crate::impl_view_meta!("Stopwatch");
}

impl_styled_view!(Stopwatch);
impl_props_builders!(Stopwatch);

#[cfg(test)]
mod tests {
    use super::*;

    // KEEP HERE: accesses private field state
    #[test]
    fn test_stopwatch_new() {
        let sw = Stopwatch::new();
        assert_eq!(sw.elapsed_millis(), 0);
        assert_eq!(sw.state, TimerState::Stopped);
    }

    // KEEP HERE: accesses private field laps
    #[test]
    fn test_stopwatch_lap() {
        let mut sw = Stopwatch::new();
        // Manually add laps to test lap storage
        sw.laps.push(1000);
        sw.laps.push(2500);

        assert_eq!(sw.laps().len(), 2);
        assert_eq!(sw.laps()[0], 1000);
        assert_eq!(sw.laps()[1], 2500);
    }

    // KEEP HERE: accesses private field state
    #[test]
    fn test_stopwatch_toggle() {
        let mut sw = Stopwatch::new();
        assert_eq!(sw.state, TimerState::Stopped);

        sw.toggle();
        assert_eq!(sw.state, TimerState::Running);

        sw.toggle();
        assert_eq!(sw.state, TimerState::Paused);

        sw.toggle();
        assert_eq!(sw.state, TimerState::Running);
    }

    // KEEP HERE: accesses private fields state, elapsed_ms, started_at, laps
    #[test]
    fn test_stopwatch_stop() {
        let mut sw = Stopwatch::new();
        sw.start();
        assert_eq!(sw.state, TimerState::Running);

        sw.stop();
        assert_eq!(sw.state, TimerState::Stopped);
        assert_eq!(sw.elapsed_ms, 0);
        assert!(sw.started_at.is_none());
        assert!(sw.laps.is_empty());
    }

    // KEEP HERE: accesses private fields elapsed_ms, laps
    #[test]
    fn test_stopwatch_reset() {
        let mut sw = Stopwatch::new();
        sw.start();
        sw.elapsed_ms = 5000;
        sw.laps.push(1000);

        sw.reset();
        assert_eq!(sw.elapsed_ms, 0);
        assert!(sw.started_at.is_some()); // Still running
        assert!(sw.laps.is_empty());
    }

    // KEEP HERE: accesses private fields elapsed_ms, laps
    #[test]
    fn test_stopwatch_reset_when_stopped() {
        let mut sw = Stopwatch::new();
        sw.elapsed_ms = 5000;
        sw.laps.push(1000);

        sw.reset();
        assert_eq!(sw.elapsed_ms, 0);
        assert!(sw.started_at.is_none()); // Not running
        assert!(sw.laps.is_empty());
    }

    // KEEP HERE: accesses private field elapsed_ms
    #[test]
    fn test_stopwatch_format_elapsed() {
        let mut sw = Stopwatch::new();
        sw.elapsed_ms = 3661000; // 1h 1m 1s
        assert_eq!(sw.format_elapsed(), "01:01:01");

        sw.elapsed_ms = 65000;
        assert_eq!(sw.format_elapsed(), "01:05");
    }

    // KEEP HERE: accesses private field elapsed_ms
    #[test]
    fn test_stopwatch_elapsed_seconds() {
        let mut sw = Stopwatch::new();
        assert_eq!(sw.elapsed_seconds(), 0.0);

        sw.elapsed_ms = 5500;
        assert_eq!(sw.elapsed_seconds(), 5.5);
    }

    // KEEP HERE: accesses private field show_laps
    #[test]
    fn test_stopwatch_show_laps() {
        let sw = Stopwatch::new().show_laps(false);
        assert!(!sw.show_laps);
    }

    // KEEP HERE: accesses private field max_laps
    #[test]
    fn test_stopwatch_max_laps() {
        let sw = Stopwatch::new().max_laps(10);
        assert_eq!(sw.max_laps, 10);
    }

    // KEEP HERE: accesses private field title
    #[test]
    fn test_stopwatch_title() {
        let sw = Stopwatch::new().title("My Stopwatch");
        assert_eq!(sw.title, Some("My Stopwatch".to_string()));
    }

    // KEEP HERE: accesses private field large_digits
    #[test]
    fn test_stopwatch_large_digits() {
        let sw = Stopwatch::new().large_digits(true);
        assert!(sw.large_digits);
    }

    // KEEP HERE: accesses private fields elapsed_ms, state
    #[test]
    fn test_stopwatch_default() {
        let sw = Stopwatch::default();
        assert_eq!(sw.elapsed_ms, 0);
        assert_eq!(sw.state, TimerState::Stopped);
    }
}
