//! Animation choreographer for coordinating complex animation sequences

use std::collections::HashMap;
use std::time::Duration;

use super::{AnimationGroup, KeyframeAnimation, Stagger};

/// Manages multiple animation groups and coordinates complex animation sequences
///
/// # Example
///
/// ```rust,ignore
/// use revue::style::animation::{Choreographer, KeyframeAnimation, AnimationGroup};
/// use std::time::Duration;
///
/// let mut choreo = Choreographer::new();
///
/// // Add a staggered entrance
/// choreo.add_staggered(
///     "list-items",
///     5,
///     Duration::from_millis(50),
///     |i| KeyframeAnimation::new(format!("item-{}", i))
///         .keyframe(0, |kf| kf.set("opacity", 0.0))
///         .keyframe(100, |kf| kf.set("opacity", 1.0))
///         .duration(Duration::from_millis(200))
/// );
///
/// choreo.start("list-items");
/// ```
pub struct Choreographer {
    groups: HashMap<String, AnimationGroup>,
    staggered: HashMap<String, Vec<KeyframeAnimation>>,
}

impl Default for Choreographer {
    fn default() -> Self {
        Self::new()
    }
}

impl Choreographer {
    /// Create a new choreographer
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            staggered: HashMap::new(),
        }
    }

    /// Add an animation group
    pub fn add_group(&mut self, name: impl Into<String>, group: AnimationGroup) {
        self.groups.insert(name.into(), group);
    }

    /// Add a staggered animation set
    pub fn add_staggered<F>(
        &mut self,
        name: impl Into<String>,
        count: usize,
        delay: Duration,
        create: F,
    ) where
        F: FnMut(usize) -> KeyframeAnimation,
    {
        let stagger = Stagger::new(count, delay);
        let animations = stagger.apply(create);
        self.staggered.insert(name.into(), animations);
    }

    /// Start a named animation group or staggered set
    pub fn start(&mut self, name: &str) {
        if let Some(group) = self.groups.get_mut(name) {
            group.start();
        }
        if let Some(anims) = self.staggered.get_mut(name) {
            for anim in anims {
                anim.start();
            }
        }
    }

    /// Update all animations
    pub fn update(&mut self) {
        for group in self.groups.values_mut() {
            group.update();
        }
    }

    /// Get a value from a staggered animation
    pub fn get_staggered(&mut self, name: &str, index: usize, property: &str) -> f32 {
        self.staggered
            .get_mut(name)
            .and_then(|anims| anims.get_mut(index))
            .map(|anim| anim.get(property))
            .unwrap_or(0.0)
    }

    /// Check if a named animation is completed
    pub fn is_completed(&self, name: &str) -> bool {
        if let Some(group) = self.groups.get(name) {
            return group.is_completed();
        }
        if let Some(anims) = self.staggered.get(name) {
            return anims.iter().all(|a| a.is_completed());
        }
        true
    }
}
