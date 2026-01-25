//! Click detection for single, double, and triple clicks
//!
//! Provides a simple detector for distinguishing between single, double, and triple clicks
//! based on timing and position.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::event::click::{ClickDetector, ClickType};
//! use revue::event::MouseEvent;
//!
//! let mut detector = ClickDetector::new();
//!
//! // Process mouse events
//! if let Some(click_type) = detector.handle_click(x, y) {
//!     match click_type {
//!         ClickType::Single => println!("Single click"),
//!         ClickType::Double => println!("Double click"),
//!         ClickType::Triple => println!("Triple click"),
//!     }
//! }
//! ```

use std::time::{Duration, Instant};

/// Type of click detected
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickType {
    /// Single click
    Single,
    /// Double click
    Double,
    /// Triple click
    Triple,
}

impl ClickType {
    /// Get the click count
    pub fn count(&self) -> u8 {
        match self {
            Self::Single => 1,
            Self::Double => 2,
            Self::Triple => 3,
        }
    }
}

/// Click detector for distinguishing single, double, and triple clicks
///
/// Tracks click timing and position to determine if a series of rapid clicks
/// should be counted as a single, double, or triple click.
#[derive(Debug, Clone)]
pub struct ClickDetector {
    /// Last click time
    last_click: Option<(Instant, u16, u16)>,
    /// Consecutive click count
    click_count: u8,
    /// Maximum time between clicks for multi-click detection
    double_click_threshold: Duration,
    /// Maximum distance between clicks for multi-click detection
    max_distance: u16,
}

impl Default for ClickDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl ClickDetector {
    /// Create a new click detector with default thresholds
    ///
    /// Default thresholds:
    /// - Double click time: 500ms (platform standard)
    /// - Max distance: 5 pixels
    pub fn new() -> Self {
        Self {
            last_click: None,
            click_count: 0,
            double_click_threshold: Duration::from_millis(500),
            max_distance: 5,
        }
    }

    /// Create with custom double-click threshold
    pub fn with_threshold(threshold: Duration) -> Self {
        Self {
            double_click_threshold: threshold,
            ..Self::new()
        }
    }

    /// Create with custom max distance
    pub fn with_max_distance(distance: u16) -> Self {
        Self {
            max_distance: distance,
            ..Self::new()
        }
    }

    /// Set the double-click threshold
    pub fn set_threshold(&mut self, threshold: Duration) {
        self.double_click_threshold = threshold;
    }

    /// Set the max distance between clicks
    pub fn set_max_distance(&mut self, distance: u16) {
        self.max_distance = distance;
    }

    /// Get the current click count (without triggering detection)
    pub fn click_count(&self) -> u8 {
        self.click_count
    }

    /// Reset the click detector
    pub fn reset(&mut self) {
        self.last_click = None;
        self.click_count = 0;
    }

    /// Handle a mouse click event and detect click type
    ///
    /// Returns the click type if a complete click sequence is detected,
    /// or `None` if still waiting for more clicks or if the click sequence
    /// has timed out.
    ///
    /// # Click Detection Logic
    ///
    /// - **Single click**: Detected after timeout with no subsequent click
    /// - **Double click**: Two clicks within threshold time and distance
    /// - **Triple click**: Three clicks within threshold time and distance
    ///
    /// # Returns
    ///
    /// - `Some(ClickType::Single)` - Single click detected (timeout expired)
    /// - `Some(ClickType::Double)` - Double click detected (2nd click)
    /// - `Some(ClickType::Triple)` - Triple click detected (3rd click)
    /// - `None` - Waiting for more clicks or click sequence reset
    pub fn handle_click(&mut self, x: u16, y: u16) -> Option<ClickType> {
        let now = Instant::now();

        if let Some((last_time, last_x, last_y)) = self.last_click {
            // Check if within time threshold
            if now.duration_since(last_time) > self.double_click_threshold {
                // Too much time elapsed, start new sequence
                self.click_count = 1;
                self.last_click = Some((now, x, y));
                return None;
            }

            // Check if within distance threshold
            let dx = x.abs_diff(last_x);
            let dy = y.abs_diff(last_y);

            if dx > self.max_distance || dy > self.max_distance {
                // Too far away, start new sequence
                self.click_count = 1;
                self.last_click = Some((now, x, y));
                return None;
            }

            // Within thresholds, increment count
            self.click_count += 1;
            self.last_click = Some((now, x, y));

            match self.click_count {
                1 => None,
                2 => Some(ClickType::Double),
                3 => {
                    // Reset after triple click
                    self.click_count = 0;
                    Some(ClickType::Triple)
                }
                _ => {
                    // More than 3 clicks, reset and treat as single
                    self.click_count = 1;
                    None
                }
            }
        } else {
            // First click
            self.click_count = 1;
            self.last_click = Some((now, x, y));
            None
        }
    }

