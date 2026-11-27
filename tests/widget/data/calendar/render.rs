//! Calendar month view tests: the whole grid, day names and weekend columns, the
//! selection, the size guard, stylesheet colors and odd years

use super::{cell, screen, screen_in};
use super::{header_fg, render, row_text};
use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::data::calendar::{Calendar, FirstDayOfWeek};
use revue::widget::traits::RenderContext;
use revue::widget::View;
use revue::widget::{days_in_month, Date};

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
    // 20x8 is the smallest area that draws a five-week month
    assert_eq!(screen_in(&cal, 20, 8)[0].trim(), "January 2025");
}

/// The size a month takes, from its setup (#998): the grid is 20 columns,
/// the focus arrow two more, week numbers four and a border two; the header
/// and day names are three rows over the weeks of the month, and a border
/// two more. In that size the month is drawn whole, one column or row less
/// and nothing is drawn.
#[test]
fn test_calendar_draws_whole_or_not_at_all() {
    // February 2015 spans four weeks, January 2025 five, March 2025 six
    for (year, month, weeks) in [(2015, 2, 4u16), (2025, 1, 5), (2025, 3, 6)] {
        for setup in 0..8u8 {
            let (week_numbers, border, focused) = (setup & 1 != 0, setup & 2 != 0, setup & 4 != 0);
            let plain = Calendar::new(year, month)
                .week_numbers(week_numbers)
                .focused(focused);
            let cal = if border {
                plain.clone().border(Color::WHITE)
            } else {
                plain.clone()
            };
            let edge = if border { 2 } else { 0 };
            let inner_w = 20 + if focused { 2 } else { 0 } + if week_numbers { 4 } else { 0 };
            let (w, h) = (inner_w + edge, 3 + weeks + edge);
            let name =
                format!("{year}-{month} numbers {week_numbers} border {border} focused {focused}");

            // Without a border, in a large area: the month touches its last
            // column and row and draws nothing past them
            let large = screen_in(&plain, 40, 14);
            let col = |rows: &[String], x: u16| -> Vec<char> {
                rows.iter()
                    .map(|r| r.chars().nth(x.into()).unwrap())
                    .collect()
            };
            assert!(
                col(&large, inner_w - 1).into_iter().any(|c| c != ' '),
                "{name}: last column"
            );
            assert!(
                (inner_w..40).all(|x| col(&large, x).into_iter().all(|c| c == ' ')),
                "{name}: past the width"
            );
            assert_ne!(
                large[usize::from(h - edge - 1)].trim(),
                "",
                "{name}: last row"
            );
            assert!(
                large[usize::from(h - edge)..]
                    .iter()
                    .all(|r| r.trim().is_empty()),
                "{name}"
            );

            // In exactly its size: every cell as in the large area, inside
            // the border when it has one
            let exact = screen_in(&cal, w, h);
            let inset = usize::from(border);
            for (y, row) in exact.iter().enumerate() {
                let row: Vec<char> = row.chars().collect();
                for (x, &ch) in row.iter().enumerate() {
                    let on_border =
                        border && (x == 0 || y == 0 || x + 1 == row.len() || y + 1 == exact.len());
                    if on_border {
                        assert_ne!(ch, ' ', "{name}: border at ({x}, {y})");
                    } else {
                        let want = large[y - inset].chars().nth(x - inset).unwrap();
                        assert_eq!(ch, want, "{name} at ({x}, {y}): {exact:#?}");
                    }
                }
            }

            // One column or one row less: nothing
            for (w, h) in [(w - 1, h), (w, h - 1)] {
                let rows = screen_in(&cal, w, h);
                assert!(
                    rows.iter().all(|r| r.trim().is_empty()),
                    "{name} at {w}x{h}: {rows:#?}"
                );
            }
        }
    }
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
fn test_calendar_draws_the_same_when_offset() {
    let cal = Calendar::new(2025, 3)
        .first_day(FirstDayOfWeek::Monday)
        .week_numbers(true)
        .border(Color::WHITE)
        .focused(true)
        .selected(Date::new(2025, 3, 12))
        .today(Date::new(2025, 3, 14));
    super::super::chart::assert_same_when_offset(cal, 30, 11);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// For any month, the days 1..=last appear in order, each in the column
    /// of its weekday, a week per row.
    #[test]
    fn test_calendar_days_sit_in_their_weekday_column(
        year in 1600i32..2400,
        month in 1u32..=12,
        monday_first in any::<bool>(),
    ) {
        let first = if monday_first { FirstDayOfWeek::Monday } else { FirstDayOfWeek::Sunday };
        let rows = screen_in(&Calendar::new(year, month).first_day(first), 30, 10);
        let offset = Date::new(year, month, 1).weekday();
        let offset = if monday_first { (offset + 6) % 7 } else { offset };
        for day in 1..=days_in_month(year, month) {
            let index = offset + day - 1;
            let (x, y) = ((index % 7) as usize * 3, 3 + (index / 7) as usize);
            let text: String = rows[y].chars().skip(x).take(2).collect();
            prop_assert_eq!(text.trim(), day.to_string(), "{}-{} row {}", year, month, y);
        }
    }
}
