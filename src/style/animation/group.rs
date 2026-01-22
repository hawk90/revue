//! Animation group support for coordinating multiple animations

use std::time::{Duration, Instant};

use super::{AnimationState, KeyframeAnimation};

/// Mode for animation group execution
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GroupMode {
    /// Run all animations simultaneously
    #[default]
    Parallel,
    /// Run animations one after another
    Sequential,
}

/// Group of animations that can run in parallel or sequence
#[derive(Clone)]
pub struct AnimationGroup {
    animations: Vec<KeyframeAnimation>,
    mode: GroupMode,
    state: AnimationState,
    start_time: Option<Instant>,
}

impl AnimationGroup {
    /// Create a parallel animation group
    pub fn parallel() -> Self {
        Self {
            animations: Vec::new(),
            mode: GroupMode::Parallel,
            state: AnimationState::Pending,
            start_time: None,
        }
    }

    /// Create a sequential animation group
    pub fn sequential() -> Self {
        Self {
            animations: Vec::new(),
            mode: GroupMode::Sequential,
            state: AnimationState::Pending,
            start_time: None,
        }
    }

    /// Add an animation to the group
    pub fn with_animation(mut self, animation: KeyframeAnimation) -> Self {
        self.animations.push(animation);
        self
    }

    /// Get total duration of the group
    pub fn total_duration(&self) -> Duration {
        match self.mode {
            GroupMode::Parallel => self
                .animations
                .iter()
                .map(|a| a.delay + a.duration)
                .max()
                .unwrap_or(Duration::ZERO),
            GroupMode::Sequential => self.animations.iter().map(|a| a.delay + a.duration).sum(),
        }
    }

    /// Start all animations
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.state = AnimationState::Running;

        match self.mode {
            GroupMode::Parallel => {
                for anim in &mut self.animations {
                    anim.start();
                }
            }
            GroupMode::Sequential => {
                // Start only the first animation; others will be started as they complete
                if let Some(first) = self.animations.first_mut() {
                    first.start();
                }
            }
        }
    }

    /// Update all animations
    pub fn update(&mut self) {
        if self.state != AnimationState::Running {
            return;
        }

        match self.mode {
            GroupMode::Parallel => {
                // Check if all are completed
                if self.animations.iter().all(|a| a.is_completed()) {
                    self.state = AnimationState::Completed;
                }
            }
            GroupMode::Sequential => {
                // Find current running animation
                let mut should_start_next = false;
                let mut next_idx = 0;

                for (i, anim) in self.animations.iter().enumerate() {
                    if anim.is_running() {
                        break;
                    }
                    if anim.is_completed()
                        && i + 1 < self.animations.len()
                        && !self.animations[i + 1].is_running()
                        && !self.animations[i + 1].is_completed()
                    {
                        should_start_next = true;
                        next_idx = i + 1;
                    }
                }

                if should_start_next {
                    self.animations[next_idx].start();
                }

                // Check if all are completed
                if self.animations.iter().all(|a| a.is_completed()) {
                    self.state = AnimationState::Completed;
                }
            }
        }
    }

    /// Check if all animations are completed
    pub fn is_completed(&self) -> bool {
        self.state == AnimationState::Completed
    }

    /// Get mutable reference to animations
    pub fn animations_mut(&mut self) -> &mut [KeyframeAnimation] {
        &mut self.animations
    }
}
