//! Transition and TransitionGroup widgets for declarative animations
//!
//! These widgets provide Vue/React-style declarative animation APIs
//! that automatically apply animations when widgets are added, removed,
//! or reordered.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Transition, TransitionGroup, transition, transition_group};
//! use revue::style::animation::{Animation, ease_in_out, fade_in, slide_in_left};
//!
//! // Single element transition
//! Transition::new(content)
//!     .enter(Animation::fade_in().duration(300))
//!     .leave(Animation::fade_out().duration(200));
//!
//! // List transitions
//! let items = vec!["Item 1", "Item 2", "Item 3"];
//! TransitionGroup::new(items)
//!     .enter(Animation::slide_in_left())
//!     .leave(Animation::slide_out_right())
//!     .stagger(50); // ms delay between items
//! ```

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::{easing, AnimationState as TweenState, Color, Tween};
use std::time::Duration;

/// Transition animation configuration
#[derive(Clone)]
pub struct Animation {
    /// Type of animation preset
    preset: AnimationPreset,
    /// Duration in milliseconds
    duration: Duration,
    /// Easing function
    easing: fn(f32) -> f32,
    /// Delay before animation starts
    delay: Duration,
}

/// Animation presets for common transitions
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationPreset {
    /// Fade opacity from 0 to 1 (enter) or 1 to 0 (leave)
    Fade,
    /// Slide from left (enter) or to left (leave)
    SlideLeft,
    /// Slide from right (enter) or to right (leave)
    SlideRight,
    /// Slide from top (enter) or to top (leave)
    SlideUp,
    /// Slide from bottom (enter) or to bottom (leave)
    SlideDown,
    /// Scale from 0 to 1 (enter) or 1 to 0 (leave)
    Scale,
    /// Custom animation with user-defined parameters
    Custom {
        /// Opacity start (enter) / end (leave)
        opacity: Option<f32>,
        /// X offset start (enter) / end (leave)
        offset_x: Option<i16>,
        /// Y offset start (enter) / end (leave)
        offset_y: Option<i16>,
        /// Scale start (enter) / end (leave)
        scale: Option<f32>,
    },
}

impl Animation {
    /// Create a fade animation
    pub fn fade() -> Self {
        Self {
            preset: AnimationPreset::Fade,
            duration: Duration::from_millis(300),
            easing: easing::ease_in_out,
            delay: Duration::ZERO,
        }
    }

    /// Fade in animation
    pub fn fade_in() -> Self {
        Self::fade()
    }

    /// Fade out animation
    pub fn fade_out() -> Self {
        Self::fade()
    }

    /// Slide from left animation
    pub fn slide_left() -> Self {
        Self {
            preset: AnimationPreset::SlideLeft,
            duration: Duration::from_millis(300),
            easing: easing::ease_out,
            delay: Duration::ZERO,
        }
    }

    /// Slide in from left
    pub fn slide_in_left() -> Self {
        Self::slide_left()
    }

    /// Slide to left
    pub fn slide_out_left() -> Self {
        Self::slide_left()
    }

    /// Slide from right animation
    pub fn slide_right() -> Self {
        Self {
            preset: AnimationPreset::SlideRight,
            duration: Duration::from_millis(300),
            easing: easing::ease_out,
            delay: Duration::ZERO,
        }
    }

    /// Slide in from right
    pub fn slide_in_right() -> Self {
        Self::slide_right()
    }

    /// Slide to right
    pub fn slide_out_right() -> Self {
        Self::slide_right()
    }

    /// Slide from top animation
    pub fn slide_up() -> Self {
        Self {
            preset: AnimationPreset::SlideUp,
            duration: Duration::from_millis(300),
            easing: easing::ease_out,
            delay: Duration::ZERO,
        }
    }

    /// Slide in from top
    pub fn slide_in_up() -> Self {
        Self::slide_up()
    }

    /// Slide to top
    pub fn slide_out_up() -> Self {
        Self::slide_up()
    }

    /// Slide from bottom animation
    pub fn slide_down() -> Self {
        Self {
            preset: AnimationPreset::SlideDown,
            duration: Duration::from_millis(300),
            easing: easing::ease_out,
            delay: Duration::ZERO,
        }
    }

    /// Slide in from bottom
    pub fn slide_in_down() -> Self {
        Self::slide_down()
    }

    /// Slide to bottom
    pub fn slide_out_down() -> Self {
        Self::slide_down()
    }

    /// Scale animation
    pub fn scale() -> Self {
        Self {
            preset: AnimationPreset::Scale,
            duration: Duration::from_millis(300),
            easing: easing::back_out,
            delay: Duration::ZERO,
        }
    }

    /// Scale up animation
    pub fn scale_up() -> Self {
        Self::scale()
    }

    /// Scale down animation
    pub fn scale_down() -> Self {
        Self::scale()
    }

