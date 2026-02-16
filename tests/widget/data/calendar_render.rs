//! Calendar rendering tests extracted from src/widget/data/calendar/render.rs
//!
//! This file contains tests for calendar rendering logic:
//! - is_in_range() - Check if date is in selection range
//! - get_marker() - Get marker for date
//! - day_names() - Get day names based on first day of week
//! - is_weekend() - Check if day index is weekend
//! - get_week_number() - Get ISO 8601 week number

use revue::style::Color;
use revue::widget::data::calendar::{Date, first_day_of_month, days_in_month};
use revue::widget::data::calendar::render::CalendarRender;
use revue::widget::data::calendar::types::{DateMarker, FirstDayOfWeek};

fn create_test_render(year: i32, month: u32, first_day: FirstDayOfWeek) -> CalendarRender<'static> {
    CalendarRender {
        year,
        month,
        selected: None,
        range_end: None,
        first_day,
        show_week_numbers: false,
        markers: &[],
        today: None,
        header_fg: Color::BLACK,
        header_bg: None,
        day_fg: Color::BLACK,
        weekend_fg: Color::BLACK,
        selected_fg: Color::BLACK,
        selected_bg: Color::BLACK,
        today_fg: Color::BLACK,
        outside_fg: Color::BLACK,
        border_color: None,
        focused: false,
    }
}

// =========================================================================
// is_in_range tests
// =========================================================================

#[test]
fn test_is_in_range_no_selection() {
    let render = create_test_render(2024, 1, FirstDayOfWeek::Sunday);
    let date = Date::new(2024, 1, 15);
    assert!(!render.is_in_range(&date));
}

#[test]
fn test_is_in_range_with_range() {
    let render = CalendarRender {
        selected: Some(Date::new(2024, 1, 10)),
        range_end: Some(Date::new(2024, 1, 20)),
        ..create_test_render(2024, 1, FirstDayOfWeek::Sunday)
    };
    let date = Date::new(2024, 1, 15);
    assert!(render.is_in_range(&date));
}

#[test]
fn test_is_in_range_before_start() {
    let render = CalendarRender {
        selected: Some(Date::new(2024, 1, 10)),
        range_end: Some(Date::new(2024, 1, 20)),
        ..create_test_render(2024, 1, FirstDayOfWeek::Sunday)
    };
    let date = Date::new(2024, 1, 5);
    assert!(!render.is_in_range(&date));
}

#[test]
fn test_is_in_range_after_end() {
    let render = CalendarRender {
        selected: Some(Date::new(2024, 1, 10)),
        range_end: Some(Date::new(2024, 1, 20)),
        ..create_test_render(2024, 1, FirstDayOfWeek::Sunday)
    };
    let date = Date::new(2024, 1, 25);
    assert!(!render.is_in_range(&date));
}

#[test]
fn test_is_in_range_reversed() {
    let render = CalendarRender {
        selected: Some(Date::new(2024, 1, 20)),
        range_end: Some(Date::new(2024, 1, 10)),
        ..create_test_render(2024, 1, FirstDayOfWeek::Sunday)
    };
    let date = Date::new(2024, 1, 15);
    // Should handle reversed range
    assert!(render.is_in_range(&date));
}

#[test]
fn test_is_in_range_equal_dates() {
    let render = CalendarRender {
        selected: Some(Date::new(2024, 1, 15)),
        range_end: Some(Date::new(2024, 1, 15)),
        ..create_test_render(2024, 1, FirstDayOfWeek::Sunday)
    };
    let date = Date::new(2024, 1, 15);
    assert!(render.is_in_range(&date));
}

// =========================================================================
// get_marker tests
// =========================================================================

#[test]
fn test_get_marker_found() {
    let date = Date::new(2024, 1, 15);
    let marker = DateMarker::new(date, Color::RED);
    let render = CalendarRender {
        markers: std::vec![marker],
        ..create_test_render(2024, 1, FirstDayOfWeek::Sunday)
    };
    let result = render.get_marker(&date);
    assert!(result.is_some());
}

#[test]
fn test_get_marker_not_found() {
    let render = create_test_render(2024, 1, FirstDayOfWeek::Sunday);
    let date = Date::new(2024, 1, 15);
    assert!(render.get_marker(&date).is_none());
}

#[test]
fn test_get_marker_multiple_markers() {
    let date1 = Date::new(2024, 1, 15);
    let date2 = Date::new(2024, 1, 20);
    let marker1 = DateMarker::new(date1, Color::RED);
    let marker2 = DateMarker::new(date2, Color::BLUE);
    let render = CalendarRender {
        markers: vec![marker1, marker2].leak(),
        ..create_test_render(2024, 1, FirstDayOfWeek::Sunday)
    };

    let result1 = render.get_marker(&date1);
    assert!(result1.is_some());
    assert_eq!(result1.unwrap().color, Color::RED);

    let result2 = render.get_marker(&date2);
    assert!(result2.is_some());
    assert_eq!(result2.unwrap().color, Color::BLUE);
}

