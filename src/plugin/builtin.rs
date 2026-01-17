//! Built-in plugins

use super::{Plugin, PluginContext};
use std::time::{Duration, Instant};

// =============================================================================
// Logger Plugin
// =============================================================================

/// Simple logging plugin for debugging
///
/// Logs lifecycle events and optionally tick counts.
///
/// # Example
///
/// ```rust,ignore
/// use revue::plugin::LoggerPlugin;
///
/// let app = App::builder()
///     .plugin(LoggerPlugin::new().verbose(true))
///     .build();
/// ```
pub struct LoggerPlugin {
    verbose: bool,
    tick_count: usize,
    log_interval: usize,
}

impl LoggerPlugin {
    /// Create a new logger plugin
    pub fn new() -> Self {
        Self {
            verbose: false,
            tick_count: 0,
            log_interval: 60, // Log every 60 ticks (~1 second at 60fps)
        }
    }

    /// Enable verbose logging (logs tick counts)
    pub fn verbose(mut self, enabled: bool) -> Self {
        self.verbose = enabled;
        self
    }

    /// Set tick logging interval
    pub fn log_interval(mut self, interval: usize) -> Self {
        self.log_interval = interval;
        self
    }
}

impl Default for LoggerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for LoggerPlugin {
    fn name(&self) -> &str {
        "logger"
    }

    fn priority(&self) -> i32 {
        100 // Run early
    }

    fn on_init(&mut self, ctx: &mut PluginContext) -> crate::Result<()> {
        ctx.log("Plugin initialized");
        Ok(())
    }

    fn on_mount(&mut self, ctx: &mut PluginContext) -> crate::Result<()> {
        let (w, h) = ctx.terminal_size();
        ctx.log(&format!("App mounted (terminal: {}x{})", w, h));
        Ok(())
    }

    fn on_tick(&mut self, ctx: &mut PluginContext, _delta: Duration) -> crate::Result<()> {
        self.tick_count += 1;

        if self.verbose && self.tick_count.is_multiple_of(self.log_interval) {
            ctx.log(&format!("Tick #{}", self.tick_count));
        }

        Ok(())
    }

    fn on_unmount(&mut self, ctx: &mut PluginContext) -> crate::Result<()> {
        ctx.log(&format!("App unmounted after {} ticks", self.tick_count));
        Ok(())
    }
}

// =============================================================================
// Performance Plugin
// =============================================================================

/// Performance monitoring plugin
///
/// Tracks frame times, FPS, and provides performance metrics.
///
/// # Example
///
/// ```rust,ignore
/// use revue::plugin::PerformancePlugin;
///
/// let app = App::builder()
///     .plugin(PerformancePlugin::new())
///     .build();
///
/// // Access metrics via plugin context
/// // ctx.get_plugin_data::<f64>("performance", "fps")
/// ```
pub struct PerformancePlugin {
    frame_times: Vec<Duration>,
    max_samples: usize,
    last_report: Instant,
    report_interval: Duration,
}

impl PerformancePlugin {
    /// Create a new performance plugin
    pub fn new() -> Self {
        Self {
            frame_times: Vec::with_capacity(120),
            max_samples: 120,
            last_report: Instant::now(),
            report_interval: Duration::from_secs(5),
        }
    }

    /// Set maximum number of frame samples to keep
    pub fn max_samples(mut self, count: usize) -> Self {
        self.max_samples = count;
        self.frame_times = Vec::with_capacity(count);
        self
    }

    /// Set report interval
    pub fn report_interval(mut self, interval: Duration) -> Self {
        self.report_interval = interval;
        self
    }

    /// Calculate average FPS from samples
    fn calculate_fps(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let total: Duration = self.frame_times.iter().sum();
        let avg_frame_time = total.as_secs_f64() / self.frame_times.len() as f64;

        if avg_frame_time > 0.0 {
            1.0 / avg_frame_time
        } else {
            0.0
        }
    }

    /// Get min/max/avg frame times
    fn frame_time_stats(&self) -> (Duration, Duration, Duration) {
        if self.frame_times.is_empty() {
            return (Duration::ZERO, Duration::ZERO, Duration::ZERO);
        }

        let min = *self.frame_times.iter().min().unwrap();
        let max = *self.frame_times.iter().max().unwrap();
        let sum: Duration = self.frame_times.iter().sum();
        let avg = sum / self.frame_times.len() as u32;

        (min, max, avg)
    }
}

impl Default for PerformancePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for PerformancePlugin {
    fn name(&self) -> &str {
        "performance"
    }

    fn priority(&self) -> i32 {
        -100 // Run late (measure actual frame time)
    }

