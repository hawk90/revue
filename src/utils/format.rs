//! Formatting utilities
//!
//! Provides human-friendly formatting for durations, sizes, numbers, and more.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::format::{format_duration, format_size, format_relative_time};
//!
//! assert_eq!(format_duration_short(3661), "1h 1m");
//! assert_eq!(format_size(1536), "1.5 KB");
//! assert_eq!(format_relative_time(3600), "1 hour ago");
//! ```

use std::time::Duration;

// ============================================================================
// Duration Formatting
// ============================================================================

/// Decomposed duration parts
struct DurationParts {
    days: u64,
    hours: u64,
    minutes: u64,
    seconds: u64,
}

impl DurationParts {
    fn from_seconds(seconds: u64) -> Self {
        Self {
            days: seconds / 86400,
            hours: (seconds % 86400) / 3600,
            minutes: (seconds % 3600) / 60,
            seconds: seconds % 60,
        }
    }
}

/// Format a duration in seconds to a human-readable string
///
/// # Example
///
/// ```rust,ignore
/// assert_eq!(format_duration(90), "1 minute, 30 seconds");
/// assert_eq!(format_duration(3661), "1 hour, 1 minute, 1 second");
/// ```
pub fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        return "0 seconds".to_string();
    }

    let p = DurationParts::from_seconds(seconds);
    let mut parts = Vec::new();

    if p.days > 0 {
        parts.push(format!(
            "{} {}",
            p.days,
            if p.days == 1 { "day" } else { "days" }
        ));
    }
    if p.hours > 0 {
        parts.push(format!(
            "{} {}",
            p.hours,
            if p.hours == 1 { "hour" } else { "hours" }
        ));
    }
    if p.minutes > 0 {
        parts.push(format!(
            "{} {}",
            p.minutes,
            if p.minutes == 1 { "minute" } else { "minutes" }
        ));
    }
    if p.seconds > 0 || parts.is_empty() {
        parts.push(format!(
            "{} {}",
            p.seconds,
            if p.seconds == 1 { "second" } else { "seconds" }
        ));
    }

    parts.join(", ")
}

/// Format a duration in short form
///
/// # Example
///
/// ```rust,ignore
/// assert_eq!(format_duration_short(90), "1m 30s");
/// assert_eq!(format_duration_short(3661), "1h 1m 1s");
/// assert_eq!(format_duration_short(86400), "1d");
/// ```
pub fn format_duration_short(seconds: u64) -> String {
    if seconds == 0 {
        return "0s".to_string();
    }

    let p = DurationParts::from_seconds(seconds);
    let mut parts = Vec::new();

    if p.days > 0 {
        parts.push(format!("{}d", p.days));
    }
    if p.hours > 0 {
        parts.push(format!("{}h", p.hours));
    }
    if p.minutes > 0 {
        parts.push(format!("{}m", p.minutes));
    }
    if p.seconds > 0 || parts.is_empty() {
        parts.push(format!("{}s", p.seconds));
    }

    parts.join(" ")
}

/// Format a duration showing only the largest unit
///
/// # Example
///
/// ```rust,ignore
/// assert_eq!(format_duration_compact(90), "1m");
/// assert_eq!(format_duration_compact(3661), "1h");
/// assert_eq!(format_duration_compact(86400), "1d");
/// ```
pub fn format_duration_compact(seconds: u64) -> String {
    if seconds == 0 {
        return "0s".to_string();
    }

    let p = DurationParts::from_seconds(seconds);

    if p.days > 0 {
        format!("{}d", p.days)
    } else if p.hours > 0 {
        format!("{}h", p.hours)
    } else if p.minutes > 0 {
        format!("{}m", p.minutes)
    } else {
        format!("{}s", p.seconds)
    }
}

/// Format a Duration struct
pub fn format_std_duration(duration: Duration) -> String {
    format_duration(duration.as_secs())
}

/// Format a Duration struct in short form
pub fn format_std_duration_short(duration: Duration) -> String {
    format_duration_short(duration.as_secs())
}

// ============================================================================
// Relative Time Formatting
// ============================================================================

