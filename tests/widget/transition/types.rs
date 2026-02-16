//! Transition and TransitionGroup widgets - type definitions tests

use revue::widget::{Animation, AnimationPreset, TransitionPhase};
use revue::style::easing;
use std::time::Duration;

// =========================================================================
// AnimationPreset enum trait tests (Copy, Clone, PartialEq, Debug)
// =========================================================================

#[test]
fn test_animation_preset_copy_trait() {
    let preset1 = AnimationPreset::Fade;
    let preset2 = preset1; // Copy should work
    assert_eq!(preset1, AnimationPreset::Fade);
    assert_eq!(preset2, AnimationPreset::Fade);
}

#[test]
fn test_animation_preset_clone_trait() {
    let preset1 = AnimationPreset::SlideLeft;
    let preset2 = preset1.clone();
    assert_eq!(preset1, preset2);
}

#[test]
fn test_animation_preset_partial_eq() {
    let fade1 = AnimationPreset::Fade;
    let fade2 = AnimationPreset::Fade;
    let slide = AnimationPreset::SlideLeft;

    assert_eq!(fade1, fade2);
    assert_ne!(fade1, slide);
}

#[test]
fn test_animation_preset_debug_trait() {
    let debug_str = format!("{:?}", AnimationPreset::Fade);
    assert!(debug_str.contains("Fade"));
}

#[test]
fn test_animation_preset_custom_debug() {
    let custom = AnimationPreset::Custom {
        opacity: Some(0.5),
        offset_x: Some(10),
        offset_y: Some(-5),
        scale: Some(1.0),
    };
    let debug_str = format!("{:?}", custom);
    assert!(debug_str.contains("Custom"));
}

// =========================================================================
// Animation constructor tests
// =========================================================================

#[test]
fn test_animation_default() {
    let anim = Animation::default();
    assert_eq!(anim.preset(), AnimationPreset::Fade);
    assert_eq!(anim.get_duration(), Duration::from_millis(300));
}

#[test]
fn test_animation_fade() {
    let anim = Animation::fade();
    assert_eq!(anim.preset(), AnimationPreset::Fade);
    assert_eq!(anim.get_duration(), Duration::from_millis(300));
    let result = anim.get_easing()(0.5);
    assert!((result - 0.5).abs() < 1.0);
    assert_eq!(anim.get_delay(), Duration::ZERO);
}

#[test]
fn test_animation_fade_in() {
    let fade_in = Animation::fade_in();
    assert_eq!(fade_in.preset(), AnimationPreset::Fade);
}

#[test]
fn test_animation_fade_out() {
    let fade_out = Animation::fade_out();
    assert_eq!(fade_out.preset(), AnimationPreset::Fade);
}

#[test]
fn test_animation_slide_left() {
    let anim = Animation::slide_left();
    assert_eq!(anim.preset(), AnimationPreset::SlideLeft);
    let _result = anim.get_easing()(0.5);
}

#[test]
fn test_animation_slide_right() {
    let anim = Animation::slide_right();
    assert_eq!(anim.preset(), AnimationPreset::SlideRight);
}

#[test]
fn test_animation_slide_up() {
    let anim = Animation::slide_up();
    assert_eq!(anim.preset(), AnimationPreset::SlideUp);
}

#[test]
fn test_animation_slide_down() {
    let anim = Animation::slide_down();
    assert_eq!(anim.preset(), AnimationPreset::SlideDown);
}

#[test]
fn test_animation_slide_in_left() {
    let anim = Animation::slide_in_left();
    assert_eq!(anim.preset(), AnimationPreset::SlideLeft);
}

#[test]
fn test_animation_slide_out_left() {
    let anim = Animation::slide_out_left();
    assert_eq!(anim.preset(), AnimationPreset::SlideLeft);
}

#[test]
fn test_animation_slide_in_right() {
    let anim = Animation::slide_in_right();
    assert_eq!(anim.preset(), AnimationPreset::SlideRight);
}

#[test]
fn test_animation_slide_out_right() {
    let anim = Animation::slide_out_right();
    assert_eq!(anim.preset(), AnimationPreset::SlideRight);
}

#[test]
fn test_animation_slide_in_up() {
    let anim = Animation::slide_in_up();
    assert_eq!(anim.preset(), AnimationPreset::SlideUp);
}

#[test]
fn test_animation_slide_out_up() {
    let anim = Animation::slide_out_up();
    assert_eq!(anim.preset(), AnimationPreset::SlideUp);
}

#[test]
fn test_animation_slide_in_down() {
    let anim = Animation::slide_in_down();
    assert_eq!(anim.preset(), AnimationPreset::SlideDown);
}

