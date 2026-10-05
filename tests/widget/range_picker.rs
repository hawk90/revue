//! RangePicker widget integration tests: rendering, widget props and
//! multi-step workflows. Builders, getters, setters, presets and key
//! handling are covered by the in-source tests in src/widget/range_picker/.

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{
    date_range_picker, range_picker, Date, FirstDayOfWeek, PresetRange, RangePicker, Time,
};

fn render(picker: &RangePicker, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    picker.render(&mut ctx);
    buffer
}

fn text(buffer: &Buffer, x: u16, y: u16, len: u16) -> String {
    (x..x + len)
        .filter_map(|x| buffer.get(x, y).map(|c| c.symbol))
        .collect()
}

fn is_blank(buffer: &Buffer) -> bool {
    (0..buffer.height())
        .all(|y| (0..buffer.width()).all(|x| buffer.get(x, y).unwrap().symbol == ' '))
}

/// March 2025: the 1st is a Saturday, 31 days
fn march_2025() -> RangePicker {
    range_picker().range(Date::new(2025, 3, 1), Date::new(2025, 3, 31))
}

// =============================================================================
// Rendering
// =============================================================================

#[test]
fn test_range_picker_render_headers_and_summary() {
    let buffer = render(&march_2025(), 80, 12);
    assert_eq!(text(&buffer, 0, 0, 15), "Start: Mar 2025");
    assert_eq!(text(&buffer, 24, 0, 13), "End: Mar 2025");
    assert_eq!(text(&buffer, 0, 9, 31), "Range: 2025-03-01 to 2025-03-31");
    assert!(text(&buffer, 0, 10, 60).starts_with("Tab: switch"));
}

#[test]
fn test_range_picker_render_days_sunday_first() {
    let buffer = render(&march_2025(), 80, 12);
    assert_eq!(text(&buffer, 0, 1, 20), "Su Mo Tu We Th Fr Sa");
    // The 1st is a Saturday: last column of the first week row
    assert_eq!(text(&buffer, 18, 2, 2), " 1");
    assert_eq!(text(&buffer, 0, 3, 20), " 2  3  4  5  6  7  8");
    // The 31st is a Monday on the sixth row
    assert_eq!(text(&buffer, 3, 7, 2), "31");
}

#[test]
fn test_range_picker_render_days_monday_first() {
    let picker = march_2025().first_day(FirstDayOfWeek::Monday);
    let buffer = render(&picker, 80, 12);
    assert_eq!(text(&buffer, 0, 1, 20), "Mo Tu We Th Fr Sa Su");
    assert_eq!(text(&buffer, 15, 2, 2), " 1");
    assert_eq!(text(&buffer, 0, 3, 20), " 3  4  5  6  7  8  9");
    assert_eq!(text(&buffer, 0, 7, 2), "31");
}

#[test]
fn test_range_picker_render_highlights_range_cursor_and_selection() {
    let picker = march_2025().range_color(Color::RED);
    let buffer = render(&picker, 80, 12);

    // Start calendar has focus: its cursor (the 1st) is inverted
    let cursor = buffer.get(19, 2).unwrap();
    assert_eq!(cursor.fg, Some(Color::BLACK));
    assert_eq!(cursor.bg, Some(Color::WHITE));

    // The 15th (Saturday, third row) is inside the range
    assert_eq!(buffer.get(18, 4).unwrap().bg, Some(Color::RED));
    assert_eq!(buffer.get(19, 4).unwrap().bg, Some(Color::RED));

    // A date outside the range is not highlighted
    let outside = range_picker().range(Date::new(2025, 3, 10), Date::new(2025, 3, 12));
    let buffer = render(&outside, 80, 12);
    assert_eq!(text(&buffer, 18, 4, 2), "15");
    assert_eq!(buffer.get(18, 4).unwrap().bg, None);
}

#[test]
fn test_range_picker_render_presets_panel() {
    let buffer = render(&march_2025(), 80, 12);
    assert_eq!(text(&buffer, 48, 0, 7), "Presets");
    // A custom range marks no preset as active
    assert_eq!(text(&buffer, 48, 1, 7), "  Today");

    let mut picker = march_2025();
    picker.apply_preset(PresetRange::Yesterday);
    let buffer = render(&picker, 80, 12);
    assert_eq!(text(&buffer, 48, 2, 11), "● Yesterday");
    // Without focus the preset cursor is not drawn
    assert_eq!(buffer.get(50, 1).unwrap().bg, None);

    // With the presets focused, the cursor row (the first preset) is a
    // highlighted bar under the text
    picker.handle_key(&Key::Tab);
    picker.handle_key(&Key::Tab);
    let buffer = render(&picker, 80, 12);
    assert_eq!(text(&buffer, 48, 1, 7), "  Today");
    let bar = buffer.get(50, 1).unwrap().bg;
    assert!(bar.is_some());
    assert!((48..64).all(|x| buffer.get(x, 1).unwrap().bg == bar));
    assert_eq!(buffer.get(50, 2).unwrap().bg, None);
}

#[test]
fn test_range_picker_render_without_presets() {
    let picker = march_2025().with_presets(false);
    let buffer = render(&picker, 80, 12);
    assert_eq!(text(&buffer, 48, 0, 7), "       ");
    assert_eq!(text(&buffer, 0, 0, 15), "Start: Mar 2025");
}

#[test]
fn test_range_picker_render_too_small_draws_nothing() {
    assert!(is_blank(&render(&march_2025(), 49, 12)));
    assert!(is_blank(&render(&march_2025(), 80, 9)));
    assert!(!is_blank(&render(&march_2025(), 50, 10)));
}

// =============================================================================
// WidgetProps / StyledView
// =============================================================================

