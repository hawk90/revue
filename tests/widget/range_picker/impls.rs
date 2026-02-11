//! Range picker builder methods, getters, setters, and helpers tests
//! Extracted from src/widget/range_picker/impls.rs

use revue::style::Color;
use revue::widget::data::calendar::{Date, FirstDayOfWeek};
use revue::widget::datetime_picker::{DateTime, Time};
use revue::widget::range_picker::{PresetRange, RangePicker};

// =========================================================================
// Builder method tests - start_date
// =========================================================================

#[test]
fn test_start_date_sets_date() {
    let picker = RangePicker::new().start_date(Date::new(2024, 6, 15));
    assert_eq!(picker.start.date, Date::new(2024, 6, 15));
}

#[test]
fn test_start_date_updates_cursor() {
    let picker = RangePicker::new().start_date(Date::new(2024, 6, 20));
    assert_eq!(picker.start_cursor_day, 20);
}

#[test]
fn test_start_date_sets_custom_preset() {
    let picker = RangePicker::new().start_date(Date::new(2024, 6, 15));
    assert_eq!(picker.active_preset, Some(PresetRange::Custom));
}

// =========================================================================
// Builder method tests - end_date
// =========================================================================

#[test]
fn test_end_date_sets_date() {
    let picker = RangePicker::new()
        .start_date(Date::new(2024, 6, 1))
        .end_date(Date::new(2024, 6, 30));
    assert_eq!(picker.end.date, Date::new(2024, 6, 30));
}

#[test]
fn test_end_date_updates_cursor() {
    let picker = RangePicker::new()
        .start_date(Date::new(2024, 6, 1))
        .end_date(Date::new(2024, 6, 25));
    assert_eq!(picker.end_cursor_day, 25);
}

#[test]
fn test_end_date_sets_custom_preset() {
    let picker = RangePicker::new().end_date(Date::new(2024, 6, 30));
    assert_eq!(picker.active_preset, Some(PresetRange::Custom));
}

#[test]
fn test_end_date_swaps_if_needed() {
    let picker = RangePicker::new()
        .start_date(Date::new(2024, 6, 20))
        .end_date(Date::new(2024, 6, 10));
    // Should swap to maintain start <= end
    assert!(picker.start.date <= picker.end.date);
}

// =========================================================================
// Builder method tests - start_time
// =========================================================================

#[test]
fn test_start_time_sets_hour() {
    let picker = RangePicker::new().start_time(Time::new(10, 0, 0));
    assert_eq!(picker.start.time.hour, 10);
}

#[test]
fn test_start_time_sets_minute() {
    let picker = RangePicker::new().start_time(Time::new(10, 30, 0));
    assert_eq!(picker.start.time.minute, 30);
}

#[test]
fn test_start_time_sets_second() {
    let picker = RangePicker::new().start_time(Time::new(10, 0, 45));
    assert_eq!(picker.start.time.second, 45);
}

// =========================================================================
// Builder method tests - end_time
// =========================================================================

#[test]
fn test_end_time_sets_hour() {
    let picker = RangePicker::new().end_time(Time::new(23, 0, 0));
    assert_eq!(picker.end.time.hour, 23);
}

#[test]
fn test_end_time_sets_minute() {
    let picker = RangePicker::new().end_time(Time::new(23, 30, 0));
    assert_eq!(picker.end.time.minute, 30);
}

#[test]
fn test_end_time_sets_second() {
    let picker = RangePicker::new().end_time(Time::new(23, 0, 59));
    assert_eq!(picker.end.time.second, 59);
}

// =========================================================================
// Builder method tests - range
// =========================================================================

#[test]
fn test_range_sets_start_and_end() {
    let picker = RangePicker::new().range(Date::new(2024, 1, 1), Date::new(2024, 12, 31));
    assert_eq!(picker.start.date, Date::new(2024, 1, 1));
    assert_eq!(picker.end.date, Date::new(2024, 12, 31));
}

#[test]
fn test_range_updates_cursors() {
    let picker = RangePicker::new().range(Date::new(2024, 6, 10), Date::new(2024, 6, 20));
    assert_eq!(picker.start_cursor_day, 10);
    assert_eq!(picker.end_cursor_day, 20);
}

#[test]
fn test_range_sets_custom_preset() {
    let picker = RangePicker::new().range(Date::new(2024, 1, 1), Date::new(2024, 12, 31));
    assert_eq!(picker.active_preset, Some(PresetRange::Custom));
}

