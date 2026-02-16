//! Calendar type tests extracted from src/widget/data/calendar/types.rs
//!
//! This file contains tests for calendar type definitions:
//! - CalendarMode enum (Month, Year, Week)
//! - FirstDayOfWeek enum (Sunday, Monday)
//! - DateMarker struct

use revue::style::Color;
use revue::widget::data::calendar::Date;
use revue::widget::data::calendar::types::{CalendarMode, DateMarker, FirstDayOfWeek};

// =========================================================================
// CalendarMode enum tests
// =========================================================================

#[test]
fn test_calendar_mode_default() {
    let mode = CalendarMode::default();
    assert_eq!(mode, CalendarMode::Month);
}

#[test]
fn test_calendar_mode_clone() {
    let mode = CalendarMode::Year;
    assert_eq!(mode.clone(), CalendarMode::Year);
}

#[test]
fn test_calendar_mode_copy() {
    let mode1 = CalendarMode::Week;
    let mode2 = mode1;
    assert_eq!(mode2, CalendarMode::Week);
}

#[test]
fn test_calendar_mode_partial_eq() {
    let mode1 = CalendarMode::Month;
    let mode2 = CalendarMode::Month;
    assert_eq!(mode1, mode2);
}

#[test]
fn test_calendar_mode_partial_ne() {
    let mode1 = CalendarMode::Month;
    let mode2 = CalendarMode::Year;
    assert_ne!(mode1, mode2);
}

#[test]
fn test_calendar_mode_month() {
    let mode = CalendarMode::Month;
    assert_eq!(mode, CalendarMode::Month);
}

#[test]
fn test_calendar_mode_year() {
    let mode = CalendarMode::Year;
    assert_eq!(mode, CalendarMode::Year);
}

#[test]
fn test_calendar_mode_week() {
    let mode = CalendarMode::Week;
    assert_eq!(mode, CalendarMode::Week);
}

// =========================================================================
// FirstDayOfWeek enum tests
// =========================================================================

#[test]
fn test_first_day_of_week_default() {
    let day = FirstDayOfWeek::default();
    assert_eq!(day, FirstDayOfWeek::Sunday);
}

#[test]
fn test_first_day_of_week_clone() {
    let day = FirstDayOfWeek::Monday;
    assert_eq!(day.clone(), FirstDayOfWeek::Monday);
}

#[test]
fn test_first_day_of_week_copy() {
    let day1 = FirstDayOfWeek::Monday;
    let day2 = day1;
    assert_eq!(day2, FirstDayOfWeek::Monday);
}

#[test]
fn test_first_day_of_week_partial_eq() {
    let day1 = FirstDayOfWeek::Sunday;
    let day2 = FirstDayOfWeek::Sunday;
    assert_eq!(day1, day2);
}

#[test]
fn test_first_day_of_week_partial_ne() {
    let day1 = FirstDayOfWeek::Sunday;
    let day2 = FirstDayOfWeek::Monday;
    assert_ne!(day1, day2);
}

#[test]
fn test_first_day_of_week_sunday() {
    let day = FirstDayOfWeek::Sunday;
    assert_eq!(day, FirstDayOfWeek::Sunday);
}

#[test]
fn test_first_day_of_week_monday() {
    let day = FirstDayOfWeek::Monday;
    assert_eq!(day, FirstDayOfWeek::Monday);
}

// =========================================================================
// DateMarker::new tests
// =========================================================================

#[test]
fn test_date_marker_new() {
    let date = Date {
        year: 2024,
        month: 1,
        day: 1,
    };
    let marker = DateMarker::new(date, Color::RED);
    assert_eq!(marker.date.year, 2024);
    assert_eq!(marker.date.month, 1);
    assert_eq!(marker.date.day, 1);
    assert_eq!(marker.color, Color::RED);
    assert!(marker.symbol.is_none());
}

#[test]
fn test_date_marker_clone() {
    let date = Date {
        year: 2024,
        month: 1,
        day: 1,
    };
    let marker1 = DateMarker::new(date, Color::BLUE);
    let marker2 = marker1.clone();
    assert_eq!(marker1.date.year, marker2.date.year);
    assert_eq!(marker1.color, marker2.color);
}

// =========================================================================
// DateMarker::symbol tests
// =========================================================================

#[test]
fn test_date_marker_symbol() {
    let date = Date {
        year: 2024,
        month: 1,
        day: 1,
    };
    let marker = DateMarker::new(date, Color::GREEN).symbol('*');
    assert_eq!(marker.symbol, Some('*'));
}

#[test]
fn test_date_marker_builder_chain() {
    let date = Date {
        year: 2024,
        month: 12,
        day: 25,
    };
    let marker = DateMarker::new(date, Color::RED).symbol('🎄');
    assert_eq!(marker.symbol, Some('🎄'));
    assert_eq!(marker.color, Color::RED);
}

#[test]
fn test_date_marker_no_symbol() {
    let date = Date {
        year: 2024,
        month: 1,
        day: 1,
    };
    let marker = DateMarker::new(date, Color::YELLOW);
    assert!(marker.symbol.is_none());
}

#[test]
fn test_date_marker_date_construction() {
    // Test with Date::new()
    let date = Date::new(2024, 6, 15);
    let marker = DateMarker::new(date, Color::CYAN);
    assert_eq!(marker.date.year, 2024);
    assert_eq!(marker.date.month, 6);
    assert_eq!(marker.date.day, 15);
}

#[test]
fn test_date_marker_different_colors() {
    let date = Date::new(2024, 1, 1);

    let red_marker = DateMarker::new(date, Color::RED);
    assert_eq!(red_marker.color, Color::RED);

    let blue_marker = DateMarker::new(date, Color::BLUE);
    assert_eq!(blue_marker.color, Color::BLUE);

    let green_marker = DateMarker::new(date, Color::GREEN);
    assert_eq!(green_marker.color, Color::GREEN);

    let custom_marker = DateMarker::new(date, Color::rgb(128, 64, 32));
    assert_eq!(custom_marker.color.r, 128);
    assert_eq!(custom_marker.color.g, 64);
    assert_eq!(custom_marker.color.b, 32);
}

#[test]
fn test_date_marker_symbol_variants() {
    let date = Date::new(2024, 1, 1);

    let star = DateMarker::new(date, Color::YELLOW).symbol('★');
    assert_eq!(star.symbol, Some('★'));

    let dot = DateMarker::new(date, Color::CYAN).symbol('●');
    assert_eq!(dot.symbol, Some('●'));

    let cross = DateMarker::new(date, Color::RED).symbol('✖');
    assert_eq!(cross.symbol, Some('✖'));

    let check = DateMarker::new(date, Color::GREEN).symbol('✔');
    assert_eq!(check.symbol, Some('✔'));
}
