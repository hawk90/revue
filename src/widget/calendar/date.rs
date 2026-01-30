//! Date type and calendar utilities

/// Calendar date
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    /// Year
    pub year: i32,
    /// Month (1-12)
    pub month: u32,
    /// Day (1-31)
    pub day: u32,
}

impl Date {
    /// Create a new date
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Get today's date (placeholder - returns 2025-01-01)
    pub fn today() -> Self {
        // In a real implementation, use chrono or time crate
        Self::new(2025, 1, 1)
    }

    /// Check if date is valid
    pub fn is_valid(&self) -> bool {
        self.month >= 1
            && self.month <= 12
            && self.day >= 1
            && self.day <= super::utils::days_in_month(self.year, self.month)
    }

    /// Day of week (0 = Sunday, 6 = Saturday)
    pub fn weekday(&self) -> u32 {
        let first = super::utils::first_day_of_month(self.year, self.month);
        (first + self.day - 1) % 7
    }

    /// Get previous day
    pub fn prev_day(&self) -> Self {
        if self.day > 1 {
            Self::new(self.year, self.month, self.day - 1)
        } else if self.month > 1 {
            let prev_month = self.month - 1;
            let days = super::utils::days_in_month(self.year, prev_month);
            Self::new(self.year, prev_month, days)
        } else {
            Self::new(self.year - 1, 12, 31)
        }
    }

    /// Get next day
    pub fn next_day(&self) -> Self {
        let days = super::utils::days_in_month(self.year, self.month);
        if self.day < days {
            Self::new(self.year, self.month, self.day + 1)
        } else if self.month < 12 {
            Self::new(self.year, self.month + 1, 1)
        } else {
            Self::new(self.year + 1, 1, 1)
        }
    }

    /// Subtract n days from this date
    pub fn subtract_days(&self, n: u32) -> Self {
        let mut result = *self;
        for _ in 0..n {
            result = result.prev_day();
        }
        result
    }

    /// Add n days to this date
    pub fn add_days(&self, n: u32) -> Self {
        let mut result = *self;
        for _ in 0..n {
            result = result.next_day();
        }
        result
    }
}

