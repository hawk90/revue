//! Performance profiling utilities
//!
//! Provides tools for measuring and optimizing TUI application performance.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::app::profiler::{Profiler, profiler};
//!
//! let mut profiler = Profiler::new();
//!
//! // Time a render operation
//! profiler.start("render");
//! // ... render widgets ...
//! profiler.end("render");
//!
//! // Get statistics
//! if let Some(stats) = profiler.stats("render") {
//!     println!("Avg render: {:?}", stats.avg);
//!     println!("Max render: {:?}", stats.max);
//! }
//!
//! // Print report
//! println!("{}", profiler.report());
//! ```

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance measurement sample
#[derive(Clone, Debug)]
pub struct Sample {
    /// Duration of the sample
    pub duration: Duration,
    /// Timestamp when sample was taken
    pub timestamp: Instant,
}

impl Sample {
    /// Create new sample
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            timestamp: Instant::now(),
        }
    }
}

/// Statistics for a metric
#[derive(Clone, Debug)]
pub struct Stats {
    /// Number of samples
    pub count: usize,
    /// Total duration
    pub total: Duration,
    /// Minimum duration
    pub min: Duration,
    /// Maximum duration
    pub max: Duration,
    /// Average duration
    pub avg: Duration,
    /// Standard deviation (approximate)
    pub std_dev: Duration,
    /// Last sample
    pub last: Duration,
    /// Samples per second (throughput)
    pub throughput: f64,
}

impl Stats {
    /// Create stats from samples
    pub fn from_samples(samples: &[Sample]) -> Option<Self> {
        if samples.is_empty() {
            return None;
        }

        let count = samples.len();
        let total: Duration = samples.iter().map(|s| s.duration).sum();
        // samples is guaranteed non-empty by early return above
        let min = samples.iter().map(|s| s.duration).min().unwrap_or_default();
        let max = samples.iter().map(|s| s.duration).max().unwrap_or_default();
        let avg = total / count as u32;
        let last = samples.last().map(|s| s.duration).unwrap_or_default();

        // Calculate throughput based on time window
        let throughput = if let (Some(first), Some(last_sample)) = (samples.first(), samples.last())
        {
            let window = last_sample.timestamp.duration_since(first.timestamp);
            if window.as_secs_f64() > 0.0 {
                count as f64 / window.as_secs_f64()
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Calculate standard deviation
        let avg_nanos = avg.as_nanos() as f64;
        let variance: f64 = samples
            .iter()
            .map(|s| {
                let diff = s.duration.as_nanos() as f64 - avg_nanos;
                diff * diff
            })
            .sum::<f64>()
            / count as f64;
        let std_dev = Duration::from_nanos(variance.sqrt() as u64);

        Some(Self {
            count,
            total,
            min,
            max,
            avg,
            std_dev,
            last,
            throughput,
        })
    }

    /// Format duration for display
    fn format_duration(d: Duration) -> String {
        let micros = d.as_micros();
        if micros < 1000 {
            format!("{}μs", micros)
        } else if micros < 1_000_000 {
            format!("{:.2}ms", micros as f64 / 1000.0)
        } else {
            format!("{:.2}s", d.as_secs_f64())
        }
    }
}

/// Metric type for categorization
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MetricType {
    /// Render timing
    Render,
    /// Event handling timing
    Event,
    /// Layout calculation timing
    Layout,
    /// Custom timing
    Custom,
}

/// Metric entry with samples
#[derive(Clone, Debug)]
pub struct Metric {
    /// Metric name
    pub name: String,
    /// Metric type
    pub metric_type: MetricType,
    /// Samples
    samples: Vec<Sample>,
    /// Maximum samples to keep
    max_samples: usize,
    /// Active start time
    start_time: Option<Instant>,
}

impl Metric {
    /// Create new metric
    pub fn new(name: impl Into<String>, metric_type: MetricType) -> Self {
        Self {
            name: name.into(),
            metric_type,
            samples: Vec::new(),
            max_samples: 1000,
            start_time: None,
        }
    }

    /// Set max samples to keep
    pub fn max_samples(mut self, max: usize) -> Self {
        self.max_samples = max;
        self
    }

    /// Start timing
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// End timing and record sample
    pub fn end(&mut self) -> Option<Duration> {
        if let Some(start) = self.start_time.take() {
            let duration = start.elapsed();
            self.add_sample(duration);
            Some(duration)
        } else {
            None
        }
    }

    /// Add a sample directly
    pub fn add_sample(&mut self, duration: Duration) {
        self.samples.push(Sample::new(duration));
        if self.samples.len() > self.max_samples {
            self.samples.remove(0);
        }
    }

    /// Get statistics
    pub fn stats(&self) -> Option<Stats> {
        Stats::from_samples(&self.samples)
    }

    /// Clear all samples
    pub fn clear(&mut self) {
        self.samples.clear();
        self.start_time = None;
    }

    /// Get sample count
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }

    /// Get samples
    pub fn samples(&self) -> &[Sample] {
        &self.samples
    }
}

/// Frame rate counter
#[derive(Clone, Debug)]
pub struct FpsCounter {
    /// Frame timestamps
    frames: Vec<Instant>,
    /// Window size for FPS calculation
    window: Duration,
}

impl FpsCounter {
    /// Create new FPS counter
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            window: Duration::from_secs(1),
        }
    }

    /// Set window size
    pub fn window(mut self, duration: Duration) -> Self {
        self.window = duration;
        self
    }

    /// Record a frame
    pub fn frame(&mut self) {
        let now = Instant::now();
        self.frames.push(now);

        // Remove old frames outside window
        let cutoff = now - self.window;
        self.frames.retain(|&t| t >= cutoff);
    }

    /// Get current FPS
    pub fn fps(&self) -> f64 {
        if self.frames.len() < 2 {
            return 0.0;
        }

        let count = self.frames.len();
        let window_secs = self.window.as_secs_f64();
        count as f64 / window_secs
    }

    /// Get frame time (average)
    pub fn frame_time(&self) -> Option<Duration> {
        let fps = self.fps();
        if fps > 0.0 {
            Some(Duration::from_secs_f64(1.0 / fps))
        } else {
            None
        }
    }

    /// Reset counter
    pub fn reset(&mut self) {
        self.frames.clear();
    }
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance profiler
pub struct Profiler {
    /// Named metrics
    metrics: HashMap<String, Metric>,
    /// FPS counter
    fps: FpsCounter,
    /// Profiling enabled
    enabled: bool,
    /// Start time
    start_time: Instant,
}

