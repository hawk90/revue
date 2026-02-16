//! Calendar utility functions

/// Days in a month (accounting for leap years)
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Check if year is a leap year
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Day of week for the first day of a month (0 = Sunday, 6 = Saturday)
/// Using Zeller's congruence
pub fn first_day_of_month(year: i32, month: u32) -> u32 {
    let m = if month < 3 {
        month as i32 + 12
    } else {
        month as i32
    };
    let y = if month < 3 { year - 1 } else { year };
    let q = 1i32; // First day of month
    let k = y % 100;
    let j = y / 100;

    let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
    // Convert from Zeller (0 = Saturday) to standard (0 = Sunday)
    // Handle negative modulo
    let h = ((h + 6) % 7 + 7) % 7;
    h as u32
}
