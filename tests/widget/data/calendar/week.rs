//! Calendar ISO 8601 week number tests

use super::{row_text, week_numbers};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::calendar::{Calendar, FirstDayOfWeek};
use revue::widget::traits::{RenderContext, View};

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

#[test]
fn test_week_numbers_53_week_year_and_new_year() {
    // 2020 has 53 ISO weeks: Thursday 2020-12-31 is in week 53
    assert_eq!(
        week_numbers(2020, 12, FirstDayOfWeek::Monday),
        vec![49, 50, 51, 52, 53]
    );
    // Monday 2019-12-30 starts week 1 of 2020
    assert_eq!(
        week_numbers(2019, 12, FirstDayOfWeek::Monday),
        vec![48, 49, 50, 51, 52, 1]
    );
    // Friday 2021-01-01 is still in week 53 of 2020
    assert_eq!(
        week_numbers(2021, 1, FirstDayOfWeek::Monday),
        vec![53, 1, 2, 3, 4]
    );
    // Wednesday 2025-01-01 is in week 1; Monday the 6th starts week 2
    assert_eq!(
        week_numbers(2025, 1, FirstDayOfWeek::Monday),
        vec![1, 2, 3, 4, 5]
    );
}