    fn on_init(&mut self, ctx: &mut PluginContext) -> crate::Result<()> {
        ctx.set_data("fps", 0.0f64);
        ctx.set_data("frame_time_ms", 0.0f64);
        Ok(())
    }

    fn on_mount(&mut self, _ctx: &mut PluginContext) -> crate::Result<()> {
        self.last_report = Instant::now();
        Ok(())
    }

    fn on_tick(&mut self, ctx: &mut PluginContext, delta: Duration) -> crate::Result<()> {
        // Record frame time
        self.frame_times.push(delta);
        if self.frame_times.len() > self.max_samples {
            self.frame_times.remove(0);
        }

        // Update metrics
        let fps = self.calculate_fps();
        ctx.set_data("fps", fps);
        ctx.set_data("frame_time_ms", delta.as_secs_f64() * 1000.0);

        // Periodic report
        if self.last_report.elapsed() >= self.report_interval {
            let (min, max, avg) = self.frame_time_stats();
            ctx.log(&format!(
                "FPS: {:.1} | Frame time: {:.2}ms (min: {:.2}ms, max: {:.2}ms)",
                fps,
                avg.as_secs_f64() * 1000.0,
                min.as_secs_f64() * 1000.0,
                max.as_secs_f64() * 1000.0
            ));
            self.last_report = Instant::now();
        }

        Ok(())
    }