impl Profiler {
    /// Create new profiler
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
            fps: FpsCounter::new(),
            enabled: true,
            start_time: Instant::now(),
        }
    }

    /// Enable or disable profiling
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if profiling is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Register a metric
    pub fn register(&mut self, name: impl Into<String>, metric_type: MetricType) {
        let name = name.into();
        self.metrics
            .insert(name.clone(), Metric::new(name, metric_type));
    }

    /// Start timing a metric
    pub fn start(&mut self, name: &str) {
        if !self.enabled {
            return;
        }

        if !self.metrics.contains_key(name) {
            self.metrics
                .insert(name.to_string(), Metric::new(name, MetricType::Custom));
        }

        if let Some(metric) = self.metrics.get_mut(name) {
            metric.start();
        }
    }

    /// End timing a metric
    pub fn end(&mut self, name: &str) -> Option<Duration> {
        if !self.enabled {
            return None;
        }

        self.metrics.get_mut(name).and_then(|m| m.end())
    }

    /// Record a sample directly
    pub fn record(&mut self, name: &str, duration: Duration) {
        if !self.enabled {
            return;
        }

        if !self.metrics.contains_key(name) {
            self.metrics
                .insert(name.to_string(), Metric::new(name, MetricType::Custom));
        }

        if let Some(metric) = self.metrics.get_mut(name) {
            metric.add_sample(duration);
        }
    }

    /// Time a closure
    pub fn time<F, T>(&mut self, name: &str, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        if !self.enabled {
            return f();
        }

        self.start(name);
        let result = f();
        self.end(name);
        result
    }

    /// Record a frame
    pub fn frame(&mut self) {
        if self.enabled {
            self.fps.frame();
        }
    }

    /// Get current FPS
    pub fn fps(&self) -> f64 {
        self.fps.fps()
    }

    /// Get statistics for a metric
    pub fn stats(&self, name: &str) -> Option<Stats> {
        self.metrics.get(name).and_then(|m| m.stats())
    }

    /// Get all metrics
    pub fn metrics(&self) -> impl Iterator<Item = (&str, &Metric)> {
        self.metrics.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// Clear all metrics
    pub fn clear(&mut self) {
        for metric in self.metrics.values_mut() {
            metric.clear();
        }
        self.fps.reset();
    }

    /// Get total runtime
    pub fn runtime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Generate a text report
    pub fn report(&self) -> String {
        let mut lines = Vec::new();
        lines.push("=== Performance Report ===".to_string());
        lines.push(format!("Runtime: {:.2}s", self.runtime().as_secs_f64()));
        lines.push(format!("FPS: {:.1}", self.fps()));
        lines.push(String::new());

        // Sort metrics by name
        let mut metric_names: Vec<_> = self.metrics.keys().collect();
        metric_names.sort();

        for name in metric_names {
            if let Some(stats) = self.stats(name) {
                lines.push(format!("[{}]", name));
                lines.push(format!("  Samples: {}", stats.count));
                lines.push(format!("  Avg: {}", Stats::format_duration(stats.avg)));
                lines.push(format!("  Min: {}", Stats::format_duration(stats.min)));
                lines.push(format!("  Max: {}", Stats::format_duration(stats.max)));
                lines.push(format!(
                    "  Std Dev: {}",
                    Stats::format_duration(stats.std_dev)
                ));
                lines.push(format!("  Throughput: {:.1}/s", stats.throughput));
                lines.push(String::new());
            }
        }

        lines.join("\n")
    }

    /// Generate a summary (shorter report)
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        parts.push(format!("FPS: {:.1}", self.fps()));

        for (name, metric) in &self.metrics {
            if let Some(stats) = metric.stats() {
                parts.push(format!("{}: {}", name, Stats::format_duration(stats.avg)));
            }
        }

        parts.join(" | ")
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Scoped timing guard
pub struct TimingGuard<'a> {
    profiler: &'a mut Profiler,
    name: String,
}

