//! Transition and TransitionGroup widget integration tests

use revue::style::easing;
use revue::widget::{
    transition, transition_group, Animation, AnimationPreset, AnimationTransition as Transition,
    TransitionGroup, TransitionPhase,
};
use std::time::Duration;

// ==================== Animation Tests ====================

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
}

#[test]
fn test_animation_fade_in() {
    let anim = Animation::fade_in();
    assert_eq!(anim.preset(), AnimationPreset::Fade);
}

#[test]
fn test_animation_fade_out() {
    let anim = Animation::fade_out();
    assert_eq!(anim.preset(), AnimationPreset::Fade);
}

#[test]
fn test_animation_slide_left() {
    let anim = Animation::slide_left();
    assert_eq!(anim.preset(), AnimationPreset::SlideLeft);
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
fn test_animation_slide_right() {
    let anim = Animation::slide_right();
    assert_eq!(anim.preset(), AnimationPreset::SlideRight);
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
fn test_animation_slide_up() {
    let anim = Animation::slide_up();
    assert_eq!(anim.preset(), AnimationPreset::SlideUp);
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
fn test_animation_slide_down() {
    let anim = Animation::slide_down();
    assert_eq!(anim.preset(), AnimationPreset::SlideDown);
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
fn test_animation_custom() {
    let anim = Animation::custom(Some(0.5), Some(10), Some(-5), Some(0.8));
    assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
}

#[test]
fn test_animation_custom_all_none() {
    let anim = Animation::custom(None, None, None, None);
    assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
}

#[test]
fn test_animation_custom_partial_values() {
    let anim1 = Animation::custom(Some(0.5), None, None, None);
    let anim2 = Animation::custom(None, Some(10), None, None);
    let anim3 = Animation::custom(None, None, Some(-5), None);
    let anim4 = Animation::custom(None, None, None, Some(0.8));

    assert!(matches!(anim1.preset(), AnimationPreset::Custom { .. }));
    assert!(matches!(anim2.preset(), AnimationPreset::Custom { .. }));
    assert!(matches!(anim3.preset(), AnimationPreset::Custom { .. }));
    assert!(matches!(anim4.preset(), AnimationPreset::Custom { .. }));
}

#[test]
fn test_animation_duration() {
    let anim = Animation::fade().duration(500);
    assert_eq!(anim.get_duration(), Duration::from_millis(500));
}

#[test]
fn test_animation_duration_various() {
    assert_eq!(
        Animation::fade().duration(100).get_duration().as_millis(),
        100
    );
    assert_eq!(
        Animation::fade().duration(500).get_duration().as_millis(),
        500
    );
    assert_eq!(
        Animation::fade().duration(1000).get_duration().as_millis(),
        1000
    );
    assert_eq!(
        Animation::fade().duration(5000).get_duration().as_millis(),
        5000
    );
}

#[test]
fn test_animation_delay() {
    let anim = Animation::fade().delay(100);
    assert_eq!(anim.get_delay(), Duration::from_millis(100));
}

#[test]
fn test_animation_delay_various() {
    assert_eq!(
        Animation::fade().delay(0).get_delay(),
        Duration::from_millis(0)
    );
    assert_eq!(
        Animation::fade().delay(50).get_delay(),
        Duration::from_millis(50)
    );
    assert_eq!(
        Animation::fade().delay(250).get_delay(),
        Duration::from_millis(250)
    );
    assert_eq!(
        Animation::fade().delay(1000).get_delay(),
        Duration::from_millis(1000)
    );
}

#[test]
fn test_animation_easing() {
    let anim = Animation::fade().easing(easing::linear);
    // Test that the easing function can be called
    let result = anim.get_easing()(0.5);
    assert_eq!(result, 0.5); // linear should return the same value
}

#[test]
fn test_animation_easing_various() {
    let anim1 = Animation::fade().easing(easing::linear);
    let anim2 = Animation::fade().easing(easing::ease_in);
    let anim3 = Animation::fade().easing(easing::ease_out);
    let anim4 = Animation::fade().easing(easing::ease_in_out);

    // All should produce valid f32 values
    let val = 0.5;
    assert!((anim1.get_easing()(val) - 0.5).abs() < 1.0);
    assert!((anim2.get_easing()(val) - 0.0).abs() < 1.0);
    assert!((anim3.get_easing()(val) - 0.0).abs() < 1.0);
    assert!((anim4.get_easing()(val) - 0.0).abs() < 1.0);
}

#[test]
fn test_animation_preset() {
    let fade = Animation::fade();
    assert_eq!(fade.preset(), AnimationPreset::Fade);

    let slide = Animation::slide_left();
    assert_eq!(slide.preset(), AnimationPreset::SlideLeft);

    let scale = Animation::scale();
    assert_eq!(scale.preset(), AnimationPreset::Scale);
}

#[test]
fn test_animation_get_duration() {
    let anim = Animation::fade().duration(500);
    assert_eq!(anim.get_duration(), Duration::from_millis(500));
}

#[test]
fn test_animation_get_delay() {
    let anim = Animation::fade().delay(100);
    assert_eq!(anim.get_delay(), Duration::from_millis(100));
}

#[test]
fn test_animation_get_easing() {
    let anim = Animation::fade().easing(easing::linear);
    let easing_fn = anim.get_easing();
    assert_eq!(easing_fn(0.5), 0.5);
}

#[test]
fn test_animation_clone() {
    let anim1 = Animation::fade().duration(500).easing(easing::linear);
    let anim2 = anim1.clone();

    assert_eq!(anim1.preset(), anim2.preset());
    assert_eq!(anim1.get_duration(), anim2.get_duration());
    assert_eq!(anim1.get_delay(), anim2.get_delay());
}

#[test]
fn test_animation_builder_chain() {
    let anim = Animation::fade()
        .duration(500)
        .delay(100)
        .easing(easing::linear);

    assert_eq!(anim.get_duration(), Duration::from_millis(500));
    assert_eq!(anim.get_delay(), Duration::from_millis(100));
}

#[test]
fn test_animation_preset_all_variants() {
    let _ = Animation::fade();
    let _ = Animation::slide_left();
    let _ = Animation::slide_right();
    let _ = Animation::slide_up();
    let _ = Animation::slide_down();
    let _ = Animation::scale();
    let _ = Animation::custom(Some(0.5), Some(10), Some(-5), Some(0.8));
}

// ==================== AnimationPreset Tests ====================

#[test]
fn test_animation_preset_fade() {
    assert_eq!(AnimationPreset::Fade, AnimationPreset::Fade);
}

#[test]
fn test_animation_preset_slide_left() {
    assert_eq!(AnimationPreset::SlideLeft, AnimationPreset::SlideLeft);
}

#[test]
fn test_animation_preset_slide_right() {
    assert_eq!(AnimationPreset::SlideRight, AnimationPreset::SlideRight);
}

#[test]
fn test_animation_preset_slide_up() {
    assert_eq!(AnimationPreset::SlideUp, AnimationPreset::SlideUp);
}

#[test]
fn test_animation_preset_slide_down() {
    assert_eq!(AnimationPreset::SlideDown, AnimationPreset::SlideDown);
}

#[test]
fn test_animation_preset_scale() {
    assert_eq!(AnimationPreset::Scale, AnimationPreset::Scale);
}

#[test]
fn test_animation_preset_custom() {
    let custom1 = AnimationPreset::Custom {
        opacity: Some(0.5),
        offset_x: Some(10),
        offset_y: Some(-5),
        scale: Some(0.8),
    };
    let custom2 = AnimationPreset::Custom {
        opacity: Some(0.5),
        offset_x: Some(10),
        offset_y: Some(-5),
        scale: Some(0.8),
    };
    assert_eq!(custom1, custom2);
}

#[test]
fn test_animation_preset_partial_equality() {
    let fade1 = Animation::fade();
    let fade2 = Animation::fade();
    assert_eq!(fade1.preset(), fade2.preset());

    let slide = Animation::slide_left();
    assert_ne!(fade1.preset(), slide.preset());
}

// ==================== TransitionPhase Tests ====================

#[test]
fn test_transition_phase_entering() {
    let phase = TransitionPhase::Entering;
    assert_eq!(phase, TransitionPhase::Entering);
}

#[test]
fn test_transition_phase_visible() {
    let phase = TransitionPhase::Visible;
    assert_eq!(phase, TransitionPhase::Visible);
}

#[test]
fn test_transition_phase_leaving() {
    let phase = TransitionPhase::Leaving;
    assert_eq!(phase, TransitionPhase::Leaving);
}

#[test]
fn test_transition_phase_all_variants() {
    let entering = TransitionPhase::Entering;
    let visible = TransitionPhase::Visible;
    let leaving = TransitionPhase::Leaving;

    assert_eq!(entering, TransitionPhase::Entering);
    assert_eq!(visible, TransitionPhase::Visible);
    assert_eq!(leaving, TransitionPhase::Leaving);
}

#[test]
fn test_transition_phase_inequality() {
    let entering = TransitionPhase::Entering;
    let visible = TransitionPhase::Visible;
    let leaving = TransitionPhase::Leaving;

    assert_ne!(entering, visible);
    assert_ne!(visible, leaving);
    assert_ne!(entering, leaving);
}

#[test]
fn test_transition_phase_clone() {
    let phase = TransitionPhase::Entering;
    assert_eq!(phase, phase.clone());
}

// ==================== Transition Widget Tests ====================

#[test]
fn test_transition_new() {
    let t = Transition::new("Hello");
    assert!(t.is_visible());
}

#[test]
fn test_transition_new_empty() {
    let t = Transition::new("");
    assert!(t.is_visible());
}

#[test]
fn test_transition_new_string() {
    let t = Transition::new(String::from("World"));
    assert!(t.is_visible());
}

#[test]
fn test_transition_default() {
    let t = Transition::default();
    assert!(t.is_visible());
    assert_eq!(t.phase(), TransitionPhase::Visible);
}

#[test]
fn test_transition_enter() {
    let enter = Animation::fade_in();
    let _t = Transition::new("Test").enter(enter);
}

#[test]
fn test_transition_leave() {
    let leave = Animation::fade_out();
    let _t = Transition::new("Test").leave(leave);
}

#[test]
fn test_transition_animations() {
    let enter = Animation::fade_in();
    let leave = Animation::fade_out();
    let _t = Transition::new("Test").animations(enter, leave);
}

#[test]
fn test_transition_is_visible_initially() {
    let t = Transition::new("Content");
    assert!(t.is_visible());
}

#[test]
fn test_transition_show() {
    let mut t = Transition::new("Content");
    t.show();
    assert!(t.is_visible());
}

#[test]
fn test_transition_hide() {
    let mut t = Transition::new("Content");
    t.hide();
    // With no leave animation, visible is set to false immediately
    assert!(!t.is_visible());
}

#[test]
fn test_transition_toggle_from_visible() {
    let mut t = Transition::new("Content");
    t.toggle();
    // With no leave animation, visible becomes false
    assert!(!t.is_visible());
}

#[test]
fn test_transition_toggle_multiple() {
    let mut t = Transition::new("Content");
    t.toggle();
    t.toggle();
    assert!(t.is_visible());
}

#[test]
fn test_transition_phase_initial() {
    let t = Transition::new("Content");
    assert_eq!(t.phase(), TransitionPhase::Visible);
}

#[test]
fn test_transition_show_when_visible() {
    let mut t = Transition::new("Content");
    let _initial_phase = t.phase();
    t.show();
    assert!(t.is_visible());
}

#[test]
fn test_transition_with_various_content() {
    let _t1 = Transition::new("String content");
    let _t2 = Transition::new(String::from("Owned string"));
    let _t3 = Transition::new("");
    let _t4 = Transition::new(String::from(""));
}

#[test]
fn test_transition_builder_chain() {
    let enter = Animation::slide_left();
    let leave = Animation::slide_right();
    let _t = Transition::new("Content").enter(enter).leave(leave);
}

// ==================== TransitionGroup Widget Tests ====================

#[test]
fn test_transition_group_new() {
    let group = TransitionGroup::new(vec!["A", "B", "C"]);
    assert_eq!(group.len(), 3);
    assert!(!group.is_empty());
}

#[test]
fn test_transition_group_new_empty() {
    let group: TransitionGroup = TransitionGroup::new(Vec::<String>::new());
    assert_eq!(group.len(), 0);
    assert!(group.is_empty());
}

#[test]
fn test_transition_group_new_from_iterator() {
    let group = TransitionGroup::new(vec!["A", "B", "C"]);
    assert_eq!(group.len(), 3);
}

#[test]
fn test_transition_group_default() {
    let group = TransitionGroup::default();
    assert_eq!(group.len(), 0);
    assert!(group.is_empty());
}

#[test]
fn test_transition_group_enter() {
    let enter = Animation::fade_in();
    let _group = TransitionGroup::new(vec!["A", "B"]).enter(enter);
}

#[test]
fn test_transition_group_leave() {
    let leave = Animation::fade_out();
    let _group = TransitionGroup::new(vec!["A", "B"]).leave(leave);
}

#[test]
fn test_transition_group_move_animation() {
    let move_anim = Animation::slide_left();
    let _group = TransitionGroup::new(vec!["A", "B"]).move_animation(move_anim);
}

#[test]
fn test_transition_group_stagger() {
    let _group = TransitionGroup::new(vec!["A", "B"]).stagger(50);
}

#[test]
fn test_transition_group_stagger_various() {
    let _g1 = TransitionGroup::new(vec!["A"]).stagger(0);
    let _g2 = TransitionGroup::new(vec!["A"]).stagger(50);
    let _g3 = TransitionGroup::new(vec!["A"]).stagger(100);
    let _g4 = TransitionGroup::new(vec!["A"]).stagger(500);
}

#[test]
fn test_transition_group_push() {
    let mut group = TransitionGroup::new(vec!["A"]);
    assert_eq!(group.len(), 1);

    group.push("B");
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_push_various_types() {
    let mut group = TransitionGroup::new(vec!["A"]);
    group.push("B");
    group.push(String::from("C"));
    group.push(&String::from("D"));

    assert_eq!(group.len(), 4);
}

#[test]
fn test_transition_group_push_multiple() {
    let mut group = TransitionGroup::new(Vec::<String>::new());
    group.push("A");
    group.push("B");
    group.push("C");
    group.push("D");
    group.push("E");

    assert_eq!(group.len(), 5);
}

#[test]
fn test_transition_group_remove() {
    let mut group = TransitionGroup::new(vec!["A", "B", "C"]);
    let removed = group.remove(1);

    assert_eq!(removed, Some(String::from("B")));
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove_first() {
    let mut group = TransitionGroup::new(vec!["A", "B", "C"]);
    let removed = group.remove(0);

    assert_eq!(removed, Some(String::from("A")));
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove_last() {
    let mut group = TransitionGroup::new(vec!["A", "B", "C"]);
    let removed = group.remove(2);

    assert_eq!(removed, Some(String::from("C")));
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove_out_of_bounds() {
    let mut group = TransitionGroup::new(vec!["A", "B"]);
    let removed = group.remove(5);

    assert_eq!(removed, None);
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove_empty() {
    let mut group = TransitionGroup::new(Vec::<String>::new());
    let removed = group.remove(0);

    assert_eq!(removed, None);
    assert_eq!(group.len(), 0);
}

#[test]
fn test_transition_group_len() {
    let group = TransitionGroup::new(vec!["A", "B", "C"]);
    assert_eq!(group.len(), 3);
}

#[test]
fn test_transition_group_len_empty() {
    let group: TransitionGroup = TransitionGroup::new(Vec::<String>::new());
    assert_eq!(group.len(), 0);
}

#[test]
fn test_transition_group_is_empty() {
    let group: TransitionGroup = TransitionGroup::new(Vec::<String>::new());
    assert!(group.is_empty());
}

#[test]
fn test_transition_group_is_empty_false() {
    let group = TransitionGroup::new(vec!["A"]);
    assert!(!group.is_empty());
}

#[test]
fn test_transition_group_items() {
    let group = TransitionGroup::new(vec!["A", "B", "C"]);
    let items = group.items();
    assert_eq!(items, &["A", "B", "C"]);
}

#[test]
fn test_transition_group_items_empty() {
    let group: TransitionGroup = TransitionGroup::new(Vec::<String>::new());
    assert!(group.items().is_empty());
}

#[test]
fn test_transition_group_builder_chain() {
    let _group = TransitionGroup::new(vec!["A", "B"])
        .enter(Animation::fade_in())
        .leave(Animation::fade_out())
        .stagger(50);
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

// ==================== Helper Functions Tests ====================

#[test]
fn test_transition_helper() {
    let t = transition("Hello");
    assert!(t.is_visible());
}

#[test]
fn test_transition_helper_empty() {
    let t = transition("");
    assert!(t.is_visible());
}

#[test]
fn test_transition_helper_string() {
    let t = transition(String::from("World"));
    assert!(t.is_visible());
}

#[test]
fn test_transition_group_helper() {
    let group = transition_group(vec!["A", "B", "C"]);
    assert_eq!(group.len(), 3);
}

#[test]
fn test_transition_group_helper_empty() {
    let group: TransitionGroup = transition_group(Vec::<String>::new());
    assert_eq!(group.len(), 0);
    assert!(group.is_empty());
}

#[test]
fn test_transition_group_helper_from_vec() {
    let items = vec!["A", "B", "C"];
    let group = transition_group(items);
    assert_eq!(group.len(), 3);
}

#[test]
fn test_transition_group_helper_from_iterator() {
    let group = transition_group(vec!["A", "B", "C"]);
    assert_eq!(group.len(), 3);
}

// ==================== Edge Cases and Integration Tests ====================

#[test]
fn test_animation_with_zero_duration() {
    let anim = Animation::fade().duration(0);
    assert_eq!(anim.get_duration(), Duration::from_millis(0));
}

#[test]
fn test_animation_with_large_duration() {
    let anim = Animation::fade().duration(100000);
    assert_eq!(anim.get_duration(), Duration::from_millis(100000));
}

#[test]
fn test_animation_with_zero_delay() {
    let anim = Animation::fade().delay(0);
    assert_eq!(anim.get_delay(), Duration::ZERO);
}

#[test]
fn test_animation_with_negative_offset_custom() {
    let anim = Animation::custom(None, Some(-100), Some(-50), None);
    assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
}

#[test]
fn test_animation_with_negative_opacity_custom() {
    let anim = Animation::custom(Some(-1.0), None, None, None);
    assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
}

#[test]
fn test_animation_with_scale_greater_than_one() {
    let anim = Animation::custom(None, None, None, Some(2.0));
    assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
}

#[test]
fn test_transition_with_long_content() {
    let long_content = "This is a very long string that should be rendered properly";
    let t = Transition::new(long_content);
    assert!(t.is_visible());
}

#[test]
fn test_transition_with_special_characters() {
    let special = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
    let t = Transition::new(special);
    assert!(t.is_visible());
}

#[test]
fn test_transition_with_unicode() {
    let unicode = "Hello 世界 🌍";
    let t = Transition::new(unicode);
    assert!(t.is_visible());
}

#[test]
fn test_transition_group_with_long_items() {
    let long_items = vec![
        "This is a very long item",
        "Another very long item",
        "Yet another long item",
    ];
    let group = TransitionGroup::new(long_items);
    assert_eq!(group.len(), 3);
}

#[test]
fn test_transition_group_with_special_characters() {
    let items = vec!["!@#$%", "^&*()", "[]{}|"];
    let group = TransitionGroup::new(items);
    assert_eq!(group.len(), 3);
}

#[test]
fn test_transition_group_with_unicode() {
    let items = vec!["Hello 世界", "🌍 Globe", "🎉 Party"];
    let group = TransitionGroup::new(items);
    assert_eq!(group.len(), 3);
}

#[test]
fn test_animation_preset_debug() {
    let fade = AnimationPreset::Fade;
    let debug_str = format!("{:?}", fade);
    assert!(debug_str.contains("Fade"));
}

#[test]
fn test_transition_phase_debug() {
    let phase = TransitionPhase::Visible;
    let debug_str = format!("{:?}", phase);
    assert!(debug_str.contains("Visible"));
}

#[test]
fn test_animation_multiple_calls_to_easing() {
    let anim = Animation::fade().easing(easing::linear);
    let easing_fn = anim.get_easing();

    // Multiple calls should work
    assert_eq!(easing_fn(0.0), 0.0);
    assert_eq!(easing_fn(0.5), 0.5);
    assert_eq!(easing_fn(1.0), 1.0);
}

#[test]
fn test_transition_group_remove_all_items() {
    let mut group = TransitionGroup::new(vec!["A", "B", "C"]);
    group.remove(0);
    group.remove(0);
    group.remove(0);

    assert_eq!(group.len(), 0);
    assert!(group.is_empty());
}

#[test]
fn test_transition_group_items_immutability() {
    let group = TransitionGroup::new(vec!["A", "B", "C"]);
    let items1 = group.items();
    let items2 = group.items();

    // Both should return the same slice
    assert_eq!(items1, items2);
}

#[test]
fn test_animation_clone_preserves_all_properties() {
    let anim1 = Animation::fade()
        .duration(500)
        .delay(100)
        .easing(easing::linear);

    let anim2 = anim1.clone();

    assert_eq!(anim1.preset(), anim2.preset());
    assert_eq!(anim1.get_duration(), anim2.get_duration());
    assert_eq!(anim1.get_delay(), anim2.get_delay());
}
