//! Countdown timer widget

use super::{render_large_time, TimerFormat, TimerState};
use crate::style::Color;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::traits::WidgetProps;
use crate::widget::{RenderContext, View};
use crate::{impl_props_builders, impl_styled_view};
use std::time::Instant;

/// Countdown timer widget
#[derive(Clone, Debug)]
pub struct Timer {
    /// Total duration in milliseconds
    total_ms: u64,
    /// Remaining duration in milliseconds
    remaining_ms: u64,
    /// State
    state: TimerState,
    /// Start instant (when running)
    started_at: Option<Instant>,
    /// Paused remaining (when paused)
    paused_remaining: Option<u64>,
    /// Display format
    format: TimerFormat,
    /// Show progress bar
    show_progress: bool,
    /// Progress bar width
    progress_width: u16,
    /// Colors
    fg: Option<Color>,
    warning_fg: Option<Color>,
    danger_fg: Option<Color>,
    /// Warning threshold (seconds)
    warning_threshold: u64,
    /// Danger threshold (seconds)
    danger_threshold: u64,
    /// Title/label
    title: Option<String>,
    /// Show large digits
    large_digits: bool,
    /// Auto-restart
    auto_restart: bool,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Timer {
    /// Create a new countdown timer with duration in seconds
    pub fn countdown(seconds: u64) -> Self {
        let total_ms = seconds * 1000;
        Self {
            total_ms,
            remaining_ms: total_ms,
            state: TimerState::Stopped,
            started_at: None,
            paused_remaining: None,
            format: TimerFormat::default(),
            show_progress: true,
            progress_width: 30,
            fg: None,
            warning_fg: Some(Color::YELLOW),
            danger_fg: Some(Color::RED),
            warning_threshold: 60,
            danger_threshold: 10,
            title: None,
            large_digits: false,
            auto_restart: false,
            props: WidgetProps::new(),
        }
    }

    /// Create a pomodoro timer (25 minutes)
    pub fn pomodoro() -> Self {
        Self::countdown(25 * 60)
            .title("Pomodoro")
            .warning_threshold(5 * 60)
            .danger_threshold(60)
    }

    /// Create a short break timer (5 minutes)
    pub fn short_break() -> Self {
        Self::countdown(5 * 60).title("Short Break")
    }

    /// Create a long break timer (15 minutes)
    pub fn long_break() -> Self {
        Self::countdown(15 * 60).title("Long Break")
    }

    /// Set display format
    pub fn format(mut self, format: TimerFormat) -> Self {
        self.format = format;
        self
    }

    /// Show progress bar
    pub fn show_progress(mut self, show: bool) -> Self {
        self.show_progress = show;
        self
    }

    /// Set progress bar width
    pub fn progress_width(mut self, width: u16) -> Self {
        self.progress_width = width;
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set warning threshold in seconds
    pub fn warning_threshold(mut self, seconds: u64) -> Self {
        self.warning_threshold = seconds;
        self
    }

    /// Set danger threshold in seconds
    pub fn danger_threshold(mut self, seconds: u64) -> Self {
        self.danger_threshold = seconds;
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

    /// Enable auto-restart
    pub fn auto_restart(mut self, restart: bool) -> Self {
        self.auto_restart = restart;
        self
    }

    /// Start the timer
    pub fn start(&mut self) {
        if self.state == TimerState::Paused {
            // Resume from pause
            self.started_at = Some(Instant::now());
            self.remaining_ms = self.paused_remaining.unwrap_or(self.remaining_ms);
            self.paused_remaining = None;
        } else if self.state != TimerState::Running {
            self.started_at = Some(Instant::now());
            self.remaining_ms = self.total_ms;
        }
        self.state = TimerState::Running;
    }

    /// Pause the timer
    pub fn pause(&mut self) {
        if self.state == TimerState::Running {
            self.update();
            self.paused_remaining = Some(self.remaining_ms);
            self.state = TimerState::Paused;
        }
    }

    /// Stop and reset the timer
    pub fn stop(&mut self) {
        self.state = TimerState::Stopped;
        self.remaining_ms = self.total_ms;
        self.started_at = None;
        self.paused_remaining = None;
    }

    /// Reset the timer
    pub fn reset(&mut self) {
        self.remaining_ms = self.total_ms;
        if self.state == TimerState::Running {
            self.started_at = Some(Instant::now());
        }
    }

    /// Toggle between running and paused
    pub fn toggle(&mut self) {
        match self.state {
            TimerState::Running => self.pause(),
            TimerState::Paused | TimerState::Stopped | TimerState::Completed => self.start(),
        }
    }

    /// Update timer state (call each frame)
    pub fn update(&mut self) {
        if self.state != TimerState::Running {
            return;
        }

        if let Some(started) = self.started_at {
            let elapsed = started.elapsed().as_millis() as u64;
            let base = self.paused_remaining.unwrap_or(self.total_ms);

            if elapsed >= base {
                self.remaining_ms = 0;
                self.state = TimerState::Completed;

                if self.auto_restart {
                    self.remaining_ms = self.total_ms;
                    self.started_at = Some(Instant::now());
                    self.state = TimerState::Running;
                }
            } else {
                self.remaining_ms = base - elapsed;
            }
        }
    }

    /// Get remaining time in seconds
    pub fn remaining_seconds(&self) -> u64 {
        self.remaining_ms / 1000
    }

    /// Get progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        if self.total_ms == 0 {
            return 1.0;
        }
        1.0 - (self.remaining_ms as f32 / self.total_ms as f32)
    }

    /// Check if completed
    pub fn is_completed(&self) -> bool {
        self.state == TimerState::Completed
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        self.state == TimerState::Running
    }

    /// Get current state
    pub fn state(&self) -> TimerState {
        self.state
    }

    /// Format remaining time as string
    pub fn format_remaining(&self) -> String {
        let total_secs = self.remaining_ms / 1000;
        let ms = self.remaining_ms % 1000;
        let secs = total_secs % 60;
        let mins = (total_secs / 60) % 60;
        let hours = total_secs / 3600;

        match self.format {
            TimerFormat::Full => format!("{:02}:{:02}:{:02}", hours, mins, secs),
            TimerFormat::Short => {
                if hours > 0 {
                    format!("{:02}:{:02}:{:02}", hours, mins, secs)
                } else {
                    format!("{:02}:{:02}", mins, secs)
                }
            }
            TimerFormat::Precise => format!("{:02}.{:03}", secs, ms),
            TimerFormat::Compact => {
                if hours > 0 {
                    format!("{}h {}m", hours, mins)
                } else if mins > 0 {
                    format!("{}m {}s", mins, secs)
                } else {
                    format!("{}s", secs)
                }
            }
        }
    }

    /// Get appropriate color based on remaining time
    /// The normal color takes `color`; the warning and danger thresholds
    /// keep theirs - those are the reading.
    fn current_color(&self, ctx: &RenderContext) -> Color {
        let secs = self.remaining_seconds();
        if secs <= self.danger_threshold {
            self.danger_fg.unwrap_or(Color::RED)
        } else if secs <= self.warning_threshold {
            self.warning_fg.unwrap_or(Color::YELLOW)
        } else {
            self.fg.unwrap_or_else(|| ctx.css_color(Color::WHITE))
        }
    }
}

impl View for Timer {
    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::vstack;
        use crate::widget::Progress;
        use crate::widget::Text;