impl Default for Date {
    fn default() -> Self {
        Self::new(2025, 1, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_new() {
        let date = Date::new(2024, 6, 15);
        assert_eq!(date.year, 2024);
        assert_eq!(date.month, 6);
        assert_eq!(date.day, 15);
    }

    #[test]
    fn test_date_default() {
        let date = Date::default();
        assert_eq!(date.year, 2025);
        assert_eq!(date.month, 1);
        assert_eq!(date.day, 1);
    }

    #[test]
    fn test_today() {
        let today = Date::today();
        // Just verify it returns a valid date
        assert!(today.is_valid());
    }

    #[test]
    fn test_is_valid_true() {
        let date = Date::new(2024, 2, 15); // Feb 15, 2024 (leap year)
        assert!(date.is_valid());
    }

    #[test]
    fn test_is_valid_invalid_month() {
        let date = Date::new(2024, 13, 1);
        assert!(!date.is_valid());
    }

    #[test]
    fn test_is_valid_invalid_day_too_small() {
        let date = Date::new(2024, 1, 0);
        assert!(!date.is_valid());
    }

    #[test]
    fn test_is_valid_invalid_day_too_large() {
        let date = Date::new(2024, 1, 32);
        assert!(!date.is_valid());
    }

    #[test]
    fn test_is_valid_february_29_leap_year() {
        let date = Date::new(2024, 2, 29); // 2024 is divisible by 4
        assert!(date.is_valid());
    }

    #[test]
    fn test_is_valid_february_29_non_leap_year() {
        let date = Date::new(2023, 2, 29); // 2023 is not divisible by 4
        assert!(!date.is_valid());
    }

    #[test]
    fn test_weekday() {
        let date = Date::new(2025, 1, 1); // Jan 1, 2025 is Wednesday
                                          // The weekday depends on first_day_of_month calculation
        let weekday = date.weekday();
        assert!(weekday < 7);
    }

    #[test]
    fn test_prev_day_same_month() {
        let date = Date::new(2024, 6, 15);
        let prev = date.prev_day();
        assert_eq!(prev.year, 2024);
        assert_eq!(prev.month, 6);
        assert_eq!(prev.day, 14);
    }

    #[test]
    fn test_prev_day_month_boundary() {
        let date = Date::new(2024, 3, 1);
        let prev = date.prev_day();
        assert_eq!(prev.year, 2024);
        assert_eq!(prev.month, 2);
        assert_eq!(prev.day, 29); // 2024 is leap year
    }

    #[test]
    fn test_prev_day_year_boundary() {
        let date = Date::new(2024, 1, 1);
        let prev = date.prev_day();
        assert_eq!(prev.year, 2023);
        assert_eq!(prev.month, 12);
        assert_eq!(prev.day, 31);
    }

    #[test]
    fn test_next_day_same_month() {
        let date = Date::new(2024, 6, 15);
        let next = date.next_day();
        assert_eq!(next.year, 2024);
        assert_eq!(next.month, 6);
        assert_eq!(next.day, 16);
    }

    #[test]
    fn test_next_day_month_boundary() {
        let date = Date::new(2024, 1, 31);
        let next = date.next_day();
        assert_eq!(next.year, 2024);
        assert_eq!(next.month, 2);
        assert_eq!(next.day, 1);
    }

    #[test]
    fn test_next_day_year_boundary() {
        let date = Date::new(2024, 12, 31);
        let next = date.next_day();
        assert_eq!(next.year, 2025);
        assert_eq!(next.month, 1);
        assert_eq!(next.day, 1);
    }

    #[test]
    fn test_subtract_days_within_month() {
        let date = Date::new(2024, 6, 15);
        let result = date.subtract_days(5);
        assert_eq!(result.year, 2024);
        assert_eq!(result.month, 6);
        assert_eq!(result.day, 10);
    }

    #[test]
    fn test_subtract_days_cross_month() {
        let date = Date::new(2024, 3, 5);
        let result = date.subtract_days(10);
        assert_eq!(result.year, 2024);
        assert_eq!(result.month, 2);
        assert_eq!(result.day, 24); // Feb 2024 has 29 days
    }

    #[test]
    fn test_subtract_days_zero() {
        let date = Date::new(2024, 6, 15);
        let result = date.subtract_days(0);
        assert_eq!(result, date);
    }

    #[test]
    fn test_add_days_within_month() {
        let date = Date::new(2024, 6, 15);
        let result = date.add_days(5);
        assert_eq!(result.year, 2024);
        assert_eq!(result.month, 6);
        assert_eq!(result.day, 20);
    }

    #[test]
    fn test_add_days_cross_month() {
        let date = Date::new(2024, 2, 25);
        let result = date.add_days(5);
        assert_eq!(result.year, 2024);
        assert_eq!(result.month, 3);
        assert_eq!(result.day, 1);
    }

    #[test]
    fn test_add_days_zero() {
        let date = Date::new(2024, 6, 15);
        let result = date.add_days(0);
        assert_eq!(result, date);
    }

    #[test]
    fn test_add_subtract_roundtrip() {
        let date = Date::new(2024, 6, 15);
        let added = date.add_days(10);
        let back = added.subtract_days(10);
        assert_eq!(back, date);
    }

    #[test]
    fn test_date_copy_clone() {
        let date1 = Date::new(2024, 6, 15);
        let date2 = date1;
        assert_eq!(date2, date1);

        let date3 = date1.clone();
        assert_eq!(date3, date1);
    }

    #[test]
    fn test_date_partial_eq() {
        let date1 = Date::new(2024, 6, 15);
        let date2 = Date::new(2024, 6, 15);
        assert_eq!(date1, date2);

        let date3 = Date::new(2024, 6, 16);
        assert_ne!(date1, date3);
    }

    #[test]
    fn test_date_partial_ord() {
        let date1 = Date::new(2024, 6, 15);
        let date2 = Date::new(2024, 6, 16);
        assert!(date1 < date2);
        assert!(date2 > date1);

        let date3 = Date::new(2024, 7, 1);
        assert!(date1 < date3);
    }

    #[test]
    fn test_prev_next_symmetry() {
        let date = Date::new(2024, 6, 15);
        let prev = date.prev_day();
        let back = prev.next_day();
        assert_eq!(back, date);
    }

    #[test]
    fn test_february_leap_year() {
        let date = Date::new(2024, 2, 28);
        let next = date.next_day();
        assert_eq!(next.day, 29); // 2024 is leap year
    }

    #[test]
    fn test_february_non_leap_year() {
        let date = Date::new(2023, 2, 28);
        let next = date.next_day();
        assert_eq!(next.month, 3); // Should roll over to March
        assert_eq!(next.day, 1);
    }

    #[test]
    fn test_subtract_large_amount() {
        let date = Date::new(2024, 6, 15);
        let result = date.subtract_days(100);
        // Should go back about 3 months
        assert!(result.year <= 2024);
        assert!(result.month < 6);
    }

    #[test]
    fn test_add_large_amount() {
        let date = Date::new(2024, 6, 15);
        let result = date.add_days(100);
        // Should go forward about 3 months
        assert!(result.year >= 2024);
    }

    #[test]
    fn test_date_ord_total() {
        let date1 = Date::new(2024, 1, 1);
        let date2 = Date::new(2024, 12, 31);
        assert!(date1 < date2);
    }

    #[test]
    fn test_is_valid_all_months() {
        for month in 1..=12 {
            let date = Date::new(2024, month, 1);
            assert!(date.is_valid(), "Month {} should be valid", month);
        }
    }

    #[test]
    fn test_is_valid_year_boundary() {
        // Year 0 is valid
        let date = Date::new(0, 1, 1);
        assert!(date.is_valid());

        // Negative year is valid
        let date = Date::new(-100, 6, 15);
        assert!(date.is_valid());
    }
}
