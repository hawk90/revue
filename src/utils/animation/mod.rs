//! Animation utilities for terminal UI
//!
//! Provides frame-based animation primitives that work well with terminal
//! render loops. Includes spring physics, keyframes, sequences, and timers.
//!
//! # Example
//!
//! ```
//! use revue::utils::animation::{Keyframes, Spring};
//!
//! // Spring animation for smooth motion, stepped at 60 fps
//! let dt = 1.0 / 60.0;
//! let mut spring = Spring::new(0.0, 100.0);
//! for _ in 0..600 {
//!     spring.update(dt);
//!     if spring.is_settled() {
//!         break;
//!     }
//! }
//! assert!((spring.value() - 100.0).abs() < 1.0);
//!
//! // Keyframe animation
//! let anim = Keyframes::new()
//!     .add(0.0, 0.0)
//!     .add(0.5, 100.0)
//!     .add(1.0, 50.0);
//! let value = anim.at(0.25); // interpolated between the first two keyframes
//! assert!(value.is_some_and(|v| v > 0.0 && v < 100.0));
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