        let color = self.current_color(ctx);
        let mut content = vstack();

        // Title
        if let Some(title) = &self.title {
            content = content.child(Text::new(title).bold());
        }

        // Time display
        let time_str = self.format_remaining();
        if self.large_digits {
            // Use block characters for large display
            let digits = render_large_time(&time_str);
            for line in digits {
                content = content.child(Text::new(line).fg(color));
            }
        } else {
            content = content.child(Text::new(&time_str).fg(color).bold());
        }

        // Progress bar
        if self.show_progress {
            let progress = Progress::new(self.progress()).filled_color(color);
            content = content.child(progress);
        }

        // State indicator
        let state_text = match self.state {
            TimerState::Stopped => "Stopped",
            TimerState::Running => "Running",
            TimerState::Paused => "Paused",
            TimerState::Completed => "Completed!",
        };
        content = content.child(Text::new(state_text).fg(PLACEHOLDER_FG));

        content.render(ctx);
    }

    crate::impl_view_meta!("Timer");
}

impl_styled_view!(Timer);
impl_props_builders!(Timer);

#[cfg(test)]
mod tests {
    use super::*;

    // KEEP HERE: accesses private field remaining_ms
    #[test]
    fn test_timer_progress() {
        let mut timer = Timer::countdown(100);
        assert_eq!(timer.progress(), 0.0);

        timer.remaining_ms = 50000; // 50%
        assert!((timer.progress() - 0.5).abs() < 0.01);
    }

