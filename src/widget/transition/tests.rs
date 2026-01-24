use crate::style::easing;
use crate::widget::transition::{Animation, AnimationPreset, Transition, TransitionGroup};

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
    let transition = Transition::new("Hello");
    assert_eq!(transition.child_content, "Hello");
    assert!(transition.is_visible());
    assert!(transition.enter_animation.is_none());
    assert!(transition.leave_animation.is_none());
}

#[test]
fn test_transition_builder() {
    let enter = Animation::fade_in();
    let leave = Animation::fade_out();
    let transition = Transition::new("Test")
        .enter(enter.clone())
        .leave(leave.clone());

    assert!(transition.enter_animation.is_some());
    assert!(transition.leave_animation.is_some());
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
    let group = TransitionGroup::new(vec!["A", "B"])
        .enter(Animation::fade_in())
        .leave(Animation::fade_out())
        .stagger(50);

    assert!(group.enter_animation.is_some());
    assert!(group.leave_animation.is_some());
    assert_eq!(group.stagger_delay, 50);
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
    let transition = transition("Hello");
    assert_eq!(transition.child_content, "Hello");

    let group = transition_group(vec!["A", "B"]);
    assert_eq!(group.len(), 2);
}
