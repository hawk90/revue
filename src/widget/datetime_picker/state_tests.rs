//! DateTime picker state tests
//!
//! These check the state that key handling leaves behind: the calendar
//! cursor day, the displayed month/year, the focused time field and the
//! configured format. None of that has a public getter, and the public
//! tests in tests/widget/datetime_picker.rs only assert that the key was
//! handled, so these read the private fields.

use super::types::{DateTimeFormat, DateTimeMode, Time, TimeField};
use super::{datetime_picker, DateTimePicker};
use crate::event::Key;
use crate::widget::data::calendar::Date;

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
fn test_picker_constraints() {
    let p = datetime_picker()
        .min_date(Date::new(2025, 1, 1))
        .max_date(Date::new(2025, 12, 31));
    assert!(p.is_date_valid(&Date::new(2025, 6, 15)));
    assert!(!p.is_date_valid(&Date::new(2024, 12, 31)));
    assert!(!p.is_date_valid(&Date::new(2026, 1, 1)));
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
