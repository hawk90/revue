//! Timer and Stopwatch widgets
//!
//! Interactive countdown timer and stopwatch with visual displays.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Timer, Stopwatch, timer, stopwatch};
//!
//! // Countdown timer (5 minutes)
//! let countdown = Timer::countdown(5 * 60)
//!     .on_complete(|| println!("Time's up!"));
//!
//! // Stopwatch
//! let sw = Stopwatch::new();
//!
//! // Pomodoro timer
//! let pomodoro = Timer::pomodoro();
//! ```

use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::widget::{RenderContext, View};
use crate::{impl_props_builders, impl_styled_view};
use std::time::Instant;

/// Timer state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerState {
    /// Timer is stopped
    Stopped,
    /// Timer is running
    Running,
    /// Timer is paused
    Paused,
    /// Timer has completed
    Completed,
}

/// Timer display format
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimerFormat {
    /// HH:MM:SS
    #[default]
    Full,
    /// MM:SS
    Short,
    /// SS.ms
    Precise,
    /// Compact (1h 23m)
    Compact,
}

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
    fn current_color(&self) -> Color {
        let secs = self.remaining_seconds();
        if secs <= self.danger_threshold {
            self.danger_fg.unwrap_or(Color::RED)
        } else if secs <= self.warning_threshold {
            self.warning_fg.unwrap_or(Color::YELLOW)
        } else {
            self.fg.unwrap_or(Color::WHITE)
        }
    }
}

impl View for Timer {
    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::vstack;
        use crate::widget::Progress;
        use crate::widget::Text;

        let color = self.current_color();
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
        content = content.child(Text::new(state_text).fg(Color::rgb(128, 128, 128)));

        content.render(ctx);
    }

    crate::impl_view_meta!("Timer");
}

impl_styled_view!(Timer);
impl_props_builders!(Timer);

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

        let color = self.fg.unwrap_or(Color::WHITE);
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
        content = content.child(Text::new(state_text).fg(Color::rgb(128, 128, 128)));

        // Lap times
        if self.show_laps && !self.laps.is_empty() {
            content = content.child(Text::new("Laps:").fg(Color::rgb(180, 180, 180)));

            let start = self.laps.len().saturating_sub(self.max_laps);
            for (i, &lap_ms) in self.laps.iter().skip(start).enumerate() {
                let lap_num = start + i + 1;
                let lap_str = format!("  #{}: {}", lap_num, format_ms(lap_ms, self.format));
                content = content.child(Text::new(lap_str).fg(Color::rgb(150, 150, 150)));
            }
        }

        content.render(ctx);
    }

    crate::impl_view_meta!("Stopwatch");
}

impl_styled_view!(Stopwatch);
impl_props_builders!(Stopwatch);

/// Format milliseconds
fn format_ms(ms: u64, format: TimerFormat) -> String {
    let total_secs = ms / 1000;
    let millis = ms % 1000;
    let secs = total_secs % 60;
    let mins = (total_secs / 60) % 60;
    let hours = total_secs / 3600;

    match format {
        TimerFormat::Full => format!("{:02}:{:02}:{:02}", hours, mins, secs),
        TimerFormat::Short => {
            if hours > 0 {
                format!("{:02}:{:02}:{:02}", hours, mins, secs)
            } else {
                format!("{:02}:{:02}", mins, secs)
            }
        }
        TimerFormat::Precise => format!("{:02}:{:02}.{:03}", mins, secs, millis),
        TimerFormat::Compact => {
            if hours > 0 {
                format!("{}h {}m {}s", hours, mins, secs)
            } else if mins > 0 {
                format!("{}m {}s", mins, secs)
            } else {
                format!("{}.{}s", secs, millis / 100)
            }
        }
    }
}

