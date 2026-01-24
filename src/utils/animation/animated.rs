//! Animated value wrapper

use crate::utils::easing::Easing;
use std::time::Duration;

use super::{Interpolatable, Timer};

/// A value that animates smoothly to targets
#[derive(Clone, Debug)]
pub struct AnimatedValue<T: Interpolatable> {
    /// Current value
    current: T,
    /// Start value of animation
    from: T,
    /// Target value
    to: T,
    /// Timer for animation
    timer: Timer,
    /// Easing function
    easing: Easing,
}

impl<T: Interpolatable> AnimatedValue<T> {
    /// Create a new animated value
    pub fn new(initial: T, duration: Duration) -> Self {
        Self {
            current: initial.clone(),
            from: initial.clone(),
            to: initial,
            timer: Timer::new(duration),
            easing: Easing::OutQuad,
        }
    }

    /// Set easing function
    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Animate to a new target
    pub fn animate_to(&mut self, target: T) {
        self.from = self.current.clone();
        self.to = target;
        self.timer.restart();
    }

    /// Set value immediately (no animation)
    pub fn set(&mut self, value: T) {
        self.current = value.clone();
        self.from = value.clone();
        self.to = value;
        self.timer.reset();
    }

    /// Get current value (and update animation)
    pub fn value(&mut self) -> &T {
        if self.timer.is_running() || !self.timer.is_finished() {
            let t = self.timer.progress_eased(self.easing);
            self.current = self.from.lerp(&self.to, t);
        }
        &self.current
    }

    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        self.timer.is_finished()
    }

    /// Get target value
    pub fn target(&self) -> &T {
        &self.to
    }
}
