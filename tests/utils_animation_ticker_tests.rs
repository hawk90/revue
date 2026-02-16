//! Tests for animation ticker module
//!
//! Extracted from src/utils/animation/ticker.rs

use revue::utils::animation::Ticker;

#[test]
fn test_ticker_new() {
    let ticker = Ticker::new();
    assert_eq!(ticker.fps(), 0.0);
    assert_eq!(ticker.elapsed_since_tick(), std::time::Duration::ZERO);
}

#[test]
fn test_ticker_default() {
    let ticker = Ticker::default();
    assert_eq!(ticker.elapsed_since_tick(), std::time::Duration::ZERO);
}

#[test]
fn test_ticker_with_target_fps() {
    let ticker = Ticker::with_target_fps(30.0);
    let duration = ticker.frame_duration();
    // Verify target FPS affects frame duration
    assert!((duration.as_secs_f64() - 1.0 / 30.0).abs() < 0.0001);
}

#[test]
fn test_ticker_tick() {
    let mut ticker = Ticker::new();
    let dt = ticker.tick();
    assert!(dt > 0.0);
    assert!(dt < 0.1);
    // After tick, elapsed_since_tick should be > ZERO
    assert!(ticker.elapsed_since_tick() > std::time::Duration::ZERO);
}

#[test]
fn test_ticker_tick_returns_clamped() {
    let mut ticker = Ticker::new();
    // Tick returns clamped value
    for _ in 0..10 {
        let dt = ticker.tick();
        assert!(dt >= 0.001 && dt <= 0.1);
    }
}

#[test]
fn test_ticker_frame_duration() {
    let ticker = Ticker::with_target_fps(60.0);
    let duration = ticker.frame_duration();
    assert!((duration.as_secs_f64() - 1.0 / 60.0).abs() < 0.0001);
}

#[test]
fn test_ticker_frame_duration_default() {
    let ticker = Ticker::new();
    let duration = ticker.frame_duration();
    assert!((duration.as_secs_f64() - 1.0 / 60.0).abs() < 0.0001);
}

#[test]
fn test_ticker_elapsed_since_tick() {
    let mut ticker = Ticker::new();
    assert!(ticker.elapsed_since_tick() == std::time::Duration::ZERO);

    ticker.tick();
    let elapsed = ticker.elapsed_since_tick();
    assert!(elapsed > std::time::Duration::ZERO);
    assert!(elapsed < std::time::Duration::from_secs(1));
}

#[test]
fn test_ticker_should_render() {
    let ticker = Ticker::new();
    assert!(ticker.should_render()); // No target fps

    let mut ticker = Ticker::with_target_fps(60.0);
    ticker.tick();
    // Immediately after tick, should not render
    assert!(!ticker.should_render());

    // After enough time, should render
    std::thread::sleep(ticker.frame_duration());
    assert!(ticker.should_render());
}

// Removed test_ticker_frame_count_increments - it accessed private frame_count field

#[test]
fn test_ticker_first_tick_default_dt() {
    let mut ticker = Ticker::new();
    let dt = ticker.tick();
    // First tick defaults to 60fps (1/60 ≈ 0.0167)
    assert!((dt - 1.0 / 60.0).abs() < 0.001);
}
