//! TransitionGroup for animating lists with automatic reordering

use super::types::Animation;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// TransitionGroup for animating lists with automatic reordering
pub struct TransitionGroup {
    /// List of items
    items: Vec<String>,
    /// Enter animation
    enter_animation: Option<Animation>,
    /// Leave animation
    leave_animation: Option<Animation>,
    /// Move/Reorder animation
    move_animation: Option<Animation>,
    /// Stagger delay in milliseconds
    stagger_delay: u64,
    /// Widget properties
    props: WidgetProps,
}

impl TransitionGroup {
    /// Create a new transition group with items
    pub fn new(items: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let items: Vec<String> = items.into_iter().map(|s| s.into()).collect();
        Self {
            items,
            enter_animation: None,
            leave_animation: None,
            move_animation: None,
            stagger_delay: 0,
            props: WidgetProps::default(),
        }
    }

    /// Set enter animation for items
    pub fn enter(mut self, animation: Animation) -> Self {
        self.enter_animation = Some(animation);
        self
    }

    /// Set leave animation for items
    pub fn leave(mut self, animation: Animation) -> Self {
        self.leave_animation = Some(animation);
        self
    }

    /// Set move/reorder animation
    pub fn move_animation(mut self, animation: Animation) -> Self {
        self.move_animation = Some(animation);
        self
    }

    /// Set stagger delay between item animations
    pub fn stagger(mut self, delay_ms: u64) -> Self {
        self.stagger_delay = delay_ms;
        self
    }

    /// Add an item to the group
    pub fn push(&mut self, item: impl Into<String>) {
        self.items.push(item.into());
    }

    /// Remove an item from the group
    pub fn remove(&mut self, index: usize) -> Option<String> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    /// Get the number of items
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the group is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get items
    pub fn items(&self) -> &[String] {
        &self.items
    }
}

impl Default for TransitionGroup {
    fn default() -> Self {
        Self::new(Vec::<String>::new())
    }
}

impl View for TransitionGroup {
    crate::impl_view_meta!("TransitionGroup");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let default_bg = Color::BLACK;
        let default_fg = Color::WHITE;

        let mut y = area.y;

        // Render each item
        for item in self.items.iter() {
            if y >= area.y + area.height {
                break;
            }

            // Render item
            for (j, ch) in item.chars().enumerate() {
                let x = area.x + j as u16;
                if x < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(default_fg);
                    cell.bg = Some(default_bg);
                    ctx.buffer.set(x, y, cell);
                }
            }

            y += 1;
        }
    }
}

impl_styled_view!(TransitionGroup);
impl_props_builders!(TransitionGroup);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::transition::types::Animation;

    #[test]
    fn test_transition_group_new() {
        let group = TransitionGroup::new(vec!["a", "b", "c"]);
        assert_eq!(group.len(), 3);
        assert!(!group.is_empty());
    }

    #[test]
    fn test_transition_group_new_empty() {
        let group = TransitionGroup::new(std::iter::empty::<&str>());
        assert_eq!(group.len(), 0);
        assert!(group.is_empty());
    }

    #[test]
    fn test_transition_group_from_vec() {
        let items = vec!["x", "y", "z"];
        let group = TransitionGroup::new(items);
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
        let group = TransitionGroup::new(vec!["a"]).enter(Animation::fade());
        let _ = group;
    }

    #[test]
    fn test_transition_group_leave() {
        let group = TransitionGroup::new(vec!["a"]).leave(Animation::fade());
        let _ = group;
    }

    #[test]
    fn test_transition_group_move_animation() {
        let group = TransitionGroup::new(vec!["a"]).move_animation(Animation::slide_left());
        let _ = group;
    }

    #[test]
    fn test_transition_group_stagger() {
        let group = TransitionGroup::new(vec!["a", "b"]).stagger(100);
        let _ = group;
    }

    #[test]
    fn test_transition_group_push() {
        let mut group = TransitionGroup::new(vec!["a"]);
        assert_eq!(group.len(), 1);
        group.push("b");
        assert_eq!(group.len(), 2);
    }

    #[test]
    fn test_transition_group_push_string() {
        let mut group = TransitionGroup::new(vec!["a"]);
        group.push("b".to_string());
        assert_eq!(group.len(), 2);
    }

    #[test]
    fn test_transition_group_remove_valid() {
        let mut group = TransitionGroup::new(vec!["a", "b", "c"]);
        let removed = group.remove(1);
        assert_eq!(removed, Some("b".to_string()));
        assert_eq!(group.len(), 2);
    }

    #[test]
    fn test_transition_group_remove_invalid() {
        let mut group = TransitionGroup::new(vec!["a", "b"]);
        let removed = group.remove(5);
        assert_eq!(removed, None);
        assert_eq!(group.len(), 2);
    }

    #[test]
    fn test_transition_group_len() {
        let group = TransitionGroup::new(vec!["a", "b", "c", "d"]);
        assert_eq!(group.len(), 4);
    }

    #[test]
    fn test_transition_group_is_empty() {
        let mut group = TransitionGroup::new(vec!["a"]);
        assert!(!group.is_empty());
        group.remove(0);
        assert!(group.is_empty());
    }

    #[test]
    fn test_transition_group_items() {
        let items = vec!["x", "y", "z"];
        let group = TransitionGroup::new(items.clone());
        let retrieved = group.items();
        assert_eq!(retrieved.len(), 3);
        assert_eq!(retrieved[0], "x");
        assert_eq!(retrieved[1], "y");
        assert_eq!(retrieved[2], "z");
    }
}
