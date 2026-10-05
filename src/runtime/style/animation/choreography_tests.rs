//! Tests for AnimationGroup, Stagger and Choreographer

use std::time::Duration;

use super::*;

#[test]
fn test_animation_group_start() {
    let mut group = AnimationGroup::parallel()
        .with_animation(KeyframeAnimation::new("a").duration(Duration::from_millis(100)));

    group.start();
    assert!(group.animations_mut()[0].is_running());
}

#[test]
fn test_animation_group_is_completed() {
    let mut group = AnimationGroup::parallel()
        .with_animation(KeyframeAnimation::new("a").duration(Duration::from_millis(1)));

    assert!(!group.is_completed());
    group.start();
    group.update();
    // Should complete quickly
}

#[test]
fn test_stagger_start_delay() {
    let stagger =
        Stagger::new(3, Duration::from_millis(100)).start_delay(Duration::from_millis(50));

    assert_eq!(stagger.delay_for(0), Duration::from_millis(50));
    assert_eq!(stagger.delay_for(1), Duration::from_millis(150));
    assert_eq!(stagger.delay_for(2), Duration::from_millis(250));
}

#[test]
fn test_stagger_easing() {
    let stagger = Stagger::new(3, Duration::from_millis(100)).easing(easing::ease_in);

    // First item should have shorter delay with ease_in
    let delay0 = stagger.delay_for(0).as_millis();
    let delay1 = stagger.delay_for(1).as_millis();
    let delay2 = stagger.delay_for(2).as_millis();

    assert!(delay0 < delay1);
    assert!(delay1 < delay2);
}

#[test]
fn test_stagger_total_duration() {
    let stagger = Stagger::new(5, Duration::from_millis(50));
    let item_duration = Duration::from_millis(100);
    let total = stagger.total_duration(item_duration);

    assert_eq!(total, Duration::from_millis(200 + 100)); // last delay + item duration
}

#[test]
fn test_stagger_zero_count() {
    let stagger = Stagger::new(0, Duration::from_millis(50));
    assert_eq!(stagger.delay_for(0), Duration::ZERO);
    assert_eq!(
        stagger.total_duration(Duration::from_millis(100)),
        Duration::ZERO
    );
}

#[test]
fn test_stagger_single_item() {
    let stagger = Stagger::new(1, Duration::from_millis(50));
    assert_eq!(stagger.delay_for(0), Duration::ZERO);
}

#[test]
fn test_stagger_apply() {
    let stagger = Stagger::new(3, Duration::from_millis(50));
    let animations = stagger.apply(|i| KeyframeAnimation::new(format!("anim-{}", i)));

    assert_eq!(animations.len(), 3);
    assert_eq!(animations[0].delay, Duration::ZERO);
    assert_eq!(animations[1].delay, Duration::from_millis(50));
    assert_eq!(animations[2].delay, Duration::from_millis(100));
}

#[test]
fn test_choreographer_new() {
    let choreo = Choreographer::new();
    assert!(choreo.is_completed("nonexistent"));
}

#[test]
fn test_choreographer_default() {
    let choreo = Choreographer::default();
    assert!(choreo.is_completed("test"));
}

#[test]
fn test_choreographer_add_group() {
    let mut choreo = Choreographer::new();
    let group = AnimationGroup::parallel()
        .with_animation(KeyframeAnimation::new("a").duration(Duration::from_millis(100)));

    choreo.add_group("test-group", group);
    choreo.start("test-group");
    assert!(!choreo.is_completed("test-group"));
}

#[test]
fn test_choreographer_add_staggered() {
    let mut choreo = Choreographer::new();
    choreo.add_staggered("items", 3, Duration::from_millis(50), |i| {
        KeyframeAnimation::new(format!("item-{}", i))
    });

    assert_eq!(choreo.get_staggered("items", 0, "opacity"), 0.0);
}

#[test]
fn test_choreographer_start_nonexistent() {
    let mut choreo = Choreographer::new();
    choreo.start("nonexistent"); // Should not panic
    assert!(choreo.is_completed("nonexistent"));
}

#[test]
fn test_choreographer_update() {
    let mut choreo = Choreographer::new();
    let group = AnimationGroup::parallel()
        .with_animation(KeyframeAnimation::new("a").duration(Duration::from_millis(1)));

    choreo.add_group("test", group);
    choreo.start("test");
    choreo.update(); // Should not panic
}

#[test]
fn test_choreographer_is_completed_empty() {
    let choreo = Choreographer::new();
    assert!(choreo.is_completed("empty"));
}

#[test]
fn test_choreographer_get_staggered_nonexistent() {
    let mut choreo = Choreographer::new();
    assert_eq!(choreo.get_staggered("nonexistent", 0, "opacity"), 0.0);
}

#[test]
fn test_choreographer_get_staggered_out_of_bounds() {
    let mut choreo = Choreographer::new();
    choreo.add_staggered("items", 2, Duration::from_millis(50), |i| {
        KeyframeAnimation::new(format!("item-{}", i))
    });

    assert_eq!(choreo.get_staggered("items", 10, "opacity"), 0.0);
}

#[test]
fn test_group_mode_default() {
    let mode = GroupMode::default();
    assert_eq!(mode, GroupMode::Parallel);
}

#[test]
fn test_group_mode_variants() {
    assert_eq!(GroupMode::Parallel, GroupMode::Parallel);
    assert_eq!(GroupMode::Sequential, GroupMode::Sequential);
    assert_ne!(GroupMode::Parallel, GroupMode::Sequential);
}
