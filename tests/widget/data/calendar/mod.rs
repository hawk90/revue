//! Calendar widget tests

mod date;
mod render;
mod types;
mod utils;

use revue::event::Key;
use revue::layout::Rect;
use revue::render::{Buffer, Cell, Modifier};
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::StyledView;
use revue::widget::View;
use revue::widget::{calendar, days_in_month, Calendar, Date, DateMarker, FirstDayOfWeek};

/// Each row of `cal` drawn in a `width` x `height` area.
fn screen_in(cal: &Calendar, width: u16, height: u16) -> Vec<String> {
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
fn screen(cal: &Calendar) -> Vec<String> {
    screen_in(cal, 30, 10)
}

/// The month and year the calendar shows (without the focus arrows).
fn header(cal: &Calendar) -> String {
    screen(cal)[0].trim_matches([' ', '◀', '▶']).to_string()
}

/// The cell at (`x`, `y`) of `cal` drawn in a 30 x 10 area.
fn cell(cal: &Calendar, x: u16, y: u16) -> Cell {
    let mut buffer = Buffer::new(30, 10);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 30, 10));
    cal.render(&mut ctx);
    *buffer.get(x, y).unwrap()
}

// =============================================================================
// 생성자 및 빌더 테스트 (Constructor and Builder Tests)
// =============================================================================

#[test]
fn test_calendar_new() {
    let cal = Calendar::new(2025, 6);
    // 2025-06-01 is a Sunday: the first column of the first week
    let rows = screen(&cal);
    assert_eq!(rows[0].trim(), "June 2025");
    assert_eq!(rows[2].trim_end(), "Su Mo Tu We Th Fr Sa");
    assert_eq!(rows[3].trim_end(), " 1  2  3  4  5  6  7");
    assert_eq!(rows[7].trim_end(), "29 30");
}
#[test]
fn test_calendar_default() {
    assert_eq!(
        screen(&Calendar::default()),
        screen(&Calendar::new(2025, 1))
    );
}
#[test]
fn test_calendar_helper() {
    assert_eq!(screen(&calendar(2025, 6)), screen(&Calendar::new(2025, 6)));
}
#[test]
fn test_calendar_month_clamping() {
    // 월이 12를 초과하면 12로 제한됨 (월 > 12는 12로 제한)
    assert_eq!(header(&Calendar::new(2025, 13)), "December 2025");
    // 월이 1 미만이면 1로 제한됨 (월 < 1은 1로 제한)
    assert_eq!(header(&Calendar::new(2025, 0)), "January 2025");
}
#[test]
fn test_calendar_builder_selected() {
    let cal = Calendar::new(2025, 1).selected(Date::new(2025, 1, 15));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 15)));
}

#[test]
fn test_calendar_builder_range() {
    let cal = Calendar::new(2025, 1).range(Date::new(2025, 1, 10), Date::new(2025, 1, 20));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 10)));
}

#[test]
fn test_calendar_builder_today() {
    let cal = Calendar::new(2025, 1).today(Date::new(2025, 1, 10));
    // Friday the 10th: row 4, column 5
    let today = cell(&cal, 16, 4);
    assert_eq!(today.symbol, '0');
    assert_eq!(today.fg, Some(Color::YELLOW));
    assert!(today.modifier.contains(Modifier::BOLD));
    assert_eq!(cell(&cal, 13, 4).fg, Some(Color::WHITE)); // the 9th
}
#[test]
fn test_calendar_builder_header_color() {
    let cal = Calendar::new(2025, 1).header_color(Color::MAGENTA);
    assert_eq!(cell(&cal, 4, 0).fg, Some(Color::MAGENTA)); // "January"
    assert_eq!(cell(&cal, 3, 2).fg, Some(Color::MAGENTA)); // "Mo"
}
#[test]
fn test_calendar_builder_header_bg() {
    let cal = Calendar::new(2025, 1).header_bg(Color::BLUE);
    for x in 4..16 {
        assert_eq!(cell(&cal, x, 0).bg, Some(Color::BLUE), "column {x}");
    }
    assert_eq!(cell(&cal, 3, 2).bg, None); // day names keep no background
}

