//! Date type tests extracted from src/widget/data/calendar/date.rs
//!
//! This file contains tests for the Date struct, including:
//! - Date creation and validation
//! - Date arithmetic (add/subtract days)
//! - Date comparisons (PartialOrd, PartialEq)
//! - Leap year handling
//! - Date navigation (prev_day, next_day)

use revue::widget::data::calendar::Date;

#[test]
fn test_date_new() {
    let date = Date::new(2024, 6, 15);
    assert_eq!(date.year, 2024);
    assert_eq!(date.month, 6);
    assert_eq!(date.day, 15);
}

#[test]
fn test_date_default() {
    let date = Date::default();
    assert_eq!(date.year, 2025);
    assert_eq!(date.month, 1);
    assert_eq!(date.day, 1);
}

#[test]
fn test_today() {
    let today = Date::today();
    // Just verify it returns a valid date
    assert!(today.is_valid());
}

#[test]
fn test_is_valid_true() {
    let date = Date::new(2024, 2, 15); // Feb 15, 2024 (leap year)
    assert!(date.is_valid());
}

#[test]
fn test_is_valid_invalid_month() {
    let date = Date::new(2024, 13, 1);
    assert!(!date.is_valid());
}

#[test]
fn test_is_valid_invalid_day_too_small() {
    let date = Date::new(2024, 1, 0);
    assert!(!date.is_valid());
}

#[test]
fn test_is_valid_invalid_day_too_large() {
    let date = Date::new(2024, 1, 32);
    assert!(!date.is_valid());
}

#[test]
fn test_is_valid_february_29_leap_year() {
    let date = Date::new(2024, 2, 29); // 2024 is divisible by 4
    assert!(date.is_valid());
}

#[test]
fn test_is_valid_february_29_non_leap_year() {
    let date = Date::new(2023, 2, 29); // 2023 is not divisible by 4
    assert!(!date.is_valid());
}

#[test]
fn test_weekday() {
    let date = Date::new(2025, 1, 1); // Jan 1, 2025 is Wednesday
                                          // The weekday depends on first_day_of_month calculation
    let weekday = date.weekday();
    assert!(weekday < 7);
}

#[test]
fn test_prev_day_same_month() {
    let date = Date::new(2024, 6, 15);
    let prev = date.prev_day();
    assert_eq!(prev.year, 2024);
    assert_eq!(prev.month, 6);
    assert_eq!(prev.day, 14);
}

#[test]
fn test_prev_day_month_boundary() {
    let date = Date::new(2024, 3, 1);
    let prev = date.prev_day();
    assert_eq!(prev.year, 2024);
    assert_eq!(prev.month, 2);
    assert_eq!(prev.day, 29); // 2024 is leap year
}

#[test]
fn test_prev_day_year_boundary() {
    let date = Date::new(2024, 1, 1);
    let prev = date.prev_day();
    assert_eq!(prev.year, 2023);
    assert_eq!(prev.month, 12);
    assert_eq!(prev.day, 31);
}

#[test]
fn test_next_day_same_month() {
    let date = Date::new(2024, 6, 15);
    let next = date.next_day();
    assert_eq!(next.year, 2024);
    assert_eq!(next.month, 6);
    assert_eq!(next.day, 16);
}

#[test]
fn test_next_day_month_boundary() {
    let date = Date::new(2024, 1, 31);
    let next = date.next_day();
    assert_eq!(next.year, 2024);
    assert_eq!(next.month, 2);
    assert_eq!(next.day, 1);
}

#[test]
fn test_next_day_year_boundary() {
    let date = Date::new(2024, 12, 31);
    let next = date.next_day();
    assert_eq!(next.year, 2025);
    assert_eq!(next.month, 1);
    assert_eq!(next.day, 1);
}

#[test]
fn test_subtract_days_within_month() {
    let date = Date::new(2024, 6, 15);
    let result = date.subtract_days(5);
    assert_eq!(result.year, 2024);
    assert_eq!(result.month, 6);
    assert_eq!(result.day, 10);
}

