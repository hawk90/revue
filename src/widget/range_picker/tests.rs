//! Range picker tests

#[cfg(test)]
mod tests {
    use super::super::{PresetRange, RangePicker};
    use crate::event::Key;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::calendar::Date;
    use crate::widget::traits::RenderContext;

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
    fn test_range_picker_focus_navigation() {
        let mut picker = RangePicker::new();

        assert_eq!(picker.get_focus(), super::super::RangeFocus::Start);

        picker.handle_key(&Key::Tab);
        assert_eq!(picker.get_focus(), super::super::RangeFocus::End);

        picker.handle_key(&Key::Tab);
        assert_eq!(picker.get_focus(), super::super::RangeFocus::Presets);

        picker.handle_key(&Key::Tab);
        assert_eq!(picker.get_focus(), super::super::RangeFocus::Start);

        picker.handle_key(&Key::BackTab);
        assert_eq!(picker.get_focus(), super::super::RangeFocus::Presets);
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

    #[test]
    fn test_range_picker_disabled() {
        let mut picker = RangePicker::new().disabled(true);

        let handled = picker.handle_key(&Key::Right);
        assert!(!handled);
    }
}