#[test]
fn test_calendar_builder_selected_color() {
    let cal = Calendar::new(2025, 1)
        .selected_color(Color::BLACK, Color::YELLOW)
        .selected(Date::new(2025, 1, 15));
    // Wednesday the 15th: row 5, column 3
    for x in [9, 10] {
        let day = cell(&cal, x, 5);
        assert_eq!(
            (day.fg, day.bg),
            (Some(Color::BLACK), Some(Color::YELLOW)),
            "column {x}"
        );
    }
}
#[test]
fn test_calendar_builder_today_color() {
    let cal = Calendar::new(2025, 1)
        .today_color(Color::GREEN)
        .today(Date::new(2025, 1, 10));
    assert_eq!(cell(&cal, 16, 4).fg, Some(Color::GREEN));
}
#[test]
fn test_calendar_builder_border() {
    let cal = Calendar::new(2025, 1).border(Color::WHITE);
    let mut buffer = Buffer::new(30, 12);
    let area = Rect::new(0, 0, 30, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);
    cal.render(&mut ctx);

    // 테두리 렌더링 확인 (Verify border is rendered)
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_calendar_builder_focused() {
    let cal = Calendar::new(2025, 1).focused(true);
    // Arrows on both sides of the month name
    assert_eq!(screen(&cal)[0].trim_end(), "◀   January 2025     ▶");
    assert_eq!(cell(&cal, 0, 0).fg, Some(Color::CYAN));
    assert_eq!(
        screen(&Calendar::new(2025, 1))[0].trim_end(),
        "    January 2025"
    );
}
#[test]
fn test_calendar_builder_chain() {
    let cal = Calendar::new(2025, 6)
        .selected(Date::new(2025, 6, 15))
        .first_day(FirstDayOfWeek::Monday)
        .week_numbers(true)
        .today(Date::new(2025, 6, 10))
        .header_color(Color::CYAN)
        .day_color(Color::WHITE)
        .weekend_color(Color::rgb(150, 150, 150))
        .selected_color(Color::BLACK, Color::YELLOW)
        .today_color(Color::GREEN)
        .focused(true);

    assert_eq!(cal.get_selected(), Some(Date::new(2025, 6, 15)));
}

// =============================================================================
// Date 테스트 (Date Tests)
// =============================================================================

#[test]
fn test_date_new() {
    let date = Date::new(2025, 6, 15);
    assert_eq!(date.year, 2025);
    assert_eq!(date.month, 6);
    assert_eq!(date.day, 15);
}

#[test]
fn test_date_default() {
    let date = Date::default();
    assert_eq!(date.year, 2025);
    assert_eq!(date.month, 1);
    assert_eq!(date.day, 1);
}

#[test]
fn test_date_today() {
    let date = Date::today();
    assert_eq!(date, Date::new(2025, 1, 1));
}

#[test]
fn test_date_valid() {
    assert!(Date::new(2025, 1, 1).is_valid());
    assert!(Date::new(2025, 2, 28).is_valid());
    assert!(Date::new(2024, 2, 29).is_valid()); // 윤년 (Leap year)
    assert!(!Date::new(2025, 2, 29).is_valid()); // 윤년 아님 (Not leap year)
    assert!(!Date::new(2025, 13, 1).is_valid());
    assert!(!Date::new(2025, 1, 32).is_valid());
    assert!(!Date::new(2025, 4, 31).is_valid());
}

#[test]
fn test_date_weekday() {
    // 2025년 1월 1일은 수요일 (3 = Wednesday)
    assert_eq!(Date::new(2025, 1, 1).weekday(), 3);
}

#[test]
fn test_date_prev_day() {
    let date = Date::new(2025, 1, 1);
    let prev = date.prev_day();
    assert_eq!(prev, Date::new(2024, 12, 31));
}

#[test]
fn test_date_prev_day_month_boundary() {
    let date = Date::new(2025, 3, 1);
    let prev = date.prev_day();
    assert_eq!(prev, Date::new(2025, 2, 28));
}

#[test]
fn test_date_prev_day_leap_year() {
    let date = Date::new(2024, 3, 1);
    let prev = date.prev_day();
    assert_eq!(prev, Date::new(2024, 2, 29));
}

#[test]
fn test_date_next_day() {
    let date = Date::new(2025, 1, 31);
    let next = date.next_day();
    assert_eq!(next, Date::new(2025, 2, 1));
}

#[test]
fn test_date_next_day_year_boundary() {
    let date = Date::new(2025, 12, 31);
    let next = date.next_day();
    assert_eq!(next, Date::new(2026, 1, 1));
}

#[test]
fn test_date_subtract_days() {
    let date = Date::new(2025, 1, 10);
    let result = date.subtract_days(5);
    assert_eq!(result, Date::new(2025, 1, 5));
}

#[test]
fn test_date_subtract_days_cross_month() {
    let date = Date::new(2025, 2, 5);
    let result = date.subtract_days(10);
    assert_eq!(result, Date::new(2025, 1, 26));
}

#[test]
fn test_date_add_days() {
    let date = Date::new(2025, 1, 10);
    let result = date.add_days(5);
    assert_eq!(result, Date::new(2025, 1, 15));
}

#[test]
fn test_date_add_days_cross_month() {
    let date = Date::new(2025, 1, 28);
    let result = date.add_days(5);
    assert_eq!(result, Date::new(2025, 2, 2));
}

#[test]
fn test_date_ordering() {
    let date1 = Date::new(2025, 1, 10);
    let date2 = Date::new(2025, 1, 15);
    let date3 = Date::new(2025, 1, 15);

    assert!(date1 < date2);
    assert!(date2 > date1);
    assert!(date2 <= date3);
    assert!(date2 >= date3);
    assert!(date2 == date3);
}

// =============================================================================
// DateMarker 테스트 (DateMarker Tests)
// =============================================================================

#[test]
fn test_date_marker_new() {
    let marker = DateMarker::new(Date::new(2025, 1, 1), Color::RED);
    assert_eq!(marker.date, Date::new(2025, 1, 1));
    assert_eq!(marker.color, Color::RED);
    assert_eq!(marker.symbol, None);
}

#[test]
fn test_date_marker_symbol() {
    let marker = DateMarker::new(Date::new(2025, 1, 1), Color::RED).symbol('★');
    assert_eq!(marker.symbol, Some('★'));
}

// =============================================================================
// 날짜 탐색 테스트 (Date Navigation Tests)
// =============================================================================

#[test]
fn test_calendar_next_month() {
    let mut cal = Calendar::new(2025, 1);
    cal.next_month();
    assert_eq!(header(&cal), "February 2025");
}
#[test]
fn test_calendar_next_month_year_boundary() {
    let mut cal = Calendar::new(2025, 12);
    cal.next_month();
    assert_eq!(header(&cal), "January 2026");
}
#[test]
fn test_calendar_prev_month() {
    let mut cal = Calendar::new(2025, 6);
    cal.prev_month();
    assert_eq!(header(&cal), "May 2025");
}
#[test]
fn test_calendar_prev_month_year_boundary() {
    let mut cal = Calendar::new(2025, 1);
    cal.prev_month();
    assert_eq!(header(&cal), "December 2024");
}
#[test]
fn test_calendar_next_year() {
    let mut cal = Calendar::new(2025, 6);
    cal.next_year();
    assert_eq!(header(&cal), "June 2026");
}
#[test]
fn test_calendar_prev_year() {
    let mut cal = Calendar::new(2025, 6);
    cal.prev_year();
    assert_eq!(header(&cal), "June 2024");
}
#[test]
fn test_calendar_navigation_cycle() {
    let mut cal = Calendar::new(2025, 6);

    // 여러 달 앞으로 이동 (Navigate forward several months)
    for _ in 0..8 {
        cal.next_month();
    }
    assert_eq!(header(&cal), "February 2026");

    // 여러 달 뒤로 이동 (Navigate backward several months)
    for _ in 0..15 {
        cal.prev_month();
    }
    assert_eq!(header(&cal), "November 2024");
}
// =============================================================================
// 선택 테스트 (Selection Tests)
// =============================================================================

#[test]
fn test_calendar_select() {
    let mut cal = Calendar::new(2025, 1);
    cal.select(Date::new(2025, 1, 15));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 15)));
}