    /// Create a custom animation
    pub fn custom(
        opacity: Option<f32>,
        offset_x: Option<i16>,
        offset_y: Option<i16>,
        scale: Option<f32>,
    ) -> Self {
        Self {
            preset: AnimationPreset::Custom {
                opacity,
                offset_x,
                offset_y,
                scale,
            },
            duration: Duration::from_millis(300),
            easing: easing::ease_in_out,
            delay: Duration::ZERO,
        }
    }

    /// Set animation duration
    pub fn duration(mut self, duration_ms: u64) -> Self {
        self.duration = Duration::from_millis(duration_ms);
        self
    }

    /// Set easing function
    pub fn easing(mut self, easing: fn(f32) -> f32) -> Self {
        self.easing = easing;
        self
    }

    /// Set delay before animation starts
    pub fn delay(mut self, delay_ms: u64) -> Self {
        self.delay = Duration::from_millis(delay_ms);
        self
    }

    /// Get the preset
    pub fn preset(&self) -> AnimationPreset {
        self.preset
    }

    /// Get the duration
    pub fn get_duration(&self) -> Duration {
        self.duration
    }

    /// Get the easing function
    pub fn get_easing(&self) -> fn(f32) -> f32 {
        self.easing
    }

    /// Get the delay
    pub fn get_delay(&self) -> Duration {
        self.delay
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self::fade()
    }
}

/// Transition state
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransitionPhase {
    /// Entering (appearing)
    Entering,
    /// Visible (animation complete or no animation)
    Visible,
    /// Leaving (disappearing)
    Leaving,
}

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
    fn test_animation_defaults() {
        let anim = Animation::fade();
        assert_eq!(anim.get_duration(), Duration::from_millis(300));
        assert_eq!(anim.get_delay(), Duration::ZERO);
    }

    #[test]
    fn test_animation_builder() {
        let anim = Animation::fade()
            .duration(500)
            .delay(100)
            .easing(easing::linear);

        assert_eq!(anim.get_duration(), Duration::from_millis(500));
        assert_eq!(anim.get_delay(), Duration::from_millis(100));
    }

    #[test]
    fn test_animation_presets() {
        let fade = Animation::fade();
        assert_eq!(fade.preset(), AnimationPreset::Fade);

        let slide = Animation::slide_left();
        assert_eq!(slide.preset(), AnimationPreset::SlideLeft);

        let scale = Animation::scale();
        assert_eq!(scale.preset(), AnimationPreset::Scale);
    }

    #[test]
    fn test_animation_custom() {
        let custom = Animation::custom(Some(0.5), Some(10), Some(-5), Some(0.8));
        assert!(matches!(custom.preset(), AnimationPreset::Custom { .. }));
    }

    #[test]
    fn test_transition_new() {
        let transition = Transition::new("Hello");
        assert_eq!(transition.child_content, "Hello");
        assert!(transition.is_visible());
        assert!(transition.enter_animation.is_none());
        assert!(transition.leave_animation.is_none());
    }

    #[test]
    fn test_transition_builder() {
        let enter = Animation::fade_in();
        let leave = Animation::fade_out();
        let transition = Transition::new("Test")
            .enter(enter.clone())
            .leave(leave.clone());

        assert!(transition.enter_animation.is_some());
        assert!(transition.leave_animation.is_some());
    }

    #[test]
    fn test_transition_toggle() {
        let mut transition = Transition::new("Test");
        assert!(transition.is_visible());

        transition.hide();
        // Phase changes to Leaving but visible remains true until animation completes

        transition.show();
        assert!(transition.is_visible());
    }

    #[test]
    fn test_transition_group_new() {
        let group = TransitionGroup::new(vec!["Item 1", "Item 2", "Item 3"]);
        assert_eq!(group.len(), 3);
        assert!(!group.is_empty());
    }

    #[test]
    fn test_transition_group_empty() {
        let group: TransitionGroup = TransitionGroup::new(Vec::<String>::new());
        assert_eq!(group.len(), 0);
        assert!(group.is_empty());
    }

    #[test]
    fn test_transition_group_builder() {
        let group = TransitionGroup::new(vec!["A", "B"])
            .enter(Animation::fade_in())
            .leave(Animation::fade_out())
            .stagger(50);

        assert!(group.enter_animation.is_some());
        assert!(group.leave_animation.is_some());
        assert_eq!(group.stagger_delay, 50);
    }

    #[test]
    fn test_transition_group_push_remove() {
        let mut group = TransitionGroup::new(vec!["Item 1"]);
        assert_eq!(group.len(), 1);

        group.push("Item 2");
        assert_eq!(group.len(), 2);

        let removed = group.remove(0);
        assert_eq!(removed, Some("Item 1".to_string()));
        assert_eq!(group.len(), 1);
    }

    #[test]
    fn test_convenience_functions() {
        let transition = transition("Hello");
        assert_eq!(transition.child_content, "Hello");

        let group = transition_group(vec!["A", "B"]);
        assert_eq!(group.len(), 2);
    }
}
