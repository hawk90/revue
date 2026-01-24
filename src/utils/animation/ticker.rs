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