#[test]
fn test_subtract_days_cross_month() {
    let date = Date::new(2024, 3, 5);
    let result = date.subtract_days(10);
    assert_eq!(result.year, 2024);
    assert_eq!(result.month, 2);
    assert_eq!(result.day, 24); // Feb 2024 has 29 days
}

#[test]
fn test_subtract_days_zero() {
    let date = Date::new(2024, 6, 15);
    let result = date.subtract_days(0);
    assert_eq!(result, date);
}

#[test]
fn test_add_days_within_month() {
    let date = Date::new(2024, 6, 15);
    let result = date.add_days(5);
    assert_eq!(result.year, 2024);
    assert_eq!(result.month, 6);
    assert_eq!(result.day, 20);
}

#[test]
fn test_add_days_cross_month() {
    let date = Date::new(2024, 2, 25);
    let result = date.add_days(5);
    assert_eq!(result.year, 2024);
    assert_eq!(result.month, 3);
    assert_eq!(result.day, 1);
}

#[test]
fn test_add_days_zero() {
    let date = Date::new(2024, 6, 15);
    let result = date.add_days(0);
    assert_eq!(result, date);
}

#[test]
fn test_add_subtract_roundtrip() {
    let date = Date::new(2024, 6, 15);
    let added = date.add_days(10);
    let back = added.subtract_days(10);
    assert_eq!(back, date);
}

#[test]
fn test_date_copy_clone() {
    let date1 = Date::new(2024, 6, 15);
    let date2 = date1;
    assert_eq!(date2, date1);

    let date3 = date1.clone();
    assert_eq!(date3, date1);
}

#[test]
fn test_date_partial_eq() {
    let date1 = Date::new(2024, 6, 15);
    let date2 = Date::new(2024, 6, 15);
    assert_eq!(date1, date2);

    let date3 = Date::new(2024, 6, 16);
    assert_ne!(date1, date3);
}

#[test]
fn test_date_partial_ord() {
    let date1 = Date::new(2024, 6, 15);
    let date2 = Date::new(2024, 6, 16);
    assert!(date1 < date2);
    assert!(date2 > date1);

    let date3 = Date::new(2024, 7, 1);
    assert!(date1 < date3);
}

#[test]
fn test_prev_next_symmetry() {
    let date = Date::new(2024, 6, 15);
    let prev = date.prev_day();
    let back = prev.next_day();
    assert_eq!(back, date);
}

#[test]
fn test_february_leap_year() {
    let date = Date::new(2024, 2, 28);
    let next = date.next_day();
    assert_eq!(next.day, 29); // 2024 is leap year
}

#[test]
fn test_february_non_leap_year() {
    let date = Date::new(2023, 2, 28);
    let next = date.next_day();
    assert_eq!(next.month, 3); // Should roll over to March
    assert_eq!(next.day, 1);
}

#[test]
fn test_subtract_large_amount() {
    let date = Date::new(2024, 6, 15);
    let result = date.subtract_days(100);
    // Should go back about 3 months
    assert!(result.year <= 2024);
    assert!(result.month < 6);
}

#[test]
fn test_add_large_amount() {
    let date = Date::new(2024, 6, 15);
    let result = date.add_days(100);
    // Should go forward about 3 months
    assert!(result.year >= 2024);
}

#[test]
fn test_date_ord_total() {
    let date1 = Date::new(2024, 1, 1);
    let date2 = Date::new(2024, 12, 31);
    assert!(date1 < date2);
}

#[test]
fn test_is_valid_all_months() {
    for month in 1..=12 {
        let date = Date::new(2024, month, 1);
        assert!(date.is_valid(), "Month {} should be valid", month);
    }
}

#[test]
fn test_is_valid_year_boundary() {
    // Year 0 is valid
    let date = Date::new(0, 1, 1);
    assert!(date.is_valid());

    // Negative year is valid
    let date = Date::new(-100, 6, 15);
    assert!(date.is_valid());
}
