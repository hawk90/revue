//! Public API tests for calendar widget
//!
//! These tests only use public APIs of the calendar module.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::calendar::{calendar, days_in_month, is_leap_year, Calendar};
use revue::widget::data::calendar::{CalendarMode, Date, DateMarker, FirstDayOfWeek};
use revue::widget::traits::{RenderContext, View};

#[test]
fn test_date_new() {
    let date = Date::new(2025, 6, 15);
    assert_eq!(date.year, 2025);
    assert_eq!(date.month, 6);
    assert_eq!(date.day, 15);
}

#[test]
fn test_date_valid() {
    assert!(Date::new(2025, 1, 1).is_valid());
    assert!(Date::new(2025, 2, 28).is_valid());
    assert!(Date::new(2024, 2, 29).is_valid()); // Leap year
    assert!(!Date::new(2025, 2, 29).is_valid()); // Not leap year
    assert!(!Date::new(2025, 13, 1).is_valid());
    assert!(!Date::new(2025, 1, 32).is_valid());
}

#[test]
fn test_days_in_month() {
    assert_eq!(days_in_month(2025, 1), 31);
    assert_eq!(days_in_month(2025, 2), 28);
    assert_eq!(days_in_month(2024, 2), 29);
    assert_eq!(days_in_month(2025, 4), 30);
}

#[test]
fn test_leap_year() {
    assert!(is_leap_year(2024));
    assert!(!is_leap_year(2025));
    assert!(is_leap_year(2000));
    assert!(!is_leap_year(1900));
}

#[test]
fn test_calendar_selection() {
    let mut cal = Calendar::new(2025, 1);

    cal.select(Date::new(2025, 1, 15));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 15)));

    cal.clear_selection();
    assert_eq!(cal.get_selected(), None);
}

#[test]
fn test_date_marker() {
    use revue::style::Color;

    let marker = DateMarker::new(Date::new(2025, 1, 1), Color::RED).symbol('★');

    assert_eq!(marker.date, Date::new(2025, 1, 1));
    assert_eq!(marker.color, Color::RED);
    assert_eq!(marker.symbol, Some('★'));
}

#[test]
fn test_calendar_render() {
    let mut buffer = Buffer::new(30, 12);
    let area = Rect::new(0, 0, 30, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let cal = Calendar::new(2025, 1)
        .selected(Date::new(2025, 1, 15))
        .today(Date::new(2025, 1, 10));

    cal.render(&mut ctx);
    // Smoke test - renders without panic
}

#[test]
fn test_calendar_with_border() {
    use revue::style::Color;

    let mut buffer = Buffer::new(30, 12);
    let area = Rect::new(0, 0, 30, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let cal = Calendar::new(2025, 1).border(Color::WHITE);
    cal.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_calendar_first_day() {
    let _cal_sun = Calendar::new(2025, 1).first_day(FirstDayOfWeek::Sunday);
    let _cal_mon = Calendar::new(2025, 1).first_day(FirstDayOfWeek::Monday);
    // Public API test - first_day() is public
}

#[test]
fn test_calendar_range() {
    let _cal = Calendar::new(2025, 1).range(Date::new(2025, 1, 10), Date::new(2025, 1, 20));
    // Public API test - range() is public
}

#[test]
fn test_calendar_week_numbers() {
    let _cal = Calendar::new(2025, 1).week_numbers(true);
    // Public API test - week_numbers() is public
}

#[test]
fn test_calendar_marker() {
    use revue::style::Color;

    let marker = DateMarker::new(Date::new(2025, 1, 1), Color::RED);
    let _cal = Calendar::new(2025, 1).marker(marker);
    // Public API test - marker() is public
}

#[test]
fn test_calendar_markers() {
    use revue::style::Color;

    let markers = vec![
        DateMarker::new(Date::new(2025, 1, 1), Color::RED),
        DateMarker::new(Date::new(2025, 1, 15), Color::YELLOW),
    ];
    let _cal = Calendar::new(2025, 1).markers(markers);
    // Public API test - markers() is public
}

#[test]
fn test_calendar_today() {
    let _cal = Calendar::new(2025, 1).today(Date::new(2025, 1, 10));
    // Public API test - today() is public
}

#[test]
fn test_calendar_header_color() {
    use revue::style::Color;

    let _cal = Calendar::new(2025, 1).header_color(Color::BLUE);
    // Public API test - header_color() is public
}

#[test]
fn test_calendar_header_bg() {
    use revue::style::Color;

    let _cal = Calendar::new(2025, 1).header_bg(Color::BLACK);
    // Public API test - header_bg() is public
}

#[test]
fn test_calendar_day_color() {
    use revue::style::Color;

    let _cal = Calendar::new(2025, 1).day_color(Color::WHITE);
    // Public API test - day_color() is public
}

#[test]
fn test_calendar_weekend_color() {
    use revue::style::Color;

    let _cal = Calendar::new(2025, 1).weekend_color(Color::GRAY);
    // Public API test - weekend_color() is public
}

#[test]
fn test_calendar_selected_color() {
    use revue::style::Color;

    let _cal = Calendar::new(2025, 1).selected_color(Color::BLACK, Color::CYAN);
    // Public API test - selected_color() is public
}

#[test]
fn test_calendar_today_color() {
    use revue::style::Color;

    let _cal = Calendar::new(2025, 1).today_color(Color::YELLOW);
    // Public API test - today_color() is public
}

#[test]
fn test_calendar_focused() {
    let _cal = Calendar::new(2025, 1).focused(true);
    // Public API test - focused() is public
}

#[test]
fn test_calendar_mode() {
    let _cal = Calendar::new(2025, 1).mode(CalendarMode::Range);
    // Public API test - mode() is public
}

#[test]
fn test_calendar_helper() {
    let cal = calendar(2025, 6);
    // calendar() helper is public, but accessing year/month fields is private
    // This test validates the helper creates a calendar without asserting on private fields
    let _ = cal; // Use the variable to avoid unused warnings
}
