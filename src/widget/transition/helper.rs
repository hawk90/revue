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

    #[test]
    fn test_transition_function() {
        let t = transition("test content");
        let _ = t;
    }

    #[test]
    fn test_transition_function_with_string() {
        let t = transition("hello".to_string());
        let _ = t;
    }

    #[test]
    fn test_transition_function_with_str() {
        let t = transition("world");
        let _ = t;
    }

    #[test]
    fn test_transition_group_function_empty() {
        let items: Vec<String> = vec![];
        let tg = transition_group(items);
        let _ = tg;
    }

    #[test]
    fn test_transition_group_function_with_vec() {
        let items = vec!["a", "b", "c"];
        let tg = transition_group(items);
        let _ = tg;
    }

    #[test]
    fn test_transition_group_function_with_strings() {
        let items = vec!["x".to_string(), "y".to_string()];
        let tg = transition_group(items);
        let _ = tg;
    }

    #[test]
    fn test_transition_group_function_with_iter() {
        let tg = transition_group(["1", "2", "3"]);
        let _ = tg;
    }
}
