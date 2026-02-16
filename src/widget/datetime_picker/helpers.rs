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
