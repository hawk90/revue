//! Calendar widget tests, laid out like src/widget/data/calendar

mod builders;
mod date;
mod days;
mod navigation;
mod render;
mod types;
mod utils;
mod week;

use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::style::Color;
use revue::widget::data::calendar::{first_day_of_month, Calendar, FirstDayOfWeek};
use revue::widget::traits::{RenderContext, View};

/// Each row of `cal` drawn in a `width` x `height` area.
pub(super) fn screen_in(cal: &Calendar, width: u16, height: u16) -> Vec<String> {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    cal.render(&mut ctx);
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect()
        })
        .collect()
}

/// Each row of `cal` drawn in a 30 x 10 area.
pub(super) fn screen(cal: &Calendar) -> Vec<String> {
    screen_in(cal, 30, 10)
}

/// The month and year the calendar shows (without the focus arrows).
pub(super) fn header(cal: &Calendar) -> String {
    screen(cal)[0].trim_matches([' ', '◀', '▶']).to_string()
}

/// The cell at (`x`, `y`) of `cal` drawn in a 30 x 10 area.
pub(super) fn cell(cal: &Calendar, x: u16, y: u16) -> Cell {
    let mut buffer = Buffer::new(30, 10);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 30, 10));
    cal.render(&mut ctx);
    *buffer.get(x, y).unwrap()
}

pub(super) fn render(cal: &Calendar) -> Buffer {
    let mut buffer = Buffer::new(30, 12);
    let area = Rect::new(0, 0, 30, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);
    cal.render(&mut ctx);
    buffer
}

/// Position of the second (always a digit) character of `day` in a borderless
/// month view without week numbers.
pub(super) fn day_pos(year: i32, month: u32, day: u32, first: FirstDayOfWeek) -> (u16, u16) {
    let first_weekday = first_day_of_month(year, month);
    let offset = match first {
        FirstDayOfWeek::Sunday => first_weekday,
        FirstDayOfWeek::Monday => (first_weekday + 6) % 7,
    };
    let index = offset + day - 1;
    ((index % 7) as u16 * 3 + 1, 3 + (index / 7) as u16)
}

pub(super) fn day_bg(buffer: &Buffer, year: i32, month: u32, day: u32) -> Option<Color> {
    let (x, y) = day_pos(year, month, day, FirstDayOfWeek::Sunday);
    buffer.get(x, y).unwrap().bg
}

pub(super) fn day_fg(buffer: &Buffer, year: i32, month: u32, day: u32) -> Option<Color> {
    let (x, y) = day_pos(year, month, day, FirstDayOfWeek::Sunday);
    buffer.get(x, y).unwrap().fg
}

pub(super) fn row_text(buffer: &Buffer, y: u16, width: u16) -> String {
    (0..width)
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

pub(super) fn header_fg(buffer: &Buffer, column: u16) -> Option<Color> {
    buffer.get(column * 3, 2).unwrap().fg
}

/// Week number printed at the start of each displayed week, top to bottom.
pub(super) fn week_numbers(year: i32, month: u32, first: FirstDayOfWeek) -> Vec<u32> {
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

/// Background the renderer gives a day inside the selected range
pub(super) const RANGE_BG: Color = Color::rgb(60, 90, 120);

/// Default selection background (`Calendar::new`)
pub(super) const SELECTED_BG: Color = Color::CYAN;

mod snapshots {

    use revue::testing::{Pilot, TestApp};
    use revue::widget::Calendar;

    #[test]
    fn test_calendar_basic() {
        let view = Calendar::new(2024, 6); // June 2024

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("calendar_basic");
    }
}
