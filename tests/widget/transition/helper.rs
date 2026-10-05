//! transition() / transition_group() helper tests

use revue::widget::{
    transition, transition_group, Animation, AnimationTransition as Transition, TransitionGroup,
    TransitionPhase,
};

#[test]
fn test_transition_helper_matches_constructor() {
    let mut from_helper = transition("content").leave(Animation::fade().duration(60_000));
    let mut from_new = Transition::new("content").leave(Animation::fade().duration(60_000));

    assert_eq!(from_helper.is_visible(), from_new.is_visible());
    assert_eq!(from_helper.phase(), from_new.phase());

    from_helper.hide();
    from_new.hide();
    assert_eq!(from_helper.phase(), TransitionPhase::Leaving);
    assert_eq!(from_new.phase(), TransitionPhase::Leaving);
}

#[test]
fn test_transition_group_helper_matches_constructor() {
    let from_helper = transition_group(vec!["a", "b", "c"]);
    let from_new = TransitionGroup::new(vec!["a", "b", "c"]);
    assert_eq!(from_helper.items(), from_new.items());

    let owned = transition_group(vec![String::from("x"), String::from("y")]);
    assert_eq!(owned.items(), &["x", "y"]);

    let empty = transition_group(Vec::<String>::new());
    assert!(empty.is_empty());
}
