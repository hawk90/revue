//! Performance profiler tests

use revue::utils::profiler::{profile, FlameNode, Profiler, Stats};
use std::thread;
use std::time::Duration;

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
