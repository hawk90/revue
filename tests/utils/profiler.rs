//! Performance profiler tests

#[allow(deprecated)] // FlameNode is deprecated since 3.17 (#1208)
use revue::utils::profiler::FlameNode;
use revue::utils::profiler::{profile, Profiler, Stats, Timing};
use revue::utils::{display_width, truncate_to_width};
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
fn test_profiler_report_truncates_long_non_ascii_name() {
    let profiler = Profiler::new();
    // 31 two-byte characters: byte 29 is inside the fifteenth one
    let name = "é".repeat(31);
    profiler.record(&name, Duration::from_millis(10));

    let report = profiler.report();
    let expected = format!("{}… ", "é".repeat(29));
    assert!(report.contains(&expected), "{report}");
    assert!(!report.contains(&name));
}

#[test]
fn test_profiler_report_truncates_long_ascii_name() {
    let profiler = Profiler::new();
    let name = "a".repeat(31);
    profiler.record(&name, Duration::from_millis(10));

    let report = profiler.report();
    assert!(report.contains(&format!("{}… ", "a".repeat(29))));
    assert!(!report.contains(&name));
}

/// The report's header and its one row, for a profiler with one operation
fn header_and_row(name: &str) -> (String, String) {
    let profiler = Profiler::new();
    profiler.record(name, Duration::from_millis(10));
    let report = profiler.report();
    let lines: Vec<&str> = report.lines().collect();
    assert_eq!(lines.len(), 6, "{report}");
    (lines[3].to_string(), lines[5].to_string())
}

/// The calls column of a row: the 8 columns after the 30-column name and a space
fn calls_column(row: &str) -> &str {
    let name_and_space = truncate_to_width(row, 31);
    assert_eq!(display_width(name_and_space), 31, "{row:?}");
    &row[name_and_space.len()..name_and_space.len() + 8]
}

/// The separator was a fixed 80 columns under an 83-column header (#1534)
#[test]
fn test_profiler_report_separator_spans_the_header() {
    let profiler = Profiler::new();
    profiler.record("a", Duration::from_millis(1));
    let report = profiler.report();
    let lines: Vec<&str> = report.lines().collect();
    assert_eq!(lines.len(), 6, "{report}");
    assert_eq!(display_width(lines[3]), 83, "{report}");
    assert_eq!(lines[4], "-".repeat(83), "{report}");
    assert_eq!(display_width(lines[5]), 83, "{report}");
}

#[test]
fn test_profiler_report_keeps_30_column_wide_name() {
    // 15 Hangul characters: 45 bytes, 30 columns, so it fits the column whole
    let name = "렌".repeat(15);
    let (header, row) = header_and_row(&name);

    assert_eq!(&row[..name.len()], name);
    assert_eq!(calls_column(&row), "       1");
    assert_eq!(calls_column(&header), "   Calls");
    assert_eq!(display_width(&row), display_width(&header));
}

#[test]
fn test_profiler_report_cuts_31_column_wide_name_by_columns() {
    // 31 columns: 14 Hangul characters (28 columns) fit before the "…",
    // and one space pads the name to 30 columns
    let name = format!("{}a", "렌".repeat(15));
    let (header, row) = header_and_row(&name);

    let expected = format!("{}… ", "렌".repeat(14));
    assert_eq!(&row[..expected.len()], expected);
    assert_eq!(calls_column(&row), "       1");
    assert_eq!(display_width(&row), display_width(&header));
}

#[test]
#[allow(deprecated)] // FlameNode is deprecated since 3.17 (#1208)
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

// The profiler keeps flat per-name stats: a guard opened inside another one
// records under its own name only, and the outer one's time includes it (#1208)
#[test]
fn nested_guards_record_flat_per_name_stats() {
    let profiler = Profiler::new();
    {
        let _outer = profiler.start("outer");
        {
            let _inner = profiler.start("inner");
            thread::sleep(Duration::from_millis(5));
        }
        let _second = profiler.start("inner");
    }

    let all = profiler.all_stats();
    assert_eq!(all.len(), 2);
    let outer = &all["outer"];
    let inner = &all["inner"];
    assert_eq!(outer.count, 1);
    assert_eq!(inner.count, 2);
    assert!(outer.total >= inner.total);
}

// Guards that drop out of opening order still record each name once (#1208)
#[test]
fn guards_dropped_out_of_order_each_record_once() {
    let profiler = Profiler::new();
    let first = profiler.start("first");
    let second = profiler.start("second");
    drop(first);
    drop(second);

    assert_eq!(profiler.stats("first").unwrap().count, 1);
    assert_eq!(profiler.stats("second").unwrap().count, 1);
    assert_eq!(profiler.all_stats().len(), 2);
}

#[test]
#[allow(deprecated)] // Timing::parent is deprecated since 3.17 (#1208)
fn timing_has_no_parent_unless_set() {
    let start = std::time::Instant::now();
    let timing = Timing::new("render", Duration::from_millis(3), start);
    assert_eq!(timing.name, "render");
    assert_eq!(timing.duration, Duration::from_millis(3));
    assert_eq!(timing.parent, None);

    let timing = timing.with_parent("frame");
    assert_eq!(timing.parent.as_deref(), Some("frame"));
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
