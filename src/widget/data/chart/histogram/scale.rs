//! Histogram value scale: the largest bar value and each bin's value

use super::{Histogram, HistogramBin};

impl Histogram {
    /// Get max value for y-axis
    pub(super) fn max_value(&self) -> f64 {
        if self.cumulative {
            1.0
        } else if self.density {
            self.bins
                .iter()
                .map(|b| b.density)
                .fold(0.0, f64::max)
                .max(0.001)
        } else {
            self.bins.iter().map(|b| b.count).max().unwrap_or(1) as f64
        }
    }

    /// Get bin value based on settings
    pub(super) fn bin_value(&self, bin: &HistogramBin, cumulative_sum: f64) -> f64 {
        if self.cumulative {
            cumulative_sum
        } else if self.density {
            bin.density
        } else {
            bin.count as f64
        }
    }
}
