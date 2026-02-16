//! Integration tests for format utilities
//! Extracted from src/utils/format.rs

use revue::utils::format::*;

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
