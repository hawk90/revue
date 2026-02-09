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

#[cfg(test)]
mod tests {
    use super::*;

    // A simple test type that implements Interpolatable
    #[derive(Clone, Debug, PartialEq)]
    struct TestValue(f64);

    impl Interpolatable for TestValue {
        fn lerp(&self, other: &Self, t: f64) -> Self {
            TestValue(self.0 + (other.0 - self.0) * t)
        }
    }

    // =========================================================================
    // Keyframe::new() tests
    // =========================================================================

    #[test]
    fn test_keyframe_new() {
        let kf = Keyframe::new(0.5, TestValue(10.0));
        assert_eq!(kf.time, 0.5);
        assert_eq!(kf.value.0, 10.0);
        assert_eq!(kf.easing, Easing::Linear);
    }

    #[test]
    fn test_keyframe_new_clamps_time_low() {
        let kf = Keyframe::new(-0.5, TestValue(5.0));
        assert_eq!(kf.time, 0.0); // Clamped to 0
    }

    #[test]
    fn test_keyframe_new_clamps_time_high() {
        let kf = Keyframe::new(1.5, TestValue(15.0));
        assert_eq!(kf.time, 1.0); // Clamped to 1
    }

    // =========================================================================
    // Keyframe::easing() tests
    // =========================================================================

    #[test]
    fn test_keyframe_easing() {
        let kf = Keyframe::new(0.5, TestValue(10.0)).easing(Easing::InQuad);
        assert_eq!(kf.easing, Easing::InQuad);
    }

    #[test]
    fn test_keyframe_easing_chainable() {
        let kf = Keyframe::new(0.5, TestValue(10.0))
            .easing(Easing::OutQuad)
            .easing(Easing::InElastic);
        assert_eq!(kf.easing, Easing::InElastic);
    }

    // =========================================================================
    // Keyframes::new() and default tests
    // =========================================================================

    #[test]
    fn test_keyframes_new() {
        let kfs: Keyframes<TestValue> = Keyframes::new();
        assert!(kfs.is_empty());
        assert_eq!(kfs.len(), 0);
    }

    #[test]
    fn test_keyframes_default() {
        let kfs: Keyframes<TestValue> = Keyframes::default();
        assert!(kfs.is_empty());
    }

    // =========================================================================
    // Keyframes::add() tests
    // =========================================================================

    #[test]
    fn test_keyframes_add() {
        let kfs = Keyframes::new()
            .add(0.0, TestValue(0.0))
            .add(0.5, TestValue(50.0))
            .add(1.0, TestValue(100.0));
        assert_eq!(kfs.len(), 3);
    }

    #[test]
    fn test_keyframes_add_sorts() {
        let kfs = Keyframes::new()
            .add(0.5, TestValue(50.0))
            .add(0.0, TestValue(0.0))
            .add(1.0, TestValue(100.0));
        assert_eq!(kfs.keyframes[0].time, 0.0);
        assert_eq!(kfs.keyframes[1].time, 0.5);
        assert_eq!(kfs.keyframes[2].time, 1.0);
    }

    #[test]
    fn test_keyframes_add_duplicate_time() {
        let kfs = Keyframes::new()
            .add(0.5, TestValue(10.0))
            .add(0.5, TestValue(20.0));
        assert_eq!(kfs.len(), 2); // Both kept, order depends on sort stability
    }

    // =========================================================================
    // Keyframes::add_eased() tests
    // =========================================================================

    #[test]
    fn test_keyframes_add_eased() {
        let kfs = Keyframes::new()
            .add_eased(0.0, TestValue(0.0), Easing::Linear)
            .add_eased(1.0, TestValue(100.0), Easing::InOutQuad);
        assert_eq!(kfs.len(), 2);
        assert_eq!(kfs.keyframes[0].easing, Easing::Linear);
        assert_eq!(kfs.keyframes[1].easing, Easing::InOutQuad);
    }

    // =========================================================================
    // Keyframes::len() / is_empty() tests
    // =========================================================================

    #[test]
    fn test_keyframes_len_empty() {
        let kfs: Keyframes<TestValue> = Keyframes::new();
        assert_eq!(kfs.len(), 0);
        assert!(kfs.is_empty());
    }

    #[test]
    fn test_keyframes_len_non_empty() {
        let kfs = Keyframes::new().add(0.5, TestValue(10.0));
        assert_eq!(kfs.len(), 1);
        assert!(!kfs.is_empty());
    }

    // =========================================================================
    // Keyframes::at() tests
    // =========================================================================

    #[test]
    fn test_keyframes_at_empty() {
        let kfs: Keyframes<TestValue> = Keyframes::new();
        assert_eq!(kfs.at(0.5), None);
    }

    #[test]
    fn test_keyframes_at_start() {
        let kfs = Keyframes::new()
            .add(0.0, TestValue(0.0))
            .add(1.0, TestValue(100.0));
        let result = kfs.at(0.0);
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, 0.0);
    }

    #[test]
    fn test_keyframes_at_end() {
        let kfs = Keyframes::new()
            .add(0.0, TestValue(0.0))
            .add(1.0, TestValue(100.0));
        let result = kfs.at(1.0);
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, 100.0);
    }

    #[test]
    fn test_keyframes_at_middle() {
        let kfs = Keyframes::new()
            .add(0.0, TestValue(0.0))
            .add(1.0, TestValue(100.0));
        let result = kfs.at(0.5);
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, 50.0); // Linear, so exactly halfway
    }

    #[test]
    fn test_keyframes_at_before_start() {
        let kfs = Keyframes::new()
            .add(0.5, TestValue(50.0))
            .add(1.0, TestValue(100.0));
        let result = kfs.at(0.0);
        assert!(result.is_some());
        // Clamped to first keyframe value
        assert_eq!(result.unwrap().0, 50.0);
    }

    #[test]
    fn test_keyframes_at_after_end() {
        let kfs = Keyframes::new()
            .add(0.0, TestValue(0.0))
            .add(0.5, TestValue(50.0));
        let result = kfs.at(1.0);
        assert!(result.is_some());
        // Clamped to last keyframe value
        assert_eq!(result.unwrap().0, 50.0);
    }

    #[test]
    fn test_keyframes_at_with_easing() {
        // Easing is applied from the keyframe we're interpolating TO
        let kfs = Keyframes::new()
            .add_eased(0.0, TestValue(0.0), Easing::Linear)
            .add_eased(1.0, TestValue(100.0), Easing::InQuad);
        let result = kfs.at(0.5);
        assert!(result.is_some());
        // InQuad at t=0.5: 0.25, so value should be 25
        assert!((result.unwrap().0 - 25.0).abs() < 0.001);
    }

    #[test]
    fn test_keyframes_at_three_keyframes() {
        let kfs = Keyframes::new()
            .add(0.0, TestValue(0.0))
            .add(0.5, TestValue(50.0))
            .add(1.0, TestValue(100.0));
        let result = kfs.at(0.75);
        assert!(result.is_some());
        // Between 0.5 and 1.0, so 75% of the way
        assert_eq!(result.unwrap().0, 75.0);
    }

    #[test]
    fn test_keyframes_clone() {
        let kfs1 = Keyframes::new()
            .add(0.0, TestValue(0.0))
            .add(1.0, TestValue(100.0));
        let kfs2 = kfs1.clone();
        assert_eq!(kfs1.len(), kfs2.len());
    }
}