#[test]
fn test_range_swaps_if_reversed() {
    let picker = RangePicker::new().range(Date::new(2024, 12, 31), Date::new(2024, 1, 1));
    assert!(picker.start.date <= picker.end.date);
}

// =========================================================================
// Builder method tests - first_day
// =========================================================================

#[test]
fn test_first_day_sunday() {
    let picker = RangePicker::new().first_day(FirstDayOfWeek::Sunday);
    assert_eq!(picker.first_day, FirstDayOfWeek::Sunday);
}

#[test]
fn test_first_day_monday() {
    let picker = RangePicker::new().first_day(FirstDayOfWeek::Monday);
    assert_eq!(picker.first_day, FirstDayOfWeek::Monday);
}

// =========================================================================
// Builder method tests - show_time
// =========================================================================

#[test]
fn test_show_time_true() {
    let picker = RangePicker::new().show_time(true);
    assert!(picker.show_time);
}

#[test]
fn test_show_time_false() {
    let picker = RangePicker::new().show_time(false);
    assert!(!picker.show_time);
}

// =========================================================================
// Builder method tests - with_presets
// =========================================================================

#[test]
fn test_with_presets_true() {
    let picker = RangePicker::new().with_presets(true);
    assert!(picker.show_presets);
}

#[test]
fn test_with_presets_false() {
    let picker = RangePicker::new().with_presets(false);
    assert!(!picker.show_presets);
}

// =========================================================================
// Builder method tests - presets
// =========================================================================

#[test]
fn test_presets_custom_list() {
    let custom = vec![
        PresetRange::Today,
        PresetRange::Yesterday,
        PresetRange::Last7Days,
    ];
    let picker = RangePicker::new().presets(custom.clone());
    assert_eq!(picker.presets, custom);
}

#[test]
fn test_presets_empty_list() {
    let picker = RangePicker::new().presets(vec![]);
    assert!(picker.presets.is_empty());
}

// =========================================================================
// Builder method tests - min_date
// =========================================================================

#[test]
fn test_min_date_sets_constraint() {
    let picker = RangePicker::new().min_date(Date::new(2024, 1, 1));
    assert_eq!(picker.min_date, Some(Date::new(2024, 1, 1)));
}

// =========================================================================
// Builder method tests - max_date
// =========================================================================

#[test]
fn test_max_date_sets_constraint() {
    let picker = RangePicker::new().max_date(Date::new(2024, 12, 31));
    assert_eq!(picker.max_date, Some(Date::new(2024, 12, 31)));
}

// =========================================================================
// Builder method tests - range_color
// =========================================================================

#[test]
fn test_range_color_sets_color() {
    let picker = RangePicker::new().range_color(Color::RED);
    assert_eq!(picker.range_bg, Color::RED);
}

// =========================================================================
// Getter tests - get_range
// =========================================================================

#[test]
fn test_get_range_returns_tuple() {
    let picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 1))
        .end_date(Date::new(2024, 12, 31));
    let (start, end) = picker.get_range();
    assert_eq!(start, Date::new(2024, 1, 1));
    assert_eq!(end, Date::new(2024, 12, 31));
}

// =========================================================================
// Getter tests - get_datetime_range
// =========================================================================

#[test]
fn test_get_datetime_range_includes_time() {
    let picker = RangePicker::new()
        .start_time(Time::new(10, 0, 0))
        .end_time(Time::new(18, 0, 0));
    let (start, end) = picker.get_datetime_range();
    assert_eq!(start.time.hour, 10);
    assert_eq!(end.time.hour, 18);
}

// =========================================================================
// Getter tests - get_start
// =========================================================================

#[test]
fn test_get_start_returns_start_date() {
    let picker = RangePicker::new().start_date(Date::new(2024, 6, 15));
    assert_eq!(picker.get_start(), Date::new(2024, 6, 15));
}

// =========================================================================
// Getter tests - get_end
// =========================================================================

#[test]
fn test_get_end_returns_end_date() {
    let picker = RangePicker::new()
        .start_date(Date::new(2024, 6, 1))
        .end_date(Date::new(2024, 6, 30));
    assert_eq!(picker.get_end(), Date::new(2024, 6, 30));
}

// =========================================================================
// Getter tests - get_active_preset
// =========================================================================

