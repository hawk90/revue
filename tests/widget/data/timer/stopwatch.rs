//! Stopwatch tests

use revue::style::Color;
use revue::widget::data::timer::{Stopwatch, TimerFormat};

#[test]
fn test_stopwatch_basic() {
    let sw = Stopwatch::new();

    assert_eq!(sw.elapsed_millis(), 0);
    assert_eq!(sw.elapsed_seconds(), 0.0);
    assert!(!sw.is_running());
}

#[test]
fn test_stopwatch_format_elapsed() {
    let sw = Stopwatch::new();
    // format_elapsed uses internal state
    assert!(sw.format_elapsed().contains(":"));
}

#[test]
fn test_stopwatch_state_transitions() {
    let mut sw = Stopwatch::new();

    // Initially not running
    assert!(!sw.is_running());

    sw.start();
    assert!(sw.is_running());

    sw.pause();
    assert!(!sw.is_running());
}

#[test]
fn test_stopwatch_toggle() {
    let mut sw = Stopwatch::new();

    sw.toggle();
    assert!(sw.is_running());

    sw.toggle();
    assert!(!sw.is_running());
}

#[test]
fn test_stopwatch_stop() {
    let mut sw = Stopwatch::new();
    sw.start();

    sw.stop();
    assert!(!sw.is_running());
    assert_eq!(sw.elapsed_millis(), 0);
}

#[test]
fn test_stopwatch_reset() {
    let mut sw = Stopwatch::new();
    sw.start();
    sw.lap(); // Add a lap first

    sw.reset();
    assert_eq!(sw.elapsed_millis(), 0);
    assert!(sw.laps().is_empty());
}

#[test]
fn test_stopwatch_laps() {
    let mut sw = Stopwatch::new();
    // Start and update to add laps
    sw.start();
    sw.update();
    sw.lap(); // Can add lap through the public method

    let laps = sw.laps();
    assert!(!laps.is_empty());
}

#[test]
fn test_stopwatch_builder() {
    // Test builder pattern - we can't directly access private fields
    // but we can verify the stopwatch was created successfully
    let _sw = Stopwatch::new()
        .format(TimerFormat::Precise)
        .show_laps(false)
        .max_laps(10)
        .fg(Color::MAGENTA)
        .title("Test Stopwatch")
        .large_digits(true);

    // If we get here without panicking, the builder works
}

mod snapshots {

    use revue::testing::{Pilot, TestApp, TestConfig};

    #[test]
    fn test_stopwatch_basic() {
        use revue::widget::Stopwatch;

        let view = Stopwatch::new();

        let config = TestConfig::with_size(30, 5);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("stopwatch_basic");
    }
}
