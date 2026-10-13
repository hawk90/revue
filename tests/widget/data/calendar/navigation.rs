//! Calendar navigation tests: months, years, the selected date and the keys

use super::header;
use proptest::prelude::*;
use revue::event::Key;
use revue::widget::{Calendar, Date};

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

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Any sequence of arrow and vim keys moves the selection by the sum
    /// of their steps (±1 day, ±7 days), and the selection stays a real date.
    /// With nothing selected, the first day step selects day 1 of the shown
    /// month, so the first key moves one day less than its step.
    #[test]
    fn test_calendar_keys_move_by_days(
        year in 1900i32..2100,
        month in 1u32..=12,
        keys in prop::collection::vec(0usize..8, 1..60),
    ) {
        let keys_steps = [
            (Key::Left, -1i64), (Key::Right, 1), (Key::Up, -7), (Key::Down, 7),
            (Key::Char('h'), -1), (Key::Char('l'), 1), (Key::Char('k'), -7), (Key::Char('j'), 7),
        ];
        let mut cal = Calendar::new(year, month).focused(true);
        let mut net = 0i64;
        for (i, &k) in keys.iter().enumerate() {
            let (key, step) = &keys_steps[k];
            prop_assert!(cal.handle_key(key));
            net += if i == 0 { step - step.signum() } else { *step };
        }
        let start = Date::new(year, month, 1);
        let expected = if net >= 0 {
            start.add_days(net as u32)
        } else {
            start.subtract_days((-net) as u32)
        };
        let selected = cal.get_selected().unwrap();
        prop_assert!(selected.is_valid());
        prop_assert_eq!(selected, expected);
    }

    /// Going forward n months and back n months returns to the same month,
    /// and n months forward shows the month n later.
    #[test]
    fn test_calendar_month_steps(year in 1900i32..2100, month in 1u32..=12, n in 0u32..40) {
        let names = [
            "January", "February", "March", "April", "May", "June", "July", "August",
            "September", "October", "November", "December",
        ];
        let mut cal = Calendar::new(year, month);
        for _ in 0..n {
            cal.next_month();
        }
        let index = (month - 1 + n) as i32;
        let later = format!("{} {}", names[(index % 12) as usize], year + index / 12);
        prop_assert_eq!(header(&cal), later);
        for _ in 0..n {
            cal.prev_month();
        }
        prop_assert_eq!(header(&cal), format!("{} {}", names[month as usize - 1], year));
    }
}
