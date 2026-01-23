//! Tests for calendar module

use super::types::{DateMarker, FirstDayOfWeek};
use super::*;
use crate::layout::Rect;
use crate::render::Buffer;

#[test]
fn test_calendar_new() {
    let cal = Calendar::new(2025, 1);
    assert_eq!(cal.year, 2025);
    assert_eq!(cal.month, 1);
}

#[test]
fn test_calendar_month_clamp() {
    let cal = Calendar::new(2025, 13);
    assert_eq!(cal.month, 12);

    let cal = Calendar::new(2025, 0);
    assert_eq!(cal.month, 1);
}

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
fn test_calendar_navigation() {
    let mut cal = Calendar::new(2025, 1);

    cal.next_month();
    assert_eq!(cal.month, 2);

    cal.prev_month();
    assert_eq!(cal.month, 1);

    cal.prev_month();
    assert_eq!(cal.month, 12);
    assert_eq!(cal.year, 2024);

    cal.next_month();
    assert_eq!(cal.month, 1);
    assert_eq!(cal.year, 2025);
}

#[test]
fn test_calendar_year_navigation() {
    let mut cal = Calendar::new(2025, 6);

    cal.next_year();
    assert_eq!(cal.year, 2026);

    cal.prev_year();
    assert_eq!(cal.year, 2025);
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
fn test_calendar_select_next_day() {
    let mut cal = Calendar::new(2025, 1).selected(Date::new(2025, 1, 31));

    cal.select_next_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 2, 1)));
    assert_eq!(cal.month, 2);
}

#[test]
fn test_calendar_select_prev_day() {
    let mut cal = Calendar::new(2025, 2).selected(Date::new(2025, 2, 1));

    cal.select_prev_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 31)));
    assert_eq!(cal.month, 1);
}

#[test]
fn test_date_marker() {
    let marker = DateMarker::new(Date::new(2025, 1, 1), Color::RED).symbol('★');

    assert_eq!(marker.date, Date::new(2025, 1, 1));
    assert_eq!(marker.color, Color::RED);
    assert_eq!(marker.symbol, Some('★'));
}

#[test]
fn test_calendar_range() {
    let cal = Calendar::new(2025, 1).range(Date::new(2025, 1, 10), Date::new(2025, 1, 20));

    let render_state = render::CalendarRender {
        year: 2025,
        month: 1,
        selected: Some(Date::new(2025, 1, 10)),
        range_end: Some(Date::new(2025, 1, 20)),
        first_day: FirstDayOfWeek::Sunday,
        show_week_numbers: false,
        markers: &[],
        today: None,
        header_fg: Color::CYAN,
        header_bg: None,
        day_fg: Color::WHITE,
        weekend_fg: Color::rgb(150, 150, 150),
        selected_fg: Color::BLACK,
        selected_bg: Color::CYAN,
        today_fg: Color::YELLOW,
        outside_fg: Color::rgb(80, 80, 80),
        border_color: None,
        focused: false,
    };

    assert!(render_state.is_in_range(&Date::new(2025, 1, 15)));
    assert!(!render_state.is_in_range(&Date::new(2025, 1, 5)));
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
    let mut buffer = Buffer::new(30, 12);
    let area = Rect::new(0, 0, 30, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let cal = Calendar::new(2025, 1).border(Color::WHITE);
    cal.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_calendar_first_day() {
    let cal_sun = Calendar::new(2025, 1).first_day(FirstDayOfWeek::Sunday);
    let cal_mon = Calendar::new(2025, 1).first_day(FirstDayOfWeek::Monday);

    let render_state_sun = render::CalendarRender {
        year: 2025,
        month: 1,
        selected: None,
        range_end: None,
        first_day: FirstDayOfWeek::Sunday,
        show_week_numbers: false,
        markers: &[],
        today: None,
        header_fg: Color::CYAN,
        header_bg: None,
        day_fg: Color::WHITE,
        weekend_fg: Color::rgb(150, 150, 150),
        selected_fg: Color::BLACK,
        selected_bg: Color::CYAN,
        today_fg: Color::YELLOW,
        outside_fg: Color::rgb(80, 80, 80),
        border_color: None,
        focused: false,
    };

    let render_state_mon = render::CalendarRender {
        first_day: FirstDayOfWeek::Monday,
        ..render_state_sun
    };

    assert_eq!(render_state_sun.day_names()[0], "Su");
    assert_eq!(render_state_mon.day_names()[0], "Mo");
}

#[test]
fn test_first_day_of_month() {
    // January 1, 2025 is Wednesday
    assert_eq!(first_day_of_month(2025, 1), 3);
}

#[test]
fn test_calendar_helper() {
    let cal = calendar(2025, 6);
    assert_eq!(cal.year, 2025);
    assert_eq!(cal.month, 6);
}

#[test]
fn test_iso_week_number() {
    let cal = Calendar::new(2025, 1);

    let render_state = render::CalendarRender {
        year: 2025,
        month: 1,
        selected: None,
        range_end: None,
        mode: CalendarMode::Month,
        first_day: FirstDayOfWeek::Sunday,
        show_week_numbers: false,
        markers: &[],
        today: None,
        header_fg: Color::CYAN,
        header_bg: None,
        day_fg: Color::WHITE,
        weekend_fg: Color::rgb(150, 150, 150),
        selected_fg: Color::BLACK,
        selected_bg: Color::CYAN,
        today_fg: Color::YELLOW,
        outside_fg: Color::rgb(80, 80, 80),
        border_color: None,
        focused: false,
    };

    // 2025-01-01 is Wednesday, ISO week 1
    assert_eq!(render_state.get_week_number(2025, 1, 1), 1);

    // 2025-01-06 is Monday, still week 1
    assert_eq!(render_state.get_week_number(2025, 1, 6), 2);

    // 2024-12-30 is Monday, week 1 of 2025
    assert_eq!(render_state.get_week_number(2024, 12, 30), 1);

    // 2024-12-28 is Saturday, week 52 of 2024
    assert_eq!(render_state.get_week_number(2024, 12, 28), 52);
}

#[test]
fn test_iso_week_number_edge_cases() {
    let cal = Calendar::new(2020, 1);

    let render_state = render::CalendarRender {
        year: 2020,
        month: 1,
        selected: None,
        range_end: None,
        mode: CalendarMode::Month,
        first_day: FirstDayOfWeek::Sunday,
        show_week_numbers: false,
        markers: &[],
        today: None,
        header_fg: Color::CYAN,
        header_bg: None,
        day_fg: Color::WHITE,
        weekend_fg: Color::rgb(150, 150, 150),
        selected_fg: Color::BLACK,
        selected_bg: Color::CYAN,
        today_fg: Color::YELLOW,
        outside_fg: Color::rgb(80, 80, 80),
        border_color: None,
        focused: false,
    };

    // 2020-01-01 is Wednesday, ISO week 1
    assert_eq!(render_state.get_week_number(2020, 1, 1), 1);

    // 2019-12-30 is Monday, week 1 of 2020
    assert_eq!(render_state.get_week_number(2019, 12, 30), 1);

    // 2020-12-31 is Thursday, week 53 of 2020
    assert_eq!(render_state.get_week_number(2020, 12, 31), 53);
}