#[test]
fn test_calendar_select_different_month() {
    let mut cal = Calendar::new(2025, 1);
    cal.select(Date::new(2024, 12, 25));
    assert_eq!(cal.get_selected(), Some(Date::new(2024, 12, 25)));
}

#[test]
fn test_calendar_clear_selection() {
    let mut cal = Calendar::new(2025, 1).selected(Date::new(2025, 1, 15));
    cal.clear_selection();
    assert_eq!(cal.get_selected(), None);
}

#[test]
fn test_calendar_select_next_day() {
    let mut cal = Calendar::new(2025, 1).selected(Date::new(2025, 1, 15));
    cal.select_next_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 16)));
}

#[test]
fn test_calendar_select_next_day_month_boundary() {
    let mut cal = Calendar::new(2025, 1).selected(Date::new(2025, 1, 31));
    cal.select_next_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 2, 1)));
    assert_eq!(header(&cal), "February 2025");
}

#[test]
fn test_calendar_select_next_day_year_boundary() {
    let mut cal = Calendar::new(2025, 12).selected(Date::new(2025, 12, 31));
    cal.select_next_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2026, 1, 1)));
    assert_eq!(header(&cal), "January 2026");
}

#[test]
fn test_calendar_select_prev_day() {
    let mut cal = Calendar::new(2025, 1).selected(Date::new(2025, 1, 15));
    cal.select_prev_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 14)));
}