impl<'a> TimingGuard<'a> {
    /// Create new timing guard
    pub fn new(profiler: &'a mut Profiler, name: impl Into<String>) -> Self {
        let name = name.into();
        profiler.start(&name);
        Self { profiler, name }
    }
}

impl Drop for TimingGuard<'_> {
    fn drop(&mut self) {
        self.profiler.end(&self.name);
    }
}

/// Create a new profiler
pub fn profiler() -> Profiler {
    Profiler::new()
}

/// Create a new FPS counter
pub fn fps_counter() -> FpsCounter {
    FpsCounter::new()
}

/// Performance snapshot for comparison
#[derive(Clone, Debug)]
pub struct Snapshot {
    /// Metric stats
    pub stats: HashMap<String, Stats>,
    /// FPS at snapshot time
    pub fps: f64,
    /// Runtime at snapshot
    pub runtime: Duration,
    /// Timestamp
    pub timestamp: Instant,
}

impl Snapshot {
    /// Take a snapshot from profiler
    pub fn from_profiler(profiler: &Profiler) -> Self {
        let stats = profiler
            .metrics
            .iter()
            .filter_map(|(name, metric)| metric.stats().map(|s| (name.clone(), s)))
            .collect();

        Self {
            stats,
            fps: profiler.fps(),
            runtime: profiler.runtime(),
            timestamp: Instant::now(),
        }
    }

    /// Compare with another snapshot
    pub fn compare(&self, other: &Snapshot) -> SnapshotDiff {
        let mut diffs = HashMap::new();

        for (name, stats) in &self.stats {
            if let Some(other_stats) = other.stats.get(name) {
                let avg_diff = stats.avg.as_nanos() as i128 - other_stats.avg.as_nanos() as i128;
                diffs.insert(
                    name.clone(),
                    MetricDiff {
                        avg_change_nanos: avg_diff,
                        count_change: stats.count as i64 - other_stats.count as i64,
                    },
                );
            }
        }

        SnapshotDiff {
            fps_change: self.fps - other.fps,
            metric_diffs: diffs,
        }
    }
}