/// Format seconds as relative time (time ago)
///
/// # Example
///
/// ```rust,ignore
/// assert_eq!(format_relative_time(30), "just now");
/// assert_eq!(format_relative_time(90), "1 minute ago");
/// assert_eq!(format_relative_time(7200), "2 hours ago");
/// ```
pub fn format_relative_time(seconds_ago: u64) -> String {
    if seconds_ago < 60 {
        return "just now".to_string();
    }

    let minutes = seconds_ago / 60;
    if minutes < 60 {
        return format!(
            "{} {} ago",
            minutes,
            if minutes == 1 { "minute" } else { "minutes" }
        );
    }

    let hours = seconds_ago / 3600;
    if hours < 24 {
        return format!(
            "{} {} ago",
            hours,
            if hours == 1 { "hour" } else { "hours" }
        );
    }

    let days = seconds_ago / 86400;
    if days < 7 {
        return format!("{} {} ago", days, if days == 1 { "day" } else { "days" });
    }

    let weeks = days / 7;
    if weeks < 4 {
        return format!(
            "{} {} ago",
            weeks,
            if weeks == 1 { "week" } else { "weeks" }
        );
    }

    let months = days / 30;
    if months < 12 {
        return format!(
            "{} {} ago",
            months,
            if months == 1 { "month" } else { "months" }
        );
    }

    let years = days / 365;
    format!(
        "{} {} ago",
        years,
        if years == 1 { "year" } else { "years" }
    )
}

/// Format relative time in short form
///
/// # Example
///
/// ```rust,ignore
/// assert_eq!(format_relative_time_short(30), "now");
/// assert_eq!(format_relative_time_short(90), "1m ago");
/// assert_eq!(format_relative_time_short(7200), "2h ago");
/// ```
pub fn format_relative_time_short(seconds_ago: u64) -> String {
    if seconds_ago < 60 {
        return "now".to_string();
    }

    let minutes = seconds_ago / 60;
    if minutes < 60 {
        return format!("{}m ago", minutes);
    }

    let hours = seconds_ago / 3600;
    if hours < 24 {
        return format!("{}h ago", hours);
    }

    let days = seconds_ago / 86400;
    if days < 7 {
        return format!("{}d ago", days);
    }

    let weeks = days / 7;
    if weeks < 4 {
        return format!("{}w ago", weeks);
    }

    let months = days / 30;
    if months < 12 {
        return format!("{}mo ago", months);
    }

    let years = days / 365;
    format!("{}y ago", years)
}

// ============================================================================
// Size Formatting
// ============================================================================

/// Format bytes as a human-readable size
///
/// Uses binary units (1 KB = 1024 bytes).
///
/// # Example
///
/// ```rust,ignore
/// assert_eq!(format_size(500), "500 B");
/// assert_eq!(format_size(1024), "1.0 KB");
/// assert_eq!(format_size(1536), "1.5 KB");
/// assert_eq!(format_size(1048576), "1.0 MB");
/// ```
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;
    const PB: u64 = TB * 1024;

    if bytes < KB {
        format!("{} B", bytes)
    } else if bytes < MB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else if bytes < GB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes < TB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes < PB {
        format!("{:.1} TB", bytes as f64 / TB as f64)
    } else {
        format!("{:.1} PB", bytes as f64 / PB as f64)
    }
}

/// Format bytes using SI units (1 KB = 1000 bytes)
pub fn format_size_si(bytes: u64) -> String {
    const KB: u64 = 1000;
    const MB: u64 = KB * 1000;
    const GB: u64 = MB * 1000;
    const TB: u64 = GB * 1000;
    const PB: u64 = TB * 1000;

    if bytes < KB {
        format!("{} B", bytes)
    } else if bytes < MB {
        format!("{:.1} kB", bytes as f64 / KB as f64)
    } else if bytes < GB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes < TB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes < PB {
        format!("{:.1} TB", bytes as f64 / TB as f64)
    } else {
        format!("{:.1} PB", bytes as f64 / PB as f64)
    }
}

/// Format bytes in compact form (no space, shorter units)
///
/// # Example
///
/// ```rust,ignore
/// assert_eq!(format_size_compact(1536), "1.5K");
/// assert_eq!(format_size_compact(1048576), "1M");
/// ```
pub fn format_size_compact(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes < KB {
        format!("{}", bytes)
    } else if bytes < MB {
        let val = bytes as f64 / KB as f64;
        if val == val.trunc() {
            format!("{}K", val as u64)
        } else {
            format!("{:.1}K", val)
        }
    } else if bytes < GB {
        let val = bytes as f64 / MB as f64;
        if val == val.trunc() {
            format!("{}M", val as u64)
        } else {
            format!("{:.1}M", val)
        }
    } else if bytes < TB {
        let val = bytes as f64 / GB as f64;
        if val == val.trunc() {
            format!("{}G", val as u64)
        } else {
            format!("{:.1}G", val)
        }
    } else {
        let val = bytes as f64 / TB as f64;
        if val == val.trunc() {
            format!("{}T", val as u64)
        } else {
            format!("{:.1}T", val)
        }
    }
}

