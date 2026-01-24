//! Animation sequences

use crate::utils::easing::Easing;
use std::time::Duration;

use super::Timer;

/// Animation step in a sequence
#[derive(Clone)]
pub struct SequenceStep {
    /// Duration of this step
    pub duration: Duration,
    /// Easing for this step
    pub easing: Easing,
    /// Target value (normalized 0.0 to 1.0)
    pub target: f64,
}

impl SequenceStep {
    /// Create a new step
    pub fn new(duration: Duration, target: f64) -> Self {
        Self {
            duration,
            easing: Easing::Linear,
            target: target.clamp(0.0, 1.0),
        }
    }

    /// Set easing
    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }
}

/// Sequential animation with multiple steps
#[derive(Clone)]
pub struct Sequence {
    /// Animation steps
    pub steps: Vec<SequenceStep>,
    /// Current step index
    current_step: usize,
    /// Timer for current step
    timer: Timer,
    /// Current value
    value: f64,
    /// Whether sequence has started
    started: bool,
    /// Whether to repeat the sequence
    pub repeat: bool,
}

impl Default for Sequence {
    fn default() -> Self {
        Self::new()
    }
}

impl Sequence {
    /// Create a new sequence
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            current_step: 0,
            timer: Timer::new(Duration::ZERO),
            value: 0.0,
            started: false,
            repeat: false,
        }
    }

    /// Add a step
    pub fn then(mut self, duration: Duration, target: f64) -> Self {
        self.steps.push(SequenceStep::new(duration, target));
        self
    }

    /// Add a step with easing
    pub fn then_eased(mut self, duration: Duration, target: f64, easing: Easing) -> Self {
        self.steps
            .push(SequenceStep::new(duration, target).easing(easing));
        self
    }

    /// Add a delay (step that holds current value)
    pub fn delay(self, duration: Duration) -> Self {
        let current = self.steps.last().map(|s| s.target).unwrap_or(0.0);
        self.then(duration, current)
    }

    /// Enable looping
    pub fn repeat(mut self, repeat: bool) -> Self {
        self.repeat = repeat;
        self
    }

    /// Start the sequence
    pub fn start(&mut self) {
        self.current_step = 0;
        self.value = 0.0;
        self.started = true;
        if let Some(step) = self.steps.first() {
            self.timer = Timer::new(step.duration);
            self.timer.start();
        }
    }

    /// Reset the sequence
    pub fn reset(&mut self) {
        self.current_step = 0;
        self.value = 0.0;
        self.started = false;
        self.timer.reset();
    }

    /// Check if sequence is running
    pub fn is_running(&self) -> bool {
        self.started && self.current_step < self.steps.len()
    }

    /// Check if sequence is complete
    pub fn is_complete(&self) -> bool {
        self.started && self.current_step >= self.steps.len()
    }

    /// Get current value (and advance if needed)
    pub fn value(&mut self) -> f64 {
        if !self.started || self.steps.is_empty() {
            return self.value;
        }

        // Check if current step is complete
        while self.current_step < self.steps.len() && self.timer.is_finished() {
            // Move to next step
            self.value = self.steps[self.current_step].target;
            self.current_step += 1;

            if self.current_step < self.steps.len() {
                self.timer = Timer::new(self.steps[self.current_step].duration);
                self.timer.start();
            } else if self.repeat {
                // Loop back to start
                self.current_step = 0;
                self.timer = Timer::new(self.steps[0].duration);
                self.timer.start();
            }
        }

        // Interpolate current step
        if self.current_step < self.steps.len() {
            let step = &self.steps[self.current_step];
            let prev_value = if self.current_step == 0 {
                0.0
            } else {
                self.steps[self.current_step - 1].target
            };
            let t = self.timer.progress_eased(step.easing);
            self.value = prev_value + (step.target - prev_value) * t;
        }

        self.value
    }
}
