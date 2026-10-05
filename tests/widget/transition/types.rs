//! Animation, AnimationPreset and TransitionPhase tests
//!
//! The basic preset/duration/delay/clone checks live in
//! tests/transition_tests.rs; these cover what that file does not.

use revue::style::easing;
use revue::widget::{Animation, AnimationPreset, TransitionPhase};
use std::time::Duration;

fn same_easing(a: fn(f32) -> f32, b: fn(f32) -> f32) -> bool {
    [0.0, 0.25, 0.5, 0.75, 1.0]
        .iter()
        .all(|&t| (a(t) - b(t)).abs() < f32::EPSILON)
}

#[test]
fn test_animation_in_out_aliases_map_to_their_direction() {
    let cases = [
        (Animation::slide_in_left(), AnimationPreset::SlideLeft),
        (Animation::slide_out_left(), AnimationPreset::SlideLeft),
        (Animation::slide_in_right(), AnimationPreset::SlideRight),
        (Animation::slide_out_right(), AnimationPreset::SlideRight),
        (Animation::slide_in_up(), AnimationPreset::SlideUp),
        (Animation::slide_out_up(), AnimationPreset::SlideUp),
        (Animation::slide_in_down(), AnimationPreset::SlideDown),
        (Animation::slide_out_down(), AnimationPreset::SlideDown),
    ];
    for (anim, preset) in cases {
        assert_eq!(anim.preset(), preset);
    }
}

#[test]
fn test_animation_presets_default_timing() {
    let all = [
        Animation::fade(),
        Animation::slide_left(),
        Animation::slide_right(),
        Animation::slide_up(),
        Animation::slide_down(),
        Animation::scale(),
        Animation::custom(None, None, None, None),
    ];
    for anim in all {
        assert_eq!(anim.get_duration(), Duration::from_millis(300));
        assert_eq!(anim.get_delay(), Duration::ZERO);
    }
}

#[test]
fn test_animation_presets_default_easing() {
    assert!(same_easing(
        Animation::fade().get_easing(),
        easing::ease_in_out
    ));
    assert!(same_easing(
        Animation::custom(None, None, None, None).get_easing(),
        easing::ease_in_out
    ));
    for slide in [
        Animation::slide_left(),
        Animation::slide_right(),
        Animation::slide_up(),
        Animation::slide_down(),
    ] {
        assert!(same_easing(slide.get_easing(), easing::ease_out));
    }
    assert!(same_easing(
        Animation::scale().get_easing(),
        easing::back_out
    ));
}

#[test]
fn test_animation_custom_keeps_its_parameters() {
    let anim = Animation::custom(Some(0.5), Some(10), Some(-5), Some(0.8));
    assert_eq!(
        anim.preset(),
        AnimationPreset::Custom {
            opacity: Some(0.5),
            offset_x: Some(10),
            offset_y: Some(-5),
            scale: Some(0.8),
        }
    );

    let partial = Animation::custom(None, Some(-100), None, None);
    assert_eq!(
        partial.preset(),
        AnimationPreset::Custom {
            opacity: None,
            offset_x: Some(-100),
            offset_y: None,
            scale: None,
        }
    );
}

#[test]
fn test_animation_custom_presets_compare_by_value() {
    let a = Animation::custom(Some(0.5), None, None, None).preset();
    let b = Animation::custom(Some(0.5), None, None, None).preset();
    let c = Animation::custom(Some(0.6), None, None, None).preset();
    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_ne!(a, AnimationPreset::Fade);
}

#[test]
fn test_animation_all_presets_distinct() {
    let presets = [
        AnimationPreset::Fade,
        AnimationPreset::SlideLeft,
        AnimationPreset::SlideRight,
        AnimationPreset::SlideUp,
        AnimationPreset::SlideDown,
        AnimationPreset::Scale,
    ];
    for (i, a) in presets.iter().enumerate() {
        for b in &presets[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn test_animation_later_builder_calls_override_earlier_ones() {
    let anim = Animation::fade()
        .duration(200)
        .delay(50)
        .easing(easing::ease_in)
        .duration(400)
        .delay(100)
        .easing(easing::linear);

    assert_eq!(anim.get_duration(), Duration::from_millis(400));
    assert_eq!(anim.get_delay(), Duration::from_millis(100));
    assert!(same_easing(anim.get_easing(), easing::linear));
}

#[test]
fn test_animation_builder_does_not_change_preset() {
    let anim = Animation::slide_up().duration(10).delay(5);
    assert_eq!(anim.preset(), AnimationPreset::SlideUp);
}

#[test]
fn test_animation_clone_is_independent() {
    let original = Animation::fade().duration(300).delay(10);
    let copy = original.clone();
    let changed = original.duration(500);

    assert_eq!(copy.get_duration(), Duration::from_millis(300));
    assert_eq!(copy.get_delay(), Duration::from_millis(10));
    assert_eq!(changed.get_duration(), Duration::from_millis(500));
}

#[test]
fn test_transition_phase_variants_distinct() {
    let phases = [
        TransitionPhase::Hidden,
        TransitionPhase::Entering,
        TransitionPhase::Visible,
        TransitionPhase::Leaving,
    ];
    for (i, a) in phases.iter().enumerate() {
        for b in &phases[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn test_debug_output_names_the_variant() {
    assert_eq!(format!("{:?}", AnimationPreset::Fade), "Fade");
    assert!(format!(
        "{:?}",
        Animation::custom(Some(0.5), None, None, None).preset()
    )
    .starts_with("Custom"));
    assert_eq!(format!("{:?}", TransitionPhase::Hidden), "Hidden");
    assert_eq!(format!("{:?}", TransitionPhase::Leaving), "Leaving");
}
