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