#[test]
fn test_calendar_select_prev_day_month_boundary() {
    let mut cal = Calendar::new(2025, 2).selected(Date::new(2025, 2, 1));
    cal.select_prev_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 31)));
    assert_eq!(header(&cal), "January 2025");
}

#[test]
fn test_calendar_select_prev_day_year_boundary() {
    let mut cal = Calendar::new(2025, 1).selected(Date::new(2025, 1, 1));
    cal.select_prev_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2024, 12, 31)));
    assert_eq!(header(&cal), "December 2024");
}

#[test]
fn test_calendar_select_next_week() {
    let mut cal = Calendar::new(2025, 1).selected(Date::new(2025, 1, 10));
    cal.select_next_week();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 17)));
}

#[test]
fn test_calendar_select_next_week_cross_month() {
    let mut cal = Calendar::new(2025, 1).selected(Date::new(2025, 1, 28));
    cal.select_next_week();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 2, 4)));
}

#[test]
fn test_calendar_select_prev_week() {
    let mut cal = Calendar::new(2025, 1).selected(Date::new(2025, 1, 15));
    cal.select_prev_week();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 8)));
}

#[test]
fn test_calendar_select_prev_week_cross_month() {
    let mut cal = Calendar::new(2025, 2).selected(Date::new(2025, 2, 5));
    cal.select_prev_week();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 29)));
}

#[test]
fn test_calendar_select_without_initial_selection() {
    let mut cal = Calendar::new(2025, 1);
    cal.select_next_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 1)));

    let mut cal2 = Calendar::new(2025, 1);
    cal2.select_prev_day();
    assert_eq!(cal2.get_selected(), Some(Date::new(2025, 1, 1)));
}

// =============================================================================
// 키보드 핸들링 테스트 (Keyboard Handling Tests)
// =============================================================================

#[test]
fn test_calendar_handle_key_left() {
    let mut cal = Calendar::new(2025, 1)
        .selected(Date::new(2025, 1, 15))
        .focused(true);
    assert!(cal.handle_key(&Key::Left));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 14)));
}

#[test]
fn test_calendar_handle_key_right() {
    let mut cal = Calendar::new(2025, 1)
        .selected(Date::new(2025, 1, 15))
        .focused(true);
    assert!(cal.handle_key(&Key::Right));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 16)));
}

#[test]
fn test_calendar_handle_key_up() {
    let mut cal = Calendar::new(2025, 1)
        .selected(Date::new(2025, 1, 15))
        .focused(true);
    assert!(cal.handle_key(&Key::Up));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 8)));
}

#[test]
fn test_calendar_handle_key_down() {
    let mut cal = Calendar::new(2025, 1)
        .selected(Date::new(2025, 1, 15))
        .focused(true);
    assert!(cal.handle_key(&Key::Down));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 22)));
}

#[test]
fn test_calendar_handle_key_vim_left() {
    let mut cal = Calendar::new(2025, 1)
        .selected(Date::new(2025, 1, 15))
        .focused(true);
    assert!(cal.handle_key(&Key::Char('h')));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 14)));
}

#[test]
fn test_calendar_handle_key_vim_right() {
    let mut cal = Calendar::new(2025, 1)
        .selected(Date::new(2025, 1, 15))
        .focused(true);
    assert!(cal.handle_key(&Key::Char('l')));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 16)));
}

#[test]
fn test_calendar_handle_key_vim_up() {
    let mut cal = Calendar::new(2025, 1)
        .selected(Date::new(2025, 1, 15))
        .focused(true);
    assert!(cal.handle_key(&Key::Char('k')));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 8)));
}

#[test]
fn test_calendar_handle_key_vim_down() {
    let mut cal = Calendar::new(2025, 1)
        .selected(Date::new(2025, 1, 15))
        .focused(true);
    assert!(cal.handle_key(&Key::Char('j')));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 22)));
}