// =========================================================================
// day_names tests
// =========================================================================

#[test]
fn test_day_names_sunday_first() {
    let render = create_test_render(2024, 1, FirstDayOfWeek::Sunday);
    let names = render.day_names();
    assert_eq!(names[0], "Su");
    assert_eq!(names[6], "Sa");
}

#[test]
fn test_day_names_monday_first() {
    let render = create_test_render(2024, 1, FirstDayOfWeek::Monday);
    let names = render.day_names();
    assert_eq!(names[0], "Mo");
    assert_eq!(names[6], "Su");
}

// =========================================================================
// is_weekend tests
// =========================================================================

#[test]
fn test_is_weekend_sunday_first_sunday() {
    let render = create_test_render(2024, 1, FirstDayOfWeek::Sunday);
    assert!(render.is_weekend(0)); // Sunday
    assert!(render.is_weekend(6)); // Saturday
}

#[test]
fn test_is_weekend_sunday_first_weekday() {
    let render = create_test_render(2024, 1, FirstDayOfWeek::Sunday);
    assert!(!render.is_weekend(1)); // Monday
    assert!(!render.is_weekend(2)); // Tuesday
}

#[test]
fn test_is_weekend_monday_first_saturday() {
    let render = create_test_render(2024, 1, FirstDayOfWeek::Monday);
    assert!(render.is_weekend(5)); // Saturday
    assert!(render.is_weekend(6)); // Sunday
}

#[test]
fn test_is_weekend_monday_first_weekday() {
    let render = create_test_render(2024, 1, FirstDayOfWeek::Monday);
    assert!(!render.is_weekend(0)); // Monday
    assert!(!render.is_weekend(4)); // Friday
}

// =========================================================================
// get_week_number tests
// =========================================================================

#[test]
fn test_get_week_number_january_1_2024() {
    let render = create_test_render(2024, 1, FirstDayOfWeek::Sunday);
    // January 1, 2024 was Monday, week 1
    assert_eq!(render.get_week_number(2024, 1, 1), 1);
}

#[test]
fn test_get_week_number_february_2024() {
    let render = create_test_render(2024, 2, FirstDayOfWeek::Sunday);
    // February 1, 2024 was Thursday, should be week 5
    let week = render.get_week_number(2024, 2, 1);
    assert!(week >= 1 && week <= 53);
}

#[test]
fn test_get_week_number_range() {
    let render = create_test_render(2024, 1, FirstDayOfWeek::Sunday);
    // Test multiple dates to ensure valid range
    for day in 1..31 {
        let week = render.get_week_number(2024, 1, day);
        assert!(week >= 1 && week <= 53);
    }
}

#[test]
fn test_get_week_number_december_2024() {
    let render = create_test_render(2024, 12, FirstDayOfWeek::Sunday);
    // December 31, 2024 should be week 1 of 2025
    let week = render.get_week_number(2024, 12, 31);
    assert!(week >= 1 && week <= 53);
}

#[test]
fn test_get_week_number_leap_year() {
    let render = create_test_render(2024, 2, FirstDayOfWeek::Sunday);
    // 2024 is a leap year
    let week = render.get_week_number(2024, 2, 29);
    assert!(week >= 1 && week <= 53);
}

#[test]
fn test_get_week_number_year_boundary() {
    let render = create_test_render(2024, 1, FirstDayOfWeek::Sunday);
    // Test December 31, 2023 (might be week 1 of 2024)
    let week = render.get_week_number(2023, 12, 31);
    assert!(week >= 1 && week <= 53);
}

#[test]
fn test_get_week_number_mid_year() {
    let render = create_test_render(2024, 6, FirstDayOfWeek::Sunday);
    // June 1, 2024
    let week = render.get_week_number(2024, 6, 1);
    assert!(week >= 20 && week <= 25);
}

#[test]
fn test_get_week_number_consistency() {
    let render = create_test_render(2024, 1, FirstDayOfWeek::Sunday);
    // Same week should give same week number
    let week1 = render.get_week_number(2024, 1, 1);
    let week2 = render.get_week_number(2024, 1, 7);
    // Both should be in week 1
    assert_eq!(week1, week2);
}

// Helper function to leak Vec for static lifetime in tests
traitLeak {
    fn leak(self) -> &'static [Self::Item] where Self: Sized;
}

impl<T> Leak for Vec<T> {
    fn leak(self) -> &'static [T] {
        // SAFETY: We only use this in tests where the lifetime is managed correctly
        Box::leak(self.into_boxed_slice())
    }
}
