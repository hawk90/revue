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
    use crate::widget::transition::types::TransitionPhase;

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

    #[test]
    fn test_transition_default() {
        let transition = Transition::default();
        assert!(transition.is_visible());
    }

    #[test]
    fn test_transition_phase() {
        let transition = Transition::new("Test");
        // Initially visible with no animation, phase is Visible
        assert_eq!(transition.phase(), TransitionPhase::Visible);
    }

    #[test]
    fn test_transition_show_when_visible() {
        let mut transition = Transition::new("Test");
        let initial_phase = transition.phase();
        transition.show();
        // Should remain visible
        assert!(transition.is_visible());
    }

    #[test]
    fn test_transition_hide_without_animation() {
        let mut transition = Transition::new("Test");
        transition.hide();
        // With no leave animation, should still be visible
        // because there's no tween to set visible = false
    }

    #[test]
    fn test_transition_show_hide_cycle() {
        let mut transition = Transition::new("Test");
        assert!(transition.is_visible());

        transition.hide();
        transition.show();
        assert!(transition.is_visible());

        transition.toggle();
        transition.toggle();
        assert!(transition.is_visible());
    }

    #[test]
    fn test_transition_animations_builder() {
        let enter = Animation::slide_left();
        let leave = Animation::slide_right();

        let _transition = Transition::new("Content").animations(enter.clone(), leave.clone());

        // Can't directly test animations, but we can verify the builder works
        let enter_anim = Animation::fade_in();
        let leave_anim = Animation::fade_out();
        let _t2 = Transition::new("Content2").animations(enter_anim, leave_anim);
    }

    #[test]
    fn test_animation_preset_variants() {
        // Test all AnimationPreset variants can be created
        let _fade = Animation::fade();
        let _slide_left = Animation::slide_left();
        let _slide_right = Animation::slide_right();
        let _slide_up = Animation::slide_up();
        let _slide_down = Animation::slide_down();
        let _scale = Animation::scale();
        let _custom = Animation::custom(Some(0.5), Some(10), Some(-5), Some(0.8));
    }

    #[test]
    fn test_transition_with_content_types() {
        let _s = Transition::new("String content");
        let _owned = Transition::new(String::from("Owned string"));
        let _empty = Transition::new("");
    }

    #[test]
    fn test_animation_duration_variants() {
        let anim = Animation::fade().duration(100);
        assert_eq!(anim.get_duration().as_millis(), 100);

        let anim2 = Animation::fade().duration(5000);
        assert_eq!(anim2.get_duration().as_millis(), 5000);
    }

    #[test]
    fn test_animation_delay_variants() {
        let anim = Animation::fade().delay(0);
        assert_eq!(anim.get_delay(), Duration::ZERO);

        let anim2 = Animation::fade().delay(250);
        assert_eq!(anim2.get_delay().as_millis(), 250);
    }

    #[test]
    fn test_animation_custom_presets() {
        let anim1 = Animation::custom(Some(0.0), Some(0), Some(0), None);
        let anim2 = Animation::custom(Some(1.0), Some(100), Some(-100), Some(1.5));
        let anim3 = Animation::custom(None, None, None, None);

        // All should create Custom presets
        assert!(matches!(anim1.preset(), AnimationPreset::Custom { .. }));
        assert!(matches!(anim2.preset(), AnimationPreset::Custom { .. }));
        assert!(matches!(anim3.preset(), AnimationPreset::Custom { .. }));
    }

    #[test]
    fn test_transition_phase_variants() {
        use crate::widget::transition::types::TransitionPhase;

        let _entering = TransitionPhase::Entering;
        let _visible = TransitionPhase::Visible;
        let _leaving = TransitionPhase::Leaving;
    }

    #[test]
    fn test_transition_group_with_various_content() {
        let group = TransitionGroup::new(vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
        ]);
        assert_eq!(group.len(), 3);
    }

    #[test]
    fn test_transition_remove_nonexistent() {
        let mut group = TransitionGroup::new(vec!["A", "B"]);
        let removed = group.remove(5); // Index out of bounds
        assert_eq!(removed, None);
        assert_eq!(group.len(), 2);
    }

    #[test]
    fn test_animation_clone_preserves_easing() {
        let anim1 = Animation::fade().easing(easing::linear);
        let anim2 = anim1.clone();

        // The easing function pointer should be the same
        assert_eq!(anim1.get_easing() as usize, anim2.get_easing() as usize);
    }

    #[test]
    fn test_transition_group_items() {
        let group = TransitionGroup::new(vec!["A", "B", "C"]);
        let items = group.items();
        assert_eq!(items, &["A", "B", "C"]);
    }

    #[test]
    fn test_transition_group_move_animation() {
        let _group = TransitionGroup::new(vec!["A", "B"]).move_animation(Animation::slide_left());
    }

    #[test]
    fn test_transition_group_stagger() {
        let group = TransitionGroup::new(vec!["A", "B"]).stagger(50);
        // Can't directly test stagger_delay, but verify builder works
    }

    #[test]
    fn test_transition_group_default() {
        let group = TransitionGroup::default();
        assert_eq!(group.len(), 0);
        assert!(group.is_empty());
    }

    #[test]
    fn test_transition_group_push_various_types() {
        let mut group = TransitionGroup::new(vec!["A"]);
        group.push("B");
        group.push(String::from("C"));
        group.push(&String::from("D"));

        assert_eq!(group.len(), 4);
    }
}
