//! The clock animations, timers and ticks read: `MockTime::install` moves all
//! of them without a sleep, one thread at a time.

use revue::event::{ClickDetector, ClickType};
use revue::patterns::{MessageState, SearchState};
use revue::state::tasks::Timer as TaskTimer;
use revue::style::{KeyframeAnimation, Tween};
use revue::testing::MockTime;
use revue::utils::animation::{Ticker, Timer as AnimationTimer};
use revue::utils::clock;
use revue::utils::debounce::{Debouncer, Throttle};
use revue::utils::EditState;
use revue::widget::traits::Timeout;
use revue::widget::{Stopwatch, Timer, ToastQueue};
use std::time::{Duration, Instant};

const MS: fn(u64) -> Duration = Duration::from_millis;

#[test]
fn the_clock_reads_real_time_without_a_mock() {
    let before = Instant::now();
    let read = clock::now();
    assert!(read >= before && read <= Instant::now());
}

#[test]
fn a_mock_moves_only_when_advanced() {
    let time = MockTime::new();
    let _clock = time.install();
    let start = clock::now();
    std::thread::sleep(MS(5));
    assert_eq!(clock::now(), start, "real time must not move the mock");
    time.advance_ms(1);
    assert_eq!(clock::since(start), MS(1));
}

#[test]
fn since_a_later_instant_is_zero() {
    let time = MockTime::new();
    let _clock = time.install();
    let later = clock::now() + MS(10);
    assert_eq!(clock::since(later), Duration::ZERO);
}

#[test]
fn installing_never_moves_time_backward() {
    let held = clock::now();
    let time = MockTime::new();
    time.advance_ms(60_000);
    let _clock = time.install();
    assert!(clock::now() >= held);
    let start = clock::now();
    time.advance_ms(7);
    assert_eq!(clock::since(start), MS(7));
}

#[test]
fn dropping_restores_the_clock_before_it() {
    let outer = MockTime::new();
    let _outer = outer.install();
    let start = clock::now();
    {
        let inner = MockTime::new();
        let _inner = inner.install();
        let inner_start = clock::now();
        inner.advance_ms(50);
        outer.advance_ms(1);
        assert_eq!(clock::since(inner_start), MS(50), "the inner clock rules");
    }
    assert_eq!(clock::since(start), MS(1), "the outer clock is back");
    drop(_outer);
    let real = clock::now();
    std::thread::sleep(MS(2));
    assert!(clock::since(real) >= MS(2), "real time is back");
}

#[test]
fn another_thread_keeps_real_time() {
    let time = MockTime::new();
    let _clock = time.install();
    let moved = std::thread::spawn(|| {
        let start = clock::now();
        std::thread::sleep(MS(2));
        clock::since(start) >= MS(2)
    })
    .join()
    .unwrap();
    assert!(moved);
}

#[test]
fn clones_of_a_mock_advance_the_same_clock() {
    let time = MockTime::new();
    let _clock = time.install();
    let start = clock::now();
    time.clone().advance_ms(3);
    assert_eq!(clock::since(start), MS(3));
}

#[test]
fn a_tween_runs_on_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut tween = Tween::new(0.0, 100.0, MS(1000));
    tween.start();
    assert_eq!(tween.value(), 0.0);
    time.advance_ms(500);
    assert_eq!(tween.value(), 50.0);
    assert!(!tween.is_completed());
    time.advance_ms(500);
    assert_eq!(tween.value(), 100.0);
    assert!(tween.is_completed());
}

#[test]
fn an_animation_timer_runs_on_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut t = AnimationTimer::new(MS(100));
    t.start();
    time.advance_ms(99);
    assert!(!t.is_finished());
    time.advance_ms(1);
    assert!(t.is_finished());
}

#[test]
fn a_toast_expires_on_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut queue = ToastQueue::new().default_duration(MS(100));
    queue.info("hello");
    queue.tick();
    assert_eq!(queue.visible_count(), 1);
    time.advance_ms(99);
    queue.tick();
    assert_eq!(queue.visible_count(), 1, "one millisecond early");
    time.advance_ms(1);
    queue.tick();
    assert_eq!(queue.visible_count(), 0);
}

