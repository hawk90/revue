//! The time and value ranges the plot spans

use super::types::TimeRange as Range;
use super::TimeSeries;

impl TimeSeries {
    /// Get time bounds for the current time range
    pub fn get_time_bounds(&self) -> (u64, u64) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        match self.time_range {
            Range::All => {
                let mut min_ts = u64::MAX;
                let mut max_ts = 0u64;
                for series in &self.series {
                    for point in &series.points {
                        min_ts = min_ts.min(point.timestamp);
                        max_ts = max_ts.max(point.timestamp);
                    }
                }
                if min_ts == u64::MAX {
                    (now - 3600, now)
                } else {
                    (min_ts, max_ts)
                }
            }
            Range::LastSeconds(s) => (now.saturating_sub(s), now),
            Range::LastMinutes(m) => (now.saturating_sub(m * 60), now),
            Range::LastHours(h) => (now.saturating_sub(h * 3600), now),
            Range::LastDays(d) => (now.saturating_sub(d * 86400), now),
            Range::Range { start, end } => (start, end),
        }
    }

    /// Get value bounds for the current data
    pub fn get_value_bounds(&self) -> (f64, f64) {
        let (time_min, time_max) = self.get_time_bounds();

        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;

        for series in &self.series {
            for point in &series.points {
                if point.timestamp >= time_min && point.timestamp <= time_max {
                    min_val = min_val.min(point.value);
                    max_val = max_val.max(point.value);
                }
            }
        }

        if min_val == f64::MAX {
            min_val = 0.0;
            max_val = 100.0;
        }

        let min = self.y_min.unwrap_or(min_val);
        let max = self.y_max.unwrap_or(max_val);

        let range = max - min;
        let padding = if range == 0.0 { 1.0 } else { range * 0.1 };

        (
            self.y_min.unwrap_or(min - padding),
            self.y_max.unwrap_or(max + padding),
        )
    }
}
