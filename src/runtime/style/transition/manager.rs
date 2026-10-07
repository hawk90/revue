//! Running transitions: per-property state and the transition manager

use super::{Easing, Transition};
use std::time::Duration;

/// Active transition state for a property
#[derive(Debug, Clone)]
pub struct ActiveTransition {
    /// Property being transitioned
    pub property: String,
    /// Start value
    pub from: f32,
    /// End value
    pub to: f32,
    /// Duration
    pub duration: Duration,
    /// Delay
    pub delay: Duration,
    /// Easing
    pub easing: Easing,
    /// Elapsed time
    pub elapsed: Duration,
    /// Whether the transition has started (after delay)
    pub started: bool,
}

impl ActiveTransition {
    /// Create a new active transition
    pub fn new(property: impl Into<String>, from: f32, to: f32, transition: &Transition) -> Self {
        Self {
            property: property.into(),
            from,
            to,
            duration: transition.duration,
            delay: transition.delay,
            easing: transition.easing,
            elapsed: Duration::ZERO,
            started: transition.delay.is_zero(),
        }
    }

    /// Update the transition with elapsed time
    pub fn update(&mut self, delta: Duration) {
        self.elapsed += delta;

        if !self.started && self.elapsed >= self.delay {
            self.started = true;
            self.elapsed = self.elapsed.saturating_sub(self.delay);
        }
    }

    /// Get current interpolated value
    pub fn current(&self) -> f32 {
        if !self.started {
            return self.from;
        }

        let progress = if self.duration.is_zero() {
            1.0
        } else {
            (self.elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0)
        };

        let eased = self.easing.apply(progress);
        self.from + (self.to - self.from) * eased
    }

    /// Check if transition is complete
    pub fn is_complete(&self) -> bool {
        self.started && self.elapsed >= self.duration
    }
}

/// Transition manager for handling multiple active transitions
#[derive(Debug, Clone, Default)]
pub struct TransitionManager {
    /// Active transitions (legacy, not node-aware)
    active: Vec<ActiveTransition>,
    /// Node-aware transitions: maps element ID to active transitions
    node_transitions: std::collections::HashMap<String, Vec<ActiveTransition>>,
}

impl TransitionManager {
    /// Create a new transition manager
    pub fn new() -> Self {
        Self {
            active: Vec::new(),
            node_transitions: std::collections::HashMap::new(),
        }
    }

    /// Start a transition for a property
    ///
    /// If reduced motion is preferred, the transition completes instantly
    /// (no animation, just jumps to final value).
    pub fn start(
        &mut self,
        property: impl Into<String>,
        from: f32,
        to: f32,
        transition: &Transition,
    ) {
        let property = property.into();

        // Remove existing transition for this property
        self.active.retain(|t| t.property != property);

        // Skip animation entirely if reduced motion is preferred
        if should_skip_animation() {
            // Don't add any transition - the property will use final value directly
            return;
        }

        // Add new transition
        self.active
            .push(ActiveTransition::new(&property, from, to, transition));
    }

    /// Update all transitions
    pub fn update(&mut self, delta: Duration) {
        for transition in &mut self.active {
            transition.update(delta);
        }

        // Remove completed transitions
        self.active.retain(|t| !t.is_complete());
    }

    /// Get current value for a property
    pub fn get(&self, property: &str) -> Option<f32> {
        self.active
            .iter()
            .find(|t| t.property == property)
            .map(|t| t.current())
    }

    /// Check if there are active transitions
    pub fn has_active(&self) -> bool {
        !self.active.is_empty() || !self.node_transitions.is_empty()
    }

    /// Get all active transition property names
    pub fn active_properties(&self) -> impl Iterator<Item = &str> {
        self.active.iter().map(|t| t.property.as_str())
    }

    /// Clear all transitions
    pub fn clear(&mut self) {
        self.active.clear();
        self.node_transitions.clear();
    }

    /// Get all current transition values as a map
    ///
    /// Returns a HashMap that can be passed to RenderContext for widgets
    /// to access animated property values during rendering.
    pub fn current_values(&self) -> std::collections::HashMap<String, f32> {
        self.active
            .iter()
            .map(|t| (t.property.clone(), t.current()))
            .collect()
    }

    // =========================================================================
    // Node-aware transition methods for partial rendering optimization
    // =========================================================================

    /// Start a transition for a specific element
    ///
    /// Associates the transition with an element ID for partial rendering.
    /// If reduced motion is preferred, no transition is added.
    pub fn start_for_node(
        &mut self,
        element_id: impl Into<String>,
        property: impl Into<String>,
        from: f32,
        to: f32,
        transition: &Transition,
    ) {
        // Skip animation entirely if reduced motion is preferred
        if should_skip_animation() {
            return;
        }

        let element_id = element_id.into();
        let property = property.into();

        let transitions = self.node_transitions.entry(element_id).or_default();

        // Remove existing transition for this property
        transitions.retain(|t| t.property != property);

        // Add new transition
        transitions.push(ActiveTransition::new(&property, from, to, transition));
    }

