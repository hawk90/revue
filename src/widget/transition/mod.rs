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
//! // List transitions
//! let items = vec!["Item 1", "Item 2", "Item 3"];
//! TransitionGroup::new(items)
//!     .enter(Animation::slide_in_left())
//!     .leave(Animation::slide_out_right())
//!     .stagger(50); // ms delay between items
//! ```

mod core;
mod group;
mod helper;
mod types;

pub use types::{Animation, AnimationPreset};

// Re-export main widgets
pub use core::Transition;
pub use group::TransitionGroup;

// Re-export helper functions
pub use helper::{transition, transition_group};

#[cfg(test)]
mod tests {
    include!("tests.rs");
}