#[test]
fn test_calendar_handle_key_prev_month() {
    let mut cal = Calendar::new(2025, 6).focused(true);
    assert!(cal.handle_key(&Key::Char('[')));
    assert_eq!(header(&cal), "May 2025");
}

#[test]
fn test_calendar_handle_key_next_month() {
    let mut cal = Calendar::new(2025, 6).focused(true);
    assert!(cal.handle_key(&Key::Char(']')));
    assert_eq!(header(&cal), "July 2025");
}

#[test]
fn test_calendar_handle_key_prev_year() {
    let mut cal = Calendar::new(2025, 6).focused(true);
    assert!(cal.handle_key(&Key::Char('{')));
    assert_eq!(header(&cal), "June 2024");
}

#[test]
fn test_calendar_handle_key_next_year() {
    let mut cal = Calendar::new(2025, 6).focused(true);
    assert!(cal.handle_key(&Key::Char('}')));
    assert_eq!(header(&cal), "June 2026");
}

#[test]
fn test_calendar_handle_key_unfocused() {
    let mut cal = Calendar::new(2025, 1)
        .selected(Date::new(2025, 1, 15))
        .focused(false);
    assert!(!cal.handle_key(&Key::Left));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 15)));
}

#[test]
fn test_calendar_handle_key_unknown() {
    let mut cal = Calendar::new(2025, 1)
        .selected(Date::new(2025, 1, 15))
        .focused(true);
    assert!(!cal.handle_key(&Key::Char('x')));
    assert!(!cal.handle_key(&Key::Enter));
    assert!(!cal.handle_key(&Key::Tab));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 15)));
}

// =============================================================================
// 렌더링 테스트 (Rendering Tests)
// =============================================================================

#[test]
fn test_calendar_render_basic() {
    let rows = screen(&Calendar::new(2025, 1));
    let expected = [
        "    January 2025",
        "",
        "Su Mo Tu We Th Fr Sa",
        "          1  2  3  4",
        " 5  6  7  8  9 10 11",
        "12 13 14 15 16 17 18",
        "19 20 21 22 23 24 25",
        "26 27 28 29 30 31",
        "",
    ];
    for (row, want) in rows.iter().zip(expected) {
        assert_eq!(row.trim_end(), want);
    }
}
#[test]
fn test_calendar_render_with_selected() {
    let cal = Calendar::new(2025, 1).selected(Date::new(2025, 1, 15));
    let day = cell(&cal, 10, 5);
    assert_eq!(day.symbol, '5');
    assert_eq!((day.fg, day.bg), (Some(Color::BLACK), Some(Color::CYAN)));
    assert!(day.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_calendar_render_with_border() {
    let cal = Calendar::new(2025, 1).border(Color::WHITE);
    let mut buffer = Buffer::new(30, 12);
    let area = Rect::new(0, 0, 30, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cal.render(&mut ctx);

    // 테두리 모서리 확인 (Verify border corners)
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(29, 0).unwrap().symbol, '┐');
    assert_eq!(buffer.get(0, 11).unwrap().symbol, '└');
    assert_eq!(buffer.get(29, 11).unwrap().symbol, '┘');
}

#[test]
fn test_calendar_render_small_area() {
    // 작은 영역에서는 렌더링되지 않아야 함 (Should not render in small area)
    let cal = Calendar::new(2025, 1);
    assert!(screen_in(&cal, 19, 8).iter().all(|r| r.trim().is_empty()));
    assert!(screen_in(&cal, 20, 7).iter().all(|r| r.trim().is_empty()));
    // 20x8 is the smallest area that draws
    assert_eq!(screen_in(&cal, 20, 8)[0].trim(), "January 2025");
}
#[test]
fn test_calendar_render_zero_area() {
    let cal = Calendar::new(2025, 1).border(Color::WHITE);
    for (w, h) in [(0, 0), (0, 12), (30, 0)] {
        let mut buffer = Buffer::new(30, 12);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
        cal.render(&mut ctx);
        for y in 0..12 {
            for x in 0..30 {
                assert_eq!(
                    buffer.get(x, y).unwrap().symbol,
                    ' ',
                    "{w}x{h} at ({x}, {y})"
                );
            }
        }
    }
}

// =============================================================================
// 윤년 및 월 경계 테스트 (Leap Year and Month Boundary Tests)
// =============================================================================

#[test]
fn test_leap_year_divisible_by_4() {
    let mut cal = Calendar::new(2024, 2);
    // 2024는 윤년 (2024 is a leap year)
    cal.select(Date::new(2024, 2, 29));
    assert!(Date::new(2024, 2, 29).is_valid());
}

#[test]
fn test_leap_year_divisible_by_100() {
    // 1900은 윤년이 아님 (1900 is not a leap year)
    assert!(!Date::new(1900, 2, 29).is_valid());
}

#[test]
fn test_leap_year_divisible_by_400() {
    // 2000은 윤년 (2000 is a leap year)
    assert!(Date::new(2000, 2, 29).is_valid());
}

#[test]
fn test_february_non_leap_year() {
    // 2025는 윤년이 아님 (2025 is not a leap year)
    assert!(!Date::new(2025, 2, 29).is_valid());
    assert!(Date::new(2025, 2, 28).is_valid());
}

#[test]
fn test_days_in_month_all_months() {
    assert_eq!(days_in_month(2025, 1), 31); // January
    assert_eq!(days_in_month(2025, 2), 28); // February (non-leap)
    assert_eq!(days_in_month(2024, 2), 29); // February (leap)
    assert_eq!(days_in_month(2025, 3), 31); // March
    assert_eq!(days_in_month(2025, 4), 30); // April
    assert_eq!(days_in_month(2025, 5), 31); // May
    assert_eq!(days_in_month(2025, 6), 30); // June
    assert_eq!(days_in_month(2025, 7), 31); // July
    assert_eq!(days_in_month(2025, 8), 31); // August
    assert_eq!(days_in_month(2025, 9), 30); // September
    assert_eq!(days_in_month(2025, 10), 31); // October
    assert_eq!(days_in_month(2025, 11), 30); // November
    assert_eq!(days_in_month(2025, 12), 31); // December
}

#[test]
fn test_month_boundary_navigation_february() {
    let mut cal = Calendar::new(2025, 1);

    // 1월 31일에서 다음 달로 (From Jan 31 to next month)
    cal.select(Date::new(2025, 1, 31));
    cal.select_next_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 2, 1)));

    // 2월 28일에서 다음 달로 (From Feb 28 to next month)
    cal.select(Date::new(2025, 2, 28));
    cal.select_next_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 3, 1)));
}