#[test]
fn a_countdown_runs_on_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut t = Timer::countdown(2);
    t.start();
    time.advance_ms(1000);
    t.update();
    assert_eq!(t.remaining_seconds(), 1);
    time.advance_ms(1000);
    t.update();
    assert!(t.is_completed());
}

#[test]
fn a_stopwatch_runs_on_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut sw = Stopwatch::new();
    sw.start();
    time.advance_ms(1234);
    sw.update();
    assert_eq!(sw.elapsed_millis(), 1234);
}

#[test]
fn a_debouncer_fires_on_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut d = Debouncer::new(MS(100));
    d.call();
    time.advance_ms(99);
    assert!(!d.is_ready());
    time.advance_ms(1);
    assert!(d.is_ready());
}

#[test]
fn a_throttle_opens_on_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut t = Throttle::new(MS(100));
    assert!(t.call());
    time.advance_ms(99);
    assert!(!t.call());
    time.advance_ms(1);
    assert!(t.call());
}

#[test]
fn a_message_times_out_on_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut m = MessageState::with_duration(MS(100));
    m.set("hi".to_string());
    time.advance_ms(99);
    assert!(!m.check_timeout());
    time.advance_ms(1);
    assert!(m.check_timeout());
    assert!(!m.has_message());
}

#[test]
fn double_clicks_use_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut d = ClickDetector::with_threshold(MS(100));
    assert_eq!(d.handle_click(1, 1), None);
    time.advance_ms(100);
    assert_eq!(
        d.handle_click(1, 1),
        Some(ClickType::Double),
        "at the limit"
    );
    d.reset();
    assert_eq!(d.handle_click(1, 1), None);
    time.advance_ms(100);
    assert_eq!(d.check_timeout(), None, "at the limit it can still double");
    time.advance_ms(1);
    assert_eq!(d.check_timeout(), Some(ClickType::Single));
}

#[test]
fn a_keyframe_animation_runs_on_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut anim = KeyframeAnimation::new("fade")
        .keyframe(0, |k| k.set("opacity", 0.0))
        .keyframe(100, |k| k.set("opacity", 1.0))
        .duration(MS(1000));
    anim.start();
    time.advance_ms(500);
    assert_eq!(anim.progress(), 0.5);
    assert!(!anim.is_completed());
    time.advance_ms(500);
    anim.get("opacity");
    assert!(anim.is_completed());
}

#[test]
fn a_ticker_measures_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut ticker = Ticker::new();
    ticker.tick();
    time.advance_ms(50);
    assert_eq!(ticker.elapsed_since_tick(), MS(50));
    assert_eq!(ticker.tick(), 0.05);
}

#[test]
fn a_timeout_expires_on_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut timeout = Timeout::new(MS(100));
    timeout.set("value");
    time.advance_ms(100);
    assert!(
        !timeout.is_expired(),
        "expires after the duration, not at it"
    );
    time.advance_ms(1);
    assert!(timeout.is_expired());
}

#[test]
fn a_task_timer_expires_on_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut timers = TaskTimer::new();
    timers.set("once", MS(100));
    assert_eq!(timers.remaining("once"), Some(MS(100)));
    time.advance_ms(99);
    assert_eq!(timers.poll_expired(), None);
    time.advance_ms(1);
    assert_eq!(timers.poll_expired(), Some("once"));
}

#[test]
fn a_search_debounces_on_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut search = SearchState::new();
    search.set_query("a");
    assert!(!search.is_ready());
    time.advance_ms(60_000);
    assert!(search.is_ready());
}

