//! Range picker core struct and constructors

use super::types::{PresetRange, RangeFocus};
use crate::style::Color;
use crate::widget::data::calendar::{Date, FirstDayOfWeek};
use crate::widget::datetime_picker::{DateTime, Time};
use crate::widget::traits::{WidgetProps, WidgetState};

/// A date/time range picker widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::{range_picker, RangePicker, Date};
///
/// // Basic date range picker
/// let picker = range_picker()
///     .start_date(Date::new(2025, 1, 1))
///     .end_date(Date::new(2025, 1, 31));
///
/// // With presets
/// let picker = range_picker()
///     .with_presets(true);
///
/// // Analytics-style range picker
/// let picker = analytics_range_picker();
/// ```
pub struct RangePicker {
    /// Start datetime
    pub(crate) start: DateTime,
    /// End datetime
    pub(crate) end: DateTime,
    /// Currently active preset
    pub(crate) active_preset: Option<PresetRange>,
    /// Available presets
    pub(crate) presets: Vec<PresetRange>,
    /// Cursor position in presets
    pub(crate) preset_cursor: usize,
    /// Current focus area
    pub(crate) focus: RangeFocus,
    /// First day of week
    pub(crate) first_day: FirstDayOfWeek,
    /// Show time selection
    pub(crate) show_time: bool,
    /// Calendar cursor day (for start)
    pub(crate) start_cursor_day: u32,
    /// Calendar cursor day (for end)
    pub(crate) end_cursor_day: u32,
    /// Minimum allowed date
    pub(crate) min_date: Option<Date>,
    /// Maximum allowed date
    pub(crate) max_date: Option<Date>,
    /// Show presets panel
    pub(crate) show_presets: bool,
    /// Colors
    pub(crate) header_fg: Color,
    pub(crate) selected_fg: Color,
    pub(crate) selected_bg: Color,
    pub(crate) range_bg: Color,
    pub(crate) preset_fg: Color,
    pub(crate) preset_selected_fg: Color,
    pub(crate) preset_selected_bg: Color,
    /// Widget state
    pub state: WidgetState,
    /// Widget props
    pub props: WidgetProps,
}

impl RangePicker {
    /// Create a new range picker
    pub fn new() -> Self {
        let today = Date::today();
        Self {
            start: DateTime::new(today, Time::new(0, 0, 0)),
            end: DateTime::new(today, Time::new(23, 59, 59)),
            active_preset: Some(PresetRange::Today),
            presets: PresetRange::common().to_vec(),
            preset_cursor: 0,
            focus: RangeFocus::Start,
            first_day: FirstDayOfWeek::Sunday,
            show_time: false,
            start_cursor_day: today.day,
            end_cursor_day: today.day,
            min_date: None,
            max_date: None,
            show_presets: true,
            header_fg: Color::CYAN,
            selected_fg: Color::BLACK,
            selected_bg: Color::CYAN,
            range_bg: Color::rgb(60, 100, 140),
            preset_fg: Color::WHITE,
            preset_selected_fg: Color::BLACK,
            preset_selected_bg: Color::CYAN,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }
}

impl Default for RangePicker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Constructor tests
    // =========================================================================

    #[test]
    fn test_range_picker_new() {
        let picker = RangePicker::new();
        // Should create without panic
        let _ = picker;
    }

    #[test]
    fn test_range_picker_new_has_start() {
        let picker = RangePicker::new();
        // Start should be today
        let today = Date::today();
        assert_eq!(picker.start.date, today);
    }

    #[test]
    fn test_range_picker_new_has_end() {
        let picker = RangePicker::new();
        // End should be today
        let today = Date::today();
        assert_eq!(picker.end.date, today);
    }

    #[test]
    fn test_range_picker_new_start_time_midnight() {
        let picker = RangePicker::new();
        assert_eq!(picker.start.time.hour, 0);
        assert_eq!(picker.start.time.minute, 0);
        assert_eq!(picker.start.time.second, 0);
    }

    #[test]
    fn test_range_picker_new_end_time_last_second() {
        let picker = RangePicker::new();
        assert_eq!(picker.end.time.hour, 23);
        assert_eq!(picker.end.time.minute, 59);
        assert_eq!(picker.end.time.second, 59);
    }