    /// Update all node-aware transitions
    pub fn update_nodes(&mut self, delta: Duration) {
        for transitions in self.node_transitions.values_mut() {
            for transition in transitions.iter_mut() {
                transition.update(delta);
            }
            // Remove completed transitions
            transitions.retain(|t| !t.is_complete());
        }

        // Remove entries with no active transitions
        self.node_transitions.retain(|_, v| !v.is_empty());
    }

    /// Get current value for a property on a specific node
    pub fn get_for_node(&self, element_id: &str, property: &str) -> Option<f32> {
        self.node_transitions
            .get(element_id)?
            .iter()
            .find(|t| t.property == property)
            .map(|t| t.current())
    }

    /// Get all element IDs with active transitions
    ///
    /// Used for partial rendering - only these nodes need to be redrawn.
    pub fn active_node_ids(&self) -> impl Iterator<Item = &str> {
        self.node_transitions.keys().map(|s| s.as_str())
    }

    /// Check if a specific node has active transitions
    pub fn node_has_active(&self, element_id: &str) -> bool {
        self.node_transitions
            .get(element_id)
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }

    /// Get all current transition values for a specific node
    pub fn current_values_for_node(
        &self,
        element_id: &str,
    ) -> std::collections::HashMap<String, f32> {
        self.node_transitions
            .get(element_id)
            .map(|transitions| {
                transitions
                    .iter()
                    .map(|t| (t.property.clone(), t.current()))
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// Check if animations should be skipped due to reduced motion preference
///
/// When reduced motion is preferred, this returns true and animations
/// should complete instantly instead of animating.
pub fn should_skip_animation() -> bool {
    crate::utils::prefers_reduced_motion()
}

/// Get effective duration considering reduced motion preference
///
/// Returns Duration::ZERO if reduced motion is preferred (instant transition).
pub fn effective_duration(duration: Duration) -> Duration {
    if should_skip_animation() {
        Duration::ZERO
    } else {
        duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_transition_new() {
        let transition = Transition::new("opacity", Duration::from_millis(300));
        let active = ActiveTransition::new("opacity", 0.0, 1.0, &transition);
        assert_eq!(active.property, "opacity");
        assert_eq!(active.from, 0.0);
        assert_eq!(active.to, 1.0);
        assert!(active.started); // delay is zero, so should be started immediately
    }

    #[test]
    fn test_active_transition_current_before_start() {
        let transition = Transition::new("opacity", Duration::from_millis(300))
            .delay(Duration::from_millis(100));
        let active = ActiveTransition::new("opacity", 0.0, 1.0, &transition);
        assert_eq!(active.current(), 0.0); // Should return from value
    }

    #[test]
    fn test_active_transition_update() {
        let transition = Transition::new("opacity", Duration::from_millis(300));
        let mut active = ActiveTransition::new("opacity", 0.0, 1.0, &transition);
        active.update(Duration::from_millis(150));
        assert!(active.started);
    }

    #[test]
    fn test_active_transition_is_complete() {
        let transition = Transition::new("opacity", Duration::from_millis(300));
        let mut active = ActiveTransition::new("opacity", 0.0, 1.0, &transition);
        assert!(!active.is_complete());
        active.update(Duration::from_millis(300));
        assert!(active.is_complete());
    }

    #[test]
    fn test_transition_manager_new() {
        let manager = TransitionManager::new();
        assert!(!manager.has_active());
    }

    #[test]
    fn test_transition_manager_default() {
        let manager = TransitionManager::default();
        assert!(!manager.has_active());
    }

    #[test]
    fn test_transition_manager_start() {
        let mut manager = TransitionManager::new();
        let transition = Transition::new("opacity", Duration::from_millis(300));
        manager.start("opacity", 0.0, 1.0, &transition);
        assert!(manager.has_active());
    }

    #[test]
    fn test_transition_manager_get() {
        let mut manager = TransitionManager::new();
        let transition = Transition::new("opacity", Duration::from_millis(300));
        manager.start("opacity", 0.0, 1.0, &transition);
        assert!(manager.get("opacity").is_some());
    }

    #[test]
    fn test_transition_manager_clear() {
        let mut manager = TransitionManager::new();
        let transition = Transition::new("opacity", Duration::from_millis(300));
        manager.start("opacity", 0.0, 1.0, &transition);
        manager.clear();
        assert!(!manager.has_active());
    }

    #[test]
    fn test_transition_manager_update() {
        let mut manager = TransitionManager::new();
        let transition = Transition::new("opacity", Duration::from_millis(300));
        manager.start("opacity", 0.0, 1.0, &transition);
        manager.update(Duration::from_millis(400));
        assert!(!manager.has_active()); // Complete transition removed
    }

    #[test]
    fn test_transition_manager_current_values() {
        let mut manager = TransitionManager::new();
        let transition = Transition::new("opacity", Duration::from_millis(300));
        manager.start("opacity", 0.0, 1.0, &transition);
        manager.update(Duration::from_millis(150));
        let values = manager.current_values();
        assert!(values.contains_key("opacity"));
    }
}
