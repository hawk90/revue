//! Date/time range picker widget
//!
//! Provides a widget for selecting date ranges with:
//! - Start and end date selection
//! - Common preset ranges (Today, Last 7 Days, etc.)
//! - Optional time selection
//! - Validation to ensure end >= start

mod core;
mod impls;
mod navigation;
mod types;
mod view;

pub use core::RangePicker;
pub use types::{PresetRange, RangeFocus};

pub use impls::analytics_range_picker;
pub use impls::date_range_picker;
pub use impls::range_picker;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Key;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::Color;
    use crate::widget::data::calendar::{Date, FirstDayOfWeek};
    use crate::widget::datetime_picker::Time;
    use crate::widget::traits::{RenderContext, View};

    // =========================================================================
    // Constructors
    // =========================================================================

    #[test]
    fn test_range_picker_new() {
        let picker = RangePicker::new();
        let (start, end) = picker.get_range();
        assert_eq!(start, end); // Default is today-today
        assert_eq!(picker.get_focus(), RangeFocus::Start);
        assert!(picker.show_presets);
    }

    #[test]
    fn test_range_picker_default() {
        let picker = RangePicker::default();
        assert_eq!(picker.get_focus(), RangeFocus::Start);
    }

    // =========================================================================
    // Builder Methods - start_date
    // =========================================================================

    #[test]
    fn test_range_picker_start_date() {
        let picker = RangePicker::new().start_date(Date::new(2025, 1, 15));
        assert_eq!(picker.get_start(), Date::new(2025, 1, 15));
        assert_eq!(picker.start_cursor_day, 15);
    }

    #[test]
    fn test_range_picker_start_date_updates_cursor() {
        let picker = RangePicker::new().start_date(Date::new(2025, 2, 28));
        assert_eq!(picker.start_cursor_day, 28);
    }

    // =========================================================================
    // Builder Methods - end_date
    // =========================================================================

    #[test]
    fn test_range_picker_end_date() {
        let picker = RangePicker::new().end_date(Date::new(2025, 12, 31));
        assert_eq!(picker.get_end(), Date::new(2025, 12, 31));
    }

    #[test]
    fn test_range_picker_end_date_updates_cursor() {
        let picker = RangePicker::new().end_date(Date::new(2025, 3, 15));
        assert_eq!(picker.end_cursor_day, 15);
    }

    // =========================================================================
    // Builder Methods - start_time
    // =========================================================================

    #[test]
    fn test_range_picker_start_time() {
        let picker = RangePicker::new().start_time(Time::new(10, 30, 0));
        assert_eq!(picker.start.time.hour, 10);
        assert_eq!(picker.start.time.minute, 30);
    }

    // =========================================================================
    // Builder Methods - end_time
    // =========================================================================

    #[test]
    fn test_range_picker_end_time() {
        let picker = RangePicker::new().end_time(Time::new(23, 59, 59));
        assert_eq!(picker.end.time.hour, 23);
        assert_eq!(picker.end.time.minute, 59);
    }

    // =========================================================================
    // Builder Methods - range
    // =========================================================================

    #[test]
    fn test_range_picker_range() {
        let picker = RangePicker::new().range(Date::new(2025, 1, 1), Date::new(2025, 1, 31));
        assert_eq!(picker.get_start(), Date::new(2025, 1, 1));
        assert_eq!(picker.get_end(), Date::new(2025, 1, 31));
    }

    #[test]
    fn test_range_picker_range_updates_cursors() {
        let picker = RangePicker::new().range(Date::new(2025, 6, 15), Date::new(2025, 6, 20));
        assert_eq!(picker.start_cursor_day, 15);
        assert_eq!(picker.end_cursor_day, 20);
    }

    // =========================================================================
    // Builder Methods - first_day
    // =========================================================================

    #[test]
    fn test_range_picker_first_day_sunday() {
        let picker = RangePicker::new().first_day(FirstDayOfWeek::Sunday);
        assert_eq!(picker.first_day, FirstDayOfWeek::Sunday);
    }

    #[test]
    fn test_range_picker_first_day_monday() {
        let picker = RangePicker::new().first_day(FirstDayOfWeek::Monday);
        assert_eq!(picker.first_day, FirstDayOfWeek::Monday);
    }

    // =========================================================================
    // Builder Methods - show_time
    // =========================================================================

    #[test]
    fn test_range_picker_show_time_true() {
        let picker = RangePicker::new().show_time(true);
        assert!(picker.show_time);
    }

    #[test]
    fn test_range_picker_show_time_false() {
        let picker = RangePicker::new().show_time(false);
        assert!(!picker.show_time);
    }

    // =========================================================================
    // Builder Methods - with_presets
    // =========================================================================

    #[test]
    fn test_range_picker_with_presets_true() {
        let picker = RangePicker::new().with_presets(true);
        assert!(picker.show_presets);
    }

    #[test]
    fn test_range_picker_with_presets_false() {
        let picker = RangePicker::new().with_presets(false);
        assert!(!picker.show_presets);
    }

    // =========================================================================
    // Builder Methods - presets
    // =========================================================================

    #[test]
    fn test_range_picker_presets() {
        let custom_presets = vec![PresetRange::Today, PresetRange::Yesterday];
        let picker = RangePicker::new().presets(custom_presets.clone());
        assert_eq!(picker.presets, custom_presets);
    }

    // =========================================================================
    // Builder Methods - min_date
    // =========================================================================

    #[test]
    fn test_range_picker_min_date() {
        let picker = RangePicker::new().min_date(Date::new(2025, 1, 1));
        assert_eq!(picker.min_date, Some(Date::new(2025, 1, 1)));
    }

    // =========================================================================
    // Builder Methods - max_date
    // =========================================================================

    #[test]
    fn test_range_picker_max_date() {
        let picker = RangePicker::new().max_date(Date::new(2025, 12, 31));
        assert_eq!(picker.max_date, Some(Date::new(2025, 12, 31)));
    }

    // =========================================================================
    // Builder Methods - range_color
    // =========================================================================

    #[test]
    fn test_range_picker_range_color() {
        let picker = RangePicker::new().range_color(Color::RED);
        assert_eq!(picker.range_bg, Color::RED);
    }

    // =========================================================================
    // Getters
    // =========================================================================

    #[test]
    fn test_range_picker_get_range() {
        let picker = RangePicker::new()
            .start_date(Date::new(2025, 1, 1))
            .end_date(Date::new(2025, 1, 31));
        let (start, end) = picker.get_range();
        assert_eq!(start, Date::new(2025, 1, 1));
        assert_eq!(end, Date::new(2025, 1, 31));
    }

    #[test]
    fn test_range_picker_get_datetime_range() {
        let picker = RangePicker::new()
            .start_time(Time::new(10, 0, 0))
            .end_time(Time::new(18, 0, 0));
        let (start, end) = picker.get_datetime_range();
        assert_eq!(start.time.hour, 10);
        assert_eq!(end.time.hour, 18);
    }

    #[test]
    fn test_range_picker_get_start() {
        let picker = RangePicker::new().start_date(Date::new(2025, 3, 15));
        assert_eq!(picker.get_start(), Date::new(2025, 3, 15));
    }

    #[test]
    fn test_range_picker_get_end() {
        let picker = RangePicker::new().end_date(Date::new(2025, 6, 30));
        assert_eq!(picker.get_end(), Date::new(2025, 6, 30));
    }

    #[test]
    fn test_range_picker_get_active_preset() {
        let picker = RangePicker::new();
        assert_eq!(picker.get_active_preset(), Some(PresetRange::Today));
    }

    #[test]
    fn test_range_picker_get_active_preset_custom() {
        let picker = RangePicker::new().start_date(Date::new(2025, 1, 1));
        assert_eq!(picker.get_active_preset(), Some(PresetRange::Custom));
    }

    #[test]
    fn test_range_picker_get_focus() {
        let picker = RangePicker::new();
        assert_eq!(picker.get_focus(), RangeFocus::Start);
    }

    // =========================================================================
    // Setters
    // =========================================================================

    #[test]
    fn test_range_picker_set_start() {
        let mut picker = RangePicker::new();
        picker.set_end(Date::new(2025, 12, 31)); // Set end first to avoid swap
        picker.set_start(Date::new(2025, 5, 20));
        assert_eq!(picker.get_start(), Date::new(2025, 5, 20));
        assert_eq!(picker.start_cursor_day, 20);
    }

    #[test]
    fn test_range_picker_set_end() {
        let mut picker = RangePicker::new();
        picker.set_start(Date::new(2025, 1, 1)); // Set start first to avoid swap
        picker.set_end(Date::new(2025, 11, 30));
        assert_eq!(picker.get_end(), Date::new(2025, 11, 30));
        assert_eq!(picker.end_cursor_day, 30);
    }

    #[test]
    fn test_range_picker_set_start_swaps_if_needed() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2025, 6, 1))
            .end_date(Date::new(2025, 6, 30));
        picker.set_start(Date::new(2025, 7, 15));
        assert_eq!(picker.get_start(), Date::new(2025, 6, 30)); // Swapped
        assert_eq!(picker.get_end(), Date::new(2025, 7, 15));
    }

    // =========================================================================
    // is_in_range
    // =========================================================================

    #[test]
    fn test_range_picker_is_in_range() {
        let picker = RangePicker::new()
            .start_date(Date::new(2025, 1, 10))
            .end_date(Date::new(2025, 1, 20));

        assert!(picker.is_in_range(&Date::new(2025, 1, 15)));
        assert!(picker.is_in_range(&Date::new(2025, 1, 10)));
        assert!(picker.is_in_range(&Date::new(2025, 1, 20)));
        assert!(!picker.is_in_range(&Date::new(2025, 1, 5)));
        assert!(!picker.is_in_range(&Date::new(2025, 1, 25)));
    }

    #[test]
    fn test_range_picker_is_in_range_same_day() {
        let picker = RangePicker::new()
            .start_date(Date::new(2025, 1, 15))
            .end_date(Date::new(2025, 1, 15));

        assert!(picker.is_in_range(&Date::new(2025, 1, 15)));
        assert!(!picker.is_in_range(&Date::new(2025, 1, 14)));
        assert!(!picker.is_in_range(&Date::new(2025, 1, 16)));
    }

    // =========================================================================
    // apply_preset
    // =========================================================================

    #[test]
    fn test_range_picker_preset_apply() {
        let mut picker = RangePicker::new();
        let today = Date::today();

        picker.apply_preset(PresetRange::Today);
        let (start, end) = picker.get_range();
        assert_eq!(start, today);
        assert_eq!(end, today);
    }

    #[test]
    fn test_range_picker_preset_apply_updates_active_preset() {
        let mut picker = RangePicker::new();
        picker.apply_preset(PresetRange::Last7Days);
        assert_eq!(picker.get_active_preset(), Some(PresetRange::Last7Days));
    }

    #[test]
    fn test_range_picker_preset_apply_updates_cursors() {
        let mut picker = RangePicker::new();
        picker.apply_preset(PresetRange::Today);
        let today = Date::today();
        assert_eq!(picker.start_cursor_day, today.day);
        assert_eq!(picker.end_cursor_day, today.day);
    }

    // =========================================================================
    // PresetRange tests
    // =========================================================================

    #[test]
    fn test_preset_names() {
        assert_eq!(PresetRange::Today.name(), "Today");
        assert_eq!(PresetRange::Yesterday.name(), "Yesterday");
        assert_eq!(PresetRange::Last7Days.name(), "Last 7 Days");
        assert_eq!(PresetRange::Last30Days.name(), "Last 30 Days");
        assert_eq!(PresetRange::ThisWeek.name(), "This Week");
        assert_eq!(PresetRange::LastWeek.name(), "Last Week");
        assert_eq!(PresetRange::ThisMonth.name(), "This Month");
        assert_eq!(PresetRange::LastMonth.name(), "Last Month");
        assert_eq!(PresetRange::ThisYear.name(), "This Year");
        assert_eq!(PresetRange::Custom.name(), "Custom");
    }

    #[test]
    fn test_preset_common() {
        let common = PresetRange::common();
        assert!(common.contains(&PresetRange::Today));
        assert!(common.contains(&PresetRange::Last7Days));
        assert!(!common.contains(&PresetRange::Custom));
    }

    #[test]
    fn test_range_picker_preset_yesterday() {
        let today = Date::today();
        let yesterday = today.prev_day();
        let (start, end) = PresetRange::Yesterday.calculate(today);
        assert_eq!(start, yesterday);
        assert_eq!(end, yesterday);
    }

    #[test]
    fn test_range_picker_preset_last7days() {
        let today = Date::today();
        let (start, end) = PresetRange::Last7Days.calculate(today);
        assert_eq!(end, today);
        let expected_start = today.subtract_days(6);
        assert_eq!(start, expected_start);
    }

    #[test]
    fn test_range_picker_preset_last30days() {
        let today = Date::today();
        let (start, end) = PresetRange::Last30Days.calculate(today);
        assert_eq!(end, today);
        let expected_start = today.subtract_days(29);
        assert_eq!(start, expected_start);
    }

    #[test]
    fn test_range_picker_preset_this_month() {
        let today = Date::today();
        let (start, end) = PresetRange::ThisMonth.calculate(today);
        assert_eq!(start.day, 1);
        assert_eq!(start.month, today.month);
        assert_eq!(end, today);
    }

    #[test]
    fn test_range_picker_preset_last_month() {
        let today = Date::new(2025, 3, 15);
        let (start, end) = PresetRange::LastMonth.calculate(today);
        assert_eq!(start.year, 2025);
        assert_eq!(start.month, 2);
        assert_eq!(start.day, 1);
        assert_eq!(end.month, 2);
        assert_eq!(end.day, 28); // Feb 2025 has 28 days
    }

    #[test]
    fn test_range_picker_preset_last_month_january_wrap() {
        let today = Date::new(2025, 1, 15);
        let (start, end) = PresetRange::LastMonth.calculate(today);
        assert_eq!(start.year, 2024);
        assert_eq!(start.month, 12);
        assert_eq!(start.day, 1);
        assert_eq!(end.month, 12);
        assert_eq!(end.day, 31);
    }

    #[test]
    fn test_range_picker_preset_this_year() {
        let today = Date::new(2025, 6, 15);
        let (start, end) = PresetRange::ThisYear.calculate(today);
        assert_eq!(start, Date::new(2025, 1, 1));
        assert_eq!(end, today);
    }

    #[test]
    fn test_preset_custom_returns_today() {
        let today = Date::today();
        let (start, end) = PresetRange::Custom.calculate(today);
        assert_eq!(start, today);
        assert_eq!(end, today);
    }

    // =========================================================================
    // RangeFocus enum tests
    // =========================================================================

    #[test]
    fn test_range_focus_default() {
        let focus = RangeFocus::default();
        assert_eq!(focus, RangeFocus::Start);
    }

    #[test]
    fn test_range_focus_partial_eq() {
        assert_eq!(RangeFocus::Start, RangeFocus::Start);
        assert_ne!(RangeFocus::Start, RangeFocus::End);
    }

    // =========================================================================
    // PresetRange enum traits
    // =========================================================================

    #[test]
    fn test_preset_range_copy() {
        let preset = PresetRange::Today;
        let copied = preset;
        assert_eq!(preset, copied);
    }

    #[test]
    fn test_preset_range_clone() {
        let preset = PresetRange::Last7Days;
        let cloned = preset.clone();
        assert_eq!(preset, cloned);
    }

    // =========================================================================
    // Helper functions
    // =========================================================================

    #[test]
    fn test_range_picker_helper() {
        let picker = range_picker();
        assert_eq!(picker.get_focus(), RangeFocus::Start);
    }

    #[test]
    fn test_date_range_picker_helper() {
        let picker = date_range_picker();
        assert!(!picker.show_time);
    }

    #[test]
    fn test_analytics_range_picker_helper() {
        let picker = analytics_range_picker();
        assert!(picker.show_presets);
        assert!(picker.presets.len() > 3);
    }

    // =========================================================================
    // swap_if_needed behavior
    // =========================================================================

    #[test]
    fn test_range_picker_swap_if_needed_already_ordered() {
        let picker = RangePicker::new()
            .start_date(Date::new(2025, 1, 1))
            .end_date(Date::new(2025, 1, 31));
        let (start, end) = picker.get_range();
        assert_eq!(start, Date::new(2025, 1, 1));
        assert_eq!(end, Date::new(2025, 1, 31));
    }

    #[test]
    fn test_range_picker_swap_if_needed_reversed() {
        let picker = RangePicker::new()
            .start_date(Date::new(2025, 12, 31))
            .end_date(Date::new(2025, 1, 1));
        let (start, end) = picker.get_range();
        assert!(start <= end);
        assert_eq!(start, Date::new(2025, 1, 1));
        assert_eq!(end, Date::new(2025, 12, 31));
    }

    #[test]
    fn test_range_picker_swap_if_needed_same_date() {
        let picker = RangePicker::new()
            .start_date(Date::new(2025, 6, 15))
            .end_date(Date::new(2025, 6, 15));
        let (start, end) = picker.get_range();
        assert_eq!(start, end);
    }

    // =========================================================================
    // WidgetProps integration
    // =========================================================================

    #[test]
    fn test_range_picker_has_widget_props() {
        let picker = RangePicker::new();
        // Verify props field exists and is accessible
        let _ = &picker.props;
    }

    // =========================================================================
    // WidgetState integration
    // =========================================================================

    #[test]
    fn test_range_picker_has_widget_state() {
        let picker = RangePicker::new();
        // Verify state field exists and is accessible
        let _ = &picker.state;
    }

    // =========================================================================
    // Key handling tests (via handle_key)
    // =========================================================================

    #[test]
    fn test_range_picker_handle_key_tab() {
        let mut picker = RangePicker::new();
        assert_eq!(picker.focus, RangeFocus::Start);

        picker.handle_key(&Key::Tab);
        assert_eq!(picker.focus, RangeFocus::End);

        picker.handle_key(&Key::Tab);
        assert_eq!(picker.focus, RangeFocus::Presets);
    }

    #[test]
    fn test_range_picker_handle_key_back_tab() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::End;

        picker.handle_key(&Key::BackTab);
        assert_eq!(picker.focus, RangeFocus::Start);
    }

    #[test]
    fn test_range_picker_handle_key_select() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2025, 6, 15))
            .end_date(Date::new(2025, 6, 20));
        picker.start_cursor_day = 10;

        picker.handle_key(&Key::Enter);
        assert_eq!(picker.start.date.day, 10);
    }

    #[test]
    fn test_range_picker_handle_key_space_selects() {
        let mut picker = RangePicker::new();
        picker.end_cursor_day = 25;
        picker.focus = RangeFocus::End;

        picker.handle_key(&Key::Char(' '));
        assert_eq!(picker.end.date.day, 25);
    }

    // =========================================================================
    // Render tests
    // =========================================================================

    #[test]
    fn test_range_picker_render() {
        let picker = RangePicker::new()
            .start_date(Date::new(2025, 6, 15))
            .end_date(Date::new(2025, 6, 20));

        let mut buffer = Buffer::new(80, 15);
        let area = Rect::new(0, 0, 80, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        picker.render(&mut ctx);
        // Smoke test - should not panic
    }

    #[test]
    fn test_range_picker_render_small_area() {
        let picker = RangePicker::new();

        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(0, 0, 30, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        picker.render(&mut ctx);
        // Should return early for small area
    }

    #[test]
    fn test_range_picker_render_without_presets() {
        let picker = RangePicker::new()
            .with_presets(false)
            .start_date(Date::new(2025, 3, 10))
            .end_date(Date::new(2025, 3, 20));

        let mut buffer = Buffer::new(60, 12);
        let area = Rect::new(0, 0, 60, 12);
        let mut ctx = RenderContext::new(&mut buffer, area);

        picker.render(&mut ctx);
    }

    #[test]
    fn test_range_picker_render_monday_start() {
        let picker = RangePicker::new()
            .first_day(FirstDayOfWeek::Monday)
            .start_date(Date::new(2025, 6, 15))
            .end_date(Date::new(2025, 6, 20));

        let mut buffer = Buffer::new(80, 15);
        let area = Rect::new(0, 0, 80, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        picker.render(&mut ctx);
    }
}
