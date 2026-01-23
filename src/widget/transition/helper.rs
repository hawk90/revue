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
