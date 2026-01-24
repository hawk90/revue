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
mod tests;