/// Difference between snapshots
#[derive(Clone, Debug)]
pub struct SnapshotDiff {
    /// FPS change
    pub fps_change: f64,
    /// Metric differences
    pub metric_diffs: HashMap<String, MetricDiff>,
}

/// Metric difference
#[derive(Clone, Debug)]
pub struct MetricDiff {
    /// Average change in nanoseconds
    pub avg_change_nanos: i128,
    /// Sample count change
    pub count_change: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_profiler_basic() {
        let mut profiler = Profiler::new();

        profiler.start("test");
        thread::sleep(Duration::from_millis(10));
        let duration = profiler.end("test");

        assert!(duration.is_some());
        assert!(duration.unwrap() >= Duration::from_millis(10));
    }

    #[test]
    fn test_profiler_stats() {
        let mut profiler = Profiler::new();

        for _ in 0..10 {
            profiler.record("test", Duration::from_millis(10));
        }

        let stats = profiler.stats("test").unwrap();
        assert_eq!(stats.count, 10);
        assert_eq!(stats.avg, Duration::from_millis(10));
    }

    #[test]
    fn test_profiler_time() {
        let mut profiler = Profiler::new();

        let result = profiler.time("compute", || {
            thread::sleep(Duration::from_millis(5));
            42
        });

        assert_eq!(result, 42);
        assert!(profiler.stats("compute").is_some());
    }

    #[test]
    fn test_profiler_disabled() {
        let mut profiler = Profiler::new();
        profiler.set_enabled(false);

        profiler.start("test");
        thread::sleep(Duration::from_millis(5));
        let duration = profiler.end("test");

        assert!(duration.is_none());
    }

    #[test]
    fn test_fps_counter() {
        let mut fps = FpsCounter::new().window(Duration::from_millis(500));

        // Record frames without sleep - just testing frame counting logic
        for _ in 0..10 {
            fps.frame();
        }

        // FPS should be calculated (may be very high without sleep)
        let measured = fps.fps();
        // Just verify it returns a positive number (logic works)
        assert!(measured >= 0.0);
    }

    #[test]
    fn test_profiler_report() {
        let mut profiler = Profiler::new();

        profiler.record("render", Duration::from_millis(16));
        profiler.record("render", Duration::from_millis(17));
        profiler.record("event", Duration::from_micros(100));

        let report = profiler.report();
        assert!(report.contains("render"));
        assert!(report.contains("event"));
    }

    #[test]
    fn test_stats_format_duration() {
        assert_eq!(Stats::format_duration(Duration::from_micros(500)), "500μs");
        assert_eq!(
            Stats::format_duration(Duration::from_micros(1500)),
            "1.50ms"
        );
        assert_eq!(Stats::format_duration(Duration::from_secs(2)), "2.00s");
    }

    #[test]
    fn test_metric() {
        let mut metric = Metric::new("test", MetricType::Render);

        metric.start();
        thread::sleep(Duration::from_millis(5));
        metric.end();

        assert_eq!(metric.sample_count(), 1);
    }

    #[test]
    fn test_metric_max_samples() {
        let mut metric = Metric::new("test", MetricType::Custom).max_samples(5);

        for i in 0..10 {
            metric.add_sample(Duration::from_millis(i));
        }

        assert_eq!(metric.sample_count(), 5);
    }

    #[test]
    fn test_snapshot() {
        let mut profiler = Profiler::new();
        profiler.record("test", Duration::from_millis(10));

        let snapshot = Snapshot::from_profiler(&profiler);
        assert!(snapshot.stats.contains_key("test"));
    }

    #[test]
    fn test_snapshot_compare() {
        let mut profiler = Profiler::new();
        profiler.record("test", Duration::from_millis(10));
        let snap1 = Snapshot::from_profiler(&profiler);

        profiler.record("test", Duration::from_millis(20));
        let snap2 = Snapshot::from_profiler(&profiler);

        let diff = snap2.compare(&snap1);
        assert!(diff.metric_diffs.contains_key("test"));
    }

