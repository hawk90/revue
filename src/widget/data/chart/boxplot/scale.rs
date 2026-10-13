//! Box plot value scale: the value bounds, the value-to-cell mapping and the notch interval

use super::group::BoxGroup;
use super::render::BoxPlotRender;
use super::BoxPlot;

impl BoxPlot {
    /// Compute value bounds
    pub(super) fn compute_bounds(&self) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for group in &self.groups {
            if let Some(stats) = group.get_stats(self.whisker_style) {
                min = min.min(stats.whisker_low);
                max = max.max(stats.whisker_high);
                for &outlier in &stats.outliers {
                    min = min.min(outlier);
                    max = max.max(outlier);
                }
            }
        }

        // Apply axis overrides
        let min = self.value_axis.min.unwrap_or(min);
        let max = self.value_axis.max.unwrap_or(max);

        // Add padding
        let range = (max - min).max(1.0);
        let padding = range * 0.1;

        (min - padding, max + padding)
    }
}

impl<'a> BoxPlotRender<'a> {
    /// Map value to screen coordinate
    pub fn value_to_screen(&self, value: f64, length: u16) -> u16 {
        let (min, max) = self.bounds;
        let range = (max - min).max(1.0);
        ((value - min) / range * (length as f64 - 1.0)) as u16
    }

    /// The notch of a group: the approximate 95% confidence interval of the
    /// median, `median ± 1.57 * IQR / sqrt(n)`.
    ///
    /// `None` when notches are off, when the group was built from
    /// precomputed [`BoxStats`](super::BoxStats) (its sample size is
    /// unknown), or when the interval is empty.
    pub fn notch_bounds(&self, group: &BoxGroup, stats: &super::BoxStats) -> Option<(f64, f64)> {
        if !self.notched || group.stats.is_some() {
            return None;
        }
        let n = group.data.iter().filter(|v| v.is_finite()).count();
        if n == 0 {
            return None;
        }
        let half = 1.57 * (stats.q3 - stats.q1) / (n as f64).sqrt();
        (half > 0.0).then_some((stats.median - half, stats.median + half))
    }
}