#[test]
fn test_month_boundary_navigation_leap_february() {
    let mut cal = Calendar::new(2024, 2);

    // 윤년 2월 29일에서 다음 달로 (From leap year Feb 29 to next month)
    cal.select(Date::new(2024, 2, 29));
    cal.select_next_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2024, 3, 1)));
}

#[test]
fn test_month_30_days() {
    let mut cal = Calendar::new(2025, 4);

    // 4월 30일에서 다음 달로 (From April 30 to next month)
    cal.select(Date::new(2025, 4, 30));
    cal.select_next_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 5, 1)));
}

#[test]
fn test_year_boundary_navigation() {
    let mut cal = Calendar::new(2025, 12);

    // 12월 31일에서 다음 해로 (From Dec 31 to next year)
    cal.select(Date::new(2025, 12, 31));
    cal.select_next_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2026, 1, 1)));

    // 1월 1일에서 이전 해로 (From Jan 1 to previous year)
    cal.select(Date::new(2026, 1, 1));
    cal.select_prev_day();
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 12, 31)));
}

// =============================================================================
// CSS 스타일링 테스트 (CSS Styling Tests)
// =============================================================================

#[test]
fn test_calendar_css_id() {
    let cal = Calendar::new(2025, 1).element_id("my-calendar");
    assert_eq!(View::id(&cal), Some("my-calendar"));

    let meta = cal.meta();
    assert_eq!(meta.id, Some("my-calendar".to_string()));
}

#[test]
fn test_calendar_css_classes() {
    let cal = Calendar::new(2025, 1).class("primary").class("interactive");

    assert!(cal.has_class("primary"));
    assert!(cal.has_class("interactive"));
    assert!(!cal.has_class("secondary"));

    let meta = cal.meta();
    assert!(meta.classes.contains("primary"));
    assert!(meta.classes.contains("interactive"));
}

#[test]
fn test_calendar_css_classes_from_view_trait() {
    let cal = Calendar::new(2025, 1).class("calendar").class("widget");

    let classes = View::classes(&cal);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"calendar".to_string()));
    assert!(classes.contains(&"widget".to_string()));
}

#[test]
fn test_calendar_styled_view_set_id() {
    let mut cal = Calendar::new(2025, 1);
    cal.set_id("test-calendar");
    assert_eq!(View::id(&cal), Some("test-calendar"));
}