/// Render time string with large block characters
fn render_large_time(time: &str) -> Vec<String> {
    const PATTERNS: [[&str; 3]; 11] = [
        ["█▀█", "█ █", "▀▀▀"], // 0
        [" ▀█", "  █", "  ▀"], // 1
        ["▀▀█", "█▀▀", "▀▀▀"], // 2
        ["▀▀█", " ▀█", "▀▀▀"], // 3
        ["█ █", "▀▀█", "  ▀"], // 4
        ["█▀▀", "▀▀█", "▀▀▀"], // 5
        ["█▀▀", "█▀█", "▀▀▀"], // 6
        ["▀▀█", "  █", "  ▀"], // 7
        ["█▀█", "█▀█", "▀▀▀"], // 8
        ["█▀█", "▀▀█", "▀▀▀"], // 9
        [" ", "•", " "],       // :
    ];

    let mut lines = vec![String::new(), String::new(), String::new()];

    for c in time.chars() {
        let idx = match c {
            '0'..='9' => (c as usize) - ('0' as usize),
            ':' => 10,
            _ => continue,
        };

        for (i, line) in lines.iter_mut().enumerate() {
            line.push_str(PATTERNS[idx][i]);
            line.push(' ');
        }
    }

    lines
}

/// Create a countdown timer
pub fn timer(seconds: u64) -> Timer {
    Timer::countdown(seconds)
}

/// Create a stopwatch
pub fn stopwatch() -> Stopwatch {
    Stopwatch::new()
}

/// Create a pomodoro timer
pub fn pomodoro() -> Timer {
    Timer::pomodoro()
}

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

    // KEEP HERE: calls private function format_ms
    #[test]
    fn test_format_ms() {
        assert_eq!(format_ms(3661000, TimerFormat::Full), "01:01:01");
        assert_eq!(format_ms(65000, TimerFormat::Short), "01:05");
        assert_eq!(format_ms(5500, TimerFormat::Precise), "00:05.500");
        assert_eq!(format_ms(90000, TimerFormat::Compact), "1m 30s");
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

    // KEEP HERE: calls private function format_ms
    #[test]
    fn test_format_ms_compact_seconds_only() {
        assert_eq!(format_ms(500, TimerFormat::Compact), "0.5s");
    }

    // KEEP HERE: calls private function format_ms
    #[test]
    fn test_format_ms_compact_hours() {
        assert_eq!(format_ms(3665000, TimerFormat::Compact), "1h 1m 5s");
    }

    // KEEP HERE: calls private function render_large_time
    #[test]
    fn test_render_large_time() {
        let result = render_large_time("12:34");
        assert_eq!(result.len(), 3);
        assert!(result[0].contains('█'));
        assert!(result[1].contains('█'));
        assert!(result[2].contains('▀'));
    }

    // KEEP HERE: calls private function render_large_time
    #[test]
    fn test_render_large_time_with_colon() {
        let result = render_large_time("1:23");
        assert_eq!(result.len(), 3);
        // Each digit is 4 chars (3 pattern + 1 space), colon is 2 chars
        // "1" (4) + ":" (2) + "2" (4) + "3" (4) = 14 chars roughly
        assert!(!result[0].is_empty());
        assert!(!result[1].is_empty());
        assert!(!result[2].is_empty());
    }

    // KEEP HERE: calls private function render_large_time
    #[test]
    fn test_render_large_time_empty() {
        let result = render_large_time("");
        assert_eq!(result.len(), 3);
        assert!(result[0].is_empty());
        assert!(result[1].is_empty());
        assert!(result[2].is_empty());
    }

    // KEEP HERE: calls private function render_large_time
    #[test]
    fn test_render_large_time_invalid_chars() {
        let result = render_large_time("ab:cd");
        // Invalid chars should be skipped
        assert_eq!(result.len(), 3);
    }

    // KEEP HERE: accesses private fields elapsed_ms, state
    #[test]
    fn test_stopwatch_default() {
        let sw = Stopwatch::default();
        assert_eq!(sw.elapsed_ms, 0);
        assert_eq!(sw.state, TimerState::Stopped);
    }
}
