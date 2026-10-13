//! ISO 8601 week numbers

use super::days_in_month;
use super::render::CalendarRender;

impl<'a> CalendarRender<'a> {
    /// Get ISO 8601 week number
    ///
    /// ISO week rules:
    /// - Weeks start on Monday
    /// - Week 1 contains the first Thursday of the year
    /// - Week numbers range from 1 to 52 or 53
    pub fn get_week_number(&self, year: i32, month: u32, day: u32) -> u32 {
        // Calculate day of year (1-based)
        let day_of_year = (1..month).map(|m| days_in_month(year, m)).sum::<u32>() + day;

        // Calculate weekday (0=Monday, 6=Sunday) using Zeller's congruence
        let weekday = {
            let m = if month < 3 {
                month as i32 + 12
            } else {
                month as i32
            };
            let y = if month < 3 { year - 1 } else { year };
            let k = y % 100;
            let j = y / 100;
            let h = (day as i32 + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
            // Convert from Zeller (0=Sat) to ISO (0=Mon)
            ((h + 5) % 7) as u32
        };

        // Calculate ISO week number
        // Thursday of the same week determines the year for ISO week
        let thursday_day_of_year = day_of_year as i32 + 3 - weekday as i32;

        if thursday_day_of_year < 1 {
            // This day belongs to the last week of the previous year
            return self.get_week_number(year - 1, 12, 31);
        }

        let days_in_year: i32 = if super::is_leap_year(year) { 366 } else { 365 };
        if thursday_day_of_year > days_in_year {
            // This day belongs to week 1 of the next year
            return 1;
        }

        // Calculate week number
        ((thursday_day_of_year as u32 - 1) / 7) + 1
    }
}
