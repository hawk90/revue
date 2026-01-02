//! Performance profiler for Revue applications
//!
//! Tracks render times, layout calculations, memory usage,
//! and provides performance insights.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::profiler::{Profiler, profile};
//!
//! // Profile a section of code
//! let result = profile("render_widget", || {
//!     expensive_render_operation()
//! });
//!
//! // Get profiling report
//! let report = Profiler::global().report();
//! println!("{}", report);
//! ```

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};
use std::time::{Duration, Instant};

// =============================================================================
// Timing Entry
// =============================================================================

/// A timing measurement
#[derive(Debug, Clone)]
pub struct Timing {
    /// Operation name
    pub name: String,
    /// Duration
    pub duration: Duration,
    /// Start time
    pub start: Instant,
    /// Parent operation (if nested)
    pub parent: Option<String>,
}

impl Timing {
    /// Create a new timing
    pub fn new(name: impl Into<String>, duration: Duration, start: Instant) -> Self {
        Self {
            name: name.into(),
            duration,
            start,
            parent: None,
        }
    }

    /// Set parent operation
    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Statistics for a profiled operation
#[derive(Debug, Clone, Default)]
pub struct Stats {
    /// Number of calls
    pub count: u64,
    /// Total duration
    pub total: Duration,
    /// Minimum duration
    pub min: Option<Duration>,
    /// Maximum duration
    pub max: Option<Duration>,
    /// Last duration
    pub last: Duration,
}

impl Stats {
    /// Create new stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a timing
    pub fn record(&mut self, duration: Duration) {
        self.count += 1;
        self.total += duration;
        self.last = duration;

        match self.min {
            Some(m) if duration < m => self.min = Some(duration),
            None => self.min = Some(duration),
            _ => {}
        }

        match self.max {
            Some(m) if duration > m => self.max = Some(duration),
            None => self.max = Some(duration),
            _ => {}
        }
    }

    /// Get average duration
    pub fn average(&self) -> Duration {
        if self.count == 0 {
            Duration::ZERO
        } else {
            self.total / self.count as u32
        }
    }

    /// Get average duration in milliseconds
    pub fn avg_ms(&self) -> f64 {
        self.average().as_secs_f64() * 1000.0
    }

    /// Get total duration in milliseconds
    pub fn total_ms(&self) -> f64 {
        self.total.as_secs_f64() * 1000.0
    }

    /// Get min duration in milliseconds
    pub fn min_ms(&self) -> f64 {
        self.min.map(|d| d.as_secs_f64() * 1000.0).unwrap_or(0.0)
    }

    /// Get max duration in milliseconds
    pub fn max_ms(&self) -> f64 {
        self.max.map(|d| d.as_secs_f64() * 1000.0).unwrap_or(0.0)
    }
}

// =============================================================================
// Profile Guard (RAII)
// =============================================================================

/// RAII guard that records timing when dropped
pub struct ProfileGuard {
    name: String,
    start: Instant,
    profiler: Arc<RwLock<ProfilerInner>>,
}

impl ProfileGuard {
    fn new(name: impl Into<String>, profiler: Arc<RwLock<ProfilerInner>>) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
            profiler,
        }
    }
}

impl Drop for ProfileGuard {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        if let Ok(mut p) = self.profiler.write() {
            p.record(&self.name, duration);
        }
    }
}

// =============================================================================
// Profiler Inner
// =============================================================================

#[derive(Debug, Default)]
struct ProfilerInner {
    /// Statistics by operation name
    stats: HashMap<String, Stats>,
    /// Recent timings (ring buffer)
    recent: Vec<Timing>,
    /// Maximum recent entries
    max_recent: usize,
    /// Is profiling enabled
    enabled: bool,
    /// Current profile stack (for nesting)
    stack: Vec<String>,
}

impl ProfilerInner {
    fn new() -> Self {
        Self {
            stats: HashMap::new(),
            recent: Vec::new(),
            max_recent: 100,
            enabled: true,
            stack: Vec::new(),
        }
    }

