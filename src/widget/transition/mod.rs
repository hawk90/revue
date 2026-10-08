//! Transition and TransitionGroup widgets for declarative animations
//!
//! These widgets provide Vue/React-style declarative animation APIs
//! that automatically apply animations when widgets are added, removed,
//! or reordered.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Transition, TransitionGroup, transition, transition_group};
//! use revue::style::animation::{Animation, ease_in_out, fade_in, slide_in_left};
//!
//! // Single element transition
//! Transition::new(content)
//!     .enter(Animation::fade_in().duration(300))
//!     .leave(Animation::fade_out().duration(200));
//!
//! // A list. TransitionGroup draws its items; it does not animate them -
//! // its enter/leave/move/stagger builders are deprecated.
//! let items = vec!["Item 1", "Item 2", "Item 3"];
//! TransitionGroup::new(items);
//! ```

mod core;
mod group;
mod helper;
mod types;

pub use types::{Animation, AnimationPreset, TransitionPhase};

// Re-export main widgets
pub use core::Transition;
pub use group::TransitionGroup;

// Re-export helper functions
pub use helper::{transition, transition_group};
