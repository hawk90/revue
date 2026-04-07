//! Performance metrics tracker for debug overlay

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Performance metrics tracker
#[derive(Debug, Clone, Default)]
pub struct PerfMetrics {
    /// Frame times (last N frames)
    pub(crate) frame_times: VecDeque<Duration>,
    /// Last frame start time
    pub(crate) last_frame_start: Option<Instant>,
    /// Layout times
    pub(crate) layout_times: VecDeque<Duration>,
    /// Render times
    pub(crate) render_times: VecDeque<Duration>,
    /// Maximum samples to keep
    pub(crate) max_samples: usize,
}

impl PerfMetrics {
    /// Create new metrics tracker
    pub fn new() -> Self {
        Self {
            frame_times: VecDeque::new(),
            last_frame_start: None,
            layout_times: VecDeque::new(),
            render_times: VecDeque::new(),
            max_samples: 60,
        }
    }

    /// Start a new frame
    pub fn start_frame(&mut self) {
        let now = Instant::now();
        if let Some(last) = self.last_frame_start {
            let elapsed = now.duration_since(last);
            self.frame_times.push_back(elapsed);
            if self.frame_times.len() > self.max_samples {
                self.frame_times.pop_front();
            }
        }
        self.last_frame_start = Some(now);
    }

    /// Record layout time
    pub fn record_layout(&mut self, duration: Duration) {
        self.layout_times.push_back(duration);
        if self.layout_times.len() > self.max_samples {
            self.layout_times.pop_front();
        }
    }

    /// Record render time
    pub fn record_render(&mut self, duration: Duration) {
        self.render_times.push_back(duration);
        if self.render_times.len() > self.max_samples {
            self.render_times.pop_front();
        }
    }

    /// Get average FPS
    pub fn fps(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let total: Duration = self.frame_times.iter().sum();
        let avg = total.as_secs_f64() / self.frame_times.len() as f64;
        if avg > 0.0 {
            1.0 / avg
        } else {
            0.0
        }
    }

    /// Get average frame time in ms
    pub fn avg_frame_time_ms(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let total: Duration = self.frame_times.iter().sum();
        total.as_secs_f64() * 1000.0 / self.frame_times.len() as f64
    }

    /// Get average layout time in ms
    pub fn avg_layout_time_ms(&self) -> f64 {
        if self.layout_times.is_empty() {
            return 0.0;
        }
        let total: Duration = self.layout_times.iter().sum();
        total.as_secs_f64() * 1000.0 / self.layout_times.len() as f64
    }

    /// Get average render time in ms
    pub fn avg_render_time_ms(&self) -> f64 {
        if self.render_times.is_empty() {
            return 0.0;
        }
        let total: Duration = self.render_times.iter().sum();
        total.as_secs_f64() * 1000.0 / self.render_times.len() as f64
    }

    /// Reset all metrics
    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.layout_times.clear();
        self.render_times.clear();
        self.last_frame_start = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perf_metrics_new() {
        let metrics = PerfMetrics::new();
        assert!(metrics.frame_times.is_empty());
        assert!(metrics.last_frame_start.is_none());
        assert!(metrics.layout_times.is_empty());
        assert!(metrics.render_times.is_empty());
        assert_eq!(metrics.max_samples, 60);
    }

    #[test]
    fn test_perf_metrics_default() {
        let metrics = PerfMetrics::default();
        assert!(metrics.frame_times.is_empty());
    }

    #[test]
    fn test_perf_metrics_start_frame() {
        let mut metrics = PerfMetrics::new();
        metrics.start_frame();
        assert!(metrics.last_frame_start.is_some());
    }

    #[test]
    fn test_perf_metrics_record_layout() {
        let mut metrics = PerfMetrics::new();
        metrics.record_layout(Duration::from_millis(5));
        assert_eq!(metrics.layout_times.len(), 1);
    }

    #[test]
    fn test_perf_metrics_record_render() {
        let mut metrics = PerfMetrics::new();
        metrics.record_render(Duration::from_millis(3));
        assert_eq!(metrics.render_times.len(), 1);
    }

    #[test]
    fn test_perf_metrics_fps_empty() {
        let metrics = PerfMetrics::new();
        assert_eq!(metrics.fps(), 0.0);
    }

    #[test]
    fn test_perf_metrics_avg_frame_time_empty() {
        let metrics = PerfMetrics::new();
        assert_eq!(metrics.avg_frame_time_ms(), 0.0);
    }

    #[test]
    fn test_perf_metrics_avg_layout_time_empty() {
        let metrics = PerfMetrics::new();
        assert_eq!(metrics.avg_layout_time_ms(), 0.0);
    }

    #[test]
    fn test_perf_metrics_avg_render_time_empty() {
        let metrics = PerfMetrics::new();
        assert_eq!(metrics.avg_render_time_ms(), 0.0);
    }

    #[test]
    fn test_perf_metrics_reset() {
        let mut metrics = PerfMetrics::new();
        metrics.start_frame();
        metrics.record_layout(Duration::from_millis(1));
        metrics.record_render(Duration::from_millis(2));
        metrics.reset();
        assert!(metrics.frame_times.is_empty());
        assert!(metrics.last_frame_start.is_none());
        assert!(metrics.layout_times.is_empty());
        assert!(metrics.render_times.is_empty());
    }

    #[test]
    fn test_perf_metrics_record_and_query() {
        let mut metrics = PerfMetrics::new();
        metrics.start_frame();
        metrics.record_layout(Duration::from_millis(1));
        metrics.record_render(Duration::from_millis(2));

        assert!(metrics.avg_layout_time_ms() > 0.0);
        assert!(metrics.avg_render_time_ms() > 0.0);
    }
}
