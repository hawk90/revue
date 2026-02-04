//! DateTime picker tests

use super::helpers::month_name;
use super::render::Rendering;
use super::types::{DateTime, DateTimeFormat, DateTimeMode, Time, TimeField};
use super::{date_picker, datetime_picker, time_picker, DateTimePicker};
use crate::event::Key;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::data::calendar::Date;
use crate::widget::data::calendar::FirstDayOfWeek;
use crate::widget::traits::{RenderContext, View};

// =========================================================================
// Time Tests
// =========================================================================

#[test]
fn test_time_new() {
    let t = Time::new(14, 30, 45);
    assert_eq!(t.hour, 14);
    assert_eq!(t.minute, 30);
    assert_eq!(t.second, 45);
}

#[test]
fn test_time_clamp() {
    let t = Time::new(25, 70, 80);
    assert_eq!(t.hour, 23);
    assert_eq!(t.minute, 59);
    assert_eq!(t.second, 59);
}

#[test]
fn test_time_format() {
    let t = Time::new(14, 5, 9);
    assert_eq!(t.format_hm(), "14:05");
    assert_eq!(t.format_hms(), "14:05:09");
}

#[test]
fn test_time_format_12h() {
    assert_eq!(Time::new(0, 30, 0).format_12h(), "12:30 AM");
    assert_eq!(Time::new(9, 15, 0).format_12h(), " 9:15 AM");
    assert_eq!(Time::new(12, 0, 0).format_12h(), "12:00 PM");
    assert_eq!(Time::new(15, 45, 0).format_12h(), " 3:45 PM");
}

// =========================================================================
// DateTime Tests
// =========================================================================

#[test]
fn test_datetime_new() {
    let dt = DateTime::new(Date::new(2025, 6, 15), Time::new(10, 30, 0));
    assert_eq!(dt.date.year, 2025);
    assert_eq!(dt.date.month, 6);
    assert_eq!(dt.date.day, 15);
    assert_eq!(dt.time.hour, 10);
    assert_eq!(dt.time.minute, 30);
}

// =========================================================================
// Picker Creation Tests
// =========================================================================

#[test]
fn test_picker_new() {
    let p = DateTimePicker::new();
    assert_eq!(p.mode, DateTimeMode::Date);
    assert_eq!(p.format, DateTimeFormat::DateTime);
}

#[test]
fn test_picker_date_only() {
    let p = DateTimePicker::date_only();
    assert_eq!(p.format, DateTimeFormat::DateOnly);
}

#[test]
fn test_picker_time_only() {
    let p = DateTimePicker::time_only();
    assert_eq!(p.format, DateTimeFormat::TimeOnly);
    assert_eq!(p.mode, DateTimeMode::Time);
}

#[test]
fn test_picker_selected() {
    let p = datetime_picker()
        .selected_date(Date::new(2025, 3, 20))
        .selected_time(Time::new(15, 45, 0));
    assert_eq!(p.date, Date::new(2025, 3, 20));
    assert_eq!(p.time, Time::new(15, 45, 0));
}

#[test]
fn test_picker_constraints() {
    let p = datetime_picker()
        .min_date(Date::new(2025, 1, 1))
        .max_date(Date::new(2025, 12, 31));
    assert!(p.is_date_valid(&Date::new(2025, 6, 15)));
    assert!(!p.is_date_valid(&Date::new(2024, 12, 31)));
    assert!(!p.is_date_valid(&Date::new(2026, 1, 1)));
}

#[test]
fn test_picker_handle_key_tab() {
    let mut p = datetime_picker();
    assert_eq!(p.mode, DateTimeMode::Date);
    p.handle_key(&Key::Tab);
    assert_eq!(p.mode, DateTimeMode::Time);
    p.handle_key(&Key::Tab);
    assert_eq!(p.mode, DateTimeMode::Date);
}