#[test]
fn test_range_picker_focus() {
    let picker = range_picker().focused(true);
    assert!(picker.is_focused());
}

#[test]
fn test_range_picker_blur() {
    let picker = range_picker().focused(true).focused(false);
    assert!(!picker.is_focused());
}

#[test]
fn test_range_picker_disabled() {
    let picker = range_picker().disabled(true);
    assert!(picker.is_disabled());
}

#[test]
fn test_range_picker_handle_key_when_disabled() {
    let mut picker = range_picker().disabled(true);
    assert!(!picker.handle_key(&Key::Tab));
    assert!(!picker.handle_key(&Key::Right));
    assert!(!picker.handle_key(&Key::Enter));
    assert_eq!(picker.get_range(), (Date::today(), Date::today()));
}

#[test]
fn test_range_picker_classes() {
    let picker = range_picker().class("date-filter").class("analytics");
    assert!(picker.has_class("date-filter"));
    assert!(picker.has_class("analytics"));
    assert!(!picker.has_class("other"));
}

#[test]
fn test_range_picker_classes_vec() {
    let picker = range_picker().classes(vec!["a", "b", "c"]);
    assert!(picker.has_class("a"));
    assert!(picker.has_class("b"));
    assert!(picker.has_class("c"));
}

// =============================================================================
// Workflows
// =============================================================================

#[test]
fn test_range_picker_set_start_multiple_times() {
    let mut picker = range_picker().end_date(Date::new(2025, 12, 31));
    picker.set_start(Date::new(2025, 1, 1));
    picker.set_start(Date::new(2025, 2, 1));
    picker.set_start(Date::new(2025, 3, 1));
    assert_eq!(picker.get_start(), Date::new(2025, 3, 1));
    assert_eq!(picker.get_end(), Date::new(2025, 12, 31));
}

#[test]
fn test_range_picker_set_end_multiple_times() {
    let mut picker = range_picker().start_date(Date::new(2025, 1, 1));
    picker.set_end(Date::new(2025, 1, 31));
    picker.set_end(Date::new(2025, 2, 28));
    picker.set_end(Date::new(2025, 3, 31));
    assert_eq!(
        picker.get_range(),
        (Date::new(2025, 1, 1), Date::new(2025, 3, 31))
    );
}

#[test]
fn test_range_picker_apply_multiple_presets() {
    let mut picker = range_picker();
    picker.apply_preset(PresetRange::Today);
    picker.apply_preset(PresetRange::Last7Days);
    picker.apply_preset(PresetRange::ThisMonth);
    assert_eq!(picker.get_active_preset(), Some(PresetRange::ThisMonth));
    assert_eq!(
        picker.get_range(),
        PresetRange::ThisMonth.calculate(Date::today())
    );
}

#[test]
fn test_range_picker_manual_range_selection() {
    let mut picker = date_range_picker();
    // Set the end first, then a start before it
    picker.set_end(Date::new(2025, 6, 30));
    picker.set_start(Date::new(2025, 6, 1));
    assert_eq!(
        picker.get_range(),
        (Date::new(2025, 6, 1), Date::new(2025, 6, 30))
    );
    assert_eq!(picker.get_active_preset(), Some(PresetRange::Custom));
}

// =============================================================================
// Date limits and time display
// =============================================================================

#[test]
#[ignore = "BUG: RangePicker stores min_date but never enforces it"]
fn test_range_picker_min_date_limits_the_start() {
    let min = Date::new(2025, 3, 10);
    let mut picker = march_2025()
        .range(min, Date::new(2025, 3, 20))
        .min_date(min);

    // Start calendar has focus with its cursor on the 10th
    picker.handle_key(&Key::Left);
    picker.handle_key(&Key::Enter);
    assert!(picker.get_start() >= min, "{:?}", picker.get_start());

    picker.handle_key(&Key::Char('['));
    assert!(picker.get_start() >= min, "{:?}", picker.get_start());
}

#[test]
#[ignore = "BUG: RangePicker stores max_date but never enforces it"]
fn test_range_picker_max_date_limits_the_end() {
    let max = Date::new(2025, 3, 20);
    let mut picker = march_2025()
        .range(Date::new(2025, 3, 10), max)
        .max_date(max);

    // End calendar, cursor on the 20th
    picker.handle_key(&Key::Tab);
    picker.handle_key(&Key::Right);
    picker.handle_key(&Key::Enter);
    assert!(picker.get_end() <= max, "{:?}", picker.get_end());

    picker.handle_key(&Key::Char(']'));
    assert!(picker.get_end() <= max, "{:?}", picker.get_end());
}

#[test]
fn test_range_picker_show_time_draws_the_times() {
    let picker = march_2025()
        .show_time(true)
        .start_time(Time::new(9, 30, 0))
        .end_time(Time::new(17, 45, 0));
    let buffer = render(&picker, 80, 14);
    let all: String = (0..14).map(|y| text(&buffer, 0, y, 80)).collect();
    assert!(all.contains("09:30"), "{all}");
    assert!(all.contains("17:45"), "{all}");
    // Each time sits next to its own date in the summary
    assert_eq!(
        text(&buffer, 0, 9, 43),
        "Range: 2025-03-01 09:30 to 2025-03-31 17:45"
    );

    // Without show_time the times stay hidden
    let picker = march_2025()
        .show_time(false)
        .start_time(Time::new(9, 30, 0))
        .end_time(Time::new(17, 45, 0));
    let buffer = render(&picker, 80, 14);
    let all: String = (0..14).map(|y| text(&buffer, 0, y, 80)).collect();
    assert!(!all.contains("09:30"), "{all}");
}
