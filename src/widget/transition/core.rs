//! Transition widget for single element animations

use super::types::{Animation, AnimationPreset, TransitionPhase};
use crate::render::Cell;
use crate::style::{AnimationState as TweenState, Color, Tween};
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
    tween: Option<Tween>,
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
                let mut tween = Tween::new(0.0, 1.0, anim.get_duration())
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
                let mut tween = Tween::new(1.0, 0.0, anim.get_duration())
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
        self.phase.clone()
    }

    /// Calculate animation progress (0.0 to 1.0)
    #[allow(dead_code)]
    fn get_progress(&mut self) -> f32 {
        self.tween.as_mut().map(|t| t.value()).unwrap_or(1.0)
    }

    /// Apply animation transformation to content
    #[allow(dead_code)]
    fn apply_animation(&mut self, content: &str, ctx: &mut RenderContext) {
        let progress = self.get_progress();
        let area = ctx.area;

        // Get the animation config based on current phase
        let anim = match self.phase {
            TransitionPhase::Entering => self.enter_animation.as_ref(),
            TransitionPhase::Leaving => self.leave_animation.as_ref(),
            TransitionPhase::Visible => {
                // No animation when visible, render content normally
                self.render_content(content, ctx, 1.0, 0, 0);
                return;
            }
        };

        let Some(animation) = anim else {
            // No animation configured, render content normally
            self.render_content(content, ctx, 1.0, 0, 0);
            return;
        };

        let preset = animation.preset();

        // Calculate animation parameters
        let (opacity, offset_x, offset_y) = match preset {
            AnimationPreset::Fade => (progress, 0, 0),
            AnimationPreset::SlideLeft => {
                let offset = ((1.0 - progress) * area.width as f32 * 0.5) as i16;
                (1.0, -offset, 0)
            }
            AnimationPreset::SlideRight => {
                let offset = ((1.0 - progress) * area.width as f32 * 0.5) as i16;
                (1.0, offset, 0)
            }
            AnimationPreset::SlideUp => {
                let offset = ((1.0 - progress) * area.height as f32 * 0.5) as i16;
                (1.0, 0, -offset)
            }
            AnimationPreset::SlideDown => {
                let offset = ((1.0 - progress) * area.height as f32 * 0.5) as i16;
                (1.0, 0, offset)
            }
            AnimationPreset::Scale => {
                // Scale affects rendered size
                (progress, 0, 0)
            }
            AnimationPreset::Custom {
                opacity: o,
                offset_x: ox,
                offset_y: oy,
                scale: _,
            } => {
                // For custom animations, interpolate values
                let enter = matches!(self.phase, TransitionPhase::Entering);
                let opacity = o.unwrap_or(if enter { 0.0 } else { 1.0 });
                let target_opacity = if enter { 1.0 } else { 0.0 };
                let current_opacity = opacity + (target_opacity - opacity) * progress;

                let offset_x = ox.unwrap_or(0);
                let offset_y = oy.unwrap_or(0);

                (current_opacity, offset_x, offset_y)
            }
        };

        self.render_content(content, ctx, opacity, offset_x, offset_y);
    }

    /// Render content with transformations applied
    #[allow(dead_code)]
    fn render_content(
        &self,
        content: &str,
        ctx: &mut RenderContext,
        opacity: f32,
        offset_x: i16,
        offset_y: i16,
    ) {
        let area = ctx.area;

        // Skip rendering if opacity is too low
        if opacity < 0.01 {
            return;
        }

        // Get default colors - use terminal defaults
        let default_bg = Color::BLACK;
        let default_fg = Color::WHITE;

        // Calculate actual render position with offset
        let base_x = area.x.saturating_add_signed(offset_x);
        let base_y = area.y.saturating_add_signed(offset_y);

        // For now, render as simple text (full widget rendering would require more complex handling)
        let mut rendered = false;
        for (j, ch) in content.chars().enumerate() {
            let x = base_x + j as u16;
            let y = base_y;
            if x < area.x + area.width && y < area.y + area.height {
                let mut cell = Cell::new(ch);
                cell.fg = Some(default_fg);
                cell.bg = Some(default_bg);

                // Apply opacity by dimming
                if opacity < 0.5 {
                    cell.modifier |= crate::render::Modifier::DIM;
                }

                ctx.buffer.set(x, y, cell);
                rendered = true;
            }
        }

        // If no content was rendered (off-screen or empty), at least render a placeholder
        if !rendered && base_x < area.x + area.width && base_y < area.y + area.height {
            let mut cell = Cell::new(' ');
            cell.fg = Some(default_fg);
            cell.bg = Some(default_bg);
            ctx.buffer.set(base_x, base_y, cell);
        }
    }

    /// Update animation state
    #[allow(dead_code)]
    fn update_animation(&mut self) {
        if self.tween.is_some() {
            let state = self
                .tween
                .as_ref()
                .map(|t| t.state())
                .unwrap_or(TweenState::Pending);

            if state == TweenState::Completed {
                self.tween = None;

                match self.phase {
                    TransitionPhase::Entering => {
                        self.phase = TransitionPhase::Visible;
                    }
                    TransitionPhase::Leaving => {
                        self.visible = false;
                        self.phase = TransitionPhase::Visible;
                    }
                    TransitionPhase::Visible => {}
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
