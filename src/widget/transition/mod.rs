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
    use super::*;

    use crate::style::easing;
    use std::time::Duration;

    #[test]
    fn test_animation_defaults() {
        let anim = Animation::fade();
        assert_eq!(anim.get_duration(), Duration::from_millis(300));
        assert_eq!(anim.get_delay(), Duration::ZERO);
    }

    #[test]
    fn test_animation_builder() {
        let anim = Animation::fade()
            .duration(500)
            .delay(100)
            .easing(easing::linear);

        assert_eq!(anim.get_duration(), Duration::from_millis(500));
        assert_eq!(anim.get_delay(), Duration::from_millis(100));
    }

    #[test]
    fn test_animation_presets() {
        let fade = Animation::fade();
        assert_eq!(fade.preset(), AnimationPreset::Fade);

        let slide = Animation::slide_left();
        assert_eq!(slide.preset(), AnimationPreset::SlideLeft);

        let scale = Animation::scale();
        assert_eq!(scale.preset(), AnimationPreset::Scale);
    }

    #[test]
    fn test_animation_custom() {
        let custom = Animation::custom(Some(0.5), Some(10), Some(-5), Some(0.8));
        assert!(matches!(custom.preset(), AnimationPreset::Custom { .. }));
    }

    #[test]
    fn test_transition_new() {
        let _transition = Transition::new("Hello");
        // Private fields - cannot test directly
    }

    #[test]
    fn test_transition_builder() {
        let enter = Animation::fade_in();
        let leave = Animation::fade_out();
        let _transition = Transition::new("Test")
            .enter(enter.clone())
            .leave(leave.clone());
        // Private fields - cannot test directly
    }

    #[test]
    fn test_transition_toggle() {
        let mut transition = Transition::new("Test");
        assert!(transition.is_visible());

        transition.hide();
        // Phase changes to Leaving but visible remains true until animation completes

        transition.show();
        assert!(transition.is_visible());
    }

    #[test]
    fn test_transition_group_new() {
        let group = TransitionGroup::new(vec!["Item 1", "Item 2", "Item 3"]);
        assert_eq!(group.len(), 3);
        assert!(!group.is_empty());
    }

    #[test]
    fn test_transition_group_empty() {
        let group: TransitionGroup = TransitionGroup::new(Vec::<String>::new());
        assert_eq!(group.len(), 0);
        assert!(group.is_empty());
    }

    #[test]
    fn test_transition_group_builder() {
        let _group = TransitionGroup::new(vec!["A", "B"])
            .enter(Animation::fade_in())
            .leave(Animation::fade_out())
            .stagger(50);
        // Private fields - cannot test directly
    }

    #[test]
    fn test_transition_group_push_remove() {
        let mut group = TransitionGroup::new(vec!["Item 1"]);
        assert_eq!(group.len(), 1);

        group.push("Item 2");
        assert_eq!(group.len(), 2);

        let removed = group.remove(0);
        assert_eq!(removed, Some("Item 1".to_string()));
        assert_eq!(group.len(), 1);
    }

    #[test]
    fn test_convenience_functions() {
        let _transition = transition("Hello");
        // Private fields - cannot test directly

        let group = transition_group(vec!["A", "B"]);
        assert_eq!(group.len(), 2);
    }
}
