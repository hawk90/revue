//! Calendar utility function tests
//!
//! This file contains tests for calendar utility functions:
//! - days_in_month() - Get days in a month accounting for leap years
//! - is_leap_year() - Check if a year is a leap year
//! - first_day_of_month() - Calculate day of week for first day of month

use revue::widget::data::calendar::{days_in_month, first_day_of_month, is_leap_year};

// =========================================================================
// days_in_month tests
// =========================================================================

#[test]
fn test_days_in_month_invalid() {
    assert_eq!(days_in_month(2024, 13), 0);
}

#[test]
fn test_days_in_month_century_leap_year() {
    // 2000 is divisible by 400, so it's a leap year
    assert_eq!(days_in_month(2000, 2), 29);
}

#[test]
fn test_days_in_month_century_non_leap() {
    // 1900 is divisible by 100 but not 400
    assert_eq!(days_in_month(1900, 2), 28);
}

// =========================================================================
// is_leap_year tests
// =========================================================================

#[test]
fn test_is_leap_year_common() {
    assert!(!is_leap_year(2023));
}

#[test]
fn test_is_leap_year_divisible_by_4() {
    assert!(is_leap_year(2024));
}

#[test]
fn test_is_leap_year_century() {
    assert!(!is_leap_year(1900));
}

#[test]
fn test_is_leap_year_century_divisible_by_400() {
    assert!(is_leap_year(2000));
}

#[test]
fn test_is_leap_year_zero_and_negative() {
    // Proleptic Gregorian rules: year 0 is divisible by 400
    assert!(is_leap_year(0));
    assert!(is_leap_year(-4));
    assert!(!is_leap_year(-100));
    assert!(is_leap_year(-400));
}

// =========================================================================
// first_day_of_month tests
// =========================================================================

#[test]
fn test_first_day_of_month_january_2024() {
    // January 1, 2024 was a Monday (1)
    let day = first_day_of_month(2024, 1);
    assert_eq!(day, 1);
}

#[test]
fn test_first_day_of_month_february_2024() {
    // February 1, 2024 was a Thursday (4)
    let day = first_day_of_month(2024, 2);
    assert_eq!(day, 4);
}

#[test]
fn test_first_day_of_month_march_2024() {
    // March 1, 2024 was a Friday (5)
    let day = first_day_of_month(2024, 3);
    assert_eq!(day, 5);
}

#[test]
fn test_first_day_of_month_january_2023() {
    // January 1, 2023 was a Sunday (0)
    let day = first_day_of_month(2023, 1);
    assert_eq!(day, 0);
}

#[test]
fn test_first_day_of_month_february_2023() {
    // February 1, 2023 was a Wednesday (3)
    let day = first_day_of_month(2023, 2);
    assert_eq!(day, 3);
}

#[test]
fn test_first_day_of_month_range() {
    // Test that result is always in valid range (0-6)
    for year in 1900..=2100 {
        for month in 1..=12 {
            let day = first_day_of_month(year, month);
            assert!(day < 7);
        }
    }
}

#[test]
fn test_first_day_of_month_march_leap_year() {
    // March 1, 2000 (leap year) was a Wednesday (3)
    let day = first_day_of_month(2000, 3);
    assert_eq!(day, 3);
}
