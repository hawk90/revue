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
