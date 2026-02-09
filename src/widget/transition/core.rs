//! Transition widget for single element animations

use super::types::{Animation, TransitionPhase};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Transition widget for single element animations
pub struct Transition {
    /// Child widget content (stored as boxed renderable)
    child_content: String,
    /// Enter animation
    enter_animation: Option<Animation>,
    /// Leave animation
    leave_animation: Option<Animation>,
    /// Current transition phase
    phase: TransitionPhase,
    /// Animation tween for progress
    tween: Option<crate::style::Tween>,
    /// Whether the content should be visible
    visible: bool,
    /// Widget properties
    props: WidgetProps,
}

impl Transition {
    /// Create a new transition with content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            child_content: content.into(),
            enter_animation: None,
            leave_animation: None,
            phase: TransitionPhase::Visible,
            tween: None,
            visible: true,
            props: WidgetProps::default(),
        }
    }

    /// Set enter animation
    pub fn enter(mut self, animation: Animation) -> Self {
        self.enter_animation = Some(animation);
        self
    }

    /// Set leave animation
    pub fn leave(mut self, animation: Animation) -> Self {
        self.leave_animation = Some(animation);
        self
    }

    /// Set both enter and leave animations
    pub fn animations(mut self, enter: Animation, leave: Animation) -> Self {
        self.enter_animation = Some(enter);
        self.leave_animation = Some(leave);
        self
    }

    /// Show the content (triggers enter animation if set)
    pub fn show(&mut self) {
        if !self.visible {
            self.visible = true;
            self.phase = TransitionPhase::Entering;
            if let Some(anim) = &self.enter_animation {
                let mut tween = crate::style::Tween::new(0.0, 1.0, anim.get_duration())
                    .easing(anim.get_easing())
                    .delay(anim.get_delay());
                tween.start();
                self.tween = Some(tween);
            }
        }
    }

    /// Hide the content (triggers leave animation if set)
    pub fn hide(&mut self) {
        if self.visible {
            self.phase = TransitionPhase::Leaving;
            if let Some(anim) = &self.leave_animation {
                let mut tween = crate::style::Tween::new(1.0, 0.0, anim.get_duration())
                    .easing(anim.get_easing())
                    .delay(anim.get_delay());
                tween.start();
                self.tween = Some(tween);
            } else {
                self.visible = false;
            }
        }
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        if self.visible {
            self.hide();
        } else {
            self.show();
        }
    }

    /// Get current visibility
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get transition phase
    pub fn phase(&self) -> TransitionPhase {
        self.phase
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self::new("")
    }
}

impl View for Transition {
    crate::impl_view_meta!("Transition");

    fn render(&self, ctx: &mut RenderContext) {
        if !self.visible && self.phase == TransitionPhase::Visible {
            return;
        }

        // Render content directly
        let area = ctx.area;
        let default_bg = Color::BLACK;
        let default_fg = Color::WHITE;

        for (j, ch) in self.child_content.chars().enumerate() {
            let x = area.x + j as u16;
            let y = area.y;
            if x < area.x + area.width && y < area.y + area.height {
                let mut cell = Cell::new(ch);
                cell.fg = Some(default_fg);
                cell.bg = Some(default_bg);
                ctx.buffer.set(x, y, cell);
            }
        }
    }

    // Note: Animation updates would happen via event handling in the widget tree
}