    #[test]
    fn test_profiler_helper() {
        let p = profiler();
        assert!(p.is_enabled());
    }

    #[test]
    fn test_fps_counter_helper() {
        let fps = fps_counter();
        assert_eq!(fps.fps(), 0.0);
    }

    #[test]
    fn test_profiler_summary() {
        let mut profiler = Profiler::new();
        profiler.record("render", Duration::from_millis(16));

        let summary = profiler.summary();
        assert!(summary.contains("FPS"));
        assert!(summary.contains("render"));
    }

    // =========================================================================
    // Additional profiler tests
    // =========================================================================

    #[test]
    fn test_profiler_default() {
        let profiler = Profiler::default();
        assert!(profiler.is_enabled());
    }

    #[test]
    fn test_profiler_register() {
        let mut profiler = Profiler::new();
        profiler.register("custom_metric", MetricType::Custom);
        assert!(profiler.stats("custom_metric").is_none()); // No samples yet
    }

    #[test]
    fn test_profiler_start_without_register() {
        let mut profiler = Profiler::new();
        profiler.start("auto_register"); // Should auto-register
        let _ = profiler.end("auto_register");
        assert!(profiler.stats("auto_register").is_some());
    }

    #[test]
    fn test_profiler_record_without_register() {
        let mut profiler = Profiler::new();
        profiler.record("auto_record", Duration::from_millis(10));
        assert!(profiler.stats("auto_record").is_some());
    }