/// Format a transfer rate (bytes per second)
///
/// # Example
///
/// ```rust,ignore
/// assert_eq!(format_rate(1536), "1.5 KB/s");
/// assert_eq!(format_rate(1048576), "1.0 MB/s");
/// ```
pub fn format_rate(bytes_per_second: u64) -> String {
    format!("{}/s", format_size(bytes_per_second))
}

/// Format a transfer rate in compact form
pub fn format_rate_compact(bytes_per_second: u64) -> String {
    format!("{}/s", format_size_compact(bytes_per_second))
}

// ============================================================================
// Number Formatting
// ============================================================================

/// Format a number with thousand separators
///
/// # Example
///
/// ```rust,ignore
/// assert_eq!(format_number(1234567), "1,234,567");
/// ```
pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(*ch);
    }

    result
}

/// Format a number with K/M/B suffixes
///
/// # Example
///
/// ```rust,ignore
/// assert_eq!(format_number_short(1500), "1.5K");
/// assert_eq!(format_number_short(1500000), "1.5M");
/// ```
pub fn format_number_short(n: u64) -> String {
    if n < 1000 {
        n.to_string()
    } else if n < 1_000_000 {
        let val = n as f64 / 1000.0;
        if val == val.trunc() {
            format!("{}K", val as u64)
        } else {
            format!("{:.1}K", val)
        }
    } else if n < 1_000_000_000 {
        let val = n as f64 / 1_000_000.0;
        if val == val.trunc() {
            format!("{}M", val as u64)
        } else {
            format!("{:.1}M", val)
        }
    } else {
        let val = n as f64 / 1_000_000_000.0;
        if val == val.trunc() {
            format!("{}B", val as u64)
        } else {
            format!("{:.1}B", val)
        }
    }
}

/// Format a percentage
///
/// # Example
///
/// ```rust,ignore
/// assert_eq!(format_percent(0.5), "50%");
/// assert_eq!(format_percent(0.333), "33%");
/// ```
pub fn format_percent(ratio: f64) -> String {
    // Clamp to i32 range to prevent overflow, then format
    let percent = (ratio * 100.0).round();
    let clamped = percent.clamp(i32::MIN as f64, i32::MAX as f64) as i32;
    format!("{}%", clamped)
}

/// Format a percentage with decimals
pub fn format_percent_precise(ratio: f64, decimals: usize) -> String {
    format!("{:.1$}%", ratio * 100.0, decimals)
}

// ============================================================================
// Miscellaneous
// ============================================================================

/// Pluralize a word based on count
///
/// # Example
///
/// ```rust,ignore
/// assert_eq!(pluralize(1, "item", "items"), "1 item");
/// assert_eq!(pluralize(5, "item", "items"), "5 items");
/// ```
pub fn pluralize(count: u64, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("{} {}", count, singular)
    } else {
        format!("{} {}", count, plural)
    }
}

/// Pluralize with "s" suffix
pub fn pluralize_s(count: u64, word: &str) -> String {
    if count == 1 {
        format!("{} {}", count, word)
    } else {
        format!("{} {}s", count, word)
    }
}

