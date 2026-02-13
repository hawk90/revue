//! Date/time range picker types tests

use revue::widget::data::calendar::{Date, days_in_month};
use revue::widget::range_picker::{PresetRange, RangeFocus};

// =========================================================================
// PresetRange::name() tests
// =========================================================================

#[test]
fn test_preset_range_name_today() {
    assert_eq!(PresetRange::Today.name(), "Today");
}

#[test]
fn test_preset_range_name_yesterday() {
    assert_eq!(PresetRange::Yesterday.name(), "Yesterday");
}

#[test]
fn test_preset_range_name_last_7_days() {
    assert_eq!(PresetRange::Last7Days.name(), "Last 7 Days");
}

#[test]
fn test_preset_range_name_last_30_days() {
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

// =========================================================================
// PresetRange::common() tests
// =========================================================================

#[test]
fn test_preset_range_common_not_empty() {
    let common = PresetRange::common();
    assert!(!common.is_empty());
}

#[test]
fn test_preset_range_common_contains_today() {
    let common = PresetRange::common();
    assert!(common.contains(&PresetRange::Today));
}

#[test]
fn test_preset_range_common_contains_yesterday() {
    let common = PresetRange::common();
    assert!(common.contains(&PresetRange::Yesterday));
}

#[test]
fn test_preset_range_common_contains_last_7_days() {
    let common = PresetRange::common();
    assert!(common.contains(&PresetRange::Last7Days));
}

#[test]
fn test_preset_range_common_contains_last_30_days() {
    let common = PresetRange::common();
    assert!(common.contains(&PresetRange::Last30Days));
}

#[test]
fn test_preset_range_common_contains_this_week() {
    let common = PresetRange::common();
    assert!(common.contains(&PresetRange::ThisWeek));
}

#[test]
fn test_preset_range_common_contains_last_week() {
    let common = PresetRange::common();
    assert!(common.contains(&PresetRange::LastWeek));
}

#[test]
fn test_preset_range_common_contains_this_month() {
    let common = PresetRange::common();
    assert!(common.contains(&PresetRange::ThisMonth));
}

#[test]
fn test_preset_range_common_contains_last_month() {
    let common = PresetRange::common();
    assert!(common.contains(&PresetRange::LastMonth));
}

#[test]
fn test_preset_range_common_does_not_contain_custom() {
    let common = PresetRange::common();
    assert!(!common.contains(&PresetRange::Custom));
}

#[test]
fn test_preset_range_common_does_not_contain_this_year() {
    let common = PresetRange::common();
    assert!(!common.contains(&PresetRange::ThisYear));
}

#[test]
fn test_preset_range_common_length() {
    let common = PresetRange::common();
    assert_eq!(common.len(), 8);
}

// =========================================================================
// PresetRange::calculate() tests - Today
// =========================================================================

#[test]
fn test_preset_range_calculate_today() {
    let today = Date::new(2024, 6, 15);
    let (start, end) = PresetRange::Today.calculate(today);
    assert_eq!(start, today);
    assert_eq!(end, today);
}

#[test]
fn test_preset_range_calculate_today_first_of_month() {
    let today = Date::new(2024, 6, 1);
    let (start, end) = PresetRange::Today.calculate(today);
    assert_eq!(start, today);
    assert_eq!(end, today);
}

#[test]
fn test_preset_range_calculate_today_last_of_month() {
    let today = Date::new(2024, 6, 30);
    let (start, end) = PresetRange::Today.calculate(today);
    assert_eq!(start, today);
    assert_eq!(end, today);
}

// =========================================================================
// PresetRange::calculate() tests - Yesterday
// =========================================================================

#[test]
fn test_preset_range_calculate_yesterday_mid_month() {
    let today = Date::new(2024, 6, 15);
    let (start, end) = PresetRange::Yesterday.calculate(today);
    assert_eq!(start, Date::new(2024, 6, 14));
    assert_eq!(end, Date::new(2024, 6, 14));
}

#[test]
fn test_preset_range_calculate_yesterday_first_of_month() {
    let today = Date::new(2024, 6, 1);
    let (start, end) = PresetRange::Yesterday.calculate(today);
    assert_eq!(start, Date::new(2024, 5, 31));
    assert_eq!(end, Date::new(2024, 5, 31));
}

#[test]
fn test_preset_range_calculate_yesterday_year_boundary() {
    let today = Date::new(2024, 1, 1);
    let (start, end) = PresetRange::Yesterday.calculate(today);
    assert_eq!(start, Date::new(2023, 12, 31));
    assert_eq!(end, Date::new(2023, 12, 31));
}

// =========================================================================
// PresetRange::calculate() tests - Last7Days
// =========================================================================

#[test]
fn test_preset_range_calculate_last_7_days() {
    let today = Date::new(2024, 6, 15);
    let (start, end) = PresetRange::Last7Days.calculate(today);
    assert_eq!(end, today);
    assert_eq!(start, Date::new(2024, 6, 9)); // 15 - 6 = 9
}

#[test]
fn test_preset_range_calculate_last_7_days_year_boundary() {
    let today = Date::new(2024, 1, 5);
    let (start, end) = PresetRange::Last7Days.calculate(today);
    assert_eq!(end, today);
    assert_eq!(start, Date::new(2023, 12, 30)); // Dec 30
}

#[test]
fn test_preset_range_calculate_last_7_days_month_boundary() {
    let today = Date::new(2024, 3, 5);
    let (start, end) = PresetRange::Last7Days.calculate(today);
    assert_eq!(end, today);
    assert_eq!(start, Date::new(2024, 2, 28)); // Feb 28 (2024 is leap year)
}

#[test]
fn test_preset_range_calculate_last_7_days_february_leap_year() {
    let today = Date::new(2024, 3, 1);
    let (start, end) = PresetRange::Last7Days.calculate(today);
    assert_eq!(end, today);
    assert_eq!(start, Date::new(2024, 2, 24)); // Feb has 29 days in 2024
}

// =========================================================================
// PresetRange::calculate() tests - Last30Days
// =========================================================================

#[test]
fn test_preset_range_calculate_last_30_days() {
    let today = Date::new(2024, 6, 15);
    let (start, end) = PresetRange::Last30Days.calculate(today);
    assert_eq!(end, today);
    assert_eq!(start, Date::new(2024, 5, 17)); // 15 - 29 = May 17
}

#[test]
fn test_preset_range_calculate_last_30_days_year_boundary() {
    let today = Date::new(2024, 1, 15);
    let (start, end) = PresetRange::Last30Days.calculate(today);
    assert_eq!(end, today);
    assert_eq!(start, Date::new(2023, 12, 17));
}

#[test]
fn test_preset_range_calculate_last_30_days_february_leap_year() {
    let today = Date::new(2024, 3, 15);
    let (start, end) = PresetRange::Last30Days.calculate(today);
    assert_eq!(end, today);
    // Feb 2024 has 29 days, so start should be Feb 15
    assert_eq!(start, Date::new(2024, 2, 15));
}

// =========================================================================
// PresetRange::calculate() tests - ThisWeek
// =========================================================================

#[test]
fn test_preset_range_calculate_this_week_sunday() {
    let today = Date::new(2024, 6, 12); // Wednesday
    let (start, end) = PresetRange::ThisWeek.calculate(today);
    assert_eq!(end, today);
    // Sunday June 9 (weekday 0)
    assert_eq!(start, Date::new(2024, 6, 9));
}

#[test]
fn test_preset_range_calculate_this_week_sunday_itself() {
    let today = Date::new(2024, 6, 9); // Sunday
    let (start, end) = PresetRange::ThisWeek.calculate(today);
    assert_eq!(end, today);
    assert_eq!(start, today);
}

#[test]
fn test_preset_range_calculate_this_week_saturday() {
    let today = Date::new(2024, 6, 15); // Saturday
    let (start, end) = PresetRange::ThisWeek.calculate(today);
    assert_eq!(end, today);
    // Sunday June 9
    assert_eq!(start, Date::new(2024, 6, 9));
}

// =========================================================================
// PresetRange::calculate() tests - LastWeek
// =========================================================================

#[test]
fn test_preset_range_calculate_last_week() {
    let today = Date::new(2024, 6, 12); // Wednesday
    let (start, end) = PresetRange::LastWeek.calculate(today);
    // This week starts June 9 (Sunday)
    // Last week ended June 8 (Saturday)
    // Last week started June 2 (Sunday)
    assert_eq!(start, Date::new(2024, 6, 2));
    assert_eq!(end, Date::new(2024, 6, 8));
}

#[test]
fn test_preset_range_calculate_last_week_year_boundary() {
    let today = Date::new(2024, 1, 10); // Wednesday
    let (start, end) = PresetRange::LastWeek.calculate(today);
    // Last week was Dec 31 - Jan 6
    assert_eq!(start.year, 2023);
    assert_eq!(start.month, 12);
    assert_eq!(end, Date::new(2024, 1, 6));
}

#[test]
fn test_preset_range_calculate_last_week_month_boundary() {
    let today = Date::new(2024, 3, 5); // Tuesday
    let (start, end) = PresetRange::LastWeek.calculate(today);
    // Last week was Feb 25 - Mar 2
    assert_eq!(start, Date::new(2024, 2, 25));
    assert_eq!(end, Date::new(2024, 3, 2));
}

// =========================================================================
// PresetRange::calculate() tests - ThisMonth
// =========================================================================

#[test]
fn test_preset_range_calculate_this_month() {
    let today = Date::new(2024, 6, 15);
    let (start, end) = PresetRange::ThisMonth.calculate(today);
    assert_eq!(start, Date::new(2024, 6, 1));
    assert_eq!(end, today);
}

#[test]
fn test_preset_range_calculate_this_month_first_day() {
    let today = Date::new(2024, 6, 1);
    let (start, end) = PresetRange::ThisMonth.calculate(today);
    assert_eq!(start, today);
    assert_eq!(end, today);
}

#[test]
fn test_preset_range_calculate_this_month_last_day() {
    let today = Date::new(2024, 6, 30);
    let (start, end) = PresetRange::ThisMonth.calculate(today);
    assert_eq!(start, Date::new(2024, 6, 1));
    assert_eq!(end, today);
}

#[test]
fn test_preset_range_calculate_this_month_february_leap_year() {
    let today = Date::new(2024, 2, 15);
    let (start, end) = PresetRange::ThisMonth.calculate(today);
    assert_eq!(start, Date::new(2024, 2, 1));
    assert_eq!(end, today);
}

#[test]
fn test_preset_range_calculate_this_month_february_non_leap_year() {
    let today = Date::new(2023, 2, 15);
    let (start, end) = PresetRange::ThisMonth.calculate(today);
    assert_eq!(start, Date::new(2023, 2, 1));
    assert_eq!(end, today);
}

// =========================================================================
// PresetRange::calculate() tests - LastMonth
// =========================================================================

#[test]
fn test_preset_range_calculate_last_month_mid_year() {
    let today = Date::new(2024, 6, 15);
    let (start, end) = PresetRange::LastMonth.calculate(today);
    assert_eq!(start, Date::new(2024, 5, 1));
    assert_eq!(end, Date::new(2024, 5, 31));
}

#[test]
fn test_preset_range_calculate_last_month_january() {
    let today = Date::new(2024, 1, 15);
    let (start, end) = PresetRange::LastMonth.calculate(today);
    assert_eq!(start, Date::new(2023, 12, 1));
    assert_eq!(end, Date::new(2023, 12, 31));
}

#[test]
fn test_preset_range_calculate_last_month_february_leap_year() {
    let today = Date::new(2024, 3, 15);
    let (start, end) = PresetRange::LastMonth.calculate(today);
    // February 2024 has 29 days
    assert_eq!(start, Date::new(2024, 2, 1));
    assert_eq!(end, Date::new(2024, 2, 29));
}

#[test]
fn test_preset_range_calculate_last_month_february_non_leap_year() {
    let today = Date::new(2023, 3, 15);
    let (start, end) = PresetRange::LastMonth.calculate(today);
    // February 2023 has 28 days
    assert_eq!(start, Date::new(2023, 2, 1));
    assert_eq!(end, Date::new(2023, 2, 28));
}

#[test]
fn test_preset_range_calculate_last_month_april() {
    let today = Date::new(2024, 5, 15);
    let (start, end) = PresetRange::LastMonth.calculate(today);
    // April has 30 days
    assert_eq!(start, Date::new(2024, 4, 1));
    assert_eq!(end, Date::new(2024, 4, 30));
}

// =========================================================================
// PresetRange::calculate() tests - ThisYear
// =========================================================================

#[test]
fn test_preset_range_calculate_this_year() {
    let today = Date::new(2024, 6, 15);
    let (start, end) = PresetRange::ThisYear.calculate(today);
    assert_eq!(start, Date::new(2024, 1, 1));
    assert_eq!(end, today);
}

#[test]
fn test_preset_range_calculate_this_year_january_first() {
    let today = Date::new(2024, 1, 1);
    let (start, end) = PresetRange::ThisYear.calculate(today);
    assert_eq!(start, today);
    assert_eq!(end, today);
}

#[test]
fn test_preset_range_calculate_this_year_december_last() {
    let today = Date::new(2024, 12, 31);
    let (start, end) = PresetRange::ThisYear.calculate(today);
    assert_eq!(start, Date::new(2024, 1, 1));
    assert_eq!(end, today);
}

// =========================================================================
// PresetRange::calculate() tests - Custom
// =========================================================================

#[test]
fn test_preset_range_calculate_custom() {
    let today = Date::new(2024, 6, 15);
    let (start, end) = PresetRange::Custom.calculate(today);
    // Custom should return today-today
    assert_eq!(start, today);
    assert_eq!(end, today);
}

// =========================================================================
// RangeFocus enum tests
// =========================================================================

#[test]
fn test_range_focus_default() {
    assert_eq!(RangeFocus::default(), RangeFocus::Start);
}

#[test]
fn test_range_focus_partial_eq() {
    assert_eq!(RangeFocus::Start, RangeFocus::Start);
    assert_eq!(RangeFocus::End, RangeFocus::End);
    assert_eq!(RangeFocus::Presets, RangeFocus::Presets);
}

#[test]
fn test_range_focus_ne() {
    assert_ne!(RangeFocus::Start, RangeFocus::End);
    assert_ne!(RangeFocus::End, RangeFocus::Presets);
    assert_ne!(RangeFocus::Start, RangeFocus::Presets);
}

#[test]
fn test_range_focus_debug() {
    let start_str = format!("{:?}", RangeFocus::Start);
    let end_str = format!("{:?}", RangeFocus::End);
    let presets_str = format!("{:?}", RangeFocus::Presets);
    assert!(start_str.contains("Start"));
    assert!(end_str.contains("End"));
    assert!(presets_str.contains("Presets"));
}

#[test]
fn test_range_focus_clone() {
    let focus = RangeFocus::Start;
    let cloned = focus.clone();
    assert_eq!(focus, cloned);
}

#[test]
fn test_range_focus_copy() {
    let focus = RangeFocus::End;
    let copied = focus;
    assert_eq!(focus, copied);
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

#[test]
fn test_preset_range_debug() {
    let today_str = format!("{:?}", PresetRange::Today);
    let custom_str = format!("{:?}", PresetRange::Custom);
    assert!(today_str.contains("Today"));
    assert!(custom_str.contains("Custom"));
}

#[test]
fn test_preset_range_partial_eq() {
    assert_eq!(PresetRange::Today, PresetRange::Today);
    assert_eq!(PresetRange::Custom, PresetRange::Custom);
}

#[test]
fn test_preset_range_ne() {
    assert_ne!(PresetRange::Today, PresetRange::Yesterday);
    assert_ne!(PresetRange::Last7Days, PresetRange::Last30Days);
}

// =========================================================================
// Calculate properties
// =========================================================================

#[test]
fn test_calculate_start_always_le_end() {
    let today = Date::new(2024, 6, 15);
    let presets = [
        PresetRange::Today,
        PresetRange::Yesterday,
        PresetRange::Last7Days,
        PresetRange::Last30Days,
        PresetRange::ThisWeek,
        PresetRange::LastWeek,
        PresetRange::ThisMonth,
        PresetRange::LastMonth,
        PresetRange::ThisYear,
    ];

    for preset in presets {
        let (start, end) = preset.calculate(today);
        assert!(
            start <= end,
            "Preset {:?} failed: start {:?} > end {:?}",
            preset,
            start,
            end
        );
    }
}

#[test]
fn test_calculate_today_is_in_range() {
    let today = Date::new(2024, 6, 15);
    let presets = [
        PresetRange::Today,
        PresetRange::Last7Days,
        PresetRange::Last30Days,
        PresetRange::ThisWeek,
        PresetRange::ThisMonth,
        PresetRange::ThisYear,
    ];

    for preset in presets {
        let (start, end) = preset.calculate(today);
        assert!(
            today >= start && today <= end,
            "Today {:?} not in range for preset {:?}: {:?} to {:?}",
            today,
            preset,
            start,
            end
        );
    }
}

#[test]
fn test_calculate_yesterday_not_in_range() {
    let today = Date::new(2024, 6, 15);
    let yesterday = Date::new(2024, 6, 14);
    let (start, end) = PresetRange::Today.calculate(today);
    assert!(yesterday < start || yesterday > end);
}

#[test]
fn test_calculate_last_week_excludes_this_week() {
    let today = Date::new(2024, 6, 12); // Wednesday
    let (_start, end) = PresetRange::LastWeek.calculate(today);
    // This week is June 9-15
    // Last week should not include June 9 or later
    assert!(end < Date::new(2024, 6, 9));
}