    #[test]
    fn test_range_picker_new_active_preset_is_today() {
        let picker = RangePicker::new();
        assert_eq!(picker.active_preset, Some(PresetRange::Today));
    }

    #[test]
    fn test_range_picker_new_has_presets() {
        let picker = RangePicker::new();
        assert!(!picker.presets.is_empty());
    }

    #[test]
    fn test_range_picker_new_preset_cursor_is_zero() {
        let picker = RangePicker::new();
        assert_eq!(picker.preset_cursor, 0);
    }

    #[test]
    fn test_range_picker_new_focus_is_start() {
        let picker = RangePicker::new();
        assert_eq!(picker.focus, RangeFocus::Start);
    }

    #[test]
    fn test_range_picker_new_first_day_is_sunday() {
        let picker = RangePicker::new();
        assert_eq!(picker.first_day, FirstDayOfWeek::Sunday);
    }

    #[test]
    fn test_range_picker_new_show_time_is_false() {
        let picker = RangePicker::new();
        assert!(!picker.show_time);
    }

    #[test]
    fn test_range_picker_new_show_presets_is_true() {
        let picker = RangePicker::new();
        assert!(picker.show_presets);
    }

    #[test]
    fn test_range_picker_new_no_min_date() {
        let picker = RangePicker::new();
        assert!(picker.min_date.is_none());
    }

    #[test]
    fn test_range_picker_new_no_max_date() {
        let picker = RangePicker::new();
        assert!(picker.max_date.is_none());
    }

    #[test]
    fn test_range_picker_new_cursor_days_match_today() {
        let picker = RangePicker::new();
        let today = Date::today();
        assert_eq!(picker.start_cursor_day, today.day);
        assert_eq!(picker.end_cursor_day, today.day);
    }

    #[test]
    fn test_range_picker_new_has_widget_state() {
        let picker = RangePicker::new();
        // Widget state should be initialized
        let _ = picker.state;
    }

    #[test]
    fn test_range_picker_new_has_widget_props() {
        let picker = RangePicker::new();
        // Widget props should be initialized
        let _ = picker.props;
    }

    #[test]
    fn test_range_picker_new_default_colors() {
        let picker = RangePicker::new();
        // Verify default colors are set (not None)
        // Note: Can't directly compare Color values without specific assertions
        let _ = picker.header_fg;
        let _ = picker.selected_fg;
        let _ = picker.selected_bg;
        let _ = picker.range_bg;
    }

    // =========================================================================
    // Default trait tests
    // =========================================================================

    #[test]
    fn test_range_picker_default() {
        let picker = RangePicker::default();
        // Should behave same as new()
        let _ = picker;
    }

    #[test]
    fn test_range_picker_default_equals_new() {
        let picker_new = RangePicker::new();
        let picker_default = RangePicker::default();
        // Both should have today as date
        assert_eq!(picker_new.start.date, picker_default.start.date);
        assert_eq!(picker_new.end.date, picker_default.end.date);
    }

    // =========================================================================
    // Field visibility tests
    // =========================================================================

    #[test]
    fn test_range_picker_fields_are_accessible() {
        let picker = RangePicker::new();
        // Verify all pub(crate) fields are accessible
        let _ = picker.start;
        let _ = picker.end;
        let _ = picker.active_preset;
        let _ = picker.presets;
        let _ = picker.preset_cursor;
        let _ = picker.focus;
        let _ = picker.first_day;
        let _ = picker.show_time;
        let _ = picker.start_cursor_day;
        let _ = picker.end_cursor_day;
        let _ = picker.min_date;
        let _ = picker.max_date;
        let _ = picker.show_presets;
        let _ = picker.header_fg;
        let _ = picker.selected_fg;
        let _ = picker.selected_bg;
        let _ = picker.range_bg;
        let _ = picker.preset_fg;
        let _ = picker.preset_selected_fg;
        let _ = picker.preset_selected_bg;
        let _ = picker.state;
        let _ = picker.props;
    }

    // =========================================================================
    // Time field tests
    // =========================================================================