#[test]
fn undo_groups_typing_by_the_mock() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut edit = EditState::single_line();
    edit.insert("a");
    time.advance_ms(100);
    edit.insert("b");
    time.advance_ms(60_000);
    edit.insert("c");
    edit.undo();
    assert_eq!(edit.text(), "ab", "a pause starts a new step");
    edit.undo();
    assert_eq!(edit.text(), "", "quick typing is one step");
}

/// Files that keep the real time, and why. Everything else reads
/// `utils::clock`, so a new `Instant::now` needs a reason here.
const REAL_TIME: &[(&str, &str)] = &[
    ("src/utils/clock.rs", "the clock itself"),
    ("src/runtime/event/reader.rs", "waits on the real terminal"),
    (
        "src/core/app/hot_reload/mod.rs",
        "debounces real file events",
    ),
    ("src/core/app/profiler.rs", "measures real work"),
    ("src/utils/profiler.rs", "measures real work"),
    ("src/devtools/profiler/core.rs", "measures real work"),
    ("src/devtools/profiler/types.rs", "measures real work"),
    ("src/devtools/events/core.rs", "stamps real events"),
    ("src/devtools/events/types.rs", "stamps real events"),
    (
        "src/devtools/time_travel/debugger/mod.rs",
        "stamps real snapshots",
    ),
    (
        "src/widget/debug_overlay/metrics.rs",
        "measures real frames",
    ),
    (
        "src/widget/debug_overlay/event_log.rs",
        "stamps real events",
    ),
    (
        "src/widget/developer/httpclient/exchange.rs",
        "times a real request",
    ),
    ("src/runtime/event/custom/types.rs", "stamps real events"),
    ("src/state/tasks/runner.rs", "waits on real threads"),
    ("src/state/worker/pool.rs", "waits on real threads"),
    ("src/state/worker/handle.rs", "waits on real threads"),
    ("src/utils/clipboard/system.rs", "waits on real processes"),
    (
        "src/utils/accessibility/announcement.rs",
        "stamps real announcements",
    ),
    (
        "src/a11y/backend/core.rs",
        "rate-limits a real screen reader",
    ),
    ("src/a11y/backend/platform.rs", "stamps real announcements"),
    (
        "src/testing/pilot/core.rs",
        "a harness waiting in real time",
    ),
    ("src/testing/ci/report.rs", "reports a real run"),
];

fn sources(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// The part of a source file that is not a test module or a comment
fn code(source: &str) -> String {
    let mut kept = String::new();
    let mut lines = source.lines().peekable();
    while let Some(line) = lines.next() {
        if line.trim() == "#[cfg(test)]"
            && lines
                .peek()
                .is_some_and(|next| next.trim_start().starts_with("mod "))
            && lines
                .peek()
                .is_some_and(|next| next.trim_end().ends_with('{'))
        {
            let indent = line.len() - line.trim_start().len();
            let close = format!("{}}}", " ".repeat(indent));
            for inner in lines.by_ref() {
                if inner == close {
                    break;
                }
            }
            continue;
        }
        if !line.trim_start().starts_with("//") {
            kept.push_str(line);
            kept.push('\n');
        }
    }
    kept
}

#[test]
fn only_the_listed_files_read_real_time() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    sources(&root.join("src"), &mut files);
    let mut unlisted = Vec::new();
    let mut used = Vec::new();
    for file in files {
        let name = file
            .strip_prefix(root)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        if name.ends_with("/tests.rs")
            || !code(&std::fs::read_to_string(&file).unwrap()).contains("Instant::now")
        {
            continue;
        }
        if REAL_TIME.iter().any(|(listed, _)| *listed == name) {
            used.push(name);
        } else {
            unlisted.push(name);
        }
    }
    assert!(
        unlisted.is_empty(),
        "read revue::utils::clock, or list the file in REAL_TIME with why: {unlisted:?}"
    );
    let stale: Vec<_> = REAL_TIME
        .iter()
        .map(|(listed, _)| *listed)
        .filter(|listed| !used.iter().any(|name| name == listed))
        .collect();
    assert!(
        stale.is_empty(),
        "these no longer read real time: {stale:?}"
    );
}
