//! Keyframe-based animation

use super::Interpolatable;
use crate::utils::easing::Easing;

/// A single keyframe in an animation
#[derive(Clone, Debug)]
pub struct Keyframe<T: Interpolatable> {
    /// Time position (0.0 to 1.0)
    pub time: f64,
    /// Value at this keyframe
    pub value: T,
    /// Easing function to use when transitioning TO this keyframe
    pub easing: Easing,
}

impl<T: Interpolatable> Keyframe<T> {
    /// Create a new keyframe
    pub fn new(time: f64, value: T) -> Self {
        Self {
            time: time.clamp(0.0, 1.0),
            value,
            easing: Easing::Linear,
        }
    }

    /// Set easing function
    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }
}

/// Keyframe-based animation
#[derive(Clone, Debug)]
pub struct Keyframes<T: Interpolatable> {
    keyframes: Vec<Keyframe<T>>,
}

impl<T: Interpolatable> Default for Keyframes<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Interpolatable> Keyframes<T> {
    /// Create new empty keyframes
    pub fn new() -> Self {
        Self {
            keyframes: Vec::new(),
        }
    }

    /// Add a keyframe
    pub fn add(mut self, time: f64, value: T) -> Self {
        self.keyframes.push(Keyframe::new(time, value));
        self.keyframes
            .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        self
    }

    /// Add a keyframe with easing
    pub fn add_eased(mut self, time: f64, value: T, easing: Easing) -> Self {
        self.keyframes
            .push(Keyframe::new(time, value).easing(easing));
        self.keyframes
            .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        self
    }

    /// Get number of keyframes
    pub fn len(&self) -> usize {
        self.keyframes.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.keyframes.is_empty()
    }

    /// Get value at time (0.0 to 1.0)
    pub fn at(&self, t: f64) -> Option<T> {
        if self.keyframes.is_empty() {
            return None;
        }

        let t = t.clamp(0.0, 1.0);

        // Find surrounding keyframes
        let mut prev = &self.keyframes[0];
        let mut next = &self.keyframes[self.keyframes.len() - 1];

        for kf in &self.keyframes {
            if kf.time <= t {
                prev = kf;
            }
            if kf.time >= t {
                next = kf;
                break;
            }
        }

        if prev.time >= next.time {
            return Some(prev.value.clone());
        }

        // Calculate local progress between keyframes
        let local_t = (t - prev.time) / (next.time - prev.time);
        let eased_t = next.easing.ease(local_t);

        Some(prev.value.lerp(&next.value, eased_t))
    }
}