#[test]
fn test_calendar_styled_view_add_class() {
    let mut cal = Calendar::new(2025, 1);
    cal.add_class("active");
    assert!(cal.has_class("active"));
}

#[test]
fn test_calendar_styled_view_remove_class() {
    let mut cal = Calendar::new(2025, 1).class("active");
    cal.remove_class("active");
    assert!(!cal.has_class("active"));
}

#[test]
fn test_calendar_styled_view_toggle_class() {
    let mut cal = Calendar::new(2025, 1);

    cal.toggle_class("selected");
    assert!(cal.has_class("selected"));

    cal.toggle_class("selected");
    assert!(!cal.has_class("selected"));
}

#[test]
fn test_calendar_styled_view_has_class() {
    let cal = Calendar::new(2025, 1).class("visible");
    assert!(cal.has_class("visible"));
    assert!(!cal.has_class("hidden"));
}

#[test]
fn test_calendar_classes_builder() {
    let cal = Calendar::new(2025, 1).classes(vec!["class1", "class2", "class3"]);

    assert!(cal.has_class("class1"));
    assert!(cal.has_class("class2"));
    assert!(cal.has_class("class3"));
    assert_eq!(View::classes(&cal).len(), 3);
}

#[test]
fn test_calendar_duplicate_class_not_added() {
    let cal = Calendar::new(2025, 1).class("test").class("test");

    let classes = View::classes(&cal);
    assert_eq!(classes.len(), 1);
    assert!(classes.contains(&"test".to_string()));
}

#[test]
fn test_calendar_css_colors_from_context() {
    use revue::style::{Style, VisualStyle};

    let cal = Calendar::new(2025, 1);
    let mut buffer = Buffer::new(30, 12);
    let area = Rect::new(0, 0, 30, 12);

    let style = Style {
        visual: VisualStyle {
            color: Color::RED,
            background: Color::BLUE,
            ..VisualStyle::default()
        },
        ..Default::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    cal.render(&mut ctx);
    // An ordinary day (Thursday the 2nd) takes the stylesheet color
    assert_eq!(buffer.get(13, 3).unwrap().fg, Some(Color::RED));
}
#[test]
fn test_calendar_inline_override_css() {
    use revue::style::{Style, VisualStyle};

    let cal = Calendar::new(2025, 1).day_color(Color::GREEN);

    let mut buffer = Buffer::new(30, 12);
    let area = Rect::new(0, 0, 30, 12);

    let style = Style {
        visual: VisualStyle {
            color: Color::RED,
            background: Color::BLUE,
            ..VisualStyle::default()
        },
        ..Default::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    cal.render(&mut ctx);
    // The builder color wins over the stylesheet
    assert_eq!(buffer.get(13, 3).unwrap().fg, Some(Color::GREEN));
}
// =============================================================================
// Meta 및 디버그 테스트 (Meta and Debug Tests)
// =============================================================================

#[test]
fn test_calendar_meta() {
    let cal = Calendar::new(2025, 1)
        .element_id("test-calendar")
        .class("primary")
        .class("large");

    let meta = cal.meta();
    assert_eq!(meta.widget_type, "Calendar");
    assert_eq!(meta.id, Some("test-calendar".to_string()));
    assert!(meta.classes.contains("primary"));
    assert!(meta.classes.contains("large"));
}

// =============================================================================
// 주 번호 테스트 (Week Number Tests)
// =============================================================================

// Note: get_week_number is a private method, so we cannot test it directly.
// Week number functionality is tested indirectly through rendering with week_numbers(true)

// =============================================================================
// 복합 시나리오 테스트 (Complex Scenario Tests)
// =============================================================================

#[test]
fn test_calendar_full_navigation_cycle() {
    let mut cal = Calendar::new(2025, 6)
        .selected(Date::new(2025, 6, 15))
        .focused(true);

    // 다음 주로 이동 (Move to next week)
    cal.handle_key(&Key::Down);
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 6, 22)));

    // 다음 달로 이동 (Move to next month) - using API method
    // Note: next_month() changes calendar view but selection stays in original month
    cal.next_month();
    // Select a date in the new month
    cal.select(Date::new(2025, 7, 22));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 7, 22)));

    // 다음 해로 이동 (Move to next year)
    cal.next_year();
    cal.select(Date::new(2026, 7, 22));
    assert_eq!(cal.get_selected(), Some(Date::new(2026, 7, 22)));

    // 이전 주로 이동 (Move to previous week)
    cal.handle_key(&Key::Up);
    assert_eq!(cal.get_selected(), Some(Date::new(2026, 7, 15)));

    // 이전 달로 이동 (Move to previous month)
    cal.prev_month();
    cal.select(Date::new(2026, 6, 15));
    assert_eq!(cal.get_selected(), Some(Date::new(2026, 6, 15)));
}