    fn record(&mut self, name: &str, duration: Duration) {
        if !self.enabled {
            return;
        }

        // Update stats
        let stats = self.stats.entry(name.to_string()).or_default();
        stats.record(duration);

        // Add to recent
        let mut timing = Timing::new(name, duration, Instant::now());
        if let Some(parent) = self.stack.last() {
            timing = timing.with_parent(parent.clone());
        }
        self.recent.push(timing);

        // Trim if needed
        if self.recent.len() > self.max_recent {
            self.recent.remove(0);
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
        self.recent.clear();
    }
}

// =============================================================================
// Profiler
// =============================================================================

/// Performance profiler
///
/// Tracks timing measurements for various operations.
#[derive(Debug, Clone)]
pub struct Profiler {
    inner: Arc<RwLock<ProfilerInner>>,
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Profiler {
    /// Create a new profiler
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(ProfilerInner::new())),
        }
    }

    /// Get the global profiler instance
    pub fn global() -> &'static Profiler {
        static INSTANCE: OnceLock<Profiler> = OnceLock::new();
        INSTANCE.get_or_init(Profiler::new)
    }

    /// Enable profiling
    pub fn enable(&self) {
        if let Ok(mut inner) = self.inner.write() {
            inner.enabled = true;
        }
    }

    /// Disable profiling
    pub fn disable(&self) {
        if let Ok(mut inner) = self.inner.write() {
            inner.enabled = false;
        }
    }

    /// Check if profiling is enabled
    pub fn is_enabled(&self) -> bool {
        self.inner.read().map(|i| i.enabled).unwrap_or(false)
    }

    /// Start a profiled section (returns guard that records on drop)
    pub fn start(&self, name: impl Into<String>) -> ProfileGuard {
        ProfileGuard::new(name, self.inner.clone())
    }

    /// Profile a closure
    pub fn profile<T, F: FnOnce() -> T>(&self, name: &str, f: F) -> T {
        let _guard = self.start(name);
        f()
    }

    /// Record a timing directly
    pub fn record(&self, name: &str, duration: Duration) {
        if let Ok(mut inner) = self.inner.write() {
            inner.record(name, duration);
        }
    }

    /// Get statistics for an operation
    pub fn stats(&self, name: &str) -> Option<Stats> {
        self.inner.read().ok()?.stats.get(name).cloned()
    }

    /// Get all statistics
    pub fn all_stats(&self) -> HashMap<String, Stats> {
        self.inner
            .read()
            .map(|i| i.stats.clone())
            .unwrap_or_default()
    }

    /// Reset all statistics
    pub fn reset(&self) {
        if let Ok(mut inner) = self.inner.write() {
            inner.reset();
        }
    }

    /// Generate a text report
    pub fn report(&self) -> String {
        let stats = self.all_stats();
        if stats.is_empty() {
            return "No profiling data collected.".to_string();
        }

        let mut output = String::new();
        output.push_str("Performance Report\n");
        output.push_str("==================\n\n");

        // Sort by total time descending
        let mut entries: Vec<_> = stats.into_iter().collect();
        entries.sort_by(|a, b| b.1.total.cmp(&a.1.total));

        output.push_str(&format!(
            "{:<30} {:>8} {:>10} {:>10} {:>10} {:>10}\n",
            "Operation", "Calls", "Total(ms)", "Avg(ms)", "Min(ms)", "Max(ms)"
        ));
        output.push_str(&"-".repeat(80));
        output.push('\n');

        for (name, stat) in entries {
            output.push_str(&format!(
                "{:<30} {:>8} {:>10.2} {:>10.3} {:>10.3} {:>10.3}\n",
                if name.len() > 30 {
                    format!("{}...", &name[..27])
                } else {
                    name.clone()
                },
                stat.count,
                stat.total_ms(),
                stat.avg_ms(),
                stat.min_ms(),
                stat.max_ms(),
            ));
        }

        output
    }

    /// Generate a compact summary
    pub fn summary(&self) -> String {
        let stats = self.all_stats();
        if stats.is_empty() {
            return String::new();
        }

        let total_time: Duration = stats.values().map(|s| s.total).sum();
        let total_calls: u64 = stats.values().map(|s| s.count).sum();

        format!(
            "{} operations, {} calls, {:.2}ms total",
            stats.len(),
            total_calls,
            total_time.as_secs_f64() * 1000.0
        )
    }
}

// =============================================================================
// Convenience Functions
// =============================================================================

/// Profile a section of code using the global profiler
pub fn profile<T, F: FnOnce() -> T>(name: &str, f: F) -> T {
    Profiler::global().profile(name, f)
}

/// Start a profiled section using the global profiler
pub fn start_profile(name: impl Into<String>) -> ProfileGuard {
    Profiler::global().start(name)
}

/// Get the global profiler report
pub fn profiler_report() -> String {
    Profiler::global().report()
}

// =============================================================================
// Scoped Profiler (Thread-local)
// =============================================================================

thread_local! {
    static THREAD_PROFILER: RefCell<Profiler> = RefCell::new(Profiler::new());
}

/// Get thread-local profiler
pub fn thread_profiler() -> Profiler {
    THREAD_PROFILER.with(|p| p.borrow().clone())
}

// =============================================================================
// Flame Graph Data
// =============================================================================

