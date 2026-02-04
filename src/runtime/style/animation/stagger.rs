//! Staggered animation support for animating multiple elements with delays

use std::time::Duration;

use super::{EasingFn, KeyframeAnimation};

/// Staggered animation helper for animating multiple elements with delays
///
/// # Example
///
/// ```rust,ignore
/// use revue::style::animation::Stagger;
/// use std::time::Duration;
///
/// let stagger = Stagger::new(5, Duration::from_millis(50));
///
/// for i in 0..5 {
///     let delay = stagger.delay_for(i);
///     // Create animation with this delay
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Stagger {
    /// Number of items
    count: usize,
    /// Delay between each item
    delay: Duration,
    /// Starting delay
    start_delay: Duration,
    /// Easing for delay distribution
    easing: Option<EasingFn>,
}

impl Stagger {
    /// Create a new stagger configuration
    pub fn new(count: usize, delay: Duration) -> Self {
        Self {
            count,
            delay,
            start_delay: Duration::ZERO,
            easing: None,
        }
    }

    /// Set starting delay
    pub fn start_delay(mut self, delay: Duration) -> Self {
        self.start_delay = delay;
        self
    }

    /// Set easing for delay distribution
    ///
    /// With easing, earlier items have shorter delays and later items have longer ones
    /// (or vice versa depending on the easing function).
    pub fn easing(mut self, easing: EasingFn) -> Self {
        self.easing = Some(easing);
        self
    }

    /// Get delay for item at index
    pub fn delay_for(&self, index: usize) -> Duration {
        if self.count == 0 {
            return self.start_delay;
        }

        let base_delay = self.delay.as_secs_f64() * index as f64;

        let adjusted = match self.easing {
            Some(ease) => {
                let t = index as f32 / (self.count - 1).max(1) as f32;
                let eased = ease(t);
                self.delay.as_secs_f64() * (self.count - 1) as f64 * eased as f64
            }
            None => base_delay,
        };

        self.start_delay + Duration::from_secs_f64(adjusted)
    }

    /// Get total duration for all items
    pub fn total_duration(&self, item_duration: Duration) -> Duration {
        if self.count == 0 {
            return Duration::ZERO;
        }

        self.delay_for(self.count - 1) + item_duration
    }

    /// Create animations for each item
    pub fn apply<F>(&self, mut create_animation: F) -> Vec<KeyframeAnimation>
    where
        F: FnMut(usize) -> KeyframeAnimation,
    {
        (0..self.count)
            .map(|i| {
                let mut anim = create_animation(i);
                anim.delay = self.delay_for(i);
                anim
            })
            .collect()
    }
}
