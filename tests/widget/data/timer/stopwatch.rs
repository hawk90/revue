//! Stopwatch tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::timer::{Stopwatch, TimerFormat};
use revue::widget::traits::{RenderContext, View};

/// `view` drawn in a `width` x `height` area
fn draw(view: &impl View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    view.render(&mut ctx);
    buffer
}

/// Each row of `buffer` as text
fn rows_of(buffer: &Buffer) -> Vec<String> {
    (0..buffer.height())
        .map(|y| {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect()
        })
        .collect()
}

/// Each row of `view` drawn in a `width` x `height` area
fn screen(view: &impl View, width: u16, height: u16) -> Vec<String> {
    rows_of(&draw(view, width, height))
}

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
    assert_eq!(screen(&sw, 20, 3)[1].trim(), "Paused");

    sw.toggle();
    assert!(sw.is_running());
}
#[test]
fn test_stopwatch_stop() {
    let mut sw = Stopwatch::new();
    sw.start();
    sw.lap();

    sw.stop();
    assert!(!sw.is_running());
    assert_eq!(sw.elapsed_millis(), 0);
    assert!(sw.laps().is_empty());
    assert_eq!(screen(&sw, 20, 3)[1].trim(), "Stopped");
}
#[test]
fn test_stopwatch_reset() {
    let mut sw = Stopwatch::new();
    sw.start();
    sw.lap(); // Add a lap first

    sw.reset();
    assert_eq!(sw.elapsed_millis(), 0);
    assert!(sw.laps().is_empty());
    // A running stopwatch keeps running; a paused one stays paused
    assert!(sw.is_running());
    sw.pause();
    sw.reset();
    assert!(!sw.is_running());
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
    let sw = Stopwatch::new()
        .format(TimerFormat::Precise)
        .show_laps(false)
        .max_laps(10)
        .fg(Color::MAGENTA)
        .title("Test Stopwatch")
        .large_digits(true);

    // Title, then three rows of block digits in the fg color, then the state
    let buffer = draw(&sw, 40, 6);
    let rows = rows_of(&buffer);
    assert_eq!(rows[0].trim(), "Test Stopwatch");
    assert!(rows[1].contains('█'), "{rows:#?}");
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(Color::MAGENTA));
    assert_eq!(rows[4].trim(), "Stopped");
}
#[test]
fn test_stopwatch_laps_shown() {
    let mut sw = Stopwatch::new();
    sw.start();
    for _ in 0..7 {
        sw.lap();
    }
    sw.pause();
    // The last max_laps (5) laps, numbered from the start
    let rows = screen(&sw, 20, 10);
    assert_eq!(rows[2].trim(), "Laps:");
    assert!(rows[3].trim().starts_with("#3:"), "{rows:#?}");
    assert!(rows[7].trim().starts_with("#7:"), "{rows:#?}");

    let rows = screen(&sw.clone().max_laps(2), 20, 10);
    assert!(rows[3].trim().starts_with("#6:"), "{rows:#?}");
    assert!(rows[5].trim().is_empty());

    let rows = screen(&sw.show_laps(false), 20, 10);
    assert!(rows.iter().all(|r| !r.contains("Laps")));
}

#[test]
fn test_stopwatch_zero_area_draws_nothing() {
    let mut sw = Stopwatch::new().title("SW");
    sw.start();
    sw.lap();
    super::super::chart::assert_zero_areas_draw_nothing(&sw);
}

#[test]
fn test_stopwatch_draws_the_same_when_offset() {
    let mut sw = Stopwatch::new().title("SW");
    sw.start();
    sw.lap();
    sw.pause();
    super::super::chart::assert_same_when_offset(sw, 20, 6);
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