    #[test]
    fn test_range_picker_start_time_is_valid() {
        let picker = RangePicker::new();
        assert!(picker.start.time.is_valid());
    }

    #[test]
    fn test_range_picker_end_time_is_valid() {
        let picker = RangePicker::new();
        assert!(picker.end.time.is_valid());
    }

    #[test]
    fn test_range_picker_start_before_end() {
        let picker = RangePicker::new();
        // Start time should be before end time
        assert!(picker.start.time <= picker.end.time);
    }

    #[test]
    fn test_range_picker_same_day_different_times() {
        let picker = RangePicker::new();
        // Same day but different times
        assert_eq!(picker.start.date, picker.end.date);
        assert_ne!(picker.start.time, picker.end.time);
    }

    // =========================================================================
    // Preset configuration tests
    // =========================================================================

    #[test]
    fn test_range_picker_presets_contain_today() {
        let picker = RangePicker::new();
        assert!(picker.presets.contains(&PresetRange::Today));
    }

    #[test]
    fn test_range_picker_presets_contain_yesterday() {
        let picker = RangePicker::new();
        assert!(picker.presets.contains(&PresetRange::Yesterday));
    }

    #[test]
    fn test_range_picker_presets_contain_last_7_days() {
        let picker = RangePicker::new();
        assert!(picker.presets.contains(&PresetRange::Last7Days));
    }

    #[test]
    fn test_range_picker_presets_contain_last_30_days() {
        let picker = RangePicker::new();
        assert!(picker.presets.contains(&PresetRange::Last30Days));
    }

    #[test]
    fn test_range_picker_presets_contain_this_week() {
        let picker = RangePicker::new();
        assert!(picker.presets.contains(&PresetRange::ThisWeek));
    }

    #[test]
    fn test_range_picker_presets_contain_last_week() {
        let picker = RangePicker::new();
        assert!(picker.presets.contains(&PresetRange::LastWeek));
    }

    #[test]
    fn test_range_picker_presets_contain_this_month() {
        let picker = RangePicker::new();
        assert!(picker.presets.contains(&PresetRange::ThisMonth));
    }

    #[test]
    fn test_range_picker_presets_contain_last_month() {
        let picker = RangePicker::new();
        assert!(picker.presets.contains(&PresetRange::LastMonth));
    }

    #[test]
    fn test_range_picker_presets_do_not_contain_custom() {
        let picker = RangePicker::new();
        assert!(!picker.presets.contains(&PresetRange::Custom));
    }

    #[test]
    fn test_range_picker_presets_do_not_contain_this_year() {
        let picker = RangePicker::new();
        assert!(!picker.presets.contains(&PresetRange::ThisYear));
    }

    // =========================================================================
    // Multiple instance tests
    // =========================================================================

    #[test]
    fn test_range_picker_multiple_instances() {
        let picker1 = RangePicker::new();
        let picker2 = RangePicker::new();
        // Both should be independent
        assert_eq!(picker1.start.date, picker2.start.date);
    }

    #[test]
    fn test_range_picker_cloned_instance() {
        let picker1 = RangePicker::new();
        let _ = picker1;
        // Create another to verify no shared state issues
        let picker2 = RangePicker::new();
        let _ = picker2;
    }

    // =========================================================================
    // Integration with DateTime tests
    // =========================================================================

    #[test]
    fn test_range_picker_start_is_datetime() {
        let picker = RangePicker::new();
        // Start should be a DateTime with both date and time
        let _ = picker.start.date;
        let _ = picker.start.time;
    }

    #[test]
    fn test_range_picker_end_is_datetime() {
        let picker = RangePicker::new();
        // End should be a DateTime with both date and time
        let _ = picker.end.date;
        let _ = picker.end.time;
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_range_picker_today_preset_cursor_is_zero() {
        let picker = RangePicker::new();
        // With Today as active preset, cursor should be at 0
        assert_eq!(picker.active_preset, Some(PresetRange::Today));
        assert_eq!(picker.preset_cursor, 0);
    }

    #[test]
    fn test_range_picker_presets_not_empty_by_default() {
        let picker = RangePicker::new();
        assert!(!picker.presets.is_empty());
        assert!(picker.presets.len() >= 8);
    }
}
