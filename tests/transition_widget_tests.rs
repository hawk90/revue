//! Integration tests for Transition widget public API

use revue::widget::{transition, Animation, AnimationPreset, AnimationTransition, TransitionPhase};

// ============================================================================
// Transition Widget Tests
// ============================================================================

#[test]
fn test_transition_new() {
    let t = AnimationTransition::new("Test content");
    assert_eq!(t.is_visible(), true);
}

#[test]
fn test_transition_new_empty() {
    let t = AnimationTransition::new("");
    assert_eq!(t.is_visible(), true);
}

#[test]
fn test_transition_default() {
    let t = AnimationTransition::default();
    assert_eq!(t.is_visible(), true);
    assert_eq!(t.phase(), TransitionPhase::Visible);
}

#[test]
fn test_transition_enter() {
    let t = AnimationTransition::new("Content").enter(Animation::fade());
    assert_eq!(t.is_visible(), true);
}

#[test]
fn test_transition_leave() {
    let t = AnimationTransition::new("Content").leave(Animation::fade());
    assert_eq!(t.is_visible(), true);
}

#[test]
fn test_transition_animations() {
    let enter = Animation::fade_in();
    let leave = Animation::fade_out();
    let t = AnimationTransition::new("Content").animations(enter, leave);
    assert_eq!(t.is_visible(), true);
}

#[test]
fn test_transition_show() {
    let mut t = AnimationTransition::new("Content");
    t.show();
    assert_eq!(t.is_visible(), true);
}

#[test]
fn test_transition_hide() {
    let mut t = AnimationTransition::new("Content");
    t.hide();
    assert_eq!(t.is_visible(), false);
}

#[test]
fn test_transition_hide_then_show() {
    let mut t = AnimationTransition::new("Content");
    t.hide();
    assert_eq!(t.is_visible(), false);
    t.show();
    assert_eq!(t.is_visible(), true);
}

#[test]
fn test_transition_toggle_from_visible() {
    let mut t = AnimationTransition::new("Content");
    assert_eq!(t.is_visible(), true);
    t.toggle();
    assert_eq!(t.is_visible(), false);
}

#[test]
fn test_transition_toggle_from_hidden() {
    let mut t = AnimationTransition::new("Content");
    t.hide();
    assert_eq!(t.is_visible(), false);
    t.toggle();
    assert_eq!(t.is_visible(), true);
}

#[test]
fn test_transition_multiple_toggles() {
    let mut t = AnimationTransition::new("Content");
    assert!(t.is_visible());

    t.toggle();
    assert!(!t.is_visible());

    t.toggle();
    assert!(t.is_visible());

    t.toggle();
    assert!(!t.is_visible());
}

#[test]
fn test_transition_is_visible() {
    let t = AnimationTransition::new("Content");
    assert!(t.is_visible());
}

#[test]
fn test_transition_phase_initial() {
    let t = AnimationTransition::new("Content");
    assert_eq!(t.phase(), TransitionPhase::Visible);
}

#[test]
fn test_transition_with_enter_animation() {
    let t = transition("Test").enter(Animation::fade_in());
    assert!(t.is_visible());
}

#[test]
fn test_transition_with_leave_animation() {
    let t = transition("Test").leave(Animation::fade_out());
    assert!(t.is_visible());
}

#[test]
fn test_transition_builder_chain() {
    let enter = Animation::fade_in();
    let leave = Animation::fade_out();
    let t = AnimationTransition::new("Content")
        .enter(enter.clone())
        .leave(leave.clone());
    assert!(t.is_visible());
}

#[test]
fn test_transition_long_content() {
    let long_content = "This is a very long content string for testing";
    let t = AnimationTransition::new(long_content);
    assert!(t.is_visible());
}

#[test]
fn test_transition_with_unicode() {
    let unicode_content = "Hello 世界 🌍";
    let t = AnimationTransition::new(unicode_content);
    assert!(t.is_visible());
}

#[test]
fn test_transition_multiple_hide_calls() {
    let mut t = AnimationTransition::new("Content");
    t.hide();
    assert!(!t.is_visible());

    // Calling hide again when already hidden should not cause issues
    t.hide();
    assert!(!t.is_visible());
}

#[test]
fn test_transition_multiple_show_calls() {
    let mut t = AnimationTransition::new("Content");
    t.show();
    assert!(t.is_visible());

    // Calling show again when already visible should not cause issues
    t.show();
    assert!(t.is_visible());
}

#[test]
fn test_transition_with_all_animations() {
    let enter = Animation::fade_in().duration(300);
    let leave = Animation::fade_out().duration(200);
    let t = AnimationTransition::new("Content").animations(enter, leave);

    // With animations set, hide() starts the leave animation tween
    // The widget remains visible until the animation completes
    assert!(t.is_visible());
}

#[test]
fn test_transition_helper_function() {
    let t = transition("Helper test");
    assert!(t.is_visible());
}