#[test]
fn test_get_active_preset_today() {
    let picker = RangePicker::new();
    assert_eq!(picker.get_active_preset(), Some(PresetRange::Today));
}

#[test]
fn test_get_active_preset_custom() {
    let picker = RangePicker::new().start_date(Date::new(2024, 6, 15));
    assert_eq!(picker.get_active_preset(), Some(PresetRange::Custom));
}

// =========================================================================
// Getter tests - is_in_range
// =========================================================================

#[test]
fn test_is_in_range_inside() {
    let picker = RangePicker::new().range(Date::new(2024, 6, 10), Date::new(2024, 6, 20));
    assert!(picker.is_in_range(&Date::new(2024, 6, 15)));
}

#[test]
fn test_is_in_range_on_start() {
    let picker = RangePicker::new().range(Date::new(2024, 6, 10), Date::new(2024, 6, 20));
    assert!(picker.is_in_range(&Date::new(2024, 6, 10)));
}

#[test]
fn test_is_in_range_on_end() {
    let picker = RangePicker::new().range(Date::new(2024, 6, 10), Date::new(2024, 6, 20));
    assert!(picker.is_in_range(&Date::new(2024, 6, 20)));
}

#[test]
fn test_is_in_range_before_start() {
    let picker = RangePicker::new().range(Date::new(2024, 6, 10), Date::new(2024, 6, 20));
    assert!(!picker.is_in_range(&Date::new(2024, 6, 5)));
}

#[test]
fn test_is_in_range_after_end() {
    let picker = RangePicker::new().range(Date::new(2024, 6, 10), Date::new(2024, 6, 20));
    assert!(!picker.is_in_range(&Date::new(2024, 6, 25)));
}

#[test]
fn test_is_in_range_same_day() {
    let picker = RangePicker::new().range(Date::new(2024, 6, 15), Date::new(2024, 6, 15));
    assert!(picker.is_in_range(&Date::new(2024, 6, 15)));
    assert!(!picker.is_in_range(&Date::new(2024, 6, 14)));
    assert!(!picker.is_in_range(&Date::new(2024, 6, 16)));
}

// =========================================================================
// Getter tests - get_focus
// =========================================================================

#[test]
fn test_get_focus_returns_start() {
    let picker = RangePicker::new();
    assert_eq!(picker.get_focus(), RangeFocus::Start);
}

// =========================================================================
// Setter tests - set_start
// =========================================================================

#[test]
fn test_set_start_updates_date() {
    let mut picker = RangePicker::new();
    picker.set_start(Date::new(2024, 6, 20));
    assert_eq!(picker.start.date, Date::new(2024, 6, 20));
}

#[test]
fn test_set_start_updates_cursor() {
    let mut picker = RangePicker::new();
    picker.set_start(Date::new(2024, 6, 25));
    assert_eq!(picker.start_cursor_day, 25);
}

#[test]
fn test_set_start_sets_custom_preset() {
    let mut picker = RangePicker::new();
    picker.set_start(Date::new(2024, 6, 15));
    assert_eq!(picker.active_preset, Some(PresetRange::Custom));
}

#[test]
fn test_set_start_swaps_if_needed() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 6, 10))
        .end_date(Date::new(2024, 6, 20));
    picker.set_start(Date::new(2024, 6, 25));
    // Should swap to maintain start <= end
    assert!(picker.start.date <= picker.end.date);
}

// =========================================================================
// Setter tests - set_end
// =========================================================================

#[test]
fn test_set_end_updates_date() {
    let mut picker = RangePicker::new();
    picker.set_start(Date::new(2024, 6, 1));
    picker.set_end(Date::new(2024, 6, 30));
    assert_eq!(picker.end.date, Date::new(2024, 6, 30));
}

#[test]
fn test_set_end_updates_cursor() {
    let mut picker = RangePicker::new();
    picker.set_start(Date::new(2024, 6, 1));
    picker.set_end(Date::new(2024, 6, 25));
    assert_eq!(picker.end_cursor_day, 25);
}

#[test]
fn test_set_end_sets_custom_preset() {
    let mut picker = RangePicker::new();
    picker.set_start(Date::new(2024, 6, 1));
    picker.set_end(Date::new(2024, 6, 30));
    assert_eq!(picker.active_preset, Some(PresetRange::Custom));
}

// =========================================================================
// Setter tests - apply_preset
// =========================================================================

