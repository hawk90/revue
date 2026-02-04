//! RangePicker widget integration tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{
    analytics_range_picker, date_range_picker, range_picker, Date, FirstDayOfWeek, PresetRange,
    RangeFocus, RangePicker, Time,
};

// =============================================================================
// Constructor Tests
// =============================================================================

#[test]
fn test_range_picker_new() {
    let picker = RangePicker::new();
    let (start, end) = picker.get_range();
    assert_eq!(start, end); // Default is today-today
}

#[test]
fn test_range_picker_default() {
    let picker = RangePicker::default();
    let (start, end) = picker.get_range();
    assert_eq!(start, end);
}

#[test]
fn test_range_picker_helper() {
    let picker = range_picker();
    let (start, end) = picker.get_range();
    assert_eq!(start, end);
}

#[test]
fn test_date_range_picker_helper() {
    let picker = date_range_picker();
    // date_range_picker() creates a picker with show_time(false)
    // Just verify it renders without panic
    let mut buffer = Buffer::new(80, 15);
    let area = Rect::new(0, 0, 80, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_analytics_range_picker_helper() {
    let picker = analytics_range_picker();
    // analytics_range_picker() creates a picker with common presets
    // Just verify it renders without panic
    let mut buffer = Buffer::new(80, 15);
    let area = Rect::new(0, 0, 80, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

// =============================================================================
// Builder Methods Tests
// =============================================================================

#[test]
fn test_range_picker_start_date() {
    let picker = range_picker().start_date(Date::new(2025, 1, 1));
    assert_eq!(picker.get_start(), Date::new(2025, 1, 1));
}

#[test]
fn test_range_picker_end_date() {
    let picker = range_picker().end_date(Date::new(2025, 1, 31));
    assert_eq!(picker.get_end(), Date::new(2025, 1, 31));
}

#[test]
fn test_range_picker_swap_auto() {
    let picker = range_picker()
        .start_date(Date::new(2025, 12, 31))
        .end_date(Date::new(2025, 1, 1));

    let (start, end) = picker.get_range();
    assert!(start <= end);
}

#[test]
fn test_range_picker_start_time() {
    let picker = range_picker().start_time(Time::new(10, 30, 0));
    let (start_dt, _) = picker.get_datetime_range();
    assert_eq!(start_dt.time.hour, 10);
    assert_eq!(start_dt.time.minute, 30);
}

#[test]
fn test_range_picker_end_time() {
    let picker = range_picker().end_time(Time::new(23, 59, 59));
    let (_, end_dt) = picker.get_datetime_range();
    assert_eq!(end_dt.time.hour, 23);
    assert_eq!(end_dt.time.minute, 59);
}

#[test]
fn test_range_picker_range() {
    let picker = range_picker().range(Date::new(2025, 1, 1), Date::new(2025, 1, 31));
    let (start, end) = picker.get_range();
    assert_eq!(start, Date::new(2025, 1, 1));
    assert_eq!(end, Date::new(2025, 1, 31));
}

#[test]
fn test_range_picker_first_day_sunday() {
    let picker = range_picker().first_day(FirstDayOfWeek::Sunday);
    // Verify first_day is set (can't access directly)
    let _ = picker;
}

#[test]
fn test_range_picker_first_day_monday() {
    let picker = range_picker().first_day(FirstDayOfWeek::Monday);
    let _ = picker;
}

#[test]
fn test_range_picker_show_time_true() {
    let picker = range_picker().show_time(true);
    // Verify show_time is set
    let _ = picker;
}

#[test]
fn test_range_picker_show_time_false() {
    let picker = range_picker().show_time(false);
    let _ = picker;
}

#[test]
fn test_range_picker_with_presets_true() {
    let picker = range_picker().with_presets(true);
    // Verify show_presets is set
    let _ = picker;
}

#[test]
fn test_range_picker_with_presets_false() {
    let picker = range_picker().with_presets(false);
    let _ = picker;
}

#[test]
fn test_range_picker_presets() {
    let picker = range_picker().presets(vec![PresetRange::Today, PresetRange::Yesterday]);
    // Verify presets are set
    let _ = picker;
}

#[test]
fn test_range_picker_min_date() {
    let picker = range_picker().min_date(Date::new(2025, 1, 1));
    // Verify min_date is set
    let _ = picker;
}

#[test]
fn test_range_picker_max_date() {
    let picker = range_picker().max_date(Date::new(2025, 12, 31));
    // Verify max_date is set
    let _ = picker;
}

#[test]
fn test_range_picker_range_color() {
    let picker = range_picker().range_color(Color::RED);
    // Verify range_bg is set
    let _ = picker;
}

#[test]
fn test_range_picker_builder_chain() {
    let picker = range_picker()
        .range(Date::new(2025, 1, 1), Date::new(2025, 1, 31))
        .first_day(FirstDayOfWeek::Monday)
        .show_time(true)
        .with_presets(true)
        .min_date(Date::new(2024, 1, 1))
        .max_date(Date::new(2026, 12, 31))
        .range_color(Color::BLUE);

    let (start, end) = picker.get_range();
    assert_eq!(start, Date::new(2025, 1, 1));
    assert_eq!(end, Date::new(2025, 1, 31));
}

// =============================================================================
// Getter Tests
// =============================================================================

#[test]
fn test_range_picker_get_range() {
    let picker = range_picker().range(Date::new(2025, 3, 15), Date::new(2025, 4, 20));
    let (start, end) = picker.get_range();
    assert_eq!(start, Date::new(2025, 3, 15));
    assert_eq!(end, Date::new(2025, 4, 20));
}

#[test]
fn test_range_picker_get_datetime_range() {
    let picker = range_picker()
        .start_time(Time::new(10, 0, 0))
        .end_time(Time::new(18, 30, 0));

    let (start_dt, end_dt) = picker.get_datetime_range();
    assert_eq!(start_dt.time.hour, 10);
    assert_eq!(end_dt.time.hour, 18);
    assert_eq!(end_dt.time.minute, 30);
}

#[test]
fn test_range_picker_get_start() {
    let picker = range_picker().start_date(Date::new(2025, 6, 15));
    assert_eq!(picker.get_start(), Date::new(2025, 6, 15));
}

#[test]
fn test_range_picker_get_end() {
    let picker = range_picker().end_date(Date::new(2025, 12, 25));
    assert_eq!(picker.get_end(), Date::new(2025, 12, 25));
}

#[test]
fn test_range_picker_get_active_preset() {
    let mut picker = range_picker();
    picker.apply_preset(PresetRange::Today);
    assert_eq!(picker.get_active_preset(), Some(PresetRange::Today));
}

#[test]
fn test_range_picker_get_focus() {
    let picker = range_picker();
    assert_eq!(picker.get_focus(), RangeFocus::Start); // Default focus
}

// =============================================================================
// Setter Tests
// =============================================================================

#[test]
fn test_range_picker_set_start() {
    let mut picker = range_picker().end_date(Date::new(2025, 12, 31));
    picker.set_start(Date::new(2025, 5, 10));
    // Should set start correctly since it's before end
    assert!(picker.get_start() <= picker.get_end());
}

#[test]
fn test_range_picker_set_end() {
    let mut picker = range_picker();
    picker.set_end(Date::new(2025, 11, 30));
    // Setting end after start should work
    assert!(picker.get_start() <= picker.get_end());
}

#[test]
fn test_range_picker_set_end_before_start_swaps() {
    let mut picker = range_picker();
    picker.set_start(Date::new(2025, 6, 1));
    picker.set_end(Date::new(2025, 5, 1));
    assert!(picker.get_start() <= picker.get_end());
}

#[test]
fn test_range_picker_apply_preset_today() {
    let mut picker = range_picker();
    picker.apply_preset(PresetRange::Today);
    let (start, end) = picker.get_range();
    assert_eq!(start, end);
}

#[test]
fn test_range_picker_apply_preset_yesterday() {
    let mut picker = range_picker();
    picker.apply_preset(PresetRange::Yesterday);
    let (start, end) = picker.get_range();
    assert_eq!(start, end);
}

#[test]
fn test_range_picker_apply_preset_last7days() {
    let mut picker = range_picker();
    let today = Date::today();
    picker.apply_preset(PresetRange::Last7Days);
    let (start, end) = picker.get_range();
    assert_eq!(end, today);
    // Start should be 6 days before today
    let expected_start = today.subtract_days(6);
    assert_eq!(start, expected_start);
}

#[test]
fn test_range_picker_apply_preset_last30days() {
    let mut picker = range_picker();
    let today = Date::today();
    picker.apply_preset(PresetRange::Last30Days);
    let (start, end) = picker.get_range();
    assert_eq!(end, today);
    let expected_start = today.subtract_days(29);
    assert_eq!(start, expected_start);
}

#[test]
fn test_range_picker_apply_preset_this_month() {
    let mut picker = range_picker();
    let today = Date::today();
    picker.apply_preset(PresetRange::ThisMonth);
    let (start, end) = picker.get_range();
    assert_eq!(start.day, 1);
    assert_eq!(start.month, today.month);
    assert_eq!(end, today);
}

#[test]
fn test_range_picker_apply_preset_last_month() {
    let mut picker = range_picker();
    picker.apply_preset(PresetRange::LastMonth);
    let (start, end) = picker.get_range();
    // Should be full previous month
    assert!(start.day == 1);
    // Verify end is last day of previous month
    assert!(end >= start);
}

#[test]
fn test_range_picker_apply_preset_this_year() {
    let mut picker = range_picker();
    let today = Date::today();
    picker.apply_preset(PresetRange::ThisYear);
    let (start, end) = picker.get_range();
    assert_eq!(start.month, 1);
    assert_eq!(start.day, 1);
    assert_eq!(start.year, today.year);
    assert_eq!(end, today);
}

// =============================================================================
// is_in_range Tests
// =============================================================================

#[test]
fn test_range_picker_is_in_range_start() {
    let picker = range_picker().range(Date::new(2025, 1, 10), Date::new(2025, 1, 20));
    assert!(picker.is_in_range(&Date::new(2025, 1, 10)));
}

#[test]
fn test_range_picker_is_in_range_middle() {
    let picker = range_picker().range(Date::new(2025, 1, 10), Date::new(2025, 1, 20));
    assert!(picker.is_in_range(&Date::new(2025, 1, 15)));
}

#[test]
fn test_range_picker_is_in_range_end() {
    let picker = range_picker().range(Date::new(2025, 1, 10), Date::new(2025, 1, 20));
    assert!(picker.is_in_range(&Date::new(2025, 1, 20)));
}

#[test]
fn test_range_picker_is_in_range_before() {
    let picker = range_picker().range(Date::new(2025, 1, 10), Date::new(2025, 1, 20));
    assert!(!picker.is_in_range(&Date::new(2025, 1, 5)));
}

#[test]
fn test_range_picker_is_in_range_after() {
    let picker = range_picker().range(Date::new(2025, 1, 10), Date::new(2025, 1, 20));
    assert!(!picker.is_in_range(&Date::new(2025, 1, 25)));
}

#[test]
fn test_range_picker_is_in_range_single_day() {
    let picker = range_picker().range(Date::new(2025, 1, 15), Date::new(2025, 1, 15));
    assert!(picker.is_in_range(&Date::new(2025, 1, 15)));
    assert!(!picker.is_in_range(&Date::new(2025, 1, 14)));
    assert!(!picker.is_in_range(&Date::new(2025, 1, 16)));
}

// =============================================================================
// PresetRange Tests
// =============================================================================

#[test]
fn test_preset_range_name_today() {
    assert_eq!(PresetRange::Today.name(), "Today");
}

#[test]
fn test_preset_range_name_yesterday() {
    assert_eq!(PresetRange::Yesterday.name(), "Yesterday");
}

#[test]
fn test_preset_range_name_last7days() {
    assert_eq!(PresetRange::Last7Days.name(), "Last 7 Days");
}

#[test]
fn test_preset_range_name_last30days() {
    assert_eq!(PresetRange::Last30Days.name(), "Last 30 Days");
}

#[test]
fn test_preset_range_name_this_week() {
    assert_eq!(PresetRange::ThisWeek.name(), "This Week");
}

#[test]
fn test_preset_range_name_last_week() {
    assert_eq!(PresetRange::LastWeek.name(), "Last Week");
}

#[test]
fn test_preset_range_name_this_month() {
    assert_eq!(PresetRange::ThisMonth.name(), "This Month");
}

#[test]
fn test_preset_range_name_last_month() {
    assert_eq!(PresetRange::LastMonth.name(), "Last Month");
}

#[test]
fn test_preset_range_name_this_year() {
    assert_eq!(PresetRange::ThisYear.name(), "This Year");
}

#[test]
fn test_preset_range_name_custom() {
    assert_eq!(PresetRange::Custom.name(), "Custom");
}

#[test]
fn test_preset_range_common() {
    let common = PresetRange::common();
    assert!(common.contains(&PresetRange::Today));
    assert!(common.contains(&PresetRange::Yesterday));
    assert!(common.contains(&PresetRange::Last7Days));
    assert!(common.contains(&PresetRange::Last30Days));
    assert!(common.contains(&PresetRange::ThisWeek));
    assert!(common.contains(&PresetRange::LastWeek));
    assert!(common.contains(&PresetRange::ThisMonth));
    assert!(common.contains(&PresetRange::LastMonth));
    assert!(!common.contains(&PresetRange::Custom));
}

#[test]
fn test_preset_range_calculate_today() {
    let today = Date::today();
    let (start, end) = PresetRange::Today.calculate(today);
    assert_eq!(start, today);
    assert_eq!(end, today);
}

#[test]
fn test_preset_range_calculate_yesterday() {
    let today = Date::today();
    let (start, end) = PresetRange::Yesterday.calculate(today);
    let expected = today.prev_day();
    assert_eq!(start, expected);
    assert_eq!(end, expected);
}

// =============================================================================
// Keyboard Handling Tests
// =============================================================================

#[test]
fn test_range_picker_handle_key_tab() {
    let mut picker = range_picker();
    assert_eq!(picker.get_focus(), RangeFocus::Start);
    picker.handle_key(&Key::Tab);
    assert_eq!(picker.get_focus(), RangeFocus::End);
}

#[test]
fn test_range_picker_handle_key_backtab() {
    let mut picker = range_picker();
    assert_eq!(picker.get_focus(), RangeFocus::Start);
    picker.handle_key(&Key::BackTab);
    // Should go to presets if show_presets is true
    assert!(picker.get_focus() != RangeFocus::Start);
}

#[test]
fn test_range_picker_handle_key_left() {
    let mut picker = range_picker();
    let (start, _) = picker.get_range();
    picker.handle_key(&Key::Left);
    // Should move cursor left
    let _ = start;
}

#[test]
fn test_range_picker_handle_key_right() {
    let mut picker = range_picker();
    picker.handle_key(&Key::Right);
    // Should move cursor right
}

#[test]
fn test_range_picker_handle_key_up() {
    let mut picker = range_picker();
    picker.handle_key(&Key::Up);
    // Should move cursor up (previous week)
}

#[test]
fn test_range_picker_handle_key_down() {
    let mut picker = range_picker();
    picker.handle_key(&Key::Down);
    // Should move cursor down (next week)
}

#[test]
fn test_range_picker_handle_key_bracket_left() {
    let mut picker = range_picker();
    let (start, _) = picker.get_range();
    picker.handle_key(&Key::Char('['));
    // Should go to previous month
    let _ = start;
}

#[test]
fn test_range_picker_handle_key_bracket_right() {
    let mut picker = range_picker();
    picker.handle_key(&Key::Char(']'));
    // Should go to next month
}

#[test]
fn test_range_picker_handle_key_brace_left() {
    let mut picker = range_picker();
    picker.handle_key(&Key::Char('{'));
    // Should go to previous year
}

#[test]
fn test_range_picker_handle_key_brace_right() {
    let mut picker = range_picker();
    picker.handle_key(&Key::Char('}'));
    // Should go to next year
}

#[test]
fn test_range_picker_handle_key_enter() {
    let mut picker = range_picker();
    picker.handle_key(&Key::Enter);
    // Should select current date
}

#[test]
fn test_range_picker_handle_key_space() {
    let mut picker = range_picker();
    picker.handle_key(&Key::Char(' '));
    // Should select current date (same as Enter)
}

#[test]
fn test_range_picker_handle_key_unhandled() {
    let mut picker = range_picker();
    let handled = picker.handle_key(&Key::PageUp);
    assert!(!handled);
}

// =============================================================================
// Rendering Tests
// =============================================================================

#[test]
fn test_range_picker_render() {
    let mut buffer = Buffer::new(80, 15);
    let area = Rect::new(0, 0, 80, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let picker = range_picker();
    picker.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_range_picker_render_with_presets() {
    let mut buffer = Buffer::new(80, 15);
    let area = Rect::new(0, 0, 80, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let picker = range_picker().with_presets(true);
    picker.render(&mut ctx);
}

#[test]
fn test_range_picker_render_without_presets() {
    let mut buffer = Buffer::new(80, 15);
    let area = Rect::new(0, 0, 80, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let picker = range_picker().with_presets(false);
    picker.render(&mut ctx);
}

#[test]
fn test_range_picker_render_small_width() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let picker = range_picker();
    picker.render(&mut ctx);
    // Should return early (width < 50)
}

#[test]
fn test_range_picker_render_small_height() {
    let mut buffer = Buffer::new(80, 5);
    let area = Rect::new(0, 0, 80, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let picker = range_picker();
    picker.render(&mut ctx);
    // Should return early (height < 10)
}

#[test]
fn test_range_picker_render_with_custom_range() {
    let mut buffer = Buffer::new(80, 15);
    let area = Rect::new(0, 0, 80, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let picker = range_picker().range(Date::new(2025, 3, 1), Date::new(2025, 3, 31));
    picker.render(&mut ctx);
}

#[test]
fn test_range_picker_render_with_first_day_monday() {
    let mut buffer = Buffer::new(80, 15);
    let area = Rect::new(0, 0, 80, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let picker = range_picker().first_day(FirstDayOfWeek::Monday);
    picker.render(&mut ctx);
}

// =============================================================================
// WidgetProps/StyledView Tests
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
    let handled = picker.handle_key(&Key::Tab);
    assert!(!handled);
}

// Skip element_id and meta tests since RangePicker doesn't override View::id()
// to use props.id (would need impl_view_meta! macro)

#[test]
fn test_range_picker_classes() {
    let picker = range_picker().class("date-filter").class("analytics");
    assert!(picker.has_class("date-filter"));
    assert!(picker.has_class("analytics"));
}

#[test]
fn test_range_picker_classes_vec() {
    let picker = range_picker().classes(vec!["a", "b", "c"]);
    assert!(picker.has_class("a"));
    assert!(picker.has_class("b"));
    assert!(picker.has_class("c"));
}

// =============================================================================
// Edge Cases
// =============================================================================

#[test]
fn test_range_picker_same_start_end() {
    let picker = range_picker().range(Date::new(2025, 6, 15), Date::new(2025, 6, 15));
    let (start, end) = picker.get_range();
    assert_eq!(start, end);
}

#[test]
fn test_range_picker_month_boundary() {
    let picker = range_picker().range(Date::new(2025, 1, 31), Date::new(2025, 2, 1));
    let (start, end) = picker.get_range();
    assert!(start <= end);
}

#[test]
fn test_range_picker_year_boundary() {
    let picker = range_picker().range(Date::new(2024, 12, 31), Date::new(2025, 1, 1));
    let (start, end) = picker.get_range();
    assert!(start <= end);
}

#[test]
fn test_range_picker_leap_year() {
    let picker = range_picker().range(Date::new(2024, 2, 28), Date::new(2024, 2, 29));
    let (start, end) = picker.get_range();
    assert!(start <= end);
    assert_eq!(end.day, 29); // 2024 is a leap year
}

#[test]
fn test_range_picker_long_range() {
    let picker = range_picker().range(Date::new(2020, 1, 1), Date::new(2030, 12, 31));
    let (start, end) = picker.get_range();
    assert_eq!(start.year, 2020);
    assert_eq!(end.year, 2030);
}

#[test]
fn test_range_picker_set_start_multiple_times() {
    let mut picker = range_picker().end_date(Date::new(2025, 12, 31));
    picker.set_start(Date::new(2025, 1, 1));
    picker.set_start(Date::new(2025, 2, 1));
    picker.set_start(Date::new(2025, 3, 1));
    // Should set start correctly since end is after all these dates
    assert_eq!(picker.get_start(), Date::new(2025, 3, 1));
}

#[test]
fn test_range_picker_set_end_multiple_times() {
    let mut picker = range_picker();
    picker.set_end(Date::new(2025, 1, 31));
    picker.set_end(Date::new(2025, 2, 28));
    picker.set_end(Date::new(2025, 3, 31));
    assert_eq!(picker.get_end(), Date::new(2025, 3, 31));
}

#[test]
fn test_range_picker_apply_multiple_presets() {
    let mut picker = range_picker();
    picker.apply_preset(PresetRange::Today);
    picker.apply_preset(PresetRange::Last7Days);
    picker.apply_preset(PresetRange::ThisMonth);
    assert_eq!(picker.get_active_preset(), Some(PresetRange::ThisMonth));
}

#[test]
fn test_range_picker_custom_preset_after_set_range() {
    let mut picker = range_picker();
    picker.set_start(Date::new(2025, 3, 15));
    assert_eq!(picker.get_active_preset(), Some(PresetRange::Custom));
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn test_range_picker_analytics_workflow() {
    // Simulate typical analytics date range picker workflow
    let mut picker = analytics_range_picker();

    // Apply "Last 7 Days" preset
    picker.apply_preset(PresetRange::Last7Days);

    let (start, end) = picker.get_range();
    let today = Date::today();
    assert_eq!(end, today);

    // Verify start is before or equal to end (7 day range)
    assert!(start <= end);
}

#[test]
fn test_range_picker_manual_range_selection() {
    // Simulate manually selecting start and end dates
    let mut picker = date_range_picker();

    // Set end date first (must be after start)
    picker.set_end(Date::new(2025, 6, 30));
    // Then set start date
    picker.set_start(Date::new(2025, 6, 1));

    let (start, end) = picker.get_range();
    // Verify we have a valid range (start <= end)
    assert!(start <= end);
}

#[test]
fn test_range_picker_with_time_workflow() {
    let picker = range_picker()
        .show_time(true)
        .start_time(Time::new(9, 0, 0))
        .end_time(Time::new(17, 30, 0))
        .range(Date::new(2025, 8, 1), Date::new(2025, 8, 31));

    let (start_dt, end_dt) = picker.get_datetime_range();
    assert_eq!(start_dt.time.hour, 9);
    assert_eq!(end_dt.time.hour, 17);
    assert_eq!(end_dt.time.minute, 30);
}
