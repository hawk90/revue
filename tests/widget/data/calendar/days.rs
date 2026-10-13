//! Calendar day grid tests: range highlighting, markers and weekend days

use super::{day_bg, day_fg, day_pos, render, RANGE_BG, SELECTED_BG};
use revue::style::Color;
use revue::widget::data::calendar::{Calendar, Date, DateMarker, FirstDayOfWeek};

#[test]
fn test_no_range_highlights_nothing() {
    let buffer = render(&Calendar::new(2024, 1));
    for day in 1..=31 {
        assert_eq!(day_bg(&buffer, 2024, 1, day), None, "day {day}");
    }
}

#[test]
fn test_range_highlights_days_between_ends() {
    let cal = Calendar::new(2024, 1).range(Date::new(2024, 1, 10), Date::new(2024, 1, 20));
    let buffer = render(&cal);

    // The start is the selected date and keeps the selection color
    assert_eq!(day_bg(&buffer, 2024, 1, 10), Some(SELECTED_BG));
    for day in 11..=20 {
        assert_eq!(day_bg(&buffer, 2024, 1, day), Some(RANGE_BG), "day {day}");
    }
}

#[test]
fn test_range_excludes_days_before_start() {
    let cal = Calendar::new(2024, 1).range(Date::new(2024, 1, 10), Date::new(2024, 1, 20));
    let buffer = render(&cal);
    for day in 1..10 {
        assert_eq!(day_bg(&buffer, 2024, 1, day), None, "day {day}");
    }
}

#[test]
fn test_range_excludes_days_after_end() {
    let cal = Calendar::new(2024, 1).range(Date::new(2024, 1, 10), Date::new(2024, 1, 20));
    let buffer = render(&cal);
    for day in 21..=31 {
        assert_eq!(day_bg(&buffer, 2024, 1, day), None, "day {day}");
    }
}

#[test]
fn test_reversed_range_is_highlighted() {
    let cal = Calendar::new(2024, 1).range(Date::new(2024, 1, 20), Date::new(2024, 1, 10));
    let buffer = render(&cal);

    assert_eq!(day_bg(&buffer, 2024, 1, 20), Some(SELECTED_BG));
    for day in 10..20 {
        assert_eq!(day_bg(&buffer, 2024, 1, day), Some(RANGE_BG), "day {day}");
    }
    assert_eq!(day_bg(&buffer, 2024, 1, 9), None);
    assert_eq!(day_bg(&buffer, 2024, 1, 21), None);
}

#[test]
fn test_single_day_range() {
    let cal = Calendar::new(2024, 1).range(Date::new(2024, 1, 15), Date::new(2024, 1, 15));
    let buffer = render(&cal);

    assert_eq!(day_bg(&buffer, 2024, 1, 15), Some(SELECTED_BG));
    assert_eq!(day_bg(&buffer, 2024, 1, 14), None);
    assert_eq!(day_bg(&buffer, 2024, 1, 16), None);
}

#[test]
fn test_marker_colors_its_day() {
    // 2024-01-15 is a Monday, so without the marker it would be a plain day
    let cal = Calendar::new(2024, 1).marker(DateMarker::new(Date::new(2024, 1, 15), Color::RED));
    let buffer = render(&cal);
    assert_eq!(day_fg(&buffer, 2024, 1, 15), Some(Color::RED));
}

#[test]
fn test_unmarked_day_uses_day_color() {
    let cal = Calendar::new(2024, 1)
        .day_color(Color::GREEN)
        .marker(DateMarker::new(Date::new(2024, 1, 15), Color::RED));
    let buffer = render(&cal);
    assert_eq!(day_fg(&buffer, 2024, 1, 16), Some(Color::GREEN));
}

#[test]
fn test_multiple_markers_keep_their_own_colors() {
    let cal = Calendar::new(2024, 1).markers(vec![
        DateMarker::new(Date::new(2024, 1, 15), Color::RED),
        DateMarker::new(Date::new(2024, 1, 17), Color::BLUE),
    ]);
    let buffer = render(&cal);
    assert_eq!(day_fg(&buffer, 2024, 1, 15), Some(Color::RED));
    assert_eq!(day_fg(&buffer, 2024, 1, 17), Some(Color::BLUE));
}

#[test]
fn test_marker_symbol_follows_the_day() {
    let cal = Calendar::new(2024, 1)
        .marker(DateMarker::new(Date::new(2024, 1, 15), Color::RED).symbol('*'));
    let buffer = render(&cal);
    let (x, y) = day_pos(2024, 1, 15, FirstDayOfWeek::Sunday);
    let cell = buffer.get(x + 1, y).unwrap();
    assert_eq!(cell.symbol, '*');
    assert_eq!(cell.fg, Some(Color::RED));
}

#[test]
fn test_weekend_days_use_weekend_color() {
    let cal = Calendar::new(2024, 1)
        .day_color(Color::WHITE)
        .weekend_color(Color::MAGENTA);
    let buffer = render(&cal);
    assert_eq!(day_fg(&buffer, 2024, 1, 6), Some(Color::MAGENTA)); // Saturday
    assert_eq!(day_fg(&buffer, 2024, 1, 7), Some(Color::MAGENTA)); // Sunday
    assert_eq!(day_fg(&buffer, 2024, 1, 8), Some(Color::WHITE)); // Monday
}