#[test]
fn test_animation_slide_out_down() {
    let anim = Animation::slide_out_down();
    assert_eq!(anim.preset(), AnimationPreset::SlideDown);
}

#[test]
fn test_animation_scale() {
    let anim = Animation::scale();
    assert_eq!(anim.preset(), AnimationPreset::Scale);
    let _result = anim.get_easing()(0.5);
}

#[test]
fn test_animation_scale_up() {
    let anim = Animation::scale_up();
    assert_eq!(anim.preset(), AnimationPreset::Scale);
}

#[test]
fn test_animation_scale_down() {
    let anim = Animation::scale_down();
    assert_eq!(anim.preset(), AnimationPreset::Scale);
}

#[test]
fn test_animation_custom_all_params() {
    let anim = Animation::custom(Some(0.5), Some(10), Some(-5), Some(0.8));
    assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
}

#[test]
fn test_animation_custom_all_none() {
    let anim = Animation::custom(None, None, None, None);
    assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
}

// =========================================================================
// Animation builder method tests
// =========================================================================

#[test]
fn test_animation_duration_builder() {
    let anim = Animation::fade().duration(500);
    assert_eq!(anim.get_duration(), Duration::from_millis(500));
}

#[test]
fn test_animation_easing_builder() {
    let anim = Animation::fade().easing(easing::linear);
    let result = anim.get_easing()(0.5);
    assert_eq!(result, 0.5);
}

#[test]
fn test_animation_delay_builder() {
    let anim = Animation::fade().delay(100);
    assert_eq!(anim.get_delay(), Duration::from_millis(100));
}

#[test]
fn test_animation_builder_chain() {
    let anim = Animation::fade()
        .duration(500)
        .delay(100)
        .easing(easing::linear);

    assert_eq!(anim.get_duration(), Duration::from_millis(500));
    assert_eq!(anim.get_delay(), Duration::from_millis(100));
    let result = anim.get_easing()(0.5);
    assert_eq!(result, 0.5);
}

// =========================================================================
// Animation getter method tests
// =========================================================================

#[test]
fn test_animation_preset_getter() {
    let anim = Animation::fade();
    assert_eq!(anim.preset(), AnimationPreset::Fade);
}

#[test]
fn test_animation_get_duration() {
    let anim = Animation::fade().duration(750);
    assert_eq!(anim.get_duration(), Duration::from_millis(750));
}

#[test]
fn test_animation_get_easing() {
    let anim = Animation::fade().easing(easing::linear);
    let easing = anim.get_easing();
    assert_eq!(easing(0.5), 0.5);
}

#[test]
fn test_animation_get_delay() {
    let anim = Animation::fade().delay(250);
    assert_eq!(anim.get_delay(), Duration::from_millis(250));
}

// =========================================================================
// Animation Clone implementation tests
// =========================================================================

#[test]
fn test_animation_clone_preserves_preset() {
    let anim = Animation::fade();
    let cloned = anim.clone();
    assert_eq!(anim.preset(), cloned.preset());
}

#[test]
fn test_animation_clone_preserves_duration() {
    let anim = Animation::fade().duration(500);
    let cloned = anim.clone();
    assert_eq!(anim.get_duration(), cloned.get_duration());
}

#[test]
fn test_animation_clone_preserves_easing() {
    let anim = Animation::fade().easing(easing::linear);
    let cloned = anim.clone();
    assert_eq!(anim.get_easing()(0.5), cloned.get_easing()(0.5));
}

#[test]
fn test_animation_clone_preserves_delay() {
    let anim = Animation::fade().delay(100);
    let cloned = anim.clone();
    assert_eq!(anim.get_delay(), cloned.get_delay());
}

#[test]
fn test_animation_clone_independence() {
    let anim1 = Animation::fade().duration(300);
    let anim2 = anim1.clone();
    let anim3 = anim1.duration(500);

    assert_eq!(anim2.get_duration().as_millis(), 300);
    assert_eq!(anim3.get_duration().as_millis(), 500);
}

// =========================================================================
// TransitionPhase enum trait tests (Clone, PartialEq, Eq, Debug)
// =========================================================================

#[test]
fn test_transition_phase_clone() {
    let phase = TransitionPhase::Entering;
    assert_eq!(phase, phase.clone());
}

#[test]
fn test_transition_phase_partial_eq() {
    let entering1 = TransitionPhase::Entering;
    let entering2 = TransitionPhase::Entering;
    let visible = TransitionPhase::Visible;

    assert_eq!(entering1, entering2);
    assert_ne!(entering1, visible);
}

