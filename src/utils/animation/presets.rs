//! Common animation presets

use crate::utils::easing::Easing;
use std::time::Duration;

use super::Sequence;

/// Fade in animation (0.0 to 1.0)
pub fn fade_in(duration_ms: u64) -> Sequence {
    Sequence::new().then_eased(Duration::from_millis(duration_ms), 1.0, Easing::OutQuad)
}

/// Fade out animation (1.0 to 0.0)
pub fn fade_out(duration_ms: u64) -> Sequence {
    Sequence::new().then_eased(Duration::from_millis(duration_ms), 0.0, Easing::InQuad)
}

/// Pulse animation (repeating scale)
pub fn pulse(duration_ms: u64) -> Sequence {
    Sequence::new()
        .then_eased(Duration::from_millis(duration_ms / 2), 1.0, Easing::OutQuad)
        .then_eased(Duration::from_millis(duration_ms / 2), 0.0, Easing::InQuad)
        .repeat(true)
}

/// Blink animation (on/off)
pub fn blink(duration_ms: u64) -> Sequence {
    Sequence::new()
        .then(Duration::from_millis(duration_ms / 2), 1.0)
        .then(Duration::from_millis(duration_ms / 2), 0.0)
        .repeat(true)
}

/// Slide in from left (-1.0 to 0.0)
pub fn slide_in_left(duration_ms: u64) -> Sequence {
    Sequence::new().then_eased(Duration::from_millis(duration_ms), 0.0, Easing::OutCubic)
}

/// Bounce animation
pub fn bounce(duration_ms: u64) -> Sequence {
    Sequence::new().then_eased(Duration::from_millis(duration_ms), 1.0, Easing::OutBounce)
}

/// Elastic animation
pub fn elastic(duration_ms: u64) -> Sequence {
    Sequence::new().then_eased(Duration::from_millis(duration_ms), 1.0, Easing::OutElastic)
}

/// Typewriter effect (linear reveal)
pub fn typewriter(total_chars: usize, chars_per_second: f64) -> Sequence {
    let duration = Duration::from_secs_f64(total_chars as f64 / chars_per_second);
    Sequence::new().then(duration, 1.0)
}