/// Ordinal suffix for a number
///
/// # Example
///
/// ```rust,ignore
/// assert_eq!(ordinal(1), "1st");
/// assert_eq!(ordinal(2), "2nd");
/// assert_eq!(ordinal(3), "3rd");
/// assert_eq!(ordinal(4), "4th");
/// assert_eq!(ordinal(11), "11th");
/// assert_eq!(ordinal(21), "21st");
/// ```
pub fn ordinal(n: u64) -> String {
    let suffix = match (n % 10, n % 100) {
        (1, 11) => "th",
        (2, 12) => "th",
        (3, 13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    };
    format!("{}{}", n, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0), "0 seconds");
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(60), "1 minute");
        assert_eq!(format_duration(90), "1 minute, 30 seconds");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3661), "1 hour, 1 minute, 1 second");
        assert_eq!(format_duration(86400), "1 day");
    }

    #[test]
    fn test_format_duration_short() {
        assert_eq!(format_duration_short(0), "0s");
        assert_eq!(format_duration_short(90), "1m 30s");
        assert_eq!(format_duration_short(3661), "1h 1m 1s");
        assert_eq!(format_duration_short(86400), "1d");
    }

    #[test]
    fn test_format_duration_compact() {
        assert_eq!(format_duration_compact(30), "30s");
        assert_eq!(format_duration_compact(90), "1m");
        assert_eq!(format_duration_compact(3661), "1h");
        assert_eq!(format_duration_compact(86400), "1d");
    }

    #[test]
    fn test_format_relative_time() {
        assert_eq!(format_relative_time(30), "just now");
        assert_eq!(format_relative_time(60), "1 minute ago");
        assert_eq!(format_relative_time(120), "2 minutes ago");
        assert_eq!(format_relative_time(3600), "1 hour ago");
        assert_eq!(format_relative_time(7200), "2 hours ago");
        assert_eq!(format_relative_time(86400), "1 day ago");
    }

    #[test]
    fn test_format_relative_time_short() {
        assert_eq!(format_relative_time_short(30), "now");
        assert_eq!(format_relative_time_short(60), "1m ago");
        assert_eq!(format_relative_time_short(3600), "1h ago");
        assert_eq!(format_relative_time_short(86400), "1d ago");
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(1048576), "1.0 MB");
        assert_eq!(format_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_format_size_compact() {
        assert_eq!(format_size_compact(500), "500");
        assert_eq!(format_size_compact(1024), "1K");
        assert_eq!(format_size_compact(1536), "1.5K");
        assert_eq!(format_size_compact(1048576), "1M");
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(100), "100");
        assert_eq!(format_number(1000), "1,000");
        assert_eq!(format_number(1234567), "1,234,567");
    }

    #[test]
    fn test_format_number_short() {
        assert_eq!(format_number_short(500), "500");
        assert_eq!(format_number_short(1500), "1.5K");
        assert_eq!(format_number_short(1000000), "1M");
        assert_eq!(format_number_short(1500000), "1.5M");
    }

    #[test]
    fn test_format_percent() {
        assert_eq!(format_percent(0.0), "0%");
        assert_eq!(format_percent(0.5), "50%");
        assert_eq!(format_percent(1.0), "100%");
        assert_eq!(format_percent(0.333), "33%");
    }

    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize(0, "item", "items"), "0 items");
        assert_eq!(pluralize(1, "item", "items"), "1 item");
        assert_eq!(pluralize(5, "item", "items"), "5 items");
    }

    #[test]
    fn test_ordinal() {
        assert_eq!(ordinal(1), "1st");
        assert_eq!(ordinal(2), "2nd");
        assert_eq!(ordinal(3), "3rd");
        assert_eq!(ordinal(4), "4th");
        assert_eq!(ordinal(11), "11th");
        assert_eq!(ordinal(12), "12th");
        assert_eq!(ordinal(13), "13th");
        assert_eq!(ordinal(21), "21st");
        assert_eq!(ordinal(22), "22nd");
        assert_eq!(ordinal(23), "23rd");
    }

    #[test]
    fn test_format_rate() {
        assert_eq!(format_rate(1024), "1.0 KB/s");
        assert_eq!(format_rate(1048576), "1.0 MB/s");
    }

    // Edge case tests for large values

    #[test]
    fn test_format_size_petabyte_range() {
        // Test very large values that might lose precision with f64
        let pb = 1_000_000_000_000_000_u64; // 1 PB
        let result = format_size(pb);
        // Should handle without panic, format may vary
        assert!(result.contains("PB") || result.contains("TB") || result.contains("EB"));
    }

    #[test]
    fn test_duration_parts_max_seconds() {
        // Test with u64::MAX - should handle gracefully
        let parts = DurationParts::from_seconds(u64::MAX);
        // Should produce some reasonable output without panic
        let _format = format!(
            "{}d {}h {}m {}s",
            parts.days, parts.hours, parts.minutes, parts.seconds
        );
        // We just verify it doesn't panic - the exact values aren't important
        assert!(parts.days > 0 || parts.hours > 0);
    }

    #[test]
    fn test_duration_parts_zero() {
        let parts = DurationParts::from_seconds(0);
        assert_eq!(parts.days, 0);
        assert_eq!(parts.hours, 0);
        assert_eq!(parts.minutes, 0);
        assert_eq!(parts.seconds, 0);
    }
}
