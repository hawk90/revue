//! Transition and TransitionGroup widgets - helper functions

use super::core::Transition;
use super::group::TransitionGroup;

/// Convenience function to create a Transition
pub fn transition(content: impl Into<String>) -> Transition {
    Transition::new(content)
}

/// Convenience function to create a TransitionGroup
pub fn transition_group(items: impl IntoIterator<Item = impl Into<String>>) -> TransitionGroup {
    TransitionGroup::new(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::transition::types::Animation;

    // =========================================================================
    // transition helper tests
    // =========================================================================

    #[test]
    fn test_transition_function_creates_visible() {
        let t = transition("test content");
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_function_with_string() {
        let t = transition("hello".to_string());
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_function_with_str() {
        let t = transition("world");
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_function_empty() {
        let t = transition("");
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_function_chainable() {
        let t = transition("content")
            .enter(Animation::fade())
            .leave(Animation::fade());
        // Verify the builder methods work
        let _ = t;
    }

    #[test]
    fn test_transition_function_with_animations() {
        let t = transition("test").animations(Animation::fade(), Animation::slide_left());
        let _ = t;
    }

    // =========================================================================
    // transition_group helper tests
    // =========================================================================

    #[test]
    fn test_transition_group_function_empty() {
        let items: Vec<String> = vec![];
        let tg = transition_group(items);
        assert!(tg.is_empty());
        assert_eq!(tg.len(), 0);
    }

    #[test]
    fn test_transition_group_function_with_vec() {
        let items = vec!["a", "b", "c"];
        let tg = transition_group(items);
        assert!(!tg.is_empty());
        assert_eq!(tg.len(), 3);
    }

    #[test]
    fn test_transition_group_function_with_strings() {
        let items = vec!["x".to_string(), "y".to_string()];
        let tg = transition_group(items);
        assert_eq!(tg.len(), 2);
    }

    #[test]
    fn test_transition_group_function_with_iter() {
        let tg = transition_group(["1", "2", "3"]);
        assert_eq!(tg.len(), 3);
    }

    #[test]
    fn test_transition_group_function_items_content() {
        let tg = transition_group(vec!["a", "b", "c"]);
        let items = tg.items();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], "a");
        assert_eq!(items[1], "b");
        assert_eq!(items[2], "c");
    }

    #[test]
    fn test_transition_group_function_chainable() {
        let tg = transition_group(vec!["a", "b"])
            .enter(Animation::fade())
            .leave(Animation::fade())
            .stagger(50);
        let _ = tg;
    }

    #[test]
    fn test_transition_group_function_mixed_types() {
        // Mixed types don't work well with vec! macro, but iterator does
        let items = vec![String::from("x"), "y".to_string(), String::from("z")];
        let tg = transition_group(items);
        assert_eq!(tg.len(), 3);
    }

    #[test]
    fn test_transition_group_function_from_array() {
        let tg = transition_group(["first", "second", "third"]);
        assert_eq!(tg.len(), 3);
        assert_eq!(tg.items()[0], "first");
    }
}
