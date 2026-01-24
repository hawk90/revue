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
    use crate::widget::calendar::Date;

    #[test]
    fn test_range_picker_new() {
        let picker = RangePicker::new();
        let (start, end) = picker.get_range();
        assert_eq!(start, end); // Default is today-today
    }

    #[test]
    fn test_range_picker_set_range() {
        let picker = RangePicker::new()
            .start_date(Date::new(2025, 1, 1))
            .end_date(Date::new(2025, 1, 31));

        let (start, end) = picker.get_range();
        assert_eq!(start, Date::new(2025, 1, 1));
        assert_eq!(end, Date::new(2025, 1, 31));
    }

    #[test]
    fn test_range_picker_swap() {
        let picker = RangePicker::new()
            .start_date(Date::new(2025, 12, 31))
            .end_date(Date::new(2025, 1, 1));

        let (start, end) = picker.get_range();
        assert!(start <= end);
    }

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
    fn test_range_picker_preset_apply() {
        let mut picker = RangePicker::new();
        let today = Date::today();

        picker.apply_preset(PresetRange::Today);
        let (start, end) = picker.get_range();
        assert_eq!(start, today);
        assert_eq!(end, today);
    }

    #[test]
    fn test_range_picker_preset_last7days() {
        let today = Date::today();
        let (start, end) = PresetRange::Last7Days.calculate(today);

        assert_eq!(end, today);
        // Start should be 6 days before today
        let expected_start = today.subtract_days(6);
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
    fn test_preset_names() {
        assert_eq!(PresetRange::Today.name(), "Today");
        assert_eq!(PresetRange::Last7Days.name(), "Last 7 Days");
        assert_eq!(PresetRange::Custom.name(), "Custom");
    }

    #[test]
    fn test_preset_common() {
        let common = PresetRange::common();
        assert!(common.contains(&PresetRange::Today));
        assert!(common.contains(&PresetRange::Last7Days));
        assert!(!common.contains(&PresetRange::Custom));
    }
}
