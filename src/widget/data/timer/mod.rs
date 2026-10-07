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

mod countdown;
mod stopwatch;

pub use countdown::Timer;
pub use stopwatch::Stopwatch;

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

    // KEEP HERE: calls private function format_ms
    #[test]
    fn test_format_ms() {
        assert_eq!(format_ms(3661000, TimerFormat::Full), "01:01:01");
        assert_eq!(format_ms(65000, TimerFormat::Short), "01:05");
        assert_eq!(format_ms(5500, TimerFormat::Precise), "00:05.500");
        assert_eq!(format_ms(90000, TimerFormat::Compact), "1m 30s");
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
}
