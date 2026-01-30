//! Integration tests for transition widgets

use revue::style::easing;
use revue::widget::{
    Animation, AnimationPreset, AnimationTransition as Transition, TransitionGroup, TransitionPhase,
};

#[test]
fn test_animation_creation() {
    let fade = Animation::fade();
    assert_eq!(fade.get_duration().as_millis(), 300);

    let slide = Animation::slide_left();
    assert_eq!(slide.get_duration().as_millis(), 300);
}

#[test]
fn test_animation_builder() {
    let anim = Animation::fade()
        .duration(500)
        .delay(100)
        .easing(easing::linear);

    assert_eq!(anim.get_duration().as_millis(), 500);
    assert_eq!(anim.get_delay().as_millis(), 100);

    // Test that easing function works
    let result = anim.get_easing()(0.5);
    assert_eq!(result, 0.5); // linear returns same value
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
fn test_animation_slide_variants() {
    assert_eq!(
        Animation::slide_right().preset(),
        AnimationPreset::SlideRight
    );
    assert_eq!(Animation::slide_up().preset(), AnimationPreset::SlideUp);
    assert_eq!(Animation::slide_down().preset(), AnimationPreset::SlideDown);
}

#[test]
fn test_animation_fade_variants() {
    assert_eq!(Animation::fade_in().preset(), AnimationPreset::Fade);
    assert_eq!(Animation::fade_out().preset(), AnimationPreset::Fade);
}

#[test]
fn test_animation_scale_variants() {
    assert_eq!(Animation::scale_up().preset(), AnimationPreset::Scale);
    assert_eq!(Animation::scale_down().preset(), AnimationPreset::Scale);
}

#[test]
fn test_animation_custom() {
    let custom = Animation::custom(Some(0.5), Some(10), Some(-5), Some(0.8));
    assert!(matches!(custom.preset(), AnimationPreset::Custom { .. }));
}

#[test]
fn test_animation_default() {
    let anim = Animation::default();
    assert_eq!(anim.preset(), AnimationPreset::Fade);
}

#[test]
fn test_animation_clone() {
    let anim1 = Animation::fade().duration(500).easing(easing::linear);
    let anim2 = anim1.clone();

    assert_eq!(anim1.preset(), anim2.preset());
    assert_eq!(anim1.get_duration(), anim2.get_duration());
    // Compare easing results
    assert_eq!(anim1.get_easing()(0.5), anim2.get_easing()(0.5));
}

#[test]
fn test_transition_creation() {
    let transition = Transition::new("Hello");
    assert!(transition.is_visible());
    assert_eq!(transition.phase(), TransitionPhase::Visible);
}

#[test]
fn test_transition_default() {
    let transition = Transition::default();
    assert!(transition.is_visible());
}

#[test]
fn test_transition_builder() {
    let enter = Animation::fade_in();
    let leave = Animation::fade_out();

    let _transition = Transition::new("Test")
        .enter(enter.clone())
        .leave(leave.clone());
}

#[test]
fn test_transition_animations_builder() {
    let enter = Animation::slide_left();
    let leave = Animation::slide_right();

    let _transition = Transition::new("Content").animations(enter, leave);
}

#[test]
fn test_transition_show_hide() {
    let mut transition = Transition::new("Test");

    transition.show();
    assert!(transition.is_visible());

    transition.hide();
    // Phase changes but visible remains true until animation completes

    transition.show();
    assert!(transition.is_visible());
}

#[test]
fn test_transition_toggle() {
    let mut transition = Transition::new("Test");

    let initial_visible = transition.is_visible();
    transition.toggle();
    transition.toggle();

    // After two toggles, should be back to initial state
    assert_eq!(transition.is_visible(), initial_visible);
}

#[test]
fn test_transition_group_creation() {
    let group = TransitionGroup::new(vec!["Item 1", "Item 2", "Item 3"]);
    assert_eq!(group.len(), 3);
    assert!(!group.is_empty());
}

#[test]
fn test_transition_group_default() {
    let group = TransitionGroup::default();
    assert_eq!(group.len(), 0);
    assert!(group.is_empty());
}

#[test]
fn test_transition_group_empty() {
    let group: TransitionGroup = TransitionGroup::new(Vec::<String>::new());
    assert_eq!(group.len(), 0);
    assert!(group.is_empty());
}

#[test]
fn test_transition_group_push() {
    let mut group = TransitionGroup::new(vec!["Item 1"]);
    assert_eq!(group.len(), 1);

    group.push("Item 2");
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove() {
    let mut group = TransitionGroup::new(vec!["Item 1", "Item 2", "Item 3"]);
    assert_eq!(group.len(), 3);

    let removed = group.remove(1);
    assert_eq!(removed, Some("Item 2".to_string()));
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove_invalid() {
    let mut group = TransitionGroup::new(vec!["Item 1", "Item 2"]);
    assert_eq!(group.len(), 2);

    let removed = group.remove(5);
    assert_eq!(removed, None);
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_items() {
    let group = TransitionGroup::new(vec!["A", "B", "C"]);
    let items = group.items();

    assert_eq!(items, &["A", "B", "C"]);
}

#[test]
fn test_transition_group_builder() {
    let enter = Animation::fade_in();
    let leave = Animation::fade_out();

    let _group = TransitionGroup::new(vec!["A", "B"])
        .enter(enter.clone())
        .leave(leave.clone())
        .stagger(50);
}

#[test]
fn test_transition_group_move_animation() {
    let _group = TransitionGroup::new(vec!["A", "B"]).move_animation(Animation::slide_left());
}

#[test]
fn test_transition_helper_functions() {
    use revue::widget::{transition, transition_group};

    let _t = transition("Hello");
    let g = transition_group(vec!["A", "B"]);
    assert_eq!(g.len(), 2);
}

#[test]
fn test_transition_phase_equality() {
    assert_eq!(TransitionPhase::Visible, TransitionPhase::Visible);
    assert_ne!(TransitionPhase::Entering, TransitionPhase::Leaving);
}

#[test]
fn test_animation_preset_partial_equality() {
    let fade1 = Animation::fade();
    let fade2 = Animation::fade();
    assert_eq!(fade1.preset(), fade2.preset());

    let slide = Animation::slide_left();
    assert_ne!(fade1.preset(), slide.preset());
}
