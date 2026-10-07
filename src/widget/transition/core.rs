//! Transition widget for single element animations

use super::types::{Animation, TransitionPhase};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Transition widget for single element animations
#[derive(Clone)]
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
    ///
    /// Also reverses a leave animation that is still running.
    pub fn show(&mut self) {
        if !self.visible || self.phase == TransitionPhase::Leaving {
            self.visible = true;
            if let Some(anim) = &self.enter_animation {
                self.phase = TransitionPhase::Entering;
                let mut tween = crate::style::Tween::new(0.0, 1.0, anim.get_duration())
                    .easing(anim.get_easing())
                    .delay(anim.get_delay());
                tween.start();
                self.tween = Some(tween);
            } else {
                // Nothing to animate: the content is fully shown right away
                self.phase = TransitionPhase::Visible;
                self.tween = None;
            }
        }
    }

    /// Hide the content (triggers leave animation if set)
    pub fn hide(&mut self) {
        if self.visible {
            if let Some(anim) = &self.leave_animation {
                self.phase = TransitionPhase::Leaving;
                let mut tween = crate::style::Tween::new(1.0, 0.0, anim.get_duration())
                    .easing(anim.get_easing())
                    .delay(anim.get_delay());
                tween.start();
                self.tween = Some(tween);
            } else {
                // Nothing to animate: the content is gone right away
                self.phase = TransitionPhase::Hidden;
                self.visible = false;
                self.tween = None;
            }
        }
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        // Content that is on its way out counts as hidden
        if self.visible && self.phase != TransitionPhase::Leaving {
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

    /// Update animation state (call each frame/tick)
    pub fn update(&mut self) {
        if let Some(ref mut tween) = self.tween {
            // Call value() to advance the tween's internal state
            let _ = tween.value();
            if tween.is_completed() {
                match self.phase {
                    TransitionPhase::Entering => {
                        self.phase = TransitionPhase::Visible;
                        self.tween = None;
                    }
                    TransitionPhase::Leaving => {
                        self.phase = TransitionPhase::Hidden;
                        self.visible = false;
                        self.tween = None;
                    }
                    _ => {}
                }
            }
        }
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
        // Hidden phase: don't render anything
        if self.phase == TransitionPhase::Hidden {
            return;
        }

        // Not visible and not animating: don't render
        if !self.visible && self.phase == TransitionPhase::Visible {
            return;
        }

        // Determine animation progress for fade effect
        let progress = if let Some(ref tween) = self.tween {
            // Use progress() (immutable) to get 0.0-1.0 without needing &mut self
            tween.progress()
        } else {
            match self.phase {
                TransitionPhase::Visible => 1.0,
                TransitionPhase::Hidden => 0.0,
                // No tween but in transition phase: fully visible
                _ => 1.0,
            }
        };

        // Calculate effective progress based on phase
        let effective_progress = match self.phase {
            TransitionPhase::Entering => progress,
            TransitionPhase::Leaving => 1.0 - progress,
            TransitionPhase::Visible => 1.0,
            TransitionPhase::Hidden => 0.0,
        };

        // Render content with animation applied
        let area = ctx.area;
        // The transitioning content takes the stylesheet; the dimming applied
        // mid-animation is progress, not a color choice.
        let default_bg = ctx.css_background(Color::BLACK);
        let default_fg = ctx.css_color(Color::WHITE);
        let content_len = self.child_content.chars().count();
        let chars_to_show = (effective_progress * content_len as f32).round() as usize;

        // Determine if we should dim (partial opacity)
        let should_dim = effective_progress > 0.0 && effective_progress < 1.0;

        // Laid out in terminal columns: a wide glyph takes two (the second a
        // continuation cell), a zero-width char none
        let mut x: u16 = 0;
        let mut glyph = [0u8; 4];
        for (j, ch) in self.child_content.chars().enumerate() {
            let cw = crate::utils::char_width(ch) as u16;
            if x.saturating_add(cw) <= area.width && area.height > 0 {
                if j < chars_to_show {
                    ctx.put_str_with(x, 0, ch.encode_utf8(&mut glyph), area.width, |ch| {
                        let mut c = Cell::new(ch).fg(default_fg).bg(default_bg);
                        if should_dim {
                            c.modifier |= Modifier::DIM;
                        }
                        c
                    });
                } else {
                    // Character not yet revealed: blank every column it takes
                    for dx in 0..cw {
                        let mut c = Cell::new(' ');
                        c.bg = Some(default_bg);
                        ctx.set(x + dx, 0, c);
                    }
                }
            }
            x = x.saturating_add(cw);
        }
    }

    // Note: Animation updates would happen via event handling in the widget tree
}

impl_styled_view!(Transition);
impl_props_builders!(Transition);