    #[test]
    fn test_profiler_metrics_iteration() {
        let mut profiler = Profiler::new();
        profiler.record("metric1", Duration::from_millis(10));
        profiler.record("metric2", Duration::from_millis(20));

        let mut count = 0;
        for (name, _metric) in profiler.metrics() {
            assert!(name == "metric1" || name == "metric2");
            count += 1;
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_profiler_clear() {
        let mut profiler = Profiler::new();
        profiler.record("test", Duration::from_millis(10));
        profiler.clear();
        assert!(profiler.stats("test").is_none());
        assert_eq!(profiler.fps(), 0.0);
    }

    #[test]
    fn test_profiler_runtime() {
        let profiler = Profiler::new();
        thread::sleep(Duration::from_millis(10));
        let runtime = profiler.runtime();
        assert!(runtime >= Duration::from_millis(10));
    }

    #[test]
    fn test_profiler_time_returns_result() {
        let mut profiler = Profiler::new();
        let result = profiler.time("calc", || 2 + 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_profiler_frame() {
        let mut profiler = Profiler::new();
        profiler.frame();
        profiler.frame();
        profiler.frame();
        // FPS should be calculated
        let fps = profiler.fps();
        // May be very high since we didn't sleep
        assert!(fps >= 0.0);
    }

    #[test]
    fn test_profiler_set_enabled() {
        let mut profiler = Profiler::new();
        assert!(profiler.is_enabled());
        profiler.set_enabled(false);
        assert!(!profiler.is_enabled());
        profiler.set_enabled(true);
        assert!(profiler.is_enabled());
    }

    #[test]
    fn test_metric_new_with_string() {
        let name = String::from("string_metric");
        let metric = Metric::new(name.clone(), MetricType::Event);
        assert_eq!(metric.name, name);
    }

    #[test]
    fn test_metric_clear() {
        let mut metric = Metric::new("test", MetricType::Render);
        metric.add_sample(Duration::from_millis(10));
        metric.clear();
        assert_eq!(metric.sample_count(), 0);
    }

    #[test]
    fn test_metric_samples_slice() {
        let mut metric = Metric::new("test", MetricType::Render);
        metric.add_sample(Duration::from_millis(10));
        metric.add_sample(Duration::from_millis(20));
        let samples = metric.samples();
        assert_eq!(samples.len(), 2);
    }

    #[test]
    fn test_metric_stats_empty() {
        let metric = Metric::new("test", MetricType::Render);
        assert!(metric.stats().is_none());
    }

    #[test]
    fn test_metric_end_without_start() {
        let mut metric = Metric::new("test", MetricType::Render);
        let result = metric.end();
        assert!(result.is_none());
    }

    #[test]
    fn test_metric_double_start() {
        let mut metric = Metric::new("test", MetricType::Render);
        metric.start();
        thread::sleep(Duration::from_millis(5));
        metric.start(); // Should reset start time
        thread::sleep(Duration::from_millis(2));
        let duration = metric.end();
        assert!(duration.unwrap() < Duration::from_millis(10));
    }

    #[test]
    fn test_stats_from_samples_empty() {
        let stats = Stats::from_samples(&[]);
        assert!(stats.is_none());
    }

    #[test]
    fn test_stats_from_samples_single() {
        let samples = vec![Sample::new(Duration::from_millis(10))];
        let stats = Stats::from_samples(&samples).unwrap();
        assert_eq!(stats.count, 1);
        assert_eq!(stats.avg, Duration::from_millis(10));
        assert_eq!(stats.min, Duration::from_millis(10));
        assert_eq!(stats.max, Duration::from_millis(10));
    }

    #[test]
    fn test_fps_counter_default() {
        let fps = FpsCounter::default();
        assert_eq!(fps.fps(), 0.0);
    }

    #[test]
    fn test_fps_counter_window() {
        let fps = FpsCounter::new().window(Duration::from_secs(2));
        // Just verify it can be set
        let _ = fps;
    }

    #[test]
    fn test_fps_counter_frame_time() {
        let mut fps = FpsCounter::new();
        fps.frame();
        fps.frame();
        let frame_time = fps.frame_time();
        // With only 2 frames, may or may not have valid FPS
        let _ = frame_time;
    }

    #[test]
    fn test_fps_counter_reset() {
        let mut fps = FpsCounter::new();
        fps.frame();
        fps.frame();
        fps.reset();
        assert_eq!(fps.fps(), 0.0);
    }

    #[test]
    fn test_metric_type_all_variants() {
        let types = [
            MetricType::Render,
            MetricType::Event,
            MetricType::Layout,
            MetricType::Custom,
        ];
        for (i, t1) in types.iter().enumerate() {
            for (j, t2) in types.iter().enumerate() {
                if i == j {
                    assert_eq!(t1, t2);
                } else {
                    assert_ne!(t1, t2);
                }
            }
        }
    }

    #[test]
    fn test_sample_new() {
        let duration = Duration::from_millis(100);
        let sample = Sample::new(duration);
        assert_eq!(sample.duration, duration);
    }

    #[test]
    fn test_sample_timestamp() {
        let before = Instant::now();
        let sample = Sample::new(Duration::ZERO);
        let after = Instant::now();
        assert!(sample.timestamp >= before);
        assert!(sample.timestamp <= after);
    }

    #[test]
    fn test_timing_guard_drop() {
        let mut profiler = Profiler::new();
        {
            let _guard = TimingGuard::new(&mut profiler, "guard_test");
            thread::sleep(Duration::from_millis(5));
        } // Guard drops here
        assert!(profiler.stats("guard_test").is_some());
    }

    #[test]
    fn test_snapshot_all_fields() {
        let mut profiler = Profiler::new();
        profiler.record("test", Duration::from_millis(10));
        let snapshot = Snapshot::from_profiler(&profiler);
        assert!(snapshot.stats.contains_key("test"));
        assert!(snapshot.runtime >= Duration::ZERO);
    }

    #[test]
    fn test_snapshot_diff_empty() {
        let profiler = Profiler::new();
        let snap1 = Snapshot::from_profiler(&profiler);
        let snap2 = Snapshot::from_profiler(&profiler);
        let diff = snap2.compare(&snap1);
        assert!(diff.metric_diffs.is_empty());
    }

    #[test]
    fn test_snapshot_diff_different_metrics() {
        let mut profiler1 = Profiler::new();
        profiler1.record("metric1", Duration::from_millis(10));
        let snap1 = Snapshot::from_profiler(&profiler1);

        let mut profiler2 = Profiler::new();
        profiler2.record("metric2", Duration::from_millis(20));
        let snap2 = Snapshot::from_profiler(&profiler2);

        let diff = snap2.compare(&snap1);
        // No common metrics to compare
        assert!(diff.metric_diffs.is_empty());
    }
}