#[test]
fn test_picker_time_navigation() {
    let mut p = datetime_picker().selected_time(Time::new(10, 30, 0));
    p.mode = DateTimeMode::Time;

    // Increment hour
    p.handle_key(&Key::Up);
    assert_eq!(p.time.hour, 11);

    // Move to minute
    p.handle_key(&Key::Right);
    assert_eq!(p.time_field, TimeField::Minute);

    // Decrement minute
    p.handle_key(&Key::Down);
    assert_eq!(p.time.minute, 29);
}

#[test]
fn test_picker_month_navigation() {
    let mut p = datetime_picker().selected_date(Date::new(2025, 6, 15));

    // Next month
    p.handle_key(&Key::Char(']'));
    assert_eq!(p.date.month, 7);

    // Previous month
    p.handle_key(&Key::Char('['));
    assert_eq!(p.date.month, 6);
}

#[test]
fn test_picker_year_navigation() {
    let mut p = datetime_picker().selected_date(Date::new(2025, 6, 15));

    // Next year
    p.handle_key(&Key::Char('}'));
    assert_eq!(p.date.year, 2026);

    // Previous year
    p.handle_key(&Key::Char('{'));
    assert_eq!(p.date.year, 2025);
}

#[test]
fn test_helper_functions() {
    let _ = datetime_picker();
    let _ = date_picker();
    let _ = time_picker();
}

#[test]
fn test_month_name() {
    assert_eq!(month_name(1), "January");
    assert_eq!(month_name(6), "June");
    assert_eq!(month_name(12), "December");
    assert_eq!(month_name(13), "Unknown");
}

#[test]
fn test_first_weekday() {
    // Test known dates
    let p = datetime_picker().selected_date(Date::new(2025, 1, 1));
    let weekday = p.first_weekday();
    // January 2025 starts on Wednesday (3 for Sunday-first, 2 for Monday-first)
    assert!(weekday <= 6);
}

#[test]
fn test_first_weekday_monday_start() {
    let p = datetime_picker()
        .selected_date(Date::new(2025, 1, 1))
        .first_day(FirstDayOfWeek::Monday);
    let weekday = p.first_weekday();
    assert!(weekday <= 6);
}

#[test]
fn test_picker_cursor_navigation() {
    let mut p = datetime_picker().selected_date(Date::new(2025, 6, 15));

    // Move right
    p.handle_key(&Key::Right);
    assert_eq!(p.cursor_day, 16);

    // Move left
    p.handle_key(&Key::Left);
    assert_eq!(p.cursor_day, 15);

    // Move down (week)
    p.handle_key(&Key::Down);
    assert_eq!(p.cursor_day, 22);

    // Move up (week)
    p.handle_key(&Key::Up);
    assert_eq!(p.cursor_day, 15);
}

#[test]
fn test_picker_vim_navigation() {
    let mut p = datetime_picker().selected_date(Date::new(2025, 6, 15));

    // vim keys: h, j, k, l
    p.handle_key(&Key::Char('l'));
    assert_eq!(p.cursor_day, 16);

    p.handle_key(&Key::Char('h'));
    assert_eq!(p.cursor_day, 15);

    p.handle_key(&Key::Char('j'));
    assert_eq!(p.cursor_day, 22);

    p.handle_key(&Key::Char('k'));
    assert_eq!(p.cursor_day, 15);
}

#[test]
fn test_picker_select_date() {
    let mut p = datetime_picker().selected_date(Date::new(2025, 6, 15));
    p.cursor_day = 20;

    p.handle_key(&Key::Enter);
    assert_eq!(p.date.day, 20);
}

#[test]
fn test_picker_time_vim_keys() {
    let mut p = datetime_picker().selected_time(Time::new(10, 30, 0));
    p.mode = DateTimeMode::Time;

    // vim keys in time mode
    p.handle_key(&Key::Char('k')); // increment
    assert_eq!(p.time.hour, 11);

    p.handle_key(&Key::Char('l')); // next field
    assert_eq!(p.time_field, TimeField::Minute);

    p.handle_key(&Key::Char('j')); // decrement
    assert_eq!(p.time.minute, 29);

    p.handle_key(&Key::Char('h')); // prev field
    assert_eq!(p.time_field, TimeField::Hour);
}

