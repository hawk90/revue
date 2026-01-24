//! Animation utilities for terminal UI
//!
//! Provides frame-based animation primitives that work well with terminal
//! render loops. Includes spring physics, keyframes, sequences, and timers.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::animation::{Spring, Keyframes, Timer};
//!
//! // Spring animation for smooth motion
//! let mut spring = Spring::new(0.0, 100.0);
//! loop {
//!     let value = spring.update(dt);
//!     if spring.is_settled() { break; }
//! }
//!
//! // Keyframe animation
//! let anim = Keyframes::new()
//!     .add(0.0, 0.0)
//!     .add(0.5, 100.0)
//!     .add(1.0, 50.0);
//! let value = anim.at(0.25);  // Interpolated between keyframes
//! ```

mod animated;
mod keyframe;
pub mod presets;
mod sequence;
mod spring;
mod ticker;
mod timer;
mod trait_;

pub use animated::AnimatedValue;
pub use keyframe::{Keyframe, Keyframes};
pub use presets::*;
pub use sequence::{Sequence, SequenceStep};
pub use spring::Spring;
pub use ticker::Ticker;
pub use timer::Timer;
pub use trait_::Interpolatable;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    //! Animation tests

    use crate::utils::easing::Easing;
    use std::time::Duration;

    use super::*;

    // =========================================================================
    // Timer Tests
    // =========================================================================

    #[test]
    fn test_timer_new() {
        let timer = Timer::new(Duration::from_secs(1));
        assert!(!timer.is_running());
        assert!(!timer.is_finished());
        assert_eq!(timer.progress(), 0.0);
    }

    #[test]
    fn test_timer_from_millis() {
        let timer = Timer::from_millis(500);
        assert_eq!(timer.duration, Duration::from_millis(500));
    }

    #[test]
    fn test_timer_start() {
        let mut timer = Timer::from_millis(1000);
        timer.start();
        assert!(timer.is_running());
    }

    #[test]
    fn test_timer_pause_resume() {
        let mut timer = Timer::from_millis(1000);
        timer.start();

        std::thread::sleep(Duration::from_millis(10));
        timer.pause();
        let elapsed_paused = timer.elapsed();

        std::thread::sleep(Duration::from_millis(50));
        let elapsed_still_paused = timer.elapsed();

        // Elapsed should not change while paused
        assert_eq!(elapsed_paused, elapsed_still_paused);

        timer.resume();
        assert!(timer.is_running());
    }

    #[test]
    fn test_timer_progress() {
        let timer = Timer::from_millis(0);
        assert_eq!(timer.progress(), 1.0); // Zero duration = finished
    }

    // =========================================================================
    // Spring Tests
    // =========================================================================

    #[test]
    fn test_spring_new() {
        let spring = Spring::new(0.0, 100.0);
        assert_eq!(spring.value(), 0.0);
        assert_eq!(spring.target(), 100.0);
    }

    #[test]
    fn test_spring_at() {
        let spring = Spring::at(50.0);
        assert_eq!(spring.value(), 50.0);
        assert_eq!(spring.target(), 50.0);
        assert!(spring.is_settled());
    }

    #[test]
    fn test_spring_update() {
        let mut spring = Spring::new(0.0, 100.0);
        let dt = 1.0 / 60.0;

        // Spring should move towards target
        let v1 = spring.update(dt);
        assert!(v1 > 0.0);

        // Keep updating
        for _ in 0..200 {
            spring.update(dt);
        }

        // Should eventually settle near target
        assert!(spring.is_settled());
        assert!((spring.value() - 100.0).abs() < 1.0);
    }

    #[test]
    fn test_spring_presets() {
        // Test that presets create valid springs
        let _snappy = Spring::snappy();
        let _gentle = Spring::gentle();
        let _bouncy = Spring::bouncy();

        // Note: We can't test internal stiffness/damping values as they're private
        // The presets are tested indirectly through their behavior in other tests
    }

    #[test]
    fn test_spring_set_target() {
        let mut spring = Spring::at(0.0);
        spring.set_target(100.0);
        assert_eq!(spring.target(), 100.0);
        assert!(!spring.is_settled());
    }

    // =========================================================================
    // Keyframes Tests
    // =========================================================================

    #[test]
    fn test_keyframes_new() {
        let kf: Keyframes<f64> = Keyframes::new();
        assert!(kf.is_empty());
    }

    #[test]
    fn test_keyframes_add() {
        let kf = Keyframes::new().add(0.0, 0.0).add(1.0, 100.0);

        assert_eq!(kf.len(), 2);
    }

    #[test]
    fn test_keyframes_at() {
        let kf = Keyframes::new().add(0.0, 0.0).add(1.0, 100.0);

        assert_eq!(kf.at(0.0), Some(0.0));
        assert_eq!(kf.at(1.0), Some(100.0));

        let mid: f64 = kf.at(0.5).unwrap();
        assert!((mid - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_keyframes_three_points() {
        let kf = Keyframes::new()
            .add(0.0, 0.0)
            .add(0.5, 100.0)
            .add(1.0, 50.0);

        assert_eq!(kf.at(0.0), Some(0.0_f64));
        assert_eq!(kf.at(0.5), Some(100.0_f64));
        assert_eq!(kf.at(1.0), Some(50.0_f64));

        let mid1: f64 = kf.at(0.25).unwrap();
        assert!((mid1 - 50.0).abs() < 0.01);

        let mid2: f64 = kf.at(0.75).unwrap();
        assert!((mid2 - 75.0).abs() < 0.01);
    }

    #[test]
    fn test_keyframes_eased() {
        let kf = Keyframes::new()
            .add(0.0, 0.0)
            .add_eased(1.0, 100.0, Easing::OutQuad);

        let mid: f64 = kf.at(0.5).unwrap();
        // OutQuad at 0.5 = 0.75, so value should be 75
        assert!((mid - 75.0).abs() < 0.01);
    }

    #[test]
    fn test_keyframes_empty() {
        let kf: Keyframes<f64> = Keyframes::new();
        assert!(kf.at(0.5).is_none());
    }

    // =========================================================================
    // AnimatedValue Tests
    // =========================================================================

    #[test]
    fn test_animated_value_new() {
        let mut av = AnimatedValue::new(0.0, Duration::from_millis(100));
        assert_eq!(*av.value(), 0.0);
    }

    #[test]
    fn test_animated_value_set() {
        let mut av = AnimatedValue::new(0.0, Duration::from_millis(100));
        av.set(50.0);
        assert_eq!(*av.value(), 50.0);
    }

    #[test]
    fn test_animated_value_target() {
        let mut av = AnimatedValue::new(0.0, Duration::from_millis(100));
        av.animate_to(100.0);
        assert_eq!(*av.target(), 100.0);
    }

    // =========================================================================
    // Sequence Tests
    // =========================================================================

    #[test]
    fn test_sequence_new() {
        let seq = Sequence::new();
        assert!(!seq.is_running());
        assert!(!seq.is_complete());
    }

    #[test]
    fn test_sequence_then() {
        let seq = Sequence::new()
            .then(Duration::from_millis(100), 0.5)
            .then(Duration::from_millis(100), 1.0);

        assert_eq!(seq.steps.len(), 2);
    }

    #[test]
    fn test_sequence_start() {
        let mut seq = Sequence::new().then(Duration::from_millis(100), 1.0);

        seq.start();
        assert!(seq.is_running());
    }

    #[test]
    fn test_sequence_value() {
        let mut seq = Sequence::new().then(Duration::from_millis(0), 1.0); // Instant completion

        seq.start();
        std::thread::sleep(Duration::from_millis(10));
        let _ = seq.value();

        assert!(seq.is_complete());
    }

    // =========================================================================
    // Ticker Tests
    // =========================================================================

    #[test]
    fn test_ticker_new() {
        let ticker = Ticker::new();
        assert_eq!(ticker.fps(), 0.0);
    }

    #[test]
    fn test_ticker_tick() {
        let mut ticker = Ticker::new();
        let dt = ticker.tick();

        // First tick should return default dt
        assert!(dt > 0.0);
        assert!(dt < 0.1);
    }

    #[test]
    fn test_ticker_with_target_fps() {
        let ticker = Ticker::with_target_fps(30.0);
        let frame_dur = ticker.frame_duration();

        assert!((frame_dur.as_secs_f64() - 1.0 / 30.0).abs() < 0.001);
    }

    // =========================================================================
    // Interpolatable Tests
    // =========================================================================

    #[test]
    fn test_interpolatable_f64() {
        let a: f64 = 0.0;
        let b: f64 = 100.0;
        assert_eq!(a.lerp(&b, 0.0), 0.0);
        assert_eq!(a.lerp(&b, 1.0), 100.0);
        assert_eq!(a.lerp(&b, 0.5), 50.0);
    }

    #[test]
    fn test_interpolatable_u8() {
        let a: u8 = 0;
        let b: u8 = 100;
        assert_eq!(a.lerp(&b, 0.5), 50);
    }

    #[test]
    fn test_interpolatable_tuple() {
        let a = (0.0, 0.0);
        let b = (100.0, 200.0);
        let mid = a.lerp(&b, 0.5);
        assert_eq!(mid, (50.0, 100.0));
    }

    // =========================================================================
    // Preset Tests
    // =========================================================================

    #[test]
    fn test_preset_fade_in() {
        let mut seq = presets::fade_in(100);
        seq.start();
        assert!(seq.is_running());
    }

    #[test]
    fn test_preset_pulse() {
        let seq = presets::pulse(200);
        assert!(seq.repeat);
    }

    #[test]
    fn test_preset_blink() {
        let seq = presets::blink(500);
        assert!(seq.repeat);
        assert_eq!(seq.steps.len(), 2);
    }
}
