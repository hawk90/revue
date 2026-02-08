//! Frame ticker

use std::time::{Duration, Instant};

/// Frame rate tracker and delta time calculator
#[derive(Clone, Debug)]
pub struct Ticker {
    last_tick: Option<Instant>,
    frame_count: u64,
    fps_update_time: Instant,
    fps: f64,
    target_fps: Option<f64>,
}

impl Default for Ticker {
    fn default() -> Self {
        Self::new()
    }
}

impl Ticker {
    /// Create a new ticker
    pub fn new() -> Self {
        Self {
            last_tick: None,
            frame_count: 0,
            fps_update_time: Instant::now(),
            fps: 0.0,
            target_fps: None,
        }
    }

    /// Create a ticker with target FPS
    pub fn with_target_fps(fps: f64) -> Self {
        Self {
            target_fps: Some(fps),
            ..Self::new()
        }
    }

    /// Tick and get delta time in seconds
    pub fn tick(&mut self) -> f64 {
        let now = Instant::now();
        let dt = match self.last_tick {
            Some(last) => (now - last).as_secs_f64(),
            None => 1.0 / 60.0, // Default to 60fps on first tick
        };
        self.last_tick = Some(now);
        self.frame_count += 1;

        // Update FPS every second
        let elapsed = now - self.fps_update_time;
        if elapsed >= Duration::from_secs(1) {
            self.fps = self.frame_count as f64 / elapsed.as_secs_f64();
            self.frame_count = 0;
            self.fps_update_time = now;
        }

        // Clamp to reasonable range
        dt.clamp(0.001, 0.1)
    }

    /// Get current FPS
    pub fn fps(&self) -> f64 {
        self.fps
    }

    /// Get frame duration for target FPS
    pub fn frame_duration(&self) -> Duration {
        match self.target_fps {
            Some(fps) => Duration::from_secs_f64(1.0 / fps),
            None => Duration::from_secs_f64(1.0 / 60.0),
        }
    }

    /// Get time since last tick
    pub fn elapsed_since_tick(&self) -> Duration {
        match self.last_tick {
            Some(last) => Instant::now() - last,
            None => Duration::ZERO,
        }
    }

    /// Check if enough time has passed for next frame
    pub fn should_render(&self) -> bool {
        match self.target_fps {
            Some(fps) => {
                let frame_duration = Duration::from_secs_f64(1.0 / fps);
                self.elapsed_since_tick() >= frame_duration
            }
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_new() {
        let ticker = Ticker::new();
        assert_eq!(ticker.frame_count, 0);
        assert_eq!(ticker.fps(), 0.0);
        assert!(ticker.last_tick.is_none());
    }

    #[test]
    fn test_ticker_default() {
        let ticker = Ticker::default();
        assert!(ticker.last_tick.is_none());
    }

    #[test]
    fn test_ticker_with_target_fps() {
        let ticker = Ticker::with_target_fps(30.0);
        assert_eq!(ticker.target_fps, Some(30.0));
    }

    #[test]
    fn test_ticker_tick() {
        let mut ticker = Ticker::new();
        let dt = ticker.tick();
        assert!(dt > 0.0);
        assert!(dt < 0.1);
        assert!(ticker.frame_count == 1);
        assert!(ticker.last_tick.is_some());
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
    fn test_ticker_fps_updates() {
        let mut ticker = Ticker::new();
        ticker.tick();
        assert_eq!(ticker.fps(), 0.0); // Not updated yet

        // After many ticks, fps should increase
        for _ in 0..1000 {
            ticker.tick();
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        let fps = ticker.fps();
        assert!(fps > 0.0);
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
        assert!(ticker.elapsed_since_tick() == Duration::ZERO);

        ticker.tick();
        let elapsed = ticker.elapsed_since_tick();
        assert!(elapsed > Duration::ZERO);
        assert!(elapsed < Duration::from_secs(1));
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

    #[test]
    fn test_ticker_frame_count_increments() {
        let mut ticker = Ticker::new();
        assert_eq!(ticker.frame_count, 0);
        ticker.tick();
        assert_eq!(ticker.frame_count, 1);
        ticker.tick();
        assert_eq!(ticker.frame_count, 2);
    }

    #[test]
    fn test_ticker_first_tick_default_dt() {
        let mut ticker = Ticker::new();
        let dt = ticker.tick();
        // First tick defaults to 60fps (1/60 ≈ 0.0167)
        assert!((dt - 1.0 / 60.0).abs() < 0.001);
    }
}