#[test]
fn test_calendar_select_and_navigate() {
    let mut cal = Calendar::new(2025, 1);

    // 날짜 선택 후 달력 탐색 (Select date then navigate calendar)
    cal.select(Date::new(2025, 1, 15));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 15)));

    cal.next_month();

    // 선택은 유지됨 (Selection is preserved)
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 15)));
}

#[test]
fn test_calendar_selection_across_months() {
    let mut cal = Calendar::new(2025, 1)
        .selected(Date::new(2025, 1, 31))
        .focused(true);

    // 1월 31일에서 오른쪽으로 이동하면 2월 1일 (Right from Jan 31 goes to Feb 1)
    cal.handle_key(&Key::Right);
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 2, 1)));

    // 2월 1일에서 왼쪽으로 이동하면 1월 31일 (Left from Feb 1 goes to Jan 31)
    cal.handle_key(&Key::Left);
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 31)));
}

#[test]
fn test_calendar_full_year_navigation() {
    let mut cal = Calendar::new(2025, 6);

    // 1년 뒤로 이동 (Navigate back 1 year)
    for _ in 0..12 {
        cal.prev_month();
    }
    assert_eq!(header(&cal), "June 2024");

    // 1년 앞으로 이동 (Navigate forward 1 year)
    for _ in 0..12 {
        cal.next_month();
    }
    assert_eq!(header(&cal), "June 2025");
}

// =============================================================================
// 엣지 케이스 테스트 (Edge Case Tests)
// =============================================================================

#[test]
fn test_calendar_very_large_year() {
    let rows = screen(&Calendar::new(9999, 12));
    assert_eq!(rows[0].trim(), "December 9999");
    // 9999-12-01 is a Wednesday
    assert_eq!(rows[3].trim_end(), "          1  2  3  4");
}
#[test]
fn test_calendar_negative_year() {
    let rows = screen(&Calendar::new(-100, 6));
    assert_eq!(rows[0].trim(), "June -100");
    assert_eq!(rows[8].trim(), "30");
}
#[test]
fn test_calendar_year_zero() {
    let rows = screen(&Calendar::new(0, 6));
    assert_eq!(rows[0].trim(), "June 0");
    // Proleptic Gregorian: 0000-06-01 is a Thursday
    assert_eq!(rows[3].trim_end(), "             1  2  3");
}
#[test]
fn test_calendar_all_months_renderable() {
    let names = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    for month in 1..=12 {
        let rows = screen(&Calendar::new(2025, month));
        assert_eq!(
            rows[0].trim(),
            format!("{} 2025", names[month as usize - 1])
        );
        // The last day ends the grid
        let last = days_in_month(2025, month).to_string();
        let grid = rows[3..].join(" ");
        assert_eq!(
            grid.split_whitespace().last(),
            Some(last.as_str()),
            "month {month}"
        );
    }
}
#[test]
fn test_calendar_multiple_selections() {
    let mut cal = Calendar::new(2025, 1);

    // 여러 날짜 순차 선택 (Sequential selection of multiple dates)
    cal.select(Date::new(2025, 1, 5));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 5)));

    cal.select(Date::new(2025, 1, 15));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 15)));

    cal.select(Date::new(2025, 1, 25));
    assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 25)));
}

#[test]
fn test_calendar_rapid_navigation() {
    let mut cal = Calendar::new(2025, 6).focused(true);

    // 빠른 연속 탐색 (Rapid consecutive navigation)
    for _ in 0..100 {
        cal.handle_key(&Key::Right);
    }

    // 선택이 계속 유효해야 함 (Selection should still be valid)
    if let Some(date) = cal.get_selected() {
        let date: Date = date;
        assert!(date.is_valid());
    }
}

#[test]
fn test_calendar_clear_then_select() {
    let mut cal = Calendar::new(2025, 1)
        .selected(Date::new(2025, 1, 15))
        .focused(true);

    cal.clear_selection();
    assert_eq!(cal.get_selected(), None);

    // 선택 후 탐색 가능 (Can navigate after selection)
    cal.handle_key(&Key::Right);
    assert!(cal.get_selected().is_some());
}

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