#[test]
fn test_transition_phase_eq_trait() {
    // Eq trait requires PartialEq and reflexivity
    let phase = TransitionPhase::Leaving;
    assert_eq!(phase, phase); // Reflexivity
}

#[test]
fn test_transition_phase_debug() {
    let debug_str = format!("{:?}", TransitionPhase::Entering);
    assert!(debug_str.contains("Entering"));

    let debug_str2 = format!("{:?}", TransitionPhase::Visible);
    assert!(debug_str2.contains("Visible"));

    let debug_str3 = format!("{:?}", TransitionPhase::Leaving);
    assert!(debug_str3.contains("Leaving"));
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_animation_duration_zero() {
    let anim = Animation::fade().duration(0);
    assert_eq!(anim.get_duration(), Duration::ZERO);
}

#[test]
fn test_animation_delay_zero() {
    let anim = Animation::fade().delay(0);
    assert_eq!(anim.get_delay(), Duration::ZERO);
}

#[test]
fn test_animation_duration_large() {
    let anim = Animation::fade().duration(10000);
    assert_eq!(anim.get_duration(), Duration::from_millis(10000));
}

#[test]
fn test_animation_delay_large() {
    let anim = Animation::fade().delay(5000);
    assert_eq!(anim.get_delay(), Duration::from_millis(5000));
}

#[test]
fn test_animation_custom_only_opacity() {
    let anim = Animation::custom(Some(0.5), None, None, None);
    assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
}

#[test]
fn test_animation_custom_only_offset_x() {
    let anim = Animation::custom(None, Some(10), None, None);
    assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
}

#[test]
fn test_animation_custom_only_offset_y() {
    let anim = Animation::custom(None, None, Some(-5), None);
    assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
}

#[test]
fn test_animation_custom_only_scale() {
    let anim = Animation::custom(None, None, None, Some(2.0));
    assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
}

#[test]
fn test_animation_custom_two_params() {
    let anim = Animation::custom(Some(0.5), Some(10), None, None);
    assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
}

#[test]
fn test_animation_easing_bounds() {
    let anim = Animation::fade();
    let easing = anim.get_easing();
    let result_start = easing(0.0);
    let result_mid = easing(0.5);
    let result_end = easing(1.0);
    assert!(result_start >= 0.0 && result_start <= 1.0);
    assert!(result_mid >= 0.0 && result_mid <= 1.0);
    assert!(result_end >= 0.0 && result_end <= 1.0);
}

#[test]
fn test_animation_all_slide_directions_distinct() {
    let left = Animation::slide_left().preset();
    let right = Animation::slide_right().preset();
    let up = Animation::slide_up().preset();
    let down = Animation::slide_down().preset();

    assert_ne!(left, right);
    assert_ne!(left, up);
    assert_ne!(left, down);
    assert_ne!(right, up);
    assert_ne!(right, down);
    assert_ne!(up, down);
}

#[test]
fn test_animation_all_presets_distinct_from_fade() {
    let fade = AnimationPreset::Fade;
    assert_ne!(fade, AnimationPreset::SlideLeft);
    assert_ne!(fade, AnimationPreset::SlideRight);
    assert_ne!(fade, AnimationPreset::SlideUp);
    assert_ne!(fade, AnimationPreset::SlideDown);
    assert_ne!(fade, AnimationPreset::Scale);
}

#[test]
fn test_animation_multiple_chained_builders() {
    let anim = Animation::fade()
        .duration(200)
        .delay(50)
        .easing(easing::linear)
        .duration(400) // Override previous duration
        .delay(100); // Override previous delay

    assert_eq!(anim.get_duration().as_millis(), 400);
    assert_eq!(anim.get_delay().as_millis(), 100);
}

#[test]
fn test_transition_phase_all_variants_creatable() {
    let _entering = TransitionPhase::Entering;
    let _visible = TransitionPhase::Visible;
    let _leaving = TransitionPhase::Leaving;
}

#[test]
fn test_transition_phase_copy_trait() {
    let phase1 = TransitionPhase::Visible;
    let phase2 = phase1; // Copy should work
    assert_eq!(phase1, TransitionPhase::Visible);
    assert_eq!(phase2, TransitionPhase::Visible);
}

#[test]
fn test_animation_custom_with_negative_values() {
    let anim = Animation::custom(Some(-0.5), Some(-100), Some(-50), Some(-1.0));
    assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
}

#[test]
fn test_animation_custom_with_large_values() {
    let anim = Animation::custom(Some(999.9), Some(30000), Some(-30000), Some(100.0));
    assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
}