    /// Check if a single click timeout has expired
    ///
    /// Call this periodically (e.g., on tick) to detect when a single click
    /// should be emitted after no subsequent clicks occur.
    ///
    /// Returns `Some(ClickType::Single)` if the timeout has expired, `None` otherwise.
    pub fn check_timeout(&mut self) -> Option<ClickType> {
        if self.click_count == 1 {
            if let Some((last_time, _, _)) = self.last_click {
                if last_time.elapsed() > self.double_click_threshold {
                    // Timeout expired, emit single click
                    let result = Some(ClickType::Single);
                    self.reset();
                    return result;
                }
            }
        }
        None
    }

    /// Get the time until next click timeout
    ///
    /// Returns `Some(duration)` if waiting for timeout, `None` if not.
    pub fn time_until_timeout(&self) -> Option<Duration> {
        if self.click_count == 1 {
            if let Some((last_time, _, _)) = self.last_click {
                let elapsed = last_time.elapsed();
                if elapsed < self.double_click_threshold {
                    return Some(self.double_click_threshold - elapsed);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_click() {
        let mut detector = ClickDetector::new();

        // First click returns None (waiting for potential double click)
        assert_eq!(detector.handle_click(10, 10), None);
        assert_eq!(detector.click_count(), 1);

        // After timeout, should detect single click
        std::thread::sleep(detector.double_click_threshold + Duration::from_millis(50));
        assert_eq!(detector.check_timeout(), Some(ClickType::Single));
    }

    #[test]
    fn test_double_click() {
        let mut detector = ClickDetector::new();

        // First click
        assert_eq!(detector.handle_click(10, 10), None);

        // Second click quickly
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(detector.handle_click(10, 10), Some(ClickType::Double));
    }

    #[test]
    fn test_triple_click() {
        let mut detector = ClickDetector::new();

        // First click
        assert_eq!(detector.handle_click(10, 10), None);

        // Second click quickly
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(detector.handle_click(10, 10), Some(ClickType::Double));

        // Third click quickly
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(detector.handle_click(10, 10), Some(ClickType::Triple));
    }

    #[test]
    fn test_click_timeout_between_clicks() {
        let mut detector = ClickDetector::with_threshold(Duration::from_millis(200));

        // First click
        assert_eq!(detector.handle_click(10, 10), None);

        // Wait too long
        std::thread::sleep(Duration::from_millis(250));

        // Second click should start new sequence
        assert_eq!(detector.handle_click(10, 10), None);
        assert_eq!(detector.click_count(), 1);
    }

    #[test]
    fn test_click_distance_limit() {
        let mut detector = ClickDetector::with_max_distance(3);

        // First click
        assert_eq!(detector.handle_click(10, 10), None);

        // Second click too far away
        assert_eq!(detector.handle_click(20, 20), None);
        assert_eq!(detector.click_count(), 1);
    }

    #[test]
    fn test_reset() {
        let mut detector = ClickDetector::new();

        detector.handle_click(10, 10);
        assert_eq!(detector.click_count(), 1);

        detector.reset();
        assert_eq!(detector.click_count(), 0);
        assert!(detector.last_click.is_none());
    }

    #[test]
    fn test_click_type_count() {
        assert_eq!(ClickType::Single.count(), 1);
        assert_eq!(ClickType::Double.count(), 2);
        assert_eq!(ClickType::Triple.count(), 3);
    }

    #[test]
    fn test_four_clicks_resets() {
        let mut detector = ClickDetector::new();

        // Three clicks
        detector.handle_click(10, 10);
        std::thread::sleep(Duration::from_millis(50));
        detector.handle_click(10, 10);
        std::thread::sleep(Duration::from_millis(50));
        assert_eq!(detector.handle_click(10, 10), Some(ClickType::Triple));

        // Fourth click should reset and not return anything
        std::thread::sleep(Duration::from_millis(50));
        assert_eq!(detector.handle_click(10, 10), None);
        assert_eq!(detector.click_count(), 1);
    }
}