    // KEEP HERE: accesses private field title
    #[test]
    fn test_pomodoro() {
        let timer = Timer::pomodoro();
        assert_eq!(timer.remaining_seconds(), 25 * 60);
        assert_eq!(timer.title, Some("Pomodoro".to_string()));
    }

    // KEEP HERE: accesses private field started_at
    #[test]
    fn test_timer_stop() {
        let mut timer = Timer::countdown(60);
        timer.start();
        assert_eq!(timer.state(), TimerState::Running);

        timer.stop();
        assert_eq!(timer.state(), TimerState::Stopped);
        assert_eq!(timer.remaining_seconds(), 60);
        assert!(timer.started_at.is_none());
    }

    // KEEP HERE: accesses private field remaining_ms
    #[test]
    fn test_timer_reset() {
        let mut timer = Timer::countdown(60);
        timer.remaining_ms = 30000;
        timer.start();

        timer.reset();
        assert_eq!(timer.remaining_seconds(), 60);
        assert!(timer.started_at.is_some()); // Still running
    }

    // KEEP HERE: accesses private field remaining_ms
    #[test]
    fn test_timer_reset_when_stopped() {
        let mut timer = Timer::countdown(60);
        timer.remaining_ms = 30000;

        timer.reset();
        assert_eq!(timer.remaining_seconds(), 60);
        assert!(timer.started_at.is_none()); // Not running
    }

    // KEEP HERE: accesses private field state
    #[test]
    fn test_timer_is_completed() {
        let mut timer = Timer::countdown(60);
        assert!(!timer.is_completed());

        timer.state = TimerState::Completed;
        assert!(timer.is_completed());
    }

    // KEEP HERE: accesses private field title
    #[test]
    fn test_timer_short_break() {
        let timer = Timer::short_break();
        assert_eq!(timer.remaining_seconds(), 5 * 60);
        assert_eq!(timer.title, Some("Short Break".to_string()));
    }

    // KEEP HERE: accesses private field title
    #[test]
    fn test_timer_long_break() {
        let timer = Timer::long_break();
        assert_eq!(timer.remaining_seconds(), 15 * 60);
        assert_eq!(timer.title, Some("Long Break".to_string()));
    }

    // KEEP HERE: accesses private field show_progress
    #[test]
    fn test_timer_show_progress() {
        let timer = Timer::countdown(60).show_progress(false);
        assert!(!timer.show_progress);
    }

    // KEEP HERE: accesses private field progress_width
    #[test]
    fn test_timer_progress_width() {
        let timer = Timer::countdown(60).progress_width(50);
        assert_eq!(timer.progress_width, 50);
    }

    // KEEP HERE: accesses private field large_digits
    #[test]
    fn test_timer_large_digits() {
        let timer = Timer::countdown(60).large_digits(true);
        assert!(timer.large_digits);
    }

    // KEEP HERE: accesses private field auto_restart
    #[test]
    fn test_timer_auto_restart() {
        let timer = Timer::countdown(60).auto_restart(true);
        assert!(timer.auto_restart);
    }
}
