//! Calendar builder tests: construction, each setting checked on the screen, CSS ids and classes

use super::{cell, header, screen};
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{calendar, Calendar, Date, FirstDayOfWeek};
use revue::widget::{StyledView, View};

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