impl_styled_view!(Transition);
impl_props_builders!(Transition);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::transition::types::{Animation, TransitionPhase};

    // =========================================================================
    // Constructor tests
    // =========================================================================

    #[test]
    fn test_transition_new_with_str() {
        let t = Transition::new("test content");
        assert!(t.is_visible());
        assert_eq!(t.phase(), TransitionPhase::Visible);
    }

    #[test]
    fn test_transition_new_with_string() {
        let t = Transition::new("hello".to_string());
        assert!(t.is_visible());
        assert_eq!(t.phase(), TransitionPhase::Visible);
    }

    #[test]
    fn test_transition_new_with_empty_str() {
        let t = Transition::new("");
        assert!(t.is_visible());
        assert_eq!(t.phase(), TransitionPhase::Visible);
    }

    #[test]
    fn test_transition_new_with_long_string() {
        let long_content = "a".repeat(1000);
        let t = Transition::new(long_content.clone());
        assert!(t.is_visible());
        assert_eq!(t.phase(), TransitionPhase::Visible);
    }

    #[test]
    fn test_transition_new_with_unicode() {
        let t = Transition::new("Hello 🎉 World 🔥");
        assert!(t.is_visible());
        assert_eq!(t.phase(), TransitionPhase::Visible);
    }

    #[test]
    fn test_transition_default_creates_empty_content() {
        let t = Transition::default();
        assert!(t.is_visible());
        assert_eq!(t.phase(), TransitionPhase::Visible);
    }

    // =========================================================================
    // Builder method tests
    // =========================================================================

    #[test]
    fn test_transition_enter_builder_returns_self() {
        let t = Transition::new("test").enter(Animation::fade());
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_leave_builder_returns_self() {
        let t = Transition::new("test").leave(Animation::fade());
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_animations_builder_returns_self() {
        let enter = Animation::fade();
        let leave = Animation::slide_left();
        let t = Transition::new("test").animations(enter, leave);
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_builder_chain() {
        let enter = Animation::fade().duration(500).delay(100);
        let leave = Animation::slide_left().duration(300);
        let t = Transition::new("test content")
            .enter(enter.clone())
            .leave(leave.clone());
        assert!(t.is_visible());
        assert_eq!(t.phase(), TransitionPhase::Visible);
    }

    #[test]
    fn test_transition_builder_with_different_animations() {
        let t1 = Transition::new("test").enter(Animation::slide_left());
        let t2 = Transition::new("test").enter(Animation::slide_right());
        let t3 = Transition::new("test").enter(Animation::scale());
        // All should be visible
        assert!(t1.is_visible());
        assert!(t2.is_visible());
        assert!(t3.is_visible());
    }

    // =========================================================================
    // Getter method tests
    // =========================================================================

    #[test]
    fn test_transition_is_visible_returns_true_initially() {
        let t = Transition::new("test");
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_is_visible_after_hide_without_animation() {
        let mut t = Transition::new("test");
        t.hide();
        // Without leave animation, hide() sets visible = false
        assert!(!t.is_visible());
    }

    #[test]
    fn test_transition_phase_returns_visible_initially() {
        let t = Transition::new("test");
        assert_eq!(t.phase(), TransitionPhase::Visible);
    }

    #[test]
    fn test_transition_phase_returns_leaving_after_hide() {
        let mut t = Transition::new("test");
        t.hide();
        assert_eq!(t.phase(), TransitionPhase::Leaving);
    }

    // =========================================================================
    // State-changing method tests
    // =========================================================================

    #[test]
    fn test_transition_show_when_hidden_sets_visible_true() {
        let mut t = Transition::new("test");
        t.hide();
        assert!(!t.is_visible());
        t.show();
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_show_when_visible_remains_visible() {
        let mut t = Transition::new("test");
        assert!(t.is_visible());
        t.show();
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_show_changes_phase_to_entering() {
        let mut t = Transition::new("test");
        t.hide();
        t.show();
        // When showing from hidden, phase becomes Entering
        assert_eq!(t.phase(), TransitionPhase::Entering);
    }

    #[test]
    fn test_transition_hide_when_visible_changes_phase() {
        let mut t = Transition::new("test");
        assert_eq!(t.phase(), TransitionPhase::Visible);
        t.hide();
        assert_eq!(t.phase(), TransitionPhase::Leaving);
    }

    #[test]
    fn test_transition_hide_without_leave_animation_sets_visible_false() {
        let mut t = Transition::new("test");
        t.hide();
        assert!(!t.is_visible());
    }

    #[test]
    fn test_transition_toggle_from_visible_to_hidden() {
        let mut t = Transition::new("test");
        assert!(t.is_visible());
        t.toggle();
        // Without leave animation, toggle sets visible = false
        assert!(!t.is_visible());
    }

    #[test]
    fn test_transition_toggle_from_hidden_to_visible() {
        let mut t = Transition::new("test");
        t.hide();
        assert!(!t.is_visible());
        t.toggle();
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_toggle_multiple_times() {
        let mut t = Transition::new("test");
        assert!(t.is_visible());
        t.toggle();
        assert!(!t.is_visible());
        t.toggle();
        assert!(t.is_visible());
        t.toggle();
        assert!(!t.is_visible());
    }

    // =========================================================================
    // Animation behavior tests
    // =========================================================================

    #[test]
    fn test_transition_with_enter_animation_builder() {
        let anim = Animation::fade().duration(500).delay(100);
        let t = Transition::new("test").enter(anim);
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_with_leave_animation_builder() {
        let anim = Animation::slide_left().duration(300);
        let t = Transition::new("test").leave(anim);
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_with_both_animations_builder() {
        let enter = Animation::fade_in().duration(200);
        let leave = Animation::fade_out().duration(300);
        let t = Transition::new("test").animations(enter, leave);
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_animation_presets_variants() {
        // Test all animation preset variants can be used
        let t1 = Transition::new("test").enter(Animation::fade());
        let t2 = Transition::new("test").enter(Animation::slide_left());
        let t3 = Transition::new("test").enter(Animation::slide_right());
        let t4 = Transition::new("test").enter(Animation::slide_up());
        let t5 = Transition::new("test").enter(Animation::slide_down());
        let t6 = Transition::new("test").enter(Animation::scale());

        assert!(t1.is_visible());
        assert!(t2.is_visible());
        assert!(t3.is_visible());
        assert!(t4.is_visible());
        assert!(t5.is_visible());
        assert!(t6.is_visible());
    }

    #[test]
    fn test_transition_custom_animation() {
        let custom = Animation::custom(Some(0.5), Some(10), Some(-5), Some(0.8));
        let t = Transition::new("test").enter(custom);
        assert!(t.is_visible());
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_transition_with_whitespace_content() {
        let t = Transition::new("   ");
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_with_newline_content() {
        let t = Transition::new("line1\nline2");
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_with_special_characters() {
        let t = Transition::new("!@#$%^&*()_+-=[]{}|;':\",./<>?");
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_multiple_hide_calls() {
        let mut t = Transition::new("test");
        t.hide();
        assert!(!t.is_visible());
        t.hide(); // Second hide should be safe
        assert!(!t.is_visible());
    }

    #[test]
    fn test_transition_multiple_show_calls() {
        let mut t = Transition::new("test");
        t.show();
        assert!(t.is_visible());
        t.show(); // Second show should be safe
        assert!(t.is_visible());
    }

    #[test]
    fn test_transition_show_hide_cycle() {
        let mut t = Transition::new("test");
        assert!(t.is_visible());

        t.hide();
        assert!(!t.is_visible());

        t.show();
        assert!(t.is_visible());

        t.hide();
        assert!(!t.is_visible());

        t.show();
        assert!(t.is_visible());
    }
}