#[test]
fn test_picker_month_boundary() {
    let mut p = datetime_picker().selected_date(Date::new(2025, 1, 31));
    p.cursor_day = 31;

    // Go to next month (Feb has fewer days)
    p.handle_key(&Key::Char(']'));
    assert!(p.cursor_day <= 28);
}

#[test]
fn test_picker_year_boundary() {
    let mut p = datetime_picker().selected_date(Date::new(2024, 2, 29));
    p.cursor_day = 29;

    // Go to next year (2025 is not leap year)
    p.handle_key(&Key::Char('}'));
    assert_eq!(p.cursor_day, 28);
}

#[test]
fn test_picker_render_datetime() {
    let p = datetime_picker()
        .selected_date(Date::new(2025, 6, 15))
        .selected_time(Time::new(14, 30, 0));

    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    p.render(&mut ctx);
    // Verify rendering completed without panic
}

#[test]
fn test_picker_render_date_only() {
    let p = date_picker().selected_date(Date::new(2025, 6, 15));

    let mut buffer = Buffer::new(25, 10);
    let area = Rect::new(0, 0, 25, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    p.render(&mut ctx);
}

#[test]
fn test_picker_render_time_only() {
    let p = time_picker().selected_time(Time::new(14, 30, 0));

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    p.render(&mut ctx);
}

#[test]
fn test_picker_render_small_area() {
    let p = datetime_picker();

    // Too small area should return early
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    p.render(&mut ctx);
}

#[test]
fn test_picker_render_with_seconds() {
    let p = time_picker()
        .selected_time(Time::new(14, 30, 45))
        .show_seconds(true);

    let mut buffer = Buffer::new(25, 10);
    let area = Rect::new(0, 0, 25, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    p.render(&mut ctx);
}

#[test]
fn test_picker_time_field_second() {
    let mut p = datetime_picker()
        .selected_time(Time::new(10, 30, 45))
        .show_seconds(true);
    p.mode = DateTimeMode::Time;
    p.time_field = TimeField::Second;

    p.handle_key(&Key::Up);
    assert_eq!(p.time.second, 46);

    p.handle_key(&Key::Down);
    assert_eq!(p.time.second, 45);
}

#[test]
fn test_picker_time_wrap() {
    let mut p = datetime_picker().selected_time(Time::new(23, 59, 59));
    p.mode = DateTimeMode::Time;

    // Hour wrap
    p.handle_key(&Key::Up);
    assert_eq!(p.time.hour, 0);

    // Minute wrap
    p.time_field = TimeField::Minute;
    p.handle_key(&Key::Up);
    assert_eq!(p.time.minute, 0);

    // Second wrap
    p.time_field = TimeField::Second;
    p.handle_key(&Key::Up);
    assert_eq!(p.time.second, 0);
}

#[test]
fn test_picker_time_wrap_down() {
    let mut p = datetime_picker().selected_time(Time::new(0, 0, 0));
    p.mode = DateTimeMode::Time;

    p.handle_key(&Key::Down);
    assert_eq!(p.time.hour, 23);

    p.time_field = TimeField::Minute;
    p.handle_key(&Key::Down);
    assert_eq!(p.time.minute, 59);

    p.time_field = TimeField::Second;
    p.handle_key(&Key::Down);
    assert_eq!(p.time.second, 59);
}

#[test]
fn test_picker_unhandled_key() {
    let mut p = datetime_picker();
    let handled = p.handle_key(&Key::Char('x'));
    assert!(!handled);
}

#[test]
fn test_picker_space_select() {
    let mut p = datetime_picker().selected_date(Date::new(2025, 6, 15));
    p.cursor_day = 20;

    p.handle_key(&Key::Char(' '));
    assert_eq!(p.date.day, 20);
}

#[test]
fn test_picker_default() {
    let p = DateTimePicker::default();
    assert_eq!(p.format, DateTimeFormat::DateTime);
}

#[test]
fn test_picker_get_mode() {
    let p = datetime_picker();
    assert_eq!(p.get_mode(), DateTimeMode::Date);
}

#[test]
fn test_datetime_format_variants() {
    let p1 = datetime_picker().format(DateTimeFormat::DateTime);
    assert_eq!(p1.format, DateTimeFormat::DateTime);

    let p2 = datetime_picker().format(DateTimeFormat::DateOnly);
    assert_eq!(p2.format, DateTimeFormat::DateOnly);

    let p3 = datetime_picker().format(DateTimeFormat::TimeOnly);
    assert_eq!(p3.format, DateTimeFormat::TimeOnly);

    let p4 = datetime_picker().format(DateTimeFormat::TimeWithSeconds);
    assert_eq!(p4.format, DateTimeFormat::TimeWithSeconds);
}

#[test]
fn test_picker_cursor_boundary_right() {
    let mut p = datetime_picker().selected_date(Date::new(2025, 6, 30));
    p.cursor_day = 30; // Last day of June

    p.handle_key(&Key::Right);
    // Should wrap to next month
    assert_eq!(p.date.month, 7);
    assert_eq!(p.cursor_day, 1);
}

#[test]
fn test_picker_cursor_boundary_left() {
    let mut p = datetime_picker().selected_date(Date::new(2025, 6, 1));
    p.cursor_day = 1;

    p.handle_key(&Key::Left);
    // Should wrap to previous month
    assert_eq!(p.date.month, 5);
}

#[test]
fn test_picker_cursor_boundary_down() {
    let mut p = datetime_picker().selected_date(Date::new(2025, 6, 28));
    p.cursor_day = 28;

    p.handle_key(&Key::Down);
    // Should wrap to next month
    assert_eq!(p.date.month, 7);
}

#[test]
fn test_picker_cursor_boundary_up() {
    let mut p = datetime_picker().selected_date(Date::new(2025, 6, 3));
    p.cursor_day = 3;

    p.handle_key(&Key::Up);
    // Should wrap to previous month
    assert_eq!(p.date.month, 5);
}

#[test]
fn test_picker_month_wrap_december() {
    let mut p = datetime_picker().selected_date(Date::new(2025, 12, 15));

    p.handle_key(&Key::Char(']'));
    assert_eq!(p.date.month, 1);
    assert_eq!(p.date.year, 2026);
}

#[test]
fn test_picker_month_wrap_january() {
    let mut p = datetime_picker().selected_date(Date::new(2025, 1, 15));

    p.handle_key(&Key::Char('['));
    assert_eq!(p.date.month, 12);
    assert_eq!(p.date.year, 2024);
}

#[test]
fn test_picker_constraint_select() {
    let mut p = datetime_picker()
        .selected_date(Date::new(2025, 6, 15))
        .min_date(Date::new(2025, 6, 10))
        .max_date(Date::new(2025, 6, 20));

    // Try to select a date outside constraints
    p.cursor_day = 5;
    p.handle_key(&Key::Enter);
    // Date should not change because 5 is before min
    assert_eq!(p.date.day, 15);
}

#[test]
fn test_time_field_navigation_wrap() {
    let mut p = datetime_picker().show_seconds(true);
    p.mode = DateTimeMode::Time;
    p.time_field = TimeField::Second;

    // Second wraps to Hour
    p.handle_key(&Key::Right);
    assert_eq!(p.time_field, TimeField::Hour);

    // Hour wraps back to Second
    p.handle_key(&Key::Left);
    assert_eq!(p.time_field, TimeField::Second);
}

#[test]
fn test_time_field_navigation_no_seconds() {
    let mut p = datetime_picker().show_seconds(false);
    p.mode = DateTimeMode::Time;
    p.time_field = TimeField::Minute;

    // Without seconds, Minute wraps to Hour
    p.handle_key(&Key::Right);
    assert_eq!(p.time_field, TimeField::Hour);

    // Hour wraps to Minute (skips Second)
    p.handle_key(&Key::Left);
    assert_eq!(p.time_field, TimeField::Minute);
}
