//! DateTime picker helper functions

/// Get month name
pub fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

/// Create a new datetime picker
pub fn datetime_picker() -> super::DateTimePicker {
    super::DateTimePicker::new()
}

/// Create a date-only picker
pub fn date_picker() -> super::DateTimePicker {
    super::DateTimePicker::date_only()
}

/// Create a time-only picker
pub fn time_picker() -> super::DateTimePicker {
    super::DateTimePicker::time_only()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_month_name_all_months() {
        assert_eq!(month_name(1), "January");
        assert_eq!(month_name(2), "February");
        assert_eq!(month_name(3), "March");
        assert_eq!(month_name(4), "April");
        assert_eq!(month_name(5), "May");
        assert_eq!(month_name(6), "June");
        assert_eq!(month_name(7), "July");
        assert_eq!(month_name(8), "August");
        assert_eq!(month_name(9), "September");
        assert_eq!(month_name(10), "October");
        assert_eq!(month_name(11), "November");
        assert_eq!(month_name(12), "December");
    }

    #[test]
    fn test_month_name_invalid() {
        assert_eq!(month_name(0), "Unknown");
        assert_eq!(month_name(13), "Unknown");
        assert_eq!(month_name(100), "Unknown");
    }

    #[test]
    fn test_datetime_picker_function() {
        let picker = datetime_picker();
        let _ = picker;
    }

    #[test]
    fn test_date_picker_function() {
        let picker = date_picker();
        let _ = picker;
    }

    #[test]
    fn test_time_picker_function() {
        let picker = time_picker();
        let _ = picker;
    }
}