/// Node in a flame graph
#[derive(Debug, Clone)]
pub struct FlameNode {
    /// Operation name
    pub name: String,
    /// Self time (excluding children)
    pub self_time: Duration,
    /// Total time (including children)
    pub total_time: Duration,
    /// Child nodes
    pub children: Vec<FlameNode>,
}

impl FlameNode {
    /// Create a new flame node
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            self_time: Duration::ZERO,
            total_time: Duration::ZERO,
            children: Vec::new(),
        }
    }

    /// Add time
    pub fn add_time(&mut self, duration: Duration) {
        self.total_time += duration;
        self.self_time += duration;
    }

    /// Add child
    pub fn add_child(&mut self, child: FlameNode) {
        // Subtract child time from self time
        self.self_time = self.self_time.saturating_sub(child.total_time);
        self.children.push(child);
    }

    /// Format as text (for terminal display)
    pub fn format_text(&self, depth: usize) -> String {
        let mut output = String::new();
        let indent = "  ".repeat(depth);
        let percent = if self.total_time.as_nanos() > 0 {
            (self.self_time.as_nanos() as f64 / self.total_time.as_nanos() as f64) * 100.0
        } else {
            100.0
        };

        output.push_str(&format!(
            "{}{} ({:.2}ms / {:.1}%)\n",
            indent,
            self.name,
            self.total_time.as_secs_f64() * 1000.0,
            percent,
        ));

        for child in &self.children {
            output.push_str(&child.format_text(depth + 1));
        }

        output
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_stats_record() {
        let mut stats = Stats::new();
        stats.record(Duration::from_millis(10));
        stats.record(Duration::from_millis(20));
        stats.record(Duration::from_millis(15));

        assert_eq!(stats.count, 3);
        assert_eq!(stats.min, Some(Duration::from_millis(10)));
        assert_eq!(stats.max, Some(Duration::from_millis(20)));
        assert_eq!(stats.average(), Duration::from_millis(15));
    }

    #[test]
    fn test_profiler_basic() {
        let profiler = Profiler::new();

        {
            let _guard = profiler.start("test_operation");
            thread::sleep(Duration::from_millis(10));
        }

        let stats = profiler.stats("test_operation");
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().count, 1);
    }

    #[test]
    fn test_profiler_closure() {
        let profiler = Profiler::new();

        let result = profiler.profile("compute", || {
            thread::sleep(Duration::from_millis(5));
            42
        });

        assert_eq!(result, 42);
        let stats = profiler.stats("compute").unwrap();
        assert!(stats.avg_ms() >= 4.0);
    }

    #[test]
    fn test_profiler_enable_disable() {
        let profiler = Profiler::new();
        assert!(profiler.is_enabled());

        profiler.disable();
        assert!(!profiler.is_enabled());

        // Recordings should be ignored when disabled
        profiler.record("test", Duration::from_millis(10));
        assert!(profiler.stats("test").is_none());

        profiler.enable();
        profiler.record("test", Duration::from_millis(10));
        assert!(profiler.stats("test").is_some());
    }

    #[test]
    fn test_profiler_reset() {
        let profiler = Profiler::new();
        profiler.record("op1", Duration::from_millis(10));
        profiler.record("op2", Duration::from_millis(20));

        assert!(!profiler.all_stats().is_empty());

        profiler.reset();
        assert!(profiler.all_stats().is_empty());
    }

    #[test]
    fn test_profiler_report() {
        let profiler = Profiler::new();
        profiler.record("render", Duration::from_millis(10));
        profiler.record("layout", Duration::from_millis(5));

        let report = profiler.report();
        assert!(report.contains("render"));
        assert!(report.contains("layout"));
        assert!(report.contains("Performance Report"));
    }

    #[test]
    fn test_flame_node() {
        let mut root = FlameNode::new("main");
        root.add_time(Duration::from_millis(100));

        let mut child = FlameNode::new("render");
        child.add_time(Duration::from_millis(50));
        root.add_child(child);

        assert_eq!(root.self_time, Duration::from_millis(50));
        assert_eq!(root.total_time, Duration::from_millis(100));
        assert_eq!(root.children.len(), 1);
    }

    #[test]
    fn test_global_profiler() {
        let profiler1 = Profiler::global();
        let profiler2 = Profiler::global();

        // Should be the same instance
        profiler1.record("global_test", Duration::from_millis(10));
        assert!(profiler2.stats("global_test").is_some());
    }

    #[test]
    fn test_convenience_functions() {
        let result = profile("test_fn", || 42);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_profiler_summary() {
        let profiler = Profiler::new();
        profiler.record("op1", Duration::from_millis(10));
        profiler.record("op1", Duration::from_millis(20));

        let summary = profiler.summary();
        assert!(summary.contains("1 operations"));
        assert!(summary.contains("2 calls"));
    }
}