#[test]
fn test_apply_preset_today() {
    let mut picker = RangePicker::new();
    picker.apply_preset(PresetRange::Today);
    let today = Date::today();
    assert_eq!(picker.start.date, today);
    assert_eq!(picker.end.date, today);
    assert_eq!(picker.active_preset, Some(PresetRange::Today));
}

#[test]
fn test_apply_preset_yesterday() {
    let mut picker = RangePicker::new();
    picker.apply_preset(PresetRange::Yesterday);
    let today = Date::today();
    let yesterday = today.prev_day();
    assert_eq!(picker.start.date, yesterday);
    assert_eq!(picker.end.date, yesterday);
    assert_eq!(picker.active_preset, Some(PresetRange::Yesterday));
}

#[test]
fn test_apply_preset_last_7_days() {
    let mut picker = RangePicker::new();
    picker.apply_preset(PresetRange::Last7Days);
    let today = Date::today();
    let expected_start = today.subtract_days(6);
    assert_eq!(picker.start.date, expected_start);
    assert_eq!(picker.end.date, today);
    assert_eq!(picker.active_preset, Some(PresetRange::Last7Days));
}

#[test]
fn test_apply_preset_updates_cursors() {
    let mut picker = RangePicker::new();
    picker.apply_preset(PresetRange::Today);
    let today = Date::today();
    assert_eq!(picker.start_cursor_day, today.day);
    assert_eq!(picker.end_cursor_day, today.day);
}

// =========================================================================
// swap_if_needed tests
// =========================================================================

#[test]
fn test_swap_if_needed_already_ordered() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 1))
        .end_date(Date::new(2024, 12, 31));
    picker.swap_if_needed();
    assert_eq!(picker.start.date, Date::new(2024, 1, 1));
    assert_eq!(picker.end.date, Date::new(2024, 12, 31));
}

#[test]
fn test_swap_if_needed_reversed() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 12, 31))
        .end_date(Date::new(2024, 1, 1));
    picker.swap_if_needed();
    assert_eq!(picker.start.date, Date::new(2024, 1, 1));
    assert_eq!(picker.end.date, Date::new(2024, 12, 31));
}

#[test]
fn test_swap_if_needed_same_day() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 6, 15))
        .end_date(Date::new(2024, 6, 15));
    picker.swap_if_needed();
    // Should not swap equal dates
    assert_eq!(picker.start.date, Date::new(2024, 6, 15));
    assert_eq!(picker.end.date, Date::new(2024, 6, 15));
}

#[test]
fn test_swap_if_needed_swaps_cursor_days() {
    let mut picker = RangePicker::new();
    // Set dates directly without swap (manually)
    picker.start.date = Date::new(2024, 12, 31);
    picker.end.date = Date::new(2024, 1, 1);
    picker.start_cursor_day = 31;
    picker.end_cursor_day = 1;
    picker.swap_if_needed();
    // After swap: start = Jan 1 (cursor_day 1), end = Dec 31 (cursor_day 31)
    assert_eq!(picker.start_cursor_day, 1);
    assert_eq!(picker.end_cursor_day, 31);
}

// =========================================================================
// Month name helper tests
// =========================================================================

#[test]
fn test_month_name_january() {
    assert_eq!(month_name(1), "Jan");
}

#[test]
fn test_month_name_february() {
    assert_eq!(month_name(2), "Feb");
}

#[test]
fn test_month_name_march() {
    assert_eq!(month_name(3), "Mar");
}

#[test]
fn test_month_name_april() {
    assert_eq!(month_name(4), "Apr");
}

#[test]
fn test_month_name_may() {
    assert_eq!(month_name(5), "May");
}

#[test]
fn test_month_name_june() {
    assert_eq!(month_name(6), "Jun");
}

#[test]
fn test_month_name_july() {
    assert_eq!(month_name(7), "Jul");
}

#[test]
fn test_month_name_august() {
    assert_eq!(month_name(8), "Aug");
}

#[test]
fn test_month_name_september() {
    assert_eq!(month_name(9), "Sep");
}

#[test]
fn test_month_name_october() {
    assert_eq!(month_name(10), "Oct");
}

#[test]
fn test_month_name_november() {
    assert_eq!(month_name(11), "Nov");
}

#[test]
fn test_month_name_december() {
    assert_eq!(month_name(12), "Dec");
}