    fn on_unmount(&mut self, ctx: &mut PluginContext) -> crate::Result<()> {
        let fps = self.calculate_fps();
        let (min, max, avg) = self.frame_time_stats();
        ctx.log(&format!(
            "Final stats - FPS: {:.1} | Avg frame: {:.2}ms | Min: {:.2}ms | Max: {:.2}ms | Samples: {}",
            fps,
            avg.as_secs_f64() * 1000.0,
            min.as_secs_f64() * 1000.0,
            max.as_secs_f64() * 1000.0,
            self.frame_times.len()
        ));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // LoggerPlugin tests
    // =========================================================================

    #[test]
    fn test_logger_plugin() {
        let mut plugin = LoggerPlugin::new().verbose(true).log_interval(10);
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("logger");

        plugin.on_init(&mut ctx).unwrap();
        plugin.on_mount(&mut ctx).unwrap();

        for _ in 0..25 {
            plugin.on_tick(&mut ctx, Duration::from_millis(16)).unwrap();
        }

        plugin.on_unmount(&mut ctx).unwrap();
    }

    #[test]
    fn test_logger_plugin_new() {
        let plugin = LoggerPlugin::new();
        assert_eq!(plugin.verbose, false);
        assert_eq!(plugin.tick_count, 0);
        assert_eq!(plugin.log_interval, 60);
    }

    #[test]
    fn test_logger_plugin_verbose() {
        let plugin = LoggerPlugin::new().verbose(true);
        assert!(plugin.verbose);

        let plugin = LoggerPlugin::new().verbose(false);
        assert!(!plugin.verbose);
    }

    #[test]
    fn test_logger_plugin_log_interval() {
        let plugin = LoggerPlugin::new().log_interval(30);
        assert_eq!(plugin.log_interval, 30);
    }

    #[test]
    fn test_logger_plugin_default() {
        let plugin = LoggerPlugin::default();
        assert_eq!(plugin.verbose, false);
        assert_eq!(plugin.tick_count, 0);
        assert_eq!(plugin.log_interval, 60);
    }

    #[test]
    fn test_logger_plugin_name() {
        let plugin = LoggerPlugin::new();
        assert_eq!(plugin.name(), "logger");
    }

    #[test]
    fn test_logger_plugin_priority() {
        let plugin = LoggerPlugin::new();
        assert_eq!(plugin.priority(), 100);
    }

    #[test]
    fn test_logger_plugin_tick_increments() {
        let mut plugin = LoggerPlugin::new();
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("logger");

        assert_eq!(plugin.tick_count, 0);
        plugin.on_tick(&mut ctx, Duration::from_millis(16)).unwrap();
        assert_eq!(plugin.tick_count, 1);
        plugin.on_tick(&mut ctx, Duration::from_millis(16)).unwrap();
        assert_eq!(plugin.tick_count, 2);
    }

    #[test]
    fn test_logger_plugin_non_verbose() {
        let mut plugin = LoggerPlugin::new().verbose(false);
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("logger");

        // Should not panic even without verbose logging
        for _ in 0..100 {
            plugin.on_tick(&mut ctx, Duration::from_millis(16)).unwrap();
        }
    }

    // =========================================================================
    // PerformancePlugin tests
    // =========================================================================

    #[test]
    fn test_performance_plugin() {
        let mut plugin = PerformancePlugin::new().max_samples(10);
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("performance");

        plugin.on_init(&mut ctx).unwrap();
        plugin.on_mount(&mut ctx).unwrap();

        // Simulate some frames
        for _ in 0..20 {
            plugin.on_tick(&mut ctx, Duration::from_millis(16)).unwrap();
        }

        let fps = ctx.get_data::<f64>("fps").unwrap();
        assert!(*fps > 0.0);

        plugin.on_unmount(&mut ctx).unwrap();
    }

    #[test]
    fn test_performance_plugin_new() {
        let plugin = PerformancePlugin::new();
        assert_eq!(plugin.max_samples, 120);
        assert_eq!(plugin.report_interval, Duration::from_secs(5));
        assert!(plugin.frame_times.is_empty());
    }

    #[test]
    fn test_performance_plugin_max_samples() {
        let plugin = PerformancePlugin::new().max_samples(50);
        assert_eq!(plugin.max_samples, 50);
    }

    #[test]
    fn test_performance_plugin_report_interval() {
        let plugin = PerformancePlugin::new().report_interval(Duration::from_secs(10));
        assert_eq!(plugin.report_interval, Duration::from_secs(10));
    }

    #[test]
    fn test_performance_plugin_default() {
        let plugin = PerformancePlugin::default();
        assert_eq!(plugin.max_samples, 120);
    }

    #[test]
    fn test_performance_plugin_name() {
        let plugin = PerformancePlugin::new();
        assert_eq!(plugin.name(), "performance");
    }

    #[test]
    fn test_performance_plugin_priority() {
        let plugin = PerformancePlugin::new();
        assert_eq!(plugin.priority(), -100);
    }

    #[test]
    fn test_performance_plugin_calculate_fps_empty() {
        let plugin = PerformancePlugin::new();
        assert_eq!(plugin.calculate_fps(), 0.0);
    }

    #[test]
    fn test_performance_plugin_calculate_fps() {
        let mut plugin = PerformancePlugin::new();
        // 16ms frames = ~62.5 FPS
        for _ in 0..10 {
            plugin.frame_times.push(Duration::from_millis(16));
        }
        let fps = plugin.calculate_fps();
        assert!(fps > 60.0 && fps < 65.0);
    }

    #[test]
    fn test_performance_plugin_frame_time_stats_empty() {
        let plugin = PerformancePlugin::new();
        let (min, max, avg) = plugin.frame_time_stats();
        assert_eq!(min, Duration::ZERO);
        assert_eq!(max, Duration::ZERO);
        assert_eq!(avg, Duration::ZERO);
    }

    #[test]
    fn test_performance_plugin_frame_time_stats() {
        let mut plugin = PerformancePlugin::new();
        plugin.frame_times.push(Duration::from_millis(10));
        plugin.frame_times.push(Duration::from_millis(20));
        plugin.frame_times.push(Duration::from_millis(30));

        let (min, max, avg) = plugin.frame_time_stats();
        assert_eq!(min, Duration::from_millis(10));
        assert_eq!(max, Duration::from_millis(30));
        assert_eq!(avg, Duration::from_millis(20));
    }

    #[test]
    fn test_performance_plugin_respects_max_samples() {
        let mut plugin = PerformancePlugin::new().max_samples(5);
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("performance");

        plugin.on_init(&mut ctx).unwrap();
        plugin.on_mount(&mut ctx).unwrap();

        // Add more than max_samples frames
        for _ in 0..10 {
            plugin.on_tick(&mut ctx, Duration::from_millis(16)).unwrap();
        }

        // Should only keep max_samples
        assert_eq!(plugin.frame_times.len(), 5);
    }

    #[test]
    fn test_performance_plugin_on_init_sets_data() {
        let mut plugin = PerformancePlugin::new();
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("performance");

        plugin.on_init(&mut ctx).unwrap();

        assert_eq!(ctx.get_data::<f64>("fps"), Some(&0.0));
        assert_eq!(ctx.get_data::<f64>("frame_time_ms"), Some(&0.0));
    }

    #[test]
    fn test_performance_plugin_updates_frame_time_ms() {
        let mut plugin = PerformancePlugin::new();
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("performance");

        plugin.on_init(&mut ctx).unwrap();
        plugin.on_mount(&mut ctx).unwrap();
        plugin.on_tick(&mut ctx, Duration::from_millis(20)).unwrap();

        let frame_time = ctx.get_data::<f64>("frame_time_ms").unwrap();
        assert!((*frame_time - 20.0).abs() < 0.01);
    }
}
