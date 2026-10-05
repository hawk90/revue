//! Calendar rendering tests
//!
//! The month view is drawn by a private renderer. These tests drive it
//! through `Calendar` and read the result out of the buffer:
//! - range highlighting (start, end, reversed, single-day)
//! - date markers
//! - day-name header for each first day of the week
//! - weekend colouring
//! - ISO 8601 week numbers

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::calendar::{
    first_day_of_month, Calendar, Date, DateMarker, FirstDayOfWeek,
};
use revue::widget::traits::{RenderContext, View};

/// Background the renderer gives a day inside the selected range
const RANGE_BG: Color = Color::rgb(60, 90, 120);
/// Default selection background (`Calendar::new`)
const SELECTED_BG: Color = Color::CYAN;

fn render(cal: &Calendar) -> Buffer {
    let mut buffer = Buffer::new(30, 12);
    let area = Rect::new(0, 0, 30, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);
    cal.render(&mut ctx);
    buffer
}

/// Position of the second (always a digit) character of `day` in a borderless
/// month view without week numbers.
fn day_pos(year: i32, month: u32, day: u32, first: FirstDayOfWeek) -> (u16, u16) {
    let first_weekday = first_day_of_month(year, month);
    let offset = match first {
        FirstDayOfWeek::Sunday => first_weekday,
        FirstDayOfWeek::Monday => (first_weekday + 6) % 7,
    };
    let index = offset + day - 1;
    ((index % 7) as u16 * 3 + 1, 3 + (index / 7) as u16)
}

fn day_bg(buffer: &Buffer, year: i32, month: u32, day: u32) -> Option<Color> {
    let (x, y) = day_pos(year, month, day, FirstDayOfWeek::Sunday);
    buffer.get(x, y).unwrap().bg
}

fn day_fg(buffer: &Buffer, year: i32, month: u32, day: u32) -> Option<Color> {
    let (x, y) = day_pos(year, month, day, FirstDayOfWeek::Sunday);
    buffer.get(x, y).unwrap().fg
}

fn row_text(buffer: &Buffer, y: u16, width: u16) -> String {
    (0..width)
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

// =========================================================================
// Range highlighting
// =========================================================================

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

    // The start is the selected date and keeps the selection colour
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

// =========================================================================
// Markers
// =========================================================================

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

// =========================================================================
// Day names
// =========================================================================

#[test]
fn test_day_names_sunday_first() {
    let buffer = render(&Calendar::new(2024, 1).first_day(FirstDayOfWeek::Sunday));
    assert_eq!(row_text(&buffer, 2, 20), "Su Mo Tu We Th Fr Sa");
}

#[test]
fn test_day_names_monday_first() {
    let buffer = render(&Calendar::new(2024, 1).first_day(FirstDayOfWeek::Monday));
    assert_eq!(row_text(&buffer, 2, 20), "Mo Tu We Th Fr Sa Su");
}

#[test]
fn test_monday_first_shifts_days() {
    // 2024-01-01 is a Monday: first column with Monday first, second with Sunday first
    let sunday = render(&Calendar::new(2024, 1).first_day(FirstDayOfWeek::Sunday));
    assert_eq!(row_text(&sunday, 3, 6), "    1 ");
    let monday = render(&Calendar::new(2024, 1).first_day(FirstDayOfWeek::Monday));
    assert_eq!(row_text(&monday, 3, 3), " 1 ");
}

// =========================================================================
// Weekends
// =========================================================================

fn header_fg(buffer: &Buffer, column: u16) -> Option<Color> {
    buffer.get(column * 3, 2).unwrap().fg
}

#[test]
fn test_weekend_columns_sunday_first() {
    let cal = Calendar::new(2024, 1)
        .first_day(FirstDayOfWeek::Sunday)
        .header_color(Color::CYAN)
        .weekend_color(Color::MAGENTA);
    let buffer = render(&cal);
    assert_eq!(header_fg(&buffer, 0), Some(Color::MAGENTA)); // Su
    assert_eq!(header_fg(&buffer, 6), Some(Color::MAGENTA)); // Sa
    for column in 1..=5 {
        assert_eq!(
            header_fg(&buffer, column),
            Some(Color::CYAN),
            "column {column}"
        );
    }
}

#[test]
fn test_weekend_columns_monday_first() {
    let cal = Calendar::new(2024, 1)
        .first_day(FirstDayOfWeek::Monday)
        .header_color(Color::CYAN)
        .weekend_color(Color::MAGENTA);
    let buffer = render(&cal);
    assert_eq!(header_fg(&buffer, 5), Some(Color::MAGENTA)); // Sa
    assert_eq!(header_fg(&buffer, 6), Some(Color::MAGENTA)); // Su
    for column in 0..=4 {
        assert_eq!(
            header_fg(&buffer, column),
            Some(Color::CYAN),
            "column {column}"
        );
    }
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

// =========================================================================
// ISO 8601 week numbers
// =========================================================================

/// Week number printed at the start of each displayed week, top to bottom.
fn week_numbers(year: i32, month: u32, first: FirstDayOfWeek) -> Vec<u32> {
    let cal = Calendar::new(year, month)
        .first_day(first)
        .week_numbers(true);
    let mut buffer = Buffer::new(35, 12);
    let area = Rect::new(0, 0, 35, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);
    cal.render(&mut ctx);

    (3..12)
        .map(|y| row_text(&buffer, y, 2))
        .take_while(|s| !s.trim().is_empty())
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[test]
fn test_week_numbers_january_2024() {
    // 2024-01-01 is a Monday, so it starts ISO week 1. With Sunday first the
    // second row starts on Sunday 7th, which still belongs to week 1.
    assert_eq!(
        week_numbers(2024, 1, FirstDayOfWeek::Sunday),
        vec![1, 1, 2, 3, 4]
    );
    assert_eq!(
        week_numbers(2024, 1, FirstDayOfWeek::Monday),
        vec![1, 2, 3, 4, 5]
    );
}

#[test]
fn test_week_numbers_february_2024() {
    // 2024-02-01 is a Thursday in week 5; Feb 29 (leap day) falls in week 9
    assert_eq!(
        week_numbers(2024, 2, FirstDayOfWeek::Monday),
        vec![5, 6, 7, 8, 9]
    );
}

#[test]
fn test_week_numbers_mid_year() {
    // 2024-06-01 is a Saturday in week 22
    assert_eq!(week_numbers(2024, 6, FirstDayOfWeek::Monday)[0], 22);
}

#[test]
fn test_week_numbers_december_rolls_into_next_year() {
    // Monday 2024-12-30 starts week 1 of 2025
    assert_eq!(
        week_numbers(2024, 12, FirstDayOfWeek::Monday),
        vec![48, 49, 50, 51, 52, 1]
    );
}

#[test]
fn test_week_numbers_december_2023_ends_in_week_52() {
    // Sunday 2023-12-31 is the last day of week 52 of 2023
    assert_eq!(
        week_numbers(2023, 12, FirstDayOfWeek::Sunday),
        vec![48, 48, 49, 50, 51, 52]
    );
}

#[test]
fn test_week_numbers_shift_the_grid() {
    let cal = Calendar::new(2024, 1).week_numbers(true);
    let mut buffer = Buffer::new(35, 12);
    let area = Rect::new(0, 0, 35, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);
    cal.render(&mut ctx);

    assert_eq!(buffer.get(0, 2).unwrap().symbol, 'W');
    assert_eq!(row_text(&buffer, 2, 24), "W   Su Mo Tu We Th Fr Sa");
}