#[test]
fn test_month_name_invalid() {
    assert_eq!(month_name(0), "???");
    assert_eq!(month_name(13), "???");
}

// =========================================================================
// Builder chaining tests
// =========================================================================

#[test]
fn test_builder_chain_start_end_range() {
    let picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 1))
        .end_date(Date::new(2024, 12, 31))
        .first_day(FirstDayOfWeek::Monday)
        .show_time(true)
        .with_presets(true);
    assert_eq!(picker.start.date, Date::new(2024, 1, 1));
    assert_eq!(picker.end.date, Date::new(2024, 12, 31));
    assert_eq!(picker.first_day, FirstDayOfWeek::Monday);
    assert!(picker.show_time);
    assert!(picker.show_presets);
}

#[test]
fn test_builder_chain_times() {
    let picker = RangePicker::new()
        .start_time(Time::new(0, 0, 0))
        .end_time(Time::new(23, 59, 59));
    assert_eq!(picker.start.time.hour, 0);
    assert_eq!(picker.end.time.hour, 23);
}

#[test]
fn test_builder_chain_constraints() {
    let picker = RangePicker::new()
        .min_date(Date::new(2024, 1, 1))
        .max_date(Date::new(2024, 12, 31));
    assert_eq!(picker.min_date, Some(Date::new(2024, 1, 1)));
    assert_eq!(picker.max_date, Some(Date::new(2024, 12, 31)));
}

#[test]
fn test_builder_chain_presets() {
    let custom_presets = vec![PresetRange::Today, PresetRange::Yesterday];
    let picker = RangePicker::new()
        .presets(custom_presets.clone())
        .with_presets(true);
    assert_eq!(picker.presets, custom_presets);
    assert!(picker.show_presets);
}

// =========================================================================
// Edge cases and integration tests
// =========================================================================

#[test]
fn test_set_start_and_end_maintains_order() {
    let mut picker = RangePicker::new();
    picker.set_start(Date::new(2024, 6, 20));
    picker.set_end(Date::new(2024, 6, 10));
    // Should swap to maintain start <= end
    assert!(picker.start.date <= picker.end.date);
}

#[test]
fn test_range_with_swapped_values() {
    let picker = RangePicker::new().range(Date::new(2024, 12, 31), Date::new(2024, 1, 1));
    // Builder should swap
    assert!(picker.start.date <= picker.end.date);
}

#[test]
fn test_apply_preset_then_set_start_becomes_custom() {
    let mut picker = RangePicker::new();
    picker.apply_preset(PresetRange::Today);
    assert_eq!(picker.active_preset, Some(PresetRange::Today));

    picker.set_start(Date::new(2024, 6, 15));
    assert_eq!(picker.active_preset, Some(PresetRange::Custom));
}

#[test]
fn test_is_in_range_with_time_components() {
    let picker = RangePicker::new()
        .start_time(Time::new(10, 0, 0))
        .end_time(Time::new(18, 0, 0))
        .range(Date::new(2024, 6, 10), Date::new(2024, 6, 20));
    // is_in_range only checks dates, not times
    assert!(picker.is_in_range(&Date::new(2024, 6, 15)));
}

#[test]
fn test_builder_all_options() {
    let picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 1))
        .end_date(Date::new(2024, 12, 31))
        .start_time(Time::new(0, 0, 0))
        .end_time(Time::new(23, 59, 59))
        .first_day(FirstDayOfWeek::Monday)
        .show_time(true)
        .with_presets(true)
        .presets(vec![PresetRange::Today])
        .min_date(Date::new(2024, 1, 1))
        .max_date(Date::new(2024, 12, 31))
        .range_color(Color::BLUE);

    assert_eq!(picker.start.date, Date::new(2024, 1, 1));
    assert_eq!(picker.end.date, Date::new(2024, 12, 31));
    assert_eq!(picker.start.time.hour, 0);
    assert_eq!(picker.end.time.hour, 23);
    assert_eq!(picker.first_day, FirstDayOfWeek::Monday);
    assert!(picker.show_time);
    assert!(picker.show_presets);
    assert_eq!(picker.presets.len(), 1);
    assert_eq!(picker.min_date, Some(Date::new(2024, 1, 1)));
    assert_eq!(picker.max_date, Some(Date::new(2024, 12, 31)));
    assert_eq!(picker.range_bg, Color::BLUE);
